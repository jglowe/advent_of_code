open Advent

let file = open_in "data/day1.txt"

let lines = Utils.file_lines file

let _ = Printf.printf "Part 1 : %d\n" (Day1.part1 lines)

let _ = Printf.printf "Part 2 : %d\n" (Day1.part2 lines)
