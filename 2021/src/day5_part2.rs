///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _             _____
//                              | |           | ____|
//                            __| | __ _ _   _| |__
//                           / _` |/ _` | | | |___ \
//                          | (_| | (_| | |_| |___) |
//                           \__,_|\__,_|\__, |____/
//                                        __/ |
//                                       |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day5 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;

        Ok(Point { x, y })
    }
}

fn main() {
    let filename = "data/day5_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let mut fault_line_points: HashMap<Point, i32> = HashMap::new();

    for line in file_contents.split("\n") {
        if line == "" {
            continue;
        }

        let (start, end) = line.split_once(" -> ").unwrap();

        let point1: Point = start.parse().unwrap();
        let point2: Point = end.parse().unwrap();

        if point1.x == point2.x {
            let start = point1.y.min(point2.y);
            let end = point1.y.max(point2.y) + 1;
            for i in start..end {
                let count = fault_line_points
                    .entry(Point::new(point1.x, i))
                    .or_insert(0);
                *count = *count + 1;
            }
        } else if point1.y == point2.y {
            let start = point1.x.min(point2.x);
            let end = point1.x.max(point2.x) + 1;
            for i in start..end {
                let count = fault_line_points
                    .entry(Point::new(i, point1.y))
                    .or_insert(0);
                *count = *count + 1;
            }
        } else if (point1.x - point2.x).abs() == (point1.y - point2.y).abs() {
            let start_x = point1.x.min(point2.x);
            let end_x = point1.x.max(point2.x) + 1;
            let (start_y, other_y) = if start_x == point1.x {
                (point1.y, point2.y)
            } else {
                (point2.y, point1.y)
            };
            for i in start_x..end_x {
                let y = if start_y > other_y {
                    start_y - (i - start_x)
                } else {
                    start_y + (i - start_x)
                };
                let count = fault_line_points.entry(Point::new(i, y)).or_insert(0);
                *count = *count + 1;
            }
        }
    }

    println!(
        "{}",
        fault_line_points
            .iter()
            .fold(0, |acc, (_, value)| acc + if *value > 1 { 1 } else { 0 })
    );
}
