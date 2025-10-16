"""
Grammar module for context-free grammars.

This module provides:
- Production rule representation
- Grammar parsing from text input
- Grammar data structure with terminals, nonterminals, and productions
"""

from typing import List, Set, Dict
from .utils import Symbol, char_to_symbol, string_to_symbols, symbols_to_string, EPSILON


class Production:
    """
    Represents a grammar production rule.

    A production has the form: A → α
    where A is a nonterminal (left-hand side)
    and α is a sequence of symbols (right-hand side)

    Examples:
        >>> Production(Symbol('S'), [Symbol('a'), Symbol('S')])
        S → aS
    """

    def __init__(self, lhs: Symbol, rhs: List[Symbol]):
        """
        Initialize a production.

        Args:
            lhs: Left-hand side symbol (must be nonterminal)
            rhs: Right-hand side (list of symbols)
        """
        self.lhs = lhs  # Left-hand side (nonterminal)
        self.rhs = rhs  # Right-hand side (list of symbols)

    def __str__(self):
        """String representation of the production."""
        rhs_str = "ε" if self.rhs == [EPSILON] else symbols_to_string(self.rhs)
        return f"{self.lhs} → {rhs_str}"

    def __repr__(self):
        """Debug representation."""
        return f"Production({self.lhs}, {self.rhs})"

    def __eq__(self, other):
        """Check equality between productions."""
        if not isinstance(other, Production):
            return False
        return self.lhs == other.lhs and self.rhs == other.rhs

    def __hash__(self):
        """Make productions hashable."""
        return hash((self.lhs, tuple(self.rhs)))


class Grammar:
    """
    Represents a context-free grammar.

    A grammar consists of:
    - Nonterminals: Set of nonterminal symbols (uppercase letters)
    - Terminals: Set of terminal symbols (lowercase, digits, symbols)
    - Productions: List of production rules
    - Start symbol: The initial nonterminal (always 'S')

    Example grammar:
        S → aS | b
        Means: S can produce "a followed by S" OR just "b"
    """

    def __init__(self, productions: List[Production]):
        """
        Initialize a grammar from a list of productions.

        Args:
            productions: List of Production objects

        The constructor automatically:
        1. Extracts nonterminals from production left-hand sides
        2. Extracts terminals from production right-hand sides
        3. Sets the start symbol to 'S'
        4. Builds a map from nonterminals to their productions
        """
        self.productions = productions
        self.start_symbol = Symbol('S')

        # Extract nonterminals (all symbols that appear on the left side)
        self.nonterminals = set()
        for prod in productions:
            self.nonterminals.add(prod.lhs)

        # Extract all symbols from right-hand sides
        rhs_symbols = set()
        for prod in productions:
            for symbol in prod.rhs:
                rhs_symbols.add(symbol)

        # Terminals are symbols that:
        # 1. Appear in right-hand sides
        # 2. Are not nonterminals
        # 3. Are not epsilon or end marker
        self.terminals = set()
        for symbol in rhs_symbols:
            if symbol.is_terminal():
                self.terminals.add(symbol)

        # Build a map: nonterminal → list of productions
        self.production_map = {}
        for prod in productions:
            if prod.lhs not in self.production_map:
                self.production_map[prod.lhs] = []
            self.production_map[prod.lhs].append(prod)

    def get_productions(self, nonterminal: Symbol) -> List[Production]:
        """
        Get all productions for a given nonterminal.

        Args:
            nonterminal: The nonterminal symbol

        Returns:
            List of productions with this nonterminal on the left side

        Examples:
            >>> grammar.get_productions(Symbol('S'))
            [S → aS, S → b]
        """
        return self.production_map.get(nonterminal, [])

    def __str__(self):
        """String representation of the entire grammar."""
        return "\n".join(str(prod) for prod in self.productions)


def parse_production_line(line: str) -> List[Production]:
    """
    Parse a single production line into one or more productions.

    Format: "A -> alpha beta gamma"
    where alpha, beta, gamma are alternatives (separated by spaces)

    Each alternative becomes a separate production:
    - "S -> aS b" becomes:
      1. S → aS
      2. S → b

    Args:
        line: Production line string

    Returns:
        List of Production objects

    Examples:
        >>> parse_production_line("S -> aS b")
        [Production(S, [a, S]), Production(S, [b])]

    Raises:
        ValueError: If the line format is invalid
    """
    # Split on " -> " to get left and right sides
    parts = line.split(" -> ")
    if len(parts) != 2:
        raise ValueError(f"Invalid production format: {line}")

    lhs_str, rhs_str = parts
    lhs_str = lhs_str.strip()
    rhs_str = rhs_str.strip()

    # Left-hand side is a single character (nonterminal)
    if len(lhs_str) != 1:
        raise ValueError(f"Left-hand side must be a single character: {lhs_str}")

    lhs = char_to_symbol(lhs_str)

    # Right-hand side has alternatives separated by spaces
    alternatives = rhs_str.split()

    # Create one production for each alternative
    productions = []
    for alt in alternatives:
        rhs = string_to_symbols(alt)
        productions.append(Production(lhs, rhs))

    return productions


def parse_grammar(lines: List[str]) -> Grammar:
    """
    Parse a grammar from text input.

    Input format:
        Line 1: n (number of nonterminals/production lines)
        Lines 2 to n+1: Production rules

    Example input:
        3
        S -> S+T T
        T -> T*F F
        F -> (S) i

    Args:
        lines: List of input lines

    Returns:
        Grammar object

    Raises:
        ValueError: If input format is invalid
    """
    if not lines:
        raise ValueError("Empty grammar input")

    # First line: number of production lines
    n_str = lines[0].strip()
    n = int(n_str)

    # Next n lines: productions
    if len(lines) < n + 1:
        raise ValueError(f"Expected {n} production lines, got {len(lines) - 1}")

    production_lines = lines[1:n+1]

    # Parse all production lines
    all_productions = []
    for line in production_lines:
        prods = parse_production_line(line)
        all_productions.extend(prods)

    return Grammar(all_productions)
