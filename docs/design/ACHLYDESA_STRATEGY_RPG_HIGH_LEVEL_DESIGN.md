# Achlydesa Strategy RPG — High-Level Design

**Status:** Consolidated high-level design; command, caravan opening, unified grid, and repair-only technology clarified 25 September 2026  
**Working title:** To be determined  
**Genre:** Continuous-theater WEGO military RPG  
**Setting:** Achlydesa, approximately twenty years across two generations  
**Primary fantasy:** Command a persistent army whose people, equipment, doctrine, relationships, victories, failures, and dead all become history.

---

## Executive brief

**The game is about commanding a force through a war that continues everywhere at once.** The player chooses where to operate, what to achieve, which positions and targets matter, and how to sustain the force. Subordinates turn those decisions into movement, observation, fighting, maintenance, and recovery.

The opening follows a small caravan escort. A reactivated Soterion checkpoint attempts to seize the caravan, and defending it places the guards in conflict with Strategos's enforcement apparatus. Surviving that incident draws the emerging coalition into a regional struggle over the buried junction that issued the order. The proposed checkpoint incident and faction organizations below are game-specific additions; the Archons and Soterion derive from the current world bible.

| Design commitment | Player experience |
|---|---|
| One 2.5D theater, one spatial grid, one clock | A march becomes a battle wherever contact occurs; no loading into an arena or resetting deployments |
| Command through intent | Choose a squad or group, give an objective, inspect its leader's proposed method, commit |
| Meaningful command responsibility | Set positions, targets, priorities, operational routes, reserves, support, and logistics |
| Automatic routine execution | Leaders manage cover, formations, fire discipline, local reactions, redistribution, and ordinary maintenance |
| Subcontinental distances | Routes and support determine whether an operation takes hours, days, or weeks |
| Persistent people and material | Casualties, captured equipment, damaged routes, and abandoned stores remain part of the campaign |
| Modern military relationships | Reconnaissance, suppression, combined arms, communications, endurance, and recovery determine options |
| Repair-only advanced technology | Recover and restore existing capabilities; no research tree, invention queue, or newly manufactured advanced arsenal |
| Open campaign with authored consequences | Factions pursue the junction independently; player choices change access, alliances, casualties, and the conditions of the central story |

The ordinary decision cycle is: **assess reports → choose the objective → assign forces and support → commit execution → respond when the plan needs new authority**. Fighting, travel, and logistics all belong to that cycle.

The player still directly commands squads and support elements. Task forces provide selection, shared priorities, and bulk orders; an additional autonomous general does not take those units away from the player. Delegation happens inside the order, where trained people choose how to accomplish it.

Existing campaign decisions retained here include a persistent army rather than one irreplaceable avatar, event-driven WEGO, a typical concentration of 15–25 squads/support elements, and the two-generation story. The newest brief takes precedence where it narrows earlier technology provisions. Numerical terrain resolutions, travel examples, new faction names, and opening details are proposals to validate in a prototype.

---

## 1. High concept

The player commands a stateless military coalition moving through a surreal, subcontinental region shaped by a few physically distant Archons and the reach of others beyond the theater. A buried Soterion coordination junction has begun operating again. Regional powers believe they can control the junction; none understands that it is attempting to reconstruct the command architecture of the dead world.

The game combines:

- one tactically detailed, freely traversable theater governed by a single event-driven WEGO clock;
- multiple independent task forces that remain physical and active everywhere on that theater;
- physical logistics, communications, reconnaissance, recovery, and restoration of useful bases;
- an RPG roster of named people organized into freely composed squads;
- a generational story in which relationships and institutions from the first campaign determine the descendants, cells, doctrines, and ruins available after an inevitable catastrophe.

Tactical command is the main attraction. Long-distance movement, preparation, intelligence, logistics, bases, and political consequences create the circumstances of combat without becoming a separate campaign mode. The player should spend time making military decisions, not performing clerical work.

---

## 2. Design pillars

### 2.1 Intent, not puppetry

The player commands squads, vehicles, and support elements through concise military intents. Leaders decide routes, formations, fireteam organization, and battle drills. The challenge is assigning achievable objectives to the right people with the right support—not manually steering every soldier.

### 2.2 Information is local

The player is not an omniscient cursor. Sightings belong to the people and sensors that observed them until the information travels through radios, signals, runners, relays, or physical contact. Plans can fail because the force is wrong, because its information is wrong, or because correct information cannot reach the person who needs it.

### 2.3 Logistics creates possibility

Food, fuel, ammunition, parts, medicine, transport, maintenance, and communications determine what an army can attempt. Logistics should generate operational choices and battlefield drama without becoming an office simulator. Distribution is automated until the player has a meaningful reason to intervene.

### 2.4 People make doctrine human

Characters are not interchangeable stat bundles. Leadership, trust, fear, training, fatigue, loyalty, prejudice, and moral limits change how an order is executed. The same intent can produce different behavior under different leaders without becoming arbitrary.

### 2.5 History survives unevenly

Battlefields retain wrecks, fortifications, graves, caches, hazards, and altered terrain. Formations accumulate names and traditions. Children inherit relationships they did not choose. Knowledge survives only through preserved people, records, tools, and facilities.

### 2.6 The surreal remains material

Achlydesa's anomalies are concrete, repeatable, exploitable, and costly. Archon systems can be studied and used, but never reduced to ordinary magic items. Their military benefits carry social, moral, political, and ontological consequences.

---

## 3. Player identity and command model

The player represents **the army as an enduring institution**, not a single avatar.

- A field commander is selected for each operation.
- The commander attaches to any chosen squad, which becomes the mobile command post.
- The commander's location determines personal risk, communications reach, decision speed, and access to firsthand information.
- The player directly issues intents to every friendly squad and independent support element in the operation.
- Platoons and task-organized groups provide communications, coordination, identity, and bulk-order tools; they do not remove units from player control.
- Repeated or copied orders reduce input burden during large battles.
- If a leader dies, the player may appoint a replacement only if the appointment can be communicated. Until then, local succession, personality, and doctrine govern behavior.

The player can freely divide the army into task forces combining squads, vehicles, reconnaissance, logistics, engineers, and support assets. A task force is an organizational and bulk-order tool, not an autonomous planning layer: every squad remains directly selectable and overrideable. Delegated forces continue under standing guidance on the same clock and interrupt only when a player-defined escalation threshold is crossed.

---

## 4. The unified theater

### 4.1 One world, one scale, one clock

The game takes place on one continuous, literal-scale theater approximately **1,000–1,500 kilometers across**. It contains portions of several surrounding regions rather than presuming a player-owned nation. Roads, tracks, settlements, ruins, waterways, bases, political claims, and anomalous jurisdictions all exist directly on the same terrain. There are no mission menus, encounter bubbles, separate tactical maps, or transitions into battle.

Time pauses while the player gives orders. Execution then advances every force, deadline, convoy, construction project, political event, and enemy plan to the earliest command-relevant decision. A window may cover seconds during close combat or days during uneventful movement.

Only a blocking decision that reaches an active command authority stops theater-wide execution. Routine reports wait for review. Before committing a long action, the player may add simple review conditions such as a waypoint, time, supply threshold, or changed contact.

There is no level scaling. Forces exist at real locations with persistent personnel, equipment, readiness, intelligence, and supply. The player may encounter an enemy too strong to fight, discover it too late, or deliberately attack its logistics instead.

The founding caravan is the coalition's initial physical circumstance, not a population-management minigame. It matters when its movement, protection, workshops, hospitals, depots, families, or evacuation constrain military action; routine internal organization remains abstracted.

At the beginning, reliable geographic knowledge follows the caravan's traveled route. The coalition extends it through reconnaissance, guides, trade, captured maps, local alliances, archives, and direct observation. Every source has an age and reliability; the world is not revealed merely because it lies inside the playable bounds.

### 4.1a The grid and height map

Use one authoritative, square spatial grid across the entire theater. As a starting engineering hypothesis, sample terrain at roughly two-meter spacing; validate this against cover fidelity, memory, pathfinding, and large-world precision before locking it. Units may move smoothly within that coordinate system. The grid determines shared spatial references rather than forcing soldiers to stand at cell centers.

Every location has elevation, slope, surface properties, passability, and persistent changes. Terrain height affects observation, exposed approaches, movement effort, radio obstruction, and firing geometry. Elevation grants useful geometry rather than a universal damage bonus.

A heightfield alone cannot represent a bridge above a road, a tunnel, or multiple floors. Add explicit traversable layers and connecting entrances at those same world coordinates. These are parts of the theater, not separate interior maps. Projectiles, observation, and movement must respect the actual layer and obstruction.

Zoom changes representation and selection detail. It never changes physical distance, weapon reach, travel time, or the existence of nearby forces. A theater overview can draw coarse symbols and routes while referencing precisely the same grid.

The scale is a serious engineering constraint. A 1,000 × 1,000 km region sampled every two meters contains 250 billion grid locations before adding layers. Do not allocate a dense mutable record for each location. Stream terrain chunks, store sparse changes, and use hierarchical pathfinding over the shared geometry. Distant quiet activity may be scheduled by events, but interacting forces must resolve consistently regardless of camera position. The prototype must establish whether this is practical before committing to the full region.

### 4.1b Distance, marches, and tempo

Keep geographic distance literal. Multi-day marches are normal between regional objectives; crossing the whole subcontinent should take much longer.

Illustrative tuning targets, not universal doctrinal rates:

| Situation | Example planning estimate | What changes it |
|---|---|---|
| A guarded foot column covers 90 km | About three to five days | Heat, load, water, security, fatigue, terrain, rests |
| A motorized convoy covers 250 km | About one to three days | Road quality, halts, maintenance, route security, fuel, crossings |
| Forces cross 1,000–1,500 km | A major expedition, potentially weeks | Transport, contested routes, supply arrangements, repairs, political access |

Calculate estimates from movement plus halts and support needs; do not impose a universal daily march allowance. An intact road can make some motorized journeys much faster. A single damaged crossing can make a shorter journey slower.

Travel is an operational commitment. A force sent to one district cannot instantly respond in another. Uncontested days advance quickly for the player; simulation time still passes for every faction, supply movement, and deadline.

### 4.2 Story urgency

Major events have visible or discoverable deadlines. Ignoring a crisis changes the theater rather than usually causing an immediate game over.

Possible consequences include:

- a settlement changing allegiance;
- a population being displaced;
- an Archon bargain being accepted by another faction;
- a bridge, gate, archive, or production site being lost;
- an enemy commander gaining experience or political authority;
- a future operation becoming harder, stranger, or unnecessary;
- a potential successor, teacher, engineer, or administrator never joining the coalition.

The story has a central spine, but the operational path through it is open.

### 4.3 Military control and political claims

Control has two distinct layers:

1. **Political claims** govern asserted law, recognition, recruitment rights, and obligations.
2. **Military control** reflects who can actually observe, patrol, supply, reinforce, and respond within an area.

A faction may claim a valley it cannot safely enter. A stateless coalition may dominate a road without claiming the towns along it. The interface shows overlapping gradients of practical reach and the route networks supporting them; political claims are a separate overlay. Control decays when forces leave according to local allies, garrisons, surveillance, supply access, and enemy pressure.

### 4.4 Optional statehood

The coalition begins stateless. It may remain a mobile coalition for the entire game.

Statehood becomes possible when the army sustains military control, accepts durable obligations to a population, and receives sufficient internal and external recognition. Founding a state changes available laws, recruitment, diplomacy, taxation, legitimacy, and responsibilities; it is not an automatic upgrade.

---

## 5. Military play on the continuous theater

### 5.1 Units and simulation scale

The normal controllable piece is a **five-to-eight-person squad** containing any mixture of generic and unique characters. Weapon crews, vehicle crews, medical teams, observers, couriers, and other specialist elements may be smaller.

A typical concentrated engagement may contain approximately **15–25 friendly squads and support elements**, but there is no fixed encounter roster or arbitrary army cap. Force size is limited by available people, trained leaders, equipment, instructors, bases, transport, supply, and political willingness. Rapid expansion dilutes leadership, training, cohesion, maintenance, and supply until institutions catch up.

Every soldier remains a persistent agent. Distant or unobserved forces use lower-frequency and event-driven simulation without receiving different rules. The camera zooms seamlessly from accurate formation symbols to visible individual soldiers.

The theater uses one authoritative **2.5D** geometry: a continuous heightfield with authored cliffs, bridges, walls, and discrete building floors. The isometric camera snaps among four cardinal angles. Buildings use cutaway roofs and layered floors in place, with authored entrances and structural breakpoints. Macro-geography, settlements, routes, Archon regions, and story sites are authored; procedural systems provide vegetation, clutter, minor damage, and local variation.

### 5.2 Continuous terrain

Movement occurs across continuous terrain. Squads snap contextually to useful cover, firing positions, formation anchors, doors, walls, trenches, and vehicle relationships.

For local movement, the player clicks a destination or target. The squad leader chooses:

- the complete route;
- formation and spacing;
- covered bounds or ordinary movement;
- local use of concealment;
- whether qualified subleaders temporarily split fireteams;
- the relevant battle drill.

The player never draws elaborate paths or micromanages individuals in local action. For long-distance movement, leaders propose an operational route. The player can accept it, draw a commanded centerline, or specify mandatory control points when the route matters. Leaders adapt exact local paths around terrain while preserving those constraints.

Forces may move anywhere physically passable, but roads strongly affect speed, fuel, wear, formation, and supply access. The player selects a march posture—rapid, balanced, guarded, or concealed. A reusable march template supplies order and intervals automatically; the player may override the order and interval of squads and vehicles when it has a meaningful military purpose. Custom and prebuilt march templates are reusable. Commanders adapt locally when terrain or traffic makes the formation impossible and report only material deviations. Reorganization happens physically and can cost time, cohesion, or security.

### 5.3 WEGO time

Orders are issued while theater time is paused and then executed simultaneously.

- Execution advances automatically to the earliest command-relevant decision, shortening to seconds near close contact and lengthening to days during safe movement or work.
- Orders cannot be rewritten during an execution window.
- Leaders handle immediate reactions according to training, doctrine, personality, and the standing intent.
- The player regains control at the next decision point.

There is no global combat switch and no boundary around an engagement. Different parts of the theater can be moving, observing, fighting, evacuating, building, or disengaging at the same time. On contact, leaders deploy from their actual march positions according to posture, doctrine, and standing intent; the player redirects them at the next decision point.

### 5.4 Persistent orders and decision points

An order remains active until it is:

- completed;
- invalidated by changed conditions;
- replaced by the player;
- judged no longer achievable by the responsible leader.

Leaders request attention at meaningful decision points rather than after every minor event. Alerts are visible, but the game does not automatically sort the player's priorities or jump the camera without consent.

After breaking contact, a squad with an uncompleted objective performs an **objective viability check**. It reassesses:

- casualties and available leadership;
- suppression, cohesion, and fatigue;
- ammunition and critical equipment;
- communications;
- current enemy disposition;
- route access;
- time remaining;
- doctrinal and moral constraints.

If the objective remains achievable, the standing order continues. If not, the leader follows applicable doctrine—holding, withdrawing, seeking support, or requesting a decision.

### 5.5 Core order vocabulary

The interface uses concise intent verbs rather than technical procedure:

- **Move** — reach a position using an appropriate route and formation.
- **Observe** — watch an area or target while managing concealment and signature.
- **Suppress** — restrict enemy observation, fire, or movement through credible threat.
- **Kill** — destroy a designated target using available methods and support.
- **Assault** — close with and take a position or defeat its occupants.
- **Hold** — retain a position or prevent enemy passage.
- **Support** — enable another element through fire, observation, transport, relay, protection, or specialist action.
- **Retreat** — physically disengage toward a viable route or destination.
- **Resupply** — transfer specified materiel through physical contact.
- **Evacuate** — stabilize and move casualties or other protected personnel.
- **Recover** — retrieve disabled vehicles, equipment, records, or bodies.

Each local squad order may include a target or destination and a few understandable constraints such as urgency, ammunition conservation, engagement threshold, or casualty tolerance. It does not become a route-planning form; manual theater centerlines and mandatory control points are a separate long-distance movement order.

### 5.5a An order should fit on one card

The default input is **unit + verb + place or target**. Standing doctrine supplies the normal constraints. Purpose, timing, support relationships, and exceptions appear only when relevant; every order must not become a questionnaire.

| Responsibility | Commanding officer | Subordinate execution |
|---|---|---|
| Position | Assign a hill, frontage, building, route, or holding area | Select covered positions, spacing, facing, and local movement |
| Target | Specify the target or effect and its priority | Allocate suitable weapons and coordinate local engagement |
| Timing | Set an arrival requirement, review time, or coordination condition | Estimate movement, halts, preparation, and local sequencing |
| Risk | Set preservation, urgency, disengagement, and protected-reserve constraints | Replan within those limits; report when they no longer suffice |
| Logistics | Choose sources, support sites, routes, escorts, and priorities | Schedule loads, convoys, transfers, refueling, and maintenance |
| Coordination | Assign supporting relationships and reserve authority | Exchange local reports and synchronize assigned support |

**Example:** “Hold the western escarpment until the caravan clears the basin. Preserve the squad; fall back to the marked rally area if holding becomes untenable. The support team has priority for your requests.”

The squad leader chooses positions, organizes observation, manages local movement and fire, and uses the approved fallback. The player returns to the decision when the caravan is delayed, the position becomes infeasible, or another objective competes for the same support.

Before commitment, show a brief plan preview: likely corridor or position, estimated completion, required support, known risks, and one sentence of reasoning. Avoid making the player approve every adaptation. Afterward, explain material deviations using the information the leader actually possessed.

### 5.6 Assault interpretation

An assault order designates the objective and command intent. The squad leader selects the approach, timing, formation, temporary fireteam organization, use of smoke or grenades, and battle drill.

The leader may abort or pause the assault if conditions become unacceptable. What counts as unacceptable depends on doctrine, competence, personality, known support, confidence in the information, and the commander's stated constraints.

---

## 6. Tactical contact loop

Contact is treated as a developing problem rather than a binary state.

### 6.1 Find

A person or sensor detects evidence: movement, dust, sound, heat, tracks, radio traffic, muzzle flash, a drone, disturbed civilians, an anomalous response, or a previously reported position.

The initial contact may be incomplete, stale, misidentified, or deliberately deceptive.

### 6.2 Develop

The force observes, probes, repositions, checks another sensor, communicates the report, or decides not to reveal itself. Reconnaissance is valuable because it discovers strengths, gaps, routes, signatures, and intentions—not merely because it removes fog from the map.

### 6.3 Fix

Suppression, threatened maneuver, obstacles, deception, isolation, or loss of communications limits the enemy's safe choices. Fixing is temporary and resource-intensive. It creates opportunity; it does not automatically cause casualties.

### 6.4 Maneuver

Other elements exploit an exposed flank, blind area, blocked route, poor field of fire, loss of leadership, or divided attention. Movement and fire become mutually supporting.

### 6.5 Decide

The commander chooses whether to assault, destroy from range, compel surrender, bypass, continue observing, or disengage. Destruction is only one useful outcome.

### 6.6 Exploit or consolidate

The force may pursue, seize terrain, capture personnel, collect intelligence, recover casualties, redistribute ammunition, repair communications, establish security, or continue toward the original objective.

---

## 7. Information, communications, and the electromagnetic battlefield

### 7.1 Local knowledge

A sighting belongs to its observer until communicated. Reports carry:

- source;
- observation time;
- estimated identity and strength;
- confidence;
- last known location or area;
- communication delay and possible distortion.

Different friendly elements may hold contradictory pictures of the same theater.

When the player pans to a disconnected distant force, the view shows terrain and the last reported command-picture state. Current local action remains hidden until a report, restored link, messenger, or survivor carries it back.

### 7.2 Loss of communications

A disconnected element cannot receive new orders. It continues its last intent, applies standing doctrine, and makes local survival decisions until contact is restored.

Communications may be restored through:

- movement back into radio coverage;
- relay drones or vehicles;
- elevated antennas;
- runners or signal teams;
- repair or replacement;
- capture or activation of unusual infrastructure;
- deliberate use of an Archon-derived system.

### 7.3 Emission and exposure

The electromagnetic spectrum functions like another layer of terrain.

- Stronger and more frequent transmissions improve coordination but increase the chance of interception, location, jamming, deception, or attack.
- Radio silence reduces exposure while slowing orders and preventing automatic sharing.
- Active sensors reveal more than passive sensors and produce more detectable signatures.
- Jamming can affect friendly systems as well as enemy ones.
- Terrain, weather, power, antenna position, damage, training, and strange regional laws affect propagation.

The game exposes consequences and probabilities rather than requiring the player to manage frequencies.

### 7.4 Electronic-warfare intents

EW elements use four primary intents:

- **Listen** — detect, classify, and possibly locate emissions.
- **Jam** — degrade selected enemy communications, navigation, sensors, or drone control within an area.
- **Deceive** — create real transmissions, emitters, decoys, or control inputs that an enemy may misclassify; it never inserts imaginary evidence into a command picture.
- **Protect** — monitor friendly emissions, reduce interference, and support resilient communications.

An element cannot perform every function equally at once. Emitting a strong effect creates a strong signature.

### 7.5 Drones and aerial systems

The provisional baseline is:

- small drones, balloons, improvised aerial systems, and relay platforms are common;
- crewed aircraft are rare strategic assets;
- Heliarchic weather and sensing make sustained conventional air superiority exceptionally difficult;
- drones require operators, batteries or fuel, maintenance, links, payloads, and replacement airframes;
- drones may observe, track, relay, strike, screen, or deceive;
- armed autonomous functions require authorization and remain vulnerable to uncertain identification, interference, capture, spoofing, and doctrinal restriction.

### 7.6 Sensor-to-shooter chain

Long-range effects require a functioning chain:

1. detect;
2. identify;
3. communicate;
4. authorize;
5. engage;
6. assess.

Breaking any link can defeat the attack without destroying the weapon. A report may also become too stale for the shooter to use confidently.

### 7.7 Layered air defense

Air defense combines:

- camouflage, concealment, dispersion, and movement;
- warning and passive detection;
- emission discipline and deception;
- electronic attack;
- small arms, guns, missiles, and specialized interceptors;
- attacks on launch teams, operators, relays, repair facilities, and supply.

Defensive ammunition and power are finite. The player must decide what deserves protection and which layer should engage which threat.

---

## 8. Combat model

### 8.1 Transparent uncertainty

The interface displays exact calculations from confirmed information, including hit, casualty, and suppression probabilities.

It does **not** reveal facts the force does not know. Unknown factors appear as explicit risk warnings, such as:

- unobserved route segment;
- target position older than three minutes;
- armor type unconfirmed;
- possible hidden firing positions;
- uncertain jamming source;
- unidentified anomalous effect.

The displayed number is exact about known inputs, not secretly omniscient.

### 8.2 Individual lethality and squad state

Every projectile or other lethal effect can wound or kill any character, including unique and story-important characters.

At squad level, combat tracks:

- suppression;
- cohesion;
- fatigue;
- confidence in leadership;
- ammunition and weapon availability;
- position and cover;
- individual wounds, incapacitation, death, and absence.

Suppression measures the immediate inability or unwillingness to expose oneself and act effectively under credible threat. Cohesion measures the squad's ability to continue acting together as an organized group. Neither is a substitute health bar.

### 8.3 Fear and survival

When fear or suppression overwhelms a squad, individuals seek survival automatically. Leadership, training, personality, trust, relationships, doctrine, perceived escape routes, and enemy behavior determine whether the group:

- rallies;
- freezes;
- disperses;
- withdraws;
- refuses an order;
- surrenders;
- continues despite extreme risk.

### 8.4 Disobedience

Characters normally obey and express personality within the method of execution. A command that severely violates a character's beliefs, loyalties, attachments, or understanding of the situation may be delayed, modified, refused, or actively opposed.

This is rare, legible, and rooted in established character state. It is not random drama.

---

## 9. Squads, leaders, and characters

### 9.1 Free-form composition

The player may freely compose five-to-eight-person squads. There are no mandatory fantasy classes. A squad's capability emerges from:

- personnel and skills;
- leaders and qualified subleaders;
- weapons and ammunition;
- tools and specialist equipment;
- communications;
- carried weight;
- transport;
- redundancy;
- familiarity and cohesion.

Powerful compositions create corresponding logistical and command burdens.

### 9.2 Fireteam splitting

Squads remain unified unless a qualified leader or subleader issues a temporary split as part of a battle drill. The local leader manages the split and reunion. The player commands the squad's intent, not each temporary team.

### 9.3 Generic and unique characters

Every generic soldier has:

- a name;
- a portrait;
- an origin and basic history;
- skills and aptitudes;
- personality tendencies;
- affinities and relationships;
- wounds, service record, and formation history.

Notable survivors can develop into full unique characters through events, deeds, relationships, promotion, injury, capture, or unusual transformation.

### 9.4 Development

- Existing skills improve through relevant use.
- Entirely new skills require training, instructors, equipment, or exceptional events.
- Major traits emerge through relationships and consequential experiences.
- There are no abstract character levels.

### 9.5 Directional affinity

The game tracks a numerical affinity from every character toward every other character. Affinity is directional: one person may love, trust, fear, resent, or admire someone who feels differently.

The matrix updates automatically through:

- shared service;
- leadership and obedience;
- rescue, abandonment, injury, and death;
- compatible or conflicting values;
- proximity and assignment;
- family and mentorship;
- political and moral decisions.

Relationship tags interpret important patterns without replacing the underlying directional values.

### 9.6 Formation identity

Squads and larger formations accumulate:

- names and insignia;
- battle history;
- reputation;
- cohesion and shared experience;
- informal practices;
- doctrinal habits;
- relationships with settlements and enemy formations.

If every member is lost, the formation may be retired or deliberately reconstituted. A reconstituted formation inherits reputation and disputed traditions, not the cohesion of the dead.

---

## 10. Vehicles, fires, engineering, and support

### 10.1 Vehicles

Each vehicle is directly controlled and crewed by roster characters. Vehicles may transport squads.

Damage is component-based. Mobility, weapons, sensors, communications, protection, power, crew stations, and cargo may fail independently. Surviving vehicles require physical recovery, parts, skilled labor, facilities, and time.

### 10.2 Indirect fire

- Mortars, guns, launchers, aircraft, and their crews all occupy physical positions on the continuous theater.
- A firing unit may be outside the current view or far beyond the local contact, but it is never placeless or invulnerable.
- Fire requests require observation, communications, authorization, available weapons, suitable ammunition, and deconfliction.
- Counterfire, displacement, concealment, and ammunition expenditure matter.

### 10.3 Battlefield engineering

Engineers breach, clear, fortify, demolish, recover, repair, construct crossings, prepare positions, and alter access routes. Their work persists on the terrain.

Engineering projects are directed by objective and constraints. Engineers propose a design with explicit costs, time, risks, resource requirements, and tradeoffs rather than making the player place every technical component.

### 10.4 Medical evacuation

Incapacitated personnel must be:

1. reached;
2. stabilized;
3. physically carried or transported;
4. delivered to an aid or evacuation point;
5. moved through the wider medical network if further treatment is possible.

Treatment consumes time, personnel, medicine, transport, and safe routes. Recovering a casualty may endanger more people; abandoning one has persistent relationship, morale, political, and doctrinal consequences.

### 10.5 Prisoners

Prisoners require guards, transport, food, medicine, and secure accommodation. They may be questioned, exchanged, recruited, tried, transferred, released, or killed through player or subordinate action. Their treatment affects intelligence, enemy behavior, diplomacy, relationships, doctrine, and later generations.

### 10.6 Withdrawal

Withdrawal is a physical operation, not a menu exit. Units disengage along available routes, carry what and whom they can, abandon what they cannot, and use rear guards, smoke, obstacles, deception, and supporting fire to escape.

---

## 11. Physical logistics

### 11.1 Carried loads

The game tracks:

- magazines;
- belts;
- shells;
- rockets and missiles;
- grenades and specialist munitions;
- batteries and power;
- vehicle fuel;
- medical supplies;
- critical tools and spare components.

Automatic loadout presets prevent repetitive inventory work. Presets can be modified and assigned by squad, formation, mission type, or doctrine.

### 11.2 Physical resupply

Supplies do not teleport. Squads replenish from physical personnel, vehicles, dumps, bases, settlements, captured stocks, or prepared caches.

Resupply assets can be delayed, misrouted, ambushed, destroyed, captured, exhausted, or cut off from the units they support.

### 11.3 Theater stocks

The coalition summarizes five primary military stock groups:

- food;
- fuel;
- ammunition;
- parts;
- medicine.

Distribution follows automated routes and priorities. The player intervenes by changing priorities, creating caches, assigning escorts, opening routes, accepting shortages, or redesigning the network.

Water belongs to the food/provisions summary but is tracked physically and displayed separately whenever it constrains desert operations. Ammunition, batteries, fuel, medicine, and parts retain compatibility internally; the five summaries are not interchangeable currencies.

The system should emphasize bottlenecks, vulnerability, distance, throughput, and recovery rather than purchase-order administration.

A task force's top-level logistics display summarizes projected endurance while marching, waiting, or fighting, with exact stocks available on inspection. The player chooses sources, routes, priorities, and protected reserves; logistics leaders schedule physical convoys and transfers. If a route is cut, the force continues from carried stocks and reports when the projected shortfall threatens its standing intent.

---

## 12. Operations without a mode transition

The player identifies an objective, organizes forces, draws routes, establishes support, and begins movement directly on the world. Story and factions create pressures, opportunities, and deadlines rather than offering a mission list. Task-force and group orders let the player command at scale, while every squad remains directly selectable.

Reconnaissance elements receive an intent—observe, screen, trail, probe, or confirm—and an area or route. Their leaders choose the detailed method. Uncertain enemy forces appear as timestamped probable corridors or areas shaped by terrain, routes, plausible speed, and observed behavior. An undetected scout continues to observe, evade, trail, or report according to its standing intent rather than forcing a separate encounter.

Reconnaissance, movement, construction, resupply, rest, and rehearsal consume overlapping theater time and create observable signatures. Forces physically occupy assembly areas, attack positions, firing positions, holding areas, and entry routes; detection, delay, traffic, or route blockage can disrupt preparation.

Reserves wait in real locations with real access routes and may be committed manually or by a narrow pre-authorization. Multiple forces coordinate through shared times or observable triggers, so lost communications, delay, and local judgment may desynchronize an operation.

All contacts share the same theater clock. The player may move the camera between simultaneous actions at decision points or leave a force under standing guidance; a distant firefight does not freeze marching columns, construction, or enemy plans elsewhere.

Unexpected contact preserves actual march order, spacing, posture, readiness, and cargo arrangement. There is no deployment screen that rearranges a force after contact.

Operation assessment is graded. Success may be costly, incomplete, temporary, politically disastrous, or strategically decisive. Failure may preserve the army, reveal intelligence, buy time, or save a population.

Only clearly declared critical conditions force a reload. Otherwise, outcomes persist.

---

## 13. Persistent terrain, settlements, and bases

There are no disposable battlefields. Every fight alters the same persistent theater terrain.

The following persist:

- craters, fires, flooding, rubble, and damaged routes;
- trenches, walls, emplacements, observation posts, and obstacles;
- mines and unexploded ordnance;
- wrecks and recoverable equipment;
- caches and abandoned supplies;
- graves and memorials;
- contamination and anomalous hazards;
- discovered passages and modified structures.

Major bases occupy actual terrain. Advanced facilities must be recovered and restored; ordinary field works and shelters use available materials. The player designates a site, its intended function, access, protection priorities, and resource allowance. Engineers handle routine layout and work sequencing. Detailed placement remains an optional military decision when dispersal, concealment, fields of fire, or route access matters.

A base is simultaneously:

- an operational node;
- a storage, recovery, and repair site;
- a home for people and records;
- a political claim;
- a place that can be approached, infiltrated, bombarded, defended, abandoned, or retaken without loading a separate map.

---

## 14. Settlements, recruitment, and administration

Settlements track:

- population;
- production;
- loyalty and political alignment;
- recruitable personnel;
- displacement;
- essential military stocks and infrastructure.

The ordinary civilian population is represented as a settlement-level number. Persistent individual NPCs are generated when a role, relationship, office, specialty, or event makes a particular person matter.

The player makes occasional high-impact decisions about obligations, conscription, protection, justice, requisition, Archon bargains, refugee policy, and military restoration priorities. Appointed characters handle routine administration. Settlement production represents background subsistence and existing civilian activity, not a player-operated factory economy. No tax sliders, civilian job allocation, building chains, or recurring administrative reports are required.

An administrator leaves ordinary field deployment but can command the settlement's defense. Their competence, beliefs, relationships, and local reputation affect implementation.

Recruitment policy is set by settlement. Administrators implement the policy and present notable candidates. The player assigns replacements to damaged squads. Unfamiliar replacements reduce cohesion until training and shared service integrate them.

---

## 15. Doctrine

Doctrine begins as observed behavior.

Repeated player choices create habits concerning:

- initiative under lost communications;
- ammunition conservation;
- casualty tolerance;
- civilian protection;
- treatment of prisoners;
- reliance on reconnaissance;
- concentration versus dispersion;
- willingness to use Archon systems;
- withdrawal and recovery;
- command centralization;
- tempo after contact.

The player may later formalize, reject, or revise these habits. Formal doctrine influences leader decisions, training, loadouts, planning estimates, and institutional identity.

Doctrine is primarily army-wide with a few task-force-level overrides. Individual leaders still interpret it.

After the generational catastrophe, descendant cells inherit incomplete, altered, or mythologized versions of the original doctrine. Reuniting cells can produce conflict over which tradition is authentic.

---

## 16. Equipment and technological development

**No technology research, invention tree, or advanced equipment production.** Humanity fights with what remains and what it can restore. This rule supersedes the earlier draft's provisions for research programs and relic reproduction.

Progression is primarily horizontal: the force gains access to surviving capabilities and becomes more capable of supporting them. A recovered weapon may be formidable but useless without its operator, compatible ammunition, working power system, and replacement components.

### 16.1 Local equipment

Rugged equipment is relatively understandable and repairable. Crews patch it, replace worn components from available stocks, and cannibalize compatible machines. Field fortifications and ordinary shelters are possible; construction does not create an industrial upgrade economy.

### 16.2 Relic technology

Relics are existing artifacts, never unlockable product lines. Recovery requires access, transport, competent personnel, compatible parts, and a suitable workshop. An archive may explain how to operate or repair an existing device; reading it does not grant the ability to manufacture another.

Restoration may recover a lost function, exchange a surviving module, or reduce unreliability. It cannot invent a higher technological tier. Cannibalization is a real trade: repairing one vehicle may permanently deprive another of a scarce component.

### 16.3 Archon and Soterion systems

These are surviving systems with existing capabilities. Gaining access can alter transit, sensing, healing, power, or command, with obligations and consequences attached. Engineers may restore a connection or operating state, but cannot turn the system into a general-purpose research laboratory.

Ordinary people remain human. Exceptional capabilities come from equipment, environment, institutions, or rare transformations rather than routine superhero progression.

### 16.4 Restoration orders

The player chooses a capability to restore and its priority. Engineers estimate:

- usable condition and recoverable functions;
- required people, donor equipment, parts, and facilities;
- time, transport, power, and supply dependencies;
- likely reliability and unresolved hazards;
- what other repairs must be deferred.

Routine inspection and maintenance then run automatically. The command decision is whether the recovered capability justifies its opportunity cost.

**Consumable boundary:** food and water remain renewable through surviving civilian activity. The provisional strict reading for military technology is finite ammunition and manufactured spares, recoverable fuel stocks, and rechargeable existing power stores where functioning energy infrastructure survives. No new advanced vehicles, weapons, electronics, or research. Whether restored old facilities may replenish mundane ammunition or fuel is a remaining scope decision; the baseline must work without that permission. Campaign duration and expenditure must be balanced against this finite inventory rather than relying on unexplained replenishment.
---

## 17. Enemy simulation

Enemy forces follow the same logistics, communications, casualty, recovery, information, and character rules as the player unless a visible faction-specific exception says otherwise.

Enemy factions differ through:

- institutions and political goals;
- doctrine and training;
- access to terrain and infrastructure;
- recruitment and tolerance for loss;
- Archon relationships;
- equipment and supply systems;
- genuine anomalous capabilities.

Named enemy commanders persist. They can be killed, wounded, captured, exchanged, recruited, disgraced, promoted, transformed, form relationships, and leave successors.

Enemy doctrine adapts only to tactics the faction has observed, survived, recovered evidence of, or learned through intelligence. The AI does not counter plans it could not know.

Enemy commanders pursue objectives with their own intelligence, routes, logistics, reserves, doctrine, politics, and competing obligations. Forces are never spawned or scaled merely to counter the player. A major power devotes attention to the coalition according to observed interference, threat to current objectives, expected cost, internal politics, and other wars—not a global notoriety meter.

The coalition initially can defeat isolated raiders but cannot defeat a sophisticated army in open warfare. That imbalance is a starting condition, not a permanent game mode. Growth emerges through recruitment, training, command depth, equipment, restoration capacity, bases, logistics, and political support. Captured infrastructure contributes only after it is physically secured, staffed, repaired, supplied, and connected to the coalition's network.

Stronger factions cannot concentrate without cost. Competing wars, security obligations, incomplete intelligence, difficult terrain, long supply lines, political divisions, and conflicting commanders all consume attention and forces. Irregular resistance therefore uses the same persistent squads, leaders, caches, safe sites, contacts, and supply relationships as every other formation. Suppressing it requires real reconnaissance, patrols, route security, raids, garrisons, and protection duties that reduce strength available elsewhere.

Local civilian support can provide information, concealment, guides, recruits, food, medical aid, transport, and warning. It is represented through settlement-level support and persistent contacts rather than an abstract insurgency score.

Military defeat is dynamic. A faction ends only through an official surrender or when no recognized command structure or organized armed formation remains. Individual survivors may continue to exist. Commanders who reject a surrender become continuing belligerents or successor factions. The player may reject or violate a surrender, but witnesses, relationships, recruitment, diplomacy, and future willingness to surrender respond.

---

## 18. The two-generation campaign

### 18.0 The caravan opening — proposed playable premise

The player begins as the caravan's guard commander, with a few understrength squads, working transports, and a small repair capability. They are competent hired guards with obligations to their passengers and employers. The army grows out of the people and resources that survive.

At an old Soterion transit checkpoint, a caravan technician uses a surviving maintenance credential to restore passage. A dormant connection answers from a buried regional junction. The checkpoint reclassifies the caravan as requisitionable continuity material and orders its people and equipment detained.

A human enforcement detachment serving Strategos arrives to collect them. Negotiation can reveal the demand and buy time, but the detachment will not accept the caravan's independent departure. The guards must resist seizure, protect an escape, or break out after partial capture. They fight a local force and checkpoint defenses; they do not defeat the Archon in the tutorial.

The resulting self-defense is recorded as interference with Strategos's protected infrastructure. Surviving reports and the checkpoint's transmissions carry that classification onward. Pursuit comes from actual enforcement forces, with finite knowledge, routes, and competing assignments.

The caravan retains evidence of the junction's return: recovered maintenance records, an operator's testimony, or a partial transmission. No single invulnerable person or unique object must survive for the story to continue. The junction existed before the caravan arrived; the incident makes its renewed reach visible.

The next settlement is dependent on the same route and receives an order to surrender the fugitives. That creates the first broader choice: secure temporary shelter, expose the seizure order, negotiate with a rival patron, or leave before retaliation. The coalition forms through shared exposure and practical agreements. Joining a revolutionary cause is a possible development, not an assumed opening personality.

### 18.0a What the junction actually controls

The proposed junction is a surviving regional coordination node of the Soterion. Its immediate functions are bounded:

- route authorization for specific surviving transit systems;
- access and scheduling for connected power, storage, and repair facilities;
- communication between otherwise incompatible surviving networks;
- old administrative and enforcement instructions accepted by particular machines.

It does not create supplies, control every relic, or command a whole continent by possession alone. A claimant needs physical access, usable interfaces, surviving operators and records, secure communications, power, and functioning endpoints. Capturing its entrance grants none of those automatically.

Its value is concentrated interoperability. A faction that restores enough links can sustain and coordinate existing forces across distances its competitors cannot. The source of that advantage is also the danger: connecting people and institutions gives the old sovereign new means to classify and incorporate them.

The game distinguishes **holding the site**, **operating a limited service**, and **submitting a force to integration**. Those states have different requirements and consequences. Control can be divided, contested, sabotaged, or relinquished.

### 18.0b Factions contesting it — proposed organizations

These organizations are additions for the game, not names already established in the world bible. The Archons' existing forms and offices remain authoritative.

| Faction | What it wants from the junction | Military character | Source of dependence or tension |
|---|---|---|---|
| Strategos's Custodians | Restore an enforcement jurisdiction and neutralize unauthorized access | Automated defenses with human detachments, strong local protection, escalating threat classification | Its own protection system can reject ceasefire signals; people within it may still negotiate |
| Heliarchic Mandate | Bind surviving energy and sensing links to the Heliarch's authority | Surveillance-supported forces, control of exposed routes, scarce powerful assets | Weather and the Archon's movements complicate its own plans; services sustain dependent towns |
| Gate Compact of Pylaios | Control authenticated passage between surviving transport endpoints | Mobile escorts, route garrisons, transport leverage | Gate capacity, access, and the human cost of transmission limit apparent mobility |
| Autophagan Delegation | Incorporate junction services into the moving city's support relationships | Cohesive forces organized around the city and its needs | The army's home is itself an Archon's body; civilian and military survival are entangled |
| Free Cistern League | Keep water, roads, and restoration sites usable without a single master | Local defense forces, guides, repair crews, uneven interoperability | Towns disagree over obligations and how much central command to accept |

Anodyne, Aletheia, and Mneme remain important powers through medical bargains, surveillance arrangements, and access to surviving records. Their clients can support or divide the main contenders. Only three or four Archons are physically present across the vast theater; influence does not require crowding every monster onto the playable map.

Factions pursue practical regional goals even without the player. A claimant may seek a transit agreement, an operator, or a power connection before attempting the junction itself. Rivalries and commitments explain why the starting escort survives between stronger forces.

### 18.0c An open war with a story spine

The first campaign moves from **survival**, through **coalition-building and contested access**, to **the junction operation**. These are changes in strategic circumstances, not isolated mission maps. Routes, allies, losses, occupied locations, and time spent determine how the coalition reaches each stage.

A deadline has an in-world cause: an approaching detachment, an agreed handover, a failing facility, or an opponent's repair progress. Missing it changes ownership or available options. Factions physically prepare operations and can succeed while the player is elsewhere.

The retained two-generation design includes an unavoidable rupture at the junction. That is an explicit boundary on the sandbox: the world is open in how the coalition acts and what it saves, while the central catastrophe is authored. The story must not pretend the player can permanently prevent it. If the coalition rejects activation, another prepared claimant can precipitate the rupture; the player then shapes containment and evacuation rather than personally authorizing integration.

The junction does not mind-control the entire roster in a cutscene. The catastrophe follows connected infrastructure, transmitted records, and established dependencies. First-generation precautions materially change who is exposed, which links can be severed, and what survives.

### 18.1 First generation

The coalition's route crosses a subcontinental theater in which **three or four Archons are physically present and widely separated**. Other Archons exert military, political, ecological, or infrastructural influence from beyond the playable region. The seven powers most important to the story are:

- **Autophagos** — mobile city-state, metabolism, and sovereign ecology;
- **Heliarch** — flying inverted whale, weather, energy, sensing, and oil;
- **Pylaios** — gates, transport, replication, and continuity;
- **Anodyne** — healing through transferred suffering;
- **Aletheia** — surveillance and compulsory exposure;
- **Strategos** — autonomous defense and escalation;
- **Mneme** — the black river of memory and archives.

A buried Soterion coordination junction begins operating again. Its early effects promise extraordinary command, transport, logistics, and interoperability. The wider war intensifies because major forces want to possess, suppress, complete, bargain over, or destroy it, though their interest and ability to intervene vary across the huge theater.

The coalition's apparent victories teach the junction its organization. Communications, doctrine, personnel records, relationships, logistics, and command procedures gradually become legible as missing Soterion components.

### 18.2 The catastrophe

The midpoint is an inevitable, large playable operation.

In the central false-victory path, the coalition activates the junction believing it has secured independent command. Its connected command network is recognized as part of the Soterion. Integration begins physically and administratively. Alternative approaches change who initiates activation and the coalition's exposure, while preserving the established rupture.

The player cannot prevent the rupture. The operation instead asks impossible questions about what to save:

- sever one relay and abandon the forces beyond it;
- preserve people or records;
- evacuate children or equipment;
- destroy a facility or risk its capture;
- maintain communications and expose a cell to integration;
- save a commander, engineer, partner, prisoner, settlement, or archive at another's expense.

First-half choices determine individual fates and what survives. Most veterans disappear through a mixture of confirmed death, unresolved disappearance, captivity, and transformation.

### 18.3 Twenty years later

The second campaign begins roughly twenty years later.

Its starting nucleus is determined by the first generation's affinity network and surviving infrastructure. Possible successors include:

- biological children;
- adoptees;
- apprentices;
- engineered descendants.

Upbringing is simulated deterministically from relationships, guardianship, shared history, location, institutions, and catastrophe outcomes. The player does not select childhood bonuses from a menu. Their influence occurred through first-generation assignments and affinities.

The new coalition begins with fragmented cells, caches, a few damaged bases, partial records, disputed doctrines, and inherited enemies. Its primary drive is to rebuild the coalition and prevent any faction or Archon from completing the junction's integration.

Surviving first-generation characters are exceptional presences rather than the default roster. Some may return as prisoners, altered beings, hidden patrons, enemies, or unresolved mysteries.

---

## 19. Archon interaction

The player may bargain with, exploit, oppose, or eventually kill an Archon.

An Archon is not a conventional boss or source of loot. It is:

- a regional military system;
- an ecology and economy;
- a political order;
- a population's means of survival;
- an organ of the unfinished Soterion;
- an anchor for a particular form of foreign authority.

Killing one requires a multi-operation campaign that understands and exploits the appetite expressed by its form. The death permanently alters regional systems and may remove services on which civilians, armies, or reality itself depend.

Archon bargains create capabilities with continuing obligations. Their costs should remain visible in ordinary logistics, relationships, settlements, and bodies rather than appearing only in cutscenes.

---

## 20. Narrative resilience, death, and failure

Any lethal effect can permanently kill any deployed character.

The story is written to absorb deaths through institutional roles, successor characters, enemy continuity, altered operations, records, and relationships. It should not depend on a large immortal cast.

A small number of explicitly communicated critical conditions may require reload, including the death of a currently indispensable story figure. These conditions are exceptional and visible before the risk is taken.

Most outcomes use graded assessment and persistent consequence rather than binary victory and defeat.

---

## 21. Interface and anti-bureaucracy rules

The interface should surface decisions, not reproduce staff work.

### Automate

- routine distribution along established routes;
- loadout presets;
- route selection and formation;
- fireteam battle drills;
- ordinary settlement administration;
- affinity updates;
- training schedules once authorized;
- standard maintenance queues;
- repeated and copied squad intents.

### Keep under player control

- objectives and priorities;
- task-force composition;
- operational routes and timing;
- allocation of scarce support;
- risk, casualty, and engagement constraints;
- appointments;
- doctrine;
- high-impact political choices;
- whether to exploit Archon systems;
- which alerts deserve attention.

### Required legibility

- show why a unit chose its method;
- show what information it possessed;
- show which leader, doctrine, trait, or relationship affected behavior;
- distinguish confirmed facts from estimates and unknown risks;
- show calculated effects conditional on known inputs, with time and outcome estimates clearly distinguished from guarantees;
- make physical supply and communications paths inspectable;
- explain why an objective became unachievable.

The goal is complexity in the simulation and clarity in command.

---

## 22. Research basis

The design abstracts rather than reproduces modern doctrine, but its foundational relationships come from real military concepts:

- combined arms: capabilities create reinforcing dilemmas rather than operating as isolated damage types;
- mission command: commanders communicate intent while trained subordinates choose methods within constraints;
- reconnaissance pull: information about strength, gaps, routes, and disposition shapes maneuver;
- contested logistics: sustainment determines operational reach and is itself vulnerable to attack;
- local and degraded command: communications can fail, forcing subordinate initiative and pre-established doctrine;
- suppression and maneuver: fire creates temporary freedom of movement rather than merely reducing health;
- layered protection: concealment, dispersion, warning, deception, electronic effects, and weapons work together;
- sensor-to-shooter chains: detection, identification, communication, authorization, engagement, and assessment can each be disrupted;
- physical casualty evacuation, recovery, withdrawal, and prisoner handling impose real opportunity costs.

Primary references consulted include:

- [U.S. Army FM 3-90, Tactics](https://armypubs.army.mil/epubs/DR_pubs/DR_a/ARN38160-FM_3-90-000-WEB-1.pdf)
- [U.S. Marine Corps MCDP 1, Warfighting](https://www.marines.mil/portals/1/publications/mcdp%201%20warfighting.pdf)
- [U.S. Marine Corps MCDP 1-3, Tactics](https://www.marines.mil/Portals/1/Publications/MCDP%201-3%20Tactics.PDF)
- [U.S. Marine Corps MCDP 4, Logistics](https://www.marines.mil/News/Publications/MCPEL/Electronic-Library-Display/Article/899840/mcdp-4/)
- [U.S. Army: SIGINT and EW for Tactical Leaders](https://www.army.mil/article-amp/286341/harnessing_sigint_and_ew_for_tactical_dominance_a_guide_for_combat_arms_leaders)
- [U.S. Army Aviation Digest, Winter 2026](https://home.army.mil/rucker/9017/7367/5561/AD_WINTER_PROOF_FINAL_3-9-2026.pdf)

These references inform relationships and tradeoffs. They do not dictate factions, terminology, force structures, or balance.

---

### 22.1 Sources checked for this revision

The references below support particular principles. The game rules, interface, fictional logistics, factions, and story are design proposals, not claims that doctrine prescribes a game.

| Source | Supported foundation | Application proposed here |
|---|---|---|
| [Army University Press, Mission Command, May 2020](https://www.armyupress.army.mil/Journals/NCO-Journal/Archives/2020/May/Mission-Command/), explaining ADP 6-0 (2019) | Competence, trust, shared understanding, intent, mission orders, initiative, and risk acceptance underpin decentralized execution | Orders specify outcomes and constraints; competent subordinates choose methods |
| [USMC announcement of MCDP 4, Logistics, 21 March 2023](https://www.marines.mil/News/Messages/Messages-Display/Article/3335876/availability-of-marine-corps-doctrinal-publication-4-logistics/) | Logistics planning shapes endurance, reach, and survivability | Endurance forecasts and physical support routes constrain operations |
| [FM 3-90, Tactics, May 2023](https://rdl.train.army.mil/catalog-ws/view/100.ATSC/17614720-DF1D-40BE-9123-F80680BF3974-1274406509298/fm3_90.pdf) and [Army combined-arms discussion, September 2025](https://www.army.mil/article-amp/287758/breaking_through_combined_arms_maneuver_against_prepared_defenses) | Coordinated capabilities and supporting effects enable maneuver | Leaders coordinate assigned support rather than acting as isolated damage sources |
| [Neufeld, Mostaghim, and Brand, 2018: A Hybrid Approach to Planning and Execution in Dynamic Environments Through Hierarchical Task Networks and Behavior Trees](https://cdn.aaai.org/ojs/13044/13044-52-16561-1-2-20201228.pdf) | A hierarchical planner can coordinate larger tasks while reactive behaviors handle local changes | Use authored intent decomposition with local execution and escalation rules |

The planning paper was evaluated in a small experimental game environment. It supports an architectural starting point, not a proven solution for this game's scale or a guarantee of military competence. Its authors explicitly identify larger search spaces and agent counts as further work. The full doctrinal PDFs were not all accessible during this revision; the claims above are limited to retrieved official text and the research paper.

### 22.2 Prototype the delegation before the continent

The first playable should contain a **continuous 20–30 km corridor** with a caravan route, checkpoint, settlement, repair site, alternative crossing, and terrain that changes observation and movement. Begin with four to six squads plus transport and support; scale toward the established 15–25-element engagement after the command loop works.

Play through travel, the checkpoint incident, disengagement, a blocked supply route, rest, and another operation in the same persistent space. Add a second simultaneous force to prove that the shared clock and reporting rules work away from the camera.

The essential acceptance checks are:

- A squad completes a feasible order without repeated corrective clicks or individual control.
- After a local obstacle or brief firefight, it resumes the standing objective when that remains achievable.
- A leader explains a material deviation in terms of its own information and constraints.
- An impossible order produces a useful request or authorized fallback, rather than silent suicide or endless idling.
- Supply interruption changes endurance and options without requiring manual truck scheduling.
- Two separated forces continue on the same clock; a disconnected engagement leaks no hidden information through a pause.
- Switching camera position cannot change the rules or outcome of the same interaction.
- A broad operation remains manageable through persistent and bulk orders; adding squads does not add recurring clerical chores.

The main development risk is trustworthy subordinate behavior. A spectacular continent cannot compensate for an AI that requires constant correction. Defer full generational simulation, all seven Archon systems, and continent-scale content until this slice works.

---

## 23. Provisional assumptions

The following are coherent enough for the high-level design but remain deliberately easy to revise:

- small drones and improvised aerial systems are common while crewed aircraft are rare;
- EW is controlled through simple area intents rather than frequency management;
- armed autonomous systems require authorization and remain fallible;
- performance budgets use simulation frequency, aggregation, and event-driven updates rather than an in-world unit cap;
- a small set of story-critical conditions may force reload while the larger plot absorbs ordinary character death;
- the square grid's provisional resolution and physical-scale streaming architecture require performance validation;
- named faction organizations and the checkpoint incident are proposed game additions;
- finite military consumables are the baseline until the scope of old-facility replenishment is decided.

---

## 24. Next design documents

This high-level design establishes the game's identity and system relationships. The next documents should resolve implementation-scale questions in roughly this order:

1. **Tactical simulation specification** — perception, ballistics abstraction, suppression, cohesion, movement, decision points, and WEGO timing.
2. **Command and information specification** — reports, communications graphs, delays, EW, alerts, leader AI, and order interpretation.
3. **Character and relationship specification** — skills, traits, affinity, disobedience, leadership, development, succession, and deterministic upbringing.
4. **Unified theater architecture** — streaming, authoritative geometry, event-driven simulation, time advancement, map knowledge, task forces, and seamless local-to-distant command.
5. **Logistics and equipment specification** — stocks, presets, transport, maintenance, recovery, restoration, and finite technological dependencies.
6. **Campaign narrative architecture** — factions, first-generation arcs, catastrophe state transfer, descendant-cell generation, and second-generation reconstruction.
7. **Content vertical slice** — one continuous corridor containing three settlements, two factions, one Archon-linked system, one base, and a complete operation from reconnaissance through aftermath.

---

## 25. Design test

Any proposed mechanic should answer yes to at least one of these questions:

- Does it create a meaningful tactical decision?
- Does it connect immediate action to theater-wide consequence?
- Does it make a character, relationship, formation, or place more historically specific?
- Does it make information, logistics, leadership, or time matter?
- Does it express Achlydesa through material consequences?

If it adds recurring input without improving one of those decisions, automate it, summarize it, or remove it.
