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

open Advent

let file = open_in "data/day6.txt"

let lines = Fileio.file_lines file

let _ = Printf.printf "Part 1 : %d\n" (Day6.part1 lines)

let _ = Printf.printf "Part 2 : %d\n" (Day6.part2 lines)
