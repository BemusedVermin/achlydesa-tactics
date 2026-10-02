//! Deterministic value-noise detail below the source resolution.
//! Implements High-Level Design §4.1a and Execution Plan §4.5 (procedural detail).
//!
//! Integer only. Lattice values come from keyed RNG draws, so a point's detail depends
//! on nothing but the parameters and its coordinates.

use ach_core::{CM_PER_M, Domain, Seed, StreamKey, draw_below, mix64};
use serde::{Deserialize, Serialize};

/// Parameters of the detail field.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetailParams {
    /// Noise seed.
    pub seed: u64,
    /// Octave-0 amplitude, cm; the field stays within the sum of the octave amplitudes.
    pub amplitude_cm: i32,
    /// Number of octaves; octave `k` halves wavelength and amplitude `k` times.
    pub octaves: u8,
    /// Octave-0 wavelength, m.
    pub base_wavelength_m: u32,
}

impl Default for DetailParams {
    fn default() -> Self {
        Self {
            seed: 0xAC41_7DE5_0000_0001,
            amplitude_cm: 150,
            octaves: 3,
            base_wavelength_m: 240,
        }
    }
}

const Q: i64 = 1 << 16;

impl DetailParams {
    /// Upper bound on `|detail_cm|`: the sum of the octave amplitudes.
    pub fn max_abs_cm(&self) -> i32 {
        (0..self.octaves).map(|k| self.octave_amplitude(k)).sum()
    }

    fn octave_amplitude(&self, k: u8) -> i32 {
        self.amplitude_cm
            .checked_shr(u32::from(k))
            .unwrap_or(0)
            .max(0)
    }

    fn octave_wavelength_cm(&self, k: u8) -> i64 {
        i64::from(
            self.base_wavelength_m
                .checked_shr(u32::from(k))
                .unwrap_or(0),
        ) * CM_PER_M
    }

    /// Detail height at a point, cm. Octaves with zero amplitude or wavelength add nothing.
    pub fn eval_cm(&self, x_cm: i64, y_cm: i64) -> i32 {
        let sum: i64 = (0..self.octaves)
            .map(|k| self.octave_cm(k, x_cm, y_cm))
            .sum();
        i32::try_from(sum).expect("invariant: octave amplitudes sum within i32")
    }

    fn octave_cm(&self, k: u8, x_cm: i64, y_cm: i64) -> i64 {
        let (amp, wave) = (self.octave_amplitude(k), self.octave_wavelength_cm(k));
        if amp == 0 || wave == 0 {
            return 0;
        }
        let (i, fx) = lattice_cell(x_cm, wave);
        let (j, fy) = lattice_cell(y_cm, wave);
        let v = |di: i64, dj: i64| i128::from(self.lattice_value(k, amp, i + di, j + dj));
        let (sx, sy) = (smoothstep(fx), smoothstep(fy));
        let w = |a: i64, b: i64| i128::from(a * b);
        let sum = v(0, 0) * w(Q - sx, Q - sy)
            + v(1, 0) * w(sx, Q - sy)
            + v(0, 1) * w(Q - sx, sy)
            + v(1, 1) * w(sx, sy);
        i64::try_from(sum >> 32).expect("invariant: convex mix of i32 values fits i64")
    }

    fn lattice_value(&self, k: u8, amp: i32, i: i64, j: i64) -> i64 {
        let subject = mix64(mix64(mix64(u64::from(k)) ^ i as u64) ^ j as u64);
        let key = StreamKey {
            domain: Domain::Terrain,
            subject,
        };
        let span = 2 * u64::from(amp.unsigned_abs()) + 1;
        let draw = draw_below(Seed(self.seed), key, 0, span);
        i64::try_from(draw).expect("invariant: span fits i64") - i64::from(amp)
    }
}

/// Lattice index (floor) and Q16 fraction of `v` within a lattice of period `wave`.
fn lattice_cell(v: i64, wave: i64) -> (i64, i64) {
    let cell = v.div_euclid(wave);
    let rem = v.rem_euclid(wave);
    (cell, (rem << 16) / wave)
}

/// `3f² - 2f³` in Q16 for `f` in Q16.
fn smoothstep(f: i64) -> i64 {
    (f * f * (3 * Q - 2 * f)) >> 32
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const KNOWN: [i32; 3] = [144, -100, 12];

    #[test]
    fn known_answers() {
        let p = DetailParams::default();
        let got = [
            p.eval_cm(0, 0),
            p.eval_cm(12_345, -67_890),
            p.eval_cm(-5_000_000, 3_210_987),
        ];
        assert_eq!(got, KNOWN);
    }

    #[test]
    fn max_abs_is_sum_of_octaves() {
        assert_eq!(DetailParams::default().max_abs_cm(), 150 + 75 + 37);
    }

    #[test]
    fn seeds_differ() {
        let a = DetailParams::default();
        let b = DetailParams {
            seed: a.seed + 1,
            ..a.clone()
        };
        let differing = (1..=50)
            .filter(|i| a.eval_cm(i * 7_919, i * 104_729) != b.eval_cm(i * 7_919, i * 104_729))
            .count();
        assert!(differing > 40);
    }

    #[test]
    fn zero_amplitude_is_flat() {
        let p = DetailParams {
            amplitude_cm: 0,
            ..DetailParams::default()
        };
        assert_eq!(p.eval_cm(1234, 5678), 0);
    }

    proptest! {
        #[test]
        fn bounded_and_deterministic(
            x in -1_000_000_000i64..1_000_000_000,
            y in -1_000_000_000i64..1_000_000_000,
        ) {
            let p = DetailParams::default();
            let d = p.eval_cm(x, y);
            prop_assert!(d.abs() <= p.max_abs_cm());
            prop_assert_eq!(d, p.eval_cm(x, y));
        }
    }
}
