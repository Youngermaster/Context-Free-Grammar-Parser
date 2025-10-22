# Testing Documentation

## Test Organization

Following Rust best practices, tests are organized into separate files in the `tests/` directory, making the codebase cleaner and more maintainable.

## Test Structure

```
cfg-parser/
├── src/
│   ├── symbol.rs         # No inline tests
│   ├── grammar.rs        # No inline tests
│   ├── first_follow.rs   # No inline tests
│   ├── ll1.rs            # No inline tests
│   ├── slr1.rs           # No inline tests
│   └── ...
└── tests/
    ├── symbol_tests.rs         # Unit tests for symbol module
    ├── grammar_tests.rs        # Unit tests for grammar module
    ├── first_follow_tests.rs   # Unit tests for FIRST/FOLLOW
    ├── ll1_tests.rs            # Unit tests for LL(1) parser
    ├── slr1_tests.rs           # Unit tests for SLR(1) parser
    └── integration_tests.rs    # End-to-end integration tests
```

## Why Separate Test Files?

### Advantages

1. **Cleaner Source Files**: Implementation files are focused on logic, not tests
2. **Better Organization**: Tests are grouped by functionality
3. **Public API Testing**: Tests in `tests/` directory test the public API only
4. **Parallel Compilation**: Cargo can compile test files in parallel
5. **Easier Maintenance**: Finding and updating tests is simpler

### Rust Testing Conventions

Rust supports two types of tests:

**Unit Tests** (traditionally in same file):
- Test private functions and implementation details
- Use `#[cfg(test)] mod tests { ... }`
- Located at the bottom of source files

**Integration Tests** (in `tests/` directory):
- Test public API only
- Each file is compiled as a separate crate
- Located in `tests/` directory

We've chosen to use **integration-style tests** for all modules to keep source files clean.

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test File
```bash
cargo test --test symbol_tests
cargo test --test grammar_tests
cargo test --test integration_tests
```

### Run Specific Test Function
```bash
cargo test test_symbol_from_char
cargo test test_ll1_simple
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Tests in Parallel
```bash
cargo test -- --test-threads=4
```

## Test Coverage

### Symbol Tests (`symbol_tests.rs`)
- ✅ Symbol creation from characters
- ✅ Symbol type checking (terminal, nonterminal, epsilon, end marker)
- ✅ Symbol ordering and comparison
- ✅ String to symbols conversion
- ✅ Symbols to string conversion
- **Total: 6 tests**

### Grammar Tests (`grammar_tests.rs`)
- ✅ Simple grammar parsing
- ✅ Alternative productions parsing
- ✅ Empty grammar error handling
- ✅ Production retrieval
- ✅ Start symbol verification
- ✅ Epsilon production handling
- ✅ Complex grammar parsing
- **Total: 7 tests**

### FIRST/FOLLOW Tests (`first_follow_tests.rs`)
- ✅ FIRST set computation
- ✅ FOLLOW set computation
- ✅ FIRST sets with epsilon
- ✅ FIRST of string sequences
- ✅ FOLLOW set propagation
- ✅ Complex FIRST/FOLLOW scenarios
- **Total: 6 tests**

### LL(1) Parser Tests (`ll1_tests.rs`)
- ✅ Basic LL(1) parser construction
- ✅ Valid string acceptance
- ✅ Invalid string rejection
- ✅ Conflict detection (non-LL(1) grammars)
- ✅ Epsilon production handling
- **Total: 5 tests**

### SLR(1) Parser Tests (`slr1_tests.rs`)
- ✅ Basic SLR(1) parser construction
- ✅ Valid expression acceptance
- ✅ Invalid expression rejection
- ✅ Conflict detection
- ✅ Operator precedence (via grammar structure)
- **Total: 5 tests**

### Integration Tests (`integration_tests.rs`)
- ✅ Example 1: SLR(1) only grammar
- ✅ Example 2: Both LL(1) and SLR(1)
- ✅ Example 3: Neither LL(1) nor SLR(1)
- ✅ Complex expression parsing
- ✅ Epsilon productions end-to-end
- **Total: 5 tests**

## Total Test Count

**34 tests** covering all major functionality

## Test Output Example

```
running 34 tests
test symbol_tests::test_epsilon_special_case ... ok
test symbol_tests::test_string_conversion ... ok
test symbol_tests::test_symbol_equality ... ok
test symbol_tests::test_symbol_from_char ... ok
test symbol_tests::test_symbol_ordering ... ok
test symbol_tests::test_symbols_to_string ... ok
test grammar_tests::test_complex_grammar ... ok
test grammar_tests::test_empty_grammar_error ... ok
test grammar_tests::test_epsilon_production ... ok
test grammar_tests::test_get_productions ... ok
test grammar_tests::test_parse_alternatives ... ok
test grammar_tests::test_parse_simple_grammar ... ok
test grammar_tests::test_start_symbol ... ok
test first_follow_tests::test_complex_first_follow ... ok
test first_follow_tests::test_first_of_string ... ok
test first_follow_tests::test_first_sets_simple ... ok
test first_follow_tests::test_first_with_epsilon ... ok
test first_follow_tests::test_follow_propagation ... ok
test first_follow_tests::test_follow_sets_simple ... ok
test ll1_tests::test_ll1_accepts_valid_strings ... ok
test ll1_tests::test_ll1_conflict_detection ... ok
test ll1_tests::test_ll1_epsilon_production ... ok
test ll1_tests::test_ll1_rejects_invalid_strings ... ok
test ll1_tests::test_ll1_simple ... ok
test slr1_tests::test_slr1_accepts_valid_expressions ... ok
test slr1_tests::test_slr1_conflict_detection ... ok
test slr1_tests::test_slr1_operator_precedence ... ok
test slr1_tests::test_slr1_rejects_invalid_expressions ... ok
test slr1_tests::test_slr1_simple ... ok
test integration_tests::test_complex_expression_parsing ... ok
test integration_tests::test_epsilon_productions ... ok
test integration_tests::test_example1_slr1_grammar ... ok
test integration_tests::test_example2_both_ll1_and_slr1 ... ok
test integration_tests::test_example3_neither_ll1_nor_slr1 ... ok

test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Continuous Integration

Tests can be easily integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run tests
  run: cargo test --all-features

- name: Run tests with coverage
  run: cargo tarpaulin --out Xml
```

## Adding New Tests

To add new tests:

1. Create or open the appropriate test file in `tests/`
2. Add a new `#[test]` function
3. Use descriptive test names (e.g., `test_grammar_validates_input`)
4. Follow the AAA pattern: Arrange, Act, Assert

Example:

```rust
#[test]
fn test_new_feature() {
    // Arrange
    let input = create_test_data();

    // Act
    let result = function_under_test(input);

    // Assert
    assert_eq!(result, expected_value);
}
```

## Test Naming Conventions

- Use `test_` prefix for all test functions
- Use descriptive names that explain what is being tested
- Group related tests in the same file
- Keep test names concise but clear

Good examples:
- `test_symbol_from_char`
- `test_ll1_accepts_valid_strings`
- `test_slr1_conflict_detection`

## Best Practices

1. **One Assertion Per Test**: When possible, test one thing at a time
2. **Clear Test Names**: Names should explain what's being tested
3. **Use Helper Functions**: Reduce duplication with test helpers
4. **Test Edge Cases**: Include boundary conditions and error cases
5. **Keep Tests Fast**: Unit tests should run in milliseconds
6. **Independent Tests**: Tests should not depend on each other
7. **Readable Assertions**: Use descriptive assertion messages

## Documentation Tests

Rust also supports documentation tests in code comments:

```rust
/// Converts a character to a symbol.
///
/// # Examples
/// ```
/// use cfg_parser::symbol::Symbol;
/// let sym = Symbol::from_char('A');
/// assert!(sym.is_nonterminal());
/// ```
pub fn from_char(c: char) -> Self { ... }
```

These are automatically tested with `cargo test --doc`.
