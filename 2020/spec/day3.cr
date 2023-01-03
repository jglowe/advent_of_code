require "spec"
require "../src/lib/day3.cr"

describe "Day2" do
  it "part 1" do
    input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"

    (7).should eq(part1(input))
  end

  it "part 2" do
    input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"

    (336).should eq(part2(input))
  end
end
