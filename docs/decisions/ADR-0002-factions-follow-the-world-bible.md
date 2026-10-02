# ADR-0002 — Factions follow the World Bible

**Status:** Accepted
**Date:** 2026-10-01 · **Conflict:** C-02

## Context
The High-Level Design lists five invented organizations contesting the junction and states they "are additions for the game, not names already established in the world bible" (High-Level Design §18.0b Factions contesting it). The World Bible defines seven polities (World Bible §States made from services). Campaign Bible counterparts hold offices in those polities (Campaign Bible §7 Political counterparts and antagonists). The World Bible also lists who wants the junction and why (World Bible §The caravan and the junction, "Why everyone wants the junction"). See C-02.

## Options
1. **Keep the five High-Level Design organizations** and add them to the World Bible. Violates the rule that lore lives in the World Bible and duplicates existing polities.
2. **Map each organization onto an existing polity, or retire it (recommended).** Matches Execution Plan §Phase 0 build item 1.
3. **Keep the names as in-world coalitions** (alliances of polities). Adds invented lore to explain a naming problem and still leaves two vocabularies.

## Decision
**PROPOSED:** Adopt option 2. Mapping:

| High-Level Design organization | Becomes | Reasoning |
|---|---|---|
| Strategos's Custodians | Assurance Marches | The Marches are the Strategos-guaranteed garrison state (World Bible §States made from services). |
| Gate Compact of Pylaios | Threshold Principalities | They are the gate-concession polities (same section). |
| Autophagan Delegation | Shell Commonwealth | The city-inside-the-owner polity (same section). |
| Free Cistern League | Basin Commonwealth's municipal companies and towns | Its "local defense forces" wording matches the Commonwealth's "municipal companies" (World Bible §The Basin Commonwealth); no new faction is needed. |
| Heliarchic Mandate | **Retired** | No polity is built on the Heliarch; he has a court but no state (World Bible §Heliarch: the man who thinks the sky needs a better audience). His junction interest is carried as patronage and weather/fire pressure on the others (Campaign Bible §5 Campaign architecture, Movement III). |

The Lamp Concord, Clear Republic and Ribbon Houses are not in the High-Level Design table. They act as the World Bible's "Why everyone wants the junction" table describes, and the High-Level Design's "Anodyne, Aletheia, and Mneme remain important powers" sentence is rewritten to name them. The Basin reformers and creditors in that table are factions within the Basin Commonwealth, as Campaign Bible §7 already treats them.

## Consequences
- **P0-02 edits (High-Level Design):** §18.0b (retitle; replace the table by the mapping above, adapting the "what it wants" and "military character" columns to the mapped polity). Check §18.0, §18.0a and §18.1 for the old names. Key phrases for the grep: "Custodians", "Heliarchic Mandate", "Gate Compact", "Autophagan Delegation", "Free Cistern League". Section 7.5 line "Heliarchic weather and sensing" is about the Archon's effect, not a faction, and stays.
- **Task specs affected:** P0-06 (Red Ledger region: the Marches garrison, Basin towns) and Phase 2+ planning tasks that name polities.
- **What becomes true:** the theater has seven polities and no others.
