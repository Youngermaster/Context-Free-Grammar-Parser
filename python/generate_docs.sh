#!/bin/bash
# Generate Python HTML documentation using pdoc3

echo "========================================="
echo "Generating Python Documentation"
echo "========================================="
echo ""

# Check if pdoc3 is installed
if ! command -v pdoc3 &> /dev/null; then
    echo "pdoc3 not found. Installing..."
    pip install pdoc3
fi

# Create docs directory
mkdir -p docs/python

# Generate HTML documentation
echo "Generating HTML documentation..."
pdoc3 --html --output-dir docs/python --force src

# Move the src folder contents up one level for cleaner URLs
if [ -d "docs/python/src" ]; then
    mv docs/python/src/* docs/python/
    rmdir docs/python/src
fi

echo ""
echo "========================================="
echo "Documentation generated successfully!"
echo "========================================="
echo ""
echo "Open docs/python/index.html in your browser"
echo "Or run: python -m http.server 8000 --directory docs/python"
echo "Then visit: http://localhost:8000"
