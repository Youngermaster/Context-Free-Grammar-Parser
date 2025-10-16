(** SLR(1) bottom-up parser *)

open Grammar
open FirstFollow

(** LR(0) item: (production, dot_position)
    For production A → α, item [A → α•β] has dot at position |α| *)
type item = production * int

(** SLR(1) action *)
type action =
  | Shift of int      (* Shift and go to state *)
  | Reduce of production  (* Reduce using production *)
  | Accept            (* Accept input *)
  | Error             (* Error *)

(** SLR(1) parser type *)
type t

(** Exception raised when grammar is not SLR(1) *)
exception Not_SLR1 of string

(** Build SLR(1) parser from grammar
    Raises Not_SLR1 if the grammar is not SLR(1) *)
val build : Grammar.t -> first_sets -> follow_sets -> t

(** Parse a string using the SLR(1) parser
    Returns true if the string is accepted, false otherwise *)
val parse : t -> string -> bool

(** Print the LR(0) automaton states (for debugging) *)
val print_states : t -> unit

(** Print the ACTION and GOTO tables (for debugging) *)
val print_tables : t -> unit
