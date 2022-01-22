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

    let mut previous_sum : Option<i32> = None;
    let mut increases : i32 = 0;
    let mut window : Vec<i32> = Vec::new();

    for line in file_contents.split("\n") {
        let number : i32 = match line.parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        window.push(number);

        if window.len() < 3 {
            continue
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

    println!("{}", increases)
}
