///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                  _            __
//                                 | |          /_ |
//                               __| | __ _ _   _| |
//                              / _` |/ _` | | | | |
//                             | (_| | (_| | |_| | |
//                              \__,_|\__,_|\__, |_|
//                                           __/ |
//                                          |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day1 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn parse_input(input: String) -> Vec<i64> {
    let mut result = Vec::new();
    let mut calorie_count = 0;
    for line in input.trim().split("\n") {
        if line == "" {
            result.push(calorie_count);
            calorie_count = 0;
        } else {
            let item_calories: i64 = match line.parse() {
                Ok(number) => number,
                Err(_) => continue,
            };
            calorie_count = calorie_count + item_calories;
        }
    }

    if calorie_count != 0 {
        result.push(calorie_count);
    }

    result
}

fn sort_calories(counts: &mut Vec<i64>) {
    counts.sort_by(|a, b| b.cmp(a));
}

fn top_n_sum(counts: &Vec<i64>, top_n: usize) -> i64 {
    let mut sum = 0;

    for i in 0..top_n {
        sum = sum + counts[i];
    }

    sum
}

fn main() {
    let filename = "data/day1_input.txt";

    let input = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {}", e),
    };

    let mut calorie_counts = parse_input(input);
    sort_calories(&mut calorie_counts);
    println!("Most calories {}", calorie_counts[0]);
    println!("Most top 3 sum calories {}", top_n_sum(&calorie_counts, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_calories() {
        let input = "1000\n\
                     2000\n\
                     3000\n\
                     \n\
                     4000\n\
                     \n\
                     5000\n\
                     6000\n\
                     \n\
                     7000\n\
                     8000\n\
                     9000\n\
                     \n\
                     10000";

        let mut calorie_counts = parse_input(input.to_string());

        sort_calories(&mut calorie_counts);

        assert_eq!(24000, calorie_counts[0]);
        assert_eq!(45000, top_n_sum(&calorie_counts, 3));
    }
}
