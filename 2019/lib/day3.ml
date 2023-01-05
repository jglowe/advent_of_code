type point = {x : int; y : int}

type section = North of int | South of int | East of int | West of int

module PointOrder = struct
  type t = point

  let compare point1 point2 =
    match Stdlib.compare point1.x point2.x with
    | 0 -> Stdlib.compare point1.y point2.y
    | c -> c
end

module PointSet = Set.Make (PointOrder)
module PointMap = Map.Make (PointOrder)

let string_to_path s =
  match String.get s 0 with
  | 'U' -> North (int_of_string (String.sub s 1 (String.length s - 1)))
  | 'D' -> South (int_of_string (String.sub s 1 (String.length s - 1)))
  | 'L' -> West (int_of_string (String.sub s 1 (String.length s - 1)))
  | 'R' -> East (int_of_string (String.sub s 1 (String.length s - 1)))
  | _ -> assert false

let rec add_line_to_set wire x y destx desty =
  if x != destx && y != desty then assert false
  else if x < destx then
    add_line_to_set (PointSet.add {x; y} wire) (x + 1) y destx desty
  else if x > destx then
    add_line_to_set (PointSet.add {x; y} wire) (x - 1) y destx desty
  else if y < desty then
    add_line_to_set (PointSet.add {x; y} wire) x (y + 1) destx desty
  else if y > desty then
    add_line_to_set (PointSet.add {x; y} wire) x (y - 1) destx desty
  else PointSet.add {x; y} wire

let populate_wire wire =
  let wire_paths = String.split_on_char ',' wire |> List.map string_to_path in
  let rec run_wire wire paths x y =
    match paths with
    | North num :: tl ->
        let wire = add_line_to_set wire x y x (y + num) in
        run_wire wire tl x (y + num)
    | South num :: tl ->
        let wire = add_line_to_set wire x y x (y - num) in
        run_wire wire tl x (y - num)
    | East num :: tl ->
        let wire = add_line_to_set wire x y (x + num) y in
        run_wire wire tl (x + num) y
    | West num :: tl ->
        let wire = add_line_to_set wire x y (x - num) y in
        run_wire wire tl (x - num) y
    | [] -> wire
  in
  run_wire PointSet.empty wire_paths 0 0

let part1 strings =
  let wire1, wire2 =
    match strings with [wire1; wire2] -> (wire1, wire2) | _ -> assert false
  in
  let wire1_set = populate_wire wire1 in
  let wire2_set = populate_wire wire2 in
  let intersections = PointSet.inter wire1_set wire2_set in
  let intersections = PointSet.remove {x = 0; y = 0} intersections in
  let find_min_distance point min =
    if Int.abs point.x + Int.abs point.y < min then
      Int.abs point.x + Int.abs point.y
    else min
  in
  PointSet.fold find_min_distance intersections Int.max_int

let find_intersection_dists wire intersections =
  let rec add_intersections_to_map map dist x y destx desty =
    let map =
      match PointSet.find_opt {x; y} intersections with
      | Some _ -> PointMap.add {x; y} dist map
      | None -> map
    in
    if x != destx && y != desty then assert false
    else if x < destx then
      add_intersections_to_map map (dist + 1) (x + 1) y destx desty
    else if x > destx then
      add_intersections_to_map map (dist + 1) (x - 1) y destx desty
    else if y < desty then
      add_intersections_to_map map (dist + 1) x (y + 1) destx desty
    else if y > desty then
      add_intersections_to_map map (dist + 1) x (y - 1) destx desty
    else (map, dist)
  in
  let wire_paths = String.split_on_char ',' wire |> List.map string_to_path in
  let rec follow_wire map paths dist x y =
    match paths with
    | North num :: tl ->
        let map, dist = add_intersections_to_map map dist x y x (y + num) in
        follow_wire map tl dist x (y + num)
    | South num :: tl ->
        let map, dist = add_intersections_to_map map dist x y x (y - num) in
        follow_wire map tl dist x (y - num)
    | East num :: tl ->
        let map, dist = add_intersections_to_map map dist x y (x + num) y in
        follow_wire map tl dist (x + num) y
    | West num :: tl ->
        let map, dist = add_intersections_to_map map dist x y (x - num) y in
        follow_wire map tl dist (x - num) y
    | [] -> map
  in
  follow_wire PointMap.empty wire_paths 0 0 0

let part2 strings =
  let wire1, wire2 =
    match strings with [wire1; wire2] -> (wire1, wire2) | _ -> assert false
  in
  let wire1_set = populate_wire wire1 in
  let wire2_set = populate_wire wire2 in
  let intersections = PointSet.inter wire1_set wire2_set in
  let intersections = PointSet.remove {x = 0; y = 0} intersections in
  let dists1 = find_intersection_dists wire1 intersections in
  let dists2 = find_intersection_dists wire2 intersections in
  let find_min_length point min =
    let dist1 = PointMap.find point dists1 in
    let dist2 = PointMap.find point dists2 in
    if dist1 + dist2 < min then dist1 + dist2 else min
  in
  PointSet.fold find_min_length intersections Int.max_int

let%test "part1 example 1" = part1 ["R8,U5,L5,D3"; "U7,R6,D4,L4"] = 6

let%test "part1 example 2" =
  part1 ["R75,D30,R83,U83,L12,D49,R71,U7,L72"; "U62,R66,U55,R34,D71,R55,D58,R83"]
  = 159

let%test "part1 example 3" =
  part1
    [ "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
    ; "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7" ]
  = 135

let%test "part2 example 1" = part2 ["R8,U5,L5,D3"; "U7,R6,D4,L4"] = 30

let%test "part2 example 2" =
  part2 ["R75,D30,R83,U83,L12,D49,R71,U7,L72"; "U62,R66,U55,R34,D71,R55,D58,R83"]
  = 610

let%test "part2 example 3" =
  part2
    [ "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
    ; "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7" ]
  = 410
