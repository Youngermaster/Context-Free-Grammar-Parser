(** SLR(1) bottom-up parser *)

open Utils
open Grammar
open FirstFollow

(** LR(0) item: (production, dot_position) *)
type item = production * int

(** SLR(1) action *)
type action =
  | Shift of int
  | Reduce of production
  | Accept
  | Error

(** Item set (state) *)
module ItemSet = Set.Make(struct
  type t = item
  let compare (p1, pos1) (p2, pos2) =
    let c = compare_symbol p1.lhs p2.lhs in
    if c <> 0 then c else
    let c = List.compare compare_symbol p1.rhs p2.rhs in
    if c <> 0 then c else
    Int.compare pos1 pos2
end)

(** SLR(1) parser type *)
type t = {
  grammar : Grammar.t;
  augmented_start : symbol;  (* S' for augmented grammar *)
  states : ItemSet.t array;
  action_table : (int * symbol, action) Hashtbl.t;
  goto_table : (int * symbol, int) Hashtbl.t;
  first_sets : first_sets;
  follow_sets : follow_sets;
}

(** Exception raised when grammar is not SLR(1) *)
exception Not_SLR1 of string

(** Compare items *)
let compare_items (p1, pos1) (p2, pos2) =
  let c = compare_symbol p1.lhs p2.lhs in
  if c <> 0 then c else
  let c = List.compare compare_symbol p1.rhs p2.rhs in
  if c <> 0 then c else
  Int.compare pos1 pos2

(** Convert item to string *)
let item_to_string (prod, dot_pos) =
  let lhs_str = symbol_to_string prod.lhs in
  let rhs = prod.rhs in
  let rec build_rhs pos acc = function
    | [] ->
        if pos = dot_pos then
          List.rev ("•" :: acc)
        else
          List.rev acc
    | sym :: rest ->
        if pos = dot_pos then
          build_rhs (pos + 1) (symbol_to_string sym :: "•" :: acc) rest
        else
          build_rhs (pos + 1) (symbol_to_string sym :: acc) rest
  in
  let rhs_str = String.concat " " (build_rhs 0 [] rhs) in
  lhs_str ^ " → " ^ rhs_str

(** Get symbol after dot in an item, if any *)
let symbol_after_dot (prod, dot_pos) =
  if dot_pos < List.length prod.rhs then
    Some (List.nth prod.rhs dot_pos)
  else
    None

(** Compute closure of a set of items
    For each item [A → α•Bβ] where B is nonterminal,
    add all items [B → •γ] for each production B → γ *)
let closure grammar items =
  let rec aux current =
    let new_items = ItemSet.fold (fun item acc ->
      match symbol_after_dot item with
      | Some sym when is_nonterminal sym ->
          let prods = get_productions grammar sym in
          let new_items = List.map (fun prod -> (prod, 0)) prods in
          List.fold_left (fun s i -> ItemSet.add i s) acc new_items
      | _ -> acc
    ) current current in

    if ItemSet.equal current new_items then
      current
    else
      aux new_items
  in
  aux items

(** Compute goto(I, X) - the set of items obtained by moving dot over X *)
let goto grammar items symbol =
  let moved = ItemSet.fold (fun (prod, dot_pos) acc ->
    match symbol_after_dot (prod, dot_pos) with
    | Some sym when compare_symbol sym symbol = 0 ->
        ItemSet.add (prod, dot_pos + 1) acc
    | _ -> acc
  ) items ItemSet.empty in
  closure grammar moved

(** Build the canonical LR(0) collection of item sets *)
let build_lr0_automaton grammar start_prod =
  let initial_item = (start_prod, 0) in
  let initial_state = closure grammar (ItemSet.singleton initial_item) in

  let states = ref [initial_state] in
  let state_map = Hashtbl.create 100 in
  Hashtbl.add state_map initial_state 0;

  let transitions = Hashtbl.create 100 in

  let rec process_states worklist =
    match worklist with
    | [] -> ()
    | state :: rest ->
        let state_id = Hashtbl.find state_map state in

        (* Get all symbols that can be shifted *)
        let symbols = ItemSet.fold (fun item acc ->
          match symbol_after_dot item with
          | Some sym -> SymbolSet.add sym acc
          | None -> acc
        ) state SymbolSet.empty in

        (* For each symbol, compute goto and add new states *)
        let new_worklist = SymbolSet.fold (fun sym worklist_acc ->
          let next_state = goto grammar state sym in
          if not (ItemSet.is_empty next_state) then begin
            if not (Hashtbl.mem state_map next_state) then begin
              let new_id = List.length !states in
              states := !states @ [next_state];
              Hashtbl.add state_map next_state new_id;
              Hashtbl.add transitions (state_id, sym) new_id;
              next_state :: worklist_acc
            end else begin
              let next_id = Hashtbl.find state_map next_state in
              Hashtbl.add transitions (state_id, sym) next_id;
              worklist_acc
            end
          end else
            worklist_acc
        ) symbols rest in

        process_states new_worklist
  in

  process_states [initial_state];
  (Array.of_list !states, transitions)

(** Build ACTION and GOTO tables for SLR(1) *)
let build_tables grammar states transitions first_sets follow_sets augmented_start start_prod =
  let action_table = Hashtbl.create 200 in
  let goto_table = Hashtbl.create 200 in

  (* Build state_map for reverse lookup *)
  let state_map = Hashtbl.create (Array.length states) in
  Array.iteri (fun i state ->
    Hashtbl.add state_map state i
  ) states;

  (* Process each state *)
  Array.iteri (fun state_id state ->
    (* Process each item in the state *)
    ItemSet.iter (fun (prod, dot_pos) ->
      if dot_pos < List.length prod.rhs then begin
        (* Shift items: [A → α•aβ] where a is terminal *)
        match symbol_after_dot (prod, dot_pos) with
        | Some sym when is_terminal sym ->
            let key = (state_id, sym) in
            if Hashtbl.mem transitions key then begin
              let next_state = Hashtbl.find transitions key in
              let action_key = (state_id, sym) in
              if Hashtbl.mem action_table action_key then
                raise (Not_SLR1 (Printf.sprintf
                  "Shift/Shift or Shift/Reduce conflict at state %d, symbol %s"
                  state_id (symbol_to_string sym)))
              else
                Hashtbl.add action_table action_key (Shift next_state)
            end
        | _ -> ()
      end else begin
        (* Reduce items: [A → α•] *)
        if prod.lhs = augmented_start then begin
          (* Accept item: [S' → S•] *)
          let action_key = (state_id, EndMarker) in
          Hashtbl.add action_table action_key Accept
        end else begin
          (* Reduce on FOLLOW(A) *)
          let follow_a = follow_of_symbol follow_sets prod.lhs in
          SymbolSet.iter (fun sym ->
            let action_key = (state_id, sym) in
            if Hashtbl.mem action_table action_key then begin
              let existing = Hashtbl.find action_table action_key in
              match existing with
              | Shift _ ->
                  raise (Not_SLR1 (Printf.sprintf
                    "Shift/Reduce conflict at state %d, symbol %s"
                    state_id (symbol_to_string sym)))
              | Reduce other_prod ->
                  raise (Not_SLR1 (Printf.sprintf
                    "Reduce/Reduce conflict at state %d, symbol %s:\n  %s\n  %s"
                    state_id (symbol_to_string sym)
                    (production_to_string other_prod)
                    (production_to_string prod)))
              | _ -> ()
            end else
              Hashtbl.add action_table action_key (Reduce prod)
          ) follow_a
        end
      end
    ) state;

    (* Build GOTO table for nonterminals *)
    Hashtbl.iter (fun (src, sym) dst ->
      if src = state_id && is_nonterminal sym then
        Hashtbl.add goto_table (state_id, sym) dst
    ) transitions
  ) states;

  (action_table, goto_table)

(** Build SLR(1) parser *)
let build grammar first_sets follow_sets =
  (* Create augmented grammar with S' → S *)
  let start = get_start_symbol grammar in
  let augmented_start = Nonterminal '\'' in  (* S' represented as ' *)
  let start_prod = { lhs = augmented_start; rhs = [start] } in

  (* Build LR(0) automaton *)
  let (states, transitions) = build_lr0_automaton grammar start_prod in

  (* Build ACTION and GOTO tables *)
  let (action_table, goto_table) =
    build_tables grammar states transitions first_sets follow_sets augmented_start start_prod in

  {
    grammar;
    augmented_start;
    states;
    action_table;
    goto_table;
    first_sets;
    follow_sets;
  }

(** Parse a string using SLR(1) shift-reduce algorithm *)
let parse parser input_str =
  (* Convert input to symbols and add $ *)
  let input_chars = List.init (String.length input_str) (String.get input_str) in
  let input_symbols = List.map char_to_symbol input_chars in
  let input = input_symbols @ [EndMarker] in

  (* Initialize stack with state 0 *)
  let initial_stack = [0] in

  (* Parsing loop *)
  let rec parse_loop stack symbols input =
    match input with
    | [] -> false  (* Unexpected end *)
    | curr_sym :: input_rest ->
        let state = List.hd stack in
        let action_key = (state, curr_sym) in

        let action =
          try Hashtbl.find parser.action_table action_key
          with Not_found -> Error
        in

        match action with
        | Accept -> true
        | Error -> false
        | Shift next_state ->
            (* Push symbol and next state *)
            parse_loop (next_state :: stack) (curr_sym :: symbols) input_rest
        | Reduce prod ->
            (* Pop |rhs| symbols and states *)
            let rhs_len = List.length prod.rhs in
            let rhs_len = if prod.rhs = [Epsilon] then 0 else rhs_len in
            let stack' = List.filteri (fun i _ -> i >= rhs_len) stack in
            let symbols' = List.filteri (fun i _ -> i >= rhs_len) symbols in

            (* Get state at top of stack after popping *)
            let state' = List.hd stack' in

            (* Find next state via GOTO *)
            let goto_key = (state', prod.lhs) in
            begin
              try
                let next_state = Hashtbl.find parser.goto_table goto_key in
                parse_loop (next_state :: stack') (prod.lhs :: symbols') input
              with Not_found -> false
            end
  in

  try
    parse_loop initial_stack [] input
  with _ -> false

(** Print the LR(0) automaton states *)
let print_states parser =
  print_endline "\nLR(0) Automaton States:";
  Array.iteri (fun i state ->
    Printf.printf "\nState %d:\n" i;
    ItemSet.iter (fun item ->
      Printf.printf "  %s\n" (item_to_string item)
    ) state
  ) parser.states

(** Print ACTION and GOTO tables *)
let print_tables parser =
  print_endline "\nACTION Table:";
  let action_entries = Hashtbl.fold (fun (state, sym) action acc ->
    (state, sym, action) :: acc
  ) parser.action_table [] in
  let sorted_actions = List.sort (fun (s1, sym1, _) (s2, sym2, _) ->
    let c = Int.compare s1 s2 in
    if c = 0 then compare_symbol sym1 sym2 else c
  ) action_entries in
  List.iter (fun (state, sym, action) ->
    let action_str = match action with
      | Shift s -> "shift " ^ string_of_int s
      | Reduce p -> "reduce " ^ production_to_string p
      | Accept -> "accept"
      | Error -> "error"
    in
    Printf.printf "  [%d, %s] = %s\n" state (symbol_to_string sym) action_str
  ) sorted_actions;

  print_endline "\nGOTO Table:";
  let goto_entries = Hashtbl.fold (fun (state, sym) next acc ->
    (state, sym, next) :: acc
  ) parser.goto_table [] in
  let sorted_gotos = List.sort (fun (s1, sym1, _) (s2, sym2, _) ->
    let c = Int.compare s1 s2 in
    if c = 0 then compare_symbol sym1 sym2 else c
  ) goto_entries in
  List.iter (fun (state, sym, next) ->
    Printf.printf "  [%d, %s] = %d\n" state (symbol_to_string sym) next
  ) sorted_gotos
