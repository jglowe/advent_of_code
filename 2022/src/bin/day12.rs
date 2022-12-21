///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                        _            __ ___
//                                       | |          /_ |__ \
//                                     __| | __ _ _   _| |  ) |
//                                    / _` |/ _` | | | | | / /
//                                   | (_| | (_| | |_| | |/ /_
//                                    \__,_|\__,_|\__, |_|____|
//                                                 __/ |
//                                                |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day12 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_map_point(map: &Vec<Vec<char>>, point: char) -> (usize, usize) {
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if map[row][column] == point {
                return (row, column);
            }
        }
    }
    panic!("Start not found");
}

fn initialize_vec<T: Copy>(rows: usize, columns: usize, default: T) -> Vec<Vec<T>> {
    let mut vector: Vec<Vec<T>> = Vec::new();

    for _ in 0..rows {
        let mut row: Vec<T> = Vec::new();
        for _ in 0..columns {
            row.push(default);
        }
        vector.push(row);
    }

    vector
}

fn find_shortest_path(map: &mut Vec<Vec<char>>) -> Option<usize> {
    let (start_row, start_column) = find_map_point(&map, 'S');
    let (end_row, end_column) = find_map_point(&map, 'E');

    map[end_row][end_column] = 'z';
    let mut distances: Vec<Vec<Option<usize>>> = initialize_vec(map.len(), map[0].len(), None);
    let mut visited: Vec<Vec<bool>> = initialize_vec(map.len(), map[0].len(), false);

    distances[start_row][start_column] = Some(0);

    let mut unknown_nodes = BinaryHeap::new();
    unknown_nodes.push(Reverse((0, (start_row, start_column))));

    while let Some(Reverse((_, (x, y)))) = unknown_nodes.pop() {
        if visited[x][y] {
            continue;
        }

        visited[x][y] = true;
        let dist = distances[x][y].unwrap();

        let mut adjcent = Vec::new();
        let current_letter = if map[x][y] == 'S' {
            'a'
        } else if map[x][y] == 'E' {
            'z'
        } else {
            map[x][y]
        };
        let next_letter = if current_letter != 'z' {
            (current_letter as u8 + 1) as char
        } else {
            'z'
        };

        if x != 0
            && (map[x - 1][y] == current_letter
                || map[x - 1][y] < current_letter
                || map[x - 1][y] == next_letter)
        {
            adjcent.push((x - 1, y));
        }
        if y != 0
            && (map[x][y - 1] == current_letter
                || map[x][y - 1] < current_letter
                || map[x][y - 1] == next_letter)
        {
            adjcent.push((x, y - 1));
        }
        if x + 1 < map.len()
            && (map[x + 1][y] == current_letter
                || map[x + 1][y] < current_letter
                || map[x + 1][y] == next_letter)
        {
            adjcent.push((x + 1, y));
        }
        if y + 1 < map[0].len()
            && (map[x][y + 1] == current_letter
                || map[x][y + 1] < current_letter
                || map[x][y + 1] == next_letter)
        {
            adjcent.push((x, y + 1));
        }

        for (i, j) in adjcent.iter() {
            if !visited[*i][*j] {
                match distances[*i][*j] {
                    Some(d) => {
                        if dist + 1 < d {
                            distances[*i][*j] = Some(dist + 1);
                        }
                    }
                    None => {
                        distances[*i][*j] = Some(dist + 1);
                    }
                }
                unknown_nodes.push(Reverse((distances[*i][*j].unwrap(), (*i, *j))));
            }
        }
    }

    map[end_row][end_column] = 'E';
    distances[end_row][end_column]
}

fn find_shortest_path_from_a(map: &mut Vec<Vec<char>>) -> usize {
    let mut a_locations: Vec<(usize, usize)> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();
    let (start_row, start_column) = find_map_point(&map, 'S');

    map[start_row][start_column] = 'a';

    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if map[row][column] == 'a' {
                a_locations.push((row, column));
            }
        }
    }

    for (row, column) in a_locations {
        map[row][column] = 'S';
        match find_shortest_path(map) {
            Some(x) => distances.push(x),
            None => (),
        }
        map[row][column] = 'a';
    }

    *distances.iter().min().unwrap()
}

fn main() {
    let input = advent::read_file("data/day12_input.txt");
    let mut map = parse_input(&input);

    println!("Shortest Path Steps: {}", find_shortest_path(&mut map).unwrap());
    println!(
        "Shortest Path Steps: {}",
        find_shortest_path_from_a(&mut map)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario() {
        let input = "Sabqponm\n\
                     abcryxxl\n\
                     accszExk\n\
                     acctuvwj\n\
                     abdefghi";

        let mut map = parse_input(&input);
        assert_eq!(31, find_shortest_path(&mut map).unwrap());
        assert_eq!(29, find_shortest_path_from_a(&mut map));
    }
}
