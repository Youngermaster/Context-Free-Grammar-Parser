(** FIRST and FOLLOW sets computation *)

open Utils
open Grammar

(** Type for FIRST sets *)
type first_sets = SymbolSet.t SymbolMap.t

(** Type for FOLLOW sets *)
type follow_sets = SymbolSet.t SymbolMap.t

(** Get FIRST set of a symbol, returning empty set if not found *)
let first_of_symbol first_map symbol =
  try SymbolMap.find symbol first_map
  with Not_found -> SymbolSet.empty

(** Get FOLLOW set of a symbol, returning empty set if not found *)
let follow_of_symbol follow_map symbol =
  try SymbolMap.find symbol follow_map
  with Not_found -> SymbolSet.empty

(** Compute FIRST set of a string (sequence of symbols)
    FIRST(X₁X₂...Xₙ) algorithm:
    - Add FIRST(X₁) - {ε} to result
    - If ε ∈ FIRST(X₁), add FIRST(X₂) - {ε}
    - Continue while ε ∈ FIRST(Xᵢ)
    - If ε ∈ FIRST(Xᵢ) for all i, add ε to result *)
let first_of_string first_map symbols =
  let rec aux result has_epsilon = function
    | [] ->
        if has_epsilon then SymbolSet.add Epsilon result
        else result
    | sym :: rest ->
        let first_sym = first_of_symbol first_map sym in
        let result' = SymbolSet.union result (SymbolSet.remove Epsilon first_sym) in
        if SymbolSet.mem Epsilon first_sym then
          aux result' true rest
        else
          result'
  in
  aux SymbolSet.empty false symbols

(** Compute FIRST sets for all symbols
    Algorithm (fixed-point iteration):
    1. For terminals: FIRST(a) = {a}
    2. For nonterminals A with production A → X₁X₂...Xₙ:
       - Add FIRST(X₁) - {ε} to FIRST(A)
       - If ε ∈ FIRST(X₁), add FIRST(X₂) - {ε}
       - Continue while ε ∈ FIRST(Xᵢ)
       - If ε ∈ FIRST(Xᵢ) for all i, add ε to FIRST(A)
    3. Repeat until no changes *)
let compute_first_sets grammar =
  let nonterminals = get_nonterminals grammar in
  let terminals = get_terminals grammar in
  let all_productions = get_all_productions grammar in

  (* Initialize FIRST sets *)
  let initial_first =
    (* Add terminals *)
    let with_terminals = SymbolSet.fold (fun t acc ->
      SymbolMap.add t (SymbolSet.singleton t) acc
    ) terminals SymbolMap.empty in
    (* Add epsilon *)
    let with_epsilon = SymbolMap.add Epsilon (SymbolSet.singleton Epsilon) with_terminals in
    (* Add end marker *)
    let with_end = SymbolMap.add EndMarker (SymbolSet.singleton EndMarker) with_epsilon in
    (* Initialize nonterminals with empty sets *)
    SymbolSet.fold (fun nt acc ->
      SymbolMap.add nt SymbolSet.empty acc
    ) nonterminals with_end
  in

  (* Fixed-point iteration *)
  let rec iterate first_map =
    let new_first_map = List.fold_left (fun acc_map prod ->
      let lhs = prod.lhs in
      let rhs = prod.rhs in
      let current_first = first_of_symbol acc_map lhs in

      (* Compute FIRST of RHS *)
      let rhs_first = first_of_string acc_map rhs in

      (* Union with current FIRST set *)
      let new_first = SymbolSet.union current_first rhs_first in

      SymbolMap.add lhs new_first acc_map
    ) first_map all_productions in

    (* Check if anything changed *)
    let changed = SymbolMap.exists (fun sym new_set ->
      let old_set = first_of_symbol first_map sym in
      not (SymbolSet.equal old_set new_set)
    ) new_first_map in

    if changed then iterate new_first_map
    else new_first_map
  in

  iterate initial_first

(** Compute FOLLOW sets for all nonterminals
    Algorithm (fixed-point iteration):
    1. FOLLOW(S) contains $
    2. For production A → αBβ:
       - Add FIRST(β) - {ε} to FOLLOW(B)
       - If ε ∈ FIRST(β) or β = ε, add FOLLOW(A) to FOLLOW(B)
    3. Repeat until no changes *)
let compute_follow_sets grammar first_map =
  let nonterminals = get_nonterminals grammar in
  let start_symbol = get_start_symbol grammar in
  let all_productions = get_all_productions grammar in

  (* Initialize FOLLOW sets *)
  let initial_follow =
    (* Initialize all nonterminals with empty sets *)
    let empty_follow = SymbolSet.fold (fun nt acc ->
      SymbolMap.add nt SymbolSet.empty acc
    ) nonterminals SymbolMap.empty in
    (* Add $ to FOLLOW(S) *)
    let start_follow = SymbolSet.singleton EndMarker in
    SymbolMap.add start_symbol start_follow empty_follow
  in

  (* Fixed-point iteration *)
  let rec iterate follow_map =
    let new_follow_map = List.fold_left (fun acc_map prod ->
      let lhs = prod.lhs in
      let rhs = prod.rhs in

      (* Process each position in the RHS *)
      let rec process_rhs acc beta = match beta with
        | [] -> acc
        | sym :: rest ->
            (* Only process nonterminals *)
            if not (is_nonterminal sym) then
              process_rhs acc rest
            else
              let current_follow = follow_of_symbol acc sym in

              (* Compute FIRST(rest) *)
              let first_rest = first_of_string first_map rest in

              (* Add FIRST(rest) - {ε} to FOLLOW(sym) *)
              let new_follow = SymbolSet.union current_follow
                              (SymbolSet.remove Epsilon first_rest) in

              (* If ε ∈ FIRST(rest) or rest is empty, add FOLLOW(lhs) to FOLLOW(sym) *)
              let new_follow' =
                if rest = [] || SymbolSet.mem Epsilon first_rest then
                  SymbolSet.union new_follow (follow_of_symbol acc lhs)
                else
                  new_follow
              in

              let acc' = SymbolMap.add sym new_follow' acc in
              process_rhs acc' rest
      in

      process_rhs acc_map rhs
    ) follow_map all_productions in

    (* Check if anything changed *)
    let changed = SymbolMap.exists (fun sym new_set ->
      let old_set = follow_of_symbol follow_map sym in
      not (SymbolSet.equal old_set new_set)
    ) new_follow_map in

    if changed then iterate new_follow_map
    else new_follow_map
  in

  iterate initial_follow

(** Print FIRST sets *)
let print_first_sets first_map grammar =
  print_endline "\nFIRST sets:";
  let nonterminals = get_nonterminals grammar in
  SymbolSet.iter (fun nt ->
    print_string ("FIRST(" ^ symbol_to_string nt ^ ") = ");
    print_symbol_set (first_of_symbol first_map nt)
  ) nonterminals

(** Print FOLLOW sets *)
let print_follow_sets follow_map grammar =
  print_endline "\nFOLLOW sets:";
  let nonterminals = get_nonterminals grammar in
  SymbolSet.iter (fun nt ->
    print_string ("FOLLOW(" ^ symbol_to_string nt ^ ") = ");
    print_symbol_set (follow_of_symbol follow_map nt)
  ) nonterminals
