# Conflict ledger

Status: PROPOSED

Produced by task P0-01 on 2026-10-01. Every conflict below is a contradiction between two statements that cannot both be true and that would lead two engineers to build different things. Each blocking or later conflict has one proposed ADR in `docs/decisions/`; Liam settles them in H-2. Resolutions follow the precedence in `CLAUDE.md` §Document map and precedence, and the proposals in Execution Plan §Phase 0 where they exist.

**Citation key.** Short names map to files in `docs/design/`: High-Level Design = `ACHLYDESA_STRATEGY_RPG_HIGH_LEVEL_DESIGN.md`; Campaign Bible = `ACHLYDESA_CAMPAIGN_AND_CHARACTER_BIBLE.md`; World Bible = `achlydesa-world-bible.md`; Field Atlas = `ACHLYDESA_FIELD_ATLAS_STYLE_AND_UI_BIBLE.md`; Technical Design = `ACHLYDESA_AUTONOMOUS_SQUAD_TECHNICAL_DESIGN.md`; Simulation = `ACHLYDESA_SIMULATION_AND_PLAYER_INTERACTIONS.md`; Execution Plan = `ACHLYDESA_EXECUTION_PLAN.md`. `§` names a heading, or for two cases a bold lead-in (Field Atlas "Presentation precedence"; Execution Plan Phase 0 "Build" item numbers). Section numbers are given where the heading has one.

**Severity key.** blocking = a Phase 0 or Phase 1 task consumes the answer (task named in each entry); later = Phase 2 onward; cosmetic = wording that P0-02 corrects without a decision.

**Summary table**

| ID | Topic | Severity | ADR |
|---|---|---|---|
| C-01 | Is the junction catastrophe unavoidable? | later | ADR-0001 |
| C-02 | Names of the factions contesting the junction | blocking | ADR-0002 |
| C-03 | The opening incident | blocking | ADR-0003 |
| C-04 | Camera and presentation | blocking | ADR-0004 |
| C-05 | Portraits for every soldier | blocking | ADR-0005 |
| C-06 | Which archons are physically present | blocking | ADR-0006 |
| C-07 | Replenishing mundane ammunition and fuel | later | ADR-0007 |
| C-08 | Awe presentation versus the anti-spectacle rules | blocking | ADR-0008 |
| C-09 | Floating point in simulation math | blocking | ADR-0009 |
| C-10 | What the second generation inherits | later | ADR-0010 |
| C-11 | Who commands the caravan | cosmetic | none |
| C-12 | Length of the two-generation span | cosmetic | none |

---

## C-01 — Is the junction catastrophe unavoidable?
**Severity:** later (Phase 8 and the generational content depend on it; P0-02 rewrites the High-Level Design text)
**Sources:**
- High-Level Design §18.2 The catastrophe: "The player cannot prevent the rupture." The same section calls the midpoint "an inevitable, large playable operation."
- High-Level Design §18.0c An open war with a story spine: "The story must not pretend the player can permanently prevent it." A rival claimant precipitates the rupture if the coalition refuses.
- Campaign Bible §5 Campaign architecture, "The Joining — a crisis with different outcomes": "Prepared containment must be possible." It also forbids manufacturing "an unstoppable betrayal to punish competent play."
- Campaign Bible §5 Campaign architecture, "The Joining" table: three outcomes (broad integration, emergency severance, prepared containment), each with a different inheritance.
- Simulation §25.4 Authored catastrophe boundary: the story "does not claim the player can indefinitely prevent the central transition"; delay or reframe, never reset.
- World Bible §The caravan and the junction, "False victory without false agency": the rupture is conditional ("If the two-generation structure requires an unavoidable rupture").
**Proposed resolution:** The Campaign Bible wins (Execution Plan §Phase 0 build item 1). A regional synchronization attempt is unavoidable in the sense that rival claimants will try it; its outcome is not. Prepared containment is a reachable, costly success. The twenty-year jump stays in every branch.
**ADR:** ADR-0001

## C-02 — Names of the factions contesting the junction
**Severity:** blocking (P0-06 places the Marches garrison and other sites; Phase 1 content names polities)
**Sources:**
- High-Level Design §18.0b Factions contesting it: five invented organizations (Strategos's Custodians, Heliarchic Mandate, Gate Compact of Pylaios, Autophagan Delegation, Free Cistern League). It states they "are additions for the game, not names already established in the world bible."
- World Bible §States made from services: seven polities (Basin Commonwealth, Shell Commonwealth, Lamp Concord, Threshold Principalities, Clear Republic, Assurance Marches, Ribbon Houses).
- Campaign Bible §7 Political counterparts and antagonists: counterparts hold offices in those polities (Marches inspection captain, Council of Returns, Lamp Concord superintendent, Threshold concession ruler, Clear Republic investigator).
- World Bible §The caravan and the junction, "Why everyone wants the junction": nine factions and constituencies, none of them the five invented names.
**Proposed resolution:** The World Bible wins. Map Custodians to the Assurance Marches, Gate Compact to the Threshold Principalities, Autophagan Delegation to the Shell Commonwealth, and Free Cistern League to the Basin Commonwealth's municipal companies and towns. Retire the Heliarchic Mandate: no polity is built on the Heliarch (see ADR-0002 for the reasoning).
**ADR:** ADR-0002

## C-03 — The opening incident
**Severity:** blocking (P0-06 lays the opening on the terrain; P0-07 designs its beats)
**Sources:**
- High-Level Design §18.0 The caravan opening: a caravan technician "uses a surviving maintenance credential to restore passage"; the checkpoint then reclassifies the caravan. The player "begins as the caravan's guard commander."
- Simulation §25.2 Opening implementation: the technician's attempt to restore passage reconnects to the junction and "Classification changes create the seizure demand."
- Campaign Bible §4 The opening: the Red Ledger incident: Captain Varo Kest "receives a directive whose effective date precedes the caravan's departure" and enforces it knowing the papers were valid. The junction first appears afterward, when a roadside terminal recognizes a rejected claim.
- World Bible §The caravan and the junction, "The opening incident": at a Marches inspection point "an updated assurance directive invalidates some passengers' passage"; the guards carry no mysterious box.
**Proposed resolution:** The Campaign Bible wins (Execution Plan §Phase 0 build item 1). The trigger is Kest's backdated directive, not a technician's credential. The junction is discovered afterward through the terminal. Simulation §25.2 conflicts with this too, and ADR-0003 flags it.
**ADR:** ADR-0003

## C-04 — Camera and presentation
**Severity:** blocking (P0-03 Experience Bible and every Phase 2 task assume one view)
**Sources:**
- High-Level Design §5.1 Units and simulation scale: "The isometric camera snaps among four cardinal angles," with cutaway roofs and visible individual soldiers at close zoom.
- Simulation §5.1 Main map: "Four snapped camera orientations, cutaway roofs, and a floor selector keep the 2.5D geometry readable."
- Field Atlas "Presentation precedence": replaces "the earlier requirement for an oblique 2.5D camera, four snapped viewing angles, visible individual people at close zoom, and literal cutaway roofs."
- Execution Plan §Phase 0 build item 1: "Camera: the atlas plus horizon plates (atlas bible, amended by this plan)."
**Proposed resolution:** The atlas, north-up and top-down, with horizon plates as reports (Execution Plan). Height, floors and line of sight remain simulation properties. Simulation §5.1 is a second stale source that P0-02 does not edit.
**ADR:** ADR-0004

## C-05 — Portraits for every soldier
**Severity:** blocking (P0-03 specifies the Roll card, which shows each name and portrait; P1-09 defers portraits to Phase 4)
**Sources:**
- High-Level Design §9.3 Generic and unique characters: every generic soldier has "a portrait."
- Simulation §19.1 People as persistent participants: every soldier has "name, portrait, origin, skills," and the rest.
- Field Atlas §2 Visual construction and production budget: the first-playable budget has "one portrait silhouette with text-led identity."
- Execution Plan §4.4 Portraits: resolves it with procedural 32×40 busts; "every soldier has one, and none are painted."
**Proposed resolution:** Every soldier has a procedural portrait generated from seed and service history. The Field Atlas budget counts painted art only, so the contradiction dissolves. The player's own avatar still has none (Campaign Bible §2 The unseen general).
**ADR:** ADR-0005

## C-06 — How many archons are physically present, and which
**Severity:** blocking (P0-04 sizes only theater archons; P0-06 places their bodies)
**Sources:**
- High-Level Design §18.1 First generation: "three or four Archons are physically present and widely separated"; it never says which.
- Simulation §24.2 Seven theater powers: "Three or four bodies are physically present in the theater"; it never says which.
- Campaign Bible §5 Campaign architecture, Movement III: the player can fight Anodyne, assault or evacuate Autophagos, face a Heliarch sky-tour, cross Mneme, use a western gate and seize an Aletheia relay.
- Campaign Bible §5 Campaign architecture, Movement IV: Strategos is "the principal first-generation antagonist," with a campaign against him and his brain.
**Proposed resolution:** Four bodies in the theater: Anodyne, Heliarch, Autophagos, Strategos. Mneme's river, a Pylaios gate endpoint and Aletheia's relays are intrusions, not roaming bodies (Execution Plan §Phase 0 build item 1).
**ADR:** ADR-0006

## C-07 — Whether old facilities can replenish mundane ammunition and fuel
**Severity:** later (Phase 3-5 logistics and balance; P1-10 uses the stock kinds but not the answer)
**Sources:**
- High-Level Design §16.4 Restoration orders: "Whether restored old facilities may replenish mundane ammunition or fuel is a remaining scope decision; the baseline must work without that permission."
- Simulation §17.1 Strict technological boundary: processed military fuel is finite; old-facility replenishment "would require a deliberate later rules change."
- Technical Design §18.2 Inventory conservation: production "from permitted existing services" is a typed delta, so the permission is assumed to exist somewhere.
- Execution Plan §Phase 0 build item 1: "baseline no. Keep a content flag and decide after the *Red Ledger* playtest."
**Proposed resolution:** Baseline no, decided now rather than left open. Keep a content-level flag, default off, so the playtest can change it without a code change.
**ADR:** ADR-0007

## C-08 — Awe presentation versus the anti-spectacle rules
**Severity:** blocking (P0-03 builds the Experience Bible from the Execution Plan's answer)
**Sources:**
- Field Atlas §2 Visual construction and production budget: archive plates "do not supply unique tactical information," and the first three layers "remain useful with every archive plate removed."
- Field Atlas §6.2 Surrealism as mapped relationships: impossible geometry may appear "in a plate or an explicitly observed map feature"; controls keep stable positions, labels and meanings.
- Field Atlas §4 Terrain, scale, height, and semantic zoom: the map "remains north-up," and a scale bar and coordinates stay available at every scale.
- Campaign Bible §10 The optional encounter inside the Soterion, "What appears": the Receiver gets "a small, unsettling archival plate and a legible operational footprint," not an animated titan.
- Execution Plan §2.2 Awe: the colossal: the true-scale footprint "runs off the sheet" with only an edge label, the compass rose turns, and a horizon plate is delivered as a decision-queue card (the "first-sighting protocol").
**Proposed resolution:** The Execution Plan's atlas-breaks-plus-horizon-plate approach governs, within the Field Atlas's legibility rules: never break measurements, controls or labels. Every plate's information is also carried as text in its report, so the plate stays optional.
**ADR:** ADR-0008

## C-09 — Floating point in simulation math
**Severity:** blocking (P1-01 and P1-02 encode the rule in crate roots and core types)
**Sources:**
- Execution Plan §3.2 Determinism rules: "Local math can use `f64`, rounded back at defined boundaries."
- `CLAUDE.md` §Simulation crates and their hard rules: "No floating-point arithmetic. Each crate root has `#![deny(clippy::float_arithmetic)]`."
- Technical Design §22.1 Deterministic contract: bit-identical replay across platforms is not promised by default. CLAUDE.md requires byte-identical journals.
**Proposed resolution:** No floating point in simulation crates, as CLAUDE.md states. Float is allowed in `ach_tools`, `ach_godot` and `spikes/`, and at no point in a value that returns to simulation state.
**ADR:** ADR-0009

## C-10 — What the second generation inherits
**Severity:** later (Phase 8 horizon and the twenty-year transition)
**Sources:**
- High-Level Design §18.3 Twenty years later: the new coalition "begins with fragmented cells, caches, a few damaged bases, partial records."
- High-Level Design §18.2 The catastrophe: "Most veterans disappear through a mixture of confirmed death, unresolved disappearance, captivity, and transformation."
- Campaign Bible §5 Campaign architecture, Movement V: depending on the settlement the army may "liberate communities from a continuity regime, reunite a fractured corridor, or defend a functioning federation."
- Campaign Bible §9 The second generation: surviving veterans "may return as advisers, civilians, opponents, or field officers"; Eren is a still-active veteran, and Ione may serve on staff.
**Proposed resolution:** The Campaign Bible wins. The inheritance is one of three states keyed to the Joining outcome; fragmented cells are the broad-integration and severance case, not the universal start. Veteran fate is a per-character outcome of first-generation choices, not a default.
**ADR:** ADR-0010

## C-11 — Who commands the caravan
**Severity:** cosmetic
**Sources:**
- High-Level Design §18.0 The caravan opening: the player "begins as the caravan's guard commander."
- Campaign Bible §2 The unseen general: the player is "command authority, not as an authored individual"; Ione Var is the visible escort captain.
- World Bible §Character roots for the SRPG: "Ione Var, 41 — caravan commander," whose life before the caravan is as a Basin company leader.
**Proposed resolution:** The player is the unseen general; Ione is the escort captain who executes in the field. P0-02 rewrites High-Level Design §18.0 accordingly. The World Bible's "caravan commander" is read as her civilian-facing title and needs no edit (the roster table says names and assignments can change in campaign writing).
**ADR:** none — cosmetic, fixed in P0-02

## C-12 — Length of the two-generation span
**Severity:** cosmetic
**Sources:**
- High-Level Design header "Setting": "approximately twenty years across two generations."
- Campaign Bible §9 The second generation: second-generation ages add "the actual first-campaign duration and the approximately twenty-year interval."
- Campaign Bible §5 Campaign architecture, Movement V: the handover is "roughly twenty years after the first settlement."
**Proposed resolution:** The span is the first campaign's length plus roughly twenty years. P0-02 corrects the High-Level Design header (the header line is front matter, not a heading, so the citation is by its "Setting" label).
**ADR:** none — cosmetic, fixed in P0-02

---

## Method and coverage

Searched all seven design documents with `grep -n` for: faction and state names; archon names near "present", "theater", "region"; squad sizes, theater size (1,000-1,500 km versus the World Bible's 1,200 km), grid spacing (two meters), `A238`, "twenty years"; "camera", "portrait", "tutorial", "research", "manufacture"; and all twelve first-generation character names and ages.

**Checked and not recorded (not contradictions):**
- Theater size: 1,000-1,500 km (High-Level Design §4.1; Field Atlas §4) contains the World Bible's approximately 1,200 km (World Bible §States made from services).
- Squad size is five to eight in the High-Level Design, Simulation and Technical Design alike.
- Grid: two-meter sampling is a hypothesis in High-Level Design §4.1a; Execution Plan §3.2 stores positions at centimeter precision. These are different layers.
- Region sizes: the 20-30 km corridor (High-Level Design §22.2; Execution Plan Phase 1) and the 150 km Red Ledger region (Execution Plan Phase 0 item 4) belong to different phases.
- Character ages and eras are consistent at A238 between the World Bible roster and the Campaign Bible. Titles differ only as in C-11.
- No design document describes a tutorial map that contradicts Execution Plan §7 Onboarding.
- Edits in P0-02 are limited to the High-Level Design and one CLAUDE.md line (`docs/tasks/P0-02.md` Deliverables). The stale passages in other documents named above (Simulation §5.1, §25.2, Execution Plan §3.2) are outranked by an Accepted ADR but are not corrected by P0-02; see Open questions in the P0-01 report.
