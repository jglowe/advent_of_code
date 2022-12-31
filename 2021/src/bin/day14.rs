///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                       _            __ _  _
//                                      | |          /_ | || |
//                                    __| | __ _ _   _| | || |_
//                                   / _` |/ _` | | | | |__   _|
//                                  | (_| | (_| | |_| | |  | |
//                                   \__,_|\__,_|\__, |_|  |_|
//                                                __/ |
//                                               |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day14 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::fs;

fn parse_input(
    input: String,
) -> (
    HashMap<(char, char), i64>,
    HashMap<(char, char), char>,
    char,
    char,
) {
    let mut rules = HashMap::new();
    let mut initial_sequence = HashMap::new();
    let mut parsing_rules = false;
    let mut left = None;
    let mut right = None;

    for line in input.trim().split("\n") {
        if line == "" {
            parsing_rules = true
        } else if !parsing_rules {
            let chars: Vec<char> = line.chars().collect();
            left = Some(chars[0]);
            right = Some(chars[chars.len() - 1]);
            for i in 0..chars.len() {
                if i < chars.len() - 1 {
                    let count = initial_sequence
                        .entry((chars[i], chars[i + 1]))
                        .or_insert(0);
                    *count += 1;
                }
            }
        } else {
            let (pair, to_insert) = line.split_once(" -> ").unwrap();

            let to_insert = to_insert.chars().collect::<Vec<char>>();
            let pair = pair.chars().collect::<Vec<char>>();
            if to_insert.len() != 1 || pair.len() != 2 {
                panic!("Unable to parse line:{}", line);
            }

            let to_insert = to_insert[0];

            rules.insert((pair[0], pair[1]), to_insert);
        }
    }

    (initial_sequence, rules, left.unwrap(), right.unwrap())
}

fn process_one_step(
    sequence: &HashMap<(char, char), i64>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), i64> {
    let mut new_sequence = HashMap::new();

    for ((left, right), existing) in sequence.iter() {
        // Add new pairs from replication rules
        if let Some(to_insert) = rules.get(&(*left, *right)) {
            let count = new_sequence.entry((*left, *to_insert)).or_insert(0);
            *count += existing;
            let count = new_sequence.entry((*to_insert, *right)).or_insert(0);
            *count += existing;
        } else {
            // Count existing pairs
            let count = new_sequence.entry((*left, *right)).or_insert(0);
            *count += existing;
        }
    }

    new_sequence
}

fn run_process(
    sequence: &HashMap<(char, char), i64>,
    rules: &HashMap<(char, char), char>,
    times: i32,
) -> HashMap<(char, char), i64> {
    let mut result = sequence.clone();

    for _ in 0..times {
        result = process_one_step(&result, rules)
    }

    result
}

fn get_most_minus_least(sequence: &HashMap<(char, char), i64>, left: &char, right: &char) -> i64 {
    let mut max = 0;
    let mut min = 0;
    let mut counts = HashMap::new();

    for ((left, right), count) in sequence.iter() {
        let number = counts.entry(*left).or_insert(0);
        *number += count;
        let number = counts.entry(*right).or_insert(0);
        *number += count;
    }

    for (i, (c, count)) in counts.iter().enumerate() {
        let dec = if *c == *left || *c == *right { 1 } else { 0 };
        if i == 0 {
            max = *count / 2 + dec;
            min = *count / 2 + dec;
        } else {
            if *count / 2 + dec > max {
                max = *count / 2 + dec;
            }
            if *count / 2 + dec < min {
                min = *count / 2 + dec;
            }
        }
    }

    max - min
}

fn main() {
    let filename = "data/day14_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let (initial_sequence, rules, left, right) = parse_input(file_contents.to_string());

    let part1_result = run_process(&initial_sequence, &rules, 10);
    println!(
        "Part 1 : {}",
        get_most_minus_least(&part1_result, &left, &right)
    );

    let part2_result = run_process(&initial_sequence, &rules, 40);
    println!(
        "Part 2 : {}",
        get_most_minus_least(&part2_result, &left, &right)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "NNCB\n\
                     \n\
                     CH -> B\n\
                     HH -> N\n\
                     CB -> H\n\
                     NH -> C\n\
                     HB -> C\n\
                     HC -> B\n\
                     HN -> C\n\
                     NN -> C\n\
                     BH -> H\n\
                     NC -> B\n\
                     NB -> B\n\
                     BN -> B\n\
                     BB -> N\n\
                     BC -> B\n\
                     CC -> N\n\
                     CN -> C\n";

        let (template_original, rules, left, right) = parse_input(input.to_string());

        let sequence = run_process(&template_original, &rules, 1);

        assert_eq!(1, get_most_minus_least(&sequence, &left, &right));

        let sequence = run_process(&template_original, &rules, 2);

        assert_eq!(5, get_most_minus_least(&sequence, &left, &right));

        let sequence = run_process(&template_original, &rules, 10);

        assert_eq!(1588, get_most_minus_least(&sequence, &left, &right));

        let sequence = run_process(&template_original, &rules, 40);

        assert_eq!(
            2188189693529,
            get_most_minus_least(&sequence, &left, &right)
        );
    }
}
