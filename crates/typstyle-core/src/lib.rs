pub mod attr;
pub mod ext;
pub mod facts;
pub mod liteval;
pub mod partial;
pub mod pretty;

mod config;
mod utils;

pub use attr::AttrStore;
pub use config::Config;
pub use facts::{FormatFacts, FunctionFact, FunctionHint, FunctionQuery, TableLikeHint};
use pretty::{PrettyPrinter, prelude::*};
use thiserror::Error;
use typst_syntax::{Source, SyntaxNode};

use crate::utils::indent_4_to_2;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The document has syntax errors")]
    SyntaxError,
    #[error("An error occurred while rendering the document")]
    RenderError,
}

/// Main struct for Typst formatting.
#[derive(Debug, Clone, Default)]
pub struct Typstyle {
    config: Config,
    experimental_facts: FormatFacts,
}

impl Typstyle {
    /// Creates a new `Typstyle` with the given style configuration.
    pub fn new(config: Config) -> Self {
        Self {
            config,
            experimental_facts: FormatFacts::default(),
        }
    }

    /// Creates a new `Typstyle` with experimental semantic facts.
    ///
    /// This API is intended for LSP integrations and advanced callers. The facts are source-specific
    /// hints such as "this function should be formatted like a table".
    pub fn with_experimental_facts(mut self, facts: FormatFacts) -> Self {
        self.experimental_facts = facts;
        self
    }

    /// Prepares a text string for formatting.
    pub fn format_text(&self, text: impl Into<String>) -> Formatter<'_> {
        // We should ensure that the source tree is spanned.
        self.format_source(Source::detached(text.into()))
    }

    /// Prepares a source for formatting.
    pub fn format_source(&self, source: Source) -> Formatter<'_> {
        Formatter::new(self.config.clone(), self.experimental_facts.clone(), source)
    }
}

/// Handles the formatting of a specific Typst source.
pub struct Formatter<'a> {
    source: Source,
    printer: PrettyPrinter<'a>,
}

impl<'a> Formatter<'a> {
    fn new(config: Config, facts: FormatFacts, source: Source) -> Self {
        let attr_store = AttrStore::new(source.root());
        let printer = PrettyPrinter::new(config, attr_store, facts);
        Self { source, printer }
    }

    /// Renders the document's pretty IR.
    pub fn render_ir(&'a self) -> Result<String, Error> {
        let doc = self.build_doc()?;
        Ok(indent_4_to_2(&format!("{doc:#?}")))
    }

    /// Renders the formatted document to a string.
    pub fn render(&'a self) -> Result<String, Error> {
        let doc = self.build_doc()?;
        let mut buf = String::new();
        doc.render_fmt(self.printer.config().max_width, &mut buf)
            .map_err(|_| Error::RenderError)?;
        let result = utils::strip_trailing_whitespace(&buf);
        Ok(result)
    }

    fn build_doc(&'a self) -> Result<ArenaDoc<'a>, Error> {
        let root = self.source.root();
        if root.erroneous() {
            return Err(Error::SyntaxError);
        }
        let markup = root.cast().unwrap();
        let doc = self.printer.convert_markup(Default::default(), markup);
        Ok(doc)
    }
}

/// Formats a `SyntaxNode` as a debug AST string with 2-space indentation.
pub fn format_ast(root: &SyntaxNode) -> String {
    indent_4_to_2(&format!("{root:#?}"))
}

#[cfg(feature = "mapping")]
mod ast_mapping;
#[cfg(feature = "mapping")]
pub use ast_mapping::{SpanMapping, format_ast_with_mapping};
