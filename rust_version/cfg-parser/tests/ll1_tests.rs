//! Unit tests for LL(1) parser

use cfg_parser::first_follow::{compute_first_sets, compute_follow_sets};
use cfg_parser::grammar::Grammar;
use cfg_parser::ll1::LL1Parser;

#[test]
fn test_ll1_simple() {
    let lines = vec![
        "3".to_string(),
        "S -> AB".to_string(),
        "A -> aA d".to_string(),
        "B -> bBc e".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    let parser = LL1Parser::build(grammar, first_sets, follow_sets);
    assert!(parser.is_ok());

    let parser = parser.unwrap();
    assert!(parser.parse("d"));
    assert!(parser.parse("adbc"));
    assert!(!parser.parse("a"));
}

#[test]
fn test_ll1_accepts_valid_strings() {
    let lines = vec![
        "3".to_string(),
        "S -> AB".to_string(),
        "A -> aA d".to_string(),
        "B -> bBc e".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);
    let parser = LL1Parser::build(grammar, first_sets, follow_sets).unwrap();

    // Valid strings
    assert!(parser.parse("d"));
    assert!(parser.parse("ad"));
    assert!(parser.parse("aad"));
    assert!(parser.parse("dbc"));
    assert!(parser.parse("adbc"));
}

#[test]
fn test_ll1_rejects_invalid_strings() {
    let lines = vec![
        "3".to_string(),
        "S -> AB".to_string(),
        "A -> aA d".to_string(),
        "B -> bBc e".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);
    let parser = LL1Parser::build(grammar, first_sets, follow_sets).unwrap();

    // Invalid strings
    assert!(!parser.parse("a"));
    assert!(!parser.parse("b"));
    assert!(!parser.parse("abc"));
    assert!(!parser.parse("dd"));
}

#[test]
fn test_ll1_conflict_detection() {
    // This grammar is not LL(1) due to left recursion
    let lines = vec![
        "2".to_string(),
        "S -> Sa b".to_string(),
        "A -> a".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    let result = LL1Parser::build(grammar, first_sets, follow_sets);
    // Should fail to build due to LL(1) conflict
    assert!(result.is_err());
}

#[test]
fn test_ll1_epsilon_production() {
    let lines = vec![
        "2".to_string(),
        "S -> A".to_string(),
        "A -> a e".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);
    let parser = LL1Parser::build(grammar, first_sets, follow_sets).unwrap();

    assert!(parser.parse("a"));
    assert!(parser.parse(""));
}
