(*******************************************************************************
 *                               _            ______
 *                              | |          |____  |
 *                            __| | __ _ _   _   / /
 *                           / _` |/ _` | | | | / /
 *                          | (_| | (_| | |_| |/ /
 *                           \__,_|\__,_|\__, /_/
 *                                        __/ |
 *                                       |___/
 *
 * Jonathan Lowe
 * github : https://github.com/jglowe
 *
 * The file for day7 of advent of code 2019
 ******************************************************************************)

let add_combinations_to_stack stack remaining_modes sequence =
  List.iter
    (fun mode ->
      let sequence = sequence @ [mode] in
      let remaining_modes = List.filter (fun a -> a != mode) remaining_modes in
      Stack.push (remaining_modes, sequence) stack )
    remaining_modes

let process_first_round processes sequence =
  let output, _, _ =
    Array.fold_left
      (fun (input, halted, ith) process ->
        if halted then (input, halted, ith)
        else
          let output, halted =
            Inicode.compute_process process [List.nth sequence ith; input]
          in
          Array.set processes ith process ;
          if halted then (input, halted, ith)
          else
            match output with
            | Some output -> (output, halted, ith + 1)
            | None -> assert false )
      (0, false, 0) processes
  in
  output

let rec process_till_halted input processes =
  let output, halted, _ =
    Array.fold_left
      (fun (input, halted, ith) process ->
        if halted then (input, halted, ith)
        else
          let output, halted = Inicode.compute_process process [input] in
          Array.set processes ith process ;
          if halted then (input, halted, ith)
          else
            match output with
            | Some output -> (output, halted, ith + 1)
            | None -> assert false )
      (input, false, 0) processes
  in
  if halted then input else process_till_halted output processes

let parse_program strings =
  match strings with
  | [] -> assert false
  | first_line :: [] ->
      String.split_on_char ',' first_line
      |> List.map int_of_string |> Array.of_list
  | _ -> assert false

let part1 strings =
  let program = parse_program strings in
  let stack = Stack.create () in
  Stack.push ([0; 1; 2; 3; 4], []) stack ;
  let rec process_stack max_thrust =
    if Stack.is_empty stack then max_thrust
    else
      let remaining_modes, sequence = Stack.pop stack in
      match remaining_modes with
      | hd :: tl ->
          add_combinations_to_stack stack (hd :: tl) sequence ;
          process_stack max_thrust
      | [] ->
          let processes =
            [| Inicode.new_process program
             ; Inicode.new_process program
             ; Inicode.new_process program
             ; Inicode.new_process program
             ; Inicode.new_process program |]
          in
          let output = process_first_round processes sequence in
          process_stack (max output max_thrust)
  in
  process_stack min_int

let part2 strings =
  let program = parse_program strings in
  let stack = Stack.create () in
  Stack.push ([5; 6; 7; 8; 9], []) stack ;
  let rec process_stack max_thrust =
    if Stack.is_empty stack then max_thrust
    else
      let remaining_modes, sequence = Stack.pop stack in
      match remaining_modes with
      | hd :: tl ->
          add_combinations_to_stack stack (hd :: tl) sequence ;
          process_stack max_thrust
      | [] ->
          let processes =
            [| Inicode.new_process program
             ; Inicode.new_process program
             ; Inicode.new_process program
             ; Inicode.new_process program
             ; Inicode.new_process program |]
          in
          let output = process_first_round processes sequence in
          process_stack (max (process_till_halted output processes) max_thrust)
  in
  process_stack min_int

let%test "Phase program 1" =
  part1 ["3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"] = 43210

let%test "Phase program 2" =
  part1
    ["3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"]
  = 54321

let%test "Phase program 3" =
  part1
    [ "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
    ]
  = 65210

let%test "Part2 ex1 " =
  part2
    [ "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
    ]
  = 139629729

let%test "Part2 ex2 " =
  part2
    [ "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
    ]
  = 18216
