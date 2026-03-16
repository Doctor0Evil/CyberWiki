//! Bridge between the HTTP layer and the CyberWiki core crate.

use cyberwiki_core::domain::knowledge_graph::KnowledgeGraph;

/// Bridge type holding references to the knowledge graph and higher-level
/// services exposed to the API layer.
pub struct CoreBridge {
    pub graph: KnowledgeGraph,
}

impl CoreBridge {
    pub fn new() -> Self {
        Self {
            graph: KnowledgeGraph::new(),
        }
    }
}
