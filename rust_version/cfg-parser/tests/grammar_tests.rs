//! Unit tests for the grammar module

use cfg_parser::grammar::*;
use cfg_parser::symbol::Symbol;

#[test]
fn test_parse_simple_grammar() {
    let lines = vec![
        "2".to_string(),
        "S -> AB".to_string(),
        "A -> a".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    assert_eq!(grammar.all_productions().len(), 2);
    assert!(grammar.nonterminals().contains(&Symbol::Nonterminal('S')));
    assert!(grammar.terminals().contains(&Symbol::Terminal('a')));
}

#[test]
fn test_parse_alternatives() {
    let lines = vec!["1".to_string(), "S -> a b c".to_string()];

    let grammar = Grammar::parse(&lines).unwrap();
    assert_eq!(grammar.all_productions().len(), 3);
}

#[test]
fn test_empty_grammar_error() {
    let lines: Vec<String> = vec![];
    let result = Grammar::parse(&lines);
    assert!(result.is_err());
}

#[test]
fn test_get_productions() {
    let lines = vec![
        "2".to_string(),
        "S -> AB AC".to_string(),
        "A -> a".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let s_prods = grammar.get_productions(Symbol::Nonterminal('S'));
    assert_eq!(s_prods.len(), 2);
}

#[test]
fn test_start_symbol() {
    let lines = vec!["1".to_string(), "S -> a".to_string()];

    let grammar = Grammar::parse(&lines).unwrap();
    assert_eq!(grammar.start_symbol(), Symbol::Nonterminal('S'));
}

#[test]
fn test_epsilon_production() {
    let lines = vec!["1".to_string(), "S -> e".to_string()];

    let grammar = Grammar::parse(&lines).unwrap();
    let prods = grammar.get_productions(Symbol::Nonterminal('S'));
    assert_eq!(prods[0].rhs, vec![Symbol::Epsilon]);
}

#[test]
fn test_complex_grammar() {
    let lines = vec![
        "3".to_string(),
        "S -> S+T T".to_string(),
        "T -> T*F F".to_string(),
        "F -> (S) i".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    assert_eq!(grammar.all_productions().len(), 6);
    assert!(grammar.terminals().contains(&Symbol::Terminal('+')));
    assert!(grammar.terminals().contains(&Symbol::Terminal('*')));
    assert!(grammar.terminals().contains(&Symbol::Terminal('(')));
    assert!(grammar.terminals().contains(&Symbol::Terminal(')')));
    assert!(grammar.terminals().contains(&Symbol::Terminal('i')));
}
