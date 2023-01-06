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

open Advent

let file = open_in "data/day4.txt"

let lines = Fileio.file_lines file

let _ = Printf.printf "Part 1 : %d\n" (Day4.part1 lines)

let _ = Printf.printf "Part 2 : %d\n" (Day4.part2 lines)
