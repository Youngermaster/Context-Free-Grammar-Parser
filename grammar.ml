(** Grammar module for context-free grammars *)

open Utils

(** A production right-hand side is a list of symbols *)
type production_rhs = symbol list

(** A production has a left-hand side (nonterminal) and a right-hand side *)
type production = {
  lhs : symbol;
  rhs : production_rhs;
}

(** Internal representation of a grammar *)
type t = {
  productions : production list;
  nonterminals : SymbolSet.t;
  terminals : SymbolSet.t;
  start_symbol : symbol;
  production_map : production list SymbolMap.t;
}

(** Convert a string to a list of symbols *)
let string_to_symbols str =
  let chars = List.init (String.length str) (String.get str) in
  List.map char_to_symbol chars

(** Parse a single production line
    Format: "A -> alpha beta gamma"
    where alpha, beta, gamma are alternatives *)
let parse_production_line line =
  let parts = Str.split (Str.regexp " -> ") line in
  match parts with
  | [lhs_str; rhs_str] ->
      let lhs = char_to_symbol (String.get (String.trim lhs_str) 0) in
      let alternatives = Str.split (Str.regexp " +") (String.trim rhs_str) in
      let productions = List.map (fun alt ->
        let rhs = string_to_symbols alt in
        { lhs; rhs }
      ) alternatives in
      productions
  | _ -> failwith ("Invalid production format: " ^ line)

(** Extract all symbols from productions *)
let extract_symbols productions =
  let rec extract_from_rhs symbols = function
    | [] -> symbols
    | sym :: rest ->
        let symbols' = SymbolSet.add sym symbols in
        extract_from_rhs symbols' rest
  in
  List.fold_left (fun acc prod ->
    extract_from_rhs acc prod.rhs
  ) SymbolSet.empty productions

(** Partition symbols into nonterminals and terminals *)
let partition_symbols symbols =
  SymbolSet.partition is_nonterminal symbols

(** Build a map from nonterminals to their productions *)
let build_production_map productions =
  List.fold_left (fun map prod ->
    let current = try SymbolMap.find prod.lhs map with Not_found -> [] in
    SymbolMap.add prod.lhs (prod :: current) map
  ) SymbolMap.empty productions

(** Parse a grammar from string input *)
let parse_grammar lines =
  match lines with
  | [] -> failwith "Empty grammar input"
  | n_str :: production_lines ->
      let n = int_of_string (String.trim n_str) in
      if List.length production_lines < n then
        failwith "Not enough production lines";
      let production_lines = List.filteri (fun i _ -> i < n) production_lines in
      let all_productions = List.concat (List.map parse_production_line production_lines) in

      (* Extract all nonterminals from LHS *)
      let lhs_nonterminals = List.fold_left (fun acc prod ->
        SymbolSet.add prod.lhs acc
      ) SymbolSet.empty all_productions in

      (* Extract all symbols from RHS *)
      let rhs_symbols = extract_symbols all_productions in

      (* Partition RHS symbols into nonterminals and others *)
      let rhs_nonterminals, other_symbols = partition_symbols rhs_symbols in

      (* All nonterminals = LHS nonterminals ∪ RHS nonterminals *)
      let all_nonterminals = SymbolSet.union lhs_nonterminals rhs_nonterminals in

      (* Terminals are non-nonterminal symbols from RHS (excluding epsilon and $) *)
      let terminals = SymbolSet.filter (fun s ->
        is_terminal s
      ) other_symbols in

      (* Start symbol is always 'S' *)
      let start_symbol = Nonterminal 'S' in

      (* Build production map *)
      let production_map = build_production_map all_productions in

      {
        productions = all_productions;
        nonterminals = all_nonterminals;
        terminals;
        start_symbol;
        production_map;
      }

(** Get all productions for a nonterminal *)
let get_productions grammar nt =
  try
    List.rev (SymbolMap.find nt grammar.production_map)
  with Not_found -> []

(** Get all nonterminals in the grammar *)
let get_nonterminals grammar = grammar.nonterminals

(** Get all terminals in the grammar *)
let get_terminals grammar = grammar.terminals

(** Get the start symbol *)
let get_start_symbol grammar = grammar.start_symbol

(** Get all productions *)
let get_all_productions grammar = grammar.productions

(** Convert production to string *)
let production_to_string prod =
  let lhs_str = symbol_to_string prod.lhs in
  let rhs_str = if prod.rhs = [Epsilon] then "ε"
                else symbols_to_string prod.rhs in
  lhs_str ^ " → " ^ rhs_str

(** Convert grammar to string representation *)
let to_string grammar =
  let prod_strings = List.map production_to_string grammar.productions in
  String.concat "\n" prod_strings
