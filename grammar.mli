(** Grammar module for context-free grammars *)

open Utils

(** A production right-hand side is a list of symbols *)
type production_rhs = symbol list

(** A production has a left-hand side (nonterminal) and a right-hand side *)
type production = {
  lhs : symbol;
  rhs : production_rhs;
}

(** Grammar type containing all productions and computed sets *)
type t

(** Parse a grammar from string input
    Format:
    - First line: number of nonterminals
    - Following lines: <NT> -> <alternative1> <alternative2> ... *)
val parse_grammar : string list -> t

(** Get all productions for a nonterminal *)
val get_productions : t -> symbol -> production list

(** Get all nonterminals in the grammar *)
val get_nonterminals : t -> SymbolSet.t

(** Get all terminals in the grammar *)
val get_terminals : t -> SymbolSet.t

(** Get the start symbol (always 'S') *)
val get_start_symbol : t -> symbol

(** Get all productions in the grammar *)
val get_all_productions : t -> production list

(** Convert production to string *)
val production_to_string : production -> string

(** Convert grammar to string representation *)
val to_string : t -> string
