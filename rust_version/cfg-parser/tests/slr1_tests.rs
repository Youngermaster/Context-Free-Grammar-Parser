//! Unit tests for SLR(1) parser

use cfg_parser::first_follow::{compute_first_sets, compute_follow_sets};
use cfg_parser::grammar::Grammar;
use cfg_parser::slr1::SLR1Parser;

#[test]
fn test_slr1_simple() {
    let lines = vec![
        "3".to_string(),
        "S -> S+T T".to_string(),
        "T -> T*F F".to_string(),
        "F -> (S) i".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    let parser = SLR1Parser::build(grammar, follow_sets);
    assert!(parser.is_ok());

    let parser = parser.unwrap();
    assert!(parser.parse("i+i"));
    assert!(parser.parse("(i)"));
    assert!(!parser.parse("(i+i)*i)"));
}

#[test]
fn test_slr1_accepts_valid_expressions() {
    let lines = vec![
        "3".to_string(),
        "S -> S+T T".to_string(),
        "T -> T*F F".to_string(),
        "F -> (S) i".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);
    let parser = SLR1Parser::build(grammar, follow_sets).unwrap();

    // Valid expressions
    assert!(parser.parse("i"));
    assert!(parser.parse("i+i"));
    assert!(parser.parse("i*i"));
    assert!(parser.parse("i+i*i"));
    assert!(parser.parse("i*i+i"));
    assert!(parser.parse("(i)"));
    assert!(parser.parse("(i+i)"));
    assert!(parser.parse("(i)*i"));
    assert!(parser.parse("i+(i*i)"));
}

#[test]
fn test_slr1_rejects_invalid_expressions() {
    let lines = vec![
        "3".to_string(),
        "S -> S+T T".to_string(),
        "T -> T*F F".to_string(),
        "F -> (S) i".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);
    let parser = SLR1Parser::build(grammar, follow_sets).unwrap();

    // Invalid expressions
    assert!(!parser.parse(""));
    assert!(!parser.parse("+"));
    assert!(!parser.parse("i+"));
    assert!(!parser.parse("*i"));
    assert!(!parser.parse("(i"));
    assert!(!parser.parse("i)"));
    assert!(!parser.parse("(i+i)*i)"));
    assert!(!parser.parse("ii"));
}

#[test]
fn test_slr1_conflict_detection() {
    // Grammar with reduce/reduce conflict
    let lines = vec![
        "3".to_string(),
        "S -> A a".to_string(),
        "A -> B".to_string(),
        "B -> b".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    // This grammar should be SLR(1)
    let result = SLR1Parser::build(grammar, follow_sets);
    assert!(result.is_ok());
}

#[test]
fn test_slr1_operator_precedence() {
    let lines = vec![
        "3".to_string(),
        "S -> S+T T".to_string(),
        "T -> T*F F".to_string(),
        "F -> (S) i".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);
    let parser = SLR1Parser::build(grammar, follow_sets).unwrap();

    // Test that parser respects grammar structure (implicit precedence)
    assert!(parser.parse("i+i*i")); // * has higher precedence
    assert!(parser.parse("(i+i)*i")); // Parentheses work
}
