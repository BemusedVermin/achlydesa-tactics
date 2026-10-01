# ADR-0004 — The atlas is the presentation

**Status:** Proposed
**Date:** 2026-10-01 · **Conflict:** C-04

## Context
The High-Level Design specifies an isometric camera that snaps among four angles, with cutaway roofs and individual soldiers visible at close zoom (High-Level Design §5.1 Units and simulation scale). Simulation §5.1 Main map repeats it. The Field Atlas replaces it with a north-up, top-down atlas and keeps height, floors and line of sight as simulation properties (Field Atlas "Presentation precedence"). The Execution Plan amends the atlas with horizon plates (Execution Plan §Phase 0 build item 1; §2.2 Awe: the colossal, Option B). Because the Field Atlas ranks above the Simulation document in precedence, the winner is already clear for presentation, but two lower documents still say otherwise. See C-04.

## Options
1. **Isometric camera.** Needs a second renderer, a rotation system and per-building art, none of which fit a solo project. It also hides what the headquarters does not know.
2. **North-up atlas plus horizon plates (recommended).** One rendering; plates are dated reports that carry the awe.
3. **Atlas only, no plates.** Cheapest, but removes the first-sighting beat and the plates that carry hope and despair (Execution Plan §2.2).

## Decision
**PROPOSED:** Adopt option 2. The presentation is a north-up top-down field atlas. Height, occlusion, floors, bridges, occupancy and line of sight are simulation properties, shown through a floor selector and a section diagram when needed, with no second combat map (Field Atlas "Presentation precedence"). Individual soldiers are not drawn at close zoom; a soldier is seen through a counter, a dossier and a portrait (ADR-0005). Horizon plates are reports, not cameras (Execution Plan §2.2). The simulation geometry remains a 2.5D heightfield, so the High-Level Design's sentence on one authoritative 2.5D geometry survives with the camera sentences removed.

## Consequences
- **P0-02 edits (High-Level Design):** §5.1 (the final paragraph: "isometric camera snaps among four cardinal angles", "cutaway roofs"; and the paragraph's "camera zooms seamlessly from accurate formation symbols to visible individual soldiers"). Key phrases for the grep: "four cardinal angles", "isometric camera", "cutaway roofs", "visible individual soldiers".
- **Not edited by P0-02:** Simulation §5.1 Main map keeps "Four snapped camera orientations"; this ADR outranks it once Accepted, and a later edit should align it.
- **Task specs affected:** P0-03 (Experience Bible builds on the atlas) and all Phase 2 tasks.
- **What becomes true:** no document may require a rotatable or oblique camera or visible individual soldiers.
