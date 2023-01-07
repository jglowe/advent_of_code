(*******************************************************************************
 *                               _             ___
 *                              | |           |__ \
 *                            __| | __ _ _   _   ) |
 *                           / _` |/ _` | | | | / /
 *                          | (_| | (_| | |_| |/ /_
 *                           \__,_|\__,_|\__, |____|
 *                                        __/ |
 *                                       |___/
 *
 * Jonathan Lowe
 * github : https://github.com/jglowe
 *
 * The file for day2 of advent of code 2019
 ******************************************************************************)

let compute_program program =
  let rec compute_program index program =
    match Array.get program index with
    | 1 ->
        let register1 = Array.get program (index + 1) in
        let register2 = Array.get program (index + 2) in
        let register3 = Array.get program (index + 3) in
        let sum =
          Array.get program register1 + Array.get program register2
        in
        Array.set program register3 sum ;
        compute_program (index + 4) program
    | 2 ->
        let register1 = Array.get program (index + 1) in
        let register2 = Array.get program (index + 2) in
        let register3 = Array.get program (index + 3) in
        let mult =
          Array.get program register1 * Array.get program register2
        in
        Array.set program register3 mult ;
        compute_program (index + 4) program
    | 99 -> program
    | _ -> assert false
  in
  compute_program 0 program

let part1 strings =
  let program = Inicode.parse_program strings in
  Array.set program 1 12 ;
  Array.set program 2 2 ;
  let process = Inicode.new_process program in
  let _ = Inicode.compute_process process [] in
  Array.get (process.program) 0

let part2 strings =
  let program = Inicode.parse_program strings in
  let rec brute_force input1 input2 =
    let program = Array.copy program in
    Array.set program 1 input1 ;
    Array.set program 2 input2 ;
    let process = Inicode.new_process program in
    let _ = Inicode.compute_process process [] in
    if Array.get process.program 0 = 19690720 then
      (input1 * 100) + input2
    else if input2 == 99 then brute_force (input1 + 1) 0
    else brute_force input1 (input2 + 1)
  in
  brute_force 0 0

let%test "compute_program ex1" =
  compute_program [|1; 0; 0; 0; 99|] = [|2; 0; 0; 0; 99|]

let%test "compute_program ex2" =
  compute_program [|2; 4; 4; 5; 99; 0|] = [|2; 4; 4; 5; 99; 9801|]
