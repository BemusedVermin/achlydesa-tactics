# ADR-0008 — Awe through atlas breaks and horizon plates

**Status:** Proposed
**Date:** 2026-10-01 · **Conflict:** C-08

## Context
The Field Atlas keeps archive plates optional and tactically non-unique (Field Atlas §2 Visual construction and production budget), allows impossible geometry only in a plate or an observed map feature with stable controls (Field Atlas §6.2 Surrealism as mapped relationships), and keeps the map north-up with a scale bar always available (Field Atlas §4 Terrain, scale, height, and semantic zoom). The Campaign Bible asks for a "small, unsettling archival plate" for the Receiver (Campaign Bible §10 The optional encounter inside the Soterion, "What appears"). The Execution Plan adds atlas breaks (footprints off the sheet, unjoined contours, a turning compass rose, dated shadows) and horizon plates delivered as first-sighting decision cards (Execution Plan §2.2 Awe: the colossal). See C-08.

## Options
1. **Field Atlas rules only.** No footprint overflow, no plates in the queue. Safest for legibility, but gives no ambient dread and no first-sighting beat.
2. **Execution Plan approach within the atlas's legibility rules (recommended).** Atlas breaks and horizon plates, with a hard rule that measurements, controls and labels never break.
3. **Execution Plan approach with no limits.** Maximum spectacle, but breaks the instrument that makes the broken parts frightening.

## Decision
**PROPOSED:** Adopt option 2. Both Option A (atlas breaks) and Option B (horizon plate) of Execution Plan §2.2 govern, subject to these reconciliations:
1. **Never broken:** the scale bar, distances, coordinates, controls, and labels (Execution Plan §2.2 "Never broken", matching Field Atlas §4).
2. **Plates are reports, not tactical sources.** A horizon plate's information is also stated in the report text, so the first three atlas layers stay sufficient with every plate removed (Field Atlas §2). The plate adds awe and specificity, not unique facts.
3. **The compass rose** is an instrument glyph whose drift is observed information; the map stays north-up (Field Atlas §4) and bearings in reports carry the correction.
4. **Footprint overflow** carries an edge label with the report age, as in Execution Plan §2.2.
5. **Plate size.** Horizon plates are 480×120 internal (Execution Plan §2.2); dossier plates remain 160×120 (Field Atlas §2). They are different artifacts. The Receiver's small archival plate is a dossier plate.
6. **First-sighting cards** arrive through the normal decision queue with the report's physical delay and never move the camera (Execution Plan §2.2).
Reasoning: keeping the instrument pristine is what makes the parts that break frightening (Execution Plan §2.2 says this directly).

## Consequences
- **P0-02 edits (High-Level Design):** none required; the High-Level Design has no section on awe presentation.
- **Not edited by P0-02:** the Field Atlas §2, §4 and §6.2 should gain the plate-versus-dossier distinction in a later edit. Until then this ADR governs.
- **Task specs affected:** P0-03 (Experience Bible: must define the plate and break rules against these six points), P0-04 (sizes drive footprints and plates), and Phase 2 and Phase 6 planning.
- **What becomes true:** awe is delivered by data-driven atlas breaks and plates, and legibility is never traded for it.
