# Achlydesa — Onboarding Design

Status: PROPOSED

Produced by task P0-07 on 2026-10-03. Expands Execution Plan §7 into eight buildable beats for the Red Ledger opening. Phase 8 builds it; Phases 2 to 5 use it to decide which UI must exist and in what order. Once Liam accepts it, it sits at the level of the Execution Plan (see `CLAUDE.md` §Document map and precedence).

**Conventions.**

- **Citation key.** World Bible = `achlydesa-world-bible.md`; Campaign Bible = `ACHLYDESA_CAMPAIGN_AND_CHARACTER_BIBLE.md`; Field Atlas = `ACHLYDESA_FIELD_ATLAS_STYLE_AND_UI_BIBLE.md`; Simulation = `ACHLYDESA_SIMULATION_AND_PLAYER_INTERACTIONS.md`; High-Level Design = `ACHLYDESA_STRATEGY_RPG_HIGH_LEVEL_DESIGN.md` (stale until P0-02; used only for the verb list, which nothing above covers); Experience Bible = `ACHLYDESA_EXPERIENCE_BIBLE.md` (itself PROPOSED); Execution Plan = `ACHLYDESA_EXECUTION_PLAN.md`; Region = `docs/canon/RED_LEDGER_REGION.md` (PROPOSED). Design documents are in `docs/design/`.
- **PROPOSED** marks every invention, with a one-line reason. Unmarked statements about canon carry a citation.
- **Numbers.** Timings and distances come from `docs/artifacts/P0-07/onboarding_timeline.py` (output in `onboarding_timeline.txt`). It reads the route and site data in `content/world/`. It is review evidence, run 2026-10-03; the simulation uses none of it. The script is not under `tools/scripts/` because this task may touch only `docs/artifacts/P0-07/`.
- **Names.** "Eren", "Petra" and the rest are the first-generation cast of Campaign Bible §6. "The Hold line", "the rally" and similar are scenario terms of this document, PROPOSED.

---

## 1. Design stance

Execution Plan §7 sets the rule: "Teach one idea at a time through the fiction. Never use a separate tutorial map." Four consequences, each from existing canon:

1. **Teaching rides decision points.** The Simulation names what may stop execution (Simulation §3.3) and the Field Atlas gives the decision card its form (Field Atlas §12.1, §19). Every lesson below arrives as a proposal, a decision card, or a report. Simulation §25.2 says tutorial prompts "teach selection, purpose, support, and review at natural planning pauses"; here those prompts are proposals from named characters, as plain text, never a modal.
2. **The first order is the smallest order.** "A new player can issue a default valid Hold order without opening advanced fields" (Field Atlas §23.2). Advanced fields open only when a situation needs a constraint (§4).
3. **No scripted outcomes.** Story objectives are conditions on the theater, "not instanced missions" (Simulation §25.1), and the story "does not bypass those systems to guarantee a scripted arrival" (Campaign Bible §12). Each beat's trigger is a world condition (§3).
4. **Feeling comes from the simulation's own facts.** Awe from computed angular size (Experience Bible §4a); despair from a record the player must read (Experience Bible §5.1); hope from a count of people (Experience Bible §6.1). The onboarding only arranges for the player to be standing where these facts occur.

### 1.1 Resolutions of source conflicts (all PROPOSED)

| # | Sources in tension | Resolution | Reason |
|---|---|---|---|
| R-1 | Execution Plan §7 lists the night march before the roadside terminal; Region §6 puts the terminal 6.0 km past the post, "on the road the survivors take". | Terminal first (beat 5), then the long march (beat 6). | Geometry: the column passes the terminal in its first six hours. The script puts it at road km 37.5, the post at 31.5. |
| R-2 | Execution Plan §7: "Sera's runner arrives late". Campaign Bible §6: Sera Pell is "encountered during Reedbank relief". | The runner is Eren Tal's. Sera first appears after Reedbank (outside this document). | The Campaign Bible outranks the Execution Plan. Sera cannot be in the column before Reedbank. Eren is "present from the opening" (Campaign Bible §6). |
| R-3 | Region §5: the tour enters at 08:00 "on the day of the checkpoint", so the whale is at its nearest to the post during the standoff. Execution Plan §7 and Experience Bible §9 make the Heliarch the *last* opening beat. | Move the tour entry to 08:00 on the day after the checkpoint (**PROPOSED amendment to Region §5**, to be made by a later task; this task may not edit it). | A 17° whale over the post, three hours into the standoff, would arrive during the densest decision of the opening and break "one idea at a time". With entry on day 1, the script gives a peak of 25° to 30° over the column at road km 47 to 58, for any pace from 18 to 30 km a day (§3 beat 7). The Lower Crossing's 84° wall stays unspent for Movement II (Region §5). |
| R-4 | Execution Plan §7: several days pass in one commit. Region §4: the column covers 74.4 km in about three days. | The long march window is one commit of 31 to 51 simulated hours (terminal to Reedbank, 38.5 km). | The script: 38.5 km at 30, 24 and 18 km a day is 31, 38 and 51 hours. The wording "several days" is not reachable on this geometry. See Open question 2. |
| R-5 | Execution Plan §7 has no hope beat. Experience Bible §9 lists "Reedbank's lamps; the relief road reopened" as Movement I's hope. | Add beat 8, Reedbank. | The task goal asks for a first hope beat, and the Experience Bible already names it. |

---

## 2. Verb and UI sources (grep evidence)

The 11 core verbs (**Move, Observe, Suppress, Kill, Assault, Hold, Support, Retreat, Resupply, Evacuate, Recover**) are in High-Level Design §5.5 ("Core order vocabulary"). The same list, with minimum inputs and infeasible-order defaults, is in Simulation §26.1 and Field Atlas §10.1. The six verbs the invariant names are Move, Hold, Observe, Retreat, Resupply, Evacuate; the onboarding uses none of the other five.

Interface names are the first column of the Field Atlas §8 table: Atlas, Unit inspector, Order composer, Operation sheet, Dispatch review, Decision queue, Report drawer, Intelligence dossier, Service sheet, Asset/site sheet, Personnel dossier, Doctrine sheet, Contact/agreement sheet, Campaign record, Save/options. Other control names used below are quoted from the section cited: `Stage order` (Field Atlas §7.2), `Commit and execute` (Field Atlas §10.2), `Execute to review` (Field Atlas §7.2), `Review dispatch (n)` (Field Atlas §7.2), `Keep fallback` (Field Atlas §12.1), `Show on atlas` and `Acknowledge` (Experience Bible §4d, §5.1). There is no help interface in Field Atlas §8 at all, so "without opening help" is met by construction; the invariant tests that nothing outside §8 is needed.

---

## 3. The beats

Common setting (**PROPOSED**; reason: the beats need one clock). The scenario starts at 06:00 on checkpoint day C. The column is 3 km west of the Meridian Inspection Point by road (road km 28.5), with the player already holding command authority as the unseen general (Campaign Bible §2, "At the opening, the player already coordinates the caravan's defense"). Radio emission is restricted within 5 km of the post (**PROPOSED** reason: Simulation §9 lets the checkpoint's detachment listen, and the restriction is why the Bluff picket sends a runner in beat 3). Movement is at 18 to 30 km a day for a guarded foot column (High-Level Design §4.1b).

Every beat has the ten fields the task requires. "Minimum valid action" is the least the player can do and still be playing correctly. Experience Bible sections give the feeling's instrument.

### Beat 1. Ione proposes positions

| Field | Specification |
|---|---|
| **Trigger** | Scenario start: the column's lead element is within 3 km of the inspection point by road, and the post is a known site (Field Atlas §5.1 "Fixed site"). |
| **Where** | `inspection_station`; the column on `dry_meridian` at road km 28.5. |
| **Newly visible UI** | Atlas (with friendly counters and a Fixed site mark); Unit inspector; Order composer showing only issuing HQ, selected unit(s), verb, target (Field Atlas §10.1 "Show the minimum fields first"); `Stage order`; Dispatch review via `Review dispatch (n)`; `Execute to review`. |
| **Verbs available** | Hold. (The verb list is present in the composer, but Ione's card pre-selects Hold; the others are not suggested.) |
| **What the player does** | Ione, the escort captain, proposes a position for each squad. The first proposal is a Hold on the road short of the post. The player selects the escort squad, accepts or moves the position, stages the order, reviews dispatch, and executes. **Minimum valid action:** accept Ione's Hold proposal unchanged, `Commit and execute`. The order record's stated purpose names what it protects (the 14 travelers on the directive, §3 beat 2), which feeds the Roll's "Protected" line (Experience Bible §5.1.1). |
| **What is taught** | An order is a unit, a verb and a place, and nothing happens until it is dispatched (Execution Plan §7: "The order card: unit + verb + place"). |
| **Who speaks** | Ione Var, proposal (a plain-text card in the Order composer). **PROPOSED** line: "Escort squads hold at the culvert, 2.5 km short of the gate. The list has fourteen names. Wagon three has all fourteen." |
| **Outcomes** | (a) Hold accepted as proposed: the escort stands on the road with the column behind it. (b) The player moves the Hold west or east of the culvert: the position is legal, and beat 2's geometry follows from it. (c) The player stages nothing and executes: the column keeps ordinary march orders, `Execute without new orders` per the session machine (Field Atlas §15), and beat 2 arrives with no Hold in place, so the card's fallback is the leader's own adaptation (Field Atlas §19). None needs a reload. |
| **Feeling** | Despair, seeded. The proposal names fourteen people and the wagon that holds them, so the later Roll's "Protected" line has a denominator (Experience Bible §5.1.1). |
| **Built in** | Phase 3 (Field Atlas §23.1 slice B; Roadmap §Phase 3 "decision queue, order composer, and dispatch"); atlas and counters from Phase 2. Content in Phase 8. |

### Beat 2. Kest's extraction team advances

| Field | Specification |
|---|---|
| **Trigger** | Kest's extraction team (a hostile element from the post) within 1.5 km of the Hold position, with the 14 listed travelers still on the road. This follows from Campaign Bible §4: "an official extraction team advances if the guards do not surrender them." |
| **Where** | `inspection_station`, `station_ridge` (the North Bluff, 3.6 km north-north-west of the post, Region §6). |
| **Newly visible UI** | Decision queue with one card (Field Atlas §12.1); `Keep fallback`; hostile-element counter, a diamond with contact ID (Field Atlas §5.1); Contact/agreement sheet, opened only if the player picks the third alternative; the top strip's change from Executing to Planning (Field Atlas §15). |
| **Verbs available** | Hold (already in play); Evacuate (first appearance, on the card). |
| **What the player does** | The first execution window is short and ends on a single request (Execution Plan §7). Ione's card states what changed, who reported and when, what continues, and two or three alternatives (Simulation §26.4 form). The alternatives: **Evacuate** the 14 to the North Bluff, out of sight of the post; **Keep fallback** (the Hold stands; Kest's team reaches it in about 25 minutes); **Send terms** to Kest for time, which opens the Contact/agreement sheet. The recommendation is Evacuate, grounded in what the card can cite: the bluff is out of the post's sight (Region §6). **Minimum valid action:** choose any alternative, or `Keep fallback`. |
| **What is taught** | The simulation stops only when a decision needs the player, and you answer a stated problem, not a general prompt (Execution Plan §7). |
| **Who speaks** | Ione Var, decision request. **PROPOSED** line: "Eleven men and a truck at 1.4 km. They want the fourteen. Our line stands. Decision useful before 06:50." |
| **Outcomes** | (a) Evacuate: the 14 reach the bluff; the team finds the Hold line and no travelers, and Kest's standing orders still apply (Campaign Bible §4: they "still require impoundment"). (b) Keep fallback: the team reaches the line; the travelers are either surrendered or defended, which the player decides in the next window. (c) Send terms: "Negotiation can release people and improve positioning" (Campaign Bible §4); the reply is a real message with a real delay, and it may refuse. Each leads to beat 3 and beat 4 without a reload. |
| **Feeling** | Despair, building: the first time the player sees how many people one command protects. No instrument fires yet. |
| **Built in** | Phase 3 (decision queue, slice C); Evacuate as a physical chain in Phase 4 (Execution Plan §5 Phase 4: "aid, evacuation"); the Contact/agreement sheet in Phase 7 or 8 (**PROPOSED**: if it is not ready, the third alternative is simply absent, which the card handles because alternatives are "proposals", Simulation §26.4). |

### Beat 3. Eren's runner arrives late

| Field | Specification |
|---|---|
| **Trigger** | A runner from the North Bluff picket physically reaches headquarters carrying a report whose observation time precedes receipt by 30 minutes or more. The delay is not scripted: it is a bluff-to-column distance of about 4 km on foot, under the emission restriction. The picket exists in the scenario as an ordinary standing arrangement ("Other guards retain ordinary march orders", Simulation §27.1 step 1), so it does not depend on the player's beat 1 choices. |
| **Where** | `station_ridge`, `inspection_station`. |
| **Newly visible UI** | Report drawer (Field Atlas §12.2) with both timestamps; the counter's "Last reported position" anchor pin with age (Field Atlas §5.1); Intelligence dossier, as an optional route to "create observation intent" (Field Atlas §8). |
| **Verbs available** | Hold, Evacuate (in play); Observe (first appearance). |
| **What the player does** | The report says what Eren saw at the post gate when he looked, and the drawer shows `observed` and `received` times. The recommended response is an Observe order on the post gate from the Bluff (Simulation §26.1: Observe needs "Area, object, or intelligence question"). **Minimum valid action:** open the report and read its ages. Optionally, stage Observe. |
| **What is taught** | Reports have ages, and information is local (Execution Plan §7: "A report plainly older than the situation it describes"). |
| **Who speaks** | Eren Tal, report (carried by a runner). **PROPOSED** line: "Bluff wall, 06:35. Gate open. Two trucks, one with a stretcher bay, still loading. Eleven men visible. Runner leaves 06:41." Received 07:22: the drawer shows both, the compact row one age (Field Atlas §12.2). |
| **Outcomes** | (a) The player orders Observe: later reports follow the same slow route, which is itself a lesson about the delay. (b) The player ignores the report: the estimate area on the anchor pin keeps growing, and the stretcher truck's position is a fact the player never learned. (c) The player sends the report on as a message to another element: it "incurs delivery" (Field Atlas §12.2). None blocks beat 4. The stretcher truck matters because "stopping to treat the wounded changes the pursuers' position" (Campaign Bible §4). |
| **Feeling** | Despair, quiet: the player cannot see what is happening now. This is dread through uncertainty, not a Roll. |
| **Built in** | Phase 3 (perception, physical messages, projection; Report drawer, slice C). Eren's picket and runner are Phase 8 content. |

### Beat 4. The withdrawal

| Field | Specification |
|---|---|
| **Trigger** | The Hold position is untenable by the leader's own report, or the extraction team is within 300 m of it with the travelers still there (**PROPOSED** threshold; reason: it is the range where Simulation §12 contact begins to matter for unarmored carriers). The player may also stage Retreat at any earlier time. |
| **Where** | `inspection_station`; the retreat destination is `station_ridge`, the rally, because "the post cannot see the bluff" (Region §6). |
| **Newly visible UI** | The Roll card, first in the Decision queue (Experience Bible §5.1) when a confirmed death has been received; Campaign record; Personnel dossier (opened from a Roll entry); `Acknowledge` (Experience Bible §5.1.4); the Last-Of registry line in the Report drawer if the category is tracked (Experience Bible §5.4). |
| **Verbs available** | Retreat (first appearance); Hold, Evacuate, Observe (in play). |
| **What the player does** | A transport loses mobility (Simulation §27.1 step 5). The escape deadline is the extraction team's closing time. The card from Tessa Ruun states the choices: leave the carrier and its cargo; unload what fits onto the two remaining wagons and leave the carrier; hold the rear longer so Tessa's crew can recover it. **PROPOSED physical constraint:** after the carrier is lost, remaining capacity is at least 15% short of the loaded mass within the deadline, so something must be left (tunable in Phase 8; reason: "Something must be left behind" is Execution Plan §7's own wording, and a capacity gap makes it physics, not script). **Minimum valid action:** choose Retreat to the rally, accepting the default of leaving the carrier. |
| **What is taught** | Retreat is a physical operation, and loss is a quantity (Execution Plan §7; High-Level Design §10.6: units "carry what and whom they can, abandon what they cannot"). |
| **Who speaks** | Tessa Ruun, decision request; later the Roll's survivor line (Experience Bible §5.1.2). **PROPOSED** line: "Carrier three won't turn. Forty-one crates aboard. We can move twenty-six in the time we have." |
| **Outcomes** | (a) Leave the carrier and everything on it: no casualty at the line, the cargo falls to Kest (his promotion is "tied to recovering requisitioned stock", Campaign Bible §4), and a clinical shortage follows later. (b) Unload the clinical freight, leave the carrier: the stock survives, the carrier is lost, and if the registry tracks "Oldest-pattern heavy carriers" it records a loss (Experience Bible §5.4). (c) Hold the rear longer: the carrier may be recovered at the price of the rear guard. **The Roll is not scripted.** It appears at the first planning pause after a confirmed death has been received (Experience Bible §5.1). If beat 4 ends with none, the despair is carried by a "Left behind" line on the card and in the Campaign record, and the first Roll comes with the first death anywhere. A patient in Neris's care, critical at the start, is the likely source: whether they live depends on whether the clinical freight left the road (Simulation §15.1). See Open question 3. |
| **Feeling** | Despair (Experience Bible §5.1, §5.1.1). |
| **Built in** | Phase 3 (Retreat method); the Roll, graves and the Campaign record in Phase 4 (Experience Bible §5 table, "Requires Phase 4"); the Last-Of registry in Phase 5. |

### Beat 5. The Milestone Terminal

| Field | Specification |
|---|---|
| **Trigger** | A column element carrying a rejected travel claim comes within 200 m of the roadside terminal with no contact for 60 minutes (**PROPOSED** quiet interval; reason: the terminal must not compete with a firefight). Per Campaign Bible §4: "Afterward, an old roadside terminal recognizes one of the rejected travel claims". |
| **Where** | `roadside_terminal`, 6.0 km past the post on `dry_meridian` (Region §6). |
| **Newly visible UI** | The Report drawer's testimony class (Field Atlas §12.2: "Separate observation, testimony, estimate, and inference"); Campaign record (its testimony list); Asset/site sheet, read-only, for the terminal. |
| **Verbs available** | Hold, Retreat, Observe, Evacuate (all already in play). Move is not suggested here. |
| **What the player does** | Mara Den reads the terminal's routing instruction and says why its wording is wrong for a court: she "recognizes its wording from the Basin Compact's inherited service agreements" (Campaign Bible §4). Neris reports the same exception in medical transfer records (same section). A request asks whether the column halts 40 minutes (**PROPOSED**) to take a copy. The alternatives: Hold the column at the terminal for the copy; Hold only Mara and two escorts there while the column continues; `Keep fallback` (the column continues, and the testimony is already in the drawer). **Minimum valid action:** any alternative, or reading the testimony. |
| **What is taught** | The world is stranger than the courts admit (Execution Plan §7: "The first anomaly, delivered as testimony"). The game never confirms it for the player: the card says `testimony`, not `confirmed`. |
| **Who speaks** | Mara Den, testimony. **PROPOSED** line: "The slip is stamped for a route office that closed before my grandmother was born. It still issues them." |
| **Outcomes** | (a) The column halts and copies: evidence in hand, time lost, and the extraction team closes the estimate area. (b) Mara and two escorts stay behind: copies are made, and a three-person element is exposed. (c) The column moves on: the evidence survives in other channels, Neris's transfer records and Mara's memory (Campaign Bible §12: "crucial historical claims need more than one evidentiary path"). |
| **Feeling** | Hope, small and unconfirmed: the courts' authority is "neither universal nor technically necessary" (Campaign Bible §4). It does not fire the hope instrument (Experience Bible §6), which counts people; the beat is a premise, not a count. |
| **Built in** | Phase 3 (Report drawer); testimony and dossiers are atlas slice E, Phase 7 (Field Atlas §23.1, Roadmap §Phase 7); content in Phase 8. If slice E is late, the terminal beat still works with the drawer alone. |

### Beat 6. The first night march

| Field | Specification |
|---|---|
| **Trigger** | The player commits a Move for the column from the terminal region toward the Reedbank junction. Nothing forces it; the extraction team's estimate area (beat 3) is the pressure. |
| **Where** | `roadside_terminal` to `reedbank`; routes `dry_meridian` (road km 37.5 to 65.1) and `reedbank_spur` (10.9 km, `broken`). |
| **Newly visible UI** | The Order composer's optional fields, opened by the first constraint the player needs: emphasis (Default, Speed, Caution, Concealment, Conservation) and protected resources (Field Atlas §10.1); the `Execute to review` review condition (Field Atlas §23.1, §15); Service sheet, opened from the resupply request (Field Atlas §8, §13.1). |
| **Verbs available** | Move and Resupply (first appearance of each); Hold, Observe, Retreat, Evacuate (in play). |
| **What the player does** | The player commits a Move with a single review condition (arrival at the junction) and executes. One simulated window now spans 31 to 51 hours (R-4). Quiet time passes quickly (Simulation §3.1: "an uncontested march may advance by days"). Overnight, Petra Oss's request arrives (a protected reserve is required, Simulation §3.3): the escort squads held the line from dawn and are dry; the only water is the clinical reserve. The alternatives: **Resupply** escorts from general stock at the next halt; release the reserve; slow the pace (Move with emphasis Conservation). **Minimum valid action:** the Move with one review condition, then `Keep fallback` on the request. |
| **What is taught** | Long-duration execution, and that the simulation will stop for exactly the problems you set as boundaries (Execution Plan §7: "a single review condition"). |
| **Who speaks** | Petra Oss, decision request. **PROPOSED** line: "Escort squads: one flask between four. Wagon two has 900 liters. The clinical water is Neris's and marked protected." |
| **Outcomes** | (a) Resupply from general stock: a physical transfer at the halt (Simulation §26.1: "Physical replenishment to available authorized target"), partial if stock is short. (b) Release the reserve: Neris's patients bear the shortage later. (c) Slow the pace: longer exposure to the estimate area, and fewer miles before the Heliarch's pass. (d) Keep fallback: the escorts drink what they carry and the card records acceptance of the stated fallback (Field Atlas §19). Water is not a finite stock kind: ADR-0007 says "Food and water remain renewable", so no outcome here is a permanent loss of a stock kind. |
| **Feeling** | Despair, as the weight of earlier loss: the shortage is the beat 4 cargo, felt (Experience Bible §5). |
| **Built in** | Phase 3 (Move, composer, review condition); Resupply and the Service sheet in Phase 5 (Execution Plan §5 Phase 5: "Supply jobs, transport, multi-day marches, rest"). |

### Beat 7. The Heliarch on the horizon

| Field | Specification |
|---|---|
| **Trigger** | The first received `Observed` claim for the Heliarch in the generation. "Once per archon per generation" (Experience Bible §4d). Physical cause: the announced tour enters the window at 08:00 on day C+1 (R-3). |
| **Where** | The column on `dry_meridian`. At pace 18, 24 and 30 km a day the script puts the whale's nearest approach to the column at road km 47.2, 52.6 and 58.2, between the terminal (`roadside_terminal`) and the Reedbank junction, at 30.0°, 29.9° and 25.2° (slant 4.5 to 5.4 km). |
| **Newly visible UI** | The first-sighting card with its `FIRST SIGHTING` badge and a horizon plate (Experience Bible §4c, §4d); `Show on atlas`; "Create observation intent" (Experience Bible §4d, third exit). |
| **Verbs available** | The six core verbs already in play; no new verb. |
| **What the player does** | The card arrives as a Critical card and causes one review pause. The player opens it (it never opens itself, Experience Bible §4d). **Minimum valid action:** none. The card needs no answer; execution may resume with `Execute without new orders` (Field Atlas §15). The card's contents are measurements: observer, observation and receipt times, bearing, estimated distance, and angle ("≈ 7° of sky at est. 20 km, reported"). The camera never moves unless the player presses `Show on atlas` (Experience Bible R4). |
| **What is taught** | Plates are reports with ages, and the colossal is a fact the atlas records (Execution Plan §7: "The first-sighting card"). |
| **Who speaks** | Petra Oss, as the card's observer (the lead carrier), report. The card text is system text under Experience Bible §4d, not a speech: no adjective, no awe score. |
| **Outcomes** | (a) The player opens the card, looks at the plate, and resumes. (b) The player presses `Show on atlas`: the camera moves to the observation point and the dated footprint appears when the Phase 6 break effects exist. (c) The player never opens it: it stays queued ("Not a decision … never repeats", Experience Bible §4d), and the sky is there regardless as the whale grows from about 1° at entry to its peak. |
| **Feeling** | Awe (Experience Bible §4a, rung 1 at first report, rung 4 at 25° and above; §4d). The player has marched under it for hours before the card is read, so the number on the card is smaller than what they remember. |
| **Built in** | Phase 6 (first-sighting protocol, plate P1, Heliarch recipe); the tour's retiming is Phase 8 content. |

### Beat 8. Reedbank's lamps

| Field | Specification |
|---|---|
| **Trigger** | The column's lead element reaches the end of `reedbank_spur` at the town, and Asha Ren, Reedbank's municipal negotiator, has received a physical message from the column. Premise: "Reedbank's relief contracts are suspended because it provided water to the fugitives" (Campaign Bible §4). |
| **Where** | `reedbank`; route `reedbank_spur`. |
| **Newly visible UI** | An arrival card in the Report drawer (Experience Bible §6.2); the Campaign record's Kept tally header (Experience Bible §6.1); the promises ledger row (Experience Bible §6.3); the Service sheet for relief allocation. |
| **Verbs available** | Move, Resupply, Evacuate, Support, Hold, Observe, Retreat. |
| **What the player does** | Asha's request asks whether the defenders will reopen the relief road, which turns "self-defense into insurgency" (Campaign Bible §4). The alternatives are graded: a one-time water delivery; reopening the road (Resupply and Support under a standing service); leaving. **Minimum valid action:** any alternative; the card is the *inciting commitment*, framed openly (Campaign Bible §4: "Frame this openly as the campaign's premise"). |
| **What is taught** | A commitment is a promise with a ledger entry and a count of people, never a score (Experience Bible §6.1, §6.3). |
| **Who speaks** | Asha Ren, request. **PROPOSED** line: "The north cistern is dry. The east quay has a pump and eleven lamps. We cannot pay for what you bring, and we will remember who brought it." |
| **Outcomes** | (a) Reopen the road: the route's condition moves from `broken` to `worn` or `intact` over time (Region §3), the promise is recorded, and the arrival card gives the Kept delta (K2, Experience Bible §6.1) with a not-recovered line. (b) A single delivery: a smaller K2 count, no standing promise. (c) Leave: the column remains itinerant. Campaign Bible §4 asks for several plans and negotiated degrees of support rather than an equal alternative the game cannot deliver, so leaving is allowed but costs Reedbank's support, and no Kept people are added. |
| **Feeling** | Hope, material (Experience Bible §6.1, §6.2). The card names recipients and lamps lit, plus what was not recovered (§6.5). |
| **Built in** | Phase 5 (arrivals, Kept tally, settlement records); the promises ledger in Phase 8 (`ach_narrative`). |

---

## 4. Disclosure schedule

Nothing appears before the beat that needs it. "First shown" is the beat. Elements are Field Atlas §8 interfaces unless marked.

| UI element or verb | First shown in | Field Atlas section | Held back until |
|---|---|---|---|
| Atlas, counters, Fixed site mark | Beat 1 | §4, §5.1, §8 | Always on |
| Unit inspector | Beat 1 | §8 | Selecting a counter |
| Order composer, minimum fields (HQ, units, verb, target) | Beat 1 | §10.1 | Selecting a unit and a verb |
| `Stage order`, Dispatch review, `Commit and execute` | Beat 1 | §7.2, §10.2 | A staged draft exists |
| `Execute to review` | Beat 1 | §7.2, §15 | Always in the bottom strip |
| Verb Hold | Beat 1 | §10.1 | Ione's proposal |
| Decision queue; decision card; `Keep fallback` | Beat 2 | §12.1, §19 | A decision reaches HQ |
| Hostile-element counter (diamond) | Beat 2 | §5.1 | A hostile element is reported |
| Verb Evacuate | Beat 2 | §10.1 | The card's alternatives |
| Contact/agreement sheet | Beat 2 (alternative c) | §8 | The player picks "Send terms" |
| Report drawer, two timestamps | Beat 3 | §12.2 | A report is received |
| Last-reported-position anchor pin with age | Beat 3 | §5.1 | A report ages |
| Verb Observe | Beat 3 | §10.1 | Eren's report recommends it |
| Intelligence dossier | Beat 3 (optional) | §8 | A contact or evidence link is opened |
| Verb Retreat | Beat 4 | §10.1 | The Hold is untenable |
| Roll card; `Acknowledge` | Beat 4 | Experience Bible §5.1 (§8 Decision queue) | A confirmed death is received |
| Campaign record | Beat 4 | §8 | The Roll or a left-behind line is acknowledged |
| Personnel dossier | Beat 4 (optional) | §8 | The player follows a Roll entry |
| Testimony class in the Report drawer | Beat 5 | §12.2 | A testimony report is received |
| Asset/site sheet | Beat 5 (optional) | §8 | The player selects the terminal |
| Verb Move (player-authored) | Beat 6 | §10.1 | The march is composed |
| Composer optional fields: emphasis, protected resources | Beat 6 | §10.1 | The first constraint the player needs |
| Review condition (scheduled review) | Beat 6 | §15, §23.1 | The march is composed |
| Verb Resupply; Service sheet | Beat 6 | §10.1, §13.1 | Petra's request |
| First-sighting card; plate; `Show on atlas` | Beat 7 | Experience Bible §4d (§8 Decision queue) | The first Observed claim is received |
| Arrival card; Kept tally header; promises ledger | Beat 8 | Experience Bible §6 (§8 Report drawer, Campaign record) | The arrival is confirmed |
| Verb Support | Beat 8 | §10.1 | The relief alternatives |
| Doctrine sheet, Operation sheet | Not in the opening | §8 | Never prompted; the player may open them from menus |
| Presentation pause (Space) | Never prompted | §9, §15 | The player's own use; it grants no authority |

The six core verbs, with each verb's first occasion: Hold (beat 1), Evacuate (beat 2), Observe (beat 3), Retreat (beat 4), Move (beat 6), Resupply (beat 6). All six occasions precede Reedbank (beat 8).

Rule for composer fields: "Expand optional purpose, emphasis, constraints, timing, support, protected resources, completion/follow-on, and fallback only when relevant" (Field Atlas §10.1). In this document the earliest such need is beat 6's protected reserve. Beats 1 to 5 never open an advanced field.

### 4.1 UI that must exist per phase

| Phase | Needed by the opening |
|---|---|
| 2 | Atlas, counters including last-reported anchor pin, report-age text (beats 1, 3) |
| 3 | Unit inspector, Order composer minimum fields, Dispatch review, session strip, Decision queue, Report drawer (beats 1 to 3) |
| 4 | Roll, Campaign record, Personnel dossier, Evacuate chain (beats 2, 4) |
| 5 | Service sheet, Resupply, arrival card, Kept tally header (beats 6, 8) |
| 6 | First-sighting card and plate (beat 7) |
| 7 | Dossiers and testimony (beat 5), Contact/agreement sheet (beat 2c) |

---

## 5. Invariants

| ID | Invariant | How it is met | Test (Phase 8) |
|---|---|---|---|
| I-1 | By Reedbank, the player has used Move, Hold, Observe, Retreat, Resupply and Evacuate at least once, without opening help. | Each verb has a designated decision point (§4). At each, the card's recommendation is that verb, while a different valid command and `Keep fallback` remain possible (Simulation §26.4: alternatives "are proposals"). | `recommended_path_uses_all_core_verbs`: accepting every recommendation reaches Reedbank having issued all six. Telemetry in playtests counts actual use. |
| I-2 | No beat pauses the simulation for instruction. | Teaching is text on proposals and cards. The only pauses are the existing ones: a decision reaches HQ, a scheduled review, and the first-sighting card, which "causes one review pause" by Experience Bible §4d and is not an instructional pause. | `no_modal_before_reedbank`: no modal dialog and no clock stop except Field Atlas §15 state transitions. |
| I-3 | No beat forces an outcome by removing options. | Each beat lists at least two outcomes (§3). Options are limited only by physics: the carrier cannot move, the water is low, the whale is where it is. | Scripted bot plays the "worst" and "refuse everything" paths; each reaches beat 8 or a graded failure that continues (Simulation §25.3). |
| I-4 | The camera never moves without consent. | `Show on atlas` is the only recenter on beats 4, 7 and 8 (Experience Bible R4). | `camera_consent` (Experience Bible R4). |
| I-5 | Beats fire on received knowledge. | Triggers are receipt times (Experience Bible R5; Campaign Bible §12). The two non-informational triggers (beats 1 and 6) are the scenario clock and the player's own commit. | `no_beat_before_receipt` (Experience Bible R5). |
| I-6 | The despair beats never gate controls. | Roll "does not gate any control" (Experience Bible R10, §5.1.4). | `roll_does_not_gate_controls`. |

---

## 6. Playtest observation sheet

Protocol: observe silently, then ask the three questions in Execution Plan §9: which moment did you feel something, who died, what were you trying to do when you got confused.

| Beat | Watch for | Failure sign (redesign) |
|---|---|---|
| 1 | How long from first selection to `Commit and execute`, and whether advanced fields were opened. | The tester opens advanced fields, or takes over two minutes, or asks what a verb is. Cut the verb list from the composer until chosen. |
| 2 | Whether the tester reads the card before choosing. | The tester dismisses the card unread, or says "what does it want me to do". The card's first line fails Field Atlas §22's formula; rewrite it. |
| 3 | Whether the tester looks at both timestamps. | The tester treats the report as current ("they are at the gate") after reading it. The age display is too quiet; move the age to the counter. |
| 4 | Whether the tester can say what was left behind and why, in one sentence. | The tester says "the game took my cargo". The physical constraint is unreadable; show the capacity gap on the card. |
| 5 | Whether the tester reads the testimony or chooses on the time cost alone. | The tester asks what a terminal is for. The anomaly is not distinct from routine; sharpen the testimony text, not the tutorial. |
| 6 | Whether the tester is surprised that the window ran more than a day. | The tester pauses to ask "is it stuck" or presses Space repeatedly. The review condition is invisible; show the next review in the bottom strip. |
| 7 | What the tester does in the first ten seconds after the card opens. | Silence is not failure. Failure is "what is that for?": the card is read as a task. Remove any actionable-looking button above the plate. |
| 8 | Whether the tester can name one person at Reedbank afterward. | None named. The manifest lists quantities only; name recipients earlier (Experience Bible §6.2 up to three). |
| 1 to 8 | The first moment the tester says they do not know what to do. | Any such moment before beat 7 is a defect in the previous beat. Record the beat and the screen. |

Telemetry (extends Experience Bible §10): time from card shown to action per beat; whether `Keep fallback` was chosen; verbs used by the time Reedbank is reached; whether `Show on atlas` was pressed on beat 7.

---

## 7. Decisions and open questions

PROPOSED here: R-1 to R-5 (§1.1); the starting clock and positions (§3 setting); the 15% capacity gap; the 300 m and 200 m trigger radii; the 40-minute halt; the emission restriction; every example line; the added beat 8.

Open questions for Liam:

1. **Tour retiming (R-3).** The Heliarch's entry moves to day C+1. That edits a PROPOSED line in Region §5 and in `content/world/red_ledger_routes.ron`; neither is in this task. Do you accept it, or prefer the whale during the standoff (the opposite order from Execution Plan §7)?
2. **Several days in one commit (R-4).** The geometry gives 31 to 51 hours. Do you want a rest day inside the window (Simulation §11 rest) to make it a true multi-day commit, or is a day and a half enough to teach that quiet time passes fast?
3. **The first Roll (beat 4).** No-forced-outcome means the Roll is not guaranteed at the withdrawal. Is the fallback acceptable (a patient in Neris's care who may die if the clinical freight is left, and a "Left behind" line otherwise)? Or should the withdrawal be tuned so that a confirmed death is near-certain on every path?
4. **Beat 2's third alternative.** The Contact/agreement sheet at the first decision may be a second idea. If the sheet is not built by then, drop the alternative.
5. **The tour announcement.** Region §5 says the tour is proclaimed before the column leaves the camp. Where does the player see that notice? Phase 7's Proclamation channel is the natural home; until then the opening has no announcement.
