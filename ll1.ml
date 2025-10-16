(** LL(1) predictive parser *)

open Utils
open Grammar
open FirstFollow

(** LL(1) parse table entry *)
type table_entry = production option

(** LL(1) parser type *)
type t = {
  grammar : Grammar.t;
  table : (symbol * symbol, production) Hashtbl.t;
  first_sets : first_sets;
  follow_sets : follow_sets;
}

(** Exception raised when grammar is not LL(1) *)
exception Not_LL1 of string

(** Build the LL(1) parse table
    For each production A → α:
    1. For each terminal a in FIRST(α), add A → α to M[A, a]
    2. If ε ∈ FIRST(α), for each b in FOLLOW(A), add A → α to M[A, b]
    If any cell has multiple entries, the grammar is not LL(1) *)
let build_table grammar first_sets follow_sets =
  let table = Hashtbl.create 100 in
  let productions = get_all_productions grammar in

  List.iter (fun prod ->
    let lhs = prod.lhs in
    let rhs = prod.rhs in

    (* Compute FIRST(α) *)
    let first_alpha = first_of_string first_sets rhs in

    (* For each terminal in FIRST(α) - {ε} *)
    SymbolSet.iter (fun sym ->
      if not (is_epsilon sym) then begin
        let key = (lhs, sym) in
        if Hashtbl.mem table key then
          let existing_prod = Hashtbl.find table key in
          raise (Not_LL1 (Printf.sprintf
            "Conflict at M[%s, %s]:\n  %s\n  %s"
            (symbol_to_string lhs)
            (symbol_to_string sym)
            (production_to_string existing_prod)
            (production_to_string prod)))
        else
          Hashtbl.add table key prod
      end
    ) first_alpha;

    (* If ε ∈ FIRST(α) *)
    if SymbolSet.mem Epsilon first_alpha then begin
      let follow_lhs = follow_of_symbol follow_sets lhs in
      SymbolSet.iter (fun sym ->
        let key = (lhs, sym) in
        if Hashtbl.mem table key then
          let existing_prod = Hashtbl.find table key in
          raise (Not_LL1 (Printf.sprintf
            "Conflict at M[%s, %s] (via epsilon):\n  %s\n  %s"
            (symbol_to_string lhs)
            (symbol_to_string sym)
            (production_to_string existing_prod)
            (production_to_string prod)))
        else
          Hashtbl.add table key prod
      ) follow_lhs
    end
  ) productions;

  table

(** Build LL(1) parser *)
let build grammar first_sets follow_sets =
  let table = build_table grammar first_sets follow_sets in
  { grammar; table; first_sets; follow_sets }

(** Get the parse table *)
let get_table parser = parser.table

(** Parse a string using stack-based LL(1) algorithm
    Stack initially contains [$, S]
    Input ends with $

    At each step:
    - If top of stack = current input symbol: pop and advance
    - If top is nonterminal: use table to get production, pop and push RHS (reversed)
    - If top is terminal but ≠ input: reject
    - If table entry is empty: reject
    - Accept when stack is [$] and input is [$] *)
let parse parser input_str =
  (* Convert input to symbols and add $ *)
  let input_chars = List.init (String.length input_str) (String.get input_str) in
  let input_symbols = List.map char_to_symbol input_chars in
  let input = input_symbols @ [EndMarker] in

  (* Initialize stack with [$, S] *)
  let start = get_start_symbol parser.grammar in
  let initial_stack = [EndMarker; start] in

  (* Parsing loop *)
  let rec parse_loop stack input =
    match stack, input with
    | [], [] -> true  (* Should not happen, handled by EndMarker check *)
    | EndMarker :: _, EndMarker :: _ -> true  (* Accept *)
    | [], _ | _, [] -> false  (* Unexpected end *)

    | top :: stack_rest, inp :: input_rest ->
        (* If top matches input, pop both *)
        if compare_symbol top inp = 0 then
          parse_loop stack_rest input_rest

        (* If top is nonterminal, use parse table *)
        else if is_nonterminal top then begin
          let key = (top, inp) in
          try
            let prod = Hashtbl.find parser.table key in
            (* Pop nonterminal and push RHS (reversed) *)
            let new_stack = match prod.rhs with
              | [Epsilon] -> stack_rest  (* Don't push epsilon *)
              | _ -> (List.rev prod.rhs) @ stack_rest
            in
            parse_loop new_stack input
          with Not_found ->
            false  (* No table entry *)
        end

        (* Top is terminal but doesn't match input *)
        else
          false
  in

  try
    parse_loop initial_stack input
  with _ -> false

(** Print the parse table *)
let print_table parser =
  print_endline "\nLL(1) Parse Table:";
  let entries = Hashtbl.fold (fun (nt, term) prod acc ->
    (nt, term, prod) :: acc
  ) parser.table [] in

  (* Sort entries for consistent output *)
  let sorted = List.sort (fun (nt1, t1, _) (nt2, t2, _) ->
    let c = compare_symbol nt1 nt2 in
    if c = 0 then compare_symbol t1 t2 else c
  ) entries in

  List.iter (fun (nt, term, prod) ->
    Printf.printf "  M[%s, %s] = %s\n"
      (symbol_to_string nt)
      (symbol_to_string term)
      (production_to_string prod)
  ) sorted
