"""
FIRST and FOLLOW sets computation for context-free grammars.

This module implements algorithms to compute:
1. FIRST sets: The set of terminals that can start strings derived from a symbol
2. FOLLOW sets: The set of terminals that can follow a nonterminal in derivations

These sets are essential for constructing LL(1) and SLR(1) parsers.
"""

from typing import Dict, Set, List
from .utils import Symbol, EPSILON, END_MARKER, print_symbol_set
from .grammar import Grammar, Production


def compute_first_sets(grammar: Grammar) -> Dict[Symbol, Set[Symbol]]:
    """
    Compute FIRST sets for all symbols in the grammar.

    FIRST(X) = set of terminals that can begin strings derived from X

    Algorithm:
    1. For terminals: FIRST(a) = {a}
    2. For nonterminals: Use fixed-point iteration
       For production A → X₁X₂...Xₙ:
       - Add FIRST(X₁) - {ε} to FIRST(A)
       - If ε ∈ FIRST(X₁), add FIRST(X₂) - {ε}
       - Continue while ε is possible
       - If all Xᵢ can derive ε, add ε to FIRST(A)
    3. Repeat until no changes

    Args:
        grammar: The grammar to analyze

    Returns:
        Dictionary mapping symbols to their FIRST sets

    Examples:
        For grammar:
            S → aB | c
            B → b | ε
        Results:
            FIRST(S) = {a, c}
            FIRST(B) = {b, ε}
            FIRST(a) = {a}
    """
    first_sets = {}

    # Step 1: Initialize FIRST sets for terminals
    for terminal in grammar.terminals:
        first_sets[terminal] = {terminal}

    # Add special symbols
    first_sets[EPSILON] = {EPSILON}
    first_sets[END_MARKER] = {END_MARKER}

    # Step 2: Initialize FIRST sets for nonterminals as empty
    for nonterminal in grammar.nonterminals:
        first_sets[nonterminal] = set()

    # Step 3: Fixed-point iteration
    changed = True
    while changed:
        changed = False

        # Process each production
        for prod in grammar.productions:
            lhs = prod.lhs
            rhs = prod.rhs

            # Compute FIRST of the right-hand side
            rhs_first = compute_first_of_string(rhs, first_sets)

            # Add to FIRST(lhs) if new symbols found
            old_size = len(first_sets[lhs])
            first_sets[lhs] = first_sets[lhs].union(rhs_first)
            new_size = len(first_sets[lhs])

            if new_size > old_size:
                changed = True

    return first_sets


def compute_first_of_string(symbols: List[Symbol],
                            first_sets: Dict[Symbol, Set[Symbol]]) -> Set[Symbol]:
    """
    Compute FIRST set of a string (sequence of symbols).

    FIRST(X₁X₂...Xₙ) algorithm:
    - Add FIRST(X₁) - {ε} to result
    - If ε ∈ FIRST(X₁), add FIRST(X₂) - {ε}
    - Continue while ε ∈ FIRST(Xᵢ)
    - If all symbols can derive ε, add ε to result

    Args:
        symbols: List of symbols
        first_sets: FIRST sets for individual symbols

    Returns:
        Set of symbols that can begin strings derived from this sequence

    Examples:
        FIRST(aB) = {a}
        FIRST(AB) where A→ε, B→b = {ε, b}
    """
    result = set()

    if not symbols:
        # Empty string produces epsilon
        return {EPSILON}

    # Process each symbol in sequence
    all_have_epsilon = True
    for symbol in symbols:
        # Get FIRST set of current symbol
        symbol_first = first_sets.get(symbol, set())

        # Add everything except epsilon
        result = result.union(symbol_first - {EPSILON})

        # If this symbol cannot derive epsilon, stop here
        if EPSILON not in symbol_first:
            all_have_epsilon = False
            break

    # If all symbols can derive epsilon, add epsilon to result
    if all_have_epsilon:
        result.add(EPSILON)

    return result


def compute_follow_sets(grammar: Grammar,
                       first_sets: Dict[Symbol, Set[Symbol]]) -> Dict[Symbol, Set[Symbol]]:
    """
    Compute FOLLOW sets for all nonterminals in the grammar.

    FOLLOW(A) = set of terminals that can immediately follow A in derivations

    Algorithm:
    1. FOLLOW(S) contains $ (end marker)
    2. For production A → αBβ:
       - Add FIRST(β) - {ε} to FOLLOW(B)
       - If ε ∈ FIRST(β) or β is empty, add FOLLOW(A) to FOLLOW(B)
    3. Repeat until no changes

    Args:
        grammar: The grammar to analyze
        first_sets: FIRST sets (computed by compute_first_sets)

    Returns:
        Dictionary mapping nonterminals to their FOLLOW sets

    Examples:
        For grammar:
            S → AB
            A → a
            B → b
        Results:
            FOLLOW(S) = {$}
            FOLLOW(A) = {b}
            FOLLOW(B) = {$}
    """
    follow_sets = {}

    # Step 1: Initialize all FOLLOW sets as empty
    for nonterminal in grammar.nonterminals:
        follow_sets[nonterminal] = set()

    # Step 2: Add $ to FOLLOW(start_symbol)
    follow_sets[grammar.start_symbol].add(END_MARKER)

    # Step 3: Fixed-point iteration
    changed = True
    while changed:
        changed = False

        # Process each production
        for prod in grammar.productions:
            lhs = prod.lhs
            rhs = prod.rhs

            # Process each position in the RHS
            for i, symbol in enumerate(rhs):
                # Only process nonterminals
                if not symbol.is_nonterminal():
                    continue

                # Get the rest of the string after this symbol
                beta = rhs[i + 1:]

                # Compute FIRST(beta)
                first_beta = compute_first_of_string(beta, first_sets)

                # Add FIRST(beta) - {ε} to FOLLOW(symbol)
                old_size = len(follow_sets[symbol])
                follow_sets[symbol] = follow_sets[symbol].union(
                    first_beta - {EPSILON}
                )

                # If beta can derive epsilon (or is empty),
                # add FOLLOW(lhs) to FOLLOW(symbol)
                if not beta or EPSILON in first_beta:
                    follow_sets[symbol] = follow_sets[symbol].union(
                        follow_sets[lhs]
                    )

                new_size = len(follow_sets[symbol])
                if new_size > old_size:
                    changed = True

    return follow_sets


def print_first_sets(first_sets: Dict[Symbol, Set[Symbol]], grammar: Grammar):
    """
    Print FIRST sets in a readable format.

    Args:
        first_sets: Dictionary of FIRST sets
        grammar: The grammar (for getting nonterminals)
    """
    print("\nFIRST sets:")
    for nt in sorted(grammar.nonterminals):
        first_set = first_sets.get(nt, set())
        print(f"FIRST({nt}) = {print_symbol_set(first_set)}")


def print_follow_sets(follow_sets: Dict[Symbol, Set[Symbol]], grammar: Grammar):
    """
    Print FOLLOW sets in a readable format.

    Args:
        follow_sets: Dictionary of FOLLOW sets
        grammar: The grammar (for getting nonterminals)
    """
    print("\nFOLLOW sets:")
    for nt in sorted(grammar.nonterminals):
        follow_set = follow_sets.get(nt, set())
        print(f"FOLLOW({nt}) = {print_symbol_set(follow_set)}")
