# Context-Free Grammar Parser - Rust Implementation

A high-performance Rust implementation of LL(1) and SLR(1) parsers for context-free grammars, as specified in the ST0270/SI2002 Formal Languages course project.

## Author

- Juan Manuel Young Hoyos

## Project Overview

This implementation provides:

1. Algorithms to compute FIRST and FOLLOW sets
2. LL(1) predictive parser (Top-Down)
3. SLR(1) parser (Bottom-Up)
4. Interactive CLI for grammar analysis and string parsing

The implementation leverages Rust's strong type system, pattern matching, ownership model, and zero-cost abstractions to create a safe, performant, and maintainable parser.

## System Requirements

- **Operating System**: macOS Darwin 25.0.0 (compatible with Linux and Windows)
- **Rust Version**: 1.87.0+ (2024 edition)
- **Build System**: Cargo (included with Rust)

## Installation

### 1. Install Rust

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Configure your current shell
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2. Build the Project

```bash
cd rust_version/cfg-parser

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release
```

## Running the Application

### Basic Usage

```bash
# From the cfg-parser directory
cargo run --release
```

### With Input File

```bash
cargo run --release < input.txt
```

### Using Here-Document

```bash
cat <<'EOF' | cargo run --release
3
S -> S+T T
T -> T*F F
F -> (S) i
i+i
(i)
(i+i)*i)

EOF
```

### Using the Compiled Binary

```bash
# After building
./target/release/cfg_parser < input.txt
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
cat <<'EOF' | cargo run --release
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

```
Grammar is SLR(1).
yes
yes
no
```

### Example 2: Both LL(1) and SLR(1)

```bash
cat <<'EOF' | cargo run --release
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

```
Select a parser (T: for LL(1), B: for SLR(1), Q: quit):
yes
yes
no
Select a parser (T: for LL(1), B: for SLR(1), Q: quit):
```

### Example 3: Neither LL(1) nor SLR(1)

```bash
cat <<'EOF' | cargo run --release
2
S -> A
A -> A b
EOF
```

**Expected Output:**

```
Grammar is neither LL(1) nor SLR(1).
```

## Project Structure

```
rust_version/cfg-parser/
├── Cargo.toml           # Project configuration and dependencies
├── README.md            # This file
└── src/
    ├── main.rs          # Entry point
    ├── cli.rs           # Command-line interface
    ├── error.rs         # Error types and Result alias
    ├── symbol.rs        # Symbol types and utilities
    ├── grammar.rs       # Grammar data structures and parsing
    ├── first_follow.rs  # FIRST/FOLLOW set computation
    ├── ll1.rs           # LL(1) predictive parser
    └── slr1.rs          # SLR(1) bottom-up parser
```

## Implementation Highlights

### Type Safety

Rust's enums and pattern matching provide compile-time guarantees:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
    Terminal(char),
    Nonterminal(char),
    Epsilon,
    EndMarker,
}
```

### Memory Safety

Rust's ownership system ensures memory safety without garbage collection:

```rust
pub fn parse(&self, input: &str) -> bool {
    let mut input_symbols = string_to_symbols(input);
    input_symbols.push(Symbol::EndMarker);
    // No manual memory management needed
}
```

### Error Handling

Using `thiserror` for ergonomic error handling:

```rust
#[derive(Error, Debug)]
pub enum GrammarError {
    #[error("LL(1) conflict at M[{nonterminal}, {terminal}]")]
    LL1Conflict { /* ... */ },
    // ...
}
```

### Performance

- **Zero-cost abstractions**: High-level code compiles to efficient machine code
- **Stack allocation**: Most data structures use stack allocation for speed
- **Efficient collections**: Uses `HashMap` and `HashSet` for O(1) lookups
- **Release optimizations**: LTO and aggressive optimization enabled

## Comparison with OCaml Implementation

### Advantages of Rust

1. **Performance**: Compiles to native code with zero-cost abstractions
2. **Memory Safety**: Ownership system prevents memory bugs at compile time
3. **Concurrency**: Safe concurrent programming (not utilized here, but available)
4. **Modern Tooling**: Cargo provides excellent package management and build system
5. **No Runtime**: No garbage collector, predictable performance
6. **Rich Type System**: Enums with data, traits, and generics

### Code Quality

- Extensive use of Rust idioms (pattern matching, `Result` type, iterators)
- Comprehensive documentation comments for API documentation
- Unit tests for core functionality
- Type-driven development ensures correctness

### Lines of Code

- Rust implementation: ~800 lines (including extensive documentation)
- OCaml implementation: ~600 lines
- The difference is due to Rust's more explicit error handling and type annotations

## Development

### Building

```bash
# Debug build (faster compilation, slower execution)
cargo build

# Release build (slower compilation, optimized execution)
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Cleaning

```bash
cargo clean
```

### Generating Documentation

```bash
cargo doc --open
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

## Benchmarking

The release build is optimized for performance:

```bash
# Compare performance
time cat input.txt | ./target/release/cfg_parser
```

## Troubleshooting

### "cargo: command not found"

Make sure Rust is installed and your PATH is configured:

```bash
source $HOME/.cargo/env
```

### Build Errors

Ensure you have Rust 1.87.0 or later:

```bash
rustc --version
rustup update
```

### Running on Different Platforms

This code is platform-independent and should work on:
- macOS (ARM and Intel)
- Linux (various distributions)
- Windows (with WSL or native)

## Advanced Usage

### Custom Input Parsing

You can modify `cli.rs` to accept different input formats or add additional validation.

### Debugging

Use `RUST_BACKTRACE=1` for detailed error traces:

```bash
RUST_BACKTRACE=1 cargo run
```

### Profiling

For performance analysis:

```bash
cargo build --release
cargo flamegraph
```

## References

- Aho, Alfred V. et al. "Compilers: Principles, Techniques, and Tools" (2nd Edition). Addison-Wesley, 2006.
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

## License

MIT License - See LICENSE file for details.

## Comparison with Other Implementations

### Rust vs OCaml

| Feature | Rust | OCaml |
|---------|------|-------|
| Memory Safety | Compile-time (ownership) | Garbage collected |
| Performance | Native, no runtime | Native, with GC |
| Type System | Explicit, traits | Implicit, structural |
| Error Handling | `Result<T, E>` | Exceptions/Option |
| Concurrency | Safe, built-in | Libraries |
| Tooling | Cargo (excellent) | Dune, opam |
| Learning Curve | Steeper | Moderate |

### Why Rust?

1. **Zero-cost abstractions**: High-level features with low-level performance
2. **Memory safety**: No null pointers, no data races
3. **Modern ecosystem**: Great tools, active community
4. **Industry adoption**: Used in production at major companies
5. **Future-proof**: Language continues to evolve with backwards compatibility

## Contributing

This is a student project for the ST0270/SI2002 Formal Languages course. For educational purposes, feel free to study and learn from this implementation.
