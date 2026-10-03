//! Grammar engine over a generic bindings trait. See Execution Plan §4.1 and §4.6.
//!
//! A Tracery-style, deterministic, voice-aware text grammar: templates bind to
//! simulation facts through [`Bindings`], and each speaker's [`VoiceProfile`]
//! biases which alternatives are chosen. There is no runtime model (Technical
//! Design §2).
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod condition;
pub mod error;
pub mod expand;
pub mod format;
pub mod grammar;
pub mod lint;
pub mod template;
pub mod voice;

pub use condition::{Condition, Literal, Op};
pub use error::TextError;
pub use expand::{Expander, ExpansionHistory};
pub use format::{Bindings, MapBindings, Value};
pub use grammar::{Alternative, Grammar};
pub use lint::{BindingSchema, LintIssue, lint};
pub use template::{Segment, Template};
pub use voice::{VoiceProfile, VoiceSet};
