(*******************************************************************************
 *                               _              ___
 *                              | |            / _ \
 *                            __| | __ _ _   _| (_) |
 *                           / _` |/ _` | | | |> _ <
 *                          | (_| | (_| | |_| | (_) |
 *                           \__,_|\__,_|\__, |\___/
 *                                        __/ |
 *                                       |___/
 *
 * Jonathan Lowe
 * github : https://github.com/jglowe
 *
 * The file for day8 of advent of code 2019
 ******************************************************************************)

let num_rows = 6

let num_columns = 25

let parse_line line =
  let rec parse_line line_index layer layers =
    if line_index = String.length line then layers
    else if
      line_index mod (num_columns * num_rows) = (num_columns * num_rows) - 1
    then
      let layer = layer @ [String.sub line line_index 1 |> int_of_string] in
      let layers = layers @ [layer] in
      parse_line (line_index + 1) [] layers
    else
      let layer = layer @ [String.sub line line_index 1 |> int_of_string] in
      parse_line (line_index + 1) layer layers
  in
  parse_line 0 [] []

let increment_if_number num acc number = if number = num then acc + 1 else acc

let find_layer_with_min_zeros layers =
  let counts =
    List.fold_left
      (fun acc layer ->
        let count = List.fold_left (increment_if_number 0) 0 layer in
        acc @ [count] )
      [] layers
  in
  let index_of_min_zero, _, _ =
    List.fold_left
      (fun (index, min_count, ith) count ->
        if count < min_count then (ith, count, ith + 1)
        else (index, min_count, ith + 1) )
      (0, max_int, 0) counts
  in
  index_of_min_zero

let print_layer layer =
  print_endline "" ; List.iter print_int layer ; print_endline ""

let render_image image =
  if List.length image != num_rows * num_columns then assert false
  else
    let _, image =
      List.fold_left
        (fun (ith, image) value ->
          let image = if ith mod num_columns = 0 then image ^ "\n" else image in
          let image = match value with
          | 0 -> image ^ " "
          | 1 -> image ^ "#"
          | 2 -> image ^ "T"
          | _ -> image ^ "E"
          in
          (ith + 1, image) )
        (0, "") image
    in
    image

let part1 strings =
  let line = match strings with [line] -> line | _ -> assert false in
  let layers = parse_line line in
  let min_zero_layer = find_layer_with_min_zeros layers in
  let layer = List.nth layers min_zero_layer in
  let ones_count = List.fold_left (increment_if_number 1) 0 layer in
  let twos_count = List.fold_left (increment_if_number 2) 0 layer in
  ones_count * twos_count

let part2 strings =
  let line = match strings with [line] -> line | _ -> assert false in
  let layers = parse_line line in
  let rendered_image = Array.of_list (List.nth layers 0) in
  List.iter
    (fun layer ->
      List.iteri
        (fun ith pixel ->
          if Array.get rendered_image ith = 2 then
            Array.set rendered_image ith pixel )
        layer )
    layers ;
  print_endline (render_image (Array.to_list rendered_image)) ;
  "none"
