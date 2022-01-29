///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                              _            __ ____
//                             | |          /_ |___ \
//                           __| | __ _ _   _| | __) |
//                          / _` |/ _` | | | | ||__ <
//                         | (_| | (_| | |_| | |___) |
//                          \__,_|\__,_|\__, |_|____/
//                                       __/ |
//                                      |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day13 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashSet;
use std::fs;

#[derive(Eq, Hash)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {x, y}
    }
    fn clone(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

enum Fold {
    Up(usize),
    Left(usize),
}

fn parse_input(input: &str) -> (HashSet<Point>, Vec<Fold>) {
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    let mut is_on_folds = false;
    for line in input.trim().split("\n") {
        if line == "" {
            is_on_folds = true
        } else if is_on_folds {
            match line.split_once("=").unwrap() {
                ("fold along x", x) => folds.push(Fold::Left(x.parse().unwrap())),
                ("fold along y", y) => folds.push(Fold::Up(y.parse().unwrap())),
                _ => panic!("Invalid String"),
            }
        } else {
            let (x, y) = line.split_once(",").unwrap();
            points.insert(Point::new(x.parse().unwrap(), y.parse().unwrap()));
        }
    }

    (points, folds)
}

fn fold_once(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let mut new_points = HashSet::new();

    match fold {
        Fold::Left(x) => for point in points {
            if point.x > *x {
                let diff = point.x - *x;
                new_points.insert(Point::new(*x - diff, point.y));
            } else {
                new_points.insert((*point).clone());
            }
        },
        Fold::Up(y) => for point in points {
            if point.y > *y {
                let diff = point.y - *y;
                new_points.insert(Point::new(point.x, *y - diff));
            } else {
                new_points.insert((*point).clone());
            }
        }
    }

    new_points
}

fn fold(points: HashSet<Point>, folds: Vec<Fold>) -> HashSet<Point> {
    let mut final_points = points;

    for fold in folds {
        final_points = fold_once(&final_points, &fold);
    }

    final_points
}

fn points_to_string(points: &HashSet<Point>) -> String {
    let mut max_x = 0;
    let mut max_y = 0;
    for point in points.iter() {
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    let mut output = "".to_string();

    for y in 0..max_y +1 {
        for x in 0..max_x + 1 {
            if (*points).contains(&Point::new(x,y)) {
                output.push_str("#");
            } else {
                output.push_str(".");
            }
        }
        output.push_str("\n");
    }

    output
}

fn main() {
    let filename = "data/day13_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let (points, folds) = parse_input(&file_contents);

    let folded_once = fold_once(&points, &folds[0]);
    println!("Part 1 : {}", folded_once.len());

    let folded_all = fold(points, folds);

    println!("Part2 :\n{}", points_to_string(&folded_all));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "6,10\n\
                     0,14\n\
                     9,10\n\
                     0,3\n\
                     10,4\n\
                     4,11\n\
                     6,0\n\
                     6,12\n\
                     4,1\n\
                     0,13\n\
                     10,12\n\
                     3,4\n\
                     3,0\n\
                     8,4\n\
                     1,10\n\
                     2,14\n\
                     8,10\n\
                     9,0\n\
                     \n\
                     fold along y=7\n\
                     fold along x=5\n";
        let (points, folds) = parse_input(&input);

        let folded_once = fold_once(&points, &folds[0]);
        assert_eq!(17, folded_once.len());

        let folded_all = fold(points, folds);

        assert_eq!(points_to_string(&folded_all), "#####\n\
                                                   #...#\n\
                                                   #...#\n\
                                                   #...#\n\
                                                   #####\n")
    }
}
