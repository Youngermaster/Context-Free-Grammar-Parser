//! CLI module for the grammar parser application.

use crate::error::Result;
use crate::first_follow::{compute_first_sets, compute_follow_sets};
use crate::grammar::Grammar;
use crate::ll1::LL1Parser;
use crate::slr1::SLR1Parser;
use std::io::{self, BufRead, Write};

/// Main CLI runner for the grammar parser.
pub fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read grammar
    let grammar_lines = read_grammar(&mut lines)?;
    let grammar = Grammar::parse(&grammar_lines)?;

    // Compute FIRST and FOLLOW sets
    let first_sets = compute_first_sets(&grammar);
    let follow_sets = compute_follow_sets(&grammar, &first_sets);

    // Try to build LL(1) parser
    let ll1_result = LL1Parser::build(grammar.clone(), first_sets.clone(), follow_sets.clone());

    // Try to build SLR(1) parser
    let slr1_result = SLR1Parser::build(grammar, follow_sets);

    // Determine which case we're in and handle accordingly
    match (ll1_result, slr1_result) {
        (Ok(ll1_parser), Ok(slr1_parser)) => {
            // Case 1: Both LL(1) and SLR(1)
            interactive_mode(ll1_parser, slr1_parser, &mut lines)?;
        }
        (Ok(ll1_parser), Err(_)) => {
            // Case 2: LL(1) only
            println!("Grammar is LL(1).");
            parse_strings(|s| ll1_parser.parse(s), &mut lines)?;
        }
        (Err(_), Ok(slr1_parser)) => {
            // Case 3: SLR(1) only
            println!("Grammar is SLR(1).");
            parse_strings(|s| slr1_parser.parse(s), &mut lines)?;
        }
        (Err(_), Err(_)) => {
            // Case 4: Neither LL(1) nor SLR(1)
            println!("Grammar is neither LL(1) nor SLR(1).");
        }
    }

    Ok(())
}

/// Reads the grammar from input lines.
///
/// First line is the number n, then n production lines.
fn read_grammar<R: BufRead>(lines: &mut io::Lines<R>) -> Result<Vec<String>> {
    let mut grammar_lines = Vec::new();

    // Read first line (number of nonterminals)
    let n_str = lines.next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Expected number of nonterminals",
        )
    })??;

    let n = n_str
        .trim()
        .parse::<usize>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number"))?;

    grammar_lines.push(n_str);

    // Read n production lines
    for _ in 0..n {
        let line = lines.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::UnexpectedEof, "Expected production line")
        })??;
        grammar_lines.push(line);
    }

    Ok(grammar_lines)
}

/// Parses strings until an empty line is encountered.
fn parse_strings<F, R>(parse_fn: F, lines: &mut io::Lines<R>) -> Result<()>
where
    F: Fn(&str) -> bool,
    R: BufRead,
{
    while let Some(Ok(line)) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        let result = parse_fn(trimmed);
        println!("{}", if result { "yes" } else { "no" });
    }

    Ok(())
}

/// Interactive mode for when grammar is both LL(1) and SLR(1).
fn interactive_mode<R: BufRead>(
    ll1_parser: LL1Parser,
    slr1_parser: SLR1Parser,
    lines: &mut io::Lines<R>,
) -> Result<()> {
    loop {
        // Print prompt
        print!("Select a parser (T: for LL(1), B: for SLR(1), Q: quit):\n");
        io::stdout().flush()?;

        // Read choice
        let choice = match lines.next() {
            Some(Ok(line)) => line.trim().to_string(),
            Some(Err(e)) => return Err(e.into()),
            None => break, // EOF
        };

        match choice.as_str() {
            "Q" | "q" => break,
            "T" | "t" => {
                parse_strings(|s| ll1_parser.parse(s), lines)?;
            }
            "B" | "b" => {
                parse_strings(|s| slr1_parser.parse(s), lines)?;
            }
            _ => {
                // Invalid choice, re-prompt
                continue;
            }
        }
    }

    Ok(())
}
