//! Context-Free Grammar Parser Library
//!
//! A Rust implementation of LL(1) and SLR(1) parsers for context-free grammars.

pub mod cli;
pub mod error;
pub mod first_follow;
pub mod grammar;
pub mod ll1;
pub mod slr1;
pub mod symbol;

// Re-export commonly used types
pub use error::{GrammarError, Result};
pub use grammar::{Grammar, Production};
pub use ll1::LL1Parser;
pub use slr1::SLR1Parser;
pub use symbol::Symbol;
