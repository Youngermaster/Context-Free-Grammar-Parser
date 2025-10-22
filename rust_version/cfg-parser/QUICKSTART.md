# Quick Start Guide

## Installation (One-Time Setup)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Build

```bash
cd rust_version/cfg-parser
cargo build --release
```

## Run Examples

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

### Example 3: Neither
```bash
cat <<'EOF' | cargo run --release
2
S -> A
A -> A b
EOF
```

## Testing

```bash
cargo test
```

## Documentation

```bash
cargo doc --open
```

## Performance

The release build uses aggressive optimizations:
- LTO (Link-Time Optimization)
- Optimization level 3
- Single codegen unit for maximum inlining

Typical parsing time for grammars: < 1ms
