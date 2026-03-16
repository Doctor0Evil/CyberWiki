//! Knowledge graph container for CyberWiki.
//!
//! Maintains collections of terms, documents, taxonomy links, and provenance
//! records with simple, deterministic lookup and traversal operations.

use std::collections::HashMap;

use super::provenance::ProvenanceRecord;
use super::taxonomy::{Taxon, TermTaxonomyMembership};
use super::term_entity::{Document, EntityId, Term};

/// In-memory representation of the CyberWiki knowledge graph.
#[derive(Default)]
pub struct KnowledgeGraph {
    pub terms: HashMap<EntityId, Term>,
    pub documents: HashMap<EntityId, Document>,
    pub taxa: HashMap<String, Taxon>,
    pub memberships: Vec<TermTaxonomyMembership>,
    pub provenance: HashMap<String, ProvenanceRecord>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self::default()
    }
}
