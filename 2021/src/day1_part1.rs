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

fn main() {
    let filename = "data/day1_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {}", e),
    };

    let mut previous_number : Option<i32> = None;
    let mut increases : i32 = 0;

    for line in file_contents.split("\n") {
        let number : i32 = match line.parse() {
            Ok(number) => number,
            Err(_) => continue
        };
        if let Some(p) = previous_number {
            if number > p {
                increases += 1;
            }
        }
        previous_number = Some(number);
    }

    println!("{}", increases)
}
