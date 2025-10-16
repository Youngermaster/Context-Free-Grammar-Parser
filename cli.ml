(** CLI module for grammar parser application *)

open Utils
open Grammar
open FirstFollow
open Ll1
open Slr1

(** Read grammar lines from stdin
    Reads n+1 lines: first line is number n, then n production lines *)
let read_grammar () =
  let n_str = input_line stdin in
  let n = int_of_string (String.trim n_str) in
  let lines = ref [n_str] in
  for _i = 1 to n do
    lines := input_line stdin :: !lines
  done;
  List.rev !lines

(** Parse strings with a parser until empty line *)
let rec parse_strings parse_fn =
  try
    let line = input_line stdin in
    let trimmed = String.trim line in
    if trimmed = "" then
      ()  (* Stop on empty line *)
    else begin
      let result = parse_fn trimmed in
      print_endline (if result then "yes" else "no");
      parse_strings parse_fn
    end
  with End_of_file -> ()

(** Interactive mode for Case 1: both LL(1) and SLR(1) *)
let rec interactive_mode ll1_parser slr1_parser =
  print_string "Select a parser (T: for LL(1), B: for SLR(1), Q: quit):\n";
  flush stdout;
  try
    let choice = String.trim (input_line stdin) in
    match choice with
    | "Q" | "q" -> ()
    | "T" | "t" ->
        parse_strings (Ll1.parse ll1_parser);
        interactive_mode ll1_parser slr1_parser
    | "B" | "b" ->
        parse_strings (Slr1.parse slr1_parser);
        interactive_mode ll1_parser slr1_parser
    | _ ->
        (* Invalid choice, prompt again *)
        interactive_mode ll1_parser slr1_parser
  with End_of_file -> ()

(** Main CLI logic *)
let run () =
  try
    (* Read grammar input *)
    let lines = read_grammar () in
    let grammar = Grammar.parse_grammar lines in

    (* Compute FIRST and FOLLOW sets *)
    let first_sets = FirstFollow.compute_first_sets grammar in
    let follow_sets = FirstFollow.compute_follow_sets grammar first_sets in

    (* Try to build LL(1) parser *)
    let ll1_result = try
      let ll1_parser = Ll1.build grammar first_sets follow_sets in
      Some ll1_parser
    with Ll1.Not_LL1 _ -> None
    in

    (* Try to build SLR(1) parser *)
    let slr1_result = try
      let slr1_parser = Slr1.build grammar first_sets follow_sets in
      Some slr1_parser
    with Slr1.Not_SLR1 _ -> None
    in

    (* Determine which case we're in *)
    match (ll1_result, slr1_result) with
    | (Some ll1_parser, Some slr1_parser) ->
        (* Case 1: Both LL(1) and SLR(1) *)
        interactive_mode ll1_parser slr1_parser

    | (Some ll1_parser, None) ->
        (* Case 2: LL(1) only *)
        print_endline "Grammar is LL(1).";
        parse_strings (Ll1.parse ll1_parser)

    | (None, Some slr1_parser) ->
        (* Case 3: SLR(1) only *)
        print_endline "Grammar is SLR(1).";
        parse_strings (Slr1.parse slr1_parser)

    | (None, None) ->
        (* Case 4: Neither LL(1) nor SLR(1) *)
        print_endline "Grammar is neither LL(1) nor SLR(1)."

  with
  | Failure msg ->
      Printf.eprintf "Error: %s\n" msg;
      exit 1
  | e ->
      Printf.eprintf "Unexpected error: %s\n" (Printexc.to_string e);
      exit 1
