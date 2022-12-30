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

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[derive(Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    map: Map,
    row: usize,
    column: usize,
}

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

fn print_map(map: &Map, troup_row: usize, troup_column: usize) {
    for (i, row) in map.iter().enumerate() {
        let mut to_print = "".to_string();
        for (j, column) in row.iter().enumerate() {
            if i == troup_row && j == troup_column {
                if *column == Point::Open {
                    to_print += "B"
                } else {
                    panic!("Shouldn't Happen")
                }
            } else {
                match column {
                    Point::Wall => to_print += "#",
                    Point::Open => to_print += ".",
                    Point::Blizzard(b) => {
                        if b.len() > 1 {
                            to_print += &b.len().to_string()
                        } else {
                            match b[0] {
                                Direction::North => to_print += "^",
                                Direction::South => to_print += "v",
                                Direction::East => to_print += ">",
                                Direction::West => to_print += "<",
                            }
                        }
                    }
                }
            }
        }
        println!("{}", to_print)
    }
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

        for (i, row) in map.iter().enumerate() {
            for (j, column) in row.iter().enumerate() {
                match column {
                    Point::Wall => (),
                    Point::Open => (),
                    Point::Blizzard(blizzards) => {
                        for direction in blizzards {
                            match direction {
                                Direction::North => match &mut next_map[i - 1][j] {
                                    Point::Open => {
                                        next_map[i - 1][j] =
                                            Point::Blizzard(vec![Direction::North]);
                                    }
                                    Point::Blizzard(b) => {
                                        b.push(Direction::North);
                                    }
                                    Point::Wall => match &mut next_map[map_rows - 2][j] {
                                        Point::Open => {
                                            next_map[map_rows - 2][j] =
                                                Point::Blizzard(vec![Direction::North]);
                                        }
                                        Point::Blizzard(b) => {
                                            b.push(Direction::North);
                                        }
                                        Point::Wall => panic!("Shouldn't happen"),
                                    },
                                },
                                Direction::South => match &mut next_map[i + 1][j] {
                                    Point::Open => {
                                        next_map[i + 1][j] =
                                            Point::Blizzard(vec![Direction::South]);
                                    }
                                    Point::Blizzard(b) => {
                                        b.push(Direction::South);
                                    }
                                    Point::Wall => match &mut next_map[1][j] {
                                        Point::Open => {
                                            next_map[1][j] =
                                                Point::Blizzard(vec![Direction::South]);
                                        }
                                        Point::Blizzard(b) => {
                                            b.push(Direction::South);
                                        }
                                        Point::Wall => panic!("Shouldn't happen"),
                                    },
                                },
                                Direction::East => match &mut next_map[i][j + 1] {
                                    Point::Open => {
                                        next_map[i][j + 1] = Point::Blizzard(vec![Direction::East]);
                                    }
                                    Point::Blizzard(b) => {
                                        b.push(Direction::East);
                                    }
                                    Point::Wall => match &mut next_map[i][1] {
                                        Point::Open => {
                                            next_map[i][1] = Point::Blizzard(vec![Direction::East]);
                                        }
                                        Point::Blizzard(b) => {
                                            b.push(Direction::East);
                                        }
                                        Point::Wall => panic!("Shouldn't happen"),
                                    },
                                },
                                Direction::West => match &mut next_map[i][j - 1] {
                                    Point::Open => {
                                        next_map[i][j - 1] = Point::Blizzard(vec![Direction::West]);
                                    }
                                    Point::Blizzard(b) => {
                                        b.push(Direction::West);
                                    }
                                    Point::Wall => match &mut next_map[i][map_columns - 2] {
                                        Point::Open => {
                                            next_map[i][map_columns - 2] =
                                                Point::Blizzard(vec![Direction::West]);
                                        }
                                        Point::Blizzard(b) => {
                                            b.push(Direction::West);
                                        }
                                        Point::Wall => panic!("Shouldn't happen"),
                                    },
                                },
                            }
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
