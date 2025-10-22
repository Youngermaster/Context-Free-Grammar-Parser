//! Integration tests matching the project specification examples

use cfg_parser::first_follow::{compute_first_sets, compute_follow_sets};
use cfg_parser::grammar::Grammar;
use cfg_parser::ll1::LL1Parser;
use cfg_parser::slr1::SLR1Parser;

/// Test Example 1 from the specification: SLR(1) only grammar
#[test]
fn test_example1_slr1_grammar() {
    let lines = vec![
        "3".to_string(),
        "S -> S+T T".to_string(),
        "T -> T*F F".to_string(),
        "F -> (S) i".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    // Should be SLR(1)
    let slr1_result = SLR1Parser::build(grammar.clone(), follow_sets.clone());
    assert!(slr1_result.is_ok(), "Grammar should be SLR(1)");

    // Should NOT be LL(1) (left recursive)
    let ll1_result = LL1Parser::build(grammar, first_sets, follow_sets);
    assert!(ll1_result.is_err(), "Grammar should not be LL(1)");

    // Test parsing
    let parser = slr1_result.unwrap();
    assert!(parser.parse("i+i"), "Should accept 'i+i'");
    assert!(parser.parse("(i)"), "Should accept '(i)'");
    assert!(!parser.parse("(i+i)*i)"), "Should reject '(i+i)*i)'");
}

/// Test Example 2 from the specification: Both LL(1) and SLR(1)
#[test]
fn test_example2_both_ll1_and_slr1() {
    let lines = vec![
        "3".to_string(),
        "S -> AB".to_string(),
        "A -> aA d".to_string(),
        "B -> bBc e".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    // Should be both LL(1) and SLR(1)
    let ll1_result = LL1Parser::build(grammar.clone(), first_sets.clone(), follow_sets.clone());
    let slr1_result = SLR1Parser::build(grammar, follow_sets);

    assert!(ll1_result.is_ok(), "Grammar should be LL(1)");
    assert!(slr1_result.is_ok(), "Grammar should be SLR(1)");

    // Test LL(1) parsing
    let ll1_parser = ll1_result.unwrap();
    assert!(ll1_parser.parse("d"), "LL(1): Should accept 'd'");
    assert!(ll1_parser.parse("adbc"), "LL(1): Should accept 'adbc'");
    assert!(!ll1_parser.parse("a"), "LL(1): Should reject 'a'");

    // Test SLR(1) parsing
    let slr1_parser = slr1_result.unwrap();
    assert!(slr1_parser.parse("d"), "SLR(1): Should accept 'd'");
    assert!(slr1_parser.parse("adbc"), "SLR(1): Should accept 'adbc'");
    assert!(!slr1_parser.parse("a"), "SLR(1): Should reject 'a'");
}

/// Test Example 3 from the specification: Neither LL(1) nor SLR(1)
#[test]
fn test_example3_neither_ll1_nor_slr1() {
    let lines = vec![
        "2".to_string(),
        "S -> A".to_string(),
        "A -> A b".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    // Should be neither LL(1) nor SLR(1) (left recursive, no base case)
    let ll1_result = LL1Parser::build(grammar.clone(), first_sets, follow_sets.clone());
    let slr1_result = SLR1Parser::build(grammar, follow_sets);

    assert!(ll1_result.is_err(), "Grammar should not be LL(1)");
    assert!(slr1_result.is_err(), "Grammar should not be SLR(1)");
}

/// Test end-to-end: Complex expression grammar
#[test]
fn test_complex_expression_parsing() {
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

    // Complex valid expressions
    assert!(parser.parse("i"));
    assert!(parser.parse("i+i+i"));
    assert!(parser.parse("i*i*i"));
    assert!(parser.parse("i+i*i+i"));
    assert!(parser.parse("(i+i)*(i+i)"));
    assert!(parser.parse("((i))"));

    // Complex invalid expressions
    assert!(!parser.parse(""));
    assert!(!parser.parse("("));
    assert!(!parser.parse(")"));
    assert!(!parser.parse("i+"));
    assert!(!parser.parse("+i"));
    assert!(!parser.parse("i++i"));
}

/// Test grammar with epsilon productions
#[test]
fn test_epsilon_productions() {
    let lines = vec![
        "3".to_string(),
        "S -> AB".to_string(),
        "A -> aA d".to_string(),
        "B -> bBc e".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    let ll1_parser = LL1Parser::build(grammar.clone(), first_sets.clone(), follow_sets.clone()).unwrap();
    let slr1_parser = SLR1Parser::build(grammar, follow_sets).unwrap();

    // Test with epsilon (B → e means B produces empty string)
    assert!(ll1_parser.parse("d")); // A → d, B → e
    assert!(slr1_parser.parse("d"));

    assert!(ll1_parser.parse("ad")); // A → aA → ad, B → e
    assert!(slr1_parser.parse("ad"));
}
