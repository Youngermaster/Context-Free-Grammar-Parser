//! SLR(1) bottom-up parser implementation.
//!
//! This module implements a shift-reduce SLR(1) parser using LR(0) automaton
//! with lookahead from FOLLOW sets.

use crate::error::{GrammarError, Result};
use crate::first_follow::FollowSets;
use crate::grammar::{Grammar, Production};
use crate::symbol::{string_to_symbols, Symbol};
use std::collections::{HashMap, HashSet, VecDeque};

/// An LR(0) item: a production with a dot position.
///
/// For example: A → α•β is represented as (Production, position)
/// where position is the index of the dot.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Item {
    production: Production,
    dot_position: usize,
}

impl Item {
    fn new(production: Production, dot_position: usize) -> Self {
        Self {
            production,
            dot_position,
        }
    }

    /// Returns the symbol after the dot, if any.
    fn symbol_after_dot(&self) -> Option<Symbol> {
        self.production.rhs.get(self.dot_position).copied()
    }

    /// Checks if the dot is at the end (reduce item).
    fn is_reduce_item(&self) -> bool {
        self.dot_position >= self.production.rhs.len()
    }
}

/// A state in the LR(0) automaton (set of items).
type ItemSet = HashSet<Item>;

/// SLR(1) action.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Shift(usize),
    Reduce(Production),
    Accept,
}

/// SLR(1) parser.
#[derive(Debug)]
pub struct SLR1Parser {
    grammar: Grammar,
    augmented_start: Symbol,
    states: Vec<ItemSet>,
    /// ACTION table: (state, terminal/end_marker) → Action
    action_table: HashMap<(usize, Symbol), Action>,
    /// GOTO table: (state, nonterminal) → state
    goto_table: HashMap<(usize, Symbol), usize>,
}

impl SLR1Parser {
    /// Builds an SLR(1) parser from a grammar.
    pub fn build(grammar: Grammar, follow_sets: FollowSets) -> Result<Self> {
        // Create augmented grammar with S' → S
        let start = grammar.start_symbol();
        let augmented_start = Symbol::Nonterminal('\'');
        let start_production = Production::new(augmented_start, vec![start]);

        // Build LR(0) automaton
        let (states, transitions) = Self::build_lr0_automaton(&grammar, &start_production);

        // Build ACTION and GOTO tables
        let (action_table, goto_table) = Self::build_tables(
            &grammar,
            &states,
            &transitions,
            &follow_sets,
            augmented_start,
            &start_production,
        )?;

        Ok(Self {
            grammar,
            augmented_start,
            states,
            action_table,
            goto_table,
        })
    }

    /// Computes the closure of a set of items.
    ///
    /// For each item [A → α•Bβ] where B is nonterminal,
    /// add all items [B → •γ] for each production B → γ.
    fn closure(grammar: &Grammar, items: ItemSet) -> ItemSet {
        let mut result = items;
        let mut changed = true;

        while changed {
            changed = false;
            let current = result.clone();

            for item in &current {
                if let Some(symbol) = item.symbol_after_dot() {
                    if symbol.is_nonterminal() {
                        for production in grammar.get_productions(symbol) {
                            let new_item = Item::new(production.clone(), 0);
                            if !result.contains(&new_item) {
                                result.insert(new_item);
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// Computes goto(I, X) - the set of items obtained by moving dot over X.
    fn goto(grammar: &Grammar, items: &ItemSet, symbol: Symbol) -> ItemSet {
        let mut moved = ItemSet::new();

        for item in items {
            if let Some(sym) = item.symbol_after_dot() {
                if sym == symbol {
                    let new_item = Item::new(item.production.clone(), item.dot_position + 1);
                    moved.insert(new_item);
                }
            }
        }

        Self::closure(grammar, moved)
    }

    /// Builds the canonical LR(0) collection of item sets.
    fn build_lr0_automaton(
        grammar: &Grammar,
        start_production: &Production,
    ) -> (Vec<ItemSet>, HashMap<(usize, Symbol), usize>) {
        let initial_item = Item::new(start_production.clone(), 0);
        let initial_state = Self::closure(grammar, HashSet::from([initial_item]));

        let mut states = vec![initial_state.clone()];
        let mut transitions: HashMap<(usize, Symbol), usize> = HashMap::new();
        let mut worklist: VecDeque<usize> = VecDeque::new();
        worklist.push_back(0);

        while let Some(state_id) = worklist.pop_front() {
            let state = states[state_id].clone();

            // Get all symbols that can be shifted
            let mut symbols = HashSet::new();
            for item in &state {
                if let Some(symbol) = item.symbol_after_dot() {
                    symbols.insert(symbol);
                }
            }

            // For each symbol, compute goto and add new states
            for symbol in symbols {
                let next_state = Self::goto(grammar, &state, symbol);

                if !next_state.is_empty() {
                    // Check if this state already exists
                    if let Some(existing_id) = states.iter().position(|s| s == &next_state) {
                        transitions.insert((state_id, symbol), existing_id);
                    } else {
                        let new_id = states.len();
                        states.push(next_state);
                        worklist.push_back(new_id);
                        transitions.insert((state_id, symbol), new_id);
                    }
                }
            }
        }

        (states, transitions)
    }

    /// Builds ACTION and GOTO tables for SLR(1).
    #[allow(clippy::too_many_arguments)]
    fn build_tables(
        _grammar: &Grammar,
        states: &[ItemSet],
        transitions: &HashMap<(usize, Symbol), usize>,
        follow_sets: &FollowSets,
        augmented_start: Symbol,
        _start_production: &Production,
    ) -> Result<(
        HashMap<(usize, Symbol), Action>,
        HashMap<(usize, Symbol), usize>,
    )> {
        let mut action_table = HashMap::new();
        let mut goto_table = HashMap::new();

        for (state_id, state) in states.iter().enumerate() {
            for item in state {
                if !item.is_reduce_item() {
                    // Shift items: [A → α•aβ] where a is terminal
                    if let Some(symbol) = item.symbol_after_dot() {
                        if symbol.is_terminal() || symbol.is_end_marker() {
                            if let Some(&next_state) = transitions.get(&(state_id, symbol)) {
                                let key = (state_id, symbol);
                                if action_table.contains_key(&key) {
                                    return Err(GrammarError::SLR1ShiftReduceConflict {
                                        state: state_id,
                                        symbol: symbol.to_string(),
                                    });
                                }
                                action_table.insert(key, Action::Shift(next_state));
                            }
                        }
                    }
                } else {
                    // Reduce items: [A → α•]
                    if item.production.lhs == augmented_start {
                        // Accept item: [S' → S•]
                        let key = (state_id, Symbol::EndMarker);
                        action_table.insert(key, Action::Accept);
                    } else {
                        // Reduce on FOLLOW(A)
                        let follow_a = follow_sets
                            .get(&item.production.lhs)
                            .cloned()
                            .unwrap_or_default();

                        for symbol in follow_a {
                            let key = (state_id, symbol);

                            if let Some(existing) = action_table.get(&key) {
                                match existing {
                                    Action::Shift(_) => {
                                        return Err(GrammarError::SLR1ShiftReduceConflict {
                                            state: state_id,
                                            symbol: symbol.to_string(),
                                        });
                                    }
                                    Action::Reduce(other_prod) => {
                                        return Err(GrammarError::SLR1ReduceReduceConflict {
                                            state: state_id,
                                            symbol: symbol.to_string(),
                                            prod1: other_prod.to_string(),
                                            prod2: item.production.to_string(),
                                        });
                                    }
                                    Action::Accept => {}
                                }
                            } else {
                                action_table.insert(key, Action::Reduce(item.production.clone()));
                            }
                        }
                    }
                }
            }

            // Build GOTO table for nonterminals
            for (key, &next_state) in transitions {
                let (src, symbol) = key;
                if *src == state_id && symbol.is_nonterminal() {
                    goto_table.insert((state_id, *symbol), next_state);
                }
            }
        }

        Ok((action_table, goto_table))
    }

    /// Parses an input string using SLR(1) shift-reduce algorithm.
    pub fn parse(&self, input: &str) -> bool {
        // Convert input to symbols and add $
        let mut input_symbols = string_to_symbols(input);
        input_symbols.push(Symbol::EndMarker);

        // Initialize stack with state 0
        let mut stack: Vec<usize> = vec![0];
        let mut symbol_stack: Vec<Symbol> = Vec::new();
        let mut input_index = 0;

        loop {
            if input_index >= input_symbols.len() {
                return false;
            }

            let state = *stack.last().unwrap();
            let current_symbol = input_symbols[input_index];
            let key = (state, current_symbol);

            let action = self.action_table.get(&key);

            match action {
                Some(Action::Accept) => return true,
                Some(Action::Shift(next_state)) => {
                    // Push symbol and next state
                    stack.push(*next_state);
                    symbol_stack.push(current_symbol);
                    input_index += 1;
                }
                Some(Action::Reduce(production)) => {
                    // Pop |rhs| symbols and states
                    let rhs_len = if production.rhs == vec![Symbol::Epsilon] {
                        0
                    } else {
                        production.rhs.len()
                    };

                    for _ in 0..rhs_len {
                        stack.pop();
                        symbol_stack.pop();
                    }

                    // Get state at top of stack after popping
                    let state_after_pop = *stack.last().unwrap();

                    // Find next state via GOTO
                    let goto_key = (state_after_pop, production.lhs);
                    if let Some(&next_state) = self.goto_table.get(&goto_key) {
                        stack.push(next_state);
                        symbol_stack.push(production.lhs);
                    } else {
                        return false;
                    }
                }
                None => return false,
            }
        }
    }
}
