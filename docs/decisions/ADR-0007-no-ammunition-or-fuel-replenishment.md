# ADR-0007 — No ammunition or fuel replenishment at baseline

**Status:** Accepted
**Date:** 2026-10-01 · **Conflict:** C-07

## Context
The High-Level Design leaves open whether restored old facilities can replenish mundane ammunition or fuel, and says the baseline must work without that permission (High-Level Design §16.4 Restoration orders). Simulation says processed military fuel and ammunition are finite, and adding replenishment would be "a deliberate later rules change" (Simulation §17.1 Strict technological boundary). The Technical Design anticipates "production from permitted existing services" as a typed delta (Technical Design §18.2 Inventory conservation). The Execution Plan chooses "baseline no" with a content flag (Execution Plan §Phase 0 build item 1). See C-07.

## Options
1. **Never.** Strict finite stocks for the whole game. Cleanest for scarcity and despair, but removes a later design lever.
2. **Baseline no, with a content flag, default off (recommended).** The playtest of Red Ledger decides whether any facility ever refills ammunition or fuel.
3. **Yes, for specific restored facilities.** Gives hope a material form but risks the "unexplained replenishment" the High-Level Design warns against.

## Decision
**PROPOSED:** Adopt option 2. Ammunition, manufactured spares and processed military fuel begin as finite physical stocks and stay finite in all baseline content. A per-facility content flag, default off, may later permit a named restored facility to produce a bounded quantity of one stock kind. The flag, if used, requires an explicit rate, source capacity, and a decision recorded as a new ADR after the Red Ledger playtest. Food and water remain renewable (High-Level Design §16.4). Reasoning: scarcity is what makes every round a decision and every loss unrecoverable; keeping the option protects the design without committing the build.

## Consequences
- **P0-02 edits (High-Level Design):** §16.4 (the "Consumable boundary" paragraph: replace "remaining scope decision" with the baseline rule and flag). Key phrase: "remaining scope decision".
- **Not edited by P0-02:** Simulation §17.1 already matches. Technical Design §18.2 stays valid because "permitted" now means flag-on.
- **Task specs affected:** P1-10 (stock kinds and the content schema need the flag field), Phase 3-5 logistics planning tasks.
- **What becomes true:** no baseline content refills ammunition or military fuel.
