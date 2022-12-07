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
// The file for day6 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

fn find_n_different_chars(input: &str, n: usize) -> Result<usize, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut char_map: HashMap<char, usize> = HashMap::new();

    if chars.len() < n {
        return Err(format!("String must be at least {} chars long", n));
    }

    for offset in 0..n {
        char_map
            .entry(chars[offset])
            .and_modify(|o| *o += 1)
            .or_insert(1);
    }

    for offset in n..chars.len() {
        if char_map
            .values()
            .map(|value| *value == 1 || *value == 0)
            .reduce(|a, b| a && b)
            .unwrap()
        {
            return Ok(offset);
        }
        char_map
            .entry(chars[offset - n])
            .and_modify(|o| *o -= 1)
            .or_insert(0);
        char_map
            .entry(chars[offset])
            .and_modify(|o| *o += 1)
            .or_insert(1);
    }

    if char_map
        .values()
        .map(|value| *value == 1 || *value == 0)
        .reduce(|a, b| a && b)
        .unwrap()
    {
        return Ok(chars.len());
    }

    Err("String starting position not found".to_string())
}

fn find_packet_start(input: &str) -> Result<usize, String> {
    find_n_different_chars(input, 4)
}

fn find_message_start(input: &str) -> Result<usize, String> {
    find_n_different_chars(input, 14)
}

fn main() {
    let input = aoc::read_file("data/day6_input.txt");

    println!("Packet Start {}", find_packet_start(&input).unwrap());
    println!("Message Start {}", find_message_start(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        assert_eq!(
            7,
            find_packet_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap()
        );
        assert_eq!(
            5,
            find_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap()
        );
        assert_eq!(
            6,
            find_packet_start("nppdvjthqldpwncqszvftbrmjlhg").unwrap()
        );
        assert_eq!(
            10,
            find_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap()
        );
        assert_eq!(
            11,
            find_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap()
        );

        assert_eq!(
            19,
            find_message_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap()
        );
        assert_eq!(
            23,
            find_message_start("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap()
        );
        assert_eq!(
            23,
            find_message_start("nppdvjthqldpwncqszvftbrmjlhg").unwrap()
        );
        assert_eq!(
            29,
            find_message_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap()
        );
        assert_eq!(
            26,
            find_message_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap()
        );
    }
}
