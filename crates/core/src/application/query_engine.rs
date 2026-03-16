//! Query engine for CyberWiki.
//!
//! Provides structured lookup operations over terms, documents, and taxonomy
//! with deterministic behavior suitable for AI-Chat integration.

use crate::domain::knowledge_graph::KnowledgeGraph;
use crate::domain::term_entity::{EntityKind, Term};

/// Query operations over the CyberWiki graph.
pub struct QueryEngine<'a> {
    graph: &'a KnowledgeGraph,
}

impl<'a> QueryEngine<'a> {
    pub fn new(graph: &'a KnowledgeGraph) -> Self {
        Self { graph }
    }

    pub fn find_term_by_name(&self, name: &str) -> Option<&Term> {
        self.graph
            .terms
            .values()
            .find(|t| t.name.eq_ignore_ascii_case(name))
    }

    pub fn list_terms_by_kind(&self, kind: EntityKind) -> Vec<&Term> {
        self.graph
            .terms
            .values()
            .filter(|t| t.kind == kind)
            .collect()
    }
}
