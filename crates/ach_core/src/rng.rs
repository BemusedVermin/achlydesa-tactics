//! Counter-based keyed random streams: the same (seed, domain, subject, counter)
//! always yields the same number, regardless of evaluation order.
//! Implements Execution Plan §3.2 and Technical Design §22.1 (determinism).

use crate::qty::PerMille;
use serde::{Deserialize, Serialize};

/// Campaign seed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Seed(pub u64);

/// Stable domain tags. NEVER renumber; only append.
#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Domain {
    /// Tests only.
    Test = 0,
    /// Terrain generation.
    Terrain = 1,
    /// Movement.
    Movement = 2,
    /// Logistics.
    Logistics = 3,
    /// Generated text.
    Text = 4,
    /// Portraits.
    Portrait = 5,
    /// Weather.
    Weather = 6,
}

/// Identifies one independent random stream within a seed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StreamKey {
    /// Which subsystem draws.
    pub domain: Domain,
    /// Subject within the domain (an id, a tile index, ...).
    pub subject: u64,
}

/// SplitMix64 finalizer. Frozen.
pub const fn mix64(z: u64) -> u64 {
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Frozen derivation:
/// `h = mix64(seed ^ 0x9E37_79B9_7F4A_7C15); h = mix64(h ^ domain as u64);
/// h = mix64(h ^ subject); h = mix64(h ^ counter)`.
pub fn draw_u64(seed: Seed, key: StreamKey, counter: u64) -> u64 {
    let h = mix64(seed.0 ^ 0x9E37_79B9_7F4A_7C15);
    let h = mix64(h ^ u64::from(key.domain as u32));
    let h = mix64(h ^ key.subject);
    mix64(h ^ counter)
}

/// Uniform in `[0, bound)` via 128-bit multiply-high. Bias is at most
/// `bound / 2^64`, which is documented and accepted. `bound == 0` yields 0.
pub fn draw_below(seed: Seed, key: StreamKey, counter: u64, bound: u64) -> u64 {
    let wide = u128::from(draw_u64(seed, key, counter)) * u128::from(bound);
    (wide >> 64) as u64
}

/// Uniform per-mille in `[0, 1000)`.
pub fn draw_permille(seed: Seed, key: StreamKey, counter: u64) -> PerMille {
    let v = draw_below(seed, key, counter, 1000);
    PerMille(i32::try_from(v).expect("invariant: draw_below(1000) < 1000 fits i32"))
}

/// Weighted choice over integer weights; returns the index, or `None` if all weights are 0.
/// Consumes exactly one draw at `counter`.
pub fn choose_weighted(seed: Seed, key: StreamKey, counter: u64, weights: &[u32]) -> Option<usize> {
    let total: u64 = weights.iter().map(|&w| u64::from(w)).sum();
    if total == 0 {
        return None;
    }
    let mut pick = draw_below(seed, key, counter, total);
    for (i, &w) in weights.iter().enumerate() {
        let w = u64::from(w);
        if pick < w {
            return Some(i);
        }
        pick -= w;
    }
    unreachable!("invariant: pick < total, so some weight absorbs it")
}

/// Stateful convenience cursor. Each call uses the current counter, then increments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RngCursor {
    /// Campaign seed.
    pub seed: Seed,
    /// Stream being read.
    pub key: StreamKey,
    /// Next counter to use.
    pub counter: u64,
}

impl RngCursor {
    /// A cursor on `key` whose first draw uses counter `start`.
    pub fn new(seed: Seed, key: StreamKey, start: u64) -> Self {
        Self {
            seed,
            key,
            counter: start,
        }
    }

    fn advance(&mut self) -> u64 {
        let c = self.counter;
        self.counter = c.wrapping_add(1);
        c
    }

    /// Next raw 64-bit draw.
    pub fn next_u64(&mut self) -> u64 {
        let c = self.advance();
        draw_u64(self.seed, self.key, c)
    }

    /// Next uniform draw in `[0, bound)`; see [`draw_below`].
    pub fn below(&mut self, bound: u64) -> u64 {
        let c = self.advance();
        draw_below(self.seed, self.key, c, bound)
    }

    /// Next uniform per-mille in `[0, 1000)`.
    pub fn permille(&mut self) -> PerMille {
        let c = self.advance();
        draw_permille(self.seed, self.key, c)
    }

    /// Next weighted choice; see [`choose_weighted`]. Consumes one counter even if `None`.
    pub fn choose_weighted(&mut self, weights: &[u32]) -> Option<usize> {
        let c = self.advance();
        choose_weighted(self.seed, self.key, c, weights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn key(domain: Domain, subject: u64) -> StreamKey {
        StreamKey { domain, subject }
    }

    // frozen P1-03; changing these breaks every save
    #[test]
    fn known_answers() {
        assert_eq!(
            draw_u64(Seed(0), key(Domain::Test, 0), 0),
            0x1957_a760_4e21_5178
        );
        assert_eq!(
            draw_u64(Seed(1), key(Domain::Terrain, 42), 7),
            0xac17_2442_8746_f535
        );
        assert_eq!(
            draw_u64(Seed(u64::MAX), key(Domain::Text, u64::MAX), u64::MAX),
            0xda7f_ba10_5048_8a71
        );
    }

    #[test]
    fn below_zero_bound_is_zero() {
        assert_eq!(draw_below(Seed(3), key(Domain::Test, 1), 9, 0), 0);
    }

    #[test]
    fn all_zero_weights_is_none() {
        assert_eq!(
            choose_weighted(Seed(3), key(Domain::Test, 1), 0, &[0, 0]),
            None
        );
        assert_eq!(choose_weighted(Seed(3), key(Domain::Test, 1), 0, &[]), None);
    }

    #[test]
    fn distribution_is_roughly_uniform() {
        let (seed, k) = (Seed(12345), key(Domain::Test, 77));
        let mut buckets = [0u32; 10];
        for c in 0..100_000 {
            buckets[draw_below(seed, k, c, 10) as usize] += 1;
        }
        for b in buckets {
            assert!((9_500..=10_500).contains(&b), "bucket count {b}");
        }
    }

    proptest! {
        #[test]
        fn below_is_in_range(seed: u64, subject: u64, counter: u64, bound in 1..=u64::MAX) {
            let v = draw_below(Seed(seed), key(Domain::Movement, subject), counter, bound);
            prop_assert!(v < bound);
        }

        #[test]
        fn permille_in_range(seed: u64, counter: u64) {
            let p = draw_permille(Seed(seed), key(Domain::Weather, 0), counter);
            prop_assert!((0..1000).contains(&p.0));
        }

        #[test]
        fn weighted_never_picks_zero(
            seed: u64, counter: u64,
            weights in proptest::collection::vec(prop_oneof![Just(0u32), 1..=u32::MAX], 0..16),
        ) {
            let r = choose_weighted(Seed(seed), key(Domain::Logistics, 5), counter, &weights);
            match r {
                Some(i) => prop_assert!(weights[i] > 0),
                None => prop_assert!(weights.iter().all(|&w| w == 0)),
            }
        }

        #[test]
        fn cursor_matches_direct(seed: u64, subject: u64, start: u64) {
            let (s, k) = (Seed(seed), key(Domain::Text, subject));
            let mut cur = RngCursor::new(s, k, start);
            for i in 0..8u64 {
                prop_assert_eq!(cur.next_u64(), draw_u64(s, k, start.wrapping_add(i)));
            }
        }
    }
}
