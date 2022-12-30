///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                       _             ___  ___
//                                      | |           |__ \|__ \
//                                    __| | __ _ _   _   ) |  ) |
//                                   / _` |/ _` | | | | / /  / /
//                                  | (_| | (_| | |_| |/ /_ / /_
//                                   \__,_|\__,_|\__, |____|____|
//                                                __/ |
//                                               |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day22 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

enum Point {
    Wall,
    Open,
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Forward(usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Map = HashMap<(usize, usize), Point>;

fn parse_input(input: &str) -> (Map, Vec<Instruction>, (usize, usize)) {
    let mut empty_line_found = false;
    let mut map: HashMap<(usize, usize), Point> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut max_row = 0;
    let mut max_column = 0;
    for (row, line) in input.lines().enumerate() {
        if line == "" {
            empty_line_found = true;
            continue;
        }
        if !empty_line_found {
            for (column, space) in line.chars().enumerate() {
                match space {
                    ' ' => (),
                    '.' => {
                        map.insert((row, column), Point::Open);
                    }
                    '#' => {
                        map.insert((row, column), Point::Wall);
                    }
                    _ => panic!("Wrong char found"),
                };
                max_column = max_column.max(column);
            }
            max_row = max_row.max(row);
        } else {
            let mut number_part = "".to_string();
            for item in line.chars() {
                match item {
                    '0' => {
                        number_part += "0";
                    }
                    '1' => {
                        number_part += "1";
                    }
                    '2' => {
                        number_part += "2";
                    }
                    '3' => {
                        number_part += "3";
                    }
                    '4' => {
                        number_part += "4";
                    }
                    '5' => {
                        number_part += "5";
                    }
                    '6' => {
                        number_part += "6";
                    }
                    '7' => {
                        number_part += "7";
                    }
                    '8' => {
                        number_part += "8";
                    }
                    '9' => {
                        number_part += "9";
                    }
                    'R' => {
                        instructions.push(Instruction::Forward(number_part.parse().unwrap()));
                        number_part = "".to_string();
                        instructions.push(Instruction::Right);
                    }
                    'L' => {
                        instructions.push(Instruction::Forward(number_part.parse().unwrap()));
                        number_part = "".to_string();
                        instructions.push(Instruction::Left);
                    }
                    _ => panic!("Invalid char in instructions"),
                }
            }
            instructions.push(Instruction::Forward(number_part.parse().unwrap()));
        }
    }
    (map, instructions, (max_row, max_column))
}

fn wrap_part1(
    map: &Map,
    direction: &Direction,
    row: usize,
    column: usize,
    max_row: usize,
    max_column: usize,
) -> (usize, usize) {
    let (n_row, n_column) = match direction {
        Direction::Up => {
            let mut row = max_row;
            while !map.contains_key(&(row, column)) {
                row -= 1;
            }
            (row, column)
        }
        Direction::Down => {
            let mut row = 0;
            while !map.contains_key(&(row, column)) {
                row += 1;
            }
            (row, column)
        }
        Direction::Left => {
            let mut column = max_column;
            while !map.contains_key(&(row, column)) {
                column -= 1;
            }
            (row, column)
        }
        Direction::Right => {
            let mut column = 0;
            while !map.contains_key(&(row, column)) {
                column += 1;
            }
            (row, column)
        }
    };
    match map.get(&(n_row, n_column)).unwrap() {
        Point::Wall => (row, column),
        Point::Open => (n_row, n_column),
    }
}

fn move_forward(
    map: &Map,
    direction: &Direction,
    row: usize,
    column: usize,
    max_row: usize,
    max_column: usize,
) -> (usize, usize) {
    let (wrap, n_row, n_column) = match direction {
        Direction::Up => {
            if row != 0 && map.contains_key(&(row - 1, column)) {
                (false, row-1, column)
            } else {
                (true, row, column)
            }
        }
        Direction::Down => {
            if map.contains_key(&(row + 1, column)) {
                (false, row+1, column)
            } else {
                (true, row, column)
            }
        }
        Direction::Left => {
            if column != 0 && map.contains_key(&(row, column - 1)) {
                (false, row, column - 1)
            } else {
                (true, row, column)
            }
        }
        Direction::Right => {
            if map.contains_key(&(row, column + 1)) {
                (false, row, column + 1)
            } else {
                (true, row, column)
            }
        }
    };

    if wrap {
        wrap_part1(map, &direction, row, column, max_row, max_column)
    } else {
        match map.get(&(n_row, n_column)).unwrap() {
            Point::Wall => (row, column),
            Point::Open => (n_row, n_column),
        }
    }

}

fn wrap_part2(
    map: &Map,
    direction: &Direction,
    row: usize,
    column: usize,
    edge_translation_map: &HashMap<(usize, usize, Direction), (usize, usize, Direction)>,
) -> (usize, usize, Direction) {
    let (n_row, n_column, n_direction) = edge_translation_map
        .get(&(row, column, direction.clone()))
        .unwrap();
    match map.get(&(*n_row, *n_column)).unwrap() {
        Point::Wall => (row, column, direction.clone()),
        Point::Open => (*n_row, *n_column, n_direction.clone()),
    }
}

fn move_forward_part2(
    map: &Map,
    direction: &Direction,
    row: usize,
    column: usize,
    edge_translation_map: &HashMap<(usize, usize, Direction), (usize, usize, Direction)>,
) -> (usize, usize, Direction) {
    let (wrap, n_row, n_column, n_direction) = match direction {
        Direction::Up => {
            if row != 0 && map.contains_key(&(row - 1, column)) {
                (false, row - 1, column, direction)
            } else {
                (true, row, column, direction)
            }
        }
        Direction::Down => {
            if map.contains_key(&(row + 1, column)) {
                (false, row + 1, column, direction)
            } else {
                (true, row, column, direction)
            }
        }
        Direction::Left => {
            if column != 0 && map.contains_key(&(row, column - 1)) {
                (false, row, column - 1, direction)
            } else {
                (true, row, column, direction)
            }
        }
        Direction::Right => {
            if map.contains_key(&(row, column + 1)) {
                (false, row, column + 1, direction)
            } else {
                (true, row, column, direction)
            }
        }
    };
    if wrap {
        wrap_part2(map, &direction, row, column, edge_translation_map)
    } else {
        match map.get(&(n_row, n_column)).unwrap() {
            Point::Wall => (row, column, direction.clone()),
            Point::Open => (n_row, n_column, n_direction.clone()),
        }
    }
}

fn part1(map: &Map, instructions: &Vec<Instruction>, max_row: usize, max_column: usize) -> usize {
    let mut direction = Direction::Right;
    let (mut row, mut column) = wrap_part1(map, &direction, 0, 0, max_row, max_column);

    for instruction in instructions {
        match (instruction, &direction) {
            (Instruction::Forward(x), d) => {
                for _ in 0..*x {
                    (row, column) = move_forward(map, d, row, column, max_row, max_column);
                }
            }
            (Instruction::Left, Direction::Up) => {
                direction = Direction::Left;
            }
            (Instruction::Left, Direction::Down) => {
                direction = Direction::Right;
            }
            (Instruction::Left, Direction::Left) => {
                direction = Direction::Down;
            }
            (Instruction::Left, Direction::Right) => {
                direction = Direction::Up;
            }
            (Instruction::Right, Direction::Up) => {
                direction = Direction::Right;
            }
            (Instruction::Right, Direction::Down) => {
                direction = Direction::Left;
            }
            (Instruction::Right, Direction::Left) => {
                direction = Direction::Up;
            }
            (Instruction::Right, Direction::Right) => {
                direction = Direction::Down;
            }
        }
    }

    1000 * (row + 1)
        + 4 * (column + 1)
        + match direction {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
}

fn part2(
    map: &Map,
    instructions: &Vec<Instruction>,
    max_row: usize,
    max_column: usize,
    cube_size: usize,
) -> usize {
    //
    // Sets up a warping map
    //
    let mut edge_translation_map: HashMap<(usize, usize, Direction), (usize, usize, Direction)> =
        HashMap::new();

    match (
        (
            map.contains_key(&(0, 0)),
            map.contains_key(&(0, cube_size)),
            map.contains_key(&(0, cube_size * 2)),
            map.contains_key(&(0, cube_size * 3)),
        ),
        (
            map.contains_key(&(cube_size, 0)),
            map.contains_key(&(cube_size, cube_size)),
            map.contains_key(&(cube_size, cube_size * 2)),
            map.contains_key(&(cube_size, cube_size * 3)),
        ),
        (
            map.contains_key(&(cube_size * 2, 0)),
            map.contains_key(&(cube_size * 2, cube_size)),
            map.contains_key(&(cube_size * 2, cube_size * 2)),
            map.contains_key(&(cube_size * 2, cube_size * 3)),
        ),
        (
            map.contains_key(&(cube_size * 3, 0)),
            map.contains_key(&(cube_size * 3, cube_size)),
            map.contains_key(&(cube_size * 3, cube_size * 2)),
            map.contains_key(&(cube_size * 3, cube_size * 3)),
        ),
    ) {
        (
            (false, false, true, false),
            (true, true, true, false),
            (false, false, true, true),
            (false, false, false, false),
        ) => {
            // Top north edge
            for column in 0..cube_size {
                edge_translation_map.insert(
                    (0, column + cube_size * 2, Direction::Up),
                    (cube_size, cube_size - column - 1, Direction::Down),
                );
                edge_translation_map.insert(
                    (cube_size, cube_size - column - 1, Direction::Up),
                    (0, column + cube_size * 2, Direction::Down),
                );
            }

            // Top east edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (row, cube_size * 3 - 1, Direction::Right),
                    (cube_size * 2 - row, cube_size * 4 - 1, Direction::Left),
                );
                edge_translation_map.insert(
                    (cube_size * 2 - row, cube_size * 4 - 1, Direction::Right),
                    (row, cube_size * 3 - 1, Direction::Left),
                );
            }

            // Top south edge taken care of.

            // Top west edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (row, cube_size * 2, Direction::Left),
                    (cube_size, cube_size + row, Direction::Down),
                );
                edge_translation_map.insert(
                    (cube_size, cube_size + row, Direction::Up),
                    (row, cube_size * 2, Direction::Right),
                );
            }

            // South right edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (cube_size + row, cube_size * 3 - 1, Direction::Right),
                    (cube_size * 2, cube_size * 4 - 1 - row, Direction::Down),
                );
                edge_translation_map.insert(
                    (cube_size * 2, cube_size * 4 - 1 - row, Direction::Up),
                    (cube_size + row, cube_size * 3 - 1, Direction::Left),
                );
            }

            // South left edge taken care of

            // South bottom edge taken care of

            // North left edge taken care of

            // North right edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (cube_size + row, 0, Direction::Left),
                    (cube_size * 3 - 1, cube_size * 4 - 1 - row, Direction::Up),
                );
                edge_translation_map.insert(
                    (cube_size * 3 - 1, cube_size * 4 - 1 - row, Direction::Down),
                    (cube_size + row, 0, Direction::Right),
                );
            }

            // Bottom south edge taken care of

            // Bottom east edge taken care of

            // Bottom north edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (cube_size * 3 - 1, cube_size * 2 + row, Direction::Down),
                    (cube_size * 2 - 1, cube_size - 1 - row, Direction::Up),
                );
                edge_translation_map.insert(
                    (cube_size * 2 - 1, cube_size - 1 - row, Direction::Down),
                    (cube_size * 3 - 1, cube_size * 2 + row, Direction::Up),
                );
            }

            // Bottom west edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (row + cube_size * 2, cube_size * 2, Direction::Left),
                    (cube_size * 2 - 1, cube_size - 1 - row, Direction::Up),
                );
                edge_translation_map.insert(
                    (cube_size * 2 - 1, cube_size - 1 - row, Direction::Down),
                    (row + cube_size * 2, cube_size * 2, Direction::Right),
                );
            }
        }
        (
            (false, true, true, false),
            (false, true, false, false),
            (true, true, false, false),
            (true, false, false, false),
        ) => {
            // Top north edge
            for column in 0..cube_size {
                edge_translation_map.insert(
                    (0, column + cube_size, Direction::Up),
                    (cube_size * 3 + column, 0, Direction::Right),
                );
                edge_translation_map.insert(
                    (cube_size * 3 + column, 0, Direction::Left),
                    (0, column + cube_size, Direction::Down),
                );
            }

            // Top east edge taken care of

            // Top south edge taken care of

            // Top west edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (row, cube_size, Direction::Left),
                    (cube_size * 3 - 1 - row, 0, Direction::Right),
                );
                edge_translation_map.insert(
                    (cube_size * 3 - 1 - row, 0, Direction::Left),
                    (row, cube_size, Direction::Right),
                );
            }

            // South left edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (cube_size + row, cube_size, Direction::Left),
                    (cube_size * 2, row, Direction::Down),
                );
                edge_translation_map.insert(
                    (cube_size * 2, row, Direction::Up),
                    (cube_size + row, cube_size, Direction::Right),
                );
            }

            // South right edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (cube_size + row, cube_size * 2 - 1, Direction::Right),
                    (cube_size - 1, cube_size * 2 + row, Direction::Up),
                );
                edge_translation_map.insert(
                    (cube_size - 1, cube_size * 2 + row, Direction::Down),
                    (cube_size + row, cube_size * 2 - 1, Direction::Left),
                );
            }

            // South bottom edge taken care of

            // North left edge taken care of

            // North right edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (cube_size * 4 - 1, row, Direction::Down),
                    (0, cube_size * 2 + row, Direction::Down),
                );
                edge_translation_map.insert(
                    (0, cube_size * 2 + row, Direction::Up),
                    (cube_size * 4 - 1, row, Direction::Up),
                );
            }

            // Bottom south edge taken care of

            // Bottom west edge taken care of

            // Bottom north edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (cube_size * 3 - 1, cube_size + row, Direction::Down),
                    (cube_size * 3 + row, cube_size - 1, Direction::Left),
                );
                edge_translation_map.insert(
                    (cube_size * 3 + row, cube_size - 1, Direction::Right),
                    (cube_size * 3 - 1, cube_size + row, Direction::Up),
                );
            }

            // Bottom east edge
            for row in 0..cube_size {
                edge_translation_map.insert(
                    (row + cube_size * 2, cube_size * 2 - 1, Direction::Right),
                    (cube_size - 1 - row, cube_size * 3 - 1, Direction::Left),
                );
                edge_translation_map.insert(
                    (cube_size - 1 - row, cube_size * 3 - 1, Direction::Right),
                    (row + cube_size * 2, cube_size * 2 - 1, Direction::Left),
                );
            }
        }
        _ => panic!("Cube not implemented"),
    };

    //
    // Finishes the map
    //

    //
    // Runs each instruction
    //

    let mut direction = Direction::Right;
    let (mut row, mut column) = wrap_part1(map, &direction, 0, 0, max_row, max_column);

    for instruction in instructions {
        (row, column, direction) = match (instruction, direction) {
            (Instruction::Forward(x), d) => {
                let mut d = d;
                for _ in 0..*x {
                    (row, column, d) =
                        move_forward_part2(map, &d, row, column, &edge_translation_map);
                }
                (row, column, d)
            }
            (Instruction::Left, Direction::Up) => (row, column, Direction::Left),
            (Instruction::Left, Direction::Down) => (row, column, Direction::Right),
            (Instruction::Left, Direction::Left) => (row, column, Direction::Down),
            (Instruction::Left, Direction::Right) => (row, column, Direction::Up),
            (Instruction::Right, Direction::Up) => (row, column, Direction::Right),
            (Instruction::Right, Direction::Down) => (row, column, Direction::Left),
            (Instruction::Right, Direction::Left) => (row, column, Direction::Up),
            (Instruction::Right, Direction::Right) => (row, column, Direction::Down),
        };
    }

    //
    // Calculates results
    //

    1000 * (row + 1)
        + 4 * (column + 1)
        + match direction {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
}

fn main() {
    let input = advent::read_file("data/day22_input.txt");
    let (map, instructions, (max_row, max_column)) = parse_input(&input);
    println!(
        "Part 1 : {}",
        part1(&map, &instructions, max_row, max_column)
    );
    println!(
        "Part 2 : {}",
        part2(&map, &instructions, max_row, max_column, 50)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5\n";
        let (map, instructions, (max_row, max_column)) = parse_input(&input);
        assert_eq!(6032, part1(&map, &instructions, max_row, max_column));
    }

    #[test]
    fn part2_test() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5\n";
        let (map, instructions, (max_row, max_column)) = parse_input(&input);
        assert_eq!(5031, part2(&map, &instructions, max_row, max_column, 4));
    }
}
