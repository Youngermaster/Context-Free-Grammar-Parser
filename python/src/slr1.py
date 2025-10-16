"""
SLR(1) parser (Bottom-Up parsing with Simple LR).

This module implements an SLR(1) shift-reduce parser using:
1. LR(0) items and automaton construction
2. ACTION and GOTO tables
3. Shift-reduce parsing algorithm

SLR(1) means:
- S: Simple (uses FOLLOW sets, not full lookahead)
- L: Left-to-right scanning
- R: Rightmost derivation in reverse
- 1: One symbol of lookahead
"""

from typing import Dict, Set, Tuple, List, FrozenSet
from .utils import Symbol, char_to_symbol, END_MARKER, EPSILON
from .grammar import Grammar, Production


class NotSLR1Exception(Exception):
    """Exception raised when a grammar is not SLR(1)."""
    pass


class LR0Item:
    """
    Represents an LR(0) item: a production with a dot position.

    An LR(0) item shows parsing progress in a production.
    Example: [A → α • β] means we've seen α and expect β

    The dot (•) position indicates how much of the production we've recognized.
    """

    def __init__(self, production: Production, dot_pos: int):
        """
        Initialize an LR(0) item.

        Args:
            production: The grammar production
            dot_pos: Position of the dot (0 to len(rhs))
        """
        self.production = production
        self.dot_pos = dot_pos

    def symbol_after_dot(self) -> Symbol | None:
        """
        Get the symbol immediately after the dot, if any.

        Returns:
            The symbol after the dot, or None if dot is at the end

        Examples:
            [A → a • B c] returns B
            [A → a b •] returns None
        """
        if self.dot_pos < len(self.production.rhs):
            return self.production.rhs[self.dot_pos]
        return None

    def __str__(self):
        """String representation with dot."""
        lhs = str(self.production.lhs)
        rhs = self.production.rhs

        # Build RHS with dot inserted
        rhs_parts = []
        for i, symbol in enumerate(rhs):
            if i == self.dot_pos:
                rhs_parts.append("•")
            rhs_parts.append(str(symbol))

        # Dot at the end
        if self.dot_pos == len(rhs):
            rhs_parts.append("•")

        rhs_str = " ".join(rhs_parts)
        return f"{lhs} → {rhs_str}"

    def __eq__(self, other):
        if not isinstance(other, LR0Item):
            return False
        return (self.production == other.production and
                self.dot_pos == other.dot_pos)

    def __hash__(self):
        return hash((self.production, self.dot_pos))


class SLR1Action:
    """Represents an action in the SLR(1) ACTION table."""

    def __init__(self, action_type: str, value=None):
        """
        Initialize an SLR(1) action.

        Args:
            action_type: One of "shift", "reduce", "accept", "error"
            value: State number (for shift) or Production (for reduce)
        """
        self.type = action_type
        self.value = value

    def __str__(self):
        if self.type == "shift":
            return f"shift {self.value}"
        elif self.type == "reduce":
            return f"reduce {self.value}"
        elif self.type == "accept":
            return "accept"
        else:
            return "error"

    def __repr__(self):
        return f"SLR1Action({self.type}, {self.value})"


class SLR1Parser:
    """
    SLR(1) bottom-up shift-reduce parser.

    Uses LR(0) automaton with FOLLOW sets for lookahead.

    Parsing algorithm:
    1. Maintain a stack of states and symbols
    2. Look up ACTION[state, input]:
       - Shift: Push input and new state
       - Reduce: Pop RHS, push LHS, goto new state
       - Accept: Input is valid
       - Error: Input is invalid
    """

    def __init__(self, grammar: Grammar, first_sets: Dict[Symbol, Set[Symbol]],
                 follow_sets: Dict[Symbol, Set[Symbol]]):
        """
        Initialize the SLR(1) parser.

        Args:
            grammar: The context-free grammar
            first_sets: FIRST sets for all symbols
            follow_sets: FOLLOW sets for all nonterminals

        Raises:
            NotSLR1Exception: If the grammar is not SLR(1)
        """
        self.grammar = grammar
        self.first_sets = first_sets
        self.follow_sets = follow_sets

        # Create augmented grammar: S' → S
        self.augmented_start = Symbol("'")
        self.start_production = Production(
            self.augmented_start,
            [grammar.start_symbol]
        )

        # Build LR(0) automaton
        self.states, self.transitions = self._build_lr0_automaton()

        # Build ACTION and GOTO tables
        self.action_table, self.goto_table = self._build_tables()

    def _closure(self, items: Set[LR0Item]) -> Set[LR0Item]:
        """
        Compute closure of a set of LR(0) items.

        For each item [A → α • B β] where B is nonterminal,
        add all items [B → • γ] for each production B → γ

        Args:
            items: Initial set of items

        Returns:
            Closure (may include additional items)
        """
        closure = set(items)
        changed = True

        while changed:
            changed = False
            new_items = set()

            for item in closure:
                symbol = item.symbol_after_dot()

                # If symbol after dot is nonterminal, add its productions
                if symbol and symbol.is_nonterminal():
                    for prod in self.grammar.get_productions(symbol):
                        new_item = LR0Item(prod, 0)
                        if new_item not in closure:
                            new_items.add(new_item)
                            changed = True

            closure.update(new_items)

        return closure

    def _goto(self, items: Set[LR0Item], symbol: Symbol) -> Set[LR0Item]:
        """
        Compute GOTO(items, symbol): items with dot moved over symbol.

        Args:
            items: Set of LR(0) items
            symbol: Symbol to move over

        Returns:
            New set of items with dot advanced
        """
        moved_items = set()

        for item in items:
            sym_after_dot = item.symbol_after_dot()
            if sym_after_dot == symbol:
                # Move dot over the symbol
                new_item = LR0Item(item.production, item.dot_pos + 1)
                moved_items.add(new_item)

        return self._closure(moved_items)

    def _build_lr0_automaton(self) -> Tuple[List[FrozenSet[LR0Item]],
                                           Dict[Tuple[int, Symbol], int]]:
        """
        Build the canonical LR(0) collection of item sets.

        Returns:
            Tuple of (states, transitions):
            - states: List of item sets (each set is a state)
            - transitions: Dict mapping (state_id, symbol) → next_state_id
        """
        # Initial item: [S' → • S]
        initial_item = LR0Item(self.start_production, 0)
        initial_state = frozenset(self._closure({initial_item}))

        states = [initial_state]
        state_to_id = {initial_state: 0}
        transitions = {}

        # Worklist: states to process
        worklist = [initial_state]

        while worklist:
            current_state = worklist.pop(0)
            current_id = state_to_id[current_state]

            # Find all symbols that can be shifted
            symbols_to_shift = set()
            for item in current_state:
                sym = item.symbol_after_dot()
                if sym:
                    symbols_to_shift.add(sym)

            # For each symbol, compute GOTO and add new states
            for symbol in symbols_to_shift:
                next_state = self._goto(current_state, symbol)

                if not next_state:
                    continue

                # Convert to frozenset for hashing
                next_state = frozenset(next_state)

                # Add state if new
                if next_state not in state_to_id:
                    state_id = len(states)
                    states.append(next_state)
                    state_to_id[next_state] = state_id
                    worklist.append(next_state)

                # Add transition
                next_id = state_to_id[next_state]
                transitions[(current_id, symbol)] = next_id

        return states, transitions

    def _build_tables(self) -> Tuple[Dict[Tuple[int, Symbol], SLR1Action],
                                     Dict[Tuple[int, Symbol], int]]:
        """
        Build ACTION and GOTO tables for SLR(1).

        Returns:
            Tuple of (action_table, goto_table)
        """
        action_table = {}
        goto_table = {}

        for state_id, state in enumerate(self.states):
            for item in state:
                sym_after_dot = item.symbol_after_dot()

                if sym_after_dot:
                    # Shift items: [A → α • a β] where a is terminal
                    if sym_after_dot.is_terminal():
                        key = (state_id, sym_after_dot)
                        if key in self.transitions:
                            next_state = self.transitions[key]
                            action = SLR1Action("shift", next_state)

                            # Check for conflicts
                            if key in action_table:
                                raise NotSLR1Exception(
                                    f"Shift/Shift or Shift/Reduce conflict at state {state_id}, "
                                    f"symbol {sym_after_dot}"
                                )

                            action_table[key] = action

                else:
                    # Reduce items: [A → α •]
                    if item.production.lhs == self.augmented_start:
                        # Accept item: [S' → S •]
                        key = (state_id, END_MARKER)
                        action_table[key] = SLR1Action("accept")
                    else:
                        # Reduce on FOLLOW(A)
                        follow_set = self.follow_sets.get(item.production.lhs, set())
                        for symbol in follow_set:
                            key = (state_id, symbol)
                            action = SLR1Action("reduce", item.production)

                            # Check for conflicts
                            if key in action_table:
                                existing = action_table[key]
                                if existing.type == "shift":
                                    raise NotSLR1Exception(
                                        f"Shift/Reduce conflict at state {state_id}, "
                                        f"symbol {symbol}"
                                    )
                                elif existing.type == "reduce":
                                    raise NotSLR1Exception(
                                        f"Reduce/Reduce conflict at state {state_id}, "
                                        f"symbol {symbol}"
                                    )

                            action_table[key] = action

            # Build GOTO table for nonterminals
            for (src, symbol), dst in self.transitions.items():
                if src == state_id and symbol.is_nonterminal():
                    goto_table[(state_id, symbol)] = dst

        return action_table, goto_table

    def parse(self, input_string: str) -> bool:
        """
        Parse an input string using SLR(1) shift-reduce algorithm.

        Args:
            input_string: The string to parse

        Returns:
            True if accepted, False otherwise
        """
        # Convert input to symbols and add $
        input_symbols = [char_to_symbol(c) for c in input_string]
        input_symbols.append(END_MARKER)

        # Initialize stacks
        state_stack = [0]  # Stack of states
        symbol_stack = []  # Stack of symbols

        input_idx = 0

        while True:
            current_state = state_stack[-1]
            current_input = input_symbols[input_idx]

            # Look up action
            key = (current_state, current_input)
            action = self.action_table.get(key, SLR1Action("error"))

            if action.type == "accept":
                return True

            elif action.type == "error":
                return False

            elif action.type == "shift":
                # Push symbol and state
                symbol_stack.append(current_input)
                state_stack.append(action.value)
                input_idx += 1

            elif action.type == "reduce":
                prod = action.value

                # Pop |rhs| symbols and states
                rhs_len = len(prod.rhs)
                if prod.rhs == [EPSILON]:
                    rhs_len = 0

                for _ in range(rhs_len):
                    if symbol_stack:
                        symbol_stack.pop()
                    if len(state_stack) > 1:
                        state_stack.pop()

                # Push LHS
                symbol_stack.append(prod.lhs)

                # Look up GOTO
                goto_state = state_stack[-1]
                goto_key = (goto_state, prod.lhs)

                if goto_key not in self.goto_table:
                    return False

                next_state = self.goto_table[goto_key]
                state_stack.append(next_state)

    def print_states(self):
        """Print LR(0) automaton states."""
        print("\nLR(0) Automaton States:")
        for i, state in enumerate(self.states):
            print(f"\nState {i}:")
            for item in sorted(state, key=str):
                print(f"  {item}")

    def print_tables(self):
        """Print ACTION and GOTO tables."""
        print("\nACTION Table:")
        sorted_actions = sorted(self.action_table.items(),
                              key=lambda x: (x[0][0], str(x[0][1])))
        for (state, symbol), action in sorted_actions:
            print(f"  [{state}, {symbol}] = {action}")

        print("\nGOTO Table:")
        sorted_gotos = sorted(self.goto_table.items(),
                            key=lambda x: (x[0][0], str(x[0][1])))
        for (state, symbol), next_state in sorted_gotos:
            print(f"  [{state}, {symbol}] = {next_state}")
