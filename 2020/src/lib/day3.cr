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

struct Point
  property row : Int64
  property column : Int64

  def initialize(@row, @column)
  end
end

struct Slope
  property row_inc : Int64
  property column_inc : Int64

  def initialize(@row_inc, @column_inc)
  end
end

alias TreeGrid = Set(Point)

def parse_tree_grid(input : String) : Tuple(TreeGrid, Int64, Int64)
  tree_grid = TreeGrid.new

  max_row = Int64.new 0
  max_column = Int64.new 0
  input.lines.each_with_index do |line, row|
    max_row = [Int64.new(row), max_row].max
    line.each_char_with_index do |char, column|
      max_column = [Int64.new(column), max_column].max
      case char
      when '#'
        point = Point.new row, column
        tree_grid.add point
      when '.'
      else
        raise "Invalid char"
      end
    end
  end

  return tree_grid, max_row, max_column
end

def conpute_trees(tree_grid : TreeGrid, slope : Slope, max_row : Int64,
                  max_column : Int64) : Int64
  trees_encountered = Int64.new 0

  point = Point.new 0, 0

  while point.row <= max_row
    point.row += slope.row_inc
    point.column += slope.column_inc

    point.column = point.column % (max_column + 1)

    if tree_grid.includes?(point)
      trees_encountered += 1
    end
  end

  return trees_encountered
end

def part1(input : String) : Int64
  tree_grid, max_row, max_column = parse_tree_grid input

  slope = Slope.new(Int64.new(1), Int64.new(3))

  return conpute_trees(tree_grid, slope, max_row, max_column)
end

def part2(input : String) : Int64
  tree_grid, max_row, max_column = parse_tree_grid input

  slope1 = Slope.new(Int64.new(1), Int64.new(1))
  slope2 = Slope.new(Int64.new(1), Int64.new(3))
  slope3 = Slope.new(Int64.new(1), Int64.new(5))
  slope4 = Slope.new(Int64.new(1), Int64.new(7))
  slope5 = Slope.new(Int64.new(2), Int64.new(1))

  return conpute_trees(tree_grid, slope1, max_row, max_column) \
       * conpute_trees(tree_grid, slope2, max_row, max_column) \
       * conpute_trees(tree_grid, slope3, max_row, max_column) \
       * conpute_trees(tree_grid, slope4, max_row, max_column) \
       * conpute_trees(tree_grid, slope5, max_row, max_column)
end
