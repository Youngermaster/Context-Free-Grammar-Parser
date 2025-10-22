# Architecture Overview

## Project Statistics

- **Total Lines of Code**: ~1,400 lines
- **Modules**: 8
- **Tests**: 10 unit tests (all passing)
- **Dependencies**: 1 (thiserror)
- **Rust Edition**: 2024

## Module Dependency Graph

```
main.rs
  └─> cli.rs
       ├─> grammar.rs
       │    ├─> symbol.rs
       │    └─> error.rs
       ├─> first_follow.rs
       │    ├─> grammar.rs
       │    └─> symbol.rs
       ├─> ll1.rs
       │    ├─> grammar.rs
       │    ├─> first_follow.rs
       │    ├─> symbol.rs
       │    └─> error.rs
       └─> slr1.rs
            ├─> grammar.rs
            ├─> first_follow.rs
            ├─> symbol.rs
            └─> error.rs
```

## Module Breakdown

### 1. `symbol.rs` (~190 lines)
**Purpose**: Core symbol types and utilities

**Key Types**:
- `Symbol` enum: Terminal, Nonterminal, Epsilon, EndMarker
- Implements: Clone, Copy, PartialEq, Eq, Hash, Ord, Display

**Functions**:
- `from_char()`: Convert character to symbol
- `string_to_symbols()`: Parse string to symbol vector
- `symbols_to_string()`: Convert symbols back to string

**Design Decisions**:
- Uses `const fn` for zero-cost abstractions
- Custom `Ord` implementation for consistent sorting
- Extensive inline annotations for optimization

### 2. `error.rs` (~50 lines)
**Purpose**: Centralized error handling

**Key Types**:
- `GrammarError` enum with thiserror derive
- `Result<T>` type alias

**Error Variants**:
- InvalidFormat, InvalidProduction, EmptyInput
- LL1Conflict, SLR1ShiftReduceConflict, SLR1ReduceReduceConflict
- IO errors (wrapped)

**Design Decisions**:
- Uses `thiserror` for ergonomic error messages
- Structured errors with context (state, symbol, productions)

### 3. `grammar.rs` (~250 lines)
**Purpose**: Grammar representation and parsing

**Key Types**:
- `Production`: LHS nonterminal + RHS symbol sequence
- `Grammar`: Complete CFG with productions, symbols, maps

**Functions**:
- `parse()`: Parse grammar from input lines
- `parse_production_line()`: Parse single production
- `get_productions()`: Get all productions for a nonterminal

**Design Decisions**:
- Uses `HashMap` for O(1) production lookup
- Separates parsing from internal representation
- Validates grammar structure during parsing

### 4. `first_follow.rs` (~220 lines)
**Purpose**: FIRST and FOLLOW set computation

**Key Functions**:
- `compute_first_sets()`: Fixed-point iteration for FIRST
- `compute_follow_sets()`: Fixed-point iteration for FOLLOW
- `first_of_string()`: FIRST of symbol sequence

**Algorithm**:
1. Initialize with terminals and epsilon
2. Iterate until no changes (fixed-point)
3. Handle epsilon productions correctly

**Design Decisions**:
- Uses `HashMap<Symbol, HashSet<Symbol>>` for sets
- Immutable iteration with cloning (safe, idiomatic Rust)
- Comprehensive comments explaining algorithm steps

### 5. `ll1.rs` (~180 lines)
**Purpose**: LL(1) predictive parser

**Key Types**:
- `LL1Parser`: Parser with table, grammar, FIRST/FOLLOW sets

**Functions**:
- `build()`: Construct LL(1) parser, detect conflicts
- `parse()`: Stack-based predictive parsing

**Algorithm**:
1. Build parse table from FIRST/FOLLOW
2. Detect LL(1) conflicts (multiple entries in cell)
3. Stack-based parsing with table lookups

**Design Decisions**:
- Uses `HashMap<(Symbol, Symbol), Production>` for table
- Returns `Result` to propagate conflicts
- Efficient stack-based parsing (no recursion)

### 6. `slr1.rs` (~370 lines)
**Purpose**: SLR(1) bottom-up parser

**Key Types**:
- `Item`: LR(0) item (production + dot position)
- `ItemSet`: Set of items (state)
- `Action`: Shift, Reduce, Accept
- `SLR1Parser`: Parser with states, ACTION/GOTO tables

**Functions**:
- `build()`: Construct SLR(1) parser
- `closure()`: Compute closure of item set
- `goto()`: Compute goto(I, X)
- `build_lr0_automaton()`: Build LR(0) states
- `build_tables()`: Build ACTION/GOTO tables
- `parse()`: Shift-reduce parsing

**Algorithm**:
1. Augment grammar with S' → S
2. Build LR(0) automaton (states and transitions)
3. Build ACTION table using FOLLOW sets
4. Build GOTO table for nonterminals
5. Parse using stack-based shift-reduce

**Design Decisions**:
- Uses state indices instead of hashing ItemSets
- Detects shift/reduce and reduce/reduce conflicts
- Efficient state comparison with vector search

### 7. `cli.rs` (~140 lines)
**Purpose**: Command-line interface

**Functions**:
- `run()`: Main CLI entry point
- `read_grammar()`: Read grammar from stdin
- `parse_strings()`: Parse input strings
- `interactive_mode()`: Handle both LL(1) and SLR(1)

**Flow**:
1. Read grammar input
2. Compute FIRST/FOLLOW sets
3. Try building LL(1) and SLR(1) parsers
4. Route to appropriate mode based on results

**Design Decisions**:
- Generic over `BufRead` for testability
- Handles all 4 cases from specification
- Clean error propagation with `?` operator

### 8. `main.rs` (~35 lines)
**Purpose**: Entry point

**Responsibilities**:
- Module declarations
- Documentation
- Error handling at top level
- Exit code on failure

## Data Flow

```
Input → Grammar Parsing → FIRST/FOLLOW → Parser Construction → String Parsing → Output
         (grammar.rs)     (first_follow.rs)  (ll1.rs/slr1.rs)   (ll1.rs/slr1.rs)
```

## Key Design Patterns

### 1. **Builder Pattern**
```rust
LL1Parser::build(grammar, first_sets, follow_sets) -> Result<LL1Parser>
```

### 2. **Result-Based Error Handling**
```rust
pub fn parse(lines: &[String]) -> Result<Grammar>
```

### 3. **Type Safety**
```rust
enum Symbol { Terminal(char), Nonterminal(char), ... }
```

### 4. **Separation of Concerns**
- Parsing → `grammar.rs`
- Analysis → `first_follow.rs`
- Parsing strategies → `ll1.rs`, `slr1.rs`
- I/O → `cli.rs`

### 5. **Immutability by Default**
Most data structures are immutable; mutations are explicit and local.

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Grammar Parsing | O(n) | n = input size |
| FIRST Computation | O(n²) worst | Fixed-point, usually faster |
| FOLLOW Computation | O(n²) worst | Fixed-point, usually faster |
| LL(1) Table Build | O(np) | n = nonterminals, p = productions |
| SLR(1) Automaton | O(s²t) | s = states, t = terminals |
| LL(1) Parse | O(n) | n = input length |
| SLR(1) Parse | O(n) | n = input length |

## Memory Usage

- **Grammar**: O(p) where p = number of productions
- **FIRST/FOLLOW**: O(n²) where n = number of symbols
- **LL(1) Table**: O(nt) where n = nonterminals, t = terminals
- **SLR(1) States**: O(s×i) where s = states, i = items per state

## Testing Strategy

### Unit Tests
- Symbol conversions and ordering
- Grammar parsing with valid/invalid input
- FIRST/FOLLOW set computation
- LL(1) parser construction and parsing
- SLR(1) parser construction and parsing
- CLI input reading

### Integration Tests
Tested via examples in README:
- Example 1: SLR(1) only
- Example 2: Both LL(1) and SLR(1)
- Example 3: Neither

## Build Optimizations

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true           # Link-Time Optimization
codegen-units = 1    # Better inlining
```

## Future Improvements

1. **Performance**
   - Use `FxHashMap` for faster hashing
   - Intern symbols for cache locality
   - Parallel FIRST/FOLLOW computation

2. **Features**
   - LR(1) and LALR(1) parsers
   - Parse tree generation
   - Error recovery in parsing
   - Grammar simplification

3. **Usability**
   - Pretty-print parse tables
   - Visualization of automata
   - Interactive debugger

4. **Code Quality**
   - Property-based testing with proptest
   - Benchmarking with criterion
   - Fuzz testing

## Comparison with OCaml

| Aspect | Rust | OCaml |
|--------|------|-------|
| Lines of Code | ~1,400 | ~600 |
| Memory Safety | Compile-time (ownership) | Runtime (GC) |
| Type Inference | Partial | Full |
| Error Handling | Result<T, E> | Exceptions |
| Pattern Matching | Exhaustive | Exhaustive |
| Performance | No runtime, faster | GC overhead |
| Concurrency | Safe by default | Requires libraries |

The Rust version is more verbose due to:
- Explicit error handling (Result types)
- More detailed type annotations
- Comprehensive documentation
- Additional safety checks

But gains:
- Better performance
- Memory safety guarantees
- Modern tooling (Cargo)
- Industry adoption
