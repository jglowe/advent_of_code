///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _              ___
//                              | |            / _ \
//                            __| | __ _ _   _| (_) |
//                           / _` |/ _` | | | |> _ <
//                          | (_| | (_| | |_| | (_) |
//                           \__,_|\__,_|\__, |\___/
//                                        __/ |
//                                       |___/
//
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day8 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn contains_each_letter(letters: &Vec<char>, string: &str) -> bool {
    let mut contains = letters.len() > 0;

    for letter in letters {
        contains = contains && string.chars().collect::<Vec<char>>().contains(letter);
    }

    contains
}

fn contains_five(letters: &Vec<char>, string: &str) -> bool {
    let mut count = 0;

    for letter in string.chars() {
        if letters.contains(&letter) {
            count += 1
        }
    }

    count == 5
}

fn main() {
    let filename = "data/day8_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let mut total_digits = 0;
    let mut digit_sum = 0;

    for line in file_contents.split("\n") {
        if line.len() == 0 {
            continue;
        }

        let (input, output) = line.split_once(" | ").unwrap();

        let input_numbers: Vec<&str> = input.split(" ").collect();
        let output_numbers: Vec<&str> = output.split(" ").collect();

        if output_numbers.len() != 4 {
            panic!("Incorrect number of output numbers");
        }

        total_digits += output_numbers.iter().fold(0, |acc, display| {
            if display.len() == 2 || display.len() == 3 || display.len() == 4 || display.len() == 7
            {
                acc + 1
            } else {
                acc
            }
        });

        let one: Vec<char> = input_numbers
            .iter()
            .find(|value| value.len() == 2)
            .unwrap()
            .chars()
            .collect();
        let four: Vec<char> = input_numbers
            .iter()
            .find(|value| value.len() == 4)
            .unwrap()
            .chars()
            .collect();
        let seven: Vec<char> = input_numbers
            .iter()
            .find(|value| value.len() == 3)
            .unwrap()
            .chars()
            .collect();
        let eight: Vec<char> = input_numbers
            .iter()
            .find(|value| value.len() == 7)
            .unwrap()
            .chars()
            .collect();

        let two_or_three_or_five: Vec<&str> = input_numbers
            .iter()
            .filter(|value| value.len() == 5)
            .map(|&x| x)
            .collect();
        let zero_or_six_or_nine: Vec<&str> = input_numbers
            .iter()
            .filter(|value| value.len() == 6)
            .map(|&x| x)
            .collect();

        let six: Vec<char> = zero_or_six_or_nine
            .iter()
            .find(|digit| {
                digit.contains(one[0]) && !digit.contains(one[1])
                    || (digit.contains(one[1]) && !digit.contains(one[0]))
            })
            .unwrap()
            .chars()
            .collect();

        let three: Vec<char> = two_or_three_or_five
            .iter()
            .find(|digit| digit.contains(one[0]) && digit.contains(one[1]))
            .unwrap()
            .chars()
            .collect();

        let zero: Vec<char> = zero_or_six_or_nine
            .iter()
            .find(|digit| {
                !contains_each_letter(&three, digit)
                    && (digit.chars().collect::<Vec<char>>() != six)
            })
            .unwrap()
            .chars()
            .collect();

        let nine: Vec<char> = zero_or_six_or_nine
            .iter()
            .find(|digit| {
                (digit.chars().collect::<Vec<char>>() != zero)
                    && (digit.chars().collect::<Vec<char>>() != six)
            })
            .unwrap()
            .chars()
            .collect();

        let five: Vec<char> = two_or_three_or_five
            .iter()
            .find(|digit| contains_five(&six, digit))
            .unwrap()
            .chars()
            .collect();

        let two: Vec<char> = two_or_three_or_five
            .iter()
            .find(|digit| {
                digit.chars().collect::<Vec<char>>() != three
                    && digit.chars().collect::<Vec<char>>() != five
            })
            .unwrap()
            .chars()
            .collect();

        let numbers = vec![zero, one, two, three, four, five, six, seven, eight, nine];

        let mut output_digits = 0;
        for digit in output_numbers {
            for (i, number) in numbers.iter().enumerate() {
                if digit.len() == number.len() && contains_each_letter(&number, digit) {
                    output_digits = output_digits * 10;
                    output_digits += i;
                }
            }
        }
        digit_sum += output_digits;
    }

    println!("{}", total_digits);
    println!("{}", digit_sum);
}
