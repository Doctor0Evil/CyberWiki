//! Markdown renderer for CyberWiki entities.
//!
//! Produces deterministic, style-guided markdown fragments from terms and
//! documents for use in documentation pages and chat responses.

use crate::domain::term_entity::{Document, Term};

pub struct MarkdownRenderer;

impl MarkdownRenderer {
    pub fn render_term(term: &Term) -> String {
        format!(
            "# {}\n\n{}\n",
            term.name, term.long_description
        )
    }

    pub fn render_document(doc: &Document) -> String {
        format!("# {}\n\n{}\n\n{}", doc.title, doc.summary, doc.body_markdown)
    }
}
