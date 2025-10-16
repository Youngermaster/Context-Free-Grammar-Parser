(** Utility module for symbol classification and basic types *)

(** Symbol type: Terminal, Nonterminal, Epsilon, or EndMarker *)
type symbol =
  | Terminal of char
  | Nonterminal of char
  | Epsilon
  | EndMarker

(** Compare symbols for ordering *)
let compare_symbol s1 s2 = match (s1, s2) with
  | (Epsilon, Epsilon) -> 0
  | (Epsilon, _) -> -1
  | (_, Epsilon) -> 1
  | (EndMarker, EndMarker) -> 0
  | (EndMarker, _) -> 1
  | (_, EndMarker) -> -1
  | (Terminal c1, Terminal c2) -> Char.compare c1 c2
  | (Terminal _, Nonterminal _) -> -1
  | (Nonterminal _, Terminal _) -> 1
  | (Nonterminal c1, Nonterminal c2) -> Char.compare c1 c2

(** Check if a character is an uppercase letter (A-Z) *)
let is_uppercase c = c >= 'A' && c <= 'Z'

(** Check if a character is a valid terminal (not uppercase, not 'e', not '$') *)
let is_terminal_char c = not (is_uppercase c) && c <> 'e' && c <> '$'

(** Check if a symbol is a terminal *)
let is_terminal = function
  | Terminal _ -> true
  | _ -> false

(** Check if a symbol is a nonterminal *)
let is_nonterminal = function
  | Nonterminal _ -> true
  | _ -> false

(** Check if a symbol is epsilon *)
let is_epsilon = function
  | Epsilon -> true
  | _ -> false

(** Check if a symbol is the end marker *)
let is_end_marker = function
  | EndMarker -> true
  | _ -> false

(** Convert a character to a symbol based on grammar conventions *)
let char_to_symbol c =
  if is_uppercase c then
    Nonterminal c
  else if c = 'e' then
    Epsilon
  else if c = '$' then
    EndMarker
  else
    Terminal c

(** Convert a symbol to a string representation *)
let symbol_to_string = function
  | Terminal c -> String.make 1 c
  | Nonterminal c -> String.make 1 c
  | Epsilon -> "ε"
  | EndMarker -> "$"

(** Convert a list of symbols to a string *)
let symbols_to_string symbols =
  String.concat "" (List.map symbol_to_string symbols)

(** Module for sets of symbols *)
module SymbolSet = Set.Make(struct
  type t = symbol
  let compare = compare_symbol
end)

(** Module for maps with symbols as keys *)
module SymbolMap = Map.Make(struct
  type t = symbol
  let compare = compare_symbol
end)

(** Convert SymbolSet to list *)
let set_to_list s = SymbolSet.elements s

(** Convert list to SymbolSet *)
let list_to_set lst = List.fold_left (fun acc x -> SymbolSet.add x acc) SymbolSet.empty lst

(** Print a set of symbols *)
let print_symbol_set set =
  let lst = set_to_list set in
  print_string "{ ";
  List.iter (fun s -> print_string (symbol_to_string s ^ " ")) lst;
  print_endline "}"

(** Union of multiple sets *)
let union_many sets =
  List.fold_left SymbolSet.union SymbolSet.empty sets
