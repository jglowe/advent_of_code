(*******************************************************************************
 *                                _             _____
 *                               | |           | ____|
 *                             __| | __ _ _   _| |__
 *                            / _` |/ _` | | | |___ \
 *                           | (_| | (_| | |_| |___) |
 *                            \__,_|\__,_|\__, |____/
 *                                         __/ |
 *                                        |___/
 *
 * Jonathan Lowe
 * github : https://github.com/jglowe
 *
 * The file for day5 of advent of code 2019
 ******************************************************************************)

let compute_program program input =
  let rec helper index program output =
    let instruction = Array.get program index in
    let param1_mode = instruction / 100 mod 10 in
    let param2_mode = instruction / 1000 mod 10 in
    let param3_mode = instruction / 10000 mod 10 in
    let get_parameter mode index =
      match mode with
      | 0 -> Array.get program index
      | 1 -> index
      | _ -> assert false
    in
    match instruction mod 100 with
    | 1 ->
        let register1 = Array.get program (index + 1) in
        let register2 = Array.get program (index + 2) in
        let register3 = Array.get program (index + 3) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        let sum = param1 + param2 in
        ( match param3_mode with
        | 0 -> Array.set program register3 sum
        | _ -> assert false ) ;
        helper (index + 4) program output
    | 2 ->
        let register1 = Array.get program (index + 1) in
        let register2 = Array.get program (index + 2) in
        let register3 = Array.get program (index + 3) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        let mult = param1 * param2 in
        ( match param3_mode with
        | 0 -> Array.set program register3 mult
        | _ -> assert false ) ;
        helper (index + 4) program output
    | 3 ->
        let register1 = Array.get program (index + 1) in
        Array.set program register1 input ;
        helper (index + 2) program output
    | 4 ->
        let register1 = Array.get program (index + 1) in
        let output = Array.get program register1 in
        helper (index + 2) program (Some output)
    | 5 ->
        let register1 = Array.get program (index + 1) in
        let register2 = Array.get program (index + 2) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 > 0 then helper param2 program output
        else helper (index + 3) program output
    | 6 ->
        let register1 = Array.get program (index + 1) in
        let register2 = Array.get program (index + 2) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 = 0 then helper param2 program output
        else helper (index + 3) program output
    | 7 ->
        let register1 = Array.get program (index + 1) in
        let register2 = Array.get program (index + 2) in
        let register3 = Array.get program (index + 3) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 < param2 then Array.set program register3 1
        else Array.set program register3 0 ;
        helper (index + 4) program output
    | 8 ->
        let register1 = Array.get program (index + 1) in
        let register2 = Array.get program (index + 2) in
        let register3 = Array.get program (index + 3) in
        let param1 = get_parameter param1_mode register1 in
        let param2 = get_parameter param2_mode register2 in
        if param1 = param2 then Array.set program register3 1
        else Array.set program register3 0 ;
        helper (index + 4) program output
    | 99 -> (program, output)
    | _ -> assert false
  in
  helper 0 program None


let part1 strings =
  let program = Inicode.parse_program strings in
  let process = Inicode.new_process program in
  let output, _ = Inicode.compute_process process [1] in
  match output with Some output -> output | None -> assert false

let part2 strings =
  let program = Inicode.parse_program strings in
  let process = Inicode.new_process program in
  let output, _ = Inicode.compute_process process [1] in
  match output with Some output -> output | None -> assert false

let%test "compute_program ex1" =
  compute_program [|3; 0; 4; 0; 99|] 1 = ([|1; 0; 4; 0; 99|], Some 1)

let%test "compute_program ex2" =
  compute_program [|1002; 4; 3; 4; 33|] 0 = ([|1002; 4; 3; 4; 99|], None)

let%test "compute_program ex3" =
  let _, output = compute_program [|3; 9; 8; 9; 10; 9; 4; 9; 99; -1; 8|] 8 in
  output = Some 1

let%test "compute_program ex4" =
  let _, output = compute_program [|3; 9; 8; 9; 10; 9; 4; 9; 99; -1; 8|] 7 in
  output = Some 0

let%test "compute_program ex5" =
  let _, output = compute_program [|3; 9; 7; 9; 10; 9; 4; 9; 99; -1; 8|] 7 in
  output = Some 1

let%test "compute_program ex6" =
  let _, output = compute_program [|3; 9; 7; 9; 10; 9; 4; 9; 99; -1; 8|] 8 in
  output = Some 0

let%test "compute_program ex7" =
  let _, output = compute_program [|3; 3; 1108; -1; 8; 3; 4; 3; 99|] 8 in
  output = Some 1

let%test "compute_program ex8" =
  let _, output = compute_program [|3; 3; 1108; -1; 8; 3; 4; 3; 99|] 7 in
  output = Some 0

let%test "compute_program ex9" =
  let _, output = compute_program [|3; 3; 1107; -1; 8; 3; 4; 3; 99|] 7 in
  output = Some 1

let%test "compute_program ex10" =
  let _, output = compute_program [|3; 3; 1107; -1; 8; 3; 4; 3; 99|] 9 in
  output = Some 0

let%test "compute_program ex11" =
  let _, output =
    compute_program
      [|3; 12; 6; 12; 15; 1; 13; 14; 13; 4; 13; 99; -1; 0; 1; 9|]
      9
  in
  output = Some 1

let%test "compute_program ex12" =
  let _, output =
    compute_program
      [|3; 12; 6; 12; 15; 1; 13; 14; 13; 4; 13; 99; -1; 0; 1; 9|]
      0
  in
  output = Some 0

let%test "compute_program ex13" =
  let _, output =
    compute_program [|3; 3; 1105; -1; 9; 1101; 0; 0; 12; 4; 12; 99; 1|] 0
  in
  output = Some 0

let%test "compute_program ex14" =
  let _, output =
    compute_program [|3; 3; 1105; -1; 9; 1101; 0; 0; 12; 4; 12; 99; 1|] 99
  in
  output = Some 1
