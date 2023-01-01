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

def parse_input(input : String) : Tuple(Array(Int64), Set(Int64))
  list = Array(Int64).new
  set = Set(Int64).new

  input.strip.each_line do |line|
    number = Int64.new line
    list.push number
    set.add number
  end

  return list, set
end

def part1(list : Array(Int64), set : Set(Int64)) : Int64
  list.each do |number|
    if set.includes?(2020 - number)
      return number * (2020 - number)
    end
  end

  return Int64.new 0
end

def part2(list : Array(Int64), set : Set(Int64)) : Int64
  list.each_cartesian list do |n1, n2|
    if set.includes?(2020 - n1 - n2)
      return n1 * n2 * (2020 - n1 - n2)
    end
  end

  return Int64.new 0
end
