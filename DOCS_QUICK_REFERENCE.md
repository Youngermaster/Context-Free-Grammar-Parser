# Documentation Quick Reference

## 🚀 One-Line Setup

```bash
./generate_all_docs.sh && open docs/index.html
```

## 📂 What Gets Generated

```
docs/
├── index.html                  ← START HERE! (Main portal)
├── python/                     ← Python API docs (7 modules)
│   ├── index.html
│   ├── utils.html
│   ├── grammar.html
│   ├── first_follow.html
│   ├── ll1.html
│   ├── slr1.html
│   └── cli.html
└── ocaml/                      ← OCaml API docs (6 modules)
    ├── index.html
    ├── Utils.html
    ├── Grammar.html
    ├── FirstFollow.html (with FirstFollow.mli content)
    ├── Ll1.html
    ├── Slr1.html
    └── Cli.html
```

## 🎯 Quick Commands

| Task | Command |
|------|---------|
| Generate all docs | `./generate_all_docs.sh` |
| Generate Python only | `cd python && ./generate_docs.sh` |
| Generate OCaml only | `./generate_ocaml_docs.sh` |
| View locally | `python -m http.server 8000 --directory docs` |
| Open in browser | `open docs/index.html` (macOS) |

## 📝 Comment Formats

### Python (uses docstrings)

```python
def my_function(param: str) -> bool:
    """
    One-line summary.

    Longer description with algorithm details.

    Args:
        param: Parameter description

    Returns:
        Return value description

    Examples:
        >>> my_function("test")
        True
    """
```

### OCaml (uses special comments)

```ocaml
(** One-line summary.

    Longer description with algorithm details.

    @param param Parameter description
    @return Return value description *)
val my_function : string -> bool
```

## 🔍 Documentation Tools

| Language | Tool | Version |
|----------|------|---------|
| Python | pdoc3 | >= 0.10.0 |
| OCaml | ocamldoc | Built-in |
| OCaml (advanced) | odoc | 3.1.0 |

## 📊 Coverage

**Python Documentation:**
- ✅ 7 modules - 100% documented
- ✅ 30+ functions with full docstrings
- ✅ 10+ classes with method documentation
- ✅ Type hints on all functions
- ✅ Examples in docstrings
- ✅ Algorithm explanations

**OCaml Documentation:**
- ✅ 6 modules with .mli interface files
- ✅ 50+ functions documented
- ✅ Complete type signatures
- ✅ Cross-module references
- ✅ Index pages (types, values, modules)

## 🌐 View Options

### Option 1: File Browser
```bash
open docs/index.html
```

### Option 2: Local Server (Recommended)
```bash
python -m http.server 8000 --directory docs
```
Then visit: http://localhost:8000

### Option 3: GitHub Pages
Push to GitHub and enable Pages in repository settings.

## 🎨 What You'll See

### Python Docs Features
- Search bar
- Syntax highlighted code
- Type hints displayed
- Source code links
- Mobile responsive
- Dark mode support

### OCaml Docs Features
- Clean, professional HTML
- Type signatures prominent
- Module hierarchy
- Cross-references
- Index by category
- Classic OCaml styling

## 🐛 Troubleshooting

**Python docs missing?**
```bash
pip install pdoc3
cd python && ./generate_docs.sh
cp -r python/docs/python ../docs/
```

**OCaml docs incomplete?**
```bash
# Make sure .mli files exist
ls *.mli
./generate_ocaml_docs.sh
```

**Index page broken?**
```bash
./generate_all_docs.sh  # Regenerates index
```

## 📖 Reading the Docs

### For Beginners
1. Start with **Python docs** (more explanatory)
2. Read module-level documentation first
3. Check function signatures and parameters
4. Look at examples in docstrings
5. Follow links to related functions

### For Advanced Users
1. **OCaml docs** show pure type signatures
2. Use index pages to find specific functions
3. Check cross-references between modules
4. Compare Python/OCaml implementations

## 📚 Module Guide

| Module | Purpose | Key Functions |
|--------|---------|---------------|
| utils | Symbol types | `Symbol`, `char_to_symbol`, `string_to_symbols` |
| grammar | Grammar parsing | `Production`, `Grammar`, `parse_grammar` |
| first_follow | Set computation | `compute_first_sets`, `compute_follow_sets` |
| ll1 | Top-down parser | `LL1Parser`, `parse`, `build_table` |
| slr1 | Bottom-up parser | `SLR1Parser`, `parse`, `_build_lr0_automaton` |
| cli | User interface | `run`, `parse_strings_until_empty` |

## 🔗 Links

- **Main Index**: `docs/index.html`
- **Python API**: `docs/python/index.html`
- **OCaml API**: `docs/ocaml/index.html`
- **Full Guide**: `DOCUMENTATION.md`

## ⚡ Pro Tips

1. **Regenerate after code changes** - Docs don't auto-update
2. **Check both implementations** - Python for explanation, OCaml for types
3. **Use search in Python docs** - Very helpful for finding functions
4. **Read module docs first** - Understand overall structure
5. **Look at examples** - Best way to learn usage

## 📦 Dependencies

**Python:**
```bash
pip install pdoc3  # That's it!
```

**OCaml:**
```bash
# ocamldoc is built-in with OCaml
# Optional: opam install odoc
```

## ✅ Verification

Check that everything generated:

```bash
# Should show ~28 files
find docs -type f | wc -l

# Should list all HTML files
ls docs/python/*.html
ls docs/ocaml/*.html

# Should open the main index
open docs/index.html
```

## 🎓 Learning Path

1. **Read**: `DOCUMENTATION.md` (this comprehensive guide)
2. **Generate**: `./generate_all_docs.sh`
3. **Open**: `docs/index.html`
4. **Explore**: Start with Python docs → utils module
5. **Compare**: Look at same module in OCaml docs
6. **Understand**: Read algorithm explanations
7. **Code**: Try implementing similar functions

---

**Generated**: Auto-updated on each build
**Tools**: pdoc3, ocamldoc
**Total Pages**: 28+ HTML files
**Coverage**: 100% of public API

Happy documenting! 📚
