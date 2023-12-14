///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                           _            __
//                                          | |          /_ |
//                                        __| | __ _ _   _| |
//                                       / _` |/ _` | | | | |
//                                      | (_| | (_| | |_| | |
//                                       \__,_|\__,_|\__, |_|
//                                                    __/ |
//                                                   |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day1 advent of code 2023
///////////////////////////////////////////////////////////////////////////////////////////////////

fn first_and_last_digits(line: &str) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for item in line.chars() {
        if item.is_digit(10) {
            match (first_digit, last_digit) {
                (None, None) => first_digit = Some(item.to_digit(10).unwrap()),
                (Some(_), None) => last_digit = Some(item.to_digit(10).unwrap()),
                (Some(_), Some(_)) => last_digit = Some(item.to_digit(10).unwrap()),
                (None, Some(_)) => panic!("First digit must be populated first"),
            }
        }
    }

    match (first_digit, last_digit) {
        (Some(x), Some(y)) => x * 10 + y,
        (Some(x), None) => x * 10 + x,
        _ => panic!("Something went wrong, please consult this error in the code"),
    }
}

fn first_and_last_digits_also_spelled_out(line: &str) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for (index, item) in line.chars().enumerate() {
        if item.is_digit(10) {
            match (first_digit, last_digit) {
                (None, None) => first_digit = Some(item.to_digit(10).unwrap()),
                (Some(_), None) => last_digit = Some(item.to_digit(10).unwrap()),
                (Some(_), Some(_)) => last_digit = Some(item.to_digit(10).unwrap()),
                (None, Some(_)) => panic!("First digit must be populated first"),
            }
        } else {
            let mut detected_digit: Option<u32> = None;
            if line.len() >= index + 3 && "one".eq(&line[index..index + 3]) {
                detected_digit = Some(1);
            }
            if line.len() >= index + 3 && "two".eq(&line[index..index + 3]) {
                detected_digit = Some(2);
            }
            if line.len() >= index + 5 && "three".eq(&line[index..index + 5]) {
                detected_digit = Some(3);
            }
            if line.len() >= index + 4 && "four".eq(&line[index..index + 4]) {
                detected_digit = Some(4);
            }
            if line.len() >= index + 4 && "five".eq(&line[index..index + 4]) {
                detected_digit = Some(5);
            }
            if line.len() >= index + 3 && "six".eq(&line[index..index + 3]) {
                detected_digit = Some(6);
            }
            if line.len() >= index + 5 && "seven".eq(&line[index..index + 5]) {
                detected_digit = Some(7);
            }
            if line.len() >= index + 5 && "eight".eq(&line[index..index + 5]) {
                detected_digit = Some(8);
            }
            if line.len() >= index + 4 && "nine".eq(&line[index..index + 4]) {
                detected_digit = Some(9);
            }
            match detected_digit {
                Some(detected_digit) => {
                    match (first_digit, last_digit) {
                        (None, None) => first_digit = Some(detected_digit),
                        (Some(_), None) => last_digit = Some(detected_digit),
                        (Some(_), Some(_)) => last_digit = Some(detected_digit),
                        (None, Some(_)) => panic!("First digit must be populated first"),
                    };
                }
                None => {}
            }
        }
    }

    match (first_digit, last_digit) {
        (Some(x), Some(y)) => x * 10 + y,
        (Some(x), None) => x * 10 + x,
        _ => panic!("Something went wrong, please consult this error in the code"),
    }
}

fn part1(input: &str) -> u32 {
    input.trim().lines().map(first_and_last_digits).sum()
}

fn part2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(first_and_last_digits_also_spelled_out)
        .sum()
}

fn main() {
    let input = advent::read_file("data/day1_input.txt");
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(281, part2(&input));
    }
}
