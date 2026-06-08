use typst_syntax::Source;
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
