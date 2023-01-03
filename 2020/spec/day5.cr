require "spec"
require "../src/lib/day5.cr"

describe "Day5" do
  it "part 1" do
    input = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL\n"

    (820).should eq(part1(input))
  end
end
