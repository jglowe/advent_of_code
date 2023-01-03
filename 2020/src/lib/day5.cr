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

def decode_row_column(line : String) : Tuple(Int64, Int64)
  row_increment = Int64.new 64
  column_increment = Int64.new 4

  row_max = Int64.new 127
  row_min = Int64.new 0

  column_max = Int64.new 7
  column_min = Int64.new 0

  line.each_char do |c|
    case c
    when 'F'
      row_max -= row_increment
      row_increment //= 2
    when 'B'
      row_min += row_increment
      row_increment //= 2
    when 'R'
      column_min += column_increment
      column_increment //= 2
    when 'L'
      column_max -= column_increment
      column_increment //= 2
    else
      raise "Invalid char \"#{c}\" in line \"#{line}\""
    end
  end

  if row_min != row_max
    raise "Max row #{row_max} != Min row #{row_min}"
  end

  if column_min != column_max
    raise "Max column #{column_max} != Min column #{column_min}"
  end

  return row_min, column_min
end

def part1(input : String) : Int64
  max_seat_id = Int64.new 0

  input.lines.each do |line|
    row, column = decode_row_column(line)
    seat_id = row * 8 + column
    max_seat_id = Math.max(seat_id, max_seat_id)
  end

  return max_seat_id
end

def part2(input : String) : Int64
  seat_ids = Set(Int64).new
  max_seat_id = Int64.new 0

  input.lines.each do |line|
    row, column = decode_row_column(line)
    seat_id = row * 8 + column
    max_seat_id = Math.max(seat_id, max_seat_id)
    seat_ids.add seat_id
  end

  seat_id = Int64.new 0
  while seat_id < max_seat_id
    if !seat_id.in?(seat_ids) && (seat_id - 1).in?(seat_ids) && (seat_id + 1).in?(seat_ids)
      return seat_id
    end

    seat_id += 1
  end

  return Int64.new 0
end
