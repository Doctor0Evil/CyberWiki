//! Taxonomy structures for CyberWiki.
//!
//! Defines hierarchical and faceted classification of terms, ensuring that
//! every documented object is placed in an explicit, scientifically grounded
//! context.

use serde::{Deserialize, Serialize};

use super::term_entity::EntityId;

/// A taxonomy node representing a classification label in the CyberWiki tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Taxon {
    pub id: TaxonId,
    pub label: String,
    pub description: String,
    pub parent: Option<TaxonId>,
}

/// Mapping between terms and the taxonomy nodes that classify them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermTaxonomyMembership {
    pub term_id: EntityId,
    pub taxon_id: TaxonId,
}
