//! Terrain tooling for the `ach` CLI: import real elevation tiles, inspect and preview the result.
//! Implements Execution Plan §4.5 (terrain source) and ADR-0100 (Copernicus GLO-30).

pub mod import;
pub mod info;
pub mod preview;
pub mod source;

/// `ach terrain <subcommand>`.
#[derive(clap::Subcommand, Debug)]
pub enum TerrainCommand {
    /// Convert source tiles into the game's chunked terrain.
    Import(import::ImportArgs),
    /// Print a summary of a terrain directory.
    Info(info::InfoArgs),
    /// Render a hillshaded PNG with sites and routes.
    Preview(preview::PreviewArgs),
}

/// Runs a terrain subcommand.
pub fn run(command: &TerrainCommand) -> anyhow::Result<()> {
    match command {
        TerrainCommand::Import(a) => import::run(a),
        TerrainCommand::Info(a) => info::run(a),
        TerrainCommand::Preview(a) => preview::run(a),
    }
}
