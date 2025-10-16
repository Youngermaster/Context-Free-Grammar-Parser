(** Utility module for symbol classification and basic types *)

(** Symbol type representing terminals, nonterminals, epsilon, and end marker *)
type symbol =
  | Terminal of char
  | Nonterminal of char
  | Epsilon
  | EndMarker

(** Compare two symbols *)
val compare_symbol : symbol -> symbol -> int

(** Check if a character is uppercase *)
val is_uppercase : char -> bool

(** Check if a character is a valid terminal *)
val is_terminal_char : char -> bool

(** Check if a symbol is a terminal *)
val is_terminal : symbol -> bool

(** Check if a symbol is a nonterminal *)
val is_nonterminal : symbol -> bool

(** Check if a symbol is epsilon *)
val is_epsilon : symbol -> bool

(** Check if a symbol is the end marker *)
val is_end_marker : symbol -> bool

(** Convert a character to a symbol *)
val char_to_symbol : char -> symbol

(** Convert a symbol to a string *)
val symbol_to_string : symbol -> string

(** Convert a list of symbols to a string *)
val symbols_to_string : symbol list -> string

(** Set module for symbols *)
module SymbolSet : Set.S with type elt = symbol

(** Map module for symbols *)
module SymbolMap : sig
  include Map.S with type key = symbol
end

(** Convert SymbolSet to list *)
val set_to_list : SymbolSet.t -> symbol list

(** Convert list to SymbolSet *)
val list_to_set : symbol list -> SymbolSet.t

(** Print a set of symbols *)
val print_symbol_set : SymbolSet.t -> unit

(** Union of multiple sets *)
val union_many : SymbolSet.t list -> SymbolSet.t
