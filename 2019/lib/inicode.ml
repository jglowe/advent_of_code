let parse_program strings =
  match strings with
  | [] -> assert false
  | first_line :: [] ->
      String.split_on_char ',' first_line
      |> List.map int_of_string |> Array.of_list
  | _ -> assert false

type process = {program : int array; mutable counter : int}

let new_process program = {program = Array.copy program; counter = 0}

let compute_process process input =
  let rec compute_process process input output halted =
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
        compute_process process input output halted
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
        compute_process process input output halted
    | 3 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let value, remainder =
          match input with hd :: tl -> (hd, tl) | [] -> assert false
        in
        Array.set process.program register1 value ;
        process.counter <- process.counter + 2 ;
        compute_process process remainder output halted
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
          compute_process process input output halted )
        else (
          process.counter <- process.counter + 3 ;
          compute_process process input output halted )
    | 6 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let register2 = Array.get process.program (process.counter + 2) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 = 0 then (
          process.counter <- param2 ;
          compute_process process input output halted )
        else (
          process.counter <- process.counter + 3 ;
          compute_process process input output halted )
    | 7 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let register2 = Array.get process.program (process.counter + 2) in
        let register3 = Array.get process.program (process.counter + 3) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 < param2 then Array.set process.program register3 1
        else Array.set process.program register3 0 ;
        process.counter <- process.counter + 4 ;
        compute_process process input output halted
    | 8 ->
        let register1 = Array.get process.program (process.counter + 1) in
        let register2 = Array.get process.program (process.counter + 2) in
        let register3 = Array.get process.program (process.counter + 3) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 = param2 then Array.set process.program register3 1
        else Array.set process.program register3 0 ;
        process.counter <- process.counter + 4 ;
        compute_process process input output halted
    | 99 -> (output, true)
    | _ -> assert false
  in
  compute_process process input None false
