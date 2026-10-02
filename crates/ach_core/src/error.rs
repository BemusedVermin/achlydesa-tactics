//! Error type for `ach_core`. Implements Execution Plan §3.2 (public fallible functions return `Result`).

/// Failures raised by core primitives.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CoreError {
    /// The `u64` identifier space ran out.
    #[error("identifier space exhausted")]
    IdExhausted,
    /// Time arithmetic left the `i64` millisecond range.
    #[error("time overflow")]
    TimeOverflow,
    /// A string was not a valid `D<day> HH:MM[:SS]` time.
    #[error("invalid time {0:?}: expected \"D<day> HH:MM[:SS]\"")]
    InvalidTime(String),
    /// A journal append used a time earlier than the previous entry.
    #[error("time went backwards: {attempted_ms} ms is before {last_ms} ms")]
    TimeWentBackwards {
        /// Time of the previous entry, ms.
        last_ms: i64,
        /// Rejected time, ms.
        attempted_ms: i64,
    },
    /// The file does not start with the expected magic bytes.
    #[error("bad magic: expected {expected:?}, found {found:?}")]
    BadMagic {
        /// Magic the caller asked for.
        expected: [u8; 4],
        /// Magic found in the file.
        found: [u8; 4],
    },
    /// The file's `format_version` is not supported.
    #[error("unsupported format version {0}")]
    UnsupportedFormat(u16),
    /// The final frame ended before its declared length.
    #[error("truncated frame")]
    TruncatedFrame,
    /// A frame exceeds the 16 MiB limit.
    #[error("frame of {0} bytes exceeds the maximum")]
    FrameTooLarge(u64),
    /// The header label does not fit its `u16` length prefix.
    #[error("label of {0} bytes is too long")]
    LabelTooLong(usize),
    /// The header label is not valid UTF-8.
    #[error("label is not valid UTF-8")]
    InvalidLabel,
    /// Postcard failed to encode or decode a payload.
    #[error("codec error: {0}")]
    Codec(String),
    /// An underlying read or write failed.
    #[error("io error: {0}")]
    Io(String),
}
