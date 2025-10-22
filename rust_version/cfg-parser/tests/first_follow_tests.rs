//! Unit tests for FIRST and FOLLOW set computation

use cfg_parser::first_follow::*;
use cfg_parser::grammar::Grammar;
use cfg_parser::symbol::Symbol;

#[test]
fn test_first_sets_simple() {
    let lines = vec![
        "2".to_string(),
        "S -> AB".to_string(),
        "A -> a".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);

    let first_a = first_sets.get(&Symbol::Nonterminal('A')).unwrap();
    assert!(first_a.contains(&Symbol::Terminal('a')));
}

#[test]
fn test_follow_sets_simple() {
    let lines = vec![
        "2".to_string(),
        "S -> AB".to_string(),
        "A -> a".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    let follow_s = follow_sets.get(&Symbol::Nonterminal('S')).unwrap();
    assert!(follow_s.contains(&Symbol::EndMarker));
}

#[test]
fn test_first_with_epsilon() {
    let lines = vec![
        "2".to_string(),
        "S -> AB".to_string(),
        "A -> a e".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);

    let first_a = first_sets.get(&Symbol::Nonterminal('A')).unwrap();
    assert!(first_a.contains(&Symbol::Terminal('a')));
    assert!(first_a.contains(&Symbol::Epsilon));
}

#[test]
fn test_first_of_string() {
    let lines = vec![
        "2".to_string(),
        "S -> AB".to_string(),
        "A -> a".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);

    let symbols = vec![Symbol::Nonterminal('A'), Symbol::Nonterminal('B')];
    let first = first_of_string(&first_sets, &symbols);

    assert!(first.contains(&Symbol::Terminal('a')));
}

#[test]
fn test_follow_propagation() {
    let lines = vec![
        "3".to_string(),
        "S -> AB".to_string(),
        "A -> a".to_string(),
        "B -> b".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    // FOLLOW(A) should contain FIRST(B)
    let follow_a = follow_sets.get(&Symbol::Nonterminal('A')).unwrap();
    assert!(follow_a.contains(&Symbol::Terminal('b')));
}

#[test]
fn test_complex_first_follow() {
    let lines = vec![
        "3".to_string(),
        "S -> AB".to_string(),
        "A -> aA d".to_string(),
        "B -> bBc e".to_string(),
    ];

    let grammar = Grammar::parse(&lines).unwrap();
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    // Check FIRST sets
    let first_s = first_sets.get(&Symbol::Nonterminal('S')).unwrap();
    assert!(first_s.contains(&Symbol::Terminal('a')));
    assert!(first_s.contains(&Symbol::Terminal('d')));

    // Check FOLLOW sets
    let follow_a = follow_sets.get(&Symbol::Nonterminal('A')).unwrap();
    assert!(follow_a.contains(&Symbol::Terminal('b')));
    assert!(follow_a.contains(&Symbol::EndMarker));
}
