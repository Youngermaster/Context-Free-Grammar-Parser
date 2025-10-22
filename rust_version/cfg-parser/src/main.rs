//! Context-Free Grammar Parser
//!
//! A Rust implementation of LL(1) and SLR(1) parsers for context-free grammars.
//!
//! This implementation provides:
//! - Algorithms to compute FIRST and FOLLOW sets
//! - LL(1) predictive parser (Top-Down)
//! - SLR(1) parser (Bottom-Up)
//! - Interactive CLI for grammar analysis and string parsing
//!
//! # Author
//! Juan Manuel Young Hoyos
//!
//! # References
//! Aho, Alfred V. et al. "Compilers: Principles, Techniques, and Tools" (2nd Edition).
//! Addison-Wesley, 2006.

mod cli;
mod error;
mod first_follow;
mod grammar;
mod ll1;
mod slr1;
mod symbol;

use std::process;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
