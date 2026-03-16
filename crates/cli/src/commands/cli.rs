//! Top-level CLI wiring for CyberWiki.

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{query::QueryCmd, validate::ValidateCmd};

#[derive(Debug, Parser)]
#[command(
    name = "cyberwiki",
    about = "Non-fictional cybernetics documentation assistant",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run validation over the CyberWiki corpus.
    Validate(ValidateCmd),
    /// Run query operations over the CyberWiki corpus.
    Query(QueryCmd),
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Validate(cmd) => cmd.run(),
            Commands::Query(cmd) => cmd.run(),
        }
    }
}
