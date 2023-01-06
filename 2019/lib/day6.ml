(*******************************************************************************
 *                               _               __
 *                              | |             / /
 *                            __| | __ _ _   _ / /_
 *                           / _` |/ _` | | | | '_ \
 *                          | (_| | (_| | |_| | (_) |
 *                           \__,_|\__,_|\__, |\___/
 *                                        __/ |
 *                                       |___/
 *
 * Jonathan Lowe
 * github : https://github.com/jglowe
 *
 * The file for day6 of advent of code 2019
 ******************************************************************************)

module StringMap = Map.Make (String)
module StringSet = Set.Make (String)

let parse_input strings =
  let rec helper remaining map set =
    match remaining with
    | hd :: tl -> (
      match String.split_on_char ')' hd with
      | [orbited; orbiter] ->
          let set = StringSet.add orbiter set in
          let set = StringSet.add orbited set in
          let map =
            match StringMap.find_opt orbiter map with
            | Some _ -> assert false
            | None -> StringMap.add orbiter orbited map
          in
          helper tl map set
      | _ -> assert false )
    | [] -> (map, set)
  in
  helper strings StringMap.empty StringSet.empty

let part1 strings =
  let map, set = parse_input strings in
  let rec find_orbit_length orbiter length =
    if orbiter = "COM" then length
    else
      let orbited = StringMap.find orbiter map in
      find_orbit_length orbited (length + 1)
  in
  let fold_function orbiter acc = acc + find_orbit_length orbiter 0 in
  StringSet.fold fold_function set 0

let part2 strings =
  let map, _ = parse_input strings in
  let your_orbit = StringMap.find "YOU" map in
  let rec find_path position path =
    if position = "COM" then path
    else
      let orbited = StringMap.find position map in
      find_path orbited (orbited :: path)
  in
  let path = find_path your_orbit [] in
  let santa_orbit = StringMap.find "SAN" map in
  let rec find_common_node position =
    let compare p = position = p in
    if List.exists compare path then position
    else
      let position = StringMap.find position map in
      find_common_node position
  in
  let rec count_path_till_node common_place node count =
    if node = common_place then count
    else
      let node = StringMap.find node map in
      count_path_till_node common_place node (count + 1)
  in
  let common_node = find_common_node santa_orbit in
  count_path_till_node common_node your_orbit 0
  + count_path_till_node common_node santa_orbit 0

let%test "part 1" =
  part1
    [ "COM)B"
    ; "B)C"
    ; "C)D"
    ; "D)E"
    ; "E)F"
    ; "B)G"
    ; "G)H"
    ; "D)I"
    ; "E)J"
    ; "J)K"
    ; "K)L" ]
  = 42

let%test "part 2" =
  part2
    [ "COM)B"
    ; "B)C"
    ; "C)D"
    ; "D)E"
    ; "E)F"
    ; "B)G"
    ; "G)H"
    ; "D)I"
    ; "E)J"
    ; "J)K"
    ; "K)L"
    ; "K)YOU"
    ; "I)SAN" ]
  = 4
