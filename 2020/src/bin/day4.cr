################################################################################
#                               _             _  _
#                              | |           | || |
#                            __| | __ _ _   _| || |_
#                           / _` |/ _` | | | |__   _|
#                          | (_| | (_| | |_| |  | |
#                           \__,_|\__,_|\__, |  |_|
#                                        __/ |
#                                       |___/
#
# Jonathan Lowe
# github : https://github.com/jglowe
#
# The file for day4 advent of code 2020
################################################################################

require "../lib/day4.cr"

input = File.read("data/day4.txt")

puts "Part 1 : #{part1(input)}"
puts "Part 2 : #{part2(input)}"
