# Contributing to Achlydesa

Welcome. This project is built mostly by AI agents working from human-reviewed specs, with human judgment at every merge. You can work alongside them.

## The ground rules

1. **Specs first.** Work happens against a spec in `docs/tasks/`. If you want to build something without a spec, open an *Idea or problem* issue, and we'll write one.
2. **Determinism is sacred.** Simulation crates have no floats, no hash maps, no wall clocks, and no unkeyed randomness (`CLAUDE.md` §Simulation crates). CI enforces most of this; reviewers enforce the rest.
3. **The dependency graph is architecture.** `xtask/allowed_deps.toml` decides which crate may use which. Changing it needs its own PR and an ADR.
4. **Canon has an order.** Accepted ADRs, then the World Bible, then the rest (`CLAUDE.md` §Document map). Don't invent lore in code or text content; propose it.
5. **Tone.** The game should make players feel awe, despair, and hope. Satire targets the powerful, never the displaced. Prose shows; it doesn't tell.

## Picking up a task

- Take a Todo issue on the board whose dependencies are closed (`python3 tools/scripts/gh_task.py ready <ID>`). Assign yourself, and set it In Progress (`gh_task.py stage <ID> in-progress`).
- Branch `task/<ID>-<slug>` from `main`. Do only the spec's Deliverables.
- Open a PR titled `<ID> — <title>` using the template, with `Closes #<issue>`. Set Awaiting review.
- `cargo xtask ci` must pass locally and in CI.

## Reviewing agent PRs

Review like you would a careful colleague's work: check the acceptance evidence, read the deviations and open questions, and look hardest at the spec's *Review focus*. Line comments plus "Request changes" trigger an automatic revision. Only code owners merge.

## Asking Claude

Mention `@claude` in an issue or PR comment for questions or small fixes. That runs the **Claude** workflow, which is available to maintainers and collaborators only.
