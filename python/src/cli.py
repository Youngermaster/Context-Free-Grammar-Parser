"""
Command-line interface for the grammar parser application.

This module handles:
1. Reading grammar input from stdin
2. Building LL(1) and SLR(1) parsers
3. Interactive mode when both parsers succeed
4. Parsing input strings and outputting results
"""

import sys
from typing import Optional
from .grammar import parse_grammar
from .first_follow import compute_first_sets, compute_follow_sets
from .ll1 import LL1Parser, NotLL1Exception
from .slr1 import SLR1Parser, NotSLR1Exception


def read_grammar_input():
    """
    Read grammar lines from standard input.

    Expected format:
        Line 1: n (number of production lines)
        Lines 2 to n+1: Production rules

    Returns:
        List of input lines

    Examples:
        3
        S -> S+T T
        T -> T*F F
        F -> (S) i
    """
    lines = []
    try:
        # Read first line (number of productions)
        first_line = input().strip()
        lines.append(first_line)

        n = int(first_line)

        # Read n production lines
        for _ in range(n):
            line = input()
            lines.append(line)

    except EOFError:
        pass
    except ValueError as e:
        print(f"Error reading input: {e}", file=sys.stderr)
        sys.exit(1)

    return lines


def parse_strings_until_empty(parse_function):
    """
    Read strings from stdin and parse them until an empty line.

    For each string, output "yes" if accepted, "no" if rejected.

    Args:
        parse_function: Function that takes a string and returns True/False
    """
    while True:
        try:
            line = input()
            trimmed = line.strip()

            # Empty line = stop parsing
            if not trimmed:
                break

            # Parse the string
            result = parse_function(trimmed)
            print("yes" if result else "no")

        except EOFError:
            break


def interactive_mode(ll1_parser: LL1Parser, slr1_parser: SLR1Parser):
    """
    Interactive mode when grammar is both LL(1) and SLR(1).

    Prompts user to select parser:
    - T or t: Use LL(1) parser
    - B or b: Use SLR(1) parser
    - Q or q: Quit

    Args:
        ll1_parser: The LL(1) parser
        slr1_parser: The SLR(1) parser
    """
    while True:
        try:
            print("Select a parser (T: for LL(1), B: for SLR(1), Q: quit):")
            choice = input().strip()

            if choice in ['Q', 'q']:
                break

            elif choice in ['T', 't']:
                # Use LL(1) parser
                parse_strings_until_empty(ll1_parser.parse)

            elif choice in ['B', 'b']:
                # Use SLR(1) parser
                parse_strings_until_empty(slr1_parser.parse)

            else:
                # Invalid choice - prompt again
                continue

        except EOFError:
            break


def run():
    """
    Main CLI logic.

    Workflow:
    1. Read grammar from stdin
    2. Compute FIRST and FOLLOW sets
    3. Try to build LL(1) parser
    4. Try to build SLR(1) parser
    5. Determine which case applies:
       - Case 1: Both LL(1) and SLR(1) → Interactive mode
       - Case 2: LL(1) only → Parse with LL(1)
       - Case 3: SLR(1) only → Parse with SLR(1)
       - Case 4: Neither → Output error message
    """
    try:
        # Step 1: Read grammar input
        lines = read_grammar_input()
        grammar = parse_grammar(lines)

        # Step 2: Compute FIRST and FOLLOW sets
        first_sets = compute_first_sets(grammar)
        follow_sets = compute_follow_sets(grammar, first_sets)

        # Step 3: Try to build LL(1) parser
        ll1_parser = None
        try:
            ll1_parser = LL1Parser(grammar, first_sets, follow_sets)
        except NotLL1Exception:
            pass  # Grammar is not LL(1)

        # Step 4: Try to build SLR(1) parser
        slr1_parser = None
        try:
            slr1_parser = SLR1Parser(grammar, first_sets, follow_sets)
        except NotSLR1Exception:
            pass  # Grammar is not SLR(1)

        # Step 5: Determine which case and act accordingly
        if ll1_parser and slr1_parser:
            # Case 1: Both LL(1) and SLR(1)
            interactive_mode(ll1_parser, slr1_parser)

        elif ll1_parser:
            # Case 2: LL(1) only
            print("Grammar is LL(1).")
            parse_strings_until_empty(ll1_parser.parse)

        elif slr1_parser:
            # Case 3: SLR(1) only
            print("Grammar is SLR(1).")
            parse_strings_until_empty(slr1_parser.parse)

        else:
            # Case 4: Neither LL(1) nor SLR(1)
            print("Grammar is neither LL(1) nor SLR(1).")

    except ValueError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

    except Exception as e:
        print(f"Unexpected error: {e}", file=sys.stderr)
        sys.exit(1)
