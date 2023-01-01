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

class Rule
  getter min : Int64
  getter max : Int64
  getter letter : Char

  def initialize(@min, @max, @letter)
  end

  def initialize(input : String)
    parts = input.split " "

    if parts.size != 2
      raise "Line incorrectly formatted"
    end

    @letter = parts[1][0]

    parts = parts[0].split "-"

    if parts.size != 2
      raise "Line incorrectly formatted"
    end

    @min = Int64.new parts[0]
    @max = Int64.new parts[1]
  end
end

class Password
  getter rule : Rule
  getter password : String

  def initialize(@rule, @password)
  end

  def initialize(line : String)
    line_parts = line.split ": "

    if line_parts.size != 2
      raise "Line incorrectly formatted"
    end

    @rule = Rule.new line_parts[0]
    @password = line_parts[1]
  end

  def is_valid? : Bool
    count = @password.count @rule.letter

    return count >= @rule.min && count <= @rule.max
  end

  def is_valid_part2? : Bool
    first = @rule.letter == password[@rule.min - 1]
    second = @rule.letter == password[@rule.max - 1]

    return (first && !second) || (!first && second)
  end
end

def parse_input(input : String) : Array(Password)
  passwords = Array(Password).new

  input.strip.each_line do |line|
    passwords.push(Password.new(line))
  end

  return passwords
end

def part1(input : String) : Int64
  passwords = parse_input input

  valid = passwords.map do |password|
    if password.is_valid?
      Int64.new 1
    else
      Int64.new 0
    end
  end

  return valid.reduce(Int64.new(0)) { |acc, i| acc + i }
end

def part2(input : String) : Int64
  passwords = parse_input input

  valid = passwords.map do |password|
    if password.is_valid_part2?
      Int64.new 1
    else
      Int64.new 0
    end
  end

  return valid.reduce(Int64.new(0)) { |acc, i| acc + i }
end
