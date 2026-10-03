#!/usr/bin/env python3
"""Achlydesa task helper: specs in docs/tasks/ <-> GitHub issues, Project board, and pull requests.

Standard library only; every GitHub call goes through the `gh` CLI, which must be authenticated.
Project writes need a token with the `project` scope (in Actions: GH_TOKEN = ACHLYDESA_BOT_TOKEN).

Commands:
  specs [--check]                    Print parsed specs (offline); --check validates sections, deps, and cycles.
  bootstrap --owner O --reviewer R   Create labels, the Project, its fields, one issue per spec; sync bodies and fields.
            [--title T] [--dry-run]  Idempotent: re-run after editing spec headers.
  graph [--phase P1] [--offline] [--output F | --issue]
                                     Dependency graph (Mermaid), colored by board status.
  check                              Verify config, labels, Status options, and that every spec has an issue.
  issue ID                           Print the task's issue number and URL.
  runs-on ID                         Print the spec's "Runs on" value (either | local | human).
  ready ID                           Exit 0 if every dependency's issue is closed, else list the blockers and exit 1.
  stage ID STAGE                     Set the board Status: todo | in-progress | awaiting-review | changes-requested | done.
  pr ID [--json]                     Show the open task PR (task/ID-*), its review decision, and whether a revision is requested.
  handoff ID                         Require an open task PR, set Awaiting review, print the PR URL.
  review ID                          Print review feedback on the open task PR since the agent's last "## Revision" comment.
  comments ID                        Print human comments on the task issue (extra requirements, e.g. after G1).
"""
from __future__ import annotations

import argparse
import json
import re
import shlex
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TASKS_DIR = ROOT / "docs" / "tasks"
CONFIG_PATH = ROOT / ".github" / "project.json"

STAGES = {
    "todo": "Todo",
    "in-progress": "In Progress",
    "awaiting-review": "Awaiting review",
    "changes-requested": "Changes requested",
    "done": "Done",
}
LABELS = {
    "task": ("0E8A16", "A task with a spec in docs/tasks/"),
    "human": ("D93F0B", "A step only Liam can do"),
    "agent:run": ("5319E7", "Add to run this task with the GitHub agent"),
    "changes-requested": ("B60205", "Revision requested (fallback when the PR author is not a bot)"),
    "runs:either": ("C2E0C6", "Can run locally or on GitHub"),
    "runs:local": ("FBCA04", "Needs local data; run with /task locally"),
    "runs:human": ("BFD4F2", "Human step"),
    "phase:P0": ("EDEDED", "Phase 0: canon"),
    "phase:P1": ("EDEDED", "Phase 1: headless truth"),
    "phase:gate": ("EDEDED", "Human setup, decisions, and feeling gates"),
}
FIELDS = {  # name -> options (None = TEXT)
    "Task ID": None,
    "Phase": [f"P{n}" for n in range(14)] + ["Gate"],
    "Agent": ["canon-editor", "task-implementer", "spec-writer", "Liam"],
    "Size": ["S", "M", "L", "XL"],
    "Runs on": ["either", "local", "human"],
}
DRY_RUN = False


# ---------------------------------------------------------------- specs (offline)

@dataclass
class Spec:
    id: str
    title: str
    agent: str
    size: str
    depends: list[str]
    slug: str
    runs_on: str
    goal: str
    path: Path

    @property
    def phase(self) -> str:
        prefix = self.id.split("-")[0]
        return prefix if re.fullmatch(r"P\d+", prefix) else "Gate"

    @property
    def issue_title(self) -> str:
        return f"{self.id} — {self.title}"


TITLE_RE = re.compile(r"^# (?P<id>[A-Z0-9]+(?:-[A-Z0-9]+)?) — (?P<title>.+)$")
FIELD_RE = re.compile(r"\*\*([^*]+):\*\* ([^·]+?)(?= ·|$)")


def parse_spec(path: Path) -> Spec:
    lines = path.read_text(encoding="utf-8").splitlines()
    m = TITLE_RE.match(lines[0])
    if not m:
        raise SystemExit(f"{path}: first line must be '# <ID> — <title>'")
    header = next((l for l in lines[:6] if l.startswith("**Agent:**")), None)
    if header is None:
        raise SystemExit(f"{path}: missing '**Agent:** …' header line")
    f = {k.strip(): v.strip() for k, v in FIELD_RE.findall(header)}
    for key in ("Agent", "Size", "Depends on", "Branch slug", "Runs on"):
        if key not in f:
            raise SystemExit(f"{path}: header lacks '{key}'")
    deps = [] if f["Depends on"] in ("—", "-", "") else [d.strip() for d in f["Depends on"].split(",")]
    text = "\n".join(lines)
    g = re.search(r"^## Goal\n(.*?)(?=^## )", text, re.S | re.M)
    goal = g.group(1).strip() if g else ""
    if m.group("id") != path.stem:
        raise SystemExit(f"{path}: ID '{m.group('id')}' does not match file name")
    return Spec(m.group("id"), m.group("title").strip(), f["Agent"], f["Size"], deps,
                f["Branch slug"], f["Runs on"], goal, path)


def index_order() -> list[str]:
    """Task IDs in the order of the index table in docs/tasks/README.md (issue creation order)."""
    readme = TASKS_DIR / "README.md"
    if not readme.exists():
        return []
    return re.findall(r"^\| ([A-Z0-9]+(?:-[A-Z0-9]+)?) \|", readme.read_text(encoding="utf-8"), re.M)


def load_specs() -> dict[str, Spec]:
    specs = {}
    order = {tid: i for i, tid in enumerate(index_order())}
    paths = sorted(TASKS_DIR.glob("*.md"), key=lambda p: (order.get(p.stem, len(order)), p.stem))
    for p in paths:
        if p.name in ("README.md", "TEMPLATE.md"):
            continue
        s = parse_spec(p)
        specs[s.id] = s
    for s in specs.values():
        for d in s.depends:
            if d not in specs:
                raise SystemExit(f"{s.id}: unknown dependency '{d}'")
    return specs


def spec_or_die(task_id: str) -> Spec:
    specs = load_specs()
    if task_id not in specs:
        raise SystemExit(f"Unknown task ID '{task_id}'. Known: {', '.join(specs)}")
    return specs[task_id]


# ---------------------------------------------------------------- gh wrapper

def gh(*args: str, mutate: bool = False, parse_json: bool = False, input_text: str | None = None):
    cmd = ["gh", *args]
    if mutate and DRY_RUN:
        print("DRY-RUN:", shlex.join(cmd), file=sys.stderr)
        return {} if parse_json else ""
    # gh always speaks UTF-8. Decode explicitly: Windows defaults to cp1252, which turns "—" into "â€”".
    res = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8", errors="replace", input=input_text)
    if res.returncode != 0:
        hint = ""
        if "unknown owner type" in res.stderr:
            hint = ("\nhint: `gh project` reports a token problem this way. The token needs scopes "
                    "project, read:org, and read:discussion (and must not be expired). Locally: "
                    "gh auth refresh -s project,read:org,read:discussion. For ACHLYDESA_BOT_TOKEN: edit the "
                    "token at https://github.com/settings/tokens and add those scopes.")
        raise SystemExit(f"gh failed: {shlex.join(cmd)}\n{res.stderr.strip()}{hint}")
    out = res.stdout.strip()
    return (json.loads(out) if out else {}) if parse_json else out


def load_config() -> dict:
    if not CONFIG_PATH.exists():
        raise SystemExit(f"{CONFIG_PATH.relative_to(ROOT)} not found. Run: gh_task.py bootstrap --owner … --reviewer …")
    return json.loads(CONFIG_PATH.read_text())


def repo_name() -> str:
    return gh("repo", "view", "--json", "nameWithOwner", "-q", ".nameWithOwner")


def default_branch() -> str:
    """The repository's default branch (e.g. master), so links and bases never assume a name."""
    return gh("repo", "view", "--json", "defaultBranchRef", "-q", ".defaultBranchRef.name") or "master"


TASK_TITLE_RE = re.compile(r"^([A-Z0-9]+(?:-[A-Z0-9]+)?)\s+[—–-]\s")


def task_issues(repo: str, warn: bool = True) -> dict[str, dict]:
    """Map task ID -> issue, from issues labeled `task`. If an ID has several issues, prefer the
    lowest-numbered open one (duplicates usually come from re-running bootstrap before a fix)."""
    items = gh("issue", "list", "--repo", repo, "--label", "task", "--state", "all", "--limit", "1000",
               "--json", "number,title,state,url,labels", parse_json=True) or []
    by_id: dict[str, list[dict]] = {}
    for it in items:
        m = TASK_TITLE_RE.match(it["title"])
        if m:
            by_id.setdefault(m.group(1), []).append(it)
    if warn and items and not by_id:
        print(f"warning: {len(items)} issues carry the `task` label but none has a title like "
              f"'P1-02 — …'. First title seen: {items[0]['title']!r}", file=sys.stderr)
    out = {}
    for tid, its in by_id.items():
        its.sort(key=lambda i: (i["state"] != "OPEN", i["number"]))
        out[tid] = its[0]
        if warn and len(its) > 1:
            print(f"warning: {tid} has {len(its)} issues ({', '.join('#' + str(i['number']) for i in its)}); "
                  f"using #{its[0]['number']}. Close the others as duplicates.", file=sys.stderr)
    return out


def issue_for(task_id: str, repo: str) -> dict:
    it = task_issues(repo).get(task_id)
    if not it:
        raise SystemExit(f"No issue for {task_id}. Run bootstrap.")
    return it


def project_fields(cfg: dict) -> dict[str, dict]:
    data = gh("project", "field-list", str(cfg["number"]), "--owner", cfg["owner"], "--format", "json",
              "--limit", "100", parse_json=True)
    return {f["name"]: f for f in data.get("fields", [])}


def project_items(cfg: dict) -> dict[int, str]:
    data = gh("project", "item-list", str(cfg["number"]), "--owner", cfg["owner"], "--format", "json",
              "--limit", "1000", parse_json=True)
    return {it["content"]["number"]: it["id"] for it in data.get("items", [])
            if it.get("content", {}).get("number") is not None}


def project_statuses(cfg: dict) -> dict[int, str]:
    """Issue number -> board Status (gh exposes single-select values under the lowercased field name)."""
    data = gh("project", "item-list", str(cfg["number"]), "--owner", cfg["owner"], "--format", "json",
              "--limit", "1000", parse_json=True)
    return {it["content"]["number"]: it.get("status") or "" for it in data.get("items", [])
            if it.get("content", {}).get("number") is not None}


def set_field(cfg: dict, fields: dict, item_id: str, name: str, value: str) -> None:
    f = fields.get(name)
    if f is None:
        raise SystemExit(f"Project field '{name}' missing. Run bootstrap or check.")
    base = ["project", "item-edit", "--id", item_id, "--project-id", cfg["project_id"], "--field-id", f["id"]]
    if "options" in f:
        opt = next((o for o in f.get("options", []) if o["name"] == value), None)
        if opt is None:
            if name == "Status":
                raise SystemExit(f"Status field has no option '{value}'. Add it in the Project settings (see docs/tasks/H-1.md).")
            print(f"warning: Project field '{name}' has no option '{value}'; left unset. Add the option in the "
                  f"Project settings, or delete the field and re-run bootstrap to recreate it with all options.",
                  file=sys.stderr)
            return
        gh(*base, "--single-select-option-id", opt["id"], mutate=True)
    else:
        gh(*base, "--text", value, mutate=True)


# ---------------------------------------------------------------- commands

REQUIRED_SECTIONS = {
    "agent": ["## Goal", "## Read first", "## Deliverables", "## Specification", "## Acceptance",
              "## Out of scope", "## Review focus"],
    "human": ["## Goal", "## Steps"],
}


def check_specs(specs: dict[str, Spec]) -> list[str]:
    problems = []
    for s in specs.values():
        text = s.path.read_text(encoding="utf-8")
        kind = "human" if s.runs_on == "human" else "agent"
        for sec in REQUIRED_SECTIONS[kind]:
            if not re.search(rf"^{re.escape(sec)}", text, re.M):
                problems.append(f"{s.id}: missing section '{sec}'")
        if s.runs_on not in ("either", "local", "human"):
            problems.append(f"{s.id}: Runs on must be either | local | human")
        if s.size not in ("S", "M", "L", "XL"):
            problems.append(f"{s.id}: Size must be S, M, L, or XL")
    # cycle detection (DFS with colors)
    color: dict[str, int] = {}
    def visit(t: str, stack: list[str]) -> None:
        color[t] = 1
        for d in specs[t].depends:
            if color.get(d) == 1:
                problems.append("dependency cycle: " + " -> ".join(stack + [t, d]))
            elif color.get(d) is None:
                visit(d, stack + [t])
        color[t] = 2
    for t in specs:
        if t not in color:
            visit(t, [])
    return problems


def cmd_specs(args) -> None:
    specs = load_specs()
    if getattr(args, "check", False):
        problems = check_specs(specs)
        if problems:
            print("\n".join("✗ " + p for p in problems))
            sys.exit(1)
        print(f"✓ {len(specs)} specs parse, dependencies resolve, no cycles, sections present.")
        return
    for s in specs.values():
        print(f"{s.id:7} {s.agent:17} {s.size:2} {s.runs_on:7} deps={','.join(s.depends) or '—':32} {s.title}")


def issue_body(s: Spec, repo: str, numbers: dict[str, int], branch: str) -> str:
    deps = ", ".join(f"#{numbers[d]} ({d})" if d in numbers else d for d in s.depends) or "—"
    how = ("This is a human step: follow the spec, then close this issue." if s.runs_on == "human" else
           f"Run locally with `/task {s.id}` in Claude Code" +
           ("" if s.runs_on == "local" else ", or add the `agent:run` label to run it on GitHub") + ".")
    return (f"**Spec:** https://github.com/{repo}/blob/{branch}/{s.path.relative_to(ROOT).as_posix()}\n"
            f"**Agent:** {s.agent} · **Size:** {s.size} · **Runs on:** {s.runs_on}\n"
            f"**Depends on:** {deps}\n\n"
            f"{s.goal}\n\n{how}\n\n"
            f"<sub>Generated by `tools/scripts/gh_task.py bootstrap`. Edit the spec, not this text. "
            f"Comments from maintainers on this issue are treated as additional requirements.</sub>\n")


def sync_dependencies(specs: dict[str, "Spec"], numbers: dict[str, int], repo: str) -> None:
    """Mirror spec dependencies into GitHub's native issue relationships (Relationships > Blocked by).
    Adds missing links and removes links to task issues the spec no longer lists. Non-task links are kept."""
    task_numbers = set(numbers.values())
    db_ids: dict[int, int] = {}

    def db_id(n: int) -> int:
        if n not in db_ids:
            db_ids[n] = int(gh("api", f"repos/{repo}/issues/{n}", "-q", ".id") or 0)
        return db_ids[n]

    for s in specs.values():
        n = numbers.get(s.id)
        if not n or n < 0:
            continue
        want = {numbers[d] for d in s.depends if numbers.get(d, -1) > 0}
        if DRY_RUN:
            for w in sorted(want):
                print(f"DRY-RUN: #{n} blocked by #{w}", file=sys.stderr)
            continue
        current = json.loads(gh("api", f"repos/{repo}/issues/{n}/dependencies/blocked_by", "--paginate") or "[]")
        have = {c["number"]: c["id"] for c in current}
        for w in sorted(want - set(have)):
            gh("api", "-X", "POST", f"repos/{repo}/issues/{n}/dependencies/blocked_by",
               "-F", f"issue_id={db_id(w)}", mutate=True)
        for extra in sorted(set(have) - want):
            if extra in task_numbers:
                gh("api", "-X", "DELETE", f"repos/{repo}/issues/{n}/dependencies/blocked_by/{have[extra]}", mutate=True)


def cmd_bootstrap(args) -> None:
    global DRY_RUN
    DRY_RUN = args.dry_run
    specs = load_specs()
    repo = repo_name()
    branch = default_branch()
    owner = args.owner

    labels = dict(LABELS)
    for sp in specs.values():
        labels.setdefault(f"phase:{sp.phase}", ("EDEDED", f"Phase {sp.phase[1:]}" if sp.phase != "Gate" else "Gates"))
    for name, (color, desc) in labels.items():
        gh("label", "create", name, "--repo", repo, "--color", color, "--description", desc, "--force", mutate=True)

    if CONFIG_PATH.exists():
        cfg = json.loads(CONFIG_PATH.read_text())
    else:
        proj = gh("project", "create", "--owner", owner, "--title", args.title, "--format", "json",
                  mutate=True, parse_json=True)
        cfg = {"owner": owner, "repo": repo, "number": proj.get("number", 0),
               "project_id": proj.get("id", "DRY"), "reviewer": args.reviewer}
        gh("project", "link", str(cfg["number"]), "--owner", owner, "--repo", repo, mutate=True)
        if not DRY_RUN:
            CONFIG_PATH.parent.mkdir(parents=True, exist_ok=True)
            CONFIG_PATH.write_text(json.dumps(cfg, indent=2) + "\n")
    if args.reviewer:
        cfg["reviewer"] = args.reviewer
        if not DRY_RUN:
            CONFIG_PATH.write_text(json.dumps(cfg, indent=2) + "\n")

    fields = project_fields(cfg) if not DRY_RUN or CONFIG_PATH.exists() else {}
    for name, options in FIELDS.items():
        if name in fields:
            continue
        a = ["project", "field-create", str(cfg["number"]), "--owner", cfg["owner"], "--name", name, "--format", "json"]
        a += ["--data-type", "TEXT"] if options is None else ["--data-type", "SINGLE_SELECT",
                                                               "--single-select-options", ",".join(options)]
        gh(*a, mutate=True)
    fields = project_fields(cfg) if not DRY_RUN else fields

    # Pass 1: make sure every spec has an issue.
    existing = task_issues(repo)
    numbers = {k: v["number"] for k, v in existing.items()}
    for s in specs.values():
        if s.id in numbers:
            continue
        labels = ["task", f"runs:{s.runs_on}", f"phase:{s.phase}"] + (["human"] if s.runs_on == "human" else [])
        a = ["issue", "create", "--repo", repo, "--title", s.issue_title, "--body", issue_body(s, repo, numbers, branch)]
        for l in labels:
            a += ["--label", l]
        if s.runs_on == "human" and cfg.get("reviewer"):
            a += ["--assignee", cfg["reviewer"]]
        url = gh(*a, mutate=True)
        numbers[s.id] = int(url.rstrip("/").split("/")[-1]) if url else -1

    # Pass 2: sync bodies (now that dependency numbers exist), board membership, and fields.
    items = project_items(cfg) if CONFIG_PATH.exists() else {}
    for s in specs.values():
        n = numbers[s.id]
        gh("issue", "edit", str(n), "--repo", repo, "--body", issue_body(s, repo, numbers, branch), mutate=True)
        item_id = items.get(n)
        if item_id is None:
            res = gh("project", "item-add", str(cfg["number"]), "--owner", cfg["owner"],
                     "--url", f"https://github.com/{repo}/issues/{n}", "--format", "json", mutate=True, parse_json=True)
            item_id = res.get("id", "DRY")
        if DRY_RUN:
            continue
        set_field(cfg, fields, item_id, "Task ID", s.id)
        set_field(cfg, fields, item_id, "Phase", s.phase)
        set_field(cfg, fields, item_id, "Agent", s.agent)
        set_field(cfg, fields, item_id, "Size", s.size)
        set_field(cfg, fields, item_id, "Runs on", s.runs_on)
    # Pass 3: native "Blocked by" relationships, so issues and the board show what is blocked.
    sync_dependencies(specs, numbers, repo)
    print("Bootstrap complete." if not DRY_RUN else "Dry run complete; nothing was changed.")
    print("Next: set the Status options and built-in workflows in the Project settings (see docs/tasks/H-1.md), "
          "then run: gh_task.py check")


GRAPH_ISSUE_TITLE = "Task graph"
GRAPH_STYLES = {
    "done": "fill:#d7ead3,stroke:#4f7a47,color:#2b3a28",
    "review": "fill:#d6e4f0,stroke:#3d6a8f,color:#1f3447",
    "changes": "fill:#f3d9d4,stroke:#9a4a3b,color:#4a221b",
    "progress": "fill:#f6e7c1,stroke:#9a7a2b,color:#4a3a12",
    "ready": "fill:#ffffff,stroke:#236965,stroke-width:3px,color:#1d2b2a",
    "blocked": "fill:#ece8e1,stroke:#a39b8e,color:#6b645a",
}


def node_state(s: "Spec", closed: set[str], status: str) -> str:
    if s.id in closed:
        return "done"
    st = status.lower()
    if st == "awaiting review":
        return "review"
    if st == "changes requested":
        return "changes"
    if st == "in progress":
        return "progress"
    return "ready" if all(d in closed for d in s.depends) else "blocked"


def render_graph(specs: dict[str, "Spec"], include: set[str], closed: set[str], statuses: dict[str, str]) -> str:
    def node_id(t: str) -> str:
        return "n_" + re.sub(r"[^A-Za-z0-9]", "_", t)

    def label(s: "Spec") -> str:
        title = s.title if len(s.title) <= 34 else s.title[:33] + "…"
        title = title.replace('"', "'")
        return f"{s.id}<br/>{title}"

    lines = ["```mermaid", "flowchart LR"]
    for t in include:
        s = specs[t]
        lbl = label(s)
        shape = f'{{{{"{lbl}"}}}}' if s.runs_on == "human" else f'["{lbl}"]'   # hexagon for human steps
        lines.append(f"  {node_id(t)}{shape}:::{node_state(s, closed, statuses.get(t, ''))}")
    for t in include:
        for d in specs[t].depends:
            if d in include:
                lines.append(f"  {node_id(d)} --> {node_id(t)}")
    for cls, style in GRAPH_STYLES.items():
        lines.append(f"  classDef {cls} {style}")
    lines.append("```")
    return "\n".join(lines)


def cmd_graph(args) -> None:
    specs = load_specs()
    phases = set(args.phase or [])
    include = {t for t, s in specs.items() if not phases or s.phase in phases}
    if phases:   # show direct dependencies from outside the selection, for context
        include |= {d for t in list(include) for d in specs[t].depends}
    ordered = [t for t in specs if t in include]
    closed: set[str] = set()
    statuses: dict[str, str] = {}
    if not args.offline:
        cfg = load_config()
        issues = task_issues(cfg["repo"], warn=False)
        closed = {t for t, it in issues.items() if it["state"] == "CLOSED"}
        by_number = project_statuses(cfg)
        statuses = {t: by_number.get(it["number"], "") for t, it in issues.items()}
    graph = render_graph(specs, ordered, closed, statuses)
    legend = ("**Legend:** green = done · blue = awaiting review · red = changes requested · amber = in progress · "
              "**teal border = ready to start** · grey = blocked. Hexagons are your steps.")
    doc = f"{legend}\n\n{graph}\n\n<sub>Generated by `tools/scripts/gh_task.py graph`. Do not edit by hand.</sub>\n"
    if args.output:
        Path(args.output).write_text(doc, encoding="utf-8")
        print(f"Wrote {args.output}")
    elif args.issue:
        cfg = load_config()
        found = gh("issue", "list", "--repo", cfg["repo"], "--state", "open", "--search",
                   f'"{GRAPH_ISSUE_TITLE}" in:title', "--json", "number,title", parse_json=True) or []
        match = next((i for i in found if i["title"] == GRAPH_ISSUE_TITLE), None)
        if match:
            gh("issue", "edit", str(match["number"]), "--repo", cfg["repo"], "--body", doc, mutate=True)
            print(f"Updated #{match['number']}")
        else:
            url = gh("issue", "create", "--repo", cfg["repo"], "--title", GRAPH_ISSUE_TITLE, "--body", doc, mutate=True)
            print(f"Created {url}. Pin it from the issue page so it stays at the top of Issues.")
    else:
        print(doc)


def cmd_check(_args) -> None:
    cfg = load_config()
    problems = []
    fields = project_fields(cfg)
    status = fields.get("Status")
    have = {o["name"] for o in (status or {}).get("options", [])}
    missing = [v for v in STAGES.values() if v not in have]
    if missing:
        problems.append(f"Status field lacks options: {missing} (add them in the Project settings)")
    for name in FIELDS:
        if name not in fields:
            problems.append(f"Project field '{name}' missing (re-run bootstrap)")
    labels = {l["name"] for l in gh("label", "list", "--repo", cfg["repo"], "--limit", "200",
                                    "--json", "name", parse_json=True)}
    problems += [f"Label '{l}' missing" for l in LABELS if l not in labels]
    issues = task_issues(cfg["repo"])
    problems += [f"No issue for {t}" for t in load_specs() if t not in issues]
    raw = gh("issue", "list", "--repo", cfg["repo"], "--label", "task", "--state", "open", "--limit", "1000",
             "--json", "title", parse_json=True) or []
    counts: dict[str, int] = {}
    for it in raw:
        m = TASK_TITLE_RE.match(it["title"])
        if m:
            counts[m.group(1)] = counts.get(m.group(1), 0) + 1
    problems += [f"{t} has {n} open issues (close duplicates)" for t, n in sorted(counts.items()) if n > 1]
    if problems:
        print("\n".join("✗ " + p for p in problems))
        sys.exit(1)
    print("✓ Project, fields, labels, and issues are in order.")


def cmd_issue(args) -> None:
    spec_or_die(args.id)
    it = issue_for(args.id, load_config()["repo"])
    print(f"{it['number']} {it['url']} {it['state']}")


def cmd_runs_on(args) -> None:
    print(spec_or_die(args.id).runs_on)


def cmd_ready(args) -> None:
    s = spec_or_die(args.id)
    issues = task_issues(load_config()["repo"])
    blockers = []
    for d in s.depends:
        it = issues.get(d)
        if it is None or it["state"] != "CLOSED":
            blockers.append(f"{d} ({'no issue' if it is None else '#' + str(it['number']) + ' ' + it['state']})")
    if blockers:
        print(f"{s.id} is blocked by: " + ", ".join(blockers))
        sys.exit(1)
    print(f"{s.id} is ready.")


def cmd_stage(args) -> None:
    if args.stage not in STAGES:
        raise SystemExit(f"Stage must be one of {list(STAGES)}")
    cfg = load_config()
    spec_or_die(args.id)
    n = issue_for(args.id, cfg["repo"])["number"]
    item = project_items(cfg).get(n)
    if item is None:
        raise SystemExit(f"Issue #{n} is not on the board. Re-run bootstrap.")
    set_field(cfg, project_fields(cfg), item, "Status", STAGES[args.stage])
    print(f"{args.id} → {STAGES[args.stage]}")


def open_task_pr(task_id: str, repo: str) -> dict | None:
    prs = gh("pr", "list", "--repo", repo, "--state", "open", "--limit", "200",
             "--json", "number,url,headRefName,reviewDecision,labels,author", parse_json=True) or []
    return next((p for p in prs if p["headRefName"].startswith(f"task/{task_id}-")), None)


def cmd_pr(args) -> None:
    spec_or_die(args.id)
    pr = open_task_pr(args.id, load_config()["repo"])
    if pr is None:
        result = {"open": False}
    else:
        labels = {l["name"] for l in pr.get("labels", [])}
        result = {"open": True, "number": pr["number"], "url": pr["url"], "branch": pr["headRefName"],
                  "review_decision": pr.get("reviewDecision") or "",
                  "revision_requested": pr.get("reviewDecision") == "CHANGES_REQUESTED" or "changes-requested" in labels}
    if args.json:
        print(json.dumps(result))
    elif not result["open"]:
        print(f"No open PR for {args.id}.")
    else:
        print(f"#{result['number']} {result['url']} branch={result['branch']} "
              f"review={result['review_decision'] or 'none'} revision_requested={result['revision_requested']}")


def cmd_handoff(args) -> None:
    """Final bookkeeping after an agent run: the task must have an open PR; set Awaiting review."""
    spec_or_die(args.id)
    cfg = load_config()
    pr = open_task_pr(args.id, cfg["repo"])
    if pr is None:
        print(f"{args.id}: no open PR from a task/{args.id}-* branch, so it was not handed off for review.")
        sys.exit(1)
    n = issue_for(args.id, cfg["repo"])["number"]
    item = project_items(cfg).get(n)
    if item is None:
        raise SystemExit(f"Issue #{n} is not on the board. Re-run bootstrap.")
    set_field(cfg, project_fields(cfg), item, "Status", STAGES["awaiting-review"])
    print(pr["url"])


def cmd_review(args) -> None:
    cfg = load_config()
    pr = open_task_pr(args.id, cfg["repo"])
    if pr is None:
        raise SystemExit(f"No open PR for {args.id}.")
    n = pr["number"]
    view = gh("pr", "view", str(n), "--repo", cfg["repo"], "--json", "reviews,comments,author", parse_json=True)
    bot = view["author"]["login"]
    marks = [c["createdAt"] for c in view.get("comments", [])
             if c["author"]["login"] == bot and c["body"].lstrip().startswith("## Revision")]
    since = max(marks) if marks else ""
    inline = json.loads(gh("api", f"repos/{cfg['repo']}/pulls/{n}/comments", "--paginate") or "[]")
    out = [f"# Review feedback for {args.id} (PR #{n}) since {since or 'the PR opened'}\n"]
    for r in view.get("reviews", []):
        if (r["author"]["login"] != bot and trusted(r.get("authorAssociation")) and r.get("submittedAt", "") > since
                and (r.get("body") or r["state"] != "COMMENTED")):
            out.append(f"## Review by {r['author']['login']} — {r['state']}\n{r.get('body') or '(no summary)'}\n")
    for c in inline:
        if c["user"]["login"] != bot and trusted(c.get("author_association")) and c["created_at"] > since:
            out.append(f"## {c['path']}:{c.get('line') or c.get('original_line')} — {c['user']['login']}\n{c['body']}\n")
    for c in view.get("comments", []):
        if c["author"]["login"] != bot and trusted(c.get("authorAssociation")) and c["createdAt"] > since:
            out.append(f"## Comment by {c['author']['login']}\n{c['body']}\n")
    print("\n".join(out) if len(out) > 1 else out[0] + "\n(No new feedback.)")


TRUSTED = {"OWNER", "MEMBER", "COLLABORATOR"}


def trusted(assoc: str | None) -> bool:
    """Only maintainers' words become agent requirements. The repo is public: anyone can comment."""
    return (assoc or "").upper() in TRUSTED


def cmd_comments(args) -> None:
    cfg = load_config()
    n = issue_for(args.id, cfg["repo"])["number"]
    view = gh("issue", "view", str(n), "--repo", cfg["repo"], "--json", "comments,author", parse_json=True)
    humans = [c for c in view.get("comments", []) if not c["author"]["login"].endswith("[bot]")
              and c["author"]["login"] != view["author"]["login"] and trusted(c.get("authorAssociation"))]
    if not humans:
        print("(No maintainer comments.)")
    for c in humans:
        print(f"## {c['author']['login']} — {c['createdAt']}\n{c['body']}\n")


def main() -> None:
    for stream in (sys.stdout, sys.stderr):
        if hasattr(stream, "reconfigure"):
            stream.reconfigure(encoding="utf-8", errors="replace")
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)
    sp = sub.add_parser("specs")
    sp.add_argument("--check", action="store_true", help="validate sections, dependencies, and cycles")
    sp.set_defaults(fn=cmd_specs)
    b = sub.add_parser("bootstrap")
    b.add_argument("--owner", required=True)
    b.add_argument("--reviewer", required=True, help="GitHub login of the human reviewer")
    b.add_argument("--title", default="Achlydesa")
    b.add_argument("--dry-run", action="store_true")
    b.set_defaults(fn=cmd_bootstrap)
    sub.add_parser("check").set_defaults(fn=cmd_check)
    g = sub.add_parser("graph")
    g.add_argument("--phase", action="append", help="limit to a phase, e.g. --phase P1 (repeatable)")
    g.add_argument("--offline", action="store_true", help="dependencies only; no GitHub state")
    g.add_argument("--output", help="write Markdown with a Mermaid graph to this file")
    g.add_argument("--issue", action="store_true", help="write the graph to the pinned 'Task graph' issue")
    g.set_defaults(fn=cmd_graph)
    for name, fn in (("issue", cmd_issue), ("runs-on", cmd_runs_on), ("ready", cmd_ready), ("handoff", cmd_handoff),
                     ("review", cmd_review), ("comments", cmd_comments)):
        c = sub.add_parser(name)
        c.add_argument("id")
        c.set_defaults(fn=fn)
    s = sub.add_parser("stage")
    s.add_argument("id")
    s.add_argument("stage")
    s.set_defaults(fn=cmd_stage)
    r = sub.add_parser("pr")
    r.add_argument("id")
    r.add_argument("--json", action="store_true")
    r.set_defaults(fn=cmd_pr)
    args = p.parse_args()
    args.fn(args)


if __name__ == "__main__":
    main()
