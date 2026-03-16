//! Provenance and audit structures for CyberWiki.
//!
//! These types encode source reliability, review chains, and cryptographic
//! attestations for every term and document stored in the graph.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use super::term_entity::EntityId;

/// Identifier for a provenance record.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProvenanceId(pub String);

/// Level of trust assigned to a source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceTrustLevel {
    RegulatoryBody,
    PeerReviewedJournal,
    IndustrialStandardBody,
    ManufacturerDocumentation,
    AcademicPreprint,
    ExpertReview,
    Unverified,
}

/// A source from which factual content was derived.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub uri: Option<String>,
    pub trust_level: SourceTrustLevel,
}

/// A cryptographic or process-level attestation for a change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    pub attestor_id: String,
    pub method: String,
    pub signature: Option<String>,
    pub timestamp: SystemTime,
}

/// Provenance record linking an entity to its sources and review trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    pub id: ProvenanceId,
    pub entity_id: EntityId,
    pub sources: Vec<Source>,
    pub attestations: Vec<Attestation>,
    pub created_at: SystemTime,
    pub last_verified_at: Option<SystemTime>,
}
