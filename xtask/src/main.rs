//! Workspace automation: `cargo xtask ci` and `cargo xtask check-deps`.

mod deps;

use std::path::Path;
use std::process::{Command, ExitCode};

/// One `cargo` invocation in the CI sequence.
const CARGO_STEPS: [&[&str]; 3] = [
    &["fmt", "--all", "--", "--check"],
    &[
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ],
    &["test", "--workspace"],
];

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("check-deps") => report(run_check_deps()),
        Some("ci") => report(ci()),
        _ => {
            eprintln!("usage: cargo xtask <ci|check-deps>");
            ExitCode::from(2)
        }
    }
}

/// Prints a failure, if any, and converts the outcome to an exit code.
fn report(result: Result<(), String>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

/// Runs the dependency guard, listing each violating edge.
fn run_check_deps() -> Result<(), String> {
    deps::check_deps().map_err(|violations| {
        let edges: Vec<String> = violations.iter().map(|v| format!("  {v}")).collect();
        format!("dependency violations:\n{}", edges.join("\n"))
    })
}

/// Runs every CI step in order, stopping at the first failure.
fn ci() -> Result<(), String> {
    for args in CARGO_STEPS {
        run_cargo(args)?;
    }
    run_check_deps()?;
    if Path::new("content/text").is_dir() {
        run_cargo(&["run", "-q", "-p", "ach_tools", "--", "text", "lint"])?;
    }
    Ok(())
}

/// Runs `cargo <args>`, failing if it exits unsuccessfully.
fn run_cargo(args: &[&str]) -> Result<(), String> {
    eprintln!("==> cargo {}", args.join(" "));
    let status = Command::new("cargo")
        .args(args)
        .status()
        .map_err(|e| format!("cannot run cargo: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`cargo {}` failed", args.join(" ")))
    }
}
