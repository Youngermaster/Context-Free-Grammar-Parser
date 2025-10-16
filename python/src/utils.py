"""
Utility module for symbol classification and basic types.

This module provides the fundamental building blocks for grammar parsing:
- Symbol types (Terminal, Nonterminal, Epsilon, EndMarker)
- Symbol classification functions
- Helper functions for symbol manipulation
"""

from enum import Enum
from typing import Set, List


class SymbolType(Enum):
    """Enumeration of symbol types in a context-free grammar."""
    TERMINAL = "terminal"           # Lowercase letters, digits, symbols
    NONTERMINAL = "nonterminal"    # Uppercase letters (A-Z)
    EPSILON = "epsilon"            # Empty string (represented as 'e')
    END_MARKER = "end_marker"      # End of input (represented as '$')


class Symbol:
    """
    Represents a symbol in a context-free grammar.

    A symbol can be:
    - Terminal: an actual character in the input (lowercase, digits, symbols)
    - Nonterminal: a grammar rule name (uppercase letters A-Z)
    - Epsilon: represents an empty string (ε)
    - EndMarker: marks the end of input ($)

    Examples:
        >>> Symbol('a')  # Terminal
        >>> Symbol('A')  # Nonterminal
        >>> Symbol('e', is_epsilon=True)  # Epsilon
        >>> Symbol('$', is_end_marker=True)  # End marker
    """

    def __init__(self, value, is_epsilon=False, is_end_marker=False):
        """
        Initialize a symbol.

        Args:
            value: The character value of the symbol
            is_epsilon: True if this is the epsilon (empty) symbol
            is_end_marker: True if this is the end-of-input marker
        """
        self.value = value
        self._is_epsilon = is_epsilon
        self._is_end_marker = is_end_marker

        # Determine the symbol type
        if is_epsilon:
            self.type = SymbolType.EPSILON
        elif is_end_marker:
            self.type = SymbolType.END_MARKER
        elif self._is_uppercase(value):
            self.type = SymbolType.NONTERMINAL
        else:
            self.type = SymbolType.TERMINAL

    @staticmethod
    def _is_uppercase(char):
        """Check if a character is uppercase (A-Z)."""
        return 'A' <= char <= 'Z'

    def is_terminal(self):
        """Check if this symbol is a terminal."""
        return self.type == SymbolType.TERMINAL

    def is_nonterminal(self):
        """Check if this symbol is a nonterminal."""
        return self.type == SymbolType.NONTERMINAL

    def is_epsilon(self):
        """Check if this symbol is epsilon (empty string)."""
        return self.type == SymbolType.EPSILON

    def is_end_marker(self):
        """Check if this symbol is the end marker."""
        return self.type == SymbolType.END_MARKER

    def __str__(self):
        """Convert symbol to string representation."""
        if self.is_epsilon():
            return "ε"
        elif self.is_end_marker():
            return "$"
        else:
            return str(self.value)

    def __repr__(self):
        """String representation for debugging."""
        return f"Symbol('{self}')"

    def __eq__(self, other):
        """Check equality between symbols."""
        if not isinstance(other, Symbol):
            return False
        return (self.value == other.value and
                self.type == other.type)

    def __hash__(self):
        """Make symbols hashable for use in sets and dictionaries."""
        return hash((self.value, self.type))

    def __lt__(self, other):
        """
        Define ordering for symbols.
        Order: Epsilon < Terminals < Nonterminals < EndMarker
        """
        if not isinstance(other, Symbol):
            return NotImplemented

        # Epsilon comes first
        if self.is_epsilon() and not other.is_epsilon():
            return True
        if not self.is_epsilon() and other.is_epsilon():
            return False

        # EndMarker comes last
        if self.is_end_marker() and not other.is_end_marker():
            return False
        if not self.is_end_marker() and other.is_end_marker():
            return True

        # Terminals come before nonterminals
        if self.is_terminal() and other.is_nonterminal():
            return True
        if self.is_nonterminal() and other.is_terminal():
            return False

        # Same type: compare values
        return self.value < other.value


def char_to_symbol(char: str) -> Symbol:
    """
    Convert a character to a Symbol based on grammar conventions.

    Rules:
    - Uppercase letters (A-Z) → Nonterminal
    - 'e' → Epsilon (empty string)
    - '$' → End marker
    - Everything else → Terminal

    Args:
        char: A single character string

    Returns:
        Symbol object representing the character

    Examples:
        >>> char_to_symbol('A')  # Nonterminal
        >>> char_to_symbol('a')  # Terminal
        >>> char_to_symbol('e')  # Epsilon
        >>> char_to_symbol('$')  # End marker
    """
    if char == 'e':
        return Symbol('e', is_epsilon=True)
    elif char == '$':
        return Symbol('$', is_end_marker=True)
    else:
        return Symbol(char)


def string_to_symbols(text: str) -> List[Symbol]:
    """
    Convert a string to a list of symbols.

    Args:
        text: Input string

    Returns:
        List of Symbol objects

    Examples:
        >>> string_to_symbols("aAb")
        [Symbol('a'), Symbol('A'), Symbol('b')]
    """
    return [char_to_symbol(c) for c in text]


def symbols_to_string(symbols: List[Symbol]) -> str:
    """
    Convert a list of symbols to a string representation.

    Args:
        symbols: List of Symbol objects

    Returns:
        String representation

    Examples:
        >>> symbols_to_string([Symbol('a'), Symbol('A')])
        'aA'
    """
    if not symbols:
        return ""
    return "".join(str(s) for s in symbols)


def print_symbol_set(symbol_set: Set[Symbol]) -> str:
    """
    Format a set of symbols for printing.

    Args:
        symbol_set: Set of Symbol objects

    Returns:
        Formatted string like "{ a b c }"

    Examples:
        >>> print_symbol_set({Symbol('a'), Symbol('b')})
        '{ a b }'
    """
    sorted_symbols = sorted(symbol_set)
    symbols_str = " ".join(str(s) for s in sorted_symbols)
    return f"{{ {symbols_str} }}"


# Pre-defined special symbols for convenience
EPSILON = Symbol('e', is_epsilon=True)
END_MARKER = Symbol('$', is_end_marker=True)
