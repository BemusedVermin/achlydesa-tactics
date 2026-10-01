# ADR-0009 — No floating point in simulation math

**Status:** Proposed
**Date:** 2026-10-01 · **Conflict:** C-09

## Context
The Execution Plan fixes positions as `i64` centimeters but allows local math in `f64` "rounded back at defined boundaries" (Execution Plan §3.2 Determinism rules). `CLAUDE.md` forbids floating-point arithmetic in simulation crates and requires byte-identical journals for the same build, seed and inputs (CLAUDE.md §Simulation crates and their hard rules). The Technical Design does not promise cross-platform bit-identical replay by default (Technical Design §22.1 Deterministic contract), which float use would endanger further. The Phase 1 specs already follow CLAUDE.md (for example `docs/tasks/P1-07.md` precomputes Tobler's function into an integer table). See C-09.

## Options
1. **Allow `f64` locally, rounded at boundaries** (Execution Plan). Faster to write, but float results can differ across compilers and platforms, which breaks byte-identical journals.
2. **No floating point in simulation crates (recommended).** Integer positions, milli-units and per-mille ratios, with curves precomputed into integer lookup tables.
3. **Allow float only in tests and offline tools.** A narrower form of option 2.

## Decision
**PROPOSED:** Adopt option 2, which already includes option 3's allowances. Simulation crates use no floating-point arithmetic, enforced by `#![deny(clippy::float_arithmetic)]`. Float may be used in `ach_tools`, `ach_godot`, `spikes/` and tests only, and never in a value that flows back into simulation state. Reasoning: byte-identical replay is the foundation of every reason trace and snapshot test, and one platform-dependent rounding would corrupt it silently.

## Consequences
- **Documents to edit (outside P0-02's two-file scope):** Execution Plan §3.2 must delete "Local math can use `f64`". Because P0-02 edits only the High-Level Design and one line of CLAUDE.md, a separate edit is needed; until then this ADR outranks the Execution Plan. The High-Level Design has no float statement.
- **Task specs affected:** none; P1-01, P1-02 and P1-07 already comply.
- **What becomes true:** no document may suggest float in simulation crates.
