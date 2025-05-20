use itertools::Itertools;
use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};
use unicode_width::UnicodeWidthStr;

use super::{
    context::AlignMode,
    doc_ext::AllocExt,
    layout::aligned::{AlignedStylist, Row},
    ArenaDoc, Context, PrettyPrinter,
};
use crate::{ext::StrExt, AttrStore};

impl<'a> PrettyPrinter<'a> {
    /// Attempt to format a math node as an aligned grid if there are align points.
    pub(super) fn try_convert_math_aligned(
        &'a self,
        ctx: Context,
        math: Math<'a>,
    ) -> Option<ArenaDoc<'a>> {
        // Skip if alignment is disabled or no math align points present
        if ctx.align_mode == AlignMode::Never
            || !self.attr_store.can_align_in_math(math.to_untyped())
        {
            return None;
        }
        let ctx = ctx.aligned(AlignMode::Outer);
        let raw_aligned = collect_aligned(math, &self.attr_store);
        let aligned = raw_aligned
            .rows
            .into_iter()
            .map(|row| match row {
                NodeRow::Comment(cmt) => Row::Free(self.convert_comment(ctx, cmt)),
                NodeRow::Cells(cells) => {
                    let mut rendered_cells = Vec::with_capacity(cells.len());
                    for cell_nodes in cells.into_iter() {
                        // Render the content of each cell into a string buffer
                        let ends_with_line_comment = cell_nodes
                            .last()
                            .is_some_and(|n| n.kind() == SyntaxKind::LineComment);

                        let mut buf = String::new();
                        self.convert_math_children(ctx, cell_nodes.into_iter())
                            .render_fmt(self.config.max_width, &mut buf)
                            .ok()?;
                        if ends_with_line_comment {
                            buf.push_str("\n "); // ensure an extra line is added
                        }
                        rendered_cells.push(buf);
                    }
                    Row::Cells(rendered_cells)
                }
            })
            .collect();

        let doc = AlignedStylist::new(&self.arena)
            .render(aligned, self.config.max_width)?
            .print(ctx.align_mode == AlignMode::Outer && raw_aligned.has_trailing_linebreak);
        Some(doc)
    }
}

struct AlignedNodes<'a> {
    rows: Vec<NodeRow<'a>>,
    has_trailing_linebreak: bool,
}

/// A raw row before rendering, coming from syntax nodes.
enum NodeRow<'a> {
    Cells(Vec<Vec<&'a SyntaxNode>>),
    Comment(&'a SyntaxNode),
}

impl NodeRow<'_> {
    pub fn len(&self) -> usize {
        match self {
            NodeRow::Cells(items) => items.len(),
            NodeRow::Comment(_) => 0,
        }
    }
}

/// Collect math syntax nodes, split into lines/cells by align points and linebreaks.
fn collect_aligned<'a>(math: Math<'a>, attrs: &AttrStore) -> AlignedNodes<'a> {
    // Helper to trim trailing space nodes from a cell
    fn trim_trailing_spaces(cell: &mut Vec<&SyntaxNode>) {
        while cell
            .last()
            .is_some_and(|last| last.kind() == SyntaxKind::Space)
        {
            cell.pop();
        }
    }

    // Gather all relevant children, then split on linebreaks
    fn collect_children<'a>(
        node: &'a SyntaxNode,
        attrs: &AttrStore,
        out: &mut Vec<&'a SyntaxNode>,
    ) {
        if !(matches!(node.kind(), SyntaxKind::Math | SyntaxKind::MathDelimited)
            && attrs.has_math_align_point(node))
        {
            out.push(node);
            return;
        }
        for child in node.children() {
            collect_children(child, attrs, out);
        }
    }

    let flat = {
        let mut flat = Vec::with_capacity(math.to_untyped().children().len());
        collect_children(math.to_untyped(), attrs, &mut flat);
        flat
    };

    // First pass: split all children into lines (split at Linebreak)
    let (lines, has_trailing_linebreak) = {
        let mut lines = flat
            .split(|n| n.kind() == SyntaxKind::Linebreak)
            .collect_vec();
        let has_trailing_linebreak = if lines.last().is_some_and(|last| last.is_empty()) {
            lines.pop();
            true
        } else {
            false
        };
        (lines, has_trailing_linebreak)
    };

    // Second pass: create rows; if a line starts with a line comment, create a Comment row.
    let mut rows = Vec::with_capacity(lines.len());
    for line in lines {
        let mut cells = Vec::new();
        let mut current_cell = Vec::new();
        for node in line {
            match node.kind() {
                SyntaxKind::MathAlignPoint => {
                    trim_trailing_spaces(&mut current_cell);
                    cells.push(std::mem::take(&mut current_cell));
                }
                SyntaxKind::Space if current_cell.is_empty() => {}
                SyntaxKind::LineComment if cells.is_empty() && current_cell.is_empty() => {
                    rows.push(NodeRow::Comment(node));
                }
                _ => {
                    current_cell.push(node);
                }
            }
        }
        trim_trailing_spaces(&mut current_cell);
        cells.push(current_cell);
        rows.push(NodeRow::Cells(cells));
    }
    AlignedNodes {
        rows,
        has_trailing_linebreak,
    }
}
