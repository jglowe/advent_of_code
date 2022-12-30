///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                      _             ___  _  _
//                                     | |           |__ \| || |
//                                   __| | __ _ _   _   ) | || |_
//                                  / _` |/ _` | | | | / /|__   _|
//                                 | (_| | (_| | |_| |/ /_   | |
//                                  \__,_|\__,_|\__, |____|  |_|
//                                               __/ |
//                                              |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day24 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Point {
    Wall,
    Open,
    Blizzard(Vec<Direction>),
}

type Map = Vec<Vec<Point>>;

fn parse_input(input: &str) -> Map {
    let mut map = Vec::new();

    for line in input.trim().lines() {
        let mut row = Vec::new();
        for point in line.chars() {
            row.push(match point {
                '^' => Point::Blizzard(vec![Direction::North]),
                'v' => Point::Blizzard(vec![Direction::South]),
                '<' => Point::Blizzard(vec![Direction::West]),
                '>' => Point::Blizzard(vec![Direction::East]),
                '.' => Point::Open,
                '#' => Point::Wall,
                _ => panic!("Invalid character"),
            });
        }
        map.push(row);
    }

    map
}

fn walk_to_destination(map: &Map, start: (usize, usize), end: (usize, usize)) -> (usize, Map) {
    let mut time_elapsed = 0;
    let mut map = map.clone();

    let map_rows = map.len();
    let map_columns = map[0].len();

    let mut possible_positions = HashSet::new();
    possible_positions.insert(start);

    loop {
        let mut next_possible_positions = HashSet::new();

        //
        // Compute next state of blizards
        //

        // Construct a new empty map
        let mut next_map = Vec::new();

        for row in &map {
            let mut next_row = Vec::new();
            for column in row {
                match column {
                    Point::Wall => next_row.push(Point::Wall),
                    _ => next_row.push(Point::Open),
                }
            }
            next_map.push(next_row);
        }

        // Move the blizards in the map
        for (i, row) in map.iter().enumerate() {
            for (j, column) in row.iter().enumerate() {
                match column {
                    Point::Wall => (),
                    Point::Open => (),
                    Point::Blizzard(blizzards) => {
                        for direction in blizzards {
                            // Get next direction, wraping as needed
                            let (row, column) = match direction {
                                Direction::North => {
                                    (if i - 1 == 0 { map_rows - 2 } else { i - 1 }, j)
                                }
                                Direction::South => {
                                    (if i + 1 == map_rows - 1 { 1 } else { i + 1 }, j)
                                }
                                Direction::East => {
                                    (i, if j + 1 == map_columns - 1 { 1 } else { j + 1 })
                                }
                                Direction::West => {
                                    (i, if j - 1 == 0 { map_columns - 2 } else { j - 1 })
                                }
                            };
                            // Placing the blizzards in their new place
                            match &mut next_map[row][column] {
                                Point::Open => {
                                    next_map[row][column] =
                                        Point::Blizzard(vec![direction.clone()]);
                                }
                                Point::Blizzard(b) => {
                                    b.push(direction.clone());
                                }
                                Point::Wall => {
                                    panic!("Shouldn't happen. Already should have wrapped around");
                                }
                            };
                        }
                    }
                }
            }
        }

        //
        // Computing where you can go
        //

        for (row, column) in possible_positions {
            if row == end.0 && column == end.1 {
                return (time_elapsed, map);
            }

            if next_map[row][column] == Point::Open {
                next_possible_positions.insert((row, column));
            }
            if row != next_map.len() - 1 && next_map[row + 1][column] == Point::Open {
                next_possible_positions.insert((row + 1, column));
            }
            if row != 0 && next_map[row - 1][column] == Point::Open {
                next_possible_positions.insert((row - 1, column));
            }
            if next_map[row][column + 1] == Point::Open {
                next_possible_positions.insert((row, column + 1));
            }
            if next_map[row][column - 1] == Point::Open {
                next_possible_positions.insert((row, column - 1));
            }
        }

        time_elapsed += 1;
        possible_positions = next_possible_positions;
        map = next_map;
    }
}

fn part1(map: &Map) -> usize {
    let (time, _) = walk_to_destination(map, (0, 1), (map.len() - 1, map[0].len() - 2));
    time
}

fn part2(map: &Map) -> usize {
    let (time1, map) = walk_to_destination(map, (0, 1), (map.len() - 1, map[0].len() - 2));
    let (time2, map) = walk_to_destination(&map, (map.len() - 1, map[0].len() - 2), (0, 1));
    let (time3, _) = walk_to_destination(&map, (0, 1), (map.len() - 1, map[0].len() - 2));
    time1 + time2 + time3
}

fn main() {
    let input = advent::read_file("data/day24_input.txt");
    let map = parse_input(&input);
    println!("Part 1 : {}", part1(&map));
    println!("Part 2 : {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "#.######\n\
                     #>>.<^<#\n\
                     #.<..<<#\n\
                     #>v.><>#\n\
                     #<^v^^>#\n\
                     ######.#\n";
        let map = parse_input(&input);
        assert_eq!(18, part1(&map));
    }
    #[test]
    fn part2_test() {
        let input = "#.######\n\
                     #>>.<^<#\n\
                     #.<..<<#\n\
                     #>v.><>#\n\
                     #<^v^^>#\n\
                     ######.#\n";
        let map = parse_input(&input);
        assert_eq!(54, part2(&map));
    }
}
