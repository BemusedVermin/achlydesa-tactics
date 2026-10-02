# ADR-0001 — The Joining: containment is possible

**Status:** Accepted
**Date:** 2026-10-01 · **Conflict:** C-01

## Context
The High-Level Design says the player "cannot prevent the rupture" (High-Level Design §18.2 The catastrophe) and that if the coalition refuses, another claimant precipitates it (High-Level Design §18.0c An open war with a story spine). The Campaign Bible says "Prepared containment must be possible" and that the game must not "manufacture an unstoppable betrayal" (Campaign Bible §5 Campaign architecture, "The Joining — a crisis with different outcomes"). The World Bible makes the rupture conditional (World Bible §The caravan and the junction, "False victory without false agency"). Simulation §25.4 Authored catastrophe boundary holds an in-between position: the player cannot "indefinitely prevent" the transition. See C-01.

## Options
1. **High-Level Design as written.** An unavoidable rupture; the player shapes only evacuation and what survives. Simple to author, but a prepared player gets no reward at the climax, which undercuts hope that is earned.
2. **Campaign Bible (recommended).** Rival claimants will always attempt a regional synchronization. The player's preparation determines which of three outcomes occurs: broad integration, emergency severance, or prepared containment. Containment is expensive and real.
3. **Compromise.** The rupture always happens but containment shrinks it to one district. Cheaper to author than option 2, but it keeps the "inevitable" framing the Campaign Bible forbids and makes containment cosmetic.

## Decision
**PROPOSED:** Adopt option 2. The Joining is a synchronization attempt that cannot be wished away, because several courts are already restoring links to the junction (Campaign Bible §5 Campaign architecture, "The Joining"). Its local outcome is a function of preparation: isolated approaches, dispersed authority, independent power and dispatch, evacuations, and which relays were cut. Prepared containment is a reachable first-generation success with a real cost: foregone short-term benefits and a weaker federation afterward. The twenty-year jump remains in every branch, and a compulsory time jump does not imply a compulsory massacre (same section). Reasoning: this keeps the despair (what is lost on the other paths is irrecoverable) while keeping the hope material and earned.

## Consequences
- **P0-02 edits (High-Level Design):** §1 High concept (the phrase "inevitable catastrophe"), §18.0c (the paragraph beginning "The retained two-generation design includes an unavoidable rupture"), §18.2 (title and "inevitable" opening, "The player cannot prevent the rupture"), §18.3 only as needed for ADR-0010. Key phrases for the P0-02 grep: "cannot prevent the rupture", "unavoidable rupture", "inevitable".
- **Not edited by P0-02:** Simulation §25.4 already sits close to the decision, but its wording on the player not being able to indefinitely prevent it should be reconciled in a later edit. Until then this ADR outranks it.
- **Task specs affected:** the Phase 8 planning task `docs/tasks/P8-00.md` must plan the three outcomes as authored branches; P0-03 and P0-07 may reference the Joining flinch (Campaign Bible §10 "What appears", open design question on flinches).
- **What becomes true:** no document may say the Joining cannot be prevented or contained locally.
