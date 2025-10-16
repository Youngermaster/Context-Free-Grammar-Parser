#!/bin/bash
# Test script to validate all examples

echo "========================================="
echo "Context-Free Grammar Parser - Test Suite"
echo "========================================="
echo ""

# Test 1: SLR(1) Grammar
echo "Test 1: Expression Grammar (SLR(1) only)"
echo "-----------------------------------------"
python main.py < examples/example1_slr1.txt
echo ""

# Test 2: Neither LL(1) nor SLR(1)
echo "Test 2: Left Recursive Grammar"
echo "-----------------------------------------"
python main.py < examples/example3_neither.txt
echo ""

# Test 3: Simple balanced grammar
echo "Test 3: Balanced Grammar (Both parsers)"
echo "-----------------------------------------"
cat <<'EOF' | python main.py
1
S -> aSb e
ab
aabb
abab

EOF

echo ""
echo "========================================="
echo "All tests completed!"
echo "========================================="
