use typst_syntax::{Source, SyntaxNode, ast::*};
use typstyle_core::{Config, FormatFacts, FunctionFact, FunctionHint, Typstyle};

fn format_with_facts(source: &str, facts: FormatFacts) -> String {
    Typstyle::new(Config::new())
        .with_experimental_facts(facts)
        .format_source(Source::detached(source))
        .render()
        .unwrap()
}

#[test]
fn formats_named_table_like_wrapper_as_table() {
    let facts = FormatFacts::new().with_function_fact(FunctionFact::by_name(
        "my-table",
        FunctionHint::table_like(None),
    ));

    let formatted = format_with_facts("#my-table(columns: 2, [1], [2], [3], [4])", facts);

    assert_eq!(
        formatted,
        "#my-table(\n  columns: 2,\n  [1], [2],\n  [3], [4],\n)\n"
    );
}

#[test]
fn uses_table_like_fact_columns_when_call_has_no_columns_argument() {
    let facts = FormatFacts::new().with_function_fact(FunctionFact::by_name(
        "two-col-table",
        FunctionHint::table_like(Some(2)),
    ));

    let formatted = format_with_facts("#two-col-table([1], [2], [3], [4])", facts);

    assert_eq!(formatted, "#two-col-table(\n  [1], [2],\n  [3], [4],\n)\n");
}

#[test]
fn normal_function_without_fact_keeps_normal_argument_layout() {
    let formatted = format_with_facts(
        "#my-table(columns: 2, [1], [2], [3], [4])",
        FormatFacts::new(),
    );

    assert_eq!(formatted, "#my-table(columns: 2, [1], [2], [3], [4])\n");
}

#[test]
fn exact_span_fact_overrides_name_fact() {
    let source = Source::detached("#same([1], [2], [3], [4])");
    let same_call = find_func_call_by_callee(source.root(), "same");
    let facts = FormatFacts::new()
        .with_function_fact(FunctionFact::by_name(
            "same",
            FunctionHint::table_like(Some(1)),
        ))
        .with_function_fact(FunctionFact::by_span(
            same_call.callee().to_untyped().span(),
            FunctionHint::table_like(Some(2)),
        ));

    let formatted = Typstyle::new(Config::new())
        .with_experimental_facts(facts)
        .format_source(source)
        .render()
        .unwrap();

    assert_eq!(formatted, "#same(\n  [1], [2],\n  [3], [4],\n)\n");
}

#[test]
fn qualified_name_fact_matches_callee_path() {
    let facts = FormatFacts::new().with_function_fact(FunctionFact::by_name(
        "pkg.my-table",
        FunctionHint::table_like(Some(2)),
    ));

    let formatted = format_with_facts("#pkg.my-table([1], [2], [3], [4])", facts);

    assert_eq!(formatted, "#pkg.my-table(\n  [1], [2],\n  [3], [4],\n)\n");
}

#[test]
fn partial_formatting_uses_experimental_facts() {
    let source = Source::detached("#two-col-table([1], [2], [3], [4])");
    let facts = FormatFacts::new().with_function_fact(FunctionFact::by_name(
        "two-col-table",
        FunctionHint::table_like(Some(2)),
    ));
    let result = Typstyle::new(Config::new())
        .with_experimental_facts(facts)
        .format_source_range(source.clone(), 0..source.text().len())
        .unwrap();

    assert_eq!(result.source_range, 0..source.text().len());
    assert_eq!(
        result.content,
        "#two-col-table(\n  [1], [2],\n  [3], [4],\n)"
    );
}

fn find_func_call_by_callee<'a>(node: &'a SyntaxNode, callee: &str) -> FuncCall<'a> {
    find_func_call_by_callee_impl(node, callee)
        .unwrap_or_else(|| panic!("missing call to `{callee}`"))
}

fn find_func_call_by_callee_impl<'a>(node: &'a SyntaxNode, callee: &str) -> Option<FuncCall<'a>> {
    if let Some(func_call) = node.cast::<FuncCall>()
        && matches!(func_call.callee(), Expr::Ident(ident) if ident.as_str() == callee)
    {
        return Some(func_call);
    }

    node.children()
        .find_map(|child| find_func_call_by_callee_impl(child, callee))
}
