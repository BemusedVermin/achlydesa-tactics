//! Error type for `ach_sched`. Implements Execution Plan §3.2 (public fallible functions return `Result`).

use ach_core::SimTime;

/// Failures raised by the event scheduler.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SchedError {
    /// The requested time is earlier than the scheduler's current time.
    #[error("time {at:?} is before now ({now:?})")]
    InPast {
        /// The requested time.
        at: SimTime,
        /// The scheduler's current time.
        now: SimTime,
    },
    /// Advancing would pass a pending event.
    #[error("cannot advance to {to:?}: event pending at {pending:?}")]
    WouldSkipEvent {
        /// The requested target time.
        to: SimTime,
        /// Time of the earliest pending event.
        pending: SimTime,
    },
    /// The insertion-sequence space ran out.
    #[error("event sequence space exhausted")]
    SeqExhausted,
}
