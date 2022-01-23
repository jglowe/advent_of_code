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

        println!("{}", line);
        let points: Vec<&str> = line.split(" -> ").collect();

        let point1: Point = Point::from_str(points[0]).unwrap();
        let point2: Point = Point::from_str(points[1]).unwrap();

        println!("{}.{} -> {}.{}", point1.x, point1.y, point2.x, point2.y);

        if point1.x == point2.x {
            let start = point1.y.min(point2.y);
            let end = point1.y.max(point2.y) + 1;
            for i in start..end {
                println!("{},{}", point1.x, i);
                let count = fault_line_points
                    .entry(Point::new(point1.x, i))
                    .or_insert(0);
                *count = *count + 1;
            }
        } else if point1.y == point2.y {
            let start = point1.x.min(point2.x);
            let end = point1.x.max(point2.x) + 1;
            for i in start..end {
                println!("{},{}", i, point1.y);
                let count = fault_line_points
                    .entry(Point::new(i, point1.y))
                    .or_insert(0);
                *count = *count + 1;
            }
        }
    }

    for i in 0..11 {
        let mut line: Vec<i32> = Vec::new();
        for j in 0..11 {
            let value: Option<&i32> = fault_line_points.get(&Point::new(j, i));
            match value {
                Some(v) => line.push(*v),
                None => line.push(0),
            }
        }

        println!(
            "{}",
            line.iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }

    let values: Vec<i32> = fault_line_points.into_values().filter(|x| *x > 1).collect();
    println!("{}", values.len());
}
