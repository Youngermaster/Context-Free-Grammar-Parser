(** FIRST and FOLLOW sets computation *)

open Utils
open Grammar

(** Type for FIRST sets: maps each symbol to its FIRST set *)
type first_sets = SymbolSet.t SymbolMap.t

(** Type for FOLLOW sets: maps each nonterminal to its FOLLOW set *)
type follow_sets = SymbolSet.t SymbolMap.t

(** Compute FIRST sets for all symbols in the grammar *)
val compute_first_sets : t -> first_sets

(** Compute FOLLOW sets for all nonterminals in the grammar *)
val compute_follow_sets : t -> first_sets -> follow_sets

(** Get FIRST set of a single symbol *)
val first_of_symbol : first_sets -> symbol -> SymbolSet.t

(** Get FIRST set of a string (sequence of symbols) *)
val first_of_string : first_sets -> symbol list -> SymbolSet.t

(** Get FOLLOW set of a nonterminal *)
val follow_of_symbol : follow_sets -> symbol -> SymbolSet.t

(** Print FIRST sets *)
val print_first_sets : first_sets -> t -> unit

(** Print FOLLOW sets *)
val print_follow_sets : follow_sets -> t -> unit
