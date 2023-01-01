require "spec"
require "../src/lib/day2.cr"

describe "Day2" do
  it "part 1" do
    input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc\n"

    (2).should eq(part1(input))
  end

  it "part 2" do
    input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc\n"

    (1).should eq(part2(input))
  end
end
