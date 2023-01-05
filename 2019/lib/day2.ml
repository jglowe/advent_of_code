let compute_program program =
  let rec helper index program =
    match Array.get program index with
    | 1 ->
        let first_index = Array.get program (index + 1) in
        let second_index = Array.get program (index + 2) in
        let third_index = Array.get program (index + 3) in
        let sum =
          Array.get program first_index + Array.get program second_index
        in
        Array.set program third_index sum ;
        helper (index + 4) program
    | 2 ->
        let first_index = Array.get program (index + 1) in
        let second_index = Array.get program (index + 2) in
        let third_index = Array.get program (index + 3) in
        let mult =
          Array.get program first_index * Array.get program second_index
        in
        Array.set program third_index mult ;
        helper (index + 4) program
    | 99 -> program
    | _ -> assert false
  in
  helper 0 program

let part1 strings =
  let program =
    match strings with
    | [] -> assert false
    | first_line :: [] ->
        String.split_on_char ',' first_line
        |> List.map int_of_string |> Array.of_list
    | _ -> assert false
  in
  Array.set program 1 12 ;
  Array.set program 2 2 ;
  Array.get (compute_program program) 0

let part2 strings =
  let program =
    match strings with
    | [] -> assert false
    | first_line :: [] ->
        String.split_on_char ',' first_line
        |> List.map int_of_string |> Array.of_list
    | _ -> assert false
  in
  let rec brute_force input1 input2 =
    let program = Array.copy program in
    Array.set program 1 input1 ;
    Array.set program 2 input2 ;
    if Array.get (compute_program program) 0 = 19690720 then
      (input1 * 100) + input2
    else if input2 == 99 then brute_force (input1 + 1) 0
    else brute_force input1 (input2 + 1)
  in
  brute_force 0 0

let%test "compute_program ex1" =
  compute_program [|1; 0; 0; 0; 99|] = [|2; 0; 0; 0; 99|]

let%test "compute_program ex2" =
  compute_program [|2; 4; 4; 5; 99; 0|] = [|2; 4; 4; 5; 99; 9801|]
