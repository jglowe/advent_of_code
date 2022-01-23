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
    let mut zero_bits: Vec<i32> = Vec::new();
    let mut one_bits: Vec<i32> = Vec::new();

    for (i, line) in file_contents.split("\n").enumerate() {
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

    println!("{}", gamma_rate * epilon_rate)
}
