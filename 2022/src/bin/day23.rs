///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                      _             ___  ____
//                                     | |           |__ \|___ \
//                                   __| | __ _ _   _   ) | __) |
//                                  / _` |/ _` | | | | / / |__ <
//                                 | (_| | (_| | |_| |/ /_ ___) |
//                                  \__,_|\__,_|\__, |____|____/
//                                               __/ |
//                                              |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day23 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(input: &str) -> HashSet<(i64, i64)> {
    let mut elf_grid = HashSet::new();
    for (row, line) in input.trim().lines().enumerate() {
        for (column, point) in line.chars().enumerate() {
            match point {
                '#' => {
                    elf_grid.insert((row as i64, column as i64));
                }
                '.' => (),
                _ => panic!("Input contains invalid character"),
            }
        }
    }
    elf_grid
}

fn part1(elf_grid: &HashSet<(i64, i64)>) -> i64 {
    let mut elf_grid = elf_grid.clone();
    let initial_number_of_elves = elf_grid.len();

    let proposal_directions = vec![
        ((-1, 0), (-1, -1), (-1, 1)),
        ((1, 0), (1, -1), (1, 1)),
        ((0, -1), (-1, -1), (1, -1)),
        ((0, 1), (-1, 1), (1, 1)),
    ];

    for i in 0..10 {
        let mut proposals: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
        for (elf_row, elf_column) in elf_grid.iter() {
            if !elf_grid.contains(&(elf_row - 1, *elf_column))
                && !elf_grid.contains(&(elf_row - 1, elf_column - 1))
                && !elf_grid.contains(&(elf_row - 1, elf_column + 1))
                && !elf_grid.contains(&(*elf_row, elf_column + 1))
                && !elf_grid.contains(&(*elf_row, elf_column - 1))
                && !elf_grid.contains(&(elf_row + 1, *elf_column))
                && !elf_grid.contains(&(elf_row + 1, elf_column - 1))
                && !elf_grid.contains(&(elf_row + 1, elf_column + 1))
            {
                proposals.insert((*elf_row, *elf_column), vec![(*elf_row, *elf_column)]);
                continue;
            }

            let mut found_direction = false;
            for j in 0..4 {
                if !elf_grid.contains(&(
                    elf_row + proposal_directions[(i + j) % 4].0 .0,
                    elf_column + proposal_directions[(i + j) % 4].0 .1,
                )) && !elf_grid.contains(&(
                    elf_row + proposal_directions[(i + j) % 4].1 .0,
                    elf_column + proposal_directions[(i + j) % 4].1 .1,
                )) && !elf_grid.contains(&(
                    elf_row + proposal_directions[(i + j) % 4].2 .0,
                    elf_column + proposal_directions[(i + j) % 4].2 .1,
                )) {
                    proposals
                        .entry((
                            elf_row + proposal_directions[(i + j) % 4].0 .0,
                            elf_column + proposal_directions[(i + j) % 4].0 .1,
                        ))
                        .and_modify(|vec| vec.push((*elf_row, *elf_column)))
                        .or_insert(vec![(*elf_row, *elf_column)]);
                    found_direction = true;
                    break;
                }
            }

            if !found_direction {
                proposals.insert((*elf_row, *elf_column), vec![(*elf_row, *elf_column)]);
            }
        }

        elf_grid = HashSet::new();
        for (proposed_location, proposing_elves) in proposals.iter() {
            if proposing_elves.len() > 1 {
                for location in proposing_elves {
                    elf_grid.insert(*location);
                }
            } else if proposing_elves.len() == 1 {
                elf_grid.insert(*proposed_location);
            } else {
                panic!("Not supposed to happen");
            }
        }
    }

    let mut max_row = 0;
    let mut min_row = 0;
    let mut max_column = 0;
    let mut min_column = 0;
    let mut number_of_elves = 0;
    for (elf_row, elf_column) in elf_grid.iter() {
        max_row = max_row.max(*elf_row);
        min_row = min_row.min(*elf_row);
        max_column = max_column.max(*elf_column);
        min_column = min_column.min(*elf_column);
        number_of_elves += 1;
    }

    assert_eq!(initial_number_of_elves as i64, number_of_elves);
    (max_row + 1 - min_row) * (max_column + 1 - min_column) - number_of_elves
}

fn part2(elf_grid: &HashSet<(i64, i64)>) -> i64 {
    let mut elf_grid = elf_grid.clone();

    let proposal_directions = vec![
        ((-1, 0), (-1, -1), (-1, 1)),
        ((1, 0), (1, -1), (1, 1)),
        ((0, -1), (-1, -1), (1, -1)),
        ((0, 1), (-1, 1), (1, 1)),
    ];

    let mut i = 0;
    loop {
        let mut proposals: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
        let mut elves_not_moved = 0;
        for (elf_row, elf_column) in elf_grid.iter() {
            if !elf_grid.contains(&(elf_row - 1, *elf_column))
                && !elf_grid.contains(&(elf_row - 1, elf_column - 1))
                && !elf_grid.contains(&(elf_row - 1, elf_column + 1))
                && !elf_grid.contains(&(*elf_row, elf_column + 1))
                && !elf_grid.contains(&(*elf_row, elf_column - 1))
                && !elf_grid.contains(&(elf_row + 1, *elf_column))
                && !elf_grid.contains(&(elf_row + 1, elf_column - 1))
                && !elf_grid.contains(&(elf_row + 1, elf_column + 1))
            {
                elves_not_moved += 1;
                proposals.insert((*elf_row, *elf_column), vec![(*elf_row, *elf_column)]);
                continue;
            }

            let mut found_direction = false;
            for j in 0..4 {
                if !elf_grid.contains(&(
                    elf_row + proposal_directions[(i + j) % 4].0 .0,
                    elf_column + proposal_directions[(i + j) % 4].0 .1,
                )) && !elf_grid.contains(&(
                    elf_row + proposal_directions[(i + j) % 4].1 .0,
                    elf_column + proposal_directions[(i + j) % 4].1 .1,
                )) && !elf_grid.contains(&(
                    elf_row + proposal_directions[(i + j) % 4].2 .0,
                    elf_column + proposal_directions[(i + j) % 4].2 .1,
                )) {
                    proposals
                        .entry((
                            elf_row + proposal_directions[(i + j) % 4].0 .0,
                            elf_column + proposal_directions[(i + j) % 4].0 .1,
                        ))
                        .and_modify(|vec| vec.push((*elf_row, *elf_column)))
                        .or_insert(vec![(*elf_row, *elf_column)]);
                    found_direction = true;
                    break;
                }
            }

            if !found_direction {
                proposals.insert((*elf_row, *elf_column), vec![(*elf_row, *elf_column)]);
            }
        }

        if elves_not_moved == elf_grid.len() {
            return (i as i64) + 1;
        }

        elf_grid = HashSet::new();
        for (proposed_location, proposing_elves) in proposals.iter() {
            if proposing_elves.len() > 1 {
                for location in proposing_elves {
                    elf_grid.insert(*location);
                }
            } else if proposing_elves.len() == 1 {
                elf_grid.insert(*proposed_location);
            } else {
                panic!("Not supposed to happen");
            }
        }
        i += 1;
    }
}

fn main() {
    let input = advent::read_file("data/day23_input.txt");
    let elf_grid = parse_input(&input);
    println!("Part1 : {}", part1(&elf_grid));
    println!("Part2 : {}", part2(&elf_grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_tiny() {
        let input = "##\n\
                     #.\n\
                     ..\n\
                     ##";
        let elf_grid = parse_input(&input);
        assert_eq!(25, part1(&elf_grid));
    }

    #[test]
    fn part1_test() {
        let input = "....#..\n\
                     ..###.#\n\
                     #...#.#\n\
                     .#...##\n\
                     #.###..\n\
                     ##.#.##\n\
                     .#..#..\n";
        let elf_grid = parse_input(&input);
        assert_eq!(110, part1(&elf_grid));
    }
    #[test]
    fn part2_test() {
        let input = "....#..\n\
                     ..###.#\n\
                     #...#.#\n\
                     .#...##\n\
                     #.###..\n\
                     ##.#.##\n\
                     .#..#..\n";
        let elf_grid = parse_input(&input);
        assert_eq!(20, part2(&elf_grid));
    }
}
