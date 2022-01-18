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

fn main() {
    let filename = "input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {}", e),
    };

    let mut depth : i32 = 0;
    let mut horizontal : i32 = 0;

    for line in file_contents.split("\n") {
        let split_line : Vec<&str> = line.split(" ").collect();
        if split_line.len() != 2 {
            continue
        }

        let command = split_line[0];

        let number : i32 = match split_line[1].parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        match command {
            "up" => depth -= number,
            "down" => depth += number,
            "forward" => horizontal += number,
            _ => println!("Unmatch command")
        }
    }

    println!("{}", horizontal * depth);
}
