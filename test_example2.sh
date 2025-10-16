#!/bin/bash
eval $(opam env)
cat <<'EOF' | dune exec ./main.exe
3
S -> AB
A -> aA d
B -> bBc e
T
d

EOF
