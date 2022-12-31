///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                _             ___
//                               | |           |__ \
//                             __| | __ _ _   _   ) |
//                            / _` |/ _` | | | | / /
//                           | (_| | (_| | |_| |/ /_
//                            \__,_|\__,_|\__, |____|
//                                         __/ |
//                                        |___/
//
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day2 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn part1(input: &str) -> i64 {
    let mut depth: i64 = 0;
    let mut horizontal: i64 = 0;

    for line in input.split("\n") {
        let split_line: Vec<&str> = line.split(" ").collect();
        if split_line.len() != 2 {
            continue;
        }

        let command = split_line[0];

        let number: i64 = split_line[1].parse().unwrap();

        match command {
            "up" => depth -= number,
            "down" => depth += number,
            "forward" => horizontal += number,
            _ => println!("Unmatch command"),
        }
    }

    horizontal * depth
}

fn part2(input: &str) -> i64 {
    let mut depth: i64 = 0;
    let mut horizontal: i64 = 0;
    let mut aim: i64 = 0;

    for line in input.split("\n") {
        let split_line: Vec<&str> = line.split(" ").collect();
        if split_line.len() != 2 {
            continue;
        }

        let command = split_line[0];

        let number: i64 = split_line[1].parse().unwrap();

        match command {
            "up" => aim -= number,
            "down" => aim += number,
            "forward" => {
                horizontal += number;
                depth += number * aim
            }
            _ => println!("Unmatch command"),
        }
    }

    horizontal * depth
}

fn main() {
    let input = match fs::read_to_string("data/day2_input.txt") {
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
        let input = "forward 5\n\
                     down 5\n\
                     forward 8\n\
                     up 3\n\
                     down 8\n\
                     forward 2\n";

        assert_eq!(150, part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = "forward 5\n\
                     down 5\n\
                     forward 8\n\
                     up 3\n\
                     down 8\n\
                     forward 2\n";

        assert_eq!(900, part2(&input));
    }
}
