///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _             _  _
//                              | |           | || |
//                            __| | __ _ _   _| || |_
//                           / _` |/ _` | | | |__   _|
//                          | (_| | (_| | |_| |  | |
//                           \__,_|\__,_|\__, |  |_|
//                                        __/ |
//                                       |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day4 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn new(min: i64, max: i64) -> Range {
        assert!(min <= max, "Range not properly formatted");

        Range { min, max }
    }

    fn from_string(range_input: &str) -> Range {
        let range: Vec<&str> = range_input.split("-").collect();

        assert!(range.len() == 2, "Range not properly formatted");

        let min: i64 = range[0].parse().unwrap();
        let max: i64 = range[1].parse().unwrap();

        Range::new(min, max)
    }

    fn either_contain(&self, other: &Range) -> bool {
        (self.min <= other.min && self.max >= other.max)
            || (other.min <= self.min && other.max >= self.max)
    }

    fn range_in_range((range1, range2): &(Range, Range)) -> bool {
        range1.either_contain(range2)
    }

    fn is_overlaping(&self, other: &Range) -> bool {
        if self.min < other.min {
            self.max >= other.min
        } else {
            other.max >= self.min
        }
    }

    fn ranges_overlap((range1, range2): &(Range, Range)) -> bool {
        range1.is_overlaping(range2)
    }
}

fn line_to_range_tuple(line: &str) -> (Range, Range) {
    let ranges: Vec<&str> = line.split(",").collect();

    assert!(ranges.len() == 2, "Range not properly formatted");

    (Range::from_string(ranges[0]), Range::from_string(ranges[1]))
}

fn parse_ranges(input: String) -> Vec<(Range, Range)> {
    input.trim().split("\n").map(line_to_range_tuple).collect()
}

fn bool_to_i64(b: bool) -> i64 {
    if b {
        1
    } else {
        0
    }
}

fn contained_number(ranges: &Vec<(Range, Range)>) -> i64 {
    ranges
        .iter()
        .map(Range::range_in_range)
        .map(bool_to_i64)
        .sum()
}

fn overlapping_number(ranges: &Vec<(Range, Range)>) -> i64 {
    ranges
        .iter()
        .map(Range::ranges_overlap)
        .map(bool_to_i64)
        .sum()
}

fn main() {
    let input = advent::read_file("data/day4_input.txt");
    let ranges = parse_ranges(input);

    println!(
        "Ranges contained within other ranges {}",
        contained_number(&ranges)
    );
    println!(
        "Ranges overlapping within other ranges {}",
        overlapping_number(&ranges)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        let input = "2-4,6-8\n\
                     2-3,4-5\n\
                     5-7,7-9\n\
                     2-8,3-7\n\
                     6-6,4-6\n\
                     2-6,4-8"
            .to_string();

        let ranges = parse_ranges(input);

        assert_eq!(2, contained_number(&ranges));
        assert_eq!(4, overlapping_number(&ranges));
    }
}
