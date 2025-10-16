(** LL(1) predictive parser *)

open Utils
open Grammar
open FirstFollow

(** LL(1) parse table entry *)
type table_entry = production option

(** LL(1) parser type *)
type t

(** Exception raised when grammar is not LL(1) *)
exception Not_LL1 of string

(** Build LL(1) parser from grammar
    Raises Not_LL1 if the grammar is not LL(1) *)
val build : Grammar.t -> first_sets -> follow_sets -> t

(** Parse a string using the LL(1) parser
    Returns true if the string is accepted, false otherwise *)
val parse : t -> string -> bool

(** Get the parse table (for debugging) *)
val get_table : t -> (symbol * symbol, production) Hashtbl.t

(** Print the parse table *)
val print_table : t -> unit
