//! LL(1) predictive parser implementation.
//!
//! This module implements a top-down LL(1) predictive parser using a parse table.

use crate::error::{GrammarError, Result};
use crate::first_follow::{first_of_string, FirstSets, FollowSets};
use crate::grammar::{Grammar, Production};
use crate::symbol::{string_to_symbols, Symbol};
use std::collections::HashMap;

/// LL(1) predictive parser.
#[derive(Debug)]
pub struct LL1Parser {
    grammar: Grammar,
    /// Parse table: M[Nonterminal, Terminal/EndMarker] = Production
    table: HashMap<(Symbol, Symbol), Production>,
    first_sets: FirstSets,
    follow_sets: FollowSets,
}

impl LL1Parser {
    /// Builds an LL(1) parser from a grammar.
    ///
    /// # Algorithm
    /// For each production A → α:
    /// 1. For each terminal a in FIRST(α), add A → α to M[A, a]
    /// 2. If ε ∈ FIRST(α), for each b in FOLLOW(A), add A → α to M[A, b]
    ///
    /// If any cell has multiple entries, the grammar is not LL(1).
    pub fn build(
        grammar: Grammar,
        first_sets: FirstSets,
        follow_sets: FollowSets,
    ) -> Result<Self> {
        let mut table: HashMap<(Symbol, Symbol), Production> = HashMap::new();

        for production in grammar.all_productions() {
            let lhs = production.lhs;
            let rhs = &production.rhs;

            // Compute FIRST(α)
            let first_alpha = first_of_string(&first_sets, rhs);

            // For each terminal in FIRST(α) - {ε}
            for symbol in &first_alpha {
                if !symbol.is_epsilon() {
                    let key = (lhs, *symbol);

                    // Check for conflicts
                    if let Some(existing_prod) = table.get(&key) {
                        return Err(GrammarError::LL1Conflict {
                            nonterminal: lhs.to_string(),
                            terminal: symbol.to_string(),
                            prod1: existing_prod.to_string(),
                            prod2: production.to_string(),
                        });
                    }

                    table.insert(key, production.clone());
                }
            }

            // If ε ∈ FIRST(α)
            if first_alpha.contains(&Symbol::Epsilon) {
                let follow_lhs = follow_sets.get(&lhs).cloned().unwrap_or_default();

                for symbol in &follow_lhs {
                    let key = (lhs, *symbol);

                    // Check for conflicts
                    if let Some(existing_prod) = table.get(&key) {
                        return Err(GrammarError::LL1Conflict {
                            nonterminal: lhs.to_string(),
                            terminal: symbol.to_string(),
                            prod1: existing_prod.to_string(),
                            prod2: production.to_string(),
                        });
                    }

                    table.insert(key, production.clone());
                }
            }
        }

        Ok(Self {
            grammar,
            table,
            first_sets,
            follow_sets,
        })
    }

    /// Parses an input string using the LL(1) parse table.
    ///
    /// # Algorithm
    /// Stack initially contains [$, S]
    /// Input ends with $
    ///
    /// At each step:
    /// - If top of stack = current input symbol: pop and advance
    /// - If top is nonterminal: use table to get production, pop and push RHS (reversed)
    /// - If top is terminal but ≠ input: reject
    /// - If table entry is empty: reject
    /// - Accept when stack is [$] and input is [$]
    pub fn parse(&self, input: &str) -> bool {
        // Convert input to symbols and add $
        let mut input_symbols = string_to_symbols(input);
        input_symbols.push(Symbol::EndMarker);

        // Initialize stack with [$, S]
        let start = self.grammar.start_symbol();
        let mut stack = vec![Symbol::EndMarker, start];

        let mut input_index = 0;

        while !stack.is_empty() && input_index < input_symbols.len() {
            let top = *stack.last().unwrap();
            let current_input = input_symbols[input_index];

            // If top matches input, pop both
            if top == current_input {
                stack.pop();
                input_index += 1;
                continue;
            }

            // If top is nonterminal, use parse table
            if top.is_nonterminal() {
                let key = (top, current_input);

                if let Some(production) = self.table.get(&key) {
                    // Pop nonterminal
                    stack.pop();

                    // Push RHS in reverse order (skip epsilon)
                    if production.rhs != vec![Symbol::Epsilon] {
                        for symbol in production.rhs.iter().rev() {
                            stack.push(*symbol);
                        }
                    }
                } else {
                    // No table entry - reject
                    return false;
                }
            } else {
                // Top is terminal but doesn't match input - reject
                return false;
            }
        }

        // Accept if both stack and input are consumed
        stack.is_empty() && input_index == input_symbols.len()
    }

    /// Returns a reference to the parse table.
    pub fn table(&self) -> &HashMap<(Symbol, Symbol), Production> {
        &self.table
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::first_follow::{compute_first_sets, compute_follow_sets};

    #[test]
    fn test_ll1_simple() {
        let lines = vec![
            "3".to_string(),
            "S -> AB".to_string(),
            "A -> aA d".to_string(),
            "B -> bBc e".to_string(),
        ];

        let grammar = Grammar::parse(&lines).unwrap();
        let first_sets = compute_first_sets(&grammar);
        let follow_sets = compute_follow_sets(&grammar, &first_sets);

        let parser = LL1Parser::build(grammar, first_sets, follow_sets);
        assert!(parser.is_ok());

        let parser = parser.unwrap();
        assert!(parser.parse("d"));
        assert!(parser.parse("adbc"));
        assert!(!parser.parse("a"));
    }
}
