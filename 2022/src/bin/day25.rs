///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                      _             ___  _____
//                                     | |           |__ \| ____|
//                                   __| | __ _ _   _   ) | |__
//                                  / _` |/ _` | | | | / /|___ \
//                                 | (_| | (_| | |_| |/ /_ ___) |
//                                  \__,_|\__,_|\__, |____|____/
//                                               __/ |
//                                              |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day25 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

fn decode(line: &str) -> usize {
    let mut number = 0;
    for c in line.chars() {
        match c {
            '0' => number *= 5,
            '1' => {
                number *= 5;
                number += 1
            }
            '2' => {
                number *= 5;
                number += 2
            }
            '-' => {
                number *= 5;
                number -= 1
            }
            '=' => {
                number *= 5;
                number -= 2
            }
            _ => panic!("Invalid character"),
        }
    }
    number
}

fn encode(number: usize) -> String {
    let mut number_string = "".to_string();

    let mut has_carry = false;
    let mut mask = 5;

    while mask / 5 <= number {
        match (number % mask) / (mask / 5) {
            0 => {
                if has_carry {
                    number_string = "1".to_string() + &number_string;
                } else {
                    number_string = "0".to_string() + &number_string;
                }
                has_carry = false;
            }
            1 => {
                if has_carry {
                    number_string = "2".to_string() + &number_string;
                } else {
                    number_string = "1".to_string() + &number_string;
                }
                has_carry = false;
            }
            2 => {
                if has_carry {
                    number_string = "=".to_string() + &number_string;
                    has_carry = true;
                } else {
                    number_string = "2".to_string() + &number_string;
                    has_carry = false;
                }
            }
            3 => {
                if has_carry {
                    number_string = "-".to_string() + &number_string;
                    has_carry = true;
                } else {
                    number_string = "=".to_string() + &number_string;
                    has_carry = true;
                }
            }
            4 => {
                if has_carry {
                    number_string = "0".to_string() + &number_string;
                    has_carry = true;
                } else {
                    number_string = "-".to_string() + &number_string;
                    has_carry = true;
                }
            }
            _ => {
                panic!("Shouldn't happen")
            }
        }

        mask *= 5;
    }
    if has_carry {
        number_string = "1".to_string() + &number_string;
    }

    number_string
}

fn part1(input: &str) -> String {
    encode(input.trim().lines().map(|line| decode(line)).sum())
}

fn main() {
    let input = advent::read_file("data/day25_input.txt");
    println!("Part 1 : {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_test() {
        assert_eq!("1".to_string(), encode(1));
        assert_eq!("2".to_string(), encode(2));
        assert_eq!("1=".to_string(), encode(3));
        assert_eq!("1-".to_string(), encode(4));
        assert_eq!("10".to_string(), encode(5));
        assert_eq!("11".to_string(), encode(6));
        assert_eq!("12".to_string(), encode(7));
        assert_eq!("2=".to_string(), encode(8));
        assert_eq!("2-".to_string(), encode(9));
        assert_eq!("20".to_string(), encode(10));
    }

    #[test]
    fn part1_test() {
        let input = "1=-0-2\n\
                     12111\n\
                     2=0=\n\
                     21\n\
                     2=01\n\
                     111\n\
                     20012\n\
                     112\n\
                     1=-1=\n\
                     1-12\n\
                     12\n\
                     1=\n\
                     122\n";
        assert_eq!(
            4890 as usize,
            input.trim().lines().map(|line| decode(line)).sum()
        );
        assert_eq!("2=-1=0".to_string(), part1(&input));
    }
}
