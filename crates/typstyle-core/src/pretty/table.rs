use itertools::Itertools;
use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind};

use super::{
    util::{func_name, has_comment_children, indent_func_name},
    ArenaDoc, Context,
};
use crate::{
    pretty::{util::get_parenthesized_args, Mode},
    PrettyPrinter,
};

const BLACK_LIST: [&str; 6] = [
    "table.cell",
    "table.vline",
    "table.hline",
    "grid.cell",
    "grid.vline",
    "grid.hline",
];

enum CellReflowMode {
    Auto,
    Never,
}

/*
aligned
需要处理：不参与的（free），
如果没有blacklist, 那就
*/

impl<'a> PrettyPrinter<'a> {
    pub(super) fn try_convert_table(
        &'a self,
        ctx: Context,
        table: FuncCall<'a>,
    ) -> Option<ArenaDoc<'a>> {
        let cols = if is_table(table) && is_formattable_table(table) {
            get_table_columns(table)
        } else {
            None
        }?;
        Some(self.convert_table(ctx, table, cols))
    }

    // only handle parenthesized args here
    fn convert_table(&'a self, ctx: Context, table: FuncCall<'a>, columns: usize) -> ArenaDoc<'a> {
        let ctx = ctx.with_mode(Mode::CodeCont);

        // 关于reorder: header/footer一定独占一行, 如果出现cell/vline/hline那么不会reflow
        // 这里需要检查是否可reorder
        /*
        流程
        - 收集rows. 来一个处理一个
        - named单独占一行
        - 如果遇到header, or spread, 单独成一行
        - 如果有cell这种就不reflow，看到换行手动reflow
        -
         */

        let mut doc = self.arena.hardline();
        doc += (self.arena).concat(table.args().items().filter_map(|node| match node {
            Arg::Named(named) => Some(self.convert_named(ctx, named) + "," + self.arena.hardline()),
            _ => None,
        }));
        #[derive(Debug)]
        struct Row<'a> {
            cells: Vec<Arg<'a>>,
        }

        let pos_args = table
            .args()
            .to_untyped()
            .children()
            .take_while(|node| node.kind() != SyntaxKind::RightParen)
            .filter_map(|node| node.cast::<Arg>())
            .filter(|node| matches!(node, Arg::Pos(_)));
        let has_predecessor = |pos: &itertools::Position| {
            matches!(
                pos,
                itertools::Position::Middle | itertools::Position::First
            )
        };
        let table: Vec<Row> = {
            let mut table = Vec::new();
            let mut row = Row {
                cells: Vec::with_capacity(columns),
            };
            for arg in pos_args {
                row.cells.push(arg);
                if row.cells.len() == columns {
                    table.push(row);
                    row = Row {
                        cells: Vec::with_capacity(columns),
                    };
                }
                if is_header_or_footer(arg) {
                    table.push(row);
                    row = Row {
                        cells: Vec::with_capacity(columns),
                    };
                }
            }
            if !row.cells.is_empty() {
                table.push(row);
            }
            table
        };
        for (row_pos, row) in table.into_iter().with_position() {
            let mut row_doc = self.arena.nil();
            for (pos, cell) in row.cells.into_iter().with_position() {
                row_doc = row_doc
                    + self.convert_arg(ctx, cell)
                    + self.arena.text(",")
                    + (if has_predecessor(&pos) {
                        self.arena.line()
                    } else if has_predecessor(&row_pos) {
                        self.arena.line_()
                    } else {
                        self.arena.nil()
                    });
            }
            doc += row_doc.group()
                + (if has_predecessor(&row_pos) {
                    self.arena.hardline()
                } else {
                    self.arena.nil()
                });
        }
        (doc.nest(self.config.tab_spaces as isize) + self.arena.hardline()).parens()
    }
}

pub fn is_table(node: FuncCall<'_>) -> bool {
    matches!(indent_func_name(node), Some("table") | Some("grid"))
}

fn is_formattable_table(node: FuncCall<'_>) -> bool {
    // 1. no comments
    // 2. ~~no spread args~~
    // 3. no named args or named args first then unnamed args
    // 4. has at least one pos arg
    // 5. ~~no table/grid.vline/hline/cell~~
    // 6. ~~if table/grid.header/footer present, they should appear before/after any unnamed args~~
    if has_comment_children(node.args().to_untyped()) {
        return false;
    }
    let mut seen_pos_arg = false;
    for (i, node) in get_parenthesized_args(node.args()).enumerate() {
        match node {
            Arg::Pos(_) => {
                seen_pos_arg = true;
                if let Some(func_call) = node.to_untyped().cast::<FuncCall>() {
                    if BLACK_LIST.contains(&func_name(func_call).as_str()) {
                        return false;
                    }
                }
            }
            Arg::Named(_) => {
                if seen_pos_arg {
                    return false;
                }
            }
            Arg::Spread(_) => return false,
        }
    }
    if !seen_pos_arg {
        return false;
    }
    true
}

fn is_header_or_footer(arg: Arg) -> bool {
    const HEADER_FOOTER: [&str; 4] = ["table.header", "table.footer", "grid.header", "grid.footer"];

    arg.to_untyped()
        .cast::<FuncCall>()
        .is_some_and(|func_call| HEADER_FOOTER.contains(&func_name(func_call).as_str()))
}

fn get_table_columns(node: FuncCall<'_>) -> Option<usize> {
    for node in node.args().items() {
        if let Arg::Named(name) = node {
            if name.name().as_str() == "columns" {
                if let Some(count) = name.expr().to_untyped().cast::<Int>() {
                    return Some(count.get() as usize);
                }
                if let Some(arr) = name.expr().to_untyped().cast::<Array>() {
                    return Some(arr.items().count());
                }
            }
        }
    }
    None
}
