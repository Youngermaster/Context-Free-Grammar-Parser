# Architecture Overview

## System Architecture

```plaintext
┌─────────────────────────────────────────────────────────────┐
│                         main.py                              │
│                    (Entry Point)                             │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                         cli.py                               │
│            (Command-Line Interface)                          │
│  - Read input                                                │
│  - Interactive mode                                          │
│  - Output results                                            │
└───┬─────────────────────┬──────────────────────┬────────────┘
    │                     │                      │
    ▼                     ▼                      ▼
┌─────────┐         ┌──────────┐         ┌──────────┐
│grammar.py│         │  ll1.py  │         │ slr1.py  │
│  Parser │         │ LL(1)    │         │ SLR(1)   │
│         │         │  Parser  │         │  Parser  │
└────┬────┘         └────┬─────┘         └────┬─────┘
     │                   │                     │
     │              ┌────┴─────────────────────┘
     │              │
     ▼              ▼
┌─────────────────────────────────┐
│      first_follow.py            │
│   FIRST/FOLLOW Computation      │
│  - compute_first_sets()         │
│  - compute_follow_sets()        │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│          utils.py               │
│      Core Data Types            │
│  - Symbol class                 │
│  - Symbol operations            │
└─────────────────────────────────┘
```

## Data Flow

```plaintext
1. Input (stdin)
   ↓
2. cli.py reads grammar
   ↓
3. grammar.py parses text → Grammar object
   ↓
4. first_follow.py computes sets
   ↓
5. Try to build LL(1) parser
   ↓
6. Try to build SLR(1) parser
   ↓
7. Determine case (both/ll1/slr1/neither)
   ↓
8. Parse input strings
   ↓
9. Output results (stdout)
```

## Module Dependencies

```
main.py
  └── cli.py
       ├── grammar.py
       │    └── utils.py
       ├── first_follow.py
       │    ├── grammar.py
       │    └── utils.py
       ├── ll1.py
       │    ├── first_follow.py
       │    ├── grammar.py
       │    └── utils.py
       └── slr1.py
            ├── first_follow.py
            ├── grammar.py
            └── utils.py
```

## Class Hierarchy

```
Symbol (utils.py)
  ├── Terminal
  ├── Nonterminal
  ├── Epsilon
  └── EndMarker

Production (grammar.py)
  ├── lhs: Symbol
  └── rhs: List[Symbol]

Grammar (grammar.py)
  ├── productions: List[Production]
  ├── nonterminals: Set[Symbol]
  ├── terminals: Set[Symbol]
  ├── start_symbol: Symbol
  └── production_map: Dict[Symbol, List[Production]]

LL1Parser (ll1.py)
  ├── grammar: Grammar
  ├── first_sets: Dict[Symbol, Set[Symbol]]
  ├── follow_sets: Dict[Symbol, Set[Symbol]]
  ├── table: Dict[(Symbol, Symbol), Production]
  └── parse(string) → bool

LR0Item (slr1.py)
  ├── production: Production
  └── dot_pos: int

SLR1Parser (slr1.py)
  ├── grammar: Grammar
  ├── first_sets: Dict[Symbol, Set[Symbol]]
  ├── follow_sets: Dict[Symbol, Set[Symbol]]
  ├── states: List[FrozenSet[LR0Item]]
  ├── action_table: Dict[(int, Symbol), Action]
  ├── goto_table: Dict[(int, Symbol), int]
  └── parse(string) → bool
```

## Algorithm Flow

### LL(1) Parser Construction

```
1. Compute FIRST sets
   └── Fixed-point iteration over productions

2. Compute FOLLOW sets
   └── Fixed-point iteration using FIRST sets

3. Build parse table
   ├── For each production A → α:
   │   ├── For each terminal in FIRST(α):
   │   │   └── Add entry M[A, terminal] = A → α
   │   └── If ε ∈ FIRST(α):
   │       └── For each terminal in FOLLOW(A):
   │           └── Add entry M[A, terminal] = A → α
   └── Check for conflicts (raise NotLL1Exception)

4. Parse string
   ├── Initialize: stack = [$, S], input = string$
   └── Loop:
       ├── If top = input: pop both
       ├── If top is nonterminal: lookup table, expand
       └── Otherwise: reject
```

### SLR(1) Parser Construction

```
1. Build augmented grammar: S' → S

2. Compute FIRST and FOLLOW sets

3. Build LR(0) automaton
   ├── Initial item: [S' → •S]
   ├── Compute closure(initial)
   └── For each state:
       ├── For each symbol:
       │   ├── Compute GOTO(state, symbol)
       │   └── Add new state if not exists
       └── Record transitions

4. Build ACTION and GOTO tables
   ├── For each state:
   │   ├── For shift items [A → α•aβ]:
   │   │   └── ACTION[state, a] = shift next_state
   │   └── For reduce items [A → α•]:
   │       └── For each symbol in FOLLOW(A):
   │           └── ACTION[state, symbol] = reduce A → α
   └── Check for conflicts (raise NotSLR1Exception)

5. Parse string
   ├── Initialize: state_stack = [0], input = string$
   └── Loop:
       ├── Lookup ACTION[state, input]
       ├── If shift: push input and next state
       ├── If reduce: pop RHS, push LHS, GOTO
       ├── If accept: return true
       └── If error: return false
```

## Parsing Decision Tree

```
                    [Grammar Input]
                          |
                          ▼
                   Parse Grammar
                          |
                          ▼
              Compute FIRST/FOLLOW Sets
                          |
         ┌────────────────┴────────────────┐
         ▼                                 ▼
   Try Build LL(1)                   Try Build SLR(1)
         |                                 |
    ┌────┴────┐                      ┌────┴────┐
    ▼         ▼                      ▼         ▼
Success    Conflict              Success    Conflict
    |         |                      |         |
    └─────┬───┘                      └────┬────┘
          ▼                               ▼
      ll1_result                     slr1_result
          └────────┬──────────────────────┘
                   ▼
         ┌─────────┴──────────┐
         │  Classification     │
         └─────────┬───────────┘
                   ▼
    ┌──────────────┼──────────────┐
    ▼              ▼              ▼
Both LL(1)    LL(1) Only     SLR(1) Only     Neither
and SLR(1)         │              │              │
    │              ▼              ▼              ▼
    ▼         Use LL(1)      Use SLR(1)      Error
Interactive    Parser         Parser         Message
   Mode           │              │
    │             └──────┬───────┘
    ▼                    ▼
  Choose          Parse Strings
  Parser               │
    │                  ▼
    └────────────► Output yes/no
```

## Memory Layout

### Grammar Object

```
Grammar
├── productions: [P1, P2, P3, ...]
│   ├── P1: {lhs: S, rhs: [a, S, b]}
│   ├── P2: {lhs: S, rhs: [ε]}
│   └── ...
├── nonterminals: {S, A, B, ...}
├── terminals: {a, b, c, ...}
├── start_symbol: S
└── production_map: {
    S: [P1, P2],
    A: [P3, P4],
    ...
}
```

### LL(1) Parse Table

```
table: {
    (S, a): S → aSb,
    (S, ε): S → ε,
    (A, b): A → b,
    ...
}
```

### SLR(1) States

```
states: [
    State 0: {[S' → •S], [S → •aSb], [S → •ε]},
    State 1: {[S' → S•]},
    State 2: {[S → a•Sb], [S → •aSb], [S → •ε]},
    ...
]

action_table: {
    (0, a): shift 2,
    (0, $): reduce S → ε,
    (1, $): accept,
    ...
}

goto_table: {
    (0, S): 1,
    (2, S): 3,
    ...
}
```

## Performance Characteristics

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| Parse grammar | O(n) | O(n) |
| FIRST sets | O(n × m) | O(n × k) |
| FOLLOW sets | O(n × m) | O(n × k) |
| LL(1) table | O(n × t) | O(n × t) |
| LR(0) automaton | O(2^n) worst case | O(2^n) worst case |
| SLR(1) tables | O(s × t) | O(s × t) |
| LL(1) parsing | O(m) | O(m) |
| SLR(1) parsing | O(m) | O(m) |

Where:

- n = number of productions
- m = length of input string
- t = number of terminals
- k = average size of FIRST/FOLLOW sets
- s = number of LR(0) states

## Thread Safety

- **Current**: Single-threaded, not thread-safe
- **Parser objects**: Can be reused for multiple parses
- **Grammar objects**: Immutable after construction
- **Sets**: Computed once, reused

## Error Handling

```
Exception Hierarchy:
    Exception
    ├── ValueError
    │   └── (Parsing errors in grammar.py)
    ├── NotLL1Exception
    │   └── (LL(1) conflicts in ll1.py)
    └── NotSLR1Exception
        └── (SLR(1) conflicts in slr1.py)
```

## Extension Points

To add a new parser type:

1. Create new module (e.g., `lr1.py`)
2. Implement parser class with `parse()` method
3. Add exception class (e.g., `NotLR1Exception`)
4. Import in `cli.py`
5. Add to classification logic
6. Update documentation

---

This architecture prioritizes:
- **Modularity**: Easy to understand and modify
- **Clarity**: Clear data flow and dependencies
- **Extensibility**: Easy to add new features
- **Maintainability**: Well-documented and tested
