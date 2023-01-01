################################################################################
#                                 _            __
#                                | |          /_ |
#                              __| | __ _ _   _| |
#                             / _` |/ _` | | | | |
#                            | (_| | (_| | |_| | |
#                             \__,_|\__,_|\__, |_|
#                                          __/ |
#                                         |___/
#
# Jonathan Lowe
# github : https://github.com/jglowe
#
# The file for day1 advent of code 2020
################################################################################

require "../lib/day1.cr"

input = File.read("data/day1.txt")

list, set = parse_input input

puts "Part 1 : #{part1(list, set)}"
puts "Part 2 : #{part2(list, set)}"
