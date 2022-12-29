///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                      _             ___   ___
//                                     | |           |__ \ / _ \
//                                   __| | __ _ _   _   ) | | | |
//                                  / _` |/ _` | | | | / /| | | |
//                                 | (_| | (_| | |_| |/ /_| |_| |
//                                  \__,_|\__,_|\__, |____|\___/
//                                               __/ |
//                                              |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day20 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<i64> {
    let list: Vec<i64> = input
        .trim()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    list
}

fn part1(list: &mut Vec<i64>) -> i64 {
    let mut numbers: HashMap<usize, (usize, i64)> = HashMap::new();

    for i in 0..list.len() {
        numbers.insert(i, (i, list[i]));
    }

    for i in 0..list.len() {
        let mut current_index = *numbers
            .iter()
            .find_map(|(key, &val)| if val.0 == i { Some(key) } else { None })
            .unwrap();
        let num = numbers.get(&current_index).unwrap().1;
        if num < 0 {
            for _ in 0..num.abs() {
                if current_index == 0 {
                    let less = *numbers.get(&(list.len() - 1)).unwrap();
                    let curr = *numbers.get(&0).unwrap();
                    numbers.insert(list.len() - 1, curr.clone());
                    numbers.insert(0, less.clone());

                    current_index = list.len() - 1;
                } else {
                    let less = *numbers.get(&(current_index - 1)).unwrap();
                    let curr = *numbers.get(&current_index).unwrap();
                    numbers.insert(current_index - 1, curr);
                    numbers.insert(current_index, less);
                    current_index -= 1;
                }
            }
        } else {
            for _ in 0..num {
                if current_index == list.len() - 1 {
                    let greater = *numbers.get(&0).unwrap();
                    let curr = *numbers.get(&(list.len() - 1)).unwrap();
                    numbers.insert(0, curr.clone());
                    numbers.insert(list.len() - 1, greater.clone());

                    current_index = 0;
                } else {
                    let greater = *numbers.get(&(current_index + 1)).unwrap();
                    let curr = *numbers.get(&current_index).unwrap();
                    numbers.insert(current_index + 1, curr);
                    numbers.insert(current_index, greater);
                    current_index += 1;
                }
            }
        }
    }

    let index_of_0 = numbers
        .iter()
        .find_map(|(key, &val)| if val.1 == 0 { Some(key) } else { None })
        .unwrap();

    let thousandth = numbers.get(&((index_of_0 + 1000) % list.len())).unwrap().1;
    let two_thousandth = numbers.get(&((index_of_0 + 2000) % list.len())).unwrap().1;
    let three_thousandth = numbers.get(&((index_of_0 + 3000) % list.len())).unwrap().1;

    thousandth + two_thousandth + three_thousandth
}

fn part2(list: &mut Vec<i64>) -> i64 {
    let mut numbers: HashMap<usize, (usize, i64)> = HashMap::new();

    for i in 0..list.len() {
        numbers.insert(i, (i, 811589153 * list[i]));
    }

    for _ in 0..10 {
        for i in 0..list.len() {
            let mut current_index = *numbers
                .iter()
                .find_map(|(key, &val)| if val.0 == i { Some(key) } else { None })
                .unwrap();
            let num = numbers.get(&current_index).unwrap().1;
            if num < 0 {
                for _ in 0..(num.abs() as usize % (list.len() - 1)) {
                    if current_index == 0 {
                        let less = *numbers.get(&(list.len() - 1)).unwrap();
                        let curr = *numbers.get(&0).unwrap();
                        numbers.insert(list.len() - 1, curr.clone());
                        numbers.insert(0, less.clone());

                        current_index = list.len() - 1;
                    } else {
                        let less = *numbers.get(&(current_index - 1)).unwrap();
                        let curr = *numbers.get(&current_index).unwrap();
                        numbers.insert(current_index - 1, curr);
                        numbers.insert(current_index, less);
                        current_index -= 1;
                    }
                }
            } else {
                for _ in 0..(num.abs() as usize % (list.len() - 1)) {
                    if current_index == list.len() - 1 {
                        let greater = *numbers.get(&0).unwrap();
                        let curr = *numbers.get(&(list.len() - 1)).unwrap();
                        numbers.insert(0, curr.clone());
                        numbers.insert(list.len() - 1, greater.clone());

                        current_index = 0;
                    } else {
                        let greater = *numbers.get(&(current_index + 1)).unwrap();
                        let curr = *numbers.get(&current_index).unwrap();
                        numbers.insert(current_index + 1, curr);
                        numbers.insert(current_index, greater);
                        current_index += 1;
                    }
                }
            }
        }
    }

    let index_of_0 = numbers
        .iter()
        .find_map(|(key, &val)| if val.1 == 0 { Some(key) } else { None })
        .unwrap();

    let thousandth = numbers.get(&((index_of_0 + 1000) % list.len())).unwrap().1;
    let two_thousandth = numbers.get(&((index_of_0 + 2000) % list.len())).unwrap().1;
    let three_thousandth = numbers.get(&((index_of_0 + 3000) % list.len())).unwrap().1;

    thousandth + two_thousandth + three_thousandth
}

fn main() {
    let input = advent::read_file("data/day20_input.txt");
    let mut list = parse_input(&input);
    println!("Part1 : {}", part1(&mut list));
    println!("Part2 : {}", part2(&mut list));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "1\n\
                     2\n\
                     -3\n\
                     3\n\
                     -2\n\
                     0\n\
                     4\n";

        let mut list = parse_input(&input);

        assert_eq!(3, part1(&mut list));
    }

    #[test]
    fn part2_test() {
        let input = "1\n\
                     2\n\
                     -3\n\
                     3\n\
                     -2\n\
                     0\n\
                     4\n";

        let mut list = parse_input(&input);

        assert_eq!(1623178306, part2(&mut list));
    }
}
