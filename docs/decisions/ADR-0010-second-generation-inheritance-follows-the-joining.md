# ADR-0010 — The second generation's inheritance follows the Joining

**Status:** Accepted
**Date:** 2026-10-01 · **Conflict:** C-10

## Context
The High-Level Design says the second coalition "begins with fragmented cells, caches, a few damaged bases, partial records" and that "Most veterans disappear" (High-Level Design §18.3 Twenty years later; §18.2 The catastrophe). The Campaign Bible gives three first-settlement results with different inheritances, three possible starting situations in Movement V, and returns of surviving veterans as advisers or officers (Campaign Bible §5 Campaign architecture, "The Joining" table and Movement V; §9 The second generation). See C-10. It is related to ADR-0001: that ADR makes the three outcomes reachable, and this one says what each hands over.

## Options
1. **Cells-only start** (High-Level Design). One handover state, simple to author; makes prepared containment pointless and contradicts ADR-0001.
2. **Three inheritance states keyed to the Joining outcome (recommended).** Matches the Campaign Bible.
3. **Free-form inheritance computed from the simulation with no authored states.** Most faithful to the sandbox, but the deterministic history process (Simulation §25.5 Twenty-year transition) would have nothing to anchor it, and the authoring cost is unbounded.

## Decision
**PROPOSED:** Adopt option 2. The inheritance is one of the Campaign Bible's three states: a continuity regime with surviving resistance enclaves (broad integration), independent towns with a sealed junction (emergency severance), or a weaker organized federation (prepared containment). Surviving first-generation characters are exceptional presences: each person's fate (dead, missing, captive, transformed, or serving) results from the player's choices, and none is a default. The "fragmented cells" start is the resistance-enclave part of the broad-integration state. Reasoning: the player's first-generation effort must be visible in the world they leave behind, which is where hope becomes material.

## Consequences
- **P0-02 edits (High-Level Design):** §18.3 (the opening sentence of its last paragraph, "The new coalition begins with fragmented cells"), §18.2 (the closing paragraph beginning "First-half choices determine individual fates", which holds the sentence "Most veterans disappear"). Key phrases for the grep: "Most veterans disappear", "fragmented cells".
- **Not edited by P0-02:** Simulation §25.5 Twenty-year transition already carries forward surviving, captured, missing, transformed and dead people by outcome and needs no edit.
- **Task specs affected:** the Phase 8 and Phase 9 planning tasks (`docs/tasks/P8-00.md`, `docs/tasks/P9-00.md`), which must plan three inheritance states.
- **What becomes true:** no document may claim every veteran disappears or that every branch starts as scattered cells.
