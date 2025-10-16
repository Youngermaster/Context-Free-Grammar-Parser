"""
LL(1) predictive parser (Top-Down parsing).

This module implements an LL(1) parser using a predictive parsing table.
LL(1) means:
- L: Left-to-right scanning of input
- L: Leftmost derivation
- 1: One symbol of lookahead

The parser uses a stack and a parse table to determine which production to apply.
"""

from typing import Dict, Set, Tuple, Optional
from .utils import Symbol, char_to_symbol, END_MARKER, EPSILON
from .grammar import Grammar, Production
from .first_follow import compute_first_of_string


class NotLL1Exception(Exception):
    """Exception raised when a grammar is not LL(1)."""
    pass


class LL1Parser:
    """
    LL(1) predictive parser.

    The parser uses a parse table M[A, a] where:
    - A is a nonterminal
    - a is a terminal or $
    - M[A, a] contains the production to use

    Parsing algorithm:
    1. Stack starts with [$, S] where S is start symbol
    2. Input ends with $
    3. At each step:
       - If top = current input: pop both and continue
       - If top is nonterminal: look up M[top, input], replace top with production RHS
       - If top is terminal but ≠ input: reject
       - If M[top, input] is empty: reject
    4. Accept when stack is [$] and input is [$]
    """

    def __init__(self, grammar: Grammar, first_sets: Dict[Symbol, Set[Symbol]],
                 follow_sets: Dict[Symbol, Set[Symbol]]):
        """
        Initialize the LL(1) parser.

        Args:
            grammar: The context-free grammar
            first_sets: FIRST sets for all symbols
            follow_sets: FOLLOW sets for all nonterminals

        Raises:
            NotLL1Exception: If the grammar is not LL(1)
        """
        self.grammar = grammar
        self.first_sets = first_sets
        self.follow_sets = follow_sets
        self.table = self._build_table()

    def _build_table(self) -> Dict[Tuple[Symbol, Symbol], Production]:
        """
        Build the LL(1) predictive parsing table.

        For each production A → α:
        1. For each terminal a in FIRST(α), add A → α to M[A, a]
        2. If ε ∈ FIRST(α), for each b in FOLLOW(A), add A → α to M[A, b]

        If any cell has multiple entries, the grammar is not LL(1).

        Returns:
            Parse table as dictionary: (nonterminal, terminal) → production

        Raises:
            NotLL1Exception: If there's a conflict (multiple productions for same cell)
        """
        table = {}

        for prod in self.grammar.productions:
            lhs = prod.lhs
            rhs = prod.rhs

            # Compute FIRST(α) where α is the RHS
            first_alpha = compute_first_of_string(rhs, self.first_sets)

            # For each terminal in FIRST(α) - {ε}
            for symbol in first_alpha:
                if not symbol.is_epsilon():
                    key = (lhs, symbol)

                    # Check for conflicts
                    if key in table:
                        existing_prod = table[key]
                        raise NotLL1Exception(
                            f"Conflict at M[{lhs}, {symbol}]:\n"
                            f"  {existing_prod}\n"
                            f"  {prod}"
                        )

                    table[key] = prod

            # If ε ∈ FIRST(α), add entries for FOLLOW(A)
            if EPSILON in first_alpha:
                follow_lhs = self.follow_sets.get(lhs, set())
                for symbol in follow_lhs:
                    key = (lhs, symbol)

                    # Check for conflicts
                    if key in table:
                        existing_prod = table[key]
                        raise NotLL1Exception(
                            f"Conflict at M[{lhs}, {symbol}] (via epsilon):\n"
                            f"  {existing_prod}\n"
                            f"  {prod}"
                        )

                    table[key] = prod

        return table

    def parse(self, input_string: str) -> bool:
        """
        Parse an input string using the LL(1) algorithm.

        Algorithm:
        1. Initialize stack with [$, S]
        2. Initialize input with input_string + $
        3. Loop:
           - If top of stack = current input: pop and advance
           - If top is nonterminal: use table to get production, pop and push RHS
           - Otherwise: reject
        4. Accept when stack is [$] and input is [$]

        Args:
            input_string: The string to parse

        Returns:
            True if the string is accepted, False otherwise

        Examples:
            >>> parser.parse("i+i")  # for expression grammar
            True
            >>> parser.parse("(i")
            False
        """
        # Convert input string to symbols and add $
        input_symbols = [char_to_symbol(c) for c in input_string]
        input_symbols.append(END_MARKER)

        # Initialize stack with [$, S]
        stack = [END_MARKER, self.grammar.start_symbol]

        # Input pointer
        input_idx = 0

        # Parsing loop
        while stack:
            top = stack[-1]  # Peek at top of stack
            current_input = input_symbols[input_idx] if input_idx < len(input_symbols) else END_MARKER

            # Case 1: Top matches current input
            if top == current_input:
                stack.pop()  # Remove from stack
                input_idx += 1  # Advance input
                continue

            # Case 2: Top is a nonterminal
            if top.is_nonterminal():
                key = (top, current_input)

                # Look up in parse table
                if key not in self.table:
                    return False  # No entry in table → reject

                prod = self.table[key]

                # Pop nonterminal from stack
                stack.pop()

                # Push RHS in reverse order (so first symbol is on top)
                # Skip epsilon productions
                if prod.rhs != [EPSILON]:
                    for symbol in reversed(prod.rhs):
                        stack.append(symbol)

                continue

            # Case 3: Top is terminal but doesn't match input
            return False

        # Accept if we've consumed all input
        return input_idx == len(input_symbols)

    def print_table(self):
        """
        Print the parse table in a readable format.
        """
        print("\nLL(1) Parse Table:")

        # Sort entries for consistent output
        sorted_entries = sorted(self.table.items(),
                              key=lambda x: (str(x[0][0]), str(x[0][1])))

        for (nt, term), prod in sorted_entries:
            print(f"  M[{nt}, {term}] = {prod}")
