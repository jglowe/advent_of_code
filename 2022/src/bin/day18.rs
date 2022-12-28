///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                        _            __  ___
//                                       | |          /_ |/ _ \
//                                     __| | __ _ _   _| | (_) |
//                                    / _` |/ _` | | | | |> _ <
//                                   | (_| | (_| | |_| | | (_) |
//                                    \__,_|\__,_|\__, |_|\___/
//                                                 __/ |
//                                                |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day18 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

fn parse_input(input: &str) -> (HashSet<Point>, Vec<Point>) {
    let mut map = HashSet::new();
    let mut points = Vec::new();
    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(",").collect();
        assert!(parts.len() == 3, "Malformed coordinate");

        let x = parts[0].parse().unwrap();
        let y = parts[1].parse().unwrap();
        let z = parts[2].parse().unwrap();
        let point = Point { x, y, z };
        points.push(point.clone());
        map.insert(point);
    }
    (map, points)
}

fn part1(map: &HashSet<Point>, points: &Vec<Point>) -> usize {
    let mut surface_area = 0;

    for point in points {
        let x_plus = Point {
            x: point.x + 1,
            y: point.y,
            z: point.z,
        };
        let x_minus = Point {
            x: point.x - 1,
            y: point.y,
            z: point.z,
        };
        let y_plus = Point {
            x: point.x,
            y: point.y + 1,
            z: point.z,
        };
        let y_minus = Point {
            x: point.x,
            y: point.y - 1,
            z: point.z,
        };
        let z_plus = Point {
            x: point.x,
            y: point.y,
            z: point.z + 1,
        };
        let z_minus = Point {
            x: point.x,
            y: point.y,
            z: point.z - 1,
        };
        if !map.contains(&x_plus) {
            surface_area += 1;
        }
        if !map.contains(&x_minus) {
            surface_area += 1;
        }
        if !map.contains(&y_plus) {
            surface_area += 1;
        }
        if !map.contains(&y_minus) {
            surface_area += 1;
        }
        if !map.contains(&z_plus) {
            surface_area += 1;
        }
        if !map.contains(&z_minus) {
            surface_area += 1;
        }
    }

    surface_area
}

fn is_in_pocket(point: &Point, map: &HashSet<Point>) -> bool {
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(point.clone());
    let mut visited: HashSet<Point> = HashSet::new();

    while let Some(point) = queue.pop_back() {
        if point.x == 0
            || point.x >= 25
            || point.y == 0
            || point.y >= 25
            || point.z == 0
            || point.z >= 25
        {
            return false;
        }
        visited.insert(point.clone());
        let x_plus = Point {
            x: point.x + 1,
            y: point.y,
            z: point.z,
        };
        let x_minus = Point {
            x: point.x - 1,
            y: point.y,
            z: point.z,
        };
        let y_plus = Point {
            x: point.x,
            y: point.y + 1,
            z: point.z,
        };
        let y_minus = Point {
            x: point.x,
            y: point.y - 1,
            z: point.z,
        };
        let z_plus = Point {
            x: point.x,
            y: point.y,
            z: point.z + 1,
        };
        let z_minus = Point {
            x: point.x,
            y: point.y,
            z: point.z - 1,
        };
        if !map.contains(&x_plus) && !visited.contains(&x_plus) {
            queue.push_back(x_plus);
        }
        if !map.contains(&x_minus) && !visited.contains(&x_minus) {
            queue.push_back(x_minus);
        }
        if !map.contains(&y_plus) && !visited.contains(&y_plus) {
            queue.push_back(y_plus);
        }
        if !map.contains(&y_minus) && !visited.contains(&y_minus) {
            queue.push_back(y_minus);
        }
        if !map.contains(&z_plus) && !visited.contains(&z_plus) {
            queue.push_back(z_plus);
        }
        if !map.contains(&z_minus) && !visited.contains(&z_minus) {
            queue.push_back(z_minus);
        }
    }
    true
}

fn part2(map: &HashSet<Point>, points: &Vec<Point>) -> usize {
    let mut surface_area = 0;

    for point in points {
        let x_plus = Point {
            x: point.x + 1,
            y: point.y,
            z: point.z,
        };
        let x_minus = Point {
            x: point.x - 1,
            y: point.y,
            z: point.z,
        };
        let y_plus = Point {
            x: point.x,
            y: point.y + 1,
            z: point.z,
        };
        let y_minus = Point {
            x: point.x,
            y: point.y - 1,
            z: point.z,
        };
        let z_plus = Point {
            x: point.x,
            y: point.y,
            z: point.z + 1,
        };
        let z_minus = Point {
            x: point.x,
            y: point.y,
            z: point.z - 1,
        };
        if !map.contains(&x_plus) && !is_in_pocket(&x_plus, &map) {
            surface_area += 1;
        }
        if !map.contains(&x_minus) && !is_in_pocket(&x_minus, &map) {
            surface_area += 1;
        }
        if !map.contains(&y_plus) && !is_in_pocket(&y_plus, &map) {
            surface_area += 1;
        }
        if !map.contains(&y_minus) && !is_in_pocket(&y_minus, &map) {
            surface_area += 1;
        }
        if !map.contains(&z_plus) && !is_in_pocket(&z_plus, &map) {
            surface_area += 1;
        }
        if !map.contains(&z_minus) && !is_in_pocket(&z_minus, &map) {
            surface_area += 1;
        }
    }

    surface_area
}

fn main() {
    let input = advent::read_file("data/day18_input.txt");
    let (map, points) = parse_input(&input);
    println!("Part 1: {}", part1(&map, &points));
    println!("Part 2: {}", part2(&map, &points));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_tiny() {
        let input = "1,1,1\n\
                     2,1,1\n";
        let (map, points) = parse_input(&input);
        assert_eq!(10, part1(&map, &points));
        assert_eq!(10, part2(&map, &points));
    }

    #[test]
    fn part1_test() {
        let input = "2,2,2\n\
                     1,2,2\n\
                     3,2,2\n\
                     2,1,2\n\
                     2,3,2\n\
                     2,2,1\n\
                     2,2,3\n\
                     2,2,4\n\
                     2,2,6\n\
                     1,2,5\n\
                     3,2,5\n\
                     2,1,5\n\
                     2,3,5\n";
        let (map, points) = parse_input(&input);
        assert_eq!(64, part1(&map, &points));
    }

    #[test]
    fn part2_test() {
        let input = "2,2,2\n\
                     1,2,2\n\
                     3,2,2\n\
                     2,1,2\n\
                     2,3,2\n\
                     2,2,1\n\
                     2,2,3\n\
                     2,2,4\n\
                     2,2,6\n\
                     1,2,5\n\
                     3,2,5\n\
                     2,1,5\n\
                     2,3,5\n";
        let (map, points) = parse_input(&input);
        assert_eq!(58, part2(&map, &points));
    }
}
