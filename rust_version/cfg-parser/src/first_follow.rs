//! FIRST and FOLLOW set computation for context-free grammars.
//!
//! This module implements algorithms from Aho et al., "Compilers: Principles,
//! Techniques, and Tools" (2nd Edition), sections 4.4.

use crate::grammar::Grammar;
use crate::symbol::Symbol;
use std::collections::{HashMap, HashSet};

/// Type alias for FIRST sets mapping.
pub type FirstSets = HashMap<Symbol, HashSet<Symbol>>;

/// Type alias for FOLLOW sets mapping.
pub type FollowSets = HashMap<Symbol, HashSet<Symbol>>;

/// Computes the FIRST sets for all symbols in the grammar.
///
/// # Algorithm
/// 1. For terminals: FIRST(a) = {a}
/// 2. For nonterminals A with production A → X₁X₂...Xₙ:
///    - Add FIRST(X₁) - {ε} to FIRST(A)
///    - If ε ∈ FIRST(X₁), add FIRST(X₂) - {ε}
///    - Continue while ε ∈ FIRST(Xᵢ)
///    - If ε ∈ FIRST(Xᵢ) for all i, add ε to FIRST(A)
/// 3. Repeat until no changes (fixed-point iteration)
pub fn compute_first_sets(grammar: &Grammar) -> FirstSets {
    let mut first_sets: FirstSets = HashMap::new();

    // Initialize FIRST sets for terminals
    for terminal in grammar.terminals() {
        first_sets.insert(*terminal, HashSet::from([*terminal]));
    }

    // Initialize epsilon and end marker
    first_sets.insert(Symbol::Epsilon, HashSet::from([Symbol::Epsilon]));
    first_sets.insert(Symbol::EndMarker, HashSet::from([Symbol::EndMarker]));

    // Initialize nonterminals with empty sets
    for nonterminal in grammar.nonterminals() {
        first_sets.insert(*nonterminal, HashSet::new());
    }

    // Fixed-point iteration
    let mut changed = true;
    while changed {
        changed = false;

        for production in grammar.all_productions() {
            let lhs = production.lhs;
            let current_first = first_sets.get(&lhs).unwrap().clone();

            // Compute FIRST of RHS
            let rhs_first = first_of_string(&first_sets, &production.rhs);

            // Union with current FIRST set
            let new_first: HashSet<Symbol> = current_first.union(&rhs_first).copied().collect();

            if new_first.len() != current_first.len() {
                first_sets.insert(lhs, new_first);
                changed = true;
            }
        }
    }

    first_sets
}

/// Computes FIRST set of a string (sequence of symbols).
///
/// # Algorithm
/// - Add FIRST(X₁) - {ε} to result
/// - If ε ∈ FIRST(X₁), add FIRST(X₂) - {ε}
/// - Continue while ε ∈ FIRST(Xᵢ)
/// - If ε ∈ FIRST(Xᵢ) for all i, add ε to result
pub fn first_of_string(first_sets: &FirstSets, symbols: &[Symbol]) -> HashSet<Symbol> {
    let mut result = HashSet::new();
    let mut has_epsilon = true;

    for symbol in symbols {
        if !has_epsilon {
            break;
        }

        let first_sym = first_sets.get(symbol).cloned().unwrap_or_default();

        // Add FIRST(symbol) - {ε}
        for sym in &first_sym {
            if !sym.is_epsilon() {
                result.insert(*sym);
            }
        }

        // Check if epsilon is in FIRST(symbol)
        has_epsilon = first_sym.contains(&Symbol::Epsilon);
    }

    // If all symbols can derive epsilon, add epsilon to result
    if has_epsilon {
        result.insert(Symbol::Epsilon);
    }

    result
}

/// Computes the FOLLOW sets for all nonterminals in the grammar.
///
/// # Algorithm
/// 1. FOLLOW(S) contains $
/// 2. For production A → αBβ:
///    - Add FIRST(β) - {ε} to FOLLOW(B)
///    - If ε ∈ FIRST(β) or β = ε, add FOLLOW(A) to FOLLOW(B)
/// 3. Repeat until no changes (fixed-point iteration)
pub fn compute_follow_sets(grammar: &Grammar, first_sets: &FirstSets) -> FollowSets {
    let mut follow_sets: FollowSets = HashMap::new();

    // Initialize all nonterminals with empty sets
    for nonterminal in grammar.nonterminals() {
        follow_sets.insert(*nonterminal, HashSet::new());
    }

    // Add $ to FOLLOW(S)
    let start_symbol = grammar.start_symbol();
    follow_sets
        .get_mut(&start_symbol)
        .unwrap()
        .insert(Symbol::EndMarker);

    // Fixed-point iteration
    let mut changed = true;
    while changed {
        changed = false;

        for production in grammar.all_productions() {
            let lhs = production.lhs;
            let rhs = &production.rhs;

            // Process each position in the RHS
            for (i, symbol) in rhs.iter().enumerate() {
                // Only process nonterminals
                if !symbol.is_nonterminal() {
                    continue;
                }

                let current_follow = follow_sets.get(symbol).unwrap().clone();
                let mut new_follow = current_follow.clone();

                // Get the rest of the production after this symbol
                let beta = &rhs[i + 1..];

                // Compute FIRST(β)
                let first_beta = first_of_string(first_sets, beta);

                // Add FIRST(β) - {ε} to FOLLOW(symbol)
                for sym in &first_beta {
                    if !sym.is_epsilon() {
                        new_follow.insert(*sym);
                    }
                }

                // If ε ∈ FIRST(β) or β is empty, add FOLLOW(lhs) to FOLLOW(symbol)
                if beta.is_empty() || first_beta.contains(&Symbol::Epsilon) {
                    let follow_lhs = follow_sets.get(&lhs).unwrap().clone();
                    new_follow = new_follow.union(&follow_lhs).copied().collect();
                }

                if new_follow.len() != current_follow.len() {
                    follow_sets.insert(*symbol, new_follow);
                    changed = true;
                }
            }
        }
    }

    follow_sets
}
