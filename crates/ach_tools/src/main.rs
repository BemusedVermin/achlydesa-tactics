//! The `ach` command-line entry point. See Execution Plan §4.1.

use clap::Parser;

/// Achlydesa command-line tools.
#[derive(Parser)]
#[command(name = "ach", version)]
struct Cli {}

fn main() {
    Cli::parse();
}
