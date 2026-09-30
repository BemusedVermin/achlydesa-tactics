---
name: task
description: Run one Achlydesa task from docs/tasks/<ID>.md end to end — branch, implement, verify, audit, open or update a pull request — then stop for review. Usage /task P1-02
argument-hint: <task-id>
disable-model-invocation: true
---

Run task **$ARGUMENTS**. This works identically in a local Claude Code session and in GitHub Actions. `gh` is authenticated (ideally as the bot). Use `python3 tools/scripts/gh_task.py` (below: `T`) for all board and issue lookups.

1. **Load.** Read `docs/tasks/$ARGUMENTS.md`. If its *Runs on* is `human`, stop: human steps are not run by agents. If it is `local` and the environment variable `GITHUB_ACTIONS` is `true`, comment on the task issue that it must run locally, and stop.

2. **Readiness.** Run `T ready $ARGUMENTS`. If it exits non-zero, report the blockers (in Actions: as a comment on the task issue) and stop.

3. **Mode.** Run `T pr $ARGUMENTS --json`.
   - An open PR with `revision_requested: true` puts you in **revision mode**. Save `T review $ARGUMENTS` to `.git/achlydesa-review-$ARGUMENTS.md`. These review notes override the spec where they conflict.
   - An open PR without a revision request means stop: the PR is awaiting review. Print its URL.
   - With no open PR you are in **fresh mode**. Save `T comments $ARGUMENTS` to `.git/achlydesa-comments-$ARGUMENTS.md`. Maintainer comments on the issue are additional requirements; they carry feedback such as G1 notes on a reopened task.

4. **Branch.**
   - Fresh: `git fetch origin && git checkout -B task/$ARGUMENTS-<slug> origin/main`, with the slug from the spec's *Branch slug*.
   - Revision: `git fetch origin && git checkout <branch from step 3> && git pull --ff-only`.
   - Then `T stage $ARGUMENTS in-progress`.

5. **Delegate.** Launch the subagent named in the spec's *Agent* field with: "Execute task $ARGUMENTS. Spec: docs/tasks/$ARGUMENTS.md." Add "Additional requirements: .git/achlydesa-comments-$ARGUMENTS.md" in fresh mode if that file has content. Add "Revision mode. Review feedback: .git/achlydesa-review-$ARGUMENTS.md. Address every item or explain why not." in revision mode.

6. **Verify.** Run every command under the spec's *Acceptance* yourself, plus `cargo xtask ci` if `Cargo.toml` exists. On failure, send the output back to the same subagent type (at most two rounds). If it still fails, continue and report the failure honestly.

7. **Audit.** Launch `invariant-auditor` with "Audit task $ARGUMENTS on the current branch against origin/main." If the verdict is FAIL with blocking findings, send them to the implementing subagent once, then re-audit.

8. **Commit and push.** Commit in small logical commits (`$ARGUMENTS: <what>`). `git push -u origin HEAD`.

9. **Pull request.**
   - **Fresh mode:**
     - Write the PR body from `.github/pull_request_template.md` into `.git/achlydesa-pr-$ARGUMENTS.md`.
     - Fill in every section: the subagent's summary, files changed, your acceptance results table, deviations, open questions, the audit verdict and unresolved findings, and artifacts (embed images from `docs/artifacts/$ARGUMENTS/` with raw GitHub links on the branch).
     - Include `Closes #<issue number from T issue $ARGUMENTS>`.
     - Then run `gh pr create --base main --title "$ARGUMENTS — <spec title>" --body-file .git/achlydesa-pr-$ARGUMENTS.md --label task`, plus `--reviewer <reviewer from .github/project.json>` when the PR author is not the reviewer.
   - **Revision mode:**
     - Post one PR comment starting `## Revision N`. List each feedback item with what changed, or why it did not, followed by updated acceptance results and the audit verdict.
     - Remove the `changes-requested` label if present: `gh pr edit <n> --remove-label changes-requested`.
     - Re-request review with `gh pr edit <n> --add-reviewer <reviewer>`.

10. **Hand off.** Run `T stage $ARGUMENTS awaiting-review`. Print a five-line summary: what was built, acceptance result, audit verdict, deviations, open questions, and the PR URL. Stop. Never merge, approve, or close issues.
