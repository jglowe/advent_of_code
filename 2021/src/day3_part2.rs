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

fn main() {
    let filename = "data/day3_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let mut line_length: usize = 0;

    for line in file_contents.split("\n") {
        line_length = line.len();
        break;
    }

    let mut oxygen_numbers: Vec<i32> = Vec::new();
    let mut co2_numbers: Vec<i32> = Vec::new();

    for line in file_contents.split("\n") {
        if line == "" {
            continue;
        }
        let mut number: i32 = 0;
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
        let bit_of_concern: i32 = 1 << line_length - i - 1;

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
                oxygen_numbers.retain(|num| {
                    num & bit_of_concern != 0
                })
            } else {
                oxygen_numbers.retain(|num| {
                    num & bit_of_concern == 0
                })
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
                co2_numbers.retain(|num| {
                    num & bit_of_concern != 0
                })
            } else {
                co2_numbers.retain(|num| {
                    num & bit_of_concern == 0
                })
            }
        }
    }

    println!("{}", oxygen_numbers[0] * co2_numbers[0]);
}
