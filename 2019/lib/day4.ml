(*******************************************************************************
 *                               _             _  _
 *                              | |           | || |
 *                            __| | __ _ _   _| || |_
 *                           / _` |/ _` | | | |__   _|
 *                          | (_| | (_| | |_| |  | |
 *                           \__,_|\__,_|\__, |  |_|
 *                                        __/ |
 *                                       |___/
 *
 * Jonathan Lowe
 * github : https://github.com/jglowe
 *
 * The file for day4 of advent of code 2019
 ******************************************************************************)

let is_valid_password number =
  ( number / 100000 mod 10 = number / 10000 mod 10
  || number / 10000 mod 10 = number / 1000 mod 10
  || number / 1000 mod 10 = number / 100 mod 10
  || number / 100 mod 10 = number / 10 mod 10
  || number / 10 mod 10 = number mod 10 )
  && number / 100000 mod 10 <= number / 10000 mod 10
  && number / 10000 mod 10 <= number / 1000 mod 10
  && number / 1000 mod 10 <= number / 100 mod 10
  && number / 100 mod 10 <= number / 10 mod 10
  && number / 10 mod 10 <= number mod 10

let is_valid_password_part2 number =
  ( number / 100000 mod 10 = number / 10000 mod 10
    && number / 10000 mod 10 != number / 1000 mod 10
  || number / 100000 mod 10 != number / 10000 mod 10
     && number / 10000 mod 10 = number / 1000 mod 10
     && number / 1000 mod 10 != number / 100 mod 10
  || number / 10000 mod 10 != number / 1000 mod 10
     && number / 1000 mod 10 = number / 100 mod 10
     && number / 100 mod 10 != number / 10 mod 10
  || number / 1000 mod 10 != number / 100 mod 10
     && number / 100 mod 10 = number / 10 mod 10
     && number / 10 mod 10 != number mod 10
  || number / 100 mod 10 != number / 10 mod 10
     && number / 10 mod 10 = number mod 10 )
  && number / 100000 mod 10 <= number / 10000 mod 10
  && number / 10000 mod 10 <= number / 1000 mod 10
  && number / 1000 mod 10 <= number / 100 mod 10
  && number / 100 mod 10 <= number / 10 mod 10
  && number / 10 mod 10 <= number mod 10

let compute_complant_passwords compliance_function strings =
  match strings with
  | [line] -> (
    match String.split_on_char '-' line with
    | [start; stop] ->
        let start, stop = (int_of_string start, int_of_string stop) in
        let rec iterate_through_numbers num stop acc =
          if num == stop then acc
          else if compliance_function num then
            iterate_through_numbers (num + 1) stop (acc + 1)
          else iterate_through_numbers (num + 1) stop acc
        in
        iterate_through_numbers start stop 0
    | _ -> assert false )
  | _ -> assert false

let part1 strings = compute_complant_passwords is_valid_password strings

let part2 strings = compute_complant_passwords is_valid_password_part2 strings

let%test "is_valid_password 1" = is_valid_password 111123

let%test "is_valid_password 2" = is_valid_password 135799

let%test "is_valid_password 3" = not (is_valid_password 223450)

let%test "is_valid_password 4" = not (is_valid_password 123789)

let%test "is_valid_password_part2 1" = is_valid_password_part2 112233

let%test "is_valid_password_part2 2" = not (is_valid_password_part2 123444)

let%test "is_valid_password_part2 3" = is_valid_password_part2 111122
