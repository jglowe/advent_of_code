///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _               __
//                              | |             / /
//                            __| | __ _ _   _ / /_
//                           / _` |/ _` | | | | '_ \
//                          | (_| | (_| | |_| | (_) |
//                           \__,_|\__,_|\__, |\___/
//                                        __/ |
//                                       |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day6 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn find_fish_count(initial_state: &Vec<usize>, days: i32) -> usize {
    let mut state : Vec<usize> = initial_state.to_vec();

    for _ in 0..days {
        let new_fish = state.remove(0);
        state.push(new_fish);
        state[6] += new_fish;
    }

    state.iter().sum()
}

fn main() {
    let filename = "data/day6_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let initial_numbers: Vec<usize> = file_contents
        .split_once("\n")
        .unwrap()
        .0
        .split(",")
        .map(|string| string.parse().unwrap())
        .collect();

    let mut initial_state = vec![0; 9];
    for num in initial_numbers {
        initial_state[num] += 1;
    }

    println!("{}", find_fish_count(&initial_state, 80));
    println!("{}", find_fish_count(&initial_state, 256));
}
