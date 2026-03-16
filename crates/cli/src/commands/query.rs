//! Query commands for CyberWiki CLI.
//!
//! Allows terminal-based retrieval of specific terms or lists by kind,
//! suitable for batch reporting and offline analysis.

use anyhow::Result;
use clap::Args;

use cyberwiki_core::application::query_engine::QueryEngine;
use cyberwiki_core::domain::knowledge_graph::KnowledgeGraph;
use cyberwiki_core::domain::term_entity::EntityKind;

#[derive(Debug, Args)]
pub struct QueryCmd {
    /// Optional exact term name to retrieve.
    #[arg(long)]
    pub name: Option<String>,

    /// Optional filter by entity kind.
    #[arg(long)]
    pub kind: Option<String>,
}

impl QueryCmd {
    pub fn run(&self) -> Result<()> {
        let graph = KnowledgeGraph::new();
        let engine = QueryEngine::new(&graph);

        if let Some(name) = &self.name {
            if let Some(term) = engine.find_term_by_name(name) {
                println!("{}", term.name);
            }
        }

        if let Some(kind) = &self.kind {
            if let Ok(parsed) = parse_kind(kind) {
                let terms = engine.list_terms_by_kind(parsed);
                for term in terms {
                    println!("{}", term.name);
                }
            }
        }

        Ok(())
    }
}

fn parse_kind(input: &str) -> Result<EntityKind, ()> {
    let normalized = input.to_lowercase();
    let kind = match normalized.as_str() {
        "bci-device" => EntityKind::BciDevice,
        "eeg-system" => EntityKind::EegAcquisitionSystem,
        "signal-pipeline" => EntityKind::SignalProcessingPipeline,
        "hmi" => EntityKind::HumanMachineInterface,
        "control-protocol" => EntityKind::ControlProtocol,
        "cyber-physical-system" => EntityKind::CyberPhysicalSystem,
        "regulatory-standard" => EntityKind::RegulatoryStandard,
        "safety-profile" => EntityKind::SafetyProfile,
        "governance-model" => EntityKind::GovernanceModel,
        "nanoswarm-architecture" => EntityKind::NanoswarmArchitecture,
        "crypto-primitive" => EntityKind::CryptographicPrimitive,
        "data-schema" => EntityKind::DataSchema,
        _ => return Err(()),
    };

    Ok(kind)
}
