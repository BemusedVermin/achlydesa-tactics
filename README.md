# Achlydesa

A continuous-theater WEGO strategy RPG about the people the archons discarded. Rust simulation, Godot 4 presentation (from Phase 2). Built by Liam with Claude Code Sonnet agents that execute reviewed task specs, locally or on GitHub.

## How the project runs

| Thing | Where it lives |
|---|---|
| The whole plan | `docs/ROADMAP.md`: Phases 0–13, their gates, and how each phase's tasks get written |
| What to build | Specs in `docs/tasks/<ID>.md` (source of truth: goal, deliverables, interfaces, acceptance) |
| What's happening | GitHub Project board: one issue per spec, Status Todo → In Progress → Awaiting review → (Changes requested) → Done |
| Review | Pull requests: agent-authored, CI-checked, code-owner review required |
| Agent rules | `CLAUDE.md`, `.claude/agents/`, and `.claude/skills/task/SKILL.md` (`/task <ID>`) |
| Design canon | `docs/design/`, `docs/canon/`, and ADRs in `docs/decisions/` |
| Glue | `tools/scripts/gh_task.py`: specs ↔ issues ↔ board ↔ PRs |

## Setup

Follow `docs/tasks/H-1.md`. It covers the repository, a bot account, the Claude GitHub App, the Project board bootstrap, and branch protection. Run the bootstrap with `--dry-run` first: `gh_task.py` was tested against a stub of `gh`, not against live GitHub.

## Running a task

**On GitHub:** add the `agent:run` label to the task's issue, or use *Actions → Agent task → Run workflow* with the ID. Only tasks marked **Runs on: either** run there; tasks needing the local terrain tiles are refused with a comment.

**Locally:** start Claude Code with the bot token (`GH_TOKEN="$(cat ~/.config/achlydesa/bot-token)" claude`), run `/model sonnet`, then `/task P1-02`. Use a fresh session per task.

Either way, the agent:
1. checks that the task's dependencies are closed;
2. branches from `master` and sets the task In Progress;
3. delegates to the spec's subagent;
4. runs the acceptance checks and `cargo xtask ci`;
5. has the invariant auditor review the diff;
6. opens a PR that closes the issue, sets the task to Awaiting review, and stops.

## Reviewing

- **Approve and merge.** The issue closes and the board moves the task to Done, which unblocks its dependents.
- **Request changes.** Submit a "Request changes" review with line comments. On GitHub the **Agent revise** workflow picks it up automatically. Locally, run `/task <ID>` again. Either way the agent answers each comment in a `## Revision N` PR comment and re-requests your review.
- **Feedback after merge** (for example, a failed feeling gate): reopen the task's issue and comment. The next `/task <ID>` starts a fresh branch and treats your comments as requirements.

## Suggested order

```
H-1 → P1-01 → P0-01 → H-2 → P0-04 → P0-05 → H-3 → P1-02 → P1-03 → P1-04
    → P0-02 → P0-03 → P0-06 → P1-06 → P1-05 → P1-S1 (spike: first image!)
    → P1-07 → P1-08 → P1-09 → P1-10 → P1-11 → P1-12 → P1-13 → P1-14
    → P1-15 → G1 → P0-07 → P1-16
```

On GitHub you can run independent tasks at the same time: P1-08, P1-09, P1-10, and P1-13 can all go once P1-02 merges. Each run takes Actions minutes and model usage, so parallelism is a spending choice.

## Later phases

Every phase from 2 onward starts with a planning task, `P<N>-00`, already on the board. It is run by the `spec-writer` agent like any other task and opens a PR containing that phase's specs. Review it hard. Merge it, re-run `gh_task.py bootstrap` to create the new issues, then execute. Each phase ends at a feeling gate, `G<N>`, which unblocks the next planning task. Details are in `docs/ROADMAP.md`.

## Collaborators

Read `CONTRIBUTING.md`. Humans pick up tasks the same way agents do: branch `task/<ID>-<slug>`, fill the PR template, pass CI. The determinism and boundary rules in `CLAUDE.md` bind everyone.
