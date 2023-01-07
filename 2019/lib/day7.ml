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

type process = {program : int array; mutable counter : int}

let compute_program process input =
  let rec compute_program process input output halted =
    let instruction = Array.get process.program process.counter in
    let param1_mode = instruction / 100 mod 10 in
    let param2_mode = instruction / 1000 mod 10 in
    let param3_mode = instruction / 10000 mod 10 in
    let get_parameter mode index =
      match mode with
      | 0 -> Array.get process.program index
      | 1 -> index
      | _ -> assert false
    in
    match instruction mod 100 with
    | 1 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let register2 = Array.get process.program (process.counter + 2) in
        let register3 = Array.get process.program (process.counter + 3) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        let sum = param1 + param2 in
        ( match param3_mode with
        | 0 -> Array.set process.program register3 sum
        | _ -> assert false ) ;
        process.counter <- process.counter + 4 ;
        compute_program process input output halted
    | 2 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let register2 = Array.get process.program (process.counter + 2) in
        let register3 = Array.get process.program (process.counter + 3) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        let mult = param1 * param2 in
        ( match param3_mode with
        | 0 -> Array.set process.program register3 mult
        | _ -> assert false ) ;
        process.counter <- process.counter + 4 ;
        compute_program process input output halted
    | 3 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let value, remainder =
          match input with hd :: tl -> (hd, tl) | [] -> assert false
        in
        Array.set process.program register1 value ;
        process.counter <- process.counter + 2 ;
        compute_program process remainder output halted
    | 4 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let output = Array.get process.program register1 in
        process.counter <- process.counter + 2 ;
        (Some output, false)
    | 5 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let register2 = Array.get process.program (process.counter + 2) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 > 0 then (
          process.counter <- param2 ;
          compute_program process input output halted )
        else (
          process.counter <- process.counter + 3 ;
          compute_program process input output halted )
    | 6 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let register2 = Array.get process.program (process.counter + 2) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 = 0 then (
          process.counter <- param2 ;
          compute_program process input output halted )
        else (
          process.counter <- process.counter + 3 ;
          compute_program process input output halted )
    | 7 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let register2 = Array.get process.program (process.counter + 2) in
        let register3 = Array.get process.program (process.counter + 3) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 < param2 then Array.set process.program register3 1
        else Array.set process.program register3 0 ;
        process.counter <- process.counter + 4 ;
        compute_program process input output halted
    | 8 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let register2 = Array.get process.program (process.counter + 2) in
        let register3 = Array.get process.program (process.counter + 3) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 = param2 then Array.set process.program register3 1
        else Array.set process.program register3 0 ;
        process.counter <- process.counter + 4 ;
        compute_program process input output halted
    | 99 -> (output, true)
    | _ -> assert false
  in
  compute_program process input None false

let add_combinations_to_stack stack remaining sequence =
  List.iter
    (fun mode ->
      let sequence = sequence @ [mode] in
      let remaining = List.filter (fun a -> a != mode) remaining in
      Stack.push (remaining, sequence) stack )
    remaining

let process_first_round processes sequence =
  let output, _, _ =
    Array.fold_left
      (fun (input, halted, ith) process ->
        if halted then (input, halted, ith)
        else
          let output, halted =
            compute_program process [List.nth sequence ith; input]
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

let part1 strings =
  let program =
    match strings with
    | [] -> assert false
    | first_line :: [] ->
        String.split_on_char ',' first_line
        |> List.map int_of_string |> Array.of_list
    | _ -> assert false
  in
  let stack = Stack.create () in
  Stack.push ([0; 1; 2; 3; 4], []) stack ;
  let rec process_stack max_thrust =
    if Stack.is_empty stack then max_thrust
    else
      let remaining, sequence = Stack.pop stack in
      match remaining with
      | hd :: tl ->
          add_combinations_to_stack stack (hd :: tl) sequence ;
          process_stack max_thrust
      | [] ->
          let processes =
            [| {program = Array.copy program; counter = 0}
             ; {program = Array.copy program; counter = 0}
             ; {program = Array.copy program; counter = 0}
             ; {program = Array.copy program; counter = 0}
             ; {program = Array.copy program; counter = 0} |]
          in
          let output = process_first_round processes sequence in
          process_stack (max output max_thrust)
  in
  process_stack min_int

let rec process_till_halted input processes =
  let output, halted, _ =
    Array.fold_left
      (fun (input, halted, ith) process ->
        if halted then (input, halted, ith)
        else
          let output, halted = compute_program process [input] in
          Array.set processes ith process ;
          if halted then (input, halted, ith)
          else
            match output with
            | Some output -> (output, halted, ith + 1)
            | None -> assert false )
      (input, false, 0) processes
  in
  if halted then input else process_till_halted output processes

let part2 strings =
  let program =
    match strings with
    | [] -> assert false
    | first_line :: [] ->
        String.split_on_char ',' first_line
        |> List.map int_of_string |> Array.of_list
    | _ -> assert false
  in
  let stack = Stack.create () in
  Stack.push ([5; 6; 7; 8; 9], []) stack ;
  let rec process_stack max_thrust =
    if Stack.is_empty stack then max_thrust
    else
      let remaining, sequence = Stack.pop stack in
      match remaining with
      | hd :: tl ->
          add_combinations_to_stack stack (hd :: tl) sequence ;
          process_stack max_thrust
      | [] ->
          let processes =
            [| {program = Array.copy program; counter = 0}
             ; {program = Array.copy program; counter = 0}
             ; {program = Array.copy program; counter = 0}
             ; {program = Array.copy program; counter = 0}
             ; {program = Array.copy program; counter = 0} |]
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
