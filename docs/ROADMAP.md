# Achlydesa — Roadmap

**Status:** living document. Each phase's planning task (`P<N>-00`) refines that phase's section, then writes its task specs.

The phases are introduced in Execution Plan §5. This roadmap turns them into something task specs can be generated from. For each phase it gives:

- what the phase builds, and which design sections it implements;
- the crates and data it touches;
- what must be true before planning it, and its exit gates;
- the decisions to settle before writing specs;
- a candidate task list, refined by the planner.

## How phases become tasks

Every phase **N ≥ 2** has two standing items, already on the board:

| Item | What it is | Depends on |
|---|---|---|
| `P<N>-00 — Plan Phase N` | An agent task (`spec-writer`). It writes Phase N's task specs from this roadmap, the design docs, the actual code, and the previous phases' PRs and gate notes. | The previous phase's feeling gate |
| `G<N> — <gate>` | Your feeling gate for Phase N. The planner rewires its dependencies to the phase's final tasks. | `P<N>-00` (until the planner updates it) |

The cycle:
1. Run `P<N>-00` like any task (`agent:run` or `/task P<N>-00`). It opens a PR containing the new specs.
2. Review the specs. This is the most important review of the phase: specs are cheap to fix and code isn't.
3. Merge, then run `python tools/scripts/gh_task.py bootstrap --owner <owner> --reviewer <you>`. That creates the phase's issues.
4. Execute the tasks.
5. Pass `G<N>`, which unblocks `P<N+1>-00`.

Phases 0 and 1 were planned in advance; their specs already exist.

## Phase overview

| Phase | Name | Delivers | Gate |
|---|---|---|---|
| 0 | One canon | Consistent docs, Experience Bible, size canon, terrain, region, onboarding | H-2, H-3 |
| 1 | Headless truth | Deterministic march over real terrain, journal, field log, trace viewer | G1 — read the log aloud |
| 2 | The atlas | Godot client, projection API, field-atlas rendering, affect bus skeleton | G2 — the print test |
| 3 | Command trust | Orders, perception and belief, messages, method library, decision queue, composer, execution replay | G3 — the replay grin |
| 4 | Blood | Fire, suppression, wounds, succession, procedural portraits, the Roll, graves | G4 — who died? |
| 5 | The road | Supply, transport, repair, Last-Of registry, settlements, arrivals, Kept tally | G5 — the convoy arrives |
| 6 | The colossal | SDF archons, horizon plates, first sighting, atlas breaking, archon sound, the flinch | G6 — someone says something out loud |
| 7 | The monsters speak | Anomaly contract, Anodyne, proclamations, enemy headquarters planning | G7 — laugh, then sick |
| 8 | *Red Ledger* | Narrative conditions, Movements I–II campaign, characters, theme, onboarding, difficulty | G8 — three outside testers |
| 9 | The scale gate | Streaming, theater-scale quiet advancement, full-theater terrain | G9 — a week of theater in budget |
| 10 | Movement III | Lamp Ward, the western gate, the common relay, the moving district, Autophagos | G10 — dependency without a clean choice |
| 11 | Movement IV | Strategos's campaign, the junction, the Joining and its outcomes | G11 — the nadir |
| 12 | The twenty-year cut | Upbringing, aged atlas and portraits, succession of generations | G12 — looking for a name from before |
| 13 | Movements V–VI | Successors, the return to the junction, the Receiver | G13 — the ending |

---

## Phase 2 — The atlas

**Builds:**
- the Godot 4 client;
- the binding between Rust and Godot (`ach_godot`);
- the projection API (`ach_know` v0);
- field-atlas rendering: elevation bands, contours, slope hatching, stipple, footprints, counters, labels, semantic zoom, day/night palettes, the pixel pipeline;
- the `ach_affect` skeleton driving one ambient drone.

**Implements:**
- Execution Plan §2.1, §3, §4.1;
- Field Atlas §1–§6, §11, §12, §18, §23.1 slice A;
- Experience Bible §affect bus.

**Crates and data:** new `ach_godot` crate, `ach_know` (projection only), `ach_affect`, `godot/` project, `content/palettes/`.

**Entry:** G1 passed. The Experience Bible (P0-03) and the onboarding design (P0-07) are merged, and P1-16 is done.

**Exit:**
- Technical: atlas slice A — a player can identify a ridge, a crossing, a unit, a contact estimate, and the selected sector. Hidden-state equivalence tests cover rendering and affect outputs.
- Feeling: **G2**, the print test. Would you print a screenshot and pin it to the wall?

**Decide before specs:**
- Exact Godot 4.x version and gdext version pins.
- Whether CI renders Godot headless for golden screenshots, or visual checks stay manual (affects *Runs on*).
- The projection API's shape. This is the most consequential interface in the game, so spec it as its own task with an exhaustive review focus.

**Candidate tasks:**
- `ach_godot` binding skeleton
- Godot project scaffold, built from code, not editor scenes
- projection API v0 (survey plus own-force reports)
- hidden-state equivalence harness
- terrain bands and contours shader
- hatching and stipple
- pixel pipeline and dithering
- palette tokens, day/night, accessibility checks
- camera and semantic zoom
- counters and labels
- sites and routes layers
- `ach_affect` skeleton with a drone in Godot audio
- screenshot harness
- spike: the caravan theme with six voices

---

## Phase 3 — Command trust

The make-or-break phase: smallest scope, longest schedule.

**Builds:**
- perception, belief, and contacts;
- physical messages and the headquarters projection v1;
- order contracts with authority and lifecycle;
- the method library (Move, Hold, Observe, Retreat), executor, reassessment, and reason traces;
- the decision queue, order composer, and dispatch;
- execution replay;
- the 30 × 30 km tactical corridor at full detail, with sight lines.

**Implements:**
- Technical Design §5–§16 and §27.1 slices 2–4;
- Field Atlas §7–§10, §19, and slices B and C;
- High-Level Design §5 and acceptance checks;
- Execution Plan §2.6.

**Crates:** `ach_know`, `ach_orders`, `ach_ai`, `ach_world` (layers, deltas, sight lines), `ach_sim`, `ach_godot`.

**Entry:** G2 passed.

**Exit:**
- Technical: slices 2–4. A squad completes a feasible order without corrective clicks, resumes its objective after a firefight, and explains each deviation. The disconnected-squad and hidden-state tests pass.
- Feeling: **G3**. Watching a replay of a plan working makes you grin, and a leader's improvisation surprises you *and* makes sense.

**Decide before specs:**
- Method representation: HTN-style decomposition or utility-scored methods.
- The order-contract schema.
- The contents of the first method library.
- The terrain detail resolution inside the tactical corridor.

**Candidate tasks:**
- terrain layers and sparse deltas
- line of sight
- perception model
- belief and contact tracks
- message transport
- projection v1
- order contract types
- authority and lifecycle
- method library framework
- Move, Hold, Observe, and Retreat methods (one task each)
- executor and reassessment
- reason traces and trace view
- decision queue (simulation side)
- decision queue UI
- order composer UI
- dispatch and confirmation
- execution replay
- disconnected-squad scenario tests

Split any XL item.

---

## Phase 4 — Blood

**Builds:**
- fire, suppression, wounds, aid, and evacuation;
- the directional affinity matrix, cohesion, and succession;
- procedural portraits shaped by history (scars, bandages);
- the Roll;
- ink fade for the missing;
- graves on the atlas;
- the grief signal.

**Implements:**
- Simulation §12–§15 and §19;
- Technical Design slice 5;
- Execution Plan §2.3 and §4.4;
- Experience Bible §despair.

**Crates:** `ach_combat`, `ach_people`, `ach_text` (Roll grammar), `ach_affect`, `ach_godot`.

**Entry:** G3 passed.

**Exit:**
- Technical: slice 5 — casualties and succession change capability without manual soldier control.
- Feeling: **G4**. The first Roll makes a tester pause; afterwards they can name someone who died.

**Decide before specs:**
- How lethal combat is.
- How far the wound model goes.
- Portrait art direction: palette, parts list, and how history marks a face.

---

## Phase 5 — The road

**Builds:**
- supply jobs and transport, convoys;
- repair and cannibalization;
- the Last-Of registry;
- settlement records;
- arrival ceremonies;
- the Kept tally;
- night-palette lamps;
- multi-day marches with bounded exceptions.

**Implements:**
- High-Level Design §11;
- Simulation §16–§18;
- Technical Design slice 6;
- Execution Plan §2.4;
- Experience Bible §hope.

**Crates:** `ach_logistics`, `ach_narrative` (settlement records only), `ach_affect`, `ach_godot`.

**Entry:** G4 passed.

**Exit:**
- Technical: slice 6 — a multi-day march with physical accounting and no recurring administrative clicks.
- Feeling: **G5**. The relief convoy reaching a settlement feels like a victory, though nobody fired a shot.

**Decide before specs:**
- Which technology categories the Last-Of registry tracks in *Red Ledger*.
- The Kept attribution rules (from the Experience Bible).

---

## Phase 6 — The colossal

**Builds:**
- the SDF recipe DSL and `ach_plate`;
- the Heliarch complete: footprint, glyph, dossier plate, horizon plate;
- the first-sighting protocol;
- atlas-breaking effects;
- archon sound signatures;
- the sky tour as a simulated transit, with oil hazards;
- **the flinch v1**: a simulation event observable through every channel, following the World Bible's evidence rule.

**Implements:**
- Execution Plan §2.2 and §4.3;
- World Bible §Cosmology, §The flinch, §Craft notes;
- size canon (P0-04);
- Experience Bible §awe.

**Crates:** `ach_plate`, `ach_anomaly` (transit only), `ach_affect`, `ach_text` (flinch reports), `ach_godot`.

**Entry:** G5 passed. The P1-S1 spike notes are reviewed.

**Exit:**
- Technical: plate render budget met; golden plates stable; delayed ignition changes a route through the ordinary command grammar.
- Feeling: **G6**. A tester opens the first-sighting card and says something out loud. A flinch, experienced cold, produces a question the game never answers.

**Decide before specs:**
- The render budget for plates.
- Which flinch channels v1 includes.
- Where the Movement I flinch falls (Campaign Bible §10 open question).

---

## Phase 7 — The monsters speak

**Builds:**
- the common anomaly contract;
- Anodyne's service dependency: wound transfer, recipients, the cost of severing;
- proclamation grammars for the Heliarch, Anodyne, and Strategos's court;
- enemy headquarters planning: Kest's detachment, and a Marches operation that pays for its commitments.

**Implements:**
- Technical Design slice 7;
- Field Atlas slice E;
- Execution Plan §2.5;
- World Bible §Corporate liturgy and the archon entries.

**Crates:** `ach_anomaly`, `ach_ai` (enemy headquarters), `ach_text`, `ach_godot`.

**Entry:** G6 passed.

**Exit:**
- Technical: Anodyne changes an operational choice; the enemy acts only on its own reports.
- Feeling: **G7**. A proclamation makes a tester laugh, then feel sick, within one minute.

---

## Phase 8 — *Red Ledger*

**Builds:**
- `ach_narrative`: world conditions, deadlines, and the promises ledger;
- the opening at the checkpoint;
- the withdrawal;
- Reedbank;
- the garrison surrender and Damar;
- Three Crossings;
- recognition and the Passage Articles;
- six founders, Asha, and three recruits;
- the caravan theme with its voices;
- onboarding (P0-07) and difficulty options;
- outside playtests.

**Implements:**
- Campaign Bible §3–§6 and §12;
- Execution Plan §7–§9;
- the onboarding design.

**Crates:** `ach_narrative`, `ach_text`, and all others for content.

**Entry:** G7 passed.

**Exit:**
- Technical: one save plays from the checkpoint to recognition, with failure branches that continue rather than reload.
- Feeling: **G8**. Three outside testers each name a moment of awe, one of despair, and one of hope, unprompted.

This is the milestone. *Red Ledger* is a finished game.

---

## Phase 9 — The scale gate

**Builds:**
- terrain streaming;
- theater-scale quiet advancement;
- full-theater terrain import (the 150 km window, then beyond);
- benchmark tiers.

**Implements:** Technical Design §21 and §27.2; High-Level Design §4.1.

**Entry:** G8 passed.

**Exit:**
- Technical: the benchmark tiers from Technical Design §27.2 run within budget on your workstation.
- Feeling: **G9**. A simulated week across the whole theater still reads like people wrote it.

## Phase 10 — Movement III: the price of allies
Lamp Ward, the western gate, the common relay, the moving district, and Autophagos as the second colossal body.

**Gate G10:** a tester faces Anodyne's dependency and finds no clean choice.

## Phase 11 — Movement IV: the first settlement
Strategos's campaign, the junction's real benefits, and the Joining with its three outcomes. The Joining is the despair nadir.

**Gate G11:** the Joining, played competently, still lands.

## Phase 12 — The twenty-year cut
Deterministic upbringing, the aged atlas, greyed portraits, the theme with voices missing, and successors' motifs.

**Gate G12:** a tester goes looking for a name from before the cut.

## Phase 13 — Movements V–VI
The inheritance, your insignia on the checkpoint, the return to the junction, and the Receiver. Liberation is honest and limited.

**Gate G13:** the ending.

Phases 10–13 are deliberately thin. Their planners rewrite these sections from what Phases 2–9 actually produced.
