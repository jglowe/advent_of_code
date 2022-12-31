///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _             ____
//                              | |           |___ \
//                            __| | __ _ _   _  __) |
//                           / _` |/ _` | | | ||__ <
//                          | (_| | (_| | |_| |___) |
//                           \__,_|\__,_|\__, |____/
//                                        __/ |
//                                       |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day3 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn part1(input: &str) -> i64 {
    let mut line_length: usize = 0;
    let mut zero_bits: Vec<i32> = Vec::new();
    let mut one_bits: Vec<i32> = Vec::new();

    for (i, line) in input.split("\n").enumerate() {
        if i == 0 {
            line_length = line.len();
            for _ in 0..line_length {
                zero_bits.push(0);
                one_bits.push(0);
            }
        }

        for (i, c) in line.chars().enumerate() {
            if c == '0' {
                zero_bits[i] += 1;
            }
            if c == '1' {
                one_bits[i] += 1;
            }
        }
    }

    let mut gamma_rate = 0;
    let mut epilon_rate = 0;

    for i in 0..line_length {
        gamma_rate = gamma_rate << 1;
        epilon_rate = epilon_rate << 1;

        if one_bits[i] > zero_bits[i] {
            gamma_rate += 1;
        } else {
            epilon_rate += 1;
        }
    }

    gamma_rate * epilon_rate
}

fn part2(input: &str) -> i64 {
    let mut line_length: usize = 0;

    for line in input.split("\n") {
        line_length = line.len();
        break;
    }

    let mut oxygen_numbers: Vec<i64> = Vec::new();
    let mut co2_numbers: Vec<i64> = Vec::new();

    for line in input.split("\n") {
        if line == "" {
            continue;
        }
        let mut number: i64 = 0;
        for digit in line.chars() {
            number = number << 1;
            if digit == '1' {
                number += 1;
            }
        }
        oxygen_numbers.push(number);
        co2_numbers.push(number);
    }

    for i in 0..line_length {
        let bit_of_concern: i64 = 1 << line_length - i - 1;

        if oxygen_numbers.len() > 1 {
            let mut zeros = 0;
            let mut ones = 0;
            for number in &oxygen_numbers {
                if number & bit_of_concern != 0 {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }

            if ones >= zeros {
                oxygen_numbers.retain(|num| num & bit_of_concern != 0)
            } else {
                oxygen_numbers.retain(|num| num & bit_of_concern == 0)
            }
        }

        if co2_numbers.len() > 1 {
            let mut zeros = 0;
            let mut ones = 0;
            for number in &co2_numbers {
                if number & bit_of_concern != 0 {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }

            if ones < zeros {
                co2_numbers.retain(|num| num & bit_of_concern != 0)
            } else {
                co2_numbers.retain(|num| num & bit_of_concern == 0)
            }
        }
    }

    oxygen_numbers[0] * co2_numbers[0]
}

fn main() {
    let input = match fs::read_to_string("data/day3_input.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("Error {}", e),
    };

    println!("Part 1 : {}", part1(&input));
    println!("Part 2 : {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "00100\n\
                     11110\n\
                     10110\n\
                     10111\n\
                     10101\n\
                     01111\n\
                     00111\n\
                     11100\n\
                     10000\n\
                     11001\n\
                     00010\n\
                     01010\n";

        assert_eq!(198, part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = "00100\n\
                     11110\n\
                     10110\n\
                     10111\n\
                     10101\n\
                     01111\n\
                     00111\n\
                     11100\n\
                     10000\n\
                     11001\n\
                     00010\n\
                     01010\n";

        assert_eq!(230, part2(&input));
    }
}
