################################################################################
#                               _             ____
#                              | |           |___ \
#                            __| | __ _ _   _  __) |
#                           / _` |/ _` | | | ||__ <
#                          | (_| | (_| | |_| |___) |
#                           \__,_|\__,_|\__, |____/
#                                        __/ |
#                                       |___/
#
# Jonathan Lowe
# github : https://github.com/jglowe
#
# The file for day3 advent of code 2020
################################################################################

require "../lib/day3.cr"

input = File.read("data/day3.txt")

puts "Part 1 : #{part1(input)}"
puts "Part 2 : #{part2(input)}"
