///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                        _            __   __
//                                       | |          /_ | / /
//                                     __| | __ _ _   _| |/ /_
//                                    / _` |/ _` | | | | | '_ \
//                                   | (_| | (_| | |_| | | (_) |
//                                    \__,_|\__,_|\__, |_|\___/
//                                                 __/ |
//                                                |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day16 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn main() {
    let filename = "data/day16_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    println!("{}", file_contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "1163751742\n\
                     1381373672\n\
                     2136511328\n\
                     3694931569\n\
                     7463417111\n\
                     1319128137\n\
                     1359912421\n\
                     3125421639\n\
                     1293138521\n\
                     2311944581\n";


    }
}
