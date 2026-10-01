# ADR-0003 — The opening is the Red Ledger incident

**Status:** Proposed
**Date:** 2026-10-01 · **Conflict:** C-03

## Context
Three accounts of the opening exist. The High-Level Design has a caravan technician use a maintenance credential at a Soterion checkpoint, which wakes the junction and triggers a Strategos detachment (High-Level Design §18.0 The caravan opening). Simulation §25.2 Opening implementation repeats that trigger. The Campaign Bible has Captain Varo Kest enforce a backdated directive at an inspection station, with the junction discovered afterward (Campaign Bible §4 The opening: the Red Ledger incident). The World Bible's version also uses a Marches inspection point and an assurance directive (World Bible §The caravan and the junction, "The opening incident"). The High-Level Design also says the player is the guard commander; see C-11. See C-03.

## Options
1. **Technician trigger** (High-Level Design, Simulation). Makes the player's side cause the catastrophe, and makes the people in the caravan a plot device, which the Campaign Bible forbids ("Neither is introduced as the bearer of a magical key").
2. **Kest's directive (recommended).** A human, knowing choice by an officer with a real shortage. The junction is discovered afterward through a roadside terminal.
3. **Both.** Kest's directive plus a technician's credential. Two causes for one event, with no gain in feeling; it makes the caravan's own action the justification for the seizure.

## Decision
**PROPOSED:** Adopt option 2. The opening is the Red Ledger incident as written in the Campaign Bible. Kest "chooses to enforce the order despite knowing that the civilian papers were valid" (Campaign Bible §4); the player can seek time, move vulnerable passengers, prepare a covered withdrawal, or negotiate, and never has to fire first to start the plot. The junction enters the story afterward when a roadside terminal recognizes a rejected travel claim. Reasoning: a decision by a person, not a misunderstanding, is what lets despair be about injustice instead of mechanism, and it keeps the caravan's people innocent so the later choice to reopen Reedbank's relief road is the player's.

## Consequences
- **P0-02 edits (High-Level Design):** §18.0 (all five paragraphs: premise, trigger, detachment, classification, evidence); §18.0a only if it refers to the technician. Replace "guard commander" per C-11. Key phrases for the grep: "maintenance credential", "technician", "restore passage".
- **Not edited by P0-02:** Simulation §25.2 carries the technician trigger. This ADR outranks it once Accepted; a later edit should align it.
- **Task specs affected:** P0-06 (sites: Cistern Camp, the inspection station), P0-07 (onboarding beats follow the Campaign Bible sequence).
- **What becomes true:** no document says the caravan's own action caused the seizure.
