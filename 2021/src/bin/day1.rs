///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                 _            __
//                                | |          /_ |
//                              __| | __ _ _   _| |
//                             / _` |/ _` | | | | |
//                            | (_| | (_| | |_| | |
//                             \__,_|\__,_|\__, |_|
//                                          __/ |
//                                         |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day1 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn part1(input: &str) -> usize {
    let mut previous_number: Option<i64> = None;
    let mut increases: usize = 0;

    for line in input.trim().split("\n") {
        let number: i64 = line.parse().unwrap();
        if let Some(p) = previous_number {
            if number > p {
                increases += 1;
            }
        }
        previous_number = Some(number);
    }

    increases
}

fn part2(input: &str) -> usize {
    let mut previous_sum: Option<i32> = None;
    let mut increases: usize = 0;
    let mut window: Vec<i32> = Vec::new();

    for line in input.trim().split("\n") {
        let number: i32 = match line.parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        window.push(number);

        if window.len() < 3 {
            continue;
        } else if window.len() > 3 {
            window.remove(0);
        }

        let sum = window[0] + window[1] + window[2];

        if let Some(p) = previous_sum {
            if sum > p {
                increases += 1;
            }
        }

        previous_sum = Some(sum);
    }

    increases
}

fn main() {
    let input = match fs::read_to_string("data/day1_input.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("Error {}", e),
    };

    println!("Part 1 : {}", part1(&input));
    println!("Part 2 : {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "199\n\
                     200\n\
                     208\n\
                     210\n\
                     200\n\
                     207\n\
                     240\n\
                     269\n\
                     260\n\
                     263\n";

        assert_eq!(7, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "199\n\
                     200\n\
                     208\n\
                     210\n\
                     200\n\
                     207\n\
                     240\n\
                     269\n\
                     260\n\
                     263\n";

        assert_eq!(5, part2(input));
    }
}
