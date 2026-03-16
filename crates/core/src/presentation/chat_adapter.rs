//! Chat adapter for CyberWiki.
//!
//! Translates free-form queries from an AI-Chat front end into structured
//! operations against the query engine.

use crate::application::query_engine::QueryEngine;
use crate::domain::knowledge_graph::KnowledgeGraph;

/// Stateless adapter bound to a read-only view of the graph.
pub struct ChatAdapter<'a> {
    engine: QueryEngine<'a>,
}

impl<'a> ChatAdapter<'a> {
    pub fn new(graph: &'a KnowledgeGraph) -> Self {
        Self {
            engine: QueryEngine::new(graph),
        }
    }
}
