//! Symbol types and utilities for context-free grammars.
//!
//! This module defines the core Symbol type and utility functions for working with
//! grammar symbols (terminals, nonterminals, epsilon, and end marker).

use std::cmp::Ordering;
use std::fmt;

/// Represents a symbol in a context-free grammar.
///
/// # Grammar Conventions
/// - Terminals: Any character that is NOT uppercase (a-z, 0-9, symbols, etc.)
/// - Nonterminals: Uppercase letters (A-Z)
/// - Epsilon: The empty string, represented by 'e'
/// - EndMarker: The end-of-input marker '$'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
    /// A terminal symbol (lowercase, digits, or special characters)
    Terminal(char),
    /// A nonterminal symbol (uppercase letter)
    Nonterminal(char),
    /// The empty string (ε)
    Epsilon,
    /// The end-of-input marker ($)
    EndMarker,
}

impl Symbol {
    /// Converts a character to a symbol based on grammar conventions.
    ///
    /// # Examples
    /// ```
    /// use cfg_parser::symbol::Symbol;
    /// let sym = Symbol::from_char('A'); // Nonterminal
    /// let sym = Symbol::from_char('a'); // Terminal
    /// let sym = Symbol::from_char('e'); // Epsilon
    /// let sym = Symbol::from_char('$'); // EndMarker
    /// ```
    pub fn from_char(c: char) -> Self {
        if c.is_ascii_uppercase() {
            Symbol::Nonterminal(c)
        } else if c == 'e' {
            Symbol::Epsilon
        } else if c == '$' {
            Symbol::EndMarker
        } else {
            Symbol::Terminal(c)
        }
    }

    /// Checks if this symbol is a terminal.
    #[inline]
    pub const fn is_terminal(&self) -> bool {
        matches!(self, Symbol::Terminal(_))
    }

    /// Checks if this symbol is a nonterminal.
    #[inline]
    pub const fn is_nonterminal(&self) -> bool {
        matches!(self, Symbol::Nonterminal(_))
    }

    /// Checks if this symbol is epsilon (ε).
    #[inline]
    pub const fn is_epsilon(&self) -> bool {
        matches!(self, Symbol::Epsilon)
    }

    /// Checks if this symbol is the end marker ($).
    #[inline]
    pub const fn is_end_marker(&self) -> bool {
        matches!(self, Symbol::EndMarker)
    }

    /// Returns the character representation of this symbol, if applicable.
    pub const fn as_char(&self) -> Option<char> {
        match self {
            Symbol::Terminal(c) | Symbol::Nonterminal(c) => Some(*c),
            Symbol::Epsilon | Symbol::EndMarker => None,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Terminal(c) | Symbol::Nonterminal(c) => write!(f, "{}", c),
            Symbol::Epsilon => write!(f, "ε"),
            Symbol::EndMarker => write!(f, "$"),
        }
    }
}

/// Custom ordering for symbols to ensure consistent sorting.
///
/// Order: Epsilon < Terminals < Nonterminals < EndMarker
impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Symbol::Epsilon, Symbol::Epsilon) => Ordering::Equal,
            (Symbol::Epsilon, _) => Ordering::Less,
            (_, Symbol::Epsilon) => Ordering::Greater,

            (Symbol::EndMarker, Symbol::EndMarker) => Ordering::Equal,
            (Symbol::EndMarker, _) => Ordering::Greater,
            (_, Symbol::EndMarker) => Ordering::Less,

            (Symbol::Terminal(c1), Symbol::Terminal(c2)) => c1.cmp(c2),
            (Symbol::Terminal(_), Symbol::Nonterminal(_)) => Ordering::Less,
            (Symbol::Nonterminal(_), Symbol::Terminal(_)) => Ordering::Greater,
            (Symbol::Nonterminal(c1), Symbol::Nonterminal(c2)) => c1.cmp(c2),
        }
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Converts a string into a vector of symbols.
pub fn string_to_symbols(s: &str) -> Vec<Symbol> {
    s.chars().map(Symbol::from_char).collect()
}

/// Converts a vector of symbols back to a string.
pub fn symbols_to_string(symbols: &[Symbol]) -> String {
    symbols.iter().map(|s| s.to_string()).collect()
}
