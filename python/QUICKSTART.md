# Quick Start Guide

Get started with the Context-Free Grammar Parser in 5 minutes!

## Installation

**Requirements:** Python 3.8 or higher (no external dependencies needed)

```bash
# Clone or download the repository
cd python
```

## Your First Parse

### Step 1: Create a simple grammar file

Create a file called `my_grammar.txt`:

```
1
S -> aSb e
```

This grammar accepts strings like: `ab`, `aabb`, `aaabbb` (matching a's and b's)

### Step 2: Run the parser

```bash
python main.py < my_grammar.txt
```

### Step 3: Test some strings

The parser will wait for input. Type these strings (press Enter after each):

```
ab
aabb
aab

```

(Type an empty line to stop)

**Output:**
```
yes
yes
no
```

## Understanding the Output

- `Grammar is LL(1).` - Uses top-down parsing
- `Grammar is SLR(1).` - Uses bottom-up parsing
- Both - You choose which parser to use interactively
- Neither - Grammar cannot be parsed

## Try the Examples

```bash
# Test the expression grammar (SLR only)
python main.py < examples/example1_slr1.txt

# Test a grammar that works with both parsers
python main.py < examples/example4_ll1.txt
```

## Input Format Cheat Sheet

```
<number_of_production_lines>
<production_line_1>
<production_line_2>
...
<string_to_parse_1>
<string_to_parse_2>
...
<empty_line_to_stop>
```

### Grammar Rules

- **Start symbol**: Must be `S` (uppercase)
- **Nonterminals**: A, B, C, ... Z (uppercase letters)
- **Terminals**: a, b, c, +, *, (, ), etc. (NOT uppercase)
- **Epsilon** (empty): Write as `e`
- **Alternatives**: Separate with spaces

### Example

```
3
S -> S+T T
T -> T*F F
F -> (S) i
```

Means:
- S can produce: `S+T` OR `T`
- T can produce: `T*F` OR `F`
- F can produce: `(S)` OR `i`

This is a simple expression grammar!

## Common Patterns

### Balanced Parentheses
```
1
S -> (S)S e
```
Accepts: `()`, `(())`, `()()`, etc.

### a^n b^n (equal a's and b's)
```
1
S -> aSb e
```
Accepts: `ab`, `aabb`, `aaabbb`, etc.

### Simple Arithmetic
```
3
E -> E+T T
T -> T*F F
F -> (E) n
```
Accepts: `n`, `n+n`, `n*n`, `n+n*n`, `(n+n)*n`, etc.

## Troubleshooting

**"Grammar is neither LL(1) nor SLR(1)"**
- Your grammar has left recursion or ambiguity
- Try rewriting the grammar

**"Error: Invalid production format"**
- Check that you have exactly n production lines after the first line
- Make sure productions follow format: `A -> alternatives`

**Nothing happens after running**
- The program is waiting for input
- Type your grammar first, then strings to parse
- End with an empty line

## Next Steps

1. Read the [full README](README.md) for detailed explanations
2. Check the code in `src/` - it's heavily commented for learning!
3. Experiment with your own grammars
4. Try modifying the examples

## Need Help?

Look at the example files in `examples/` directory for working examples!

Happy parsing!
