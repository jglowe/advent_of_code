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

open Advent

let file = open_in "data/day5.txt"

let lines = Fileio.file_lines file

let _ = Printf.printf "Part 1 : %d\n" (Day5.part1 lines)

let _ = Printf.printf "Part 2 : %d\n" (Day5.part2 lines)
