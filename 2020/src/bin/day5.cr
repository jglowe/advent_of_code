################################################################################
#                               _             _____
#                              | |           | ____|
#                            __| | __ _ _   _| |__
#                           / _` |/ _` | | | |___ \
#                          | (_| | (_| | |_| |___) |
#                           \__,_|\__,_|\__, |____/
#                                        __/ |
#                                       |___/
#
# Jonathan Lowe
# github : https://github.com/jglowe
#
# The file for day5 advent of code 2020
################################################################################

require "../lib/day5.cr"

input = File.read("data/day5.txt")

puts "Part 1 : #{part1(input)}"
puts "Part 2 : #{part2(input)}"
