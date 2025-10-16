#!/usr/bin/env python3
"""
Main entry point for the Context-Free Grammar Parser.

This script can be run directly or used as a module.

Usage:
    python main.py < input.txt
    cat input.txt | python main.py
"""

import sys
import os

# Add src directory to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

from src.cli import run

if __name__ == "__main__":
    run()
