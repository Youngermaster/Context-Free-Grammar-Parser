//! Grammar module for context-free grammars.
//!
//! This module provides data structures and parsing logic for working with
//! context-free grammars, including productions and grammar representation.

use crate::error::{GrammarError, Result};
use crate::symbol::{string_to_symbols, symbols_to_string, Symbol};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// A production rule in a context-free grammar.
///
/// Represents a rule of the form: LHS → RHS
/// where LHS is a single nonterminal and RHS is a sequence of symbols.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Production {
    /// Left-hand side (always a nonterminal)
    pub lhs: Symbol,
    /// Right-hand side (sequence of symbols)
    pub rhs: Vec<Symbol>,
}

impl Production {
    /// Creates a new production.
    pub fn new(lhs: Symbol, rhs: Vec<Symbol>) -> Self {
        Self { lhs, rhs }
    }
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rhs_str = if self.rhs == vec![Symbol::Epsilon] {
            "ε".to_string()
        } else {
            symbols_to_string(&self.rhs)
        };
        write!(f, "{} → {}", self.lhs, rhs_str)
    }
}

/// A context-free grammar.
///
/// Contains all productions, symbols, and provides methods for grammar analysis.
#[derive(Debug, Clone)]
pub struct Grammar {
    /// All productions in the grammar
    productions: Vec<Production>,
    /// All nonterminal symbols
    nonterminals: HashSet<Symbol>,
    /// All terminal symbols
    terminals: HashSet<Symbol>,
    /// The start symbol (always 'S')
    start_symbol: Symbol,
    /// Map from nonterminals to their productions
    production_map: HashMap<Symbol, Vec<Production>>,
}

impl Grammar {
    /// Parses a grammar from input lines.
    ///
    /// # Format
    /// - First line: number of nonterminals (n)
    /// - Next n lines: productions in format "A -> alpha beta gamma"
    ///   where alpha, beta, gamma are alternative productions separated by spaces
    pub fn parse(lines: &[String]) -> Result<Self> {
        if lines.is_empty() {
            return Err(GrammarError::EmptyInput);
        }

        let n = lines[0]
            .trim()
            .parse::<usize>()
            .map_err(|e| GrammarError::InvalidFormat(format!("Invalid number: {}", e)))?;

        if lines.len() < n + 1 {
            return Err(GrammarError::NotEnoughProductions {
                expected: n,
                actual: lines.len() - 1,
            });
        }

        let mut all_productions = Vec::new();

        // Parse each production line
        for line in &lines[1..=n] {
            let productions = Self::parse_production_line(line)?;
            all_productions.extend(productions);
        }

        Self::from_productions(all_productions)
    }

    /// Parses a single production line.
    ///
    /// Format: "A -> alpha beta gamma"
    /// Returns multiple productions (one for each alternative)
    fn parse_production_line(line: &str) -> Result<Vec<Production>> {
        let parts: Vec<&str> = line.split("->").collect();
        if parts.len() != 2 {
            return Err(GrammarError::InvalidProduction(line.to_string()));
        }

        let lhs_str = parts[0].trim();
        if lhs_str.is_empty() {
            return Err(GrammarError::InvalidProduction(
                "Empty left-hand side".to_string(),
            ));
        }

        let lhs = Symbol::from_char(lhs_str.chars().next().unwrap());

        let rhs_str = parts[1].trim();
        let alternatives: Vec<&str> = rhs_str.split_whitespace().collect();

        let mut productions = Vec::new();
        for alt in alternatives {
            let rhs = string_to_symbols(alt);
            productions.push(Production::new(lhs, rhs));
        }

        Ok(productions)
    }

    /// Creates a grammar from a list of productions.
    fn from_productions(productions: Vec<Production>) -> Result<Self> {
        if productions.is_empty() {
            return Err(GrammarError::EmptyInput);
        }

        // Extract all nonterminals from LHS
        let lhs_nonterminals: HashSet<Symbol> = productions.iter().map(|p| p.lhs).collect();

        // Extract all symbols from RHS
        let mut rhs_symbols = HashSet::new();
        for prod in &productions {
            for sym in &prod.rhs {
                rhs_symbols.insert(*sym);
            }
        }

        // Partition RHS symbols
        let rhs_nonterminals: HashSet<Symbol> = rhs_symbols
            .iter()
            .filter(|s| s.is_nonterminal())
            .copied()
            .collect();

        // All nonterminals = LHS ∪ RHS nonterminals
        let nonterminals: HashSet<Symbol> =
            lhs_nonterminals.union(&rhs_nonterminals).copied().collect();

        // Terminals = non-nonterminal symbols from RHS (excluding epsilon and $)
        let terminals: HashSet<Symbol> = rhs_symbols
            .iter()
            .filter(|s| s.is_terminal())
            .copied()
            .collect();

        // Start symbol is always 'S'
        let start_symbol = Symbol::Nonterminal('S');

        // Build production map
        let mut production_map: HashMap<Symbol, Vec<Production>> = HashMap::new();
        for prod in &productions {
            production_map
                .entry(prod.lhs)
                .or_default()
                .push(prod.clone());
        }

        Ok(Self {
            productions,
            nonterminals,
            terminals,
            start_symbol,
            production_map,
        })
    }

    /// Returns all productions for a given nonterminal.
    pub fn get_productions(&self, nt: Symbol) -> &[Production] {
        self.production_map
            .get(&nt)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Returns all productions in the grammar.
    pub fn all_productions(&self) -> &[Production] {
        &self.productions
    }

    /// Returns all nonterminals in the grammar.
    pub fn nonterminals(&self) -> &HashSet<Symbol> {
        &self.nonterminals
    }

    /// Returns all terminals in the grammar.
    pub fn terminals(&self) -> &HashSet<Symbol> {
        &self.terminals
    }

    /// Returns the start symbol.
    pub fn start_symbol(&self) -> Symbol {
        self.start_symbol
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for prod in &self.productions {
            writeln!(f, "{}", prod)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_grammar() {
        let lines = vec![
            "2".to_string(),
            "S -> AB".to_string(),
            "A -> a".to_string(),
        ];

        let grammar = Grammar::parse(&lines).unwrap();
        assert_eq!(grammar.productions.len(), 2);
        assert!(grammar.nonterminals.contains(&Symbol::Nonterminal('S')));
        assert!(grammar.terminals.contains(&Symbol::Terminal('a')));
    }

    #[test]
    fn test_parse_alternatives() {
        let lines = vec![
            "1".to_string(),
            "S -> a b c".to_string(),
        ];

        let grammar = Grammar::parse(&lines).unwrap();
        assert_eq!(grammar.productions.len(), 3);
    }
}
