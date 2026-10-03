//! The `ach` command-line entry point. See Execution Plan §4.1.

use ach_tools::terrain::{self, TerrainCommand};
use clap::{Parser, Subcommand};

/// Achlydesa command-line tools.
#[derive(Parser)]
#[command(name = "ach", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Terrain import, inspection and preview.
    Terrain {
        #[command(subcommand)]
        command: TerrainCommand,
    },
}

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Command::Terrain { command } => terrain::run(&command),
    }
}
