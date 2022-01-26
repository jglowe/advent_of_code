///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                              _            __  ___
//                             | |          /_ |/ _ \
//                           __| | __ _ _   _| | | | |
//                          / _` |/ _` | | | | | | | |
//                         | (_| | (_| | |_| | | |_| |
//                          \__,_|\__,_|\__, |_|\___/
//                                       __/ |
//                                      |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day10 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn char_matches(left: char, right: char) -> bool {
    (left == '(' && right == ')')
        || (left == '[' && right == ']')
        || (left == '{' && right == '}')
        || (left == '<' && right == '>')
}

fn corrupted_char_points(c: char) -> i64 {
    if c == '>' {
        25137
    } else if c == '}' {
        1197
    } else if c == ']' {
        57
    } else if c == ')' {
        3
    } else {
        panic!("Invalid char: {}", c);
    }
}

fn completed_char_points(c: char) -> i64 {
    if c == '<' {
        4
    } else if c == '{' {
        3
    } else if c == '[' {
        2
    } else if c == '(' {
        1
    } else {
        panic!("Invalid char: {}", c);
    }
}

fn main() {
    let filename = "data/day10_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let mut corrupted_points = 0;
    let mut completed_points = Vec::new();

    for line in file_contents.trim().split("\n") {
        let mut brackets: Vec<char> = Vec::new();
        let mut is_corrupted = false;
        for c in line.chars() {
            if ['<', '(', '[', '{'].contains(&c) {
                brackets.push(c);
            } else {
                let points = match brackets.pop() {
                    Some(left) => {
                        if char_matches(left, c) {
                            None
                        } else {
                            Some(corrupted_char_points(c))
                        }
                    }
                    None => Some(corrupted_char_points(c)),
                };

                if let Some(point) = points {
                    corrupted_points += point;
                    is_corrupted = true;
                    break;
                }
            }
        }
        if !is_corrupted {
            completed_points.push(
                brackets
                    .iter()
                    .rev()
                    .fold(0, |acc, c| (acc * 5) + completed_char_points(*c)),
            );
        }
    }

    completed_points.sort();

    println!("{}", corrupted_points);
    println!("{}", completed_points[completed_points.len() / 2]);
}
