///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _            ______
//                              | |          |____  |
//                            __| | __ _ _   _   / /
//                           / _` |/ _` | | | | / /
//                          | (_| | (_| | |_| |/ /
//                           \__,_|\__,_|\__, /_/
//                                        __/ |
//                                       |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day7 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn calculate_min_fuel(positions: &Vec<i32>, calculate_fuel: fn(i32, i32) -> i32) -> i32 {
    let max = *positions.iter().max().unwrap();
    let min = *positions.iter().min().unwrap();

    let mut min_fuel = positions
        .iter()
        .fold(0, |acc, number| acc + calculate_fuel(*number, min));
    for i in min + 1..max + 1 {
        let fuel: i32 = positions
            .iter()
            .fold(0, |acc, number| acc + calculate_fuel(*number, i));
        min_fuel = fuel.min(min_fuel);
    }

    min_fuel
}

fn main() {
    let filename = "data/day7_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let initial_numbers: Vec<i32> = file_contents
        .trim()
        .split(",")
        .map(|string| string.parse().unwrap())
        .collect();

    println!(
        "{}",
        calculate_min_fuel(&initial_numbers, |x, y| (x - y).abs())
    );
    println!(
        "{}",
        calculate_min_fuel(&initial_numbers, |x, y| (x - y).abs() * ((x - y).abs() + 1)
            / 2)
    );
}
