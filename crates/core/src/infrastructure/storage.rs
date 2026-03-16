//! Storage abstractions for CyberWiki.
//!
//! Defines traits for persisting and loading the knowledge graph using
//! pluggable backends.

use crate::domain::knowledge_graph::KnowledgeGraph;

pub trait GraphStorage {
    fn load(&self) -> anyhow::Result<KnowledgeGraph>;
    fn save(&self, graph: &KnowledgeGraph) -> anyhow::Result<()>;
}
