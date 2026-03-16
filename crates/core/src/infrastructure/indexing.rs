//! Indexing utilities for CyberWiki.
//!
//! Supports auxiliary indexes used by query engines and interactive clients.

use std::collections::HashMap;

use crate::domain::knowledge_graph::KnowledgeGraph;
use crate::domain::term_entity::EntityId;

/// A simple index mapping tags to term identifiers.
pub struct TagIndex {
    pub terms_by_tag: HashMap<String, Vec<EntityId>>,
}

impl TagIndex {
    pub fn build(graph: &KnowledgeGraph) -> Self {
        let mut terms_by_tag: HashMap<String, Vec<EntityId>> = HashMap::new();

        for (id, term) in &graph.terms {
            for tag in &term.tags {
                terms_by_tag
                    .entry(tag.to_lowercase())
                    .or_default()
                    .push(id.clone());
            }
        }

        TagIndex { terms_by_tag }
    }
}
