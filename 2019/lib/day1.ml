let calculate_fuel mass = if (mass / 3) - 2 < 0 then 0 else (mass / 3) - 2

let calculate_fuels_fuel mass =
  let rec calculate_fuel_fuel total_fuel mass =
    let fuel = calculate_fuel mass in
    if fuel > 0 then calculate_fuel_fuel (total_fuel + fuel) fuel
    else total_fuel
  in
  calculate_fuel_fuel 0 mass

let part1 strings =
  let masses = List.map int_of_string strings in
  let fuels = List.map calculate_fuel masses in
  List.fold_left ( + ) 0 fuels

let part2 strings =
  let masses = List.map int_of_string strings in
  let fuels = List.map calculate_fuels_fuel masses in
  List.fold_left ( + ) 0 fuels

let%test "Mass Calculation 1" = calculate_fuel 12 = 2

let%test "Mass Calculation 2" = calculate_fuel 14 = 2

let%test "Mass Calculation 3" = calculate_fuel 1969 = 654

let%test "Mass Calculation 4" = calculate_fuel 100756 = 33583

let%test "part1" = part1 ["12"; "14"; "1969"; "100756"] = 34241

let%test "Mass Calculation 1" = calculate_fuels_fuel 12 = 2

let%test "Mass Calculation 2" = calculate_fuels_fuel 14 = 2

let%test "Mass Calculation 3" = calculate_fuels_fuel 1969 = 966

let%test "Mass Calculation 4" = calculate_fuels_fuel 100756 = 50346

let%test "part2" = part1 ["12"; "14"; "1969"; "100756"] = 34241
