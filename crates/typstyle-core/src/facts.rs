use typst_syntax::Span;

/// Experimental semantic facts supplied by an external analyzer.
///
/// This is intentionally separate from [`crate::Config`]: formatting options are stable user
/// preferences, while facts describe source-specific semantic knowledge such as "this wrapper
/// function forwards its arguments to `table`".
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FormatFacts {
    function_facts: Vec<FunctionFact>,
}

impl FormatFacts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_function_fact(&mut self, fact: FunctionFact) {
        self.function_facts.push(fact);
    }

    pub fn with_function_fact(mut self, fact: FunctionFact) -> Self {
        self.add_function_fact(fact);
        self
    }

    pub fn add_function_name_hint(&mut self, callee_name: impl Into<String>, hint: FunctionHint) {
        self.add_function_fact(FunctionFact::by_name(callee_name, hint));
    }

    pub fn function_hint(&self, query: FunctionQuery<'_>) -> Option<FunctionHint> {
        self.function_facts
            .iter()
            .find(|fact| fact.matches_span(query.callee_span))
            .or_else(|| {
                self.function_facts
                    .iter()
                    .find(|fact| fact.matches_path(query))
            })
            .map(|fact| fact.hint)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionFact {
    pub callee_span: Option<Span>,
    pub callee_name: Option<String>,
    pub hint: FunctionHint,
}

impl FunctionFact {
    pub fn by_name(callee_name: impl Into<String>, hint: FunctionHint) -> Self {
        Self {
            callee_span: None,
            callee_name: Some(callee_name.into()),
            hint,
        }
    }

    pub fn by_span(callee_span: Span, hint: FunctionHint) -> Self {
        Self {
            callee_span: Some(callee_span),
            callee_name: None,
            hint,
        }
    }

    fn matches_span(&self, callee_span: Option<Span>) -> bool {
        self.callee_span
            .zip(callee_span)
            .is_some_and(|(fact_span, query_span)| fact_span == query_span)
    }

    fn matches_path(&self, query: FunctionQuery<'_>) -> bool {
        let Some(callee_name) = self.callee_name.as_deref() else {
            return false;
        };
        query.callee_path == Some(callee_name) || query.callee_name == Some(callee_name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionQuery<'a> {
    pub callee_span: Option<Span>,
    pub callee_name: Option<&'a str>,
    pub callee_path: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionHint {
    TableLike(TableLikeHint),
    GridLike(TableLikeHint),
}

impl FunctionHint {
    pub fn table_like(columns: Option<usize>) -> Self {
        Self::TableLike(TableLikeHint { columns })
    }

    pub fn grid_like(columns: Option<usize>) -> Self {
        Self::GridLike(TableLikeHint { columns })
    }

    pub fn table_like_hint(self) -> TableLikeHint {
        match self {
            Self::TableLike(hint) | Self::GridLike(hint) => hint,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableLikeHint {
    pub columns: Option<usize>,
}
