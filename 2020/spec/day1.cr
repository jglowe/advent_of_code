require "spec"
require "../src/lib/day1.cr"

describe "Day1" do
  it "part 1" do
    input = "1721
979
366
299
675
1456\n"

    list, set = parse_input input

    (514579).should eq(part1(list, set))
  end

  it "part 2" do
    input = "1721
979
366
299
675
1456\n"

    list, set = parse_input input

    (241861950).should eq(part2(list, set))
  end
end
