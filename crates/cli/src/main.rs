//! CyberWiki CLI entry point.
//!
//! Binds command-line arguments to CyberWiki core operations, enabling
//! scripted validation and analysis of the documentation corpus.

mod commands;

use anyhow::Result;
use clap::Parser;

use crate::commands::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.exec()
}
