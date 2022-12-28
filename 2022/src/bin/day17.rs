///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                       _            __ ______
//                                      | |          /_ |____  |
//                                    __| | __ _ _   _| |   / /
//                                   / _` |/ _` | | | | |  / /
//                                  | (_| | (_| | |_| | | / /
//                                   \__,_|\__,_|\__, |_|/_/
//                                                __/ |
//                                               |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day17 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum RockType {
    Dash,
    Plus,
    RevL,
    Pipe,
    Cube,
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    chamber: Vec<Vec<bool>>,
    jet_pattern_index: usize,
    rock_type: RockType,
}

fn can_be_moved_by_jets(chamber: &Vec<Vec<bool>>, rock: &Vec<(usize, usize)>, right: bool) -> bool {
    rock.iter()
        .map(|(row, column)| {
            if right {
                *column < 6 && !chamber[*column + 1][*row]
            } else {
                *column > 0 && !chamber[*column - 1][*row]
            }
        })
        .reduce(|a, b| a && b)
        .unwrap()
}

fn run_simulation(input: &str, rocks: usize) -> usize {
    let jet_pattern: Vec<char> = input.trim().chars().collect();
    let mut jet_pattern_index = 0;
    let mut chamber: Vec<Vec<bool>> = Vec::new();
    for _ in 0..7 {
        let mut column = Vec::new();
        for _ in 0..10000 {
            column.push(false);
        }
        chamber.push(column);
    }
    let mut top: Option<usize> = None;

    for i in 0..rocks {
        let rock_type = match i % 5 {
            0 => RockType::Dash,
            1 => RockType::Plus,
            2 => RockType::RevL,
            3 => RockType::Pipe,
            4 => RockType::Cube,
            _ => panic!("Module not working on {} % 5", i),
        };

        let rock_bottom = match top {
            None => 3,
            Some(x) => x + 4,
        };

        let mut rock: Vec<(usize, usize)> = match rock_type {
            RockType::Dash => vec![
                (rock_bottom, 2),
                (rock_bottom, 3),
                (rock_bottom, 4),
                (rock_bottom, 5),
            ],
            RockType::Plus => vec![
                (rock_bottom + 1, 2),
                (rock_bottom + 1, 3),
                (rock_bottom + 1, 4),
                (rock_bottom, 3),
                (rock_bottom + 2, 3),
            ],
            RockType::RevL => vec![
                (rock_bottom, 2),
                (rock_bottom, 3),
                (rock_bottom, 4),
                (rock_bottom + 1, 4),
                (rock_bottom + 2, 4),
            ],
            RockType::Pipe => vec![
                (rock_bottom, 2),
                (rock_bottom + 1, 2),
                (rock_bottom + 2, 2),
                (rock_bottom + 3, 2),
            ],
            RockType::Cube => vec![
                (rock_bottom, 2),
                (rock_bottom, 3),
                (rock_bottom + 1, 2),
                (rock_bottom + 1, 3),
            ],
        };

        let mut moving = true;

        while moving {
            match jet_pattern[jet_pattern_index] {
                '>' => {
                    if can_be_moved_by_jets(&chamber, &rock, true) {
                        for rock in &mut rock {
                            rock.1 += 1;
                        }
                    }
                }
                '<' => {
                    if can_be_moved_by_jets(&chamber, &rock, false) {
                        for rock in &mut rock {
                            rock.1 -= 1;
                        }
                    }
                }
                c => panic!("Invalid character in the input \"{}\"", c),
            }

            jet_pattern_index = (jet_pattern_index + 1) % jet_pattern.len();

            let can_fall = rock
                .iter()
                .map(|(row, column)| *row > 0 && !chamber[*column][*row - 1])
                .reduce(|a, b| a && b)
                .unwrap();

            if can_fall {
                for chunk in &mut rock {
                    chunk.0 -= 1;
                }
            } else {
                moving = false;
            }
        }

        for (row, column) in &rock {
            chamber[*column][*row] = true;
        }

        top = match top {
            Some(top) => Some(top.max(rock.iter().map(|r| r.0).max().unwrap())),
            None => Some(rock.iter().map(|r| r.0).max().unwrap()),
        }
    }

    top.unwrap() + 1
}

fn is_contained(chamber: &Vec<Vec<bool>>, row: usize, column: usize, max_row: usize) -> bool {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((row, column));

    while let Some((row, column)) = queue.pop_back() {
        if visited.contains(&(row, column)) {
            continue;
        }
        if row >= max_row {
            return false;
        }
        visited.insert((row, column));
        if !chamber[column][row] {
            if column > 0 {
                queue.push_back((row, column - 1));
            };
            if column < 6 {
                queue.push_back((row, column + 1));
            };
            if row > 0 {
                queue.push_back((row - 1, column));
            };
            queue.push_back((row + 1, column));
        }
    }

    true
}

fn can_reduce(
    chamber: &Vec<Vec<bool>>,
    top: &Vec<Option<usize>>,
    window_floor: usize,
    max_row: usize,
) -> (bool, usize) {
    let has_rock_bottom = top.iter().fold(true, |x, y| match (x, y) {
        (_, None) => false,
        (b, Some(_)) => b,
    });

    if !has_rock_bottom {
        return (false, 0);
    }

    let mut bottoms = Vec::new();
    for column in 0..7 {
        for i in 0..max_row {
            if !is_contained(chamber, i + 1, column, max_row) {
                bottoms.push(i);
                break;
            }
        }
    }
    match bottoms.iter().min() {
        Some(x) => {
            if *x > 0 {
                (true, x + window_floor)
            } else {
                (false, 0)
            }
        }
        None => (false, 0),
    }
}

fn part2(input: &str, rocks: usize) -> usize {
    let jet_pattern: Vec<char> = input.trim().chars().collect();
    let mut jet_pattern_index = 0;
    let mut chamber: Vec<Vec<bool>> = Vec::new();
    let mut states: HashMap<State, (usize, usize)> = HashMap::new();
    let mut top: Vec<Option<usize>> = Vec::new();
    let max_row = 1000;
    for _ in 0..7 {
        let mut column = Vec::new();
        for _ in 0..max_row {
            column.push(false);
        }
        chamber.push(column);
        top.push(None);
    }
    let mut window_floor = 0;

    let mut cycle_found = false;

    let mut i = 0;
    while i < rocks {
        let rock_type = match i % 5 {
            0 => RockType::Dash,
            1 => RockType::Plus,
            2 => RockType::RevL,
            3 => RockType::Pipe,
            4 => RockType::Cube,
            _ => panic!("Module not working on {} % 5", i),
        };

        let rock_bottom = match top.iter().fold(None, |x, y| match (x, y) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(x)) => Some(x),
            (Some(x), Some(y)) => Some(x.max(y)),
        }) {
            None => 3 - window_floor,
            Some(x) => x + 4 - window_floor,
        };

        let mut rock: Vec<(usize, usize)> = match rock_type {
            RockType::Dash => vec![
                (rock_bottom, 2),
                (rock_bottom, 3),
                (rock_bottom, 4),
                (rock_bottom, 5),
            ],
            RockType::Plus => vec![
                (rock_bottom + 1, 2),
                (rock_bottom + 1, 3),
                (rock_bottom + 1, 4),
                (rock_bottom, 3),
                (rock_bottom + 2, 3),
            ],
            RockType::RevL => vec![
                (rock_bottom, 2),
                (rock_bottom, 3),
                (rock_bottom, 4),
                (rock_bottom + 1, 4),
                (rock_bottom + 2, 4),
            ],
            RockType::Pipe => vec![
                (rock_bottom, 2),
                (rock_bottom + 1, 2),
                (rock_bottom + 2, 2),
                (rock_bottom + 3, 2),
            ],
            RockType::Cube => vec![
                (rock_bottom, 2),
                (rock_bottom, 3),
                (rock_bottom + 1, 2),
                (rock_bottom + 1, 3),
            ],
        };

        let mut moving = true;

        while moving {
            match jet_pattern[jet_pattern_index] {
                '>' => {
                    if can_be_moved_by_jets(&chamber, &rock, true) {
                        for rock in &mut rock {
                            rock.1 += 1;
                        }
                    }
                }
                '<' => {
                    if can_be_moved_by_jets(&chamber, &rock, false) {
                        for rock in &mut rock {
                            rock.1 -= 1;
                        }
                    }
                }
                c => panic!("Invalid character in the input \"{}\"", c),
            }

            jet_pattern_index = (jet_pattern_index + 1) % jet_pattern.len();

            let can_fall = rock
                .iter()
                .map(|(row, column)| *row > 0 && !chamber[*column][*row - 1])
                .reduce(|a, b| a && b)
                .unwrap();

            if can_fall {
                for chunk in &mut rock {
                    chunk.0 -= 1;
                }
            } else {
                moving = false;
            }
        }

        for (row, column) in &rock {
            chamber[*column][*row] = true;
        }

        for (row, column) in &rock {
            top[*column] = match top[*column] {
                Some(x) => Some((*row + window_floor).max(x)),
                None => Some(*row + window_floor),
            }
        }

        let has_rock_bottom = top.iter().fold(true, |x, y| match (x, y) {
            (_, None) => false,
            (b, Some(_)) => b,
        });

        let (reduce, min_index) = can_reduce(&chamber, &top, window_floor, max_row);

        if min_index > window_floor && has_rock_bottom && reduce {
            let old_chamber = chamber;
            chamber = Vec::new();
            for col in 0..7 {
                let mut column = Vec::new();
                for row in 0..max_row {
                    if row + min_index - window_floor < old_chamber[0].len() {
                        column.push(old_chamber[col][row + min_index - window_floor]);
                    } else {
                        column.push(false);
                    }
                }
                chamber.push(column);
            }
            window_floor = min_index;
        }

        if !cycle_found && has_rock_bottom {
            let state = State {
                chamber: chamber.clone(),
                jet_pattern_index,
                rock_type: rock_type.clone(),
            };

            if states.contains_key(&state) {
                let (prev_floor, rock_number) =
                    states.get(&state).unwrap();

                // The - 1 took hours and hours to debug.
                let jump = (window_floor - prev_floor) * ((rocks - i - 1) / (i - rock_number));
                i += (rocks - i - 1) / (i - rock_number) * (i - rock_number);

                window_floor += jump;
                for x in top.iter_mut() {
                    match x {
                        None => panic!("Shound't be none here"),
                        Some(x) => *x += jump,
                    }
                }
                cycle_found = true;
            } else {
                states.insert(state, (window_floor, i));
            }
        }
        i += 1;
    }

    *top.iter()
        .fold(None, |x, y| match (x, y) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(x)) => Some(x),
            (Some(x), Some(y)) => Some(x.max(y)),
        })
        .unwrap()
        + 1
}

fn main() {
    let input = advent::read_file("data/day17_input.txt");

    println!("2022 Height {}", part2(&input, 2022));
    println!("1000000000000 height {}", part2(&input, 1000000000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";

        assert_eq!(3068, run_simulation(&input, 2022));
        assert_eq!(3068, part2(&input, 2022));
    }

    #[test]
    fn part2_test() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";
        assert_eq!(1514285714288, part2(&input, 1000000000000));
    }
}
