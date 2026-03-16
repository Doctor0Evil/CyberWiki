//! Validation commands for CyberWiki CLI.
//!
//! Executes structural checks over the knowledge graph and reports any
//! violations of editorial or structural rules.

use anyhow::Result;
use clap::Args;

use cyberwiki_core::application::validation_rules::{ValidationOutcome, ValidationRules};
use cyberwiki_core::domain::knowledge_graph::KnowledgeGraph;

#[derive(Debug, Args)]
pub struct ValidateCmd;

impl ValidateCmd {
    pub fn run(&self) -> Result<()> {
        let graph = KnowledgeGraph::new();

        for term in graph.terms.values() {
            match ValidationRules::validate_term(term) {
                ValidationOutcome::Accepted => {}
                ValidationOutcome::Rejected { reason } => {
                    eprintln!("Validation failed for term {}: {}", term.name, reason);
                }
            }
        }

        Ok(())
    }
}
