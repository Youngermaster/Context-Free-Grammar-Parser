//! Error types for the CFG parser.

use thiserror::Error;

/// Errors that can occur during grammar parsing and analysis.
#[derive(Error, Debug)]
pub enum GrammarError {
    #[error("Invalid grammar format: {0}")]
    InvalidFormat(String),

    #[error("Invalid production format: {0}")]
    InvalidProduction(String),

    #[error("Empty grammar input")]
    EmptyInput,

    #[error("Not enough production lines: expected {expected}, got {actual}")]
    NotEnoughProductions { expected: usize, actual: usize },

    #[error("LL(1) conflict at M[{nonterminal}, {terminal}]:\n  {prod1}\n  {prod2}")]
    LL1Conflict {
        nonterminal: String,
        terminal: String,
        prod1: String,
        prod2: String,
    },

    #[error("SLR(1) Shift/Reduce conflict at state {state}, symbol {symbol}")]
    SLR1ShiftReduceConflict { state: usize, symbol: String },

    #[error(
        "SLR(1) Reduce/Reduce conflict at state {state}, symbol {symbol}:\n  {prod1}\n  {prod2}"
    )]
    SLR1ReduceReduceConflict {
        state: usize,
        symbol: String,
        prod1: String,
        prod2: String,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Type alias for Results in this crate.
pub type Result<T> = std::result::Result<T, GrammarError>;
