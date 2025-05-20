use crate::{
    ext::StrExt,
    pretty::{doc_ext::AllocExt, ArenaDoc},
};
use pretty::{Arena, DocAllocator};
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};
use unicode_width::UnicodeWidthStr;

pub enum Row<'a> {
    Cells(Vec<String>),
    Free(ArenaDoc<'a>),
}

/// A fully measured grid of rows and column widths.
struct MeasuredAligned<'a> {
    arena: &'a Arena<'a>,
    rows: Vec<MeasuredRow<'a>>,
    col_widths: Vec<usize>,
}

/// A single row, either a list of cells or a standalone comment.
enum MeasuredRow<'a> {
    Cells(Vec<MeasuredCell>),
    Free(ArenaDoc<'a>),
}

/// A formatted cell, storing its content and computed width.
enum MeasuredCell {
    Empty,
    SingleLine(String, usize), // text and its width
    MultiLine(Vec<(String, usize)>), // lines with their widths
                               // todo: free
}

impl MeasuredCell {
    /// Return the maximum display width of this cell.
    pub fn max_width(&self) -> usize {
        match self {
            MeasuredCell::Empty => 0,
            MeasuredCell::SingleLine(_, width) => *width,
            MeasuredCell::MultiLine(lines) => {
                lines.iter().map(|(_, width)| *width).max().unwrap_or(0)
            }
        }
    }

    /// Return the width of the last line in this cell.
    pub fn width(&self) -> usize {
        match self {
            MeasuredCell::Empty => 0,
            MeasuredCell::SingleLine(_, width) => *width,
            MeasuredCell::MultiLine(lines) => lines.last().map(|(_, width)| *width).unwrap_or(0),
        }
    }
}

struct AlignedNodes<'a> {
    rows: Vec<NodeRow<'a>>,
    has_trailing_linebreak: bool,
}

/// A raw row before rendering, coming from syntax nodes.
pub enum NodeRow<'a> {
    Cells(Vec<ArenaDoc<'a>>),
    Comment(ArenaDoc<'a>),
}

impl NodeRow<'_> {
    pub fn len(&self) -> usize {
        match self {
            NodeRow::Cells(items) => items.len(),
            NodeRow::Comment(_) => 0,
        }
    }
}

pub struct AlignedStylist<'a> {
    arena: &'a Arena<'a>,
}

pub struct AlignedStyle {
    pub row_sep: &'static str,
}

impl<'a> AlignedStylist<'a> {
    pub fn new(arena: &'a Arena<'a>) -> Self {
        Self { arena }
    }

    /// Build aligned rows by measuring each cell and tracking column widths.
    pub fn render(
        self,
        aligned_elems: Vec<Row<'a>>,
        max_width: usize,
    ) -> Option<MeasuredAligned<'a>> {
        // Determine how many columns we need
        let col_num = aligned_elems
            .iter()
            .map(|row| match row {
                Row::Cells(cells) => cells.len(),
                Row::Free(_) => 0,
            })
            .max()
            .unwrap_or_default();

        // Early‑exit if even the empty grid would exceed max width
        if col_num > max_width {
            return None;
        }

        let mut col_widths = vec![0; col_num];
        let mut grid_width = col_num;

        // Render each raw row into a Row and set column widths
        let rows = (aligned_elems.into_iter()).try_fold(vec![], |mut rows, row| {
            let rendered_row = match row {
                Row::Free(free) => MeasuredRow::Free(free),
                Row::Cells(cells) => {
                    let mut rendered_cells = Vec::with_capacity(cells.len());
                    for (j, buf) in cells.into_iter().enumerate() {
                        let measure_width = |line: &str| {
                            let render_width = line.width();
                            if j == 0 || j + 1 == col_num {
                                render_width + 1
                            } else {
                                render_width + 2
                            }
                        };

                        let rendered_cell = if buf.is_empty() {
                            MeasuredCell::Empty
                        } else if buf.has_linebreak() {
                            MeasuredCell::MultiLine(
                                buf.lines()
                                    .map(|line| (line.to_string(), measure_width(line)))
                                    .collect(),
                            )
                        } else {
                            let line_width = measure_width(&buf);
                            MeasuredCell::SingleLine(buf, line_width)
                        };

                        // Update col_widths and bail out if we exceed max_width
                        let cell_width = rendered_cell.width();
                        if cell_width > col_widths[j] {
                            grid_width += cell_width - col_widths[j];
                            col_widths[j] = cell_width;
                            if grid_width > max_width {
                                return None; // bail out
                            }
                        }

                        rendered_cells.push(rendered_cell);
                    }
                    MeasuredRow::Cells(rendered_cells)
                }
            };
            rows.push(rendered_row);
            Some(rows)
        })?;

        Some(MeasuredAligned {
            arena: self.arena,
            rows,
            col_widths,
        })
    }
}

impl<'a> MeasuredAligned<'a> {
    /// Combine aligned cells together, inserting '&', spaces, and linebreaks.
    pub fn print(self, add_trailing_linebreak: bool) -> ArenaDoc<'a> {
        let rows = self.rows;
        let col_widths = self.col_widths;
        let num_rows = rows.len();
        let num_cols = col_widths.len();
        let col_widths_sum = {
            let mut sums = Vec::with_capacity(num_cols + 1);
            sums.push(0);
            for &width in &col_widths {
                sums.push(sums.last().unwrap() + width);
            }
            sums
        };
        let grid_width = col_widths_sum.last().unwrap() + num_cols;

        enum Alignment {
            Left,
            Right,
        }

        (self.arena).concat(rows.into_iter().enumerate().map(|(i, row)| match row {
            MeasuredRow::Free(free) => {
                // Emit a full‑line comment followed by a hard linebreak
                free + self.arena.hardline()
            }
            MeasuredRow::Cells(cells) => {
                let mut row_doc = self.arena.nil();

                // For each cell: pad to column width and insert separators
                let num_cells = cells.len();
                let mut is_prev_empty = false;
                for (j, cell) in cells.into_iter().enumerate() {
                    let alignment = if j % 2 == 1 || num_cols == 1 {
                        Alignment::Left
                    } else {
                        Alignment::Right
                    };
                    let col_width = col_widths[j];

                    let pad = |cell_doc: ArenaDoc<'a>, width: usize| {
                        let pad_spaces = self.arena.spaces(col_width - width);
                        match alignment {
                            Alignment::Left => cell_doc + pad_spaces,
                            Alignment::Right => pad_spaces + cell_doc,
                        }
                    };

                    let cell_width = cell.max_width();
                    let (padded_cell_doc, is_cur_empty) = match cell {
                        MeasuredCell::Empty => (pad(self.arena.nil(), 0), true),
                        MeasuredCell::SingleLine(line, width) => {
                            (pad(self.arena.text(line), width), false)
                        }
                        MeasuredCell::MultiLine(lines) => {
                            let padding_left = match alignment {
                                Alignment::Left => 0,
                                Alignment::Right => col_width.saturating_sub(cell_width),
                            };
                            let indent = {
                                let mut indent = col_widths_sum[j] + j + padding_left;
                                if j > 0 {
                                    indent += 1;
                                }
                                indent
                            };

                            let trailing_padding =
                                col_width - padding_left - lines[lines.len() - 1].1;
                            let doc = self.arena.spaces(padding_left)
                                + self.arena.intersperse(
                                    lines.into_iter().map(|(line, _)| line),
                                    self.arena.hardline(),
                                )
                                + self.arena.spaces(trailing_padding);
                            (doc.nest(indent as isize), false)
                        }
                    };

                    let sep = {
                        let mut sep = self.arena.nil();
                        if j > 0 {
                            if !is_prev_empty {
                                sep += self.arena.space();
                            }
                            sep += self.arena.text("&");
                            if !is_cur_empty {
                                sep += self.arena.space();
                            }
                        }
                        sep
                    };

                    row_doc += sep + padded_cell_doc;

                    is_prev_empty = is_cur_empty;
                }

                // If row has fewer cells than columns, add trailing spaces
                if num_cells < num_cols {
                    let mut padding = grid_width - num_cells - col_widths_sum[num_cells];
                    if !is_prev_empty {
                        padding += 1;
                    }
                    row_doc += self.arena.spaces(padding);
                }
                // Append trailing backslashes and linebreaks when multiple rows
                if num_rows > 1 {
                    row_doc += if add_trailing_linebreak || i + 1 != num_rows {
                        self.arena.text(" \\")
                    } else {
                        self.arena.text(" ")
                    };
                }
                if i + 1 != num_rows {
                    row_doc += self.arena.hardline();
                }
                row_doc
            }
        }))
    }
}
