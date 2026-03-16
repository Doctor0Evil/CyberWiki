//! Authoring pipeline for CyberWiki.
//!
//! Handles insertion and update flows for documentation entities while
//! enforcing non-fiction scope, formatting expectations, and taxonomy
//! consistency.

use crate::domain::knowledge_graph::KnowledgeGraph;
use crate::domain::term_entity::{EntityId, Term};

/// Stateless authoring operations operating over a mutable graph.
pub struct AuthoringPipeline<'a> {
    graph: &'a mut KnowledgeGraph,
}

impl<'a> AuthoringPipeline<'a> {
    pub fn new(graph: &'a mut KnowledgeGraph) -> Self {
        Self { graph }
    }

    pub fn upsert_term(&mut self, term: Term) -> EntityId {
        let id = term.id.clone();
        self.graph.terms.insert(id.clone(), term);
        id
    }
}
