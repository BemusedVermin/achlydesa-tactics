//! Dependency guard: checks workspace edges against `xtask/allowed_deps.toml`.
//!
//! The table is the architecture (Execution Plan §4.1). Internal edges are
//! checked against `[internal]`; every other dependency against `[external]`.

use std::collections::BTreeSet;
use std::process::Command;

use serde_json::Value;
use toml::Table;

/// The allow-list, embedded so the guard cannot run against a stale copy.
const ALLOWED: &str = include_str!("../allowed_deps.toml");

/// Wildcard entry that matches any package.
const ANY: &str = "*";

/// Wildcard entry that matches any package under `spikes/`.
const ANY_SPIKE: &str = "spikes/*";

/// Runs `cargo metadata` and checks every workspace package.
///
/// Returns `Ok(())` when the graph is allowed, otherwise the violating edges.
pub fn check_deps() -> Result<(), Vec<String>> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .output()
        .map_err(|e| vec![format!("cannot run cargo metadata: {e}")])?;
    if !output.status.success() {
        return Err(vec![format!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )]);
    }
    let metadata: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| vec![format!("invalid cargo metadata output: {e}")])?;
    let allowed: Table = ALLOWED
        .parse()
        .map_err(|e| vec![format!("invalid allowed_deps.toml: {e}")])?;
    let violations = violations(&metadata, &allowed);
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

/// A workspace package, reduced to what the guard needs.
struct Member<'a> {
    name: &'a str,
    is_spike: bool,
    deps: Vec<&'a str>,
}

/// Returns one message per violating edge in `metadata`, in a stable order.
fn violations(metadata: &Value, allowed: &Table) -> Vec<String> {
    let members = members(metadata);
    let names: BTreeSet<&str> = members.iter().map(|m| m.name).collect();
    let spikes: BTreeSet<&str> = members
        .iter()
        .filter(|m| m.is_spike)
        .map(|m| m.name)
        .collect();
    let mut out = Vec::new();
    for member in &members {
        for dep in &member.deps {
            let problem = if names.contains(dep) {
                internal_problem(member, dep, spikes.contains(dep), allowed)
            } else {
                external_problem(member, dep, allowed)
            };
            if let Some(problem) = problem {
                out.push(format!("{} -> {dep}: {problem}", member.name));
            }
        }
    }
    out
}

/// Extracts the workspace members from `cargo metadata` output.
fn members(metadata: &Value) -> Vec<Member<'_>> {
    let ids: BTreeSet<&str> = strings(&metadata["workspace_members"]).collect();
    let mut members: Vec<Member<'_>> = array(&metadata["packages"])
        .filter(|p| p["id"].as_str().is_some_and(|id| ids.contains(id)))
        .filter_map(|p| {
            let name = p["name"].as_str()?;
            let manifest = p["manifest_path"].as_str()?;
            let mut deps: Vec<&str> = array(&p["dependencies"])
                .filter_map(|d| d["name"].as_str())
                .collect();
            deps.sort_unstable();
            deps.dedup();
            Some(Member {
                name,
                is_spike: manifest.contains("/spikes/"),
                deps,
            })
        })
        .collect();
    members.sort_by_key(|m| m.name);
    members
}

/// Iterates a JSON array, yielding nothing if the value is not one.
fn array(value: &Value) -> impl Iterator<Item = &Value> {
    value.as_array().into_iter().flatten()
}

/// Iterates the string elements of a JSON array.
fn strings(value: &Value) -> impl Iterator<Item = &str> {
    array(value).filter_map(Value::as_str)
}

/// Checks an edge between two workspace packages.
fn internal_problem(
    from: &Member<'_>,
    to: &str,
    to_is_spike: bool,
    allowed: &Table,
) -> Option<String> {
    if from.is_spike {
        return None;
    }
    if to_is_spike {
        return Some("workspace crates may not depend on spikes".into());
    }
    let permitted = allowed.get("internal").and_then(|t| t.get(from.name));
    if allows(permitted, &[to]) {
        None
    } else {
        Some("internal dependency not in [internal] allow-list".into())
    }
}

/// Checks an edge to a crate outside the workspace.
fn external_problem(from: &Member<'_>, to: &str, allowed: &Table) -> Option<String> {
    let permitted = allowed.get("external").and_then(|t| t.get(to));
    let mut keys = vec![from.name];
    if from.is_spike {
        keys.push(ANY_SPIKE);
    }
    if allows(permitted, &keys) {
        None
    } else {
        Some("external dependency not in [external] allow-list".into())
    }
}

/// True if the TOML array `permitted` contains `ANY` or one of `keys`.
fn allows(permitted: Option<&toml::Value>, keys: &[&str]) -> bool {
    permitted
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_str)
        .any(|entry| entry == ANY || keys.contains(&entry))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn table() -> Table {
        ALLOWED.parse().expect("invariant: allow-list parses")
    }

    fn package(name: &str, dir: &str, deps: &[&str]) -> Value {
        json!({
            "id": name,
            "name": name,
            "manifest_path": format!("/repo/{dir}/{name}/Cargo.toml"),
            "dependencies": deps.iter().map(|d| json!({ "name": d })).collect::<Vec<_>>(),
        })
    }

    fn metadata(packages: Vec<Value>) -> Value {
        let ids: Vec<Value> = packages.iter().map(|p| p["id"].clone()).collect();
        json!({ "workspace_members": ids, "packages": packages })
    }

    #[test]
    fn allowed_edges_pass() {
        let m = metadata(vec![
            package("ach_core", "crates", &[]),
            package("ach_world", "crates", &["ach_core", "serde"]),
        ]);
        assert!(violations(&m, &table()).is_empty());
    }

    #[test]
    fn forbidden_internal_edge_is_named() {
        let m = metadata(vec![
            package("ach_text", "crates", &[]),
            package("ach_sim", "crates", &["ach_text"]),
        ]);
        assert_eq!(
            violations(&m, &table()),
            ["ach_sim -> ach_text: internal dependency not in [internal] allow-list"]
        );
    }

    #[test]
    fn unlisted_and_misplaced_external_deps_fail() {
        let m = metadata(vec![package("ach_core", "crates", &["rand", "clap"])]);
        assert_eq!(violations(&m, &table()).len(), 2);
    }

    #[test]
    fn spikes_may_use_workspace_crates_and_spike_externals() {
        let m = metadata(vec![
            package("ach_core", "crates", &[]),
            package("p1_s1", "spikes", &["ach_core", "image"]),
        ]);
        assert!(violations(&m, &table()).is_empty());
    }

    #[test]
    fn workspace_crates_may_not_use_spikes() {
        let m = metadata(vec![
            package("p1_s1", "spikes", &[]),
            package("ach_tools", "crates", &["p1_s1"]),
        ]);
        assert_eq!(violations(&m, &table()).len(), 1);
    }
}
