# Context-Free Grammar Parser - Python Implementation

A clean, modular Python implementation of LL(1) and SLR(1) parsers for context-free grammars. Designed to be **beginner-friendly** and easy to understand, following industry best practices.

## Author

Juan Manuel Young Hoyos

## Project Overview

This implementation provides:

1. **FIRST and FOLLOW set computation** - Fundamental algorithms for parser construction
2. **LL(1) Predictive Parser** - Top-down parsing with predictive table
3. **SLR(1) Parser** - Bottom-up shift-reduce parsing with LR(0) automaton
4. **Interactive CLI** - User-friendly command-line interface

The code is structured to be:
- **Modular**: Each component is in its own file with clear responsibilities
- **Well-documented**: Extensive comments and docstrings explain every concept
- **Beginner-friendly**: Simple, clear code that avoids unnecessary complexity
- **Pythonic**: Follows Python best practices and conventions

## Requirements

- **Python 3.8+** (uses type hints and modern Python features)
- **No external dependencies** - Uses only Python standard library

## Project Structure

```
python/
├── main.py              # Entry point - run this file
├── src/
│   ├── __init__.py      # Package initialization
│   ├── utils.py         # Symbol types and utility functions
│   ├── grammar.py       # Grammar parsing and data structures
│   ├── first_follow.py  # FIRST/FOLLOW set computation
│   ├── ll1.py          # LL(1) predictive parser
│   ├── slr1.py         # SLR(1) bottom-up parser
│   └── cli.py          # Command-line interface
├── examples/           # Example input files
└── README.md          # This file
```

### Module Responsibilities

1. **utils.py** - Foundation
   - `Symbol` class: Represents terminals, nonterminals, epsilon, end marker
   - Symbol classification and conversion functions
   - Set operations for symbols

2. **grammar.py** - Grammar Representation
   - `Production` class: Represents a grammar rule (A → α)
   - `Grammar` class: Complete grammar with terminals, nonterminals, productions
   - `parse_grammar()`: Converts text input to Grammar object

3. **first_follow.py** - Set Computation
   - `compute_first_sets()`: Computes FIRST sets for all symbols
   - `compute_follow_sets()`: Computes FOLLOW sets for all nonterminals
   - Uses fixed-point iteration algorithm

4. **ll1.py** - Top-Down Parser
   - `LL1Parser` class: Predictive parser with parse table
   - Stack-based parsing algorithm
   - Conflict detection for non-LL(1) grammars

5. **slr1.py** - Bottom-Up Parser
   - `LR0Item` class: Represents items in the LR(0) automaton
   - `SLR1Parser` class: Shift-reduce parser with ACTION/GOTO tables
   - LR(0) automaton construction
   - Conflict detection for non-SLR(1) grammars

6. **cli.py** - User Interface
   - Input/output handling
   - Interactive mode when both parsers work
   - String parsing and result output

## Running the Application

### Basic Usage

```bash
cd python
python main.py
```

Then enter your grammar and strings interactively.

### With Input File

```bash
python main.py < input.txt
```

### Using Here-Document (macOS/Linux)

```bash
cat <<'EOF' | python main.py
3
S -> S+T T
T -> T*F F
F -> (S) i
i+i
(i)

EOF
```

## Input Format

```
n                              # Number of production lines (integer)
<nonterminal> -> <alternatives separated by spaces>  # n lines
<string1>                      # Strings to parse (optional)
<string2>
<empty line>                   # Empty line terminates input
```

### Grammar Conventions

- **Start symbol**: Always `S` (uppercase S)
- **Nonterminals**: Uppercase letters (A-Z)
- **Terminals**: Lowercase letters, digits, symbols (NOT uppercase)
- **Epsilon**: Represented as `e` in input
- **End marker**: `$` (automatically added, not allowed as terminal)
- **Alternatives**: Separated by spaces on the same line

### Example Grammar

```
3
S -> S+T T
T -> T*F F
F -> (S) i
```

This represents:
- S → S+T | T
- T → T*F | F
- F → (S) | i

## Output Behavior

The application detects which type of parser the grammar supports:

### Case 1: Grammar is both LL(1) AND SLR(1)

```
Select a parser (T: for LL(1), B: for SLR(1), Q: quit):
```

Enter:
- `T` or `t` - Use LL(1) parser
- `B` or `b` - Use SLR(1) parser
- `Q` or `q` - Quit

After selecting, enter strings to parse (empty line to return to menu).

### Case 2: Grammar is LL(1) only

```
Grammar is LL(1).
```

Automatically uses LL(1) parser. Enter strings until empty line.

### Case 3: Grammar is SLR(1) only

```
Grammar is SLR(1).
```

Automatically uses SLR(1) parser. Enter strings until empty line.

### Case 4: Grammar is neither LL(1) nor SLR(1)

```
Grammar is neither LL(1) nor SLR(1).
```

Program terminates.

### Parsing Results

For each input string:
- `yes` - String is accepted by the grammar
- `no` - String is rejected by the grammar

## Examples

### Example 1: SLR(1) Grammar (Expression Grammar)

**Input:**
```
3
S -> S+T T
T -> T*F F
F -> (S) i
i+i
(i)
(i+i)*i)

```

**Output:**
```
Grammar is SLR(1).
yes
yes
no
```

**Explanation:**
- `i+i` ✓ Valid: i + i
- `(i)` ✓ Valid: parenthesized i
- `(i+i)*i)` ✗ Invalid: mismatched parentheses

### Example 2: Both LL(1) and SLR(1)

**Input:**
```
3
S -> AB
A -> aA d
B -> bBc e
T
d
adbc
a

Q
```

**Output:**
```
Select a parser (T: for LL(1), B: for SLR(1), Q: quit):
yes
yes
no
Select a parser (T: for LL(1), B: for SLR(1), Q: quit):
```

**Explanation:**
- Grammar is both LL(1) and SLR(1)
- User selects LL(1) parser (T)
- Tests three strings
- User quits (Q)

### Example 3: Neither LL(1) nor SLR(1)

**Input:**
```
2
S -> A
A -> Ab a
```

**Output:**
```
Grammar is neither LL(1) nor SLR(1).
```

**Explanation:**
- Grammar has left recursion (A → Ab)
- Cannot be parsed by LL(1) or SLR(1)

## Understanding the Code

### How Symbols Work

```python
# utils.py
from src.utils import Symbol, char_to_symbol

# Create symbols
terminal_a = Symbol('a')          # Terminal: 'a'
nonterminal_S = Symbol('S')       # Nonterminal: 'S'
epsilon = Symbol('e', is_epsilon=True)  # Epsilon: ε
end_marker = Symbol('$', is_end_marker=True)  # End: $

# Convert character to symbol
sym = char_to_symbol('A')  # Automatically detects it's a nonterminal
```

### How Grammars Work

```python
# grammar.py
from src.grammar import Production, Grammar, parse_grammar

# Create a production: S → aA
prod = Production(
    lhs=Symbol('S'),
    rhs=[Symbol('a'), Symbol('A')]
)

# Parse grammar from text
lines = ["2", "S -> aA b", "A -> c"]
grammar = parse_grammar(lines)
```

### How FIRST/FOLLOW Sets Work

```python
# first_follow.py
from src.first_follow import compute_first_sets, compute_follow_sets

first_sets = compute_first_sets(grammar)
follow_sets = compute_follow_sets(grammar, first_sets)

# Access FIRST set of a nonterminal
first_S = first_sets[Symbol('S')]
```

### How Parsers Work

```python
# ll1.py and slr1.py
from src.ll1 import LL1Parser
from src.slr1 import SLR1Parser

# Create parsers
ll1_parser = LL1Parser(grammar, first_sets, follow_sets)
slr1_parser = SLR1Parser(grammar, first_sets, follow_sets)

# Parse strings
result = ll1_parser.parse("aac")  # Returns True or False
```

## Key Algorithms Explained

### FIRST Sets

**What**: Set of terminals that can start strings derived from a symbol.

**Algorithm**:
1. For terminals: FIRST(a) = {a}
2. For nonterminals: Iterate until no changes
   - For A → X₁X₂...Xₙ:
     - Add FIRST(X₁) - {ε}
     - If X₁ can derive ε, add FIRST(X₂) - {ε}
     - Continue while ε is possible

**Example**: For S → aB | c, FIRST(S) = {a, c}

### FOLLOW Sets

**What**: Set of terminals that can follow a nonterminal in derivations.

**Algorithm**:
1. Add $ to FOLLOW(start symbol)
2. Iterate until no changes
   - For A → αBβ:
     - Add FIRST(β) - {ε} to FOLLOW(B)
     - If β can derive ε, add FOLLOW(A) to FOLLOW(B)

### LL(1) Parsing

**What**: Top-down parsing using a stack and predictive table.

**Algorithm**:
1. Stack: [$, S], Input: string + $
2. Loop:
   - If top = input: pop both
   - If top is nonterminal: use table, replace with production
   - Otherwise: reject
3. Accept when stack = [$] and input = [$]

### SLR(1) Parsing

**What**: Bottom-up parsing using shift-reduce actions.

**Algorithm**:
1. Build LR(0) automaton (states and transitions)
2. Construct ACTION (shift/reduce/accept) and GOTO tables
3. Parse using stack of states:
   - Shift: Push input and next state
   - Reduce: Pop RHS, push LHS, goto next state
   - Accept/Error

## Testing

Create test files in the `examples/` directory:

```bash
# Test with example1.txt
python main.py < examples/example1.txt
```

## Best Practices Demonstrated

1. **Type Hints**: All functions have type annotations
2. **Docstrings**: Every class and function is documented
3. **Single Responsibility**: Each class/function has one clear purpose
4. **Descriptive Names**: Variable and function names explain their purpose
5. **Error Handling**: Clear exceptions and error messages
6. **Immutability**: Uses frozenset for hashable collections
7. **DRY Principle**: No repeated code, shared utilities
8. **Comments**: Complex algorithms are explained step-by-step

## Common Issues

### "Symbol not found" errors

Make sure:
- Start symbol is 'S' (uppercase)
- Nonterminals are uppercase (A-Z)
- Terminals are lowercase or symbols

### Parser conflicts

LL(1) conflicts occur when:
- Grammar has left recursion
- Multiple productions start with same symbol

SLR(1) conflicts occur when:
- Shift/reduce or reduce/reduce conflicts in ACTION table

### Import errors

Run from the `python/` directory:
```bash
cd python
python main.py
```

## Learning Resources

This implementation follows concepts from:
- **"Compilers: Principles, Techniques, and Tools"** (Dragon Book) by Aho, Lam, Sethi, and Ullman
- **Formal Languages and Automata Theory** courses (ST0270/SI2002)

## Comparison with OCaml Implementation

| Aspect | Python | OCaml |
|--------|--------|-------|
| Type Safety | Runtime (duck typing) | Compile-time (strong static) |
| Paradigm | Multi-paradigm | Functional-first |
| Verbosity | More explicit | More concise |
| Beginner-friendly | ✓✓✓ Very accessible | ✓✓ Requires FP knowledge |
| Performance | Interpreted | Compiled to native |
| Best for | Learning, prototyping | Production, correctness |

## Contributing

This is an educational project. Feel free to:
- Add more examples
- Improve documentation
- Add unit tests
- Optimize algorithms

## License

MIT License - See LICENSE file for details.

---

**Happy Parsing!** If you have questions about the code, check the inline comments and docstrings - they explain every step in detail.
