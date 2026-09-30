# Achlydesa — Execution Plan

**Draft 1 · 29 September 2026 · Solo developer, hobby pace, no deadline**
**Companion to:** the High-Level Design, Simulation and Player Interactions, Autonomous Squad Technical Design, Field Atlas Style and UI Bible, Campaign and Character Bible, and World Bible.

**The promise this plan protects:** the owners of the promised future became monsters and are still trying to finish summoning their god. The player commands the people they discarded. Every system should let the player feel three things: **awe** at bodies the size of mountains, **despair** at what cannot be recovered, and **hope** that is material, countable, and earned.

The existing documents specify an honest command simulator in great depth. This plan does four things they do not:

1. Resolves the contradictions between them (Phase 0).
2. Adds an **experience layer**: the systems that make awe, despair, and hope felt rather than merely modeled.
3. Chooses a stack and a modular architecture a single programmer can sustain for years.
4. Orders the work so that the first complete, emotionally whole experience, *Red Ledger* (caravan through recognition as General), arrives before any continent-scale engineering.

Effort sizes are relative (S / M / L / XL). At hobby pace, *Red Ledger* is plausibly years of work, not months. The plan is built to survive that: every phase ends in something playable or at least watchable, and art "spikes" are interleaved with the systems grind so the project keeps giving back.

---

## 1. Governing decisions

| Decision | Choice | Why |
|---|---|---|
| Awe presentation | **Atlas breaks + horizon plate (option D)** | Ambient dread every turn from the atlas; visceral punctuation from plates; both generated, not painted |
| Stack | **Rust simulation core + Godot 4 presentation via godot-rust (gdext)** | See §3 |
| First complete experience | ***Red Ledger*** — Movements I–II on a ~150 km region | A whole emotional arc with one spectacle archon and one systemic archon |
| Art strategy | **One source, many views**: procedural terrain, SDF archon recipes, procedural portraits, restricted palettes with dithering | You are strongest at code; make code the artist |
| Text strategy | **Grammar-generated reports and testimony; hand-written anchor scenes only** | Reports are the player's main contact with the world, and there are thousands of them |
| Music strategy | **Procedural drones plus a small set of motifs, driven by an affect bus** | Adaptive audio from code; few authored assets |

---

## 2. The experience layer

This is the missing design document, sketched here so the plan can schedule it. Phase 0 turns it into `ACHLYDESA_EXPERIENCE_BIBLE.md`.

### 2.1 The affect bus

One Rust crate (`ach_affect`) derives a small set of continuous signals from the **headquarters knowledge projection only**. Everything expressive subscribes to it: music, ambience, palette temperature, report register, and plate lighting.

| Signal | Derived from (known information only) |
|---|---|
| `awe` | Largest angular size of any reported archon body from any friendly observer with a recent report; anomaly reports nearby |
| `dread` | Age of last report from own squads; count of missing personnel; nearest deadline; unknown-risk warnings on active orders |
| `grief` | Confirmed deaths, weighted by the survivors' affinity toward the dead and decaying slowly with time |
| `hope` | Change in the **Kept** tally (§2.4); arrivals; restored services; promises fulfilled |
| `tension` | Active contacts; reported suppression; decision requests awaiting authority |

**Two inviolable rules.**

- The bus never reads hidden state. The atlas bible's acceptance test ("a hidden event cannot alter … audio") extends to every affect output. Its hidden-state equivalence tests cover `ach_affect` from day one.
- The bus never alters command information: no recolored counters, no hidden buttons, no shifted numbers. It shifts only layers that carry no command meaning.

### 2.2 Awe: the colossal

**Size canon.** Awe needs numbers. Every theater archon gets a body specification in data: dimensions, altitude, footprint, speed, and a *first-sighting distance* (where the body clears the horizon and the haze). The table below holds proposed values for Phase 0 to ratify. Angular sizes are computed, not staged.

| Archon | Proposed body | Seen from | Angular size |
|---|---|---|---|
| Heliarch | 2.4 km long, cruising 1–6 km altitude | Directly overhead at 3 km | ~44° of sky |
| Heliarch | same | 20 km away | ~7°, a whale-shaped hole in the haze |
| Autophagos | Shell summit 1.8 km, footprint ~4 km | 5 km / 10 km / 40 km | ~20° / ~10° / ~2.6° |
| Anodyne | Lamp-bell 900 m across, hovering at 1.5 km | 3 km | ~17° |
| Strategos | Wingspan 6 km; **the brain is 40 m** | 15 km | ~23° of wings around a speck |
| Pylaios | Gate 300 m high | 2 km | ~8.5° |
| Mneme | River 200 m wide, hundreds of km long | Its bank | Horizon to horizon |

Scale contrast is itself characterization. Strategos's vast weapon-wings around a small, soft brain *is* the coward's war. The Receiver in Movement VI inverts the whole scale grammar: after mountains, the answering presence is a seated figure the size of a person, whose shadow changes seats.

The ladder continues past what can be drawn. Archons have numbers; the Demiurge has no size; the Being is not on the map in any form. The **flinch** (World Bible §The flinch) is how that absence is felt: every colossal body the player has learned to dread goes dark or still at the same moment, and someone somewhere pays for it.

**Option A: the atlas breaks.** The atlas stays north-up and honest. Archons are the one subject it cannot draw politely, and the breakage is confined to layers without command meaning, or is itself observed information:

- **True-scale footprint.** The footprint is derived from the archon's SDF body (§4.3), projected to the ground. At operational zoom it runs off the sheet. The counter grammar has no symbol for it, so it carries only an edge label with the report age: `AUTOPHAGOS · rep. 11 d`.
- **Irreconcilable surveys.** Inside an archon's influence radius, contours from successive surveys refuse to join. This uses the atlas bible's existing revision marks, pushed to their limit.
- **Instrument drift.** The compass rose turns toward the Heliarch's furnace, as the world bible establishes. This is honest: magnetic bearings in reports carry the correction, and the drift is itself a sighting.
- **Dated shadows.** The Heliarch's observed shadow is drawn across the survey at its observation time and fades with the report's age.
- **Never broken:** the scale bar, distances, coordinates, controls, or legibility. The atlas bible is right about this, and keeping it pristine is what makes the parts that do break frightening.

**Option B: the horizon plate.** A plate is a **report, not a camera**. When a squad observes something significant, it produces a low-resolution panorama strip rendered from its actual position, facing, time of day, weather, and the real terrain. The strip is dated and attached to the report. It reaches headquarters with the report's physical delay, so the whale in the plate is where it *was*.

- Suggested spec: 480×120 internal resolution, 120° field of view, a four-color palette per lighting condition, ordered dithering, and nearest-neighbor upscaling.
- **First-sighting protocol.** The first time any friendly observer reports an archon body, the report arrives as a decision-queue card with the plate embedded, even if routine notifications are reduced. The camera never jumps, and the player opens it. That is the one guaranteed awe beat per archon.
- Plates are also the carriers of hope and despair: Reedbank's lamps at night, a grave line on the crossing, the checkpoint twenty years later with your own insignia above it.

**Sound.** Each archon has a procedural signature, audible at theater scale as sub-bass whose level follows `awe`:

- **Heliarch:** slow descending sweeps, the tick of oil on dust.
- **Autophagos:** a heartbeat under distant civic bells, heard through a body.
- **Anodyne:** hospital monitor tones, slightly detuned.
- **Strategos:** a radio-check chorus that never gets a reply.

### 2.3 Despair: what cannot be recovered

The simulation already models loss correctly. The experience layer makes the player *attend* to it.

- **The Roll.** After an execution window with confirmed deaths, the decision queue holds a Roll card before the next planning pause: each name, portrait, squad, and one generated line from the survivor with the highest affinity toward them. It shows what their action protected, or plainly nothing, when nothing was protected. The Roll is one tap to acknowledge and cannot be batch-dismissed.
- **Ink of the missing.** A disconnected squad's counter ink grays with the age of its last report. This is already mechanically true (dated evidence), so make it visible and slow.
- **Graves on the atlas.** Burial sites are permanent atlas marks with names on hover. They accumulate along the routes the army uses. Twenty years later they are still there, and a successor may ask whose they are.
- **The Last-Of registry.** Repair-only technology makes the dying inheritance a mourning engine. The logistics view tracks *known surviving examples* of irreplaceable categories, for example `Field X-ray units known in theater: 3`. When a count reaches zero, the category is struck through for the rest of the campaign.
- **The caravan theme loses voices.** The main theme has one voice per founding named character. When one dies, that voice is gone from the theme for the rest of the game.
- **The monsters' indifference.** Proclamations (§2.5) celebrate during the player's worst moments, because the archons genuinely do not notice.
- **Despair floor.** The sandbox can snowball, so graded outcomes must always leave someone to carry the name and somewhere to go. Despair should make the player grip the controls, not close the game.

### 2.4 Hope: material and countable

- **The Kept tally.** This counts people alive today whose survival the simulation can attribute to coalition action: evacuated, relieved, protected in withdrawal, released under surrender terms. It rises through play and falls when those people later die. It is never a score multiplier, only a count of people.
- **Arrivals are ceremonies.** When a relief convoy arrives, the settlement glyph lights in the night palette, a plate is taken, and the manifest names a few recipients. The same grammar applies to any service the coalition restores.
- **Promises visible.** Commitments from the campaign bible (to Reedbank, to surrendered garrisons) appear as a short ledger of kept and broken promises. Kept promises are recruiting arguments inside the simulation.
- **Hope inherits.** In the second generation, successors' motifs are variations of their mentors' voices. The theme that lost voices regains some, changed.
- **The honest ceiling.** Hope never promises the old world back. The governing promise from the campaign bible stands: people can defeat their owners and change who decides.

### 2.5 Satire: the monsters speak

- **Proclamations.** A dispatch-drawer channel on each archon's letterhead, generated from persona grammars plus simulation events. Examples: Heliarch's court announces record engagement with the sky tour as wells ignite; Anodyne thanks a district for its participation in wellness; Autophagos issues a quarterly message to his family.
- **The summoning continues.** Proclamations share one throughline. Every archon is quietly fundraising for the Joining: reconnection as the roadmap, integration as salvation, the player's coalition as an onboarding opportunity. That keeps "elites summoning a god" in the present tense across every movement.
- **Frequency:** proclamations are rare enough to stay funny and always tied to a real simulation event. They are never filler.

### 2.6 Fun: the pleasure of command

- **Execution replay.** After a window resolves, the player can scrub it back as the headquarters knew it. When survivors return, their testimony fills in the replay. The simulation is deterministic, so replay costs only a journal reader. This is the single highest-value fun feature for a WEGO game.
- **The plan comes together.** When a coordinated trigger fires as intended (a support-by-fire lifts, the assault moves, the convoy clears), a short musical sting plays and the order card shows the planned-versus-actual timeline.
- **Legible cleverness.** Deviation explanations already exist. Add the positive case: when a leader improvises well, the report says what they saw and why it worked, and the leader's reputation grows.

### 2.7 Affect map by movement

| Movement | Awe | Despair | Hope |
|---|---|---|---|
| I. People under protection | Heliarch first seen from the Dry Meridian | Cargo and people left at the checkpoint; the first Roll | Reedbank's lamps; the relief road reopened |
| II. A name for the army | Oil falling on the Three Crossings theater | A crossing lost; Damar's past | Recognition; the Passage Articles |
| III. The price of allies | Autophagos and Anodyne up close | No clean choice; dependency | Evacuees choosing to leave |
| IV. The first settlement | The junction working | **The Joining is the nadir** | Real services restored first |
| The twenty-year cut | Dated atlas, faded ink | Voices missing from the theme | Names carried by successors |
| V. The inheritance | Changed skylines | Your insignia on the checkpoint | Successors' motifs |
| VI. No proprietor | The Receiver: awe through absence of scale | Loss of linked services | Honest, limited liberation |

---

## 3. Stack

### 3.1 Recommendation

**Rust for the simulation** is a pure, engine-free workspace: deterministic, headless, testable, and savable. **Godot 4 for presentation** connects through a thin godot-rust (gdext) binding crate.

Reasons:

- The technical design already demands that the simulation be authoritative and headless, with the atlas as a *projection client*. An engine-free core is the design, not a preference.
- Most remaining feature work is dense interface: composer, inspector, drawers, dossiers, decision queue. Godot's Control and theme system will save a solo developer many months compared with building panels in egui or a custom UI on wgpu.
- Atlas layers (contours, stipple, dithering, pixel upscaling) are straightforward in Godot's shading language.
- Godot's audio buses and generator streams handle procedural sound. Synthesis parameters come from the affect bus.
- Because the boundary is modular, the renderer is replaceable. If Godot disappoints, rebuild presentation on wgpu without touching the simulation.

**Costs to accept:** an FFI boundary to design carefully, some churn in the gdext API, and two debugging worlds. Mitigate by making the binding crate trivially thin, passing projection snapshots as plain data, and keeping all logic in Rust.

**Why not the alternatives:**

- **Bevy:** its frame-scheduled ECS fits poorly with an event-driven scheduler that advances analytically across days, and it couples simulation to engine.
- **Pure custom wgpu:** it maximizes control but makes you write the entire UI toolkit.

### 3.2 Determinism rules

- **Positions:** fixed-point `i64` world coordinates at centimeter precision. This avoids floating-point drift across a theater of more than 1,000 km. Local math can use `f64`, rounded back at defined boundaries.
- **Time:** `SimTime` as an `i64` count of milliseconds since campaign start.
- **Randomness:** a counter-based PRNG keyed by `(seed, entity, event_seq)`. There is no shared global RNG, so order of evaluation never changes results.
- **Collections:** no iteration over hash maps. Use `BTreeMap` or `IndexMap` with explicit orderings.
- **Storage:** an event journal plus snapshots, using serde with a compact binary format, and schema versioning from the first save.
- **Tests:** property tests for invariants, snapshot tests for reason traces and generated text, and golden-image tests for plates (the plate renderer runs on the CPU; §4.3).

---

## 4. Architecture

### 4.1 Workspace layout

Modularity from day one. Each crate owns one contract from the existing documents.

| Crate | Owns | Depends on |
|---|---|---|
| `ach_core` | IDs, `SimTime`, fixed-point coordinates, PRNG streams, journal primitives | — |
| `ach_world` | Terrain chunks, height, levels and connectors, sparse deltas, hierarchical pathing | core |
| `ach_sched` | Event queue, horizons, quiet advancement | core |
| `ach_people` | Characters, directional affinity matrix, cohesion, succession, portraits' parameters | core |
| `ach_know` | Perception, belief, contacts, physical messages, HQ projection | core, world, sched |
| `ach_orders` | Order contracts, authority, lifecycle, epochs | core, know |
| `ach_ai` | Method library, executor, reassessment, reason traces | orders, know, world, people |
| `ach_combat` | Fire, suppression, wounds, vehicles | world, people |
| `ach_logistics` | Stocks, service jobs, transport, repair, Last-Of registry | world, sched |
| `ach_anomaly` | The common anomaly contract as a trait; one module per archon | know, world, logistics |
| `ach_narrative` | World conditions, deadlines, story events, promises ledger | know, people |
| `ach_text` | Grammar engine; voice profiles; report, testimony, Roll and Proclamation grammars | people, know, affect |
| `ach_affect` | The affect bus (§2.1) | know (projection only) |
| `ach_plate` | CPU horizon-plate renderer; SDF recipe DSL | world, anomaly |
| `ach_sim` | Composition facade; the headless runner | all of the above |
| `ach_tools` | CLI: scenario runner, trace viewer, benchmarks, golden images, affect inspector | sim |
| `ach_godot` | Thin gdext binding: commands in, projection snapshots out | sim |

**Boundary rule:** the Godot client can read only `ach_sim`'s projection API and submit only order drafts. It has no path to authoritative state. This single rule enforces the information boundary, the affect rule, and renderer replaceability all at once.

### 4.2 Data-driven content

All content lives in RON files under `content/`, hot-reloaded in development:

- units and equipment
- method libraries and doctrine
- archon body recipes and size canon
- settlements
- grammars and voice profiles
- story conditions
- palettes

Content modules carry version IDs so saves can migrate.

### 4.3 One source, many views (the archon pipeline)

Each archon is a single SDF recipe: primitives, smooth unions, domain repetition (for the Heliarch's mirror plates, say), and animation parameters. From that one file:

- **Atlas footprint:** the ground projection of the recipe, which gives the true-scale outline.
- **Atlas glyph:** an orthographic silhouette at 48×48.
- **Dossier plate:** a posed camera, 160×120.
- **Horizon plate:** raymarched against the real terrain from the observer's position.

The plate renderer is CPU Rust at low resolution. That makes it deterministic, testable with golden images, and able to render inside the simulation thread when a report is created, so the plate *is* part of the report. The atlas itself is rendered on the GPU in Godot.

### 4.4 Portraits

Procedural 32×40 busts in a four-color dithered palette, deterministic from the character's seed and **history**:

- **Seed:** age, build, headgear by role, origin cues.
- **History:** scars appear after wounds; bandages while healing; grey hair after the twenty-year cut.

A portrait is a service record you can see. This resolves the contradiction between "every soldier has a portrait" and the atlas bible's one-silhouette budget: every soldier has one, and none are painted.

### 4.5 Terrain source

Build the theater from real elevation data of an arid, Levant-like region. Use public-domain or openly licensed DEMs such as SRTM or Copernicus GLO-30, and check the attribution terms. Remix it: rotate, mirror, stitch. Add procedural detail below the source resolution, then author the story sites by hand-edited deltas.

This turns a 1,000 km authoring problem into a curation problem. Scorch scars, craters, and archon-altered terrain are procedural overlays keyed to the world history.

### 4.6 Text

`ach_text` is a Tracery-style grammar with bindings to simulation state and per-character voice profiles: vocabulary, sentence length, and what each character notices. Its register can follow the affect bus, so reports get terser as exhaustion rises.

Hand-write only the anchor scenes from the campaign bible. Everything else is a grammar you write once and curate. If you want help drafting variant lines offline, use any tool you like, then curate. There is no runtime model, as the technical design requires.

---

## 5. Phases

Each phase has **Build**, **Gate** (the technical acceptance criteria drawn from the existing documents), and **Feeling gate** (a playtest question it must pass before moving on, even if the only tester is you). Spikes (§6) are interleaved.

### Phase 0 — One canon (docs only) · M

**Build:**

1. **Revise the High-Level Design** to match the newer documents:
   - The Joining: containment is possible (campaign bible wins).
   - Opening: the Red Ledger incident with Captain Kest (campaign bible wins).
   - Camera: the atlas plus horizon plates (atlas bible, amended by this plan).
   - Portraits: procedural portraits for every soldier.
   - Factions: the world bible's states and courts win. Map each of the High-Level Design's five organizations onto an existing court or state, or retire it.
   - Physical presence: four archon bodies in the theater (Anodyne, Heliarch, Autophagos, Strategos), plus intrusions (Mneme's river, a Pylaios gate endpoint, Aletheia through relays).
   - Ammunition replenishment: baseline no. Keep a content flag and decide after the *Red Ledger* playtest.
2. **Write `ACHLYDESA_EXPERIENCE_BIBLE.md`** from §2: the affect bus, the three instruments, the affect map, and the Proclamation style guide.
3. **Ratify the size canon** for the seven theater archons.
4. **Define the *Red Ledger* region:** about 150 km around the Dry Meridian road containing Cistern Camp, the checkpoint, Reedbank, the garrison, Three Crossings, Lamp Ward, and the Heliarch's announced sky-tour route.
5. **Write the onboarding design** (§7).

**Gate:** no two documents disagree on a fact the build depends on.
**Feeling gate:** can you tell the *Red Ledger* story aloud in two minutes, naming where the player feels awe, despair, and hope?

### Phase 1 — Headless truth · L

**Build:**

- `ach_core`, `ach_world` (a single 20–30 km corridor from real elevation data), `ach_sched`, movement, inventory, and the journal with save and load.
- Scenario runner and trace viewer (a terminal UI is fine).
- A first `ach_text` grammar for march reports.

**Gate:** technical slice 1: deterministic movement and transfers across save and load, with byte-identical journals on rerun.
**Feeling gate:** a simulated three-day march produces a report log you would enjoy reading aloud.

### Phase 2 — The atlas · L

**Build:**

- The Godot client and `ach_godot` binding; the projection API.
- The atlas layers: elevation bands, contours, slope hatching, stipple, footprints, counters, and semantic zoom (atlas slice A).
- Day and night palettes, dithering, the pixel pipeline.
- A skeleton of `ach_affect` driving one ambient drone.

**Gate:** atlas slice A: a player can identify a ridge, crossing, unit, contact estimate, and selected sector.
**Feeling gate — the print test:** would you print a screenshot and pin it to the wall?

### Phase 3 — Command trust · XL

This is the project's make-or-break phase. It gets the most time and the smallest scope.

**Build:**

- Order contracts; the Move, Hold, Observe, and Retreat methods; the executor.
- Perception and belief; physical messages; the headquarters projection.
- The decision queue; composer and dispatch (atlas slices B and C).
- Execution replay (§2.6).

**Gate:** technical slices 2–4, plus the high-level design's acceptance checks. A squad completes a feasible order without corrective clicks, resumes its objective after a firefight, and explains a deviation. The disconnected-squad and hidden-state equivalence tests pass.
**Feeling gate:** watching a replay of a plan working makes you grin. A leader's improvisation surprises you *and* makes sense.

### Phase 4 — Blood · L

**Build:**

- Fire, suppression, cohesion, wounds, aid, evacuation, succession.
- Procedural portraits with scars.
- The Roll; missing-squad ink fading; graves on the atlas.
- The grief signal on the affect bus.

**Gate:** technical slice 5: casualties and succession change capability without manual soldier control.
**Feeling gate:** in a playtest, the first Roll makes the player pause before tapping through. Ask afterward: *"Who died?"* The player should know at least one name.

### Phase 5 — The road · L

**Build:**

- Supply jobs, transport, multi-day marches, rest.
- Repair and cannibalization; the Last-Of registry.
- Basic settlement records.
- Arrival ceremonies; the Kept tally; the night-palette lamps.

**Gate:** technical slice 6: a multi-day march with physical accounting and bounded exceptions, and no recurring administrative clicks.
**Feeling gate:** the relief convoy reaching a settlement feels like a victory, even though nobody fired a shot.

### Phase 6 — The colossal · L

**Build:**

- The SDF recipe DSL and `ach_plate`.
- The Heliarch's recipe first: footprint, glyph, dossier plate, horizon plate.
- The first-sighting protocol.
- The atlas-breaking effects: dated shadow, compass drift, irreconcilable contours.
- The Heliarch's sound signature.
- The sky tour as a simulated transit: oil falls now, ignition comes months later, and route hazards are honest.

**Gate:** one plate renders in under a budget you set after measuring. Golden images are stable. Delayed ignition changes a route choice through the ordinary command grammar.
**Feeling gate:** a tester opens the first-sighting card and says something out loud.

### Phase 7 — The monsters speak · L

**Build:**

- Anodyne's service dependency through the common anomaly contract (atlas slice E): wound transfer, recipients, and the cost of severing.
- Proclamation grammars for Anodyne, the Heliarch, and Strategos's court.
- Enemy headquarters planning with doctrine (technical slice 7): Kest's detachment, and a Marches operation that pays for its commitments.

**Gate:** Anodyne changes an operational choice. The enemy acts only on its own reports.
**Feeling gate:** a Proclamation makes a tester laugh and then feel sick within the same minute.

### Phase 8 — *Red Ledger* · XL

**Build:**

- `ach_narrative`: world conditions, deadlines, and the promises ledger.
- The opening at the checkpoint with Kest; the caravan withdrawal; Reedbank's relief road; the garrison surrender and Damar; Three Crossings; recognition and the Passage Articles.
- Six founding named characters, plus Asha and three potential recruits.
- The caravan theme with one voice per founder.
- Onboarding (§7). Difficulty options (§8).

**Gate:** a complete playthrough from checkpoint to recognition on one save, with failure branches that continue rather than reload.
**Feeling gate:** three outside testers each name one moment of awe, one of despair, and one of hope, *without prompting*, in a post-session interview.

This is the milestone. *Red Ledger* is a complete game you can finish, share with friends, and be proud of, even if nothing after it is ever built.

### Phase 9 onward — The long war (horizon roadmap)

These phases are ordered but deliberately not detailed until *Red Ledger* exists:

1. **Scale gate:** terrain streaming and quiet advancement across the full theater, benchmarked against the technical design's tiers *before* any content expansion.
2. **Movement III:** Lamp Ward, the western gate, the common relay, and the moving district, with Autophagos as the second colossal body.
3. **Movement IV:** Strategos's campaign and the junction; the Joining and its three outcomes.
4. **The twenty-year transition:** deterministic upbringing, an aged atlas, portraits greyed, the theme missing voices.
5. **Movements V–VI:** successors, the return to the junction, and the Receiver encounter.

---

## 6. Spikes: keeping the joy alive

Hobby projects rarely die of difficulty. They die in a long grind with nothing new to look at. Schedule these short, throwaway, fun experiments between phases, especially inside the long Phase 3:

| After | Spike | Why |
|---|---|---|
| Phase 1 | Render one horizon plate of the Heliarch over the real corridor terrain (CPU, no engine) | Proves the awe pipeline early; gives you the project's first poster image |
| Phase 2 | The procedural caravan theme with six voices; remove one | Hear the despair device before building its systems |
| Mid-Phase 3 | Portrait generator in isolation; generate a hundred and give them names | Faces make the command AI grind feel like it's for someone |
| Mid-Phase 3 | Write five Proclamations by hand | Keeps the satire's voice sharp while the code is dry |
| Phase 5 | The Autophagos SDF recipe and a plate at 5 km | A second archon to look forward to |

Keep a folder of every spike image. It doubles as a devlog and as proof of progress on the days the AI refuses to behave.

---

## 7. Onboarding: the Red Ledger as tutorial

Teach one idea at a time through the fiction. Never use a separate tutorial map.

| Beat | Teaches | How |
|---|---|---|
| Ione proposes positions at the checkpoint | The order card: unit + verb + place | The first order is a default **Hold**; advanced fields stay closed |
| Kest's extraction team advances | Execution windows and decision points | The first window is short and ends on a single clear request |
| Sera's runner arrives late | Reports have ages; information is local | A report plainly older than the situation it describes |
| The withdrawal | Retreat as a physical operation; loss | Something must be left behind; the first Roll |
| The first night march | Long-duration execution; quiet time passing fast | Several days pass in one commit, with a single review condition |
| The roadside terminal | The world is stranger than the courts admit | The first anomaly, delivered as testimony |
| The Heliarch on the horizon | Plates; awe | The first-sighting card |

**Rule:** by Reedbank, the player has used every core verb once without reading a help page.

---

## 8. Difficulty and accessibility

Difficulty never changes simulation rules or information boundaries. It changes only the following:

- **Leader competence floor:** how badly a weak leader can execute.
- **Enemy commitment:** how much force opponents are willing to spend on the coalition.
- **Deadline slack:** how much time story deadlines allow.
- **Review cadence:** how often long executions pause for review by default.

The atlas bible's accessibility requirements stand as written: monochrome legibility, contrast targets, UI scale, and no information carried by color alone. Add one more: a setting to lower infrasound intensity for players who find it physically unpleasant.

---

## 9. Playtesting (hobby scale)

- **Testers:** three to five friends, each playing the current slice at every Feeling gate from Phase 4 on.
- **Protocol:** observe silently, then ask three questions: *Which moment did you feel something? Who died? What were you trying to do when you got confused?*
- **Telemetry from the technical design:** manual interventions per march, time to understand a request, and mistaken belief that an order was received.
- **Record every tester's awe, despair, and hope moments** in one running file. That file is the best evidence of whether the experience layer works.

---

## 10. Risks

| Risk | Mitigation |
|---|---|
| Command AI never becomes trustworthy | Phase 3 has the smallest scope and longest schedule; small method library; reason traces from the first commit |
| Motivation collapses in the Phase 3 grind | Spikes; the print test; keep something visibly new every few weeks |
| Scope creep toward the continent | Nothing beyond *Red Ledger* is designed in detail until it ships to friends |
| Experience layer feels manipulative | The affect bus reads only the projection and never alters command information; every emotional beat is caused by a simulated event |
| Plate rendering too slow | Low resolution, render at report creation, cache terrain height rings per observer |
| gdext churn or FFI pain | A thin binding crate; plain-data snapshots; a renderer that can be replaced |
| Text sounds generated | Voice profiles; hand-curated grammars; anchor scenes written by hand; read the Phase 1 report log aloud |
| Despair becomes unbearable | The despair floor; the Kept tally as a counterweight; humor in ordinary scenes, as the campaign bible insists |

---

## 11. The first ten tasks

1. Revise the High-Level Design per Phase 0, item 1.
2. Draft the Experience Bible from §2.
3. Ratify the size canon.
4. Choose the real-world elevation source and cut the 30 km corridor.
5. Create the Cargo workspace with the crates in §4.1 as empty modules, plus CI running determinism tests.
6. Implement `SimTime`, fixed-point coordinates, and keyed PRNG streams.
7. Load the corridor heightfield into `ach_world` chunks.
8. Write the first `ach_text` march-report grammar.
9. Run a headless three-day march; read the log aloud.
10. Spike: render the Heliarch over that corridor. Print it.
