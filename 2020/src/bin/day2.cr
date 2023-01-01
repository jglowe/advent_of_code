################################################################################
#                                _             ___
#                               | |           |__ \
#                             __| | __ _ _   _   ) |
#                            / _` |/ _` | | | | / /
#                           | (_| | (_| | |_| |/ /_
#                            \__,_|\__,_|\__, |____|
#                                         __/ |
#                                        |___/
#
# Jonathan Lowe
# github : https://github.com/jglowe
#
# The file for day2 advent of code 2020
################################################################################

require "../lib/day2.cr"

input = File.read("data/day2.txt")

puts "Part 1 : #{part1(input)}"
puts "Part 2 : #{part2(input)}"
