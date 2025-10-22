//! Unit tests for the symbol module

use cfg_parser::symbol::*;

#[test]
fn test_symbol_from_char() {
    assert!(Symbol::from_char('A').is_nonterminal());
    assert!(Symbol::from_char('a').is_terminal());
    assert!(Symbol::from_char('e').is_epsilon());
    assert!(Symbol::from_char('$').is_end_marker());
    assert!(Symbol::from_char('+').is_terminal());
}

#[test]
fn test_symbol_ordering() {
    assert!(Symbol::Epsilon < Symbol::Terminal('a'));
    assert!(Symbol::Terminal('a') < Symbol::Nonterminal('A'));
    assert!(Symbol::Nonterminal('A') < Symbol::EndMarker);
}

#[test]
fn test_string_conversion() {
    let symbols = string_to_symbols("AaB");
    assert_eq!(symbols.len(), 3);
    assert!(symbols[0].is_nonterminal());
    assert!(symbols[1].is_terminal());
    assert!(symbols[2].is_nonterminal());
}

#[test]
fn test_symbols_to_string() {
    let symbols = vec![
        Symbol::Nonterminal('S'),
        Symbol::Terminal('a'),
        Symbol::EndMarker,
    ];
    let s = symbols_to_string(&symbols);
    assert_eq!(s, "Sa$");
}

#[test]
fn test_symbol_equality() {
    assert_eq!(Symbol::Terminal('a'), Symbol::Terminal('a'));
    assert_ne!(Symbol::Terminal('a'), Symbol::Terminal('b'));
    assert_ne!(Symbol::Terminal('a'), Symbol::Nonterminal('A'));
}

#[test]
fn test_epsilon_special_case() {
    let epsilon = Symbol::from_char('e');
    assert!(epsilon.is_epsilon());
    assert!(!epsilon.is_terminal());
    assert!(!epsilon.is_nonterminal());
}
