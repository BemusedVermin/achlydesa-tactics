//! Deterministic, serializable, cancellable event queue.
//! Implements Technical Design §16.1 (authoritative time), §16.2 (event ordering) and
//! §16.3 (scheduler outline), and Simulation §3.1-§3.2. See also Execution Plan §4.1.
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod error;

use ach_core::SimTime;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub use error::SchedError;

/// Total order: time, then phase (lower first), then insertion sequence.
///
/// At equal times a lower `phase` runs first (Technical Design §16.2); phase constants
/// are defined by the caller (`ach_sim`). Equal time and phase fall back to FIFO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EventKey {
    /// When the event fires.
    pub time: SimTime,
    /// Causal phase within a timestamp; lower runs first.
    pub phase: u8,
    /// Insertion sequence, unique per scheduler.
    pub seq: u64,
}

/// Handle returned by [`Scheduler::schedule`]; identical to the event's key.
pub type EventHandle = EventKey;

/// The authoritative queue of future events and the current simulation time.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scheduler<E> {
    queue: BTreeMap<EventKey, E>,
    next_seq: u64,
    now: SimTime,
}

impl<E> Scheduler<E> {
    /// An empty scheduler whose clock reads `start`.
    pub fn new(start: SimTime) -> Self {
        Self {
            queue: BTreeMap::new(),
            next_seq: 0,
            now: start,
        }
    }

    /// The current simulation time.
    pub fn now(&self) -> SimTime {
        self.now
    }

    /// Number of pending events.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Whether no events are pending.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Queues `event` at `at` in `phase`.
    ///
    /// # Errors
    /// [`SchedError::InPast`] if `at` is before [`now`](Self::now);
    /// [`SchedError::SeqExhausted`] if the sequence counter overflows.
    pub fn schedule(
        &mut self,
        at: SimTime,
        phase: u8,
        event: E,
    ) -> Result<EventHandle, SchedError> {
        if at < self.now {
            return Err(SchedError::InPast { at, now: self.now });
        }
        let next = self
            .next_seq
            .checked_add(1)
            .ok_or(SchedError::SeqExhausted)?;
        let key = EventKey {
            time: at,
            phase,
            seq: self.next_seq,
        };
        self.next_seq = next;
        self.queue.insert(key, event);
        Ok(key)
    }

    /// Removes a pending event, returning it, or `None` if already fired or cancelled.
    pub fn cancel(&mut self, handle: EventHandle) -> Option<E> {
        self.queue.remove(&handle)
    }

    /// The earliest pending event, without removing it.
    pub fn peek(&self) -> Option<(&EventKey, &E)> {
        self.queue.first_key_value()
    }

    /// Pops the earliest event and sets `now` to its time.
    pub fn pop_next(&mut self) -> Option<(EventKey, E)> {
        let (key, event) = self.queue.pop_first()?;
        self.now = key.time;
        Some((key, event))
    }

    /// Pops the earliest event only if its time is at or before `limit`.
    pub fn pop_until(&mut self, limit: SimTime) -> Option<(EventKey, E)> {
        if self.peek()?.0.time > limit {
            return None;
        }
        self.pop_next()
    }

    /// Moves `now` forward to `t` without firing anything.
    ///
    /// # Errors
    /// [`SchedError::InPast`] if `t < now`; [`SchedError::WouldSkipEvent`] if an event is
    /// pending strictly before `t`.
    pub fn advance_to(&mut self, t: SimTime) -> Result<(), SchedError> {
        if t < self.now {
            return Err(SchedError::InPast {
                at: t,
                now: self.now,
            });
        }
        if let Some((key, _)) = self.peek()
            && key.time < t
        {
            return Err(SchedError::WouldSkipEvent {
                to: t,
                pending: key.time,
            });
        }
        self.now = t;
        Ok(())
    }

    /// Pending events in firing order, for debugging views.
    pub fn iter(&self) -> impl Iterator<Item = (&EventKey, &E)> {
        self.queue.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn build(specs: &[(i64, u8)]) -> (Scheduler<usize>, Vec<EventHandle>) {
        let mut s = Scheduler::new(SimTime(0));
        let handles = specs
            .iter()
            .enumerate()
            .map(|(i, &(t, p))| s.schedule(SimTime(t), p, i).unwrap())
            .collect();
        (s, handles)
    }

    fn drain(s: &mut Scheduler<usize>) -> Vec<(EventKey, usize)> {
        std::iter::from_fn(|| s.pop_next()).collect()
    }

    fn specs() -> impl Strategy<Value = Vec<(i64, u8)>> {
        prop::collection::vec((0i64..5, 0u8..3), 0..60)
    }

    proptest! {
        #[test]
        fn pops_in_strictly_increasing_key_order(specs in specs()) {
            let (mut s, _) = build(&specs);
            let popped = drain(&mut s);
            prop_assert_eq!(popped.len(), specs.len());
            prop_assert!(popped.windows(2).all(|w| w[0].0 < w[1].0));
        }

        #[test]
        fn cancel_removes_exactly_the_chosen_subset(
            specs in specs(),
            mask in prop::collection::vec(any::<bool>(), 60),
        ) {
            let (mut s, handles) = build(&specs);
            let (mut reference, _) = build(&specs);
            let mut expected = drain(&mut reference);
            for (i, h) in handles.iter().enumerate() {
                if mask[i] {
                    prop_assert_eq!(s.cancel(*h), Some(i));
                    prop_assert_eq!(s.cancel(*h), None);
                }
            }
            expected.retain(|(_, i)| !mask[*i]);
            prop_assert_eq!(drain(&mut s), expected);
        }

        #[test]
        fn clone_mid_run_continues_identically(specs in specs(), cut in 0usize..60) {
            let (mut s, _) = build(&specs);
            for _ in 0..cut {
                s.pop_next();
            }
            let mut copy = s.clone();
            prop_assert_eq!(&copy, &s);
            prop_assert_eq!(drain(&mut copy), drain(&mut s));
        }
    }

    #[test]
    fn lower_phase_first_then_fifo_at_equal_time() {
        let (mut s, _) = build(&[(1, 2), (1, 0), (1, 2), (1, 0)]);
        let order: Vec<usize> = drain(&mut s).into_iter().map(|(_, i)| i).collect();
        assert_eq!(order, [1, 3, 0, 2]);
    }

    #[test]
    fn schedule_in_past_is_rejected() {
        let mut s = Scheduler::new(SimTime(10));
        assert_eq!(
            s.schedule(SimTime(9), 0, ()),
            Err(SchedError::InPast {
                at: SimTime(9),
                now: SimTime(10)
            })
        );
        assert!(s.schedule(SimTime(10), 0, ()).is_ok());
    }

    #[test]
    fn pop_next_advances_now() {
        let (mut s, _) = build(&[(7, 0)]);
        s.pop_next();
        assert_eq!(s.now(), SimTime(7));
    }

    #[test]
    fn pop_until_respects_limit() {
        let (mut s, _) = build(&[(5, 0), (9, 0)]);
        assert!(s.pop_until(SimTime(4)).is_none());
        assert_eq!(s.now(), SimTime(0));
        assert!(s.pop_until(SimTime(5)).is_some());
        assert!(s.pop_until(SimTime(8)).is_none());
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn advance_to_refuses_to_skip_pending_event() {
        let (mut s, _) = build(&[(5, 0)]);
        assert_eq!(
            s.advance_to(SimTime(6)),
            Err(SchedError::WouldSkipEvent {
                to: SimTime(6),
                pending: SimTime(5)
            })
        );
        assert_eq!(s.now(), SimTime(0));
        assert_eq!(s.advance_to(SimTime(5)), Ok(()));
        assert_eq!(s.now(), SimTime(5));
        assert!(matches!(
            s.advance_to(SimTime(4)),
            Err(SchedError::InPast { .. })
        ));
    }

    #[test]
    fn advance_to_on_empty_queue_moves_clock() {
        let mut s: Scheduler<()> = Scheduler::new(SimTime(0));
        assert_eq!(s.advance_to(SimTime(1_000)), Ok(()));
        assert_eq!(s.now(), SimTime(1_000));
    }
}
