//! Errors for the grammar engine. Implements Execution Plan §4.6.

use thiserror::Error;

/// Everything that can go wrong loading or expanding a grammar.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TextError {
    /// The RON source did not parse.
    #[error("RON parse error: {0}")]
    Ron(String),
    /// A condition string is malformed.
    #[error("bad condition {condition:?}: {reason}")]
    BadCondition {
        /// The offending condition source.
        condition: String,
        /// What is wrong with it.
        reason: String,
    },
    /// A template string is malformed.
    #[error("bad template {template:?}: {reason}")]
    BadTemplate {
        /// The offending template source.
        template: String,
        /// What is wrong with it.
        reason: String,
    },
    /// Two grammars define the same symbol.
    #[error("duplicate symbol {0:?}")]
    DuplicateSymbol(String),
    /// A symbol has more alternatives than history can index.
    #[error("symbol {symbol:?} has too many alternatives")]
    TooManyAlternatives {
        /// The symbol.
        symbol: String,
    },
    /// A template references a symbol the grammar does not define.
    #[error("unknown symbol {0:?}")]
    UnknownSymbol(String),
    /// A template references a binding that is not present.
    #[error("unknown binding {0:?}")]
    UnknownBinding(String),
    /// A `{path:fmt}` names a format that does not exist.
    #[error("unknown format {0:?}")]
    UnknownFormat(String),
    /// A format was applied to a value of the wrong type.
    #[error("format {fmt:?} cannot be applied to {found}")]
    FormatMismatch {
        /// The format name.
        fmt: String,
        /// The value variant found.
        found: &'static str,
    },
    /// Nesting exceeded the depth limit.
    #[error("symbol nesting too deep")]
    TooDeep,
    /// Output exceeded the length limit.
    #[error("expansion too long")]
    TooLong,
    /// No alternative of the symbol is eligible.
    #[error("no alternative available for symbol {symbol:?}")]
    NoAlternative {
        /// The symbol that could not be expanded.
        symbol: String,
    },
}
