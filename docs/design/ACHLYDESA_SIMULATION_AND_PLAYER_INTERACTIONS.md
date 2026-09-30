# Achlydesa — Simulation and Player Interactions

**Status:** Complete first-pass systems design, 25 September 2026. Rules are specified for prototyping; numerical balance and performance remain unvalidated.

**Companion documents:** `ACHLYDESA_STRATEGY_RPG_HIGH_LEVEL_DESIGN.md` (revision of 25 September 2026), `ACHLYDESA_AUTONOMOUS_SQUAD_TECHNICAL_DESIGN.md`, and `achlydesa-world-bible.md`.

**Purpose:** Define the simulation, the player's commands, subordinate authority, information available to the player, and the consequences linking immediate combat to the persistent campaign. This document expands the existing design rather than changing its genre or cosmology. It is a functional specification, not an engine implementation or an assertion that the proposed AI is already proven.

## Contents

1. Command experience and scope
2. Persistent entities and invariants
3. Time, simultaneous execution, and decision pauses
4. Orders, authority, and coordination
5. Player interface and interaction grammar
6. Terrain, height, structures, and navigation
7. Weather, hazards, and environmental persistence
8. Observation, intelligence, and knowledge
9. Communications and electronic warfare
10. Autonomous leaders and individual behavior
11. Marches, traffic, rest, and operational preparation
12. Combat, suppression, morale, and withdrawal
13. Fire support and air defense
14. Vehicles, drones, and autonomous weapons
15. Wounds, medical care, prisoners, and the missing
16. Inventory, physical supply, and distribution
17. Repair, salvage, recovery, and technological decline
18. Engineering, facilities, and persistent bases
19. Characters, relationships, recruitment, and training
20. Doctrine and institutional memory
21. Settlements, civilians, trade, and political control
22. Diplomacy, agreements, and faction relationships
23. Enemy command and irregular warfare
24. Archons, anomalies, and the Soterion junction
25. Story, deadlines, failure, and generational change
26. Interaction catalog and command outcomes
27. Complete play sequences
28. Simulation architecture and performance
29. Prototype values and balancing levers
30. Verification and implementation sequence
31. Research basis and design boundaries

## 1. Command experience and scope

The player commands a persistent army through named field commanders. They choose objectives, positions, targets, timing, support, routes, reserves, and logistical priorities. Squads and crews decide how to execute those orders. The playable experience is assessing an imperfect situation and committing people to a course of action.

The world is a continuous 2.5D height-map theater roughly 1,000–1,500 km across, using one spatial grid and one clock. Individual soldiers persist within squads of approximately five to eight people. A typical concentrated operation involves 15–25 friendly squads and support elements, with no arbitrary total force cap. Multiple task forces may march, fight, repair, or negotiate simultaneously.

### 1.1 Responsibility boundary

| Player owns | Subordinates own | Background simulation owns |
|---|---|---|
| Objective and purpose | Local action plan and battle drill | Weather and physical consequences |
| Assigned position, area, or target | Exact cover, spacing, facing, and movement | Other factions' actions |
| Operational route constraints | Local detours and formation adjustments | Civilian subsistence and migration |
| Support priority and reserve commitment | Local synchronization and requests | Ordinary settlement administration |
| Risk and engagement constraints | Immediate survival and casualty response | Routine recordkeeping |
| Supply sources, corridors, allocations | Loads, convoy timing, transfers, maintenance | Character history and relationships |
| Appointments and force composition | Succession when disconnected | Existing infrastructure deterioration |
| Alliances and major obligations | Implementation within delegated authority | Political reactions to observable events |

There are no individual action points, per-soldier target assignments, inventory puzzles, medical procedures, radio configuration, civilian job allocation, research queues, or factory chains. Inspectable detail supports diagnosis and attachment; it must not become mandatory maintenance.

Task forces are selection and coordination groups. They never become a separate autonomous army layer that hides squads from the player. A shared operation plan distributes bounded orders to their members, all of whom remain selectable.

### 1.2 What counts as a worthwhile decision

Interrupt when the force needs authority to change a goal, commit a protected resource, accept materially different risk, or enter a political obligation. Do not interrupt for a local route adjustment, ordinary reload, successful routine delivery, or replacement of a failed action by another already authorized action.

The default player loop is **read reports → select an objective → assign forces and support → inspect the plan → commit → review consequential change**. A player who issues sound orders should often be able to watch an operation work.

## 2. Persistent entities and invariants

### 2.1 Entity model

| Entity | Essential persistent state | Principal relationships |
|---|---|---|
| Person | Identity, age, skills, traits, health, fatigue, affiliations, equipment, memories | Squad, family, mentor, captor, patient, employer |
| Squad or crew | Members, leader, intent, doctrine, cohesion, capabilities, local beliefs | Task force, support units, command authority |
| Vehicle or device | Position, owner, crew, components, power, cargo, operating state | Repair requirement, transport assignment, controlling link |
| Item or supply lot | Type, compatibility, quantity, condition, mass, volume, location | Holder, reservation, transfer, provenance |
| Site or structure | Geometry, damage, access, capacity, power, staff, stocks | Settlement, routes, owners, service clients |
| Route segment | Geometry, surface, width, load limits, hazards, current accessibility | Traffic, crossings, work projects, supply corridors |
| Contact or report | Observer, evidence, time, classification, location estimate, confidence | Local belief, message history, corroborating sources |
| Message or order | Issuer, recipients, content, version, creation and receipt times | Delivery path, supersession, acknowledgment evidence |
| Faction | Leaders, objectives, forces, resources, relationships, known world | Claims, settlements, allies, conflicts, successors |
| Settlement | Population groups, needs, stocks, capacity, security, leadership, support | Trade routes, migration groups, garrisons, patrons |
| Agreement | Parties, obligations, duration, conditions, witnesses, fulfillment | Access, supply, ceasefire, prisoners, services |
| Archon or anomaly | Body or footprint, observable rules, services, connections, dependencies | Jurisdiction, worshippers, triggers, physical consequences |
| Campaign event | Preconditions, deadlines, participants, possible outcomes, resolved facts | Objectives, successor roles, world changes |

All entities have stable identifiers. Ownership, location, custody, knowledge, and authority are separate: owning a cache does not make its contents available, and receiving an old report does not reveal its current state.

### 2.2 Non-negotiable invariants

1. People, supplies, vehicles, prisoners, and portable equipment move physically or through a specifically defined anomalous transit mechanism.
2. A resource cannot be assigned twice. Reservations prevent competing plans from silently promising the same truck, ammunition, engineer, or power capacity.
3. Orders require a delivery path. Looking at a unit does not create one.
4. Actors select actions from their knowledge. Ground truth governs collision and damage, not strategic omniscience.
5. Camera position, zoom, playback speed, and hidden enemy proximity cannot alter outcomes or reveal secrets through interface timing.
6. Deployed characters can die. Named status does not grant combat immunity.
7. Repair uses surviving means. There is no technological invention or advanced manufacturing loop.
8. Forces and settlements can decline independently of the player. Campaign pressure comes from actual competing activity, not spawning a counterforce when the player becomes successful.
9. An anomaly changes only its authored rules. It does not excuse unrelated inconsistencies.
10. Consequences have provenance: what happened, who observed it, who was affected, and what became known.

## 3. Time, simultaneous execution, and decision pauses

### 3.1 Event-driven WEGO

Planning pauses the entire theater. Committing begins simultaneous execution everywhere. The next planning pause occurs when a qualifying decision reaches a player-controlled command authority, an agreed review condition occurs, or active plans reach a natural review point.

There is no universal combat turn duration. Seconds may matter during a local exchange; an uncontested march may advance by days. All processes share absolute simulation timestamps.

During execution the player may pan, inspect received information, and alter playback speed. A presentation pause can stop viewing for accessibility, but does not permit new orders or reveal future events. New orders are committed only at a command decision or scheduled review. Planning time itself is free.

### 3.2 Scheduler contract

1. Freeze the committed order versions and schedule their physical delivery.
2. Determine the next relevant simulation event: message arrival, movement boundary, consumption threshold, hazard change, project milestone, observation opportunity, or interaction.
3. Advance quiet processes only as far as that event, using accumulated time and actual resources.
4. Resolve interacting processes in stable chronological order. Simultaneous effects within a tactical step use a common pre-step state where needed to avoid first-processed units receiving an artificial advantage.
5. Update observations and local plans; create reports and decision requests.
6. Deliver messages through the actual command network.
7. Pause only if a request now at a command authority needs player judgment, or a scheduled review is due. Otherwise continue.

The engine may shorten an internal computation step because an unseen encounter becomes possible. That must not alter player-visible playback pacing, produce a special alert, or end the execution window.

### 3.3 What stops execution

| Event | Default treatment |
|---|---|
| Scout sees an enemy while its Observe order remains feasible | Report; continue |
| Squad adapts formation, reloads, uses local smoke, or treats a casualty within its orders | Continue; summarize if material |
| Required crossing is blocked and every known alternative violates the deadline or risk limit | Decision request on receipt |
| A protected reserve or last critical stock is required | Decision request on receipt |
| A known moral or diplomatic boundary prevents execution | Decision request on receipt |
| Leader dies and qualified succession works | Report; continue |
| Succession fails and the formation needs higher authority | Decision request if communication is possible |
| A planned waypoint, time, or player-selected review threshold is reached | Planning pause |
| Hidden enemy moves, disconnected unit fights, or an unobserved site changes hands | No privileged pause |

Requests from one cause are grouped: a destroyed bridge generates one route decision with affected convoys and forces attached. New critical requests cannot be muted away by generic notification settings. Repeated identical requests are suppressed until facts or authority change.

### 3.4 Multiple command posts

Each task force has an appointed field commander and its own received picture. The player can inspect any of those command pictures at a legitimate planning pause. Information does not merge between disconnected headquarters. A request reaching either active player-controlled commander can pause the shared clock; a request trapped with a disconnected squad cannot.

This is an institutional viewpoint, not omniscience: the player may personally remember information from another headquarters, but selecting a foreign report cannot attach its contents to a remote unit without transmission. A commander may still order movement toward a known geographic location; the receiving leader plans from its own knowledge. Sharing target intelligence requires a message or standing contingency. Command cards explicitly show which headquarters is issuing an order.

## 4. Orders, authority, and coordination

### 4.1 Minimal order grammar

The normal order is **selected unit + verb + target/place**. Doctrine fills the defaults. Optional fields are purpose, emphasis, completion condition, timing, support relationship, protected resources, and fallback. The interface exposes only fields relevant to the chosen task.

Emphasis is one of **Speed, Caution, Concealment, Conservation**, or the default. It changes evaluation within hard constraints; Speed never grants permission to cross a forbidden boundary or spend a protected reserve.

An order stores its assumptions and known information at issue. The leader reassesses them as conditions change. The interface shows the leader's proposed corridor or position, intended effect, resource needs, estimate, main unknowns, and explanation. Forecasts are conditional estimates, not guarantees.

### 4.2 Order lifecycle

**Draft → committed → in transit → received → preparing → executing → completed.** Executing may branch to **adapting**, **suspended**, **failed**, or **superseded**. Failed means the objective cannot currently be fulfilled; it does not freeze the people, who follow a fallback or local survival doctrine.

Each order has a sequence number. An older delayed instruction cannot overwrite a newer received instruction. Canceling a committed order sends a cancellation; the old order remains effective until the cancellation arrives. UI labels distinguish **requested**, **reported received**, and **execution observed**. No acknowledgment is invented from internal delivery state. Completion and failure are also reported beliefs in the command view: an unobserved destroyed target does not instantly mark an objective complete for headquarters.

Standing Hold, Observe, Escort, Patrol, and Support tasks persist until their condition ends or another order replaces them. Completed finite tasks enter their specified follow-on; otherwise they secure themselves, sustain locally, and report completion. A unit never becomes defenseless because its movement arrow ended.

### 4.3 Conflict resolution

Hard constraints take precedence over preference scores. If no plan satisfies the constraints, the leader explains the conflict. For competing resource claims use explicit player priority, then already committed obligations, then issued order time. Ties appear as a resource conflict before commitment when known.

Support relationships cannot create command loops: a unit supports a primary recipient or a prioritized set, but does not recursively wait on a unit whose action depends on it. The plan checker highlights such cycles. It checks known conditions only and cannot detect a hidden enemy for the player.

### 4.4 Coordinated action without a scripting language

Allow a small set of timing relationships: **start now**, **start at time**, **after reported condition**, **support named element**, and **remain reserve until authorized condition**. A force can be ordered to start after another reports readiness. If that report never arrives, the waiting unit uses an agreed timeout or requests a decision.

Group orders copy intent with role-aware targets. Holding a frontage distributes sectors among squads; observing an area distributes coverage; marching to a destination preserves a template. The player can edit squad assignments before commitment, without positioning individual soldiers. Linked tasks share an operation label and priority, not an additional general AI.

Reserve pre-authorization is narrow: a named reserve may reinforce a named sector on a reported trigger within a stated limit. It may not begin a different offensive or chase an enemy across the theater.

## 5. Player interface and interaction grammar

### 5.1 Main map

The main screen is always the theater. Zoom reveals people, squads, formations, settlements, and route relationships at suitable scales. Four snapped camera orientations, cutaway roofs, and a floor selector keep the 2.5D geometry readable.

The interface consists of a time/commit bar, command-post selector, compact roster, contextual order card, and report drawer. Optional overlays show **terrain/observation**, **contacts**, **command connections**, **supply/endurance**, **routes/traffic**, **political claims**, or **anomalous jurisdictions**. Display at most one primary heatmap; retain essential unit and contact markers on top.

Every symbol uses shape or text as well as color. Orders and uncertainty remain readable over unusual scenery. The world may be visually hostile; the command interface stays clean and consistent.

### 5.2 Typical inputs

| Action | Interaction |
|---|---|
| Give a local order | Select squad, choose verb, click object or draw a small area, inspect, commit |
| Assign a frontage | Select squads, choose Hold, draw frontage, review sector allocation |
| Plan a march | Select force, choose destination and posture, accept route or add control points |
| Allocate support | Select support unit, select recipient or area, set priority if needed |
| Establish supply | Select force, choose source and support corridor, review endurance |
| Set a reserve | Select units, designate reserve area, choose commitment authority |
| Inspect failure | Open unit rationale, see changed assumption, choose a new intent or constraint |
| Negotiate | Select reachable contact, choose desired agreement, review terms and delivery delay |
| Restore capability | Select damaged asset/site, choose desired function and priority, review tradeoffs |

All actions also have explicit buttons and keyboard navigation. No essential action relies on a hidden gesture, color, or timed click.

### 5.3 Information depth

At a glance show intent, connection, capability warnings, cohesion, suppression, and casualty urgency. Inspection shows the leader's reasoning, relevant people, support, stocks, and known evidence. Deeper inspection exposes contributing calculations and history. Opening detailed panels is never required to reload, eat, rest, redistribute ordinary ammunition, or maintain a vehicle.

Draft orders can be undone freely. Committed orders require replacement through communications. The player-facing log contains received reports and later testimony, not an omniscient replay. It can filter by operation, person, location, or decision.

## 6. Terrain, height, structures, and navigation

**State:** Elevation, slope, material, surface, cover, concealment, bearing capacity, passages, authored structural states, and persistent alterations. A provisional two-meter square grid provides common coordinates; continuous positions and finer authored object geometry handle doors, walls, and firing apertures.

**Rules:** Foot movement, wheeled/tracked vehicles, carrying casualties, and towing have different passability and cost functions. Height changes line of sight, line of fire, exposed silhouette, radio obstruction, movement effort, and drainage. It does not provide an automatic damage multiplier. Cover intercepts physical effects; concealment affects observation. A solid wall can provide both, smoke usually only the latter.

Bridges, tunnels, roofs, and building floors are explicit layers connected by entrances, ramps, stairs, or damage. All remain in the same theater. A force on a roof cannot block a road beneath it merely by sharing horizontal coordinates.

Navigation operates at corridor, local route, and immediate movement levels. Leaders use known maps and observation to choose a route; physical movement discovers unexpected blockages. Traffic reservations coordinate friendly use of narrow segments. The player may designate mandatory crossing points, prohibited areas, or an operational centerline, but not edit individual step paths.

**Player:** Choose destinations, frontage, approach constraints, passage priorities, and routes worth securing. Inspect known travel costs and observation geometry. A line-of-sight preview uses known geometry and current reports; undiscovered interior cover remains uncertain.

**Automatic:** Formation adaptation, local cover choice, queueing, reasonable detours, door use, and avoidance of observed hazards.

**Escalation:** No permissible route, a crossing requiring scarce engineering, an important timing change, entry into forbidden territory, or congestion that jeopardizes the objective.

## 7. Weather, hazards, and environmental persistence

**State:** Regional weather fields, day/night light, temperature, wind, visibility, dust, precipitation where applicable, local fire/fuel, smoke, contamination, unstable structures, and known anomalous overlays. Weather is generated as spatially coherent evolving fields; it is not independently rerolled for every battle.

**Rules:** Heat and cold affect exertion, water demand, exposure, equipment reliability, and recovery. Wind and dust affect sensing, smoke, flight, and some weapon effects. Rain changes surfaces and catchments where it occurs. Shadows and darkness alter detection rather than invisibly applying universal combat penalties.

Fires require fuel and ignition; they spread through adjacent suitable material, consume it, produce smoke, and damage exposed objects. Ruined structures change through authored damage states. Craters, rubble, unstable bridges, abandoned mines, and contamination persist until weathering or work changes them. Large hydrological changes use authored drainage connections, not an unrestricted fluid simulation.

Civilian and wild life are population or hazard groups unless a particular animal matters. This is not a complete ecosystem simulator. Ecological state matters where it changes water, provisions, cover, disease exposure, travel, or an Archon's services.

**Player:** Choose timing, route, shelter, protective equipment preset, hazard boundaries, and whether to commit despite a forecast. Engineers or scouts can assess a hazard; assessment takes time and can be wrong within the limits of their evidence.

**Automatic:** Routine shelter, hydration, protective posture, local fire avoidance, and schedule adjustments within the order's tolerance.

**Escalation:** Forecast endurance crosses a mission limit, an unknown hazard blocks a route, fire threatens a designated asset, or protection requires abandoning the objective. Forecasts show source and uncertainty; nobody knows tomorrow's Heliarch movement simply because the engine does.

## 8. Observation, intelligence, and knowledge

### 8.1 Four layers

Ground truth is the actual world. Observation is evidence detected by an actor. Local belief is that actor's interpretation. The command picture contains information delivered to a particular headquarters. The UI displays the latter and connected local reporting, never the entire ground truth.

A contact records a real source: a person, vehicle, track, sound, emission, decoy, or anomalous effect. Classification may be wrong; source existence is not random phantom noise. Rumors are separately labeled testimony and may be lies. A false rumor does not become a detected combat icon until evidence supports it.

### 8.2 Detection and tracking

Detection opportunities depend on range, occlusion, signatures, sensor type, attention, weather, motion, familiarity, and observer condition. Repeated observation improves classification and location. Losing contact produces a timestamped possible-location region that expands along plausible routes; it does not track the hidden target's true movement.

Multiple reports retain provenance. Five copies of one report do not count as five independent witnesses. Contradictory reports remain visible until resolved; the staff can propose an assessment but cannot silently erase inconvenient evidence.

Map knowledge also has age. A purchased map may accurately locate a road while missing a collapsed bridge. Enemy positions, local water, access rights, and anomalous boundaries are separate knowledge layers.

### 8.3 Player and subordinate actions

**Player:** Designate an area to observe, a route to screen, a contact to trail, or a question to answer: “Can loaded vehicles cross here?” or “Is the junction occupied?” Select how much exposure is acceptable. Requests use a small library of intelligence questions linked to decisions, not free-text language interpretation.

**Automatic:** Scouts choose observation positions, maintain concealment, shift viewpoint, cross-check available sensors, and report changes that matter to their task. They avoid engaging when their observation intent does not authorize it.

**Escalation:** The question cannot be answered within the allowed exposure, the observer is compromised, a discovery invalidates a command assumption, or the player preselected the finding as a review trigger.

**Presentation:** Exact weapon probabilities may be shown conditional on known or explicitly assumed inputs. Unseen armor, uncertain geometry, and stale positions remain named unknowns. A precise calculation for a believed target is not a guarantee about the real target.

## 9. Communications and electronic warfare

**State:** Physical radios, wire, relays, runners, power, equipment condition, terrain obstruction, interference, emissions, and queued messages. Player-facing connection states are **Connected, Delayed, Cut**.

Messages have creation, send, arrival, and report times. Orders and critical reports precede routine detail. Limited communication may truncate low-priority fragments while preserving delivered content; it does not turn instructions into arbitrary nonsense. Local authentication is handled by characters and equipment, not a player minigame.

Commands cannot route through an allied network without access rights. Interception gives the enemy only what its capability can actually extract: perhaps an emission location, perhaps traffic character, perhaps readable content. Detecting a radio never automatically reveals every order and roster entry.

| EW intent | Autonomous execution | Constraint visible to player |
|---|---|---|
| Listen | Detect and classify actual emissions, compare bearings | Coverage, delay, ambiguity, operator availability |
| Jam | Apply available interference toward assigned area or signal family | Power, signature, friendly interference, limited coverage |
| Deceive | Operate physical decoys or transmit supported false signals | Compatible capability, plausibility, chance of exposure |
| Protect | Reduce avoidable emissions and adapt supported links | Reduced reporting speed, equipment and route limitations |

**Player:** Place relays, assign EW areas, set emission posture, permit allied access, or order repair of a connection. Prepared wire is a persistent engineering project; no tactical cable-drawing interaction is required during combat.

**Automatic:** Link selection, ordinary retransmission, report prioritization, supported countermeasures, runners where authorized, and local action under lost communications.

**Escalation:** Loss of a command-critical link threatens the operation, restoration needs a protected resource, or a unit must exceed its current authority. A disconnected unit cannot notify the player of disconnection by magic; the headquarters observes a missing expected report or loss of its own known link.

## 10. Autonomous leaders and individual behavior

### 10.1 Planning model

The squad leader decomposes an intent into authored tasks with preconditions, resource needs, alternatives, and abort conditions. Reactive behaviors execute those tasks and handle small changes. A path obstruction should usually cause a local detour, not a full rewrite of the operation. A missing essential capability should trigger reassessment rather than improvising an impossible action.

Evaluate candidates against objective success, risk, time, expenditure, exposure, coordination demand, doctrine, and personal limits. Enforce hard constraints before preference ranking. Leaders use their local beliefs. They cannot choose the safest route by querying hidden enemy locations.

The four visible leadership qualities remain **Judgment, Coordination, Initiative, Composure**. These affect perception, plan comparison, timing, coordination, and recovery. Low quality produces plausible limitations—slower recognition or weaker coordination—not deliberately foolish pathfinding.

### 10.2 Autonomous behavior contract

| Situation | Required behavior |
|---|---|
| Minor obstacle | Adapt locally if purpose and constraints remain satisfied |
| Sudden fire | Execute a trained immediate response, then reassess the standing objective |
| Short-lived suppression | Seek protection, recover, and resume when feasible |
| Loss of expected support | Reevaluate feasibility; do not assume the support still exists |
| Leader incapacitated | Recognized qualified successor takes over; report when possible |
| Objective achieved | Execute follow-on or secure and sustain locally |
| Objective impossible | Authorized fallback plus a useful request for authority |
| New opportunity | Exploit only if it advances the current intent within accepted risk |

Individual agents handle movement, posture, aiming, reloads, aid, carrying, and immediate survival. Temporary fireteams are leader-managed and require qualified subleaders. The player commands the squad, not those temporary teams.

### 10.3 Trust and explanation

Each material decision leaves a reason record: relevant observations, considered method, controlling constraint, chosen alternative, and missing information. Only reports actually received become player-visible. “I stopped because the bridge will not carry our truck” is useful; “AI failed” is not.

An inexperienced leader may recommend a poor but plausible plan. The preview should make the reasoning inspectable. The remedy is changing intent, support, doctrine, training, or appointment—not unlocking individual micromanagement.

## 11. Marches, traffic, rest, and operational preparation

**State:** Position, march template, posture, route, load, transport seats, road capacity, fatigue, carried water/fuel, maintenance due, security requirements, and arrival estimate.

A march includes movement, halts, observation, minor repair, navigation, bottlenecks, and rest. A destination's distance is not divided by maximum vehicle speed to promise an arrival. Terrain and security can dominate travel time. Every detachment deploys from its real march position if contact occurs; there is no pre-battle rearrangement screen.

Choose **Rapid, Balanced, Guarded, Concealed** march posture. Rapid reduces discretionary halts and accepts exposure or fatigue within limits. Guarded reserves effort for observation and security. Concealed prioritizes lower-signature routes and timing. Templates automatically place reconnaissance, main body, support, and recovery assets according to available capabilities; the player can alter the order and spacing of squads and vehicles.

Fatigue accumulates through exertion, load, heat, inadequate sleep, and prolonged stress. Rest requires time and a tolerable site; food and water enable recovery. Watch and maintenance rotations occur automatically. If the mission's deadline conflicts with sustainable rest, show the tradeoff rather than silently stopping every hour.

Preparation tasks—reconnaissance, replenishment, rehearsal, rest, liaison, or restoring a crossing—run concurrently when they use different resources. The operation card shows a critical path and conflicts. Two tasks using the same engineer cannot both receive full work rate.

**Player:** Choose destination, route constraints, march posture, arrival requirement, protected loads, and review points. Decide whether to delay, split the force, shed cargo, use transport, or accept fatigue.

**Automatic:** Exact local paths, traffic yielding, ordinary rest, compatible refueling, movement reporting, and formation changes.

**Escalation:** Material delay, lost route, insufficient water/fuel, inability to carry casualties or vital cargo, a commitment requiring new support, or unexpected contact that makes the standing mission infeasible.

## 12. Combat, suppression, morale, and withdrawal

### 12.1 Resolution model

Every consequential shot is resolved against the physical theater. A shot selects an aim point from the shooter's belief, samples a seeded trajectory from weapon and shooter state, checks intervening geometry and materials, and applies terminal effects. Visual effects follow that result. Analytic trajectories may replace continuous projectile rigid bodies, but offscreen combat cannot become a separate casualty-table system.

Weapons expose range-dependent effectiveness, ammunition compatibility, firing behavior, heat or operating limits where meaningful, and reliability. Armor interacts with direction, location, material, and threat type. There are no arbitrary faction damage bonuses. Protection can prevent penetration without preventing suppression, visibility loss, mobility damage, or crew distress.

Damage changes body functions, equipment functions, structural state, and hazards. A vehicle may remain a useful firing position after losing mobility; an intact weapon without an operator cannot act. Destruction can remove usable parts, scatter cargo, ignite supplies, or create salvage with remaining condition.

### 12.2 Three distinct human states

| State | Meaning | Changes caused |
|---|---|---|
| Suppression | Immediate response to credible perceived danger | Exposure, observation, movement, firing, aid, complex actions |
| Morale | Confidence and willingness to continue the present struggle | Persistence, surrender, desertion pressure, acceptance of hardship |
| Cohesion | Ability to act as an organized group | Coordination, succession, rally, information sharing, withdrawal |

Suppression grows from nearby effects and perceived threat, not a magical radius around any firing weapon. It recovers when the threat falls and protection or leadership helps. Morale changes through outcomes, expectations, leadership, provisions, legitimacy, and treatment. Cohesion develops through familiarity and shared service and is damaged by disorganization, casualties, betrayal, and replacement shock.

Squad cohesion is summarized as **Unified, Strained, Fractured**. Causes are inspectable. Low cohesion does not directly weaken bullets or armor; it makes coordination and recovery harder.

### 12.3 Engagement and contact development

A force may detect, investigate, observe, suppress, fix, maneuver, assault, bypass, accept surrender, or withdraw. The commander specifies the desired effect. Leaders choose local methods supported by actual capabilities and permissions.

Engagement posture defaults to **defend against threats and engage assigned targets**. Alternatives are **avoid contact**, **observe only**, or **engage identified hostile forces within the assigned area**. Positive identification thresholds and civilian restrictions are standing doctrine. They are not manually re-entered for each burst.

Area fire is allowed against credible locations when weapon, purpose, expenditure permission, and collateral constraints permit it. A contact's probability region does not magically fill with damage. Effects land in real locations and may miss entirely.

### 12.4 Recovery after contact

When a local exchange ends, the leader checks surviving command, casualties, suppression, cohesion, ammunition, equipment, routes, support, and time. It resumes the original objective if feasible. Otherwise it consolidates, seeks support, uses a fallback, or requests a new decision.

Regrouping, casualty care, redistribution, perimeter security, and reporting compete for available people and time. They are automatically coordinated. The player intervenes if continuing now means abandoning a person, protected equipment, or another objective.

Withdrawal is physical. Leaders select a viable destination and organize disengagement with available means. Surrounding geometry, casualties, vehicles, and enemy pressure can make escape incomplete. Surrender becomes an option when further resistance or escape is judged untenable; willingness also depends on what people know about the opponent's treatment of prisoners.

**Player:** Assign effects, sectors, targets, support, risk, and fallback. Decide whether to commit reserves, continue an attack, exploit, accept surrender, or leave.

**Automatic:** Fire distribution, reloads, immediate reactions, positioning, local aid, rally, temporary splits, and objective reassessment.

**Escalation:** Objective infeasibility, unapproved pursuit, protected reserve expenditure, severe political/character conflict, or a change in acceptable losses. A casualty does not automatically pause the whole theater unless it creates one of those decisions or meets a chosen review condition.

## 13. Fire support and air defense

### 13.1 Fire-support contract

A fire-support order identifies **effect + target/area + supported unit or priority**. Effects are **Suppress, Neutralize, Destroy, Screen with smoke**, and **Illuminate**, restricted to supported ammunition and equipment. The card explains availability, target information quality, expected delay, finite expenditure, and known friendly/civilian exposure.

An observer or other valid source must locate the target. A functioning chain must deliver the request, authorize the expenditure, assign a compatible weapon, and assess the result. Staff and crews handle technical computation, sequencing, and ordinary adjustment. The player never enters ballistic tables or manually aims every mortar.

Support units occupy real positions and can be located, attacked, delayed, deprived of ammunition, or forced to move. Weapon limits, target geometry, stale information, and line of fire may invalidate a request. A requested effect can fail without creating a fake precision result.

### 13.2 Allocation and safety of coordination

The commander assigns recipients and priority. Existing support commitments reserve time and ammunition; requests compete for what remains. The support leader can stop when the intended effect is reported, the expenditure allowance is reached, the target is lost, or risk crosses an established constraint.

Friendly locations used for coordination are reported positions with age and uncertainty. Conflicting or stale locations cause delay, a request for better information, or an escalation. There is no omniscient automatic guarantee against friendly fire. Defaults favor withholding when required confidence is absent; relaxing them is a consequential command choice.

Counterfire depends on observed effects, sensors, inference, communications, and available weapons. Hearing a shot does not reveal a perfect firing coordinate. Crews may displace under standing doctrine when feasible; moving costs response time and can interrupt promised support.

### 13.3 Air defense

The player designates defended assets or areas and priorities. Units combine detection, concealment, dispersion, electronic effects, and compatible weapons. Operators select feasible engagements under threat classification and ammunition constraints. Air-defense coverage is conditional on terrain, sensors, readiness, links, and weapon reach, not an invulnerable circle.

Unknown aerial contacts remain uncertain. Scarce defensive ammunition is reserved for the assigned priority unless the player permits broader use. A saturation threat forces a choice about what to protect rather than automatic infinite interception.

**Escalation:** Conflicting priority requests, essential target data unavailable, dangerous overlap, insufficient authorized expenditure, or abandonment of a protected asset.

## 14. Vehicles, drones, and autonomous weapons

### 14.1 Vehicles and transport

Each vehicle has a crew, occupants, cargo, compatible fuel/power, mobility, weapons, sensors, communications, protection, and structural safety. It is summarized as **Operational, Degraded, Disabled, Destroyed**, with a few capability flags explaining why. Detailed damage supports repair decisions without requiring a bespoke cockpit maintenance interface.

Embarkation needs available seats or carrying capacity, physical access, and time. Cargo weight and volume compete with passengers and equipment. Squads may share transport according to a manifest managed automatically. Changing assignments while dispersed requires actual movement to a meeting point.

Crews choose whether to remain, fight degraded, attempt a temporary repair, or abandon based on local danger, available means, mission, and doctrine. A vehicle's occupants suffer effects according to the physical damage model; transport does not make them an abstract invulnerable cargo statistic.

**Player:** Assign vehicle intent, transport recipients, protected cargo, recovery priority, or support role. Authorize abandoning a scarce chassis when circumstances require it.

### 14.2 Drones and scarce aircraft

Drones have physical launch/recovery sites or carriers, operators or bounded onboard autonomy, payload, energy, condition, sensors, control range, and susceptibility to interference. Typical tasks are Observe, Relay, Trail, Screen, or an authorized attack using an existing suitable system.

Operators automatically manage flight path, orbit, battery reserve, landing, charging, and platform rotation. Drones lost beyond retrieval range remain lost. A crash creates a physical wreck where appropriate. Replacing a drone requires another surviving airframe; there is no replenishing drone-production queue.

Crewed aircraft are rare existing assets with crews, bases, maintenance, fuel, and recovery requirements. They move continuously above the same world coordinates. Their presence does not open an air-combat minigame or separate sortie arena. Deployment requires an operational plan and credible sustainment.

### 14.3 Autonomous weapons

Autonomous systems use a specific permitted target set, area, operating interval, identification standard, and loss-of-link behavior. Default loss-of-link actions are hold, return, or continue the already authorized bounded task according to the device's surviving design. They cannot acquire new strategic objectives.

Ordinary autonomous systems do not inherit Strategos's anomalous authority. Relic vulnerabilities are specific device traits, not a universal ability to hack any enemy. Capturing a controller or acquiring a protocol grants only its defined access.

**Escalation:** A mission exceeds remaining endurance, recovery becomes impossible, identification is inadequate, a protected payload is at risk, or new authority is required after communication loss.

## 15. Wounds, medical care, prisoners, and the missing

### 15.1 Wounds and treatment

Injuries track region, type, progression, functional impairment, pain, stabilization, and treatment. The player sees **Stable, Urgent, Critical**, or **Unassessed**, with the observer and assessment age. A person's actual condition can worsen before news reaches headquarters.

Care requires access, time, skill, supplies, and an appropriate environment. Agents handle self-aid, buddy aid, triage, and treatment within their capability. No manual procedure choices are exposed. Care may preserve life without restoring combat ability; lasting impairment and rehabilitation persist.

Evacuation has a physical chain: reach the casualty, stabilize as feasible, move to an aid point, transport if required, and obtain available treatment. Carrying removes people from other tasks. Full vehicles, blocked routes, or an overwhelmed aid site cause a visible queue and prognosis risk.

**Player:** Designate aid/collection sites and evacuation support; set whether the mission or recovery takes priority; allocate scarce transport or medical access. Decide exceptional abandonment, detours, or Archon treatment bargains.

**Automatic:** Triage, ordinary treatment, carrier selection, compatible material use, treatment queueing, and updates as conditions change.

### 15.2 Death, missing personnel, and records

Confirmed death, missing, captured, and unresolved absence are distinct. The game does not tell headquarters that an unobserved person died. Equipment remains where it was lost or travels with whoever recovered it. Body recovery is optional; absence of recovery affects knowledge and mourning rather than imposing a compulsory chore.

Records can later reconcile a disappearance. A returning survivor brings only what they remember, possess, or can report. Their return cannot retroactively reveal every hidden event.

### 15.3 Prisoners and detainees

Capture requires surrender or physical control and enough supervision to maintain custody. Prisoners consume provisions, space, care, and guard capacity. Their location and custody persist through transport, escape, exchange, release, rescue, or death.

The default is accept surrender when feasible, disarm, guard, provide available care, and report. The player can arrange exchange, release, transfer, recruitment consideration, or adjudication through an authorized authority. No interrogation minigame or torture optimization system exists. Interviews take time and yield testimony with provenance and credibility, not guaranteed truth.

Exceptional violence or refusal of protection creates character, witness, diplomatic, and military consequences under the same world model. Those consequences depend on evidence reaching people; they are not a universal morality meter.

## 16. Inventory, physical supply, and distribution

### 16.1 Resource model

The top level summarizes **provisions, fuel/power, ammunition, parts, medicine**. Water is always separately visible when it limits desert endurance. Internally, ammunition calibers or energy requirements, spare compatibility, fuel type, and medical capability are preserved. Categories are displays, not interchangeable currencies.

Small identical items can be represented as countable lots. Unique weapons, critical components, and named equipment have individual identities. Mass, volume, condition, container, custody, and location limit movement. Supplies held in a distant store do not increase a squad's carried endurance.

Loadout presets assign compatible equipment by skill, role, strength, task, and availability. The player can lock a significant item to a person. Conflicts show a capability warning and proposed substitution; no ordinary pocket arrangement is required.

### 16.2 Demand and endurance

Forecast consumption separately for **waiting, marching, and fighting**. For resource r, a simple operational estimate is usable local stock divided by expected net consumption, incorporating only sufficiently credible deliveries. The force's limiting endurance is the earliest essential-resource shortfall, not an average of five bars.

Fighting consumption depends on the plan and uncertainty, so it is shown as a range. A delivery forecast includes departure readiness, route time, load, and known threats. Unconfirmed deliveries appear separately; they cannot silently turn an unsafe plan green.

Display the limiting cause in practical terms: “The convoy can reach the junction, but has no fuel allocated for return,” or “The force can wait three days; sustained fighting would exhaust mortar ammunition much sooner.” Exact inventory remains inspectable.

### 16.3 Automatic distribution

The commander chooses sources, permitted corridors, support sites, priority recipients, protected reserves, and escort policy. Logistics leaders then:

1. derive demand from standing missions and presets;
2. reserve compatible stock, carrying capacity, and receiving capacity;
3. plan physical deliveries using known routes;
4. assign available transport and authorized protection;
5. dispatch, report material changes, transfer, and update stocks;
6. reuse transport and schedule the next necessary delivery.

The system avoids trivial deliveries by grouping demand, but cannot delay a critical requirement merely to fill a truck. It cannot borrow protected ammunition or strip another force's necessary load without authorization. A failed delivery releases only reservations for resources still available; captured cargo remains captured.

Local redistribution uses people and time during a suitable lull. Resupply vehicles need physical contact with recipients. The interface may show stock held by nearby allies, but access requires an agreement.

### 16.4 Player interventions

Move a depot, authorize a different corridor, assign escorts, concentrate limited transport, release reserves, ration a category, postpone an operation, or withdraw before endurance fails. Staff propose options with their military consequences. The player never has to schedule every truck or approve every magazine handover.

**Escalation:** Forecast shortfall makes an active mission infeasible, priority commitments compete for one resource, a route is lost, or a transfer requires abandoning protected stocks or personnel. Routine shortages remain notifications when the current plan can absorb them.

## 17. Repair, salvage, recovery, and technological decline

### 17.1 Strict technological boundary

No research tree, invention project, advanced manufacturing, unlimited relic duplication, or automatic replenishment of manufactured equipment. Manuals unlock understanding of surviving systems. They do not manufacture missing machines.

Food, water, and basic civilian subsistence are renewable where existing conditions allow. Military ammunition and manufactured parts begin as finite physical stocks. Existing rechargeable systems can draw from functioning energy sources. The first implementation also treats processed military fuel as finite recoverable supply; adding old-facility replenishment would require a deliberate later rules change, not an assumed hidden production loop.

The content budget must support the campaign: finite does not mean every cache is nearly empty. Large surviving stores, rival inventories, trade, capture, abandonment, and recovery distribute what exists. The generational transition accounts for intervening consumption and loss. It cannot quietly refill the world.

### 17.2 Condition, wear, and failure

Equipment has condition and a small set of relevant component capabilities. Use and harsh environments accumulate wear; damage can cause immediate failures. Preventive maintenance consumes time, tools, parts where needed, and qualified labor. Reliability forecasts reflect known condition and uncertainty about unfamiliar relics.

Failures use deterministic seeded risk tied to exposure and condition. Saving, zooming, or waiting for an animation cannot reroll them. Do not roll so often that functioning equipment becomes randomly unusable without explanation.

### 17.3 Recovery and repair workflow

**Assess → secure access → recover or work on site → obtain compatible means → restore function → verify and return to service.** Not every step is necessary for every repair. A field fix may recover mobility without repairing structural damage. Workshops enable work that field teams cannot perform.

The player selects **restore mobility**, **return to service**, **recover intact**, or **salvage**, with priority and constraints. Engineers show time, donor parts, transport, facilities, uncertain condition, and displaced work. Routine jobs queue automatically by readiness priority.

Cannibalization irreversibly transfers or destroys usable parts from a donor. The player approves it for named/rare assets or when it removes a capability. Previously authorized salvage of generic wrecks proceeds automatically. An item cannot simultaneously exist in a donor and repaired recipient.

### 17.4 Archaeology and exploration

Sites contain bounded physical equipment, records, hazards, and access requirements. An assessment order produces observations and possible restoration tasks. A recovered archive may reveal routes, maintenance instructions, names, or historical evidence. Skill, tools, access, and time determine what can be interpreted.

There is no mandatory lockpicking, wire-matching, or salvage-click minigame. Exploration decisions concern which site to risk, whom to send, what to extract, and what to leave. Artifacts linked to the Soterion carry specific system interactions rather than universal “ancient power” bonuses.

**Escalation:** Scarce donor sacrifice, an uncertain hazardous activation, inability to retrieve an asset within the deadline, or repair priorities threatening an operation.

## 18. Engineering, facilities, and persistent bases

### 18.1 Work projects

Engineering covers assessment, clearing, breaching, demolition, obstacles, field fortification, crossings, route repair, prepared communications, recovery access, and restoration of existing facilities. Orders specify the intended military function and area. Engineers choose the detailed method within available knowledge and equipment.

Every project has prerequisites, physical workers, materials, tools, work rate, access, progress, exposure, and a resulting persistent change. Parallel workers help only where tasks can actually be divided. More people cannot compensate for the absence of a necessary tool or usable component.

Interrupted work remains as a partial state: an unfinished crossing, exposed repair, marked hazard area, or incomplete defensive position. Capturing a half-restored facility does not grant full service. Ownership changes may also remove the people who understand it.

### 18.2 Facility service model

| Facility | Inputs and constraints | Output |
|---|---|---|
| Depot | Physical stocks, staff, handling capacity, protection | Storage and throughput |
| Workshop | Surviving equipment, tools, compatible parts, skilled staff, power | Restoration of existing assets |
| Aid site | Medical skill, supplies, beds/space, access, utilities | Treatment and recovery capacity |
| Relay site | Working equipment, power, position, access | Defined communications coverage |
| Training site | Instructors, trainees, equipment, provisions, time | Skills and shared practice |
| Water/power service | Functioning existing infrastructure, operators, inputs | Water or energy within bounded capacity |

Outputs stop or decline when prerequisites fail. Capacity is shared between military and civilian clients. Requisitioning every treatment bed or power connection has visible local consequences.

### 18.3 Base interaction

The player selects a site, desired services, protected assets, access, defensive areas, and allowable resources. Engineers and staff propose a plan. Existing structures are adapted where possible; ordinary shelters and earthworks are built with available materials. Advanced equipment must be recovered.

After commitment, routine layout, staffing, shift work, maintenance, and transfers run automatically. Detailed facility placement is optional where geography matters. There is no building upgrade tree. A larger base requires more actual assets and obligations, not a tier purchase.

Minefields and other obstacles are physical, persistent, and imperfectly known. Their construction, detection, marking, clearance, and degradation are abstract engineering tasks with capability and time costs. The player selects the intended area and effect; engineers choose the method.

**Escalation:** Material shortages, unsafe access, destroyed prerequisites, competing service priorities, a major schedule overrun, or abandonment that leaves important people or assets exposed.

## 19. Characters, relationships, recruitment, and training

### 19.1 People as persistent participants

Every soldier has a name, portrait, origin, skills, leadership qualities, traits, health, fatigue, service record, and relationships. Generic recruits can become notable through service and events. Named characters use the same combat rules.

Skills improve through relevant experience with diminishing returns and require instruction or unusual opportunity for new disciplines. Training consumes instructors, equipment, provisions, and time. Combat does not grant abstract levels or instantly teach unrelated specialties.

Traits express stable tendencies or consequential changes. They affect judgment and behavior in specific situations. No trait should be an unexplained universal bonus. A cautious leader values preservation; they do not make every subordinate's armor stronger.

### 19.2 Relationships and psychological consequences

Directional affinity records how one person regards another. Trust, affection, fear, resentment, obligation, kinship, and mentorship are readable interpretations of significant relationships. The conceptual relationship space covers every pair, but untouched relationships use defaults; persistent storage can remain sparse.

Ordinary affinity contributes quietly to integration, cohesion, and recovery. Only strong relationships create explicit unusual reactions: a rescue attempt, refusal of a successor, shared surrender, or conflict after abandonment. These require established causes and visible warning when the commander could anticipate them.

Trauma is an individual persistent history with authored manifestations, triggers, coping, and potential recovery. It is not a universal “madness” score or a source of random incompetence. Rest, stable relationships, treatment, role changes, and meaningful experiences can help where appropriate. Simulation does not imply a clinical model of real people.

### 19.3 Recruitment and replenishment

Recruitment draws from actual settlement pools, returning personnel, volunteers, allies, or prisoners who choose to join under appropriate circumstances. It requires willingness, access, equipment, training, provisions, and leadership capacity. No manpower appears because a unit slot is empty.

The player sets broad recruitment policy and chooses squad assignments or a reinforcement preset. Administrators present notable candidates and capability shortages; routine enrollment is automatic. Replacements take physical time to arrive and need integration. Rapid expansion dilutes instruction, equipment quality, and command depth.

### 19.4 Appointments and development

Appoint field commanders, squad leaders, specialists, instructors, and administrators. One person cannot work full time in several distant roles. Moving them requires travel and handover. Local succession operates when command cannot intervene.

Automatic training schedules fill authorized rest/reconstitution periods. The player chooses a capability goal and available time, not every drill. Readiness shows skill, cohesion, equipment, condition, and missing means separately.

**Escalation:** Critical vacancy, contested succession, a significant refusal or relationship rupture, a recruit decision with political stakes, or training/rest needs that conflict with an operational deadline.

### 19.5 Conversations and personal commitments

Character scenes arise from shared service, a consequential order, a pending appointment, known relationships, or a character seeking the commander's attention. They occur through a plausible meeting or communication. A conversation may reveal a personal boundary, provide evidence, offer a role, dispute an order, or create a promise.

The player chooses substantive responses and commitments. There is no repeated gift loop, daily conversation checklist, or universal dialogue skill that substitutes for an actual relationship. Personal commitments join the same obligation system as institutional ones: a promise to recover a missing person matters when a later route or deadline makes it costly.

Ordinary affinity, private memories, and concealed loyalties are not omniscient character statistics. The roster shows reported service, observed behavior, declared values, and known relationships. Once a hidden trait affects conduct, the player can investigate or reinterpret earlier evidence. Known appointment conflicts should be warned before commitment; unrevealed ones must still have an established internal cause.

## 20. Doctrine and institutional memory

Doctrine is a short set of standing principles used when orders omit detail or communications fail. Defaults are competent and usable at campaign start. The player is never required to program the AI before it can act.

| Doctrine area | Baseline | Meaningful alternatives |
|---|---|---|
| Initiative | Adapt method within current intent | More conservative reporting; broader opportunity exploitation within a named area |
| Preservation | Avoid losses that no longer serve the objective | Urgent commitment with explicit purpose; earlier fallback |
| Expenditure | Spend what the assigned effect reasonably needs | Conserve scarce categories; prioritize rapid effect |
| Lost communications | Continue feasible intent, then agreed fallback | Return at a review time; hold and seek contact |
| Casualties | Provide feasible aid and recover without sacrificing the whole force | Evacuation takes priority; continue a declared critical objective |
| Civilians/prisoners | Respect declared protection and surrender rules | Player political choices may alter obligations with consequences |
| Emissions | Communicate what the mission needs | Restricted reporting; greater reporting at increased exposure |
| Relics | Operate known systems within approved conditions | Require new authorization for uncertain integration or activation |

Task-force overrides are limited and explicit. A local order can supersede a soft preference but must flag any hard policy conflict. Leaders do not silently rewrite army doctrine.

The game observes repeated consequential decisions and may propose a habit: “Our units repeatedly withdraw before losing mobility; formalize that preference?” Suggestions arrive at natural reviews, not during every encounter. Adoption is a player choice. No single desperate action becomes permanent doctrine automatically.

Changing doctrine takes communication and, for new methods, training. The command picture shows which formations have received and practiced a revision. Descendants inherit records, teachers, stories, and disputed practice rather than a perfect global settings file.

## 21. Settlements, civilians, trade, and political control

### 21.1 Settlement simulation

Settlements track population groups, subsistence, water, service access, shelter, health pressure, security, leadership, political relationships, recruitable people, and stored goods. Ordinary civilians are cohorts; named officials, contacts, specialists, families, and witnesses become persistent individuals when relevant.

Existing civilian activity produces basic provisions and maintains everyday life within local capacity. There are no player-operated industrial recipes. Scarcity affects prices, migration, support, recruitment, and political decisions. Population cannot recover instantly after displacement or losses.

Daily or event-driven updates resolve needs, service interruptions, movement, disease pressure, and local decisions. Disease remains a bounded exposure-and-care model; there is no detailed pathogen laboratory or medical management game.

### 21.2 Physical civilians and caravans

Civilian movement occurs in represented groups with routes, carrying capacity, needs, escorts if any, and affiliations. Near an interaction, relevant members or crowd groups occupy physical space. Their casualties and displacement reconcile to the same population ledger; presentation detail cannot change vulnerability.

The founding caravan has people, transport, workshops, provisions, and obligations. Staff handle its internal life. Its presence matters when choosing routes, protecting dependents, allocating scarce transport, resting, or evacuating.

### 21.3 Control and support

Political claims express asserted authority. Military control expresses practical observation, access, patrol, response, and sustainment. Neither is a capture-circle flag. A garrison can dominate a town while being unable to secure the road beyond it.

Local support is a relationship with a population and its institutions. Protection, reliable agreements, kinship, ideology, dependence, coercion, collateral harm, and alternatives influence it. Benefits are concrete: guides, warning, recruits, food, shelter, treatment, transport, and concealment. One reputation score cannot purchase every benefit.

### 21.4 Trade and requisition

Trade agreements specify goods/services, quantity or priority, location, delivery, consideration, and conditions. Routine settlement of established agreements is automatic. Payments may use local accepted currency or barter; each ledger records actual obligations and transfers rather than a universal world wallet.

The player negotiates access, a supply contract, emergency purchases, or requisition. Merchant or coalition staff select ordinary lots and complete delivery under the agreed terms. Requisition removes goods from somebody's available stock and creates local consequences. Taking food does not leave civilian provisions untouched.

### 21.5 Optional statehood

Statehood remains optional. If the coalition accepts durable governance, appointed administrators handle ordinary administration. The player addresses military access, major obligations, recognition, recruitment policy, emergency allocation, and disputes affecting operations. No mandatory tax, labor, or urban-planning layer is added.

**Escalation:** A threatened agreement, evacuation crisis, service collapse requiring military resources, significant unrest affecting access, or a demand that changes the coalition's obligations. Routine population bookkeeping never interrupts execution.

## 22. Diplomacy, agreements, and faction relationships

Diplomacy occurs through reachable people with authority. A nearby meeting requires travel and security; remote talks require a connection or messenger. Planning freezes the player’s decision time, but dispatch, travel, waiting, and in-world negotiation consume the shared clock.

Agreements include passage, supply, escort, military cooperation, intelligence sharing, medical access, prisoner exchange, ceasefire, surrender, and Archon service. Each records scope, duration, parties, fulfillment, termination, and known authority. A mayor cannot promise every neighboring militia's obedience.

Faction attitudes derive from interests, dependencies, credible strength, commitments, prior treatment, ideology, and observed actions. Individual negotiators may disagree with their institution. A compact summary explains the current position; exact private thresholds are not exposed as a guaranteed persuasion percentage.

**Player:** Choose the desired agreement, substantive offer, concessions, and boundaries. Dialogue presents evidence, threats, appeals, or proposals grounded in known relationships. There is no repeatable conversation farming for free loyalty.

**Automatic:** Routine liaison, deliveries under contracts, renewal requests when appropriate, and enforcement of agreed access rules by informed subordinates.

**Escalation:** A new obligation, change of allegiance, coalition disagreement, disputed violation, surrender terms, or threatened termination of essential services.

Ceasefires take effect for units as orders reach them. Incidents can occur through delay, rejection, or uncertainty; witnesses and subsequent negotiations matter. Peace does not instantly erase deployed forces, contested sites, prisoners, or caches. A surrendered command may dissolve while a rejecting commander becomes a continuing belligerent or successor faction.

### 22.1 Allied forces and limited authority

An ally is not automatically player-controlled. Cooperative forces normally receive requests through liaison and accept according to commitments and their own priorities. An agreement may place specified units under the coalition's operational command for a purpose and duration; those units then accept the same squad-level intents within the agreement's limits.

Personnel allegiance, ownership of equipment, permission to use supplies, and operational command remain separate. The player cannot dissolve an allied formation into replacements, consume its protected stocks, or redirect it to an unrelated war without authority. A change of government or broken agreement can revoke cooperation through communicated orders and physical withdrawal, not instantaneous disappearance.

Promises of allied support appear as commitments with reporting age and confidence. The player can plan around them, but cannot inspect private allied reserves or guaranteed future behavior.

## 23. Enemy command and irregular warfare

### 23.1 Faction planning

Each faction headquarters maintains its own picture of the theater, resource commitments, objectives, relationships, and doctrine. It evaluates possible operations by expected value to its goals, feasibility, cost, risk, political obligations, and opportunity cost. It acts on observed or credibly reported opportunities, not the player's uncommitted plans.

A candidate operation must identify available forces, a plausible route, support, time, and a completion condition. The faction reserves those means and issues orders through its command network. Local leaders execute through the same squad and support systems as friendly forces.

Faction leaders reconsider plans at reports, deadlines, resource thresholds, leadership changes, or periodic reviews. They do not globally replan every frame. An opponent already committed to a distant campaign cannot redirect instantly when the player captures a minor outpost.

### 23.2 Goals and priorities

Goals are concrete: secure a route, restore a site, protect a settlement, obtain an operator, escort a delegation, relieve a garrison, isolate an opponent, or control a junction service. A faction's hierarchy gives these different importance. Internal factions and commanders can disagree about acceptable losses and obligations.

The AI's strategic utility model is a design tool, not a visible objective-point economy. The player learns motives through behavior, negotiations, records, and intelligence. Enemy failures can come from incomplete information, bad estimates, overextension, rivalry, or an authentic capability gap.

### 23.3 Learning and deception

Adaptation requires evidence the faction received. Surviving observers, captured orders, recovered devices, and reports can change threat estimates or doctrine. Losing a unit without a report does not teach headquarters exactly how it died.

Repeatedly observed player behavior can lead to cautious counters, but never a perfect prediction of future clicks. Physical decoys and misleading signals can influence those beliefs. The AI's information dependencies must be inspectable in developer tools.

### 23.4 Irregular forces

Irregular resistance consists of persistent people, squads, leaders, caches, safe locations, contacts, and supply relationships. Cells need food, information, recruits, transport, and usable weapons like everyone else. They move and act on the same terrain.

A stronger force suppresses resistance through real reconnaissance, patrols, route security, protection, raids, and garrisons. These commitments reduce the force available for other operations. Civilian hostility can deny intelligence or support, but it does not spawn infinite enemies from an abstract unrest bar.

The player's own army can disperse into cells when outmatched. Dispersal preserves some people and reduces coordination; it does not make them unfindable. Reassembly requires contact, trust, travel, a purpose, and logistical feasibility.

### 23.5 Defeat and surrender

A faction's military organization ends through recognized surrender or when no recognized command structure or organized armed formation remains. Individual survivors may remain. A destroyed headquarters does not automatically delete independent formations; succession and local authority determine whether they continue.

Capture, leadership removal, desertion, defection, lost services, and negotiation can produce fragmentation. There is no requirement to kill every person in a region. Different successor factions can inherit units, commitments, grudges, and contested claims.

Difficulty should primarily change initial resources, intelligence, competence, political pressure, or decision quality within the same rules. It must not grant offscreen supplies, knowledge, accuracy, or reinforcement spawning.

## 24. Archons, anomalies, and the Soterion junction

### 24.1 A common anomaly contract

Every anomalous system needs six authored definitions:

1. **Where it applies:** a moving body, footprint, connection graph, property relation, boundary, or named target set.
2. **What activates it:** a physical action or state such as entry, contact, authorization, use, return, or transmission.
3. **What it changes:** the exact physical, informational, identity, or service rule affected.
4. **What it depends on:** continuing connections, people, places, devices, or conditions.
5. **What observers can learn:** signatures, repeated outcomes, records, and effective precautions.
6. **What ending or interrupting it does:** service loss, damage, release, migration, or another explicit consequence.

These definitions make play consistent without explaining the cosmology. Characters can learn a useful precaution while remaining wrong about why it works. No noosphere, universal gestalt system, or common consciousness is confirmed.

An anomaly must not change unannounced simply to defeat a successful player plan. Changed behavior needs an underlying state transition and observable evidence. Unknown rules may surprise the player, but must be investigable afterward.

### 24.2 Seven theater powers

The forms below come from the world bible. Their simulation boundaries are game-design proposals. Three or four bodies are physically present in the theater; the others exert influence through particular clients, services, connections, or sites.

| Power | Simulated manifestation | Player interaction and cost |
|---|---|---|
| Autophagos | The hermit-crab city is a moving settlement with inhabitants, access, resource flows, and organs expressed as infrastructure | Negotiate passage or services, defend or disrupt particular connections, learn how the city's survival and body are linked; killing it destroys a functioning society |
| Heliarch | A moving inverted whale affects regional sensing/weather; deposited oil remains attached to actual places, stocks, and bodies and can ignite on its return | Track reported movement, alter routes and exposure, assess contamination, seek patronage; distant sensing requires a defined source and delivery path |
| Pylaios | A gate changes spatial relations and transmits through destruction and reconstruction under specific access conditions | Secure endpoints, negotiate passage, allocate finite service, choose whether people travel; preserve the identity discontinuity in the internal event record without forcing characters to understand it |
| Anodyne | Connected bodies form a directed network of protection and transferred injury | Seek treatment, identify who bears its cost, alter participation or connections, evacuate dependents; damage cannot simply vanish into an unmodeled pool |
| Aletheia | White-gloved hands reveal protected or concealed things within an authored jurisdiction; the central covered absence remains excluded | Bargain over disclosure, protect people from exposure, investigate the exception; revealed information still needs a recipient and communication to benefit a distant army |
| Strategos | Weapon-wings protect a brain whose fear or surrender signals are interpreted as threats | Navigate enforcement areas, bargain with human clients, study escalation triggers, disrupt particular systems; defeating one detachment does not kill the Archon |
| Mneme | The moving black ribbon carries recorded lives and reconstructs its own purpose from what it samples | Obtain testimony or operating knowledge through accessible records, escort specialists, contest archive access; the archive is not a source of guaranteed current intelligence |

Each capability card states its operational extent and known prerequisites. The fact that an Archon has broad metaphysical potential does not give every worshipper a global power. A client must possess an actual relationship, service, or device.

### 24.3 Archon campaigns and death

An Archon is a persistent body and regional arrangement, not a health bar whose level matches the army. Conventional attacks produce the consequences implied by its materials and rules; they cannot ignore transferred injury, continuous reconstruction, or another established property.

A campaign against one proceeds through observation, access, identification of dependencies, protection or abandonment of affected people, and an authored physical intervention that exploits its appetite. Clues do not act as abstract damage-unlock tokens. Evidence teaches a rule; the player can act on that rule whenever the actual preconditions exist.

For each Archon, content authors must define a tested death-state transition, the services lost, who can survive without them, persistent remains, and a limited set of conflicting cosmic consequences. This specification defines the common mechanic; the exact kill scenarios remain authored encounter content rather than pretending seven complete narrative campaigns can be generated from one formula.

### 24.4 Junction state model

The junction's territorial possession, working services, connected clients, and integration authority are separate state variables.

| Operational state | Conditions | Consequences |
|---|---|---|
| Dormant or unreachable | No functioning access chain | Physical site exists; little current service |
| Contact established | Usable interface, power, and surviving protocol/credential | Limited replies, classification, initial records |
| Isolated service available | One restored endpoint and bounded operating authority | A specific transit, communications, power, or repair service works |
| Network coordination | Multiple functioning endpoints and authorized relationships | Shared scheduling and interoperability, greater dependence and legibility |
| Integration underway | Authored activation conditions recognize a connected command institution | Orders, identities, routes, or services are subjected to Soterion authority |
| Severed or contained | Particular links disabled, authority revoked where possible, or endpoints isolated | Loss of associated benefits; only defined integration paths interrupted |

A claimant can control the entrance while lacking operators or power. Another can still use a remote endpoint until that connection is actually denied. Disconnecting one relay affects only the paths that rely on it. The junction cannot invent a fuel depot, heal everyone, or command every machine in the world.

### 24.5 Interaction sequence

The player chooses **Assess, Restore service, Connect, Use, Restrict, Isolate, or Disable**, selects an endpoint or function, and reviews known prerequisites and consequences. Engineers and operators perform physical work. A limited service can be used without granting every form of authority, but the interface shows what is actually known; no perfectly accurate “corruption percentage” reveals the mystery.

The operational display distinguishes physical service availability from the commander's understanding of its consequences. Evidence of integration includes changed permissions, unexpected orders, identity classification, and service behavior. These are concrete events with locations and recipients.

New activation, authority transfer, irreversible disconnection, or significant uncertainty about harm requires the player. Routine operation inside an approved service contract is automatic. Authorizing a service does not silently authorize every later expansion.

## 25. Story, deadlines, failure, and generational change

### 25.1 Narrative events as world conditions

Story content references persistent roles and conditions: the checkpoint commander, a credential, a surviving witness, a settlement's water service, a faction's prepared operation. It should tolerate different people filling a role. Death can close a relationship or change an outcome without making the entire plot incomprehensible.

Events have preconditions, participants, known or discoverable stakes, timing, and consequences. A deadline must be backed by something happening: a column's arrival, a negotiated handover, a stock shortfall, or restoration progress. Interrupting that cause can change the deadline.

Story objectives are places, people, services, and obligations on the theater. They are not instanced missions. A target can move, be captured by someone else, die, or cease to matter before the player arrives.

### 25.2 Opening implementation

The caravan reaches the dormant checkpoint in ordinary movement. Its technician's attempt to restore passage reconnects to the junction. Classification changes create the seizure demand. Strategos's local detachment acts on the instruction and its own information.

The player can negotiate for time, prepare an escape, resist detention, or break out after partial capture. The opening must support imperfect survival. Evidence can remain in several plausible channels—testimony, records, intercepted traffic—so the tutorial does not require an invulnerable courier.

Tutorial prompts teach selection, purpose, support, and review at natural planning pauses. They never require clicking individual soldiers or performing a scripted combat exploit. Afterward, ordinary faction decisions drive pursuit and the next settlement's response.

### 25.3 Failure and campaign continuity

Most objectives have graded results: secured intact, secured damaged, temporarily denied, exchanged, abandoned, or lost. Tactical retreat can preserve the army and still count as political failure. Captured personnel may become future objectives. Losing equipment changes later capability.

Campaign defeat occurs when there is no viable player-controlled institution left to continue and no authored surviving nucleus can plausibly assume play. Exceptional critical-story failure conditions must be declared before commitment. Killing the current commander alone is not a reload condition when succession is possible.

### 25.4 Authored catastrophe boundary

The existing two-generation premise retains a Soterion rupture. The player controls preparation, exposure, participation, alliances, and rescue. The story does not claim the player can indefinitely prevent the central transition. If another faction initiates it, that faction needs a physically supported path to doing so; the script must not conjure a completed network immediately after the player destroys it.

Content must provide credible alternative initiators and prerequisites. If none survives in a particular campaign state, delay or reframe the rupture using a previously established remaining mechanism. Do not invalidate successful operations through an unexplained reset. This is a key campaign authoring test.

During the rupture, threats propagate through defined connections. Breaking a relay, evacuating a site, preserving independent records, or separating a unit beforehand materially changes outcomes. The game asks what to save through ordinary orders and competing resources, not a detached menu of abstract sacrifices.

### 25.5 Twenty-year transition

The generation jump is an explicit narrative transition, not ordinary high-speed theater execution. A deterministic history process advances surviving people, institutions, settlements, routes, and stores through bounded authored events and causal dependencies. It preserves provenance and resource accounting. It does not attempt twenty years of unobserved shot-by-shot war or reuse an offscreen combat shortcut during active play.

Carry forward:

- surviving, captured, missing, transformed, and dead people;
- family, adoption, mentorship, and guardianship;
- equipment condition, salvage, caches, finite stock consumption, and destroyed assets;
- settlement survival, migration, leaders, commitments, and access;
- preserved records, instructors, doctrine, and contested traditions;
- Archon states, services, junction connections, and unresolved threats.

Descendants emerge from these relationships and opportunities. There is no childhood-bonus menu. Second-generation characters inherit opportunities and burdens, not their parents' exact statistics. Missing records produce uncertainty, not secretly restored lost knowledge.

The second campaign returns to the same geographic world after documented changes. Ruins, service failures, altered populations, and memories make it recognizable. The player receives only the successor institution's available information; personal recollection of the first campaign is not automatically a complete contemporary map.

## 26. Interaction catalog and command outcomes

This is the complete initial vocabulary. Specialist tasks appear contextually under an appropriate primary action; the player is not shown a permanent wall of buttons. Every order also uses the shared authority, delivery, resource, and fallback rules in section 4.

### 26.1 Field orders

| Order | Minimum player input | Completion or persistence | Default response if infeasible |
|---|---|---|---|
| Move | Destination; optional emphasis | Arrive in viable local position | Adapt locally, otherwise hold safely and request |
| Observe | Area, object, or intelligence question | Persist until condition/time or question answered | Preserve concealment, relocate within area, report limits |
| Screen | Corridor/area and protected force | Persist while protection task exists | Maintain warning function, avoid unauthorized decisive engagement |
| Trail | Contact and exposure limit | Persist while contact can be followed | Search plausible local continuation, report loss without tracking truth |
| Patrol | Area/route and purpose | Repeat within assigned area and endurance | Use authorized alternative; report lost coverage |
| Escort | Protected force/person/cargo and destination | Reach destination or release condition | Preserve protected element, seek viable route or new authority |
| Hold | Position/frontage and optional condition | Persist | Reposition within area; use fallback if position untenable |
| Suppress | Target/area and supported purpose | Required effect, allowance, or release | Seek feasible position/support; report inability |
| Kill / Destroy | Designated target | Credible assessed destruction or loss of feasible contact | Report uncertain result or missing capability; do not chase indefinitely |
| Assault | Position/objective | Take and secure objective to defined standard | Suspend, support, or withdraw within authority |
| Support | Recipient/area and support role | Persist while commitment applies | Report capability or priority conflict |
| Retreat | Destination/rally area or approved fallback | Disengage and reorganize there | Use best viable authorized escape; report losses/encirclement |
| Resupply | Source and preset/capability | Physical replenishment to available authorized target | Partial transfer if useful; report limiting stock/access |
| Evacuate | People/casualties and receiving point | Physical handover to valid care or protection | Stabilize/protect locally and request means |
| Recover | Asset and destination/purpose | Asset recovered or assigned salvage completed | Secure/mark, estimate missing means, request |
| Rest / Reconstitute | Site, readiness goal, or available time | Goal/time reached | Continue feasible recovery; report missing care, stock, or safety |

Screen, Trail, Patrol, and Escort are specialized Observe/Support/Move intents internally. Their labels clarify the desired result without exposing battle drills.

### 26.2 Specialist and persistent actions

| Action | Player decision | Automatic work | Decision returned when |
|---|---|---|---|
| Request fire | Effect, target, priority | Observer/weapon assignment and execution | Data, expenditure, or coordination limits prevent action |
| Defend against air threats | Protected asset/area, priority | Detection and feasible interception | Coverage or finite ammunition cannot meet obligations |
| Listen / Jam / Deceive / Protect | Area or supported force | EW method and operator tasking | Exposure or interference exceeds authority |
| Assess site/hazard | Question, area, risk | Inspection and interpretation | Assessment needs new risk or rare capability |
| Breach / Clear | Desired passage/area | Feasible engineering task | Means absent or exposure unacceptable |
| Fortify / Prepare crossing | Function, area, resource allowance | Survey, layout, sequencing | Scarce resources or deadline conflict |
| Demolish / Disable | Object/function and permitted effects | Authorized specialist work | Dependencies or civilian consequences exceed permission |
| Restore | Desired capability, priority | Diagnosis, repair queue, verification | Donor sacrifice or new hazard requires authority |
| Salvage | Site/assets, preservation limits | Recovery, sorting, transport | Rare asset destruction or storage/transport conflict |
| Establish depot/aid/relay | Site, purpose, allocated assets | Setup and ordinary operation | Essential input or protection fails |
| Set supply service | Sources, corridor, recipients, priority | Forecasting, loads, dispatch, transfers | Mission shortfall or protected resource conflict |
| Train | Capability goal, personnel/time budget | Sessions and assessment | Mission or essential instructor conflict |
| Recruit / Reinforce | Policy, available resources, receiving units | Enrollment, travel, preset assignment | Notable candidate or incompatible commitment |
| Appoint / Reorganize | People and roles | Communication, handover, integration | No valid command path or known refusal |
| Negotiate / Exchange | Desired agreement and offer | Liaison and delivery | Substantive terms or authority change |
| Manage obligations | Major policy or emergency allocation | Ordinary administration | New political/military commitment |
| Operate junction service | Function, endpoint, bounded authority | Approved operation | Expanded integration, hazard, or disconnection cost |

### 26.3 Decisions that are always explicit

Commit a protected reserve; abandon a declared protected group; authorize a new uncertain anomalous service; sacrifice a unique donor asset; enter or materially alter an alliance; accept durable governance; authorize a major change in risk; change the operation's objective.

These may be pre-authorized narrowly in a specific plan. They cannot be smuggled into a general “be aggressive” setting.

### 26.4 Reusable escalation card

Every decision card contains **what changed**, **who reported it and when**, **which assumption failed**, **what continues automatically**, **two or three feasible alternatives**, and **the resource/time/risk difference**. Alternatives are proposals, not exhaustive choices; the player may issue a different valid command.

Example: “The crossing cannot carry the loaded transport. The escort is holding at the western approach. Engineers estimate a repair after inspection; a reported southern crossing adds travel but has not been checked. Choose repair assessment, scout the alternative, or unload and continue with reduced carrying capacity.” No option supplies knowledge that has not been obtained.

## 27. Complete play sequences

### 27.1 The checkpoint incident

1. **Planning:** The caravan is physically approaching the checkpoint. The player assigns an observer, a holding position, an escort, and a protected reserve. Other guards retain ordinary march orders.
2. **Execution:** Operators attempt the approved restoration. The checkpoint answers, issues a seizure classification, and transmits to a local Custodian detachment. The coalition learns only what its people observe or receive.
3. **Decision:** The demand conflicts with the caravan's objective and obligations. The player may negotiate for time, prepare departure, or resist. The enemy's own movement and orders remain physical.
4. **Contact:** Guards deploy from their actual positions. Leaders seek cover, coordinate support, and aid a casualty without individual clicks. The reserve stays protected unless its condition is met.
5. **Complication:** A transport loses mobility. Its crew continues useful work while engineers assess it. The player receives a decision only when recovery competes with the escape deadline.
6. **Withdrawal:** The player chooses which asset to leave or recover. People, cargo, enemy observations, and damaged infrastructure persist. No victory screen replenishes the force.
7. **Aftermath:** Leaders reorganize and resume the caravan mission if feasible. Reports, testimony, and surviving transmissions determine what the next settlement and enemy headquarters know.

This sequence exercises movement, orders, information, combat, repair, evacuation, logistics, and story through one interface and clock.

### 27.2 A three-day march and a lost crossing

**Initial situation:** The coalition must reach a settlement before a handover. The player accepts a guarded route and assigns an automated supply service. One scout remains elsewhere under Observe. The force carries enough provisions for the plan but depends on a scheduled fuel delivery.

During execution, the scout continues independently. A bridge on the supply corridor collapses from previously modeled damage. The transport driver discovers the blockage and reports; until that report reaches logistics staff, headquarters retains the old estimate. Internal simulation changes do not give the player an early warning.

The delivered report causes the forecast to miss the protected return-fuel threshold. The system groups affected commitments into one card. The player chooses among restoring access, scouting a longer route, releasing reserve fuel, reducing the deployment, or postponing the operation. Staff schedule whichever choice is authorized.

If the player arrives late, the handover may occur. That changes access and the next objective; it need not end the campaign. This is logistical play without truck-by-truck management.

### 27.3 Two simultaneous contacts

Two task forces have their own commanders. The western headquarters receives a request to commit its reserve. At the same time, an eastern patrol has lost contact and is fighting under its last intent.

The western request pauses the shared clock. The player can inspect the eastern command picture, but sees only the patrol's last report. The patrol's current casualties and positions remain hidden. Planning in the west cannot directly command the isolated squad.

After commitment, both regions advance. The patrol's leader withdraws when its standing limit is exceeded. A survivor later reaches a connected unit and the story enters the eastern picture. No camera movement or pause revealed it early.

### 27.4 Restoring a relic without a research tree

A recovered sensor vehicle has a working array but damaged power conversion. Engineers identify a compatible component in another surviving chassis. The player sees the donor capability that will be lost, work time, transport, and remaining reliability uncertainty.

The commander authorizes the trade and gives restoration priority over routine work. Actual components move, staff spend time, and both vehicles' states change. The restored sensor extends a particular force's observation capability; it still needs operators, power, communications, and protection. The faction has recovered an option, not unlocked an infinite production line.

### 27.5 Junction activation and containment

The coalition holds an entrance and operates a limited relay service. Another faction controls a power endpoint. Operators propose expanding coordination to improve dispersed operations. The player reviews known permissions and dependencies before authorizing or rejecting the expansion.

When the campaign's rupture begins through a supported activation path, affected services issue new classifications and orders. Units react according to what they receive. The player must choose which links to sever, which people to evacuate, and which records or equipment to preserve. Severing a link can interrupt integration but also remove communication or power needed for rescue.

The outcome is recorded per person, site, service, and institution. The later generation inherits those specific survivors and absences. A cutscene cannot quietly undo an evacuation the simulation completed.

## 28. Simulation architecture and performance

### 28.1 Authoritative state and presentation

The simulation owns positions, knowledge, orders, effects, inventories, relationships, and event times. Rendering interpolates and summarizes. UI forecasts read a faction's beliefs; developer diagnostics may inspect ground truth separately. Cosmetic randomness never consumes gameplay random streams.

Use stable event keys and independent seeded streams for perception, reliability, combat, and authored variability. Store stream state, event queues, order versions, reservations, and beliefs in saves. Equal state, seed, and committed orders should reproduce equal authoritative outcomes.

### 28.2 Spatial representation

A two-meter grid over a 1,000 × 1,000 km region has approximately 250 billion locations. This is an address space, not permission to allocate a mutable object per cell. Store terrain in compressed streamed chunks, objects and modifications sparsely, and pathfinding at several spatial resolutions that reference the same geometry.

Chunk boundaries cannot interrupt a shot, radio path, fire front, moving force, or observer. Interaction queries cross chunks. Cache terrain and visibility data with explicit invalidation when structures, smoke, weather, or other relevant state changes.

### 28.3 Simulation frequency by activity

Active close interactions use fine tactical steps and continuous collision checks as needed. Quiet travel, rest, maintenance, and service flows can advance analytically to their next event. All military people and equipment retain identities. Distant combat uses the same effects and beliefs, regardless of what is rendered.

Before a quiet process could interact with another actor or hazard, bring the relevant entities to a consistent timestamp and appropriate update detail. Conservative swept bounds and scheduled observation opportunities prevent a fast-moving column from passing through an ambush, hazard, or potential sighting between updates.

If exact analytical advancement is not possible, use smaller steps. Do not substitute lower simulation fidelity merely to maintain attractive fast-forward speed. Simulation throughput may limit how quickly large periods can be processed; playback must not promise a fixed acceleration at every workload.

### 28.4 Shared service systems

Use common reservations for people, stock, vehicles, facility capacity, and work slots. Use common movement and custody transfers for cargo, casualties, prisoners, and passengers. Use common report provenance for scouts, negotiations, archives, and anomalous observations. These prevent each feature from inventing incompatible shortcuts.

A service interruption publishes changed capability and affected commitments. Dependent plans reassess only if relevant; an unrelated weather report should not make every squad recompute its route.

### 28.5 Data contracts

An **Order** contains issuer/authority, recipients, version, intent, target reference, constraints, priorities, known assumptions, completion rule, follow-on, fallback, and delivery state.

A **Plan** contains tasks, prerequisites, resource reservations, dependencies, estimated intervals, local branches, abort conditions, belief version, and explanation.

A **Report** contains source, observed time, transmitted content, classification/confidence, location region, recipients, delivery state, and provenance links.

A **World change** contains timestamp, actual causes, affected entities, physical/resource deltas, observable signatures, and resulting events. Observation is derived from this event under perception rules, not automatically broadcast.

A **Campaign handoff** contains surviving identities and assets, accounted losses, preserved knowledge, relations, institutions, and authored historical transitions. It is distinct from ordinary distant simulation.

### 28.6 Save, loading, and recovery

Save at planning boundaries and optionally maintain an internal recovery checkpoint during long execution. A resumed execution retains pending events and cannot reroll outcomes. An autosave before irreversible commitment is a usability feature; ironman restrictions are optional and cannot substitute for reliable behavior.

No player visual rewind of hidden action is included. Developer replay and state hashes are essential for diagnosing causal errors, nondeterminism, information leaks, or double-spent resources.

## 29. Prototype values and balancing levers

These are initial test values or hypotheses, not measured research results. The systems above remain the design even if these numbers change.

| Parameter | Starting point | Validation question |
|---|---|---|
| Terrain sample spacing | Approximately 2 m; finer authored object geometry | Can cover and navigation coexist with feasible world storage? |
| Prototype region | Continuous 20–30 km corridor | Do travel, support, contact, and aftermath work without mode changes? |
| Initial friendly force | 4–6 squads plus transport/support | Is delegation understandable before scaling? |
| Target concentrated operation | 15–25 squads/support elements | Can players issue and review meaningful orders without recurring clerical work? |
| Active tactical step | Trial 0.1 s with continuous shot/collision event timing | Are interactions stable without excessive computation? |
| Local leader reassessment | Event-triggered, with staggered trial checks around 1–3 s during active contact | Are decisions responsive without constant plan churn? |
| Quiet settlement review | Daily plus immediate relevant events | Do shortages and population changes remain causal? |
| Notification batching | By cause and operation at a decision/review point | Does one problem produce one useful decision? |
| Travel example | 90 km guarded foot movement over roughly 3–5 days in suitable conditions | Do rest, heat, security, and supply dominate credibly? |
| Motor movement example | 250 km over roughly 1–3 days on impaired/contested routes | Can intact routes be faster without making distance meaningless? |

Tune lethality through credible exposure, weapon effectiveness, protection, detection, and support—not inflated health pools for important characters. Tune difficulty through force circumstances and competence. Tune decision density through standing authority and meaningful escalation, not artificially making AI fail more often.

Measure **corrective interventions** separately from deliberate changes of plan. A player redirecting a reserve after new information is commanding; repeatedly rescuing squads from obviously bad routine execution is compensating for broken automation.

Rest, logistics, and preparation should create choices before commitment and exceptions afterward. They should not generate a tax of regular confirmation clicks.

## 30. Verification and implementation sequence

### 30.1 Acceptance scenarios

| Scenario | Required result |
|---|---|
| Feasible Hold with local threats | Leader handles cover, fire distribution, aid, and adaptation without corrective soldier orders |
| Objective still valid after contact | Unit resumes rather than idling until clicked |
| Impossible Assault | Preview or received reassessment identifies missing means; fallback remains useful |
| Disconnected patrol under attack | No current telemetry or hidden-event pause reaches headquarters |
| Out-of-order messages | Old order cannot overwrite a newer received version |
| Two plans claim one truck | Reservation conflict appears; no duplicate transport capacity |
| Bridge loss | Navigation, traffic, supply, arrival, and story dependencies update through evidence |
| Local leader death | Qualified succession works; conflict occurs only from established causes |
| Captured supplies | Ownership/custody change once; loser cannot still spend them |
| Offscreen fighting | Same state and orders produce the same combat result with any camera location |
| Long quiet advance | No actor skips a potential observation or interaction |
| Fire-support uncertainty | Stale friendly/target data creates an explicit limitation, not omniscient coordination |
| Evacuation queue | Treatment and transport compete physically; prognosis changes coherently |
| Repair by cannibalization | Donor loses the actual part and associated capability |
| Faction adaptation | Changed enemy behavior traces to acquired evidence |
| Ceasefire | Disconnected units stop only when orders arrive or local agreement occurs |
| Anomalous service loss | Dependent populations/forces lose the actual benefit; no generic curse debuff substitutes |
| Twenty-year handoff | People, relationships, records, assets, and accounted consumption follow prior outcomes |

### 30.2 Development order

**First: command trust.** Implement the shared clock, local knowledge, message delivery, a small terrain area, squad movement, core intents, suppression, succession, and explanations. Prove sensible autonomous execution and information boundaries.

**Second: one complete operation.** Add physical supply, transport, wounds, evacuation, recovery, repair, a crossing, and persistent aftermath. Run the checkpoint and lost-crossing sequences from preparation through another mission.

**Third: simultaneous forces and capable opposition.** Add multiple headquarters, support conflicts, reconnaissance, fire support, EW, and an enemy operation planner using the same rules. Test 15–25 friendly elements without UI overload.

**Fourth: living campaign.** Add settlements, agreements, recruitment, training, doctrine, named relationships, irregular cells, and faction continuity. These should feed existing command decisions rather than introduce a managerial interface.

**Fifth: Achlydesa's defining systems.** Implement one Archon service and the junction's bounded connections. Prove that anomalies follow inspectable, consistent rules and produce ordinary military and human consequences.

**Sixth: scale and generational content.** Validate terrain streaming and event advancement, then expand the authored theater, Archon campaigns, and history handoff. Do not build a continent's content before the command model survives realistic workloads.

### 30.3 Scope control

The full design covers the intended systems, but not every asset statistic, encounter script, animation, or line of dialogue. Those are content and tuning work. The largest unresolved engineering risks are world-scale simulation throughput, reliable multi-unit planning under imperfect information, and preventing alert overload. The largest narrative risk is making an authored catastrophe respect successful sandbox actions.

No engine, multiplayer mode, procedural whole-world generator, or free-text command parser is required by this design. Baseline play is single-player; a multiplayer mode would need a separate policy for shared planning pauses and information boundaries.

## 31. Research basis and design boundaries

The game uses real concepts to choose relationships worth simulating. The particular mechanics, defaults, event rules, and fictional manifestations are design proposals. No publication establishes that this complete game is feasible or balanced.

| Primary or official source | Relevant support | Application and limit |
|---|---|---|
| [U.S. Army University Press: Mission Command, May 2020](https://www.armyupress.army.mil/Journals/NCO-Journal/Archives/2020/May/Mission-Command/), discussing ADP 6-0 (2019) | Competence, trust, shared understanding, intent, mission orders, initiative, and risk acceptance | Command outcomes and constraints while delegating execution; doctrine does not prescribe our UI or pause system |
| [USMC: MCDP 4 Logistics publication announcement, March 2023](https://www.marines.mil/News/Messages/Messages-Display/Article/3335876/availability-of-marine-corps-doctrinal-publication-4-logistics/) | Logistics as part of operational planning and the conditions for endurance and reach | Physical supply and support constrain feasible missions; automatic distribution is a game design choice |
| [Neufeld, Mostaghim, and Brand, 2018: A Hybrid Approach to Planning and Execution in Dynamic Environments Through Hierarchical Task Networks and Behavior Trees](https://cdn.aaai.org/ojs/13044/13044-52-16561-1-2-20201228.pdf) | Combining high-level task planning with reactive local execution | A candidate subordinate-AI architecture; the paper's small experiments do not validate this theater scale or military domain |
| [Parasuraman, Sheridan, and Wickens, 2000: A Model for Types and Levels of Human Interaction with Automation](https://doi.org/10.1109/3468.844354), [indexed abstract](https://pubmed.ncbi.nlm.nih.gov/11760769/) | Separates automation of acquisition, analysis, decision selection, and action implementation | Automate routine acquisition/execution and assist analysis while preserving consequential player decisions; the specific boundary must be tested in play |

Previously cited tactical manuals in the companion documents remain useful background. This specification does not rely on unverified numerical march rates, medical procedures, or weapon performance as authoritative facts. All quantitative examples are explicitly prototype assumptions.

The final design criterion is practical: a system earns its place when it changes a command decision, makes a person's or place's history matter, or makes Achlydesa's rules tangible. If it mainly asks the player to service it repeatedly, its routine execution belongs to subordinates or the background simulation.
