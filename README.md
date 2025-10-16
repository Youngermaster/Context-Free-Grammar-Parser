# Context-Free Grammar Parser - OCaml Implementation

An OCaml implementation of LL(1) and SLR(1) parsers for context-free grammars, as specified in the ST0270/SI2002 Formal Languages course project.

## Author

- Juan Manuel Young Hoyos

## Project Overview

This implementation provides:

1. Algorithms to compute FIRST and FOLLOW sets
2. LL(1) predictive parser (Top-Down)
3. SLR(1) parser (Bottom-Up)
4. Interactive CLI for grammar analysis and string parsing

The implementation leverages OCaml's strong type system, pattern matching, and functional programming paradigms to create a clean, maintainable, and efficient parser.

## System Requirements

- **Operating System**: macOS Darwin 25.0.0 (compatible with Linux)
- **OCaml Version**: 4.14+ or 5.x
- **Build System**: Dune 3.0+
- **Package Manager**: opam 2.0+

## Installation

### 1. Install OCaml and opam

```bash
# Install opam (OCaml package manager)
bash -c "sh <(curl -fsSL https://opam.ocaml.org/install.sh)"

# Initialize opam
opam init

# Install OCaml compiler
opam switch create 5.3.0
eval $(opam env)
```

### 2. Install Dependencies

```bash
# Install dune build system
opam install dune

# Install OCaml LSP and tools (optional, for development)
opam install ocaml-lsp-server odoc ocamlformat utop
```

### 3. Build the Project

```bash
cd ocaml-implementation
eval $(opam env)
dune build
```

## Running the Application

### Basic Usage

```bash
# From the ocaml-implementation directory
eval $(opam env)
dune exec ./main.exe
```

### With Input File

```bash
dune exec ./main.exe < input.txt
```

### Using Here-Document

```bash
cat <<'EOF' | dune exec ./main.exe
3
S -> S+T T
T -> T*F F
F -> (S) i
i+i
(i)
(i+i)*i)

EOF
```

## Input Format

```plaintext
n                              # number of non-terminals (n > 0)
<nonterminal> -> <alternatives separated by spaces>  # n lines
<string1>                      # strings to parse (optional)
<string2>
<empty line>                   # terminates parsing
```

### Grammar Conventions

- **Start symbol**: Always 'S'
- **Non-terminals**: Capital letters (A-Z)
- **Terminals**: NOT uppercase letters (lowercase, digits, symbols)
- **Epsilon**: Represented as 'e'
- **End marker**: '$' (automatically appended, not allowed as terminal)

## Output Behavior

### Case 1: Grammar is both LL(1) AND SLR(1)

```plaintext
Select a parser (T: for LL(1), B: for SLR(1), Q: quit):
```

- Enter `T` for LL(1), `B` for SLR(1), or `Q` to quit
- Parse strings until empty line, then re-prompt

### Case 2: Grammar is LL(1) only

```plaintext
Grammar is LL(1).
```

- Accepts strings until empty line, then terminates

### Case 3: Grammar is SLR(1) only

```plaintext
Grammar is SLR(1).
```

- Accepts strings until empty line, then terminates

### Case 4: Grammar is neither LL(1) nor SLR(1)

```plaintext
Grammar is neither LL(1) nor SLR(1).
```

- Terminates immediately

For valid grammars, outputs `yes` if string is accepted, `no` otherwise.

## Examples

### Example 1: SLR(1) Grammar

```bash
cat <<'EOF' | dune exec ./main.exe
3
S -> S+T T
T -> T*F F
F -> (S) i
i+i
(i)
(i+i)*i)

EOF
```

**Expected Output:**

```bash
Grammar is SLR(1).
yes
yes
no
```

### Example 2: Both LL(1) and SLR(1)

```bash
cat <<'EOF' | dune exec ./main.exe
3
S -> AB
A -> aA d
B -> bBc e
T
d
adbc
a

Q
EOF
```

**Expected Output:**

```bash
Select a parser (T: for LL(1), B: for SLR(1), Q: quit):
yes
yes
no
Select a parser (T: for LL(1), B: for SLR(1), Q: quit):
```

### Example 3: Neither LL(1) nor SLR(1)

```bash
cat <<'EOF' | dune exec ./main.exe
2
S -> A
A -> A b
EOF
```

**Expected Output:**

```plaintext
Grammar is neither LL(1) nor SLR(1).
```

## Project Structure

```bash
ocaml-implementation/
├── dune-project          # Dune project configuration
├── dune                  # Build configuration
├── main.ml               # Entry point
├── cli.ml/.mli           # Command-line interface
├── grammar.ml/.mli       # Grammar data structures
├── utils.ml/.mli         # Symbol types and utilities
├── firstFollow.ml/.mli   # FIRST/FOLLOW set computation
├── ll1.ml/.mli           # LL(1) predictive parser
├── slr1.ml/.mli          # SLR(1) bottom-up parser
└── README.md             # This file
```

## Implementation Highlights

### Type Safety

OCaml's algebraic data types provide compile-time guarantees:

```ocaml
type symbol =
  | Terminal of char
  | Nonterminal of char
  | Epsilon
  | EndMarker
```

### Functional Algorithms

FIRST and FOLLOW computation using immutable data structures and fixed-point iteration:

```ocaml
let rec iterate first_map =
  let new_first_map = compute_new_first first_map in
  if changed first_map new_first_map then
    iterate new_first_map
  else
    new_first_map
```

### Pattern Matching

Clean, exhaustive parsing logic:

```ocaml
match (ll1_result, slr1_result) with
| (Some ll1, Some slr1) -> interactive_mode ll1 slr1
| (Some ll1, None) -> print_endline "Grammar is LL(1)."
| (None, Some slr1) -> print_endline "Grammar is SLR(1)."
| (None, None) -> print_endline "Grammar is neither LL(1) nor SLR(1)."
```

## Comparison with Python Implementation

### Advantages of OCaml

1. **Type Safety**: Compile-time error detection vs runtime errors
2. **Immutability**: Prevents accidental state mutations
3. **Pattern Matching**: More concise and exhaustive than if/else chains
4. **Performance**: Compiled to native code, significantly faster
5. **Expressiveness**: Algebraic data types model grammar concepts naturally

### Code Conciseness

OCaml's type system and pattern matching reduce boilerplate:

- Python: ~500 lines
- OCaml: ~600 lines (including extensive comments and type signatures)

### Maintainability

- Strong static typing catches errors at compile time
- Type inference reduces verbosity while maintaining safety
- Module system provides clear separation of concerns

## Development

### Building

```bash
dune build
```

### Cleaning

```bash
dune clean
```

### Running Tests

```bash
# Test Example 1
cat <<'EOF' | dune exec ./main.exe
3
S -> S+T T
T -> T*F F
F -> (S) i
i+i

EOF

# Expected: Grammar is SLR(1). / yes
```

### Formatting (if ocamlformat installed)

```bash
dune build @fmt
```

## Troubleshooting

### "command not found: dune"

Run `eval $(opam env)` to set up the OCaml environment in your current shell.

### Build Errors

Make sure you have OCaml 4.14+ or 5.x installed:

```bash
ocaml --version
```

### Warning as Errors

The project is configured to suppress certain warnings (-w -32-27-33). If you want to see all warnings:

Edit `dune` file and remove the `(flags (:standard -w -32-27-33))` line.

## References

- Aho, Alfred V. et al. "Compilers: Principles, Techniques, and Tools" (2nd Edition). Addison-Wesley, 2006.
- [OCaml Manual](https://ocaml.org/manual/)
- [Dune Documentation](https://dune.readthedocs.io/)

## License

MIT License - See LICENSE file for details.
