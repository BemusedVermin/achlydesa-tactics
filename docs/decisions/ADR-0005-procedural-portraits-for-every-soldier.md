# ADR-0005 — Procedural portraits for every soldier

**Status:** Accepted
**Date:** 2026-10-01 · **Conflict:** C-05

## Context
The High-Level Design (§9.3 Generic and unique characters) and Simulation (§19.1 People as persistent participants) give every soldier a portrait. The Field Atlas budgets "one portrait silhouette with text-led identity" for the first playable (Field Atlas §2 Visual construction and production budget). The Execution Plan resolves it: "every soldier has one, and none are painted" (Execution Plan §4.4 Portraits). The Campaign Bible gives the player no portrait (Campaign Bible §2 The unseen general). See C-05.

## Options
1. **Painted portraits for named characters only; text for the rest.** Matches the atlas budget but drops the portrait from the generic soldier, which weakens the Roll and the sense that every death is a person.
2. **Procedural portraits for every soldier (recommended).** Deterministic from seed and service history; the atlas budget counts painted art only.
3. **One silhouette for everyone.** Meets the atlas budget literally. Soldiers become indistinguishable, which works against despair at irrecoverable loss.

## Decision
**PROPOSED:** Adopt option 2. Every soldier has a procedural portrait: a 32×40 four-color dithered bust, deterministic from the character's seed and history, with scars appearing after wounds, bandages while healing, and grey hair after the twenty-year cut (Execution Plan §4.4). The Field Atlas art budget is read as a budget for painted illustrations, and the one portrait silhouette stays as the fallback for an unrecorded person. The player has no portrait. Reasoning: a face that changes with the service record makes each loss visible and countable, which is where despair earns its weight.

## Consequences
- **P0-02 edits (High-Level Design):** §9.3 (change "a portrait" to a procedural portrait generated from seed and history). No other sections change. Key phrase: "a portrait".
- **Not edited by P0-02:** Simulation §19.1 and the Field Atlas §2 budget already read correctly under the new interpretation, with one clarifying sentence each recommended later.
- **Task specs affected:** P0-03 (Roll card), P1-03 (the `Portrait` RNG domain), P1-09 (portraits deferred to Phase 4, unchanged), Phase 4 planning.
- **What becomes true:** "portrait" in any document means the procedural bust unless it says "dossier plate."
