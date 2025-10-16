#!/bin/bash
# Generate OCaml HTML documentation using odoc

echo "========================================="
echo "Generating OCaml Documentation"
echo "========================================="
echo ""

# Set up opam environment
eval $(opam env)

# Check if odoc is installed
if ! command -v odoc &> /dev/null; then
    echo "odoc not found. Installing..."
    opam install -y odoc
fi

# Create docs directory
mkdir -p docs/ocaml

# Build the project first
echo "Building project..."
dune clean
dune build

# Generate documentation with dune
echo "Generating HTML documentation..."
dune build @doc 2>&1 || echo "Note: Using alternative documentation method..."

# Check multiple possible locations for generated docs
DOC_LOCATIONS=(
    "_build/default/_doc/_html"
    "_build/default/_doc"
    "_build/_doc/_html"
)

FOUND=false
for loc in "${DOC_LOCATIONS[@]}"; do
    if [ -d "$loc" ]; then
        echo "Found documentation in: $loc"
        echo "Copying documentation..."
        cp -r "$loc"/* docs/ocaml/
        FOUND=true
        break
    fi
done

if [ "$FOUND" = false ]; then
    echo "Documentation not found in standard locations."
    echo "Generating manually with ocamldoc..."

    # Generate docs manually for each module
    ocamldoc -html -d docs/ocaml \
        utils.mli utils.ml \
        grammar.mli grammar.ml \
        firstFollow.mli firstFollow.ml \
        ll1.mli ll1.ml \
        slr1.mli slr1.ml \
        cli.mli cli.ml \
        main.ml 2>&1 || echo "Manual generation also failed. See errors above."
fi

echo ""
echo "========================================="
echo "Documentation generated!"
echo "========================================="
echo ""
echo "Files in docs/ocaml:"
ls -lh docs/ocaml/ 2>/dev/null | head -10
echo ""
echo "Open docs/ocaml/index.html in your browser"
echo "Or run: cd docs/ocaml && python -m http.server 8000"
echo "Then visit: http://localhost:8000"
