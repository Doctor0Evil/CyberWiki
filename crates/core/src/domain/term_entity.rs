//! Term and document entities for CyberWiki.
//!
//! These entities represent non-fictional concepts, devices, protocols,
//! and standards in cybernetics, BCI, EEG, nanoswarm engineering, and
//! related governance frameworks.

use serde::{Deserialize, Serialize};

/// Unique identifier for any entity tracked in the CyberWiki graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub String);

/// Classification of a CyberWiki term or document.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityKind {
    BciDevice,
    EegAcquisitionSystem,
    SignalProcessingPipeline,
    HumanMachineInterface,
    ControlProtocol,
    CyberPhysicalSystem,
    RegulatoryStandard,
    SafetyProfile,
    GovernanceModel,
    NanoswarmArchitecture,
    CryptographicPrimitive,
    DataSchema,
    Other(String),
}

/// A canonical, non-fictional term describing a real-world object or concept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    pub id: EntityId,
    pub name: String,
    pub kind: EntityKind,
    pub short_definition: String,
    pub long_description: String,
    pub primary_domain: String,
    pub tags: Vec<String>,
}

/// A documentation artifact such as a specification page, standard summary,
/// architecture note, or integration guide.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: EntityId,
    pub title: String,
    pub summary: String,
    pub body_markdown: String,
    pub related_terms: Vec<EntityId>,
    pub tags: Vec<String>,
}
