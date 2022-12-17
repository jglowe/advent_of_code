///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                _             ___
//                               | |           / _ \
//                             __| | __ _ _   | (_) |
//                            / _` |/ _` | | | \__, |
//                           | (_| | (_| | |_| | / /
//                            \__,_|\__,_|\__, |/_/
//                                         __/ |
//                                        |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day9 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn move_tail(head: &Point, tail: &Point) -> Point {
    // Detects if nothing needs to be moved.
    if (tail.x - 1 == head.x || tail.x == head.x || tail.x + 1 == head.x)
        && (tail.y - 1 == head.y || tail.y == head.y || tail.y + 1 == head.y)
    {
        return tail.clone();
    }

    if tail.x == head.x {
        if tail.y + 2 == head.y {
            return Point {
                x: tail.x,
                y: tail.y + 1,
            };
        }
        if tail.y - 2 == head.y {
            return Point {
                x: tail.x,
                y: tail.y - 1,
            };
        }
    }
    if tail.y == head.y {
        if tail.x + 2 == head.x {
            return Point {
                x: tail.x + 1,
                y: tail.y,
            };
        }
        if tail.x - 2 == head.x {
            return Point {
                x: tail.x - 1,
                y: tail.y,
            };
        }
    }

    if tail.x < head.x && tail.y < head.y {
        return Point {
            x: tail.x + 1,
            y: tail.y + 1,
        };
    }

    if tail.x > head.x && tail.y > head.y {
        return Point {
            x: tail.x - 1,
            y: tail.y - 1,
        };
    }

    if tail.x > head.x && tail.y < head.y {
        return Point {
            x: tail.x - 1,
            y: tail.y + 1,
        };
    }

    if tail.x < head.x && tail.y > head.y {
        return Point {
            x: tail.x + 1,
            y: tail.y - 1,
        };
    }

    assert!(false, "Movement not accounted for {:?} {:?}", head, tail);
    Point { x: 0, y: 0 }
}

fn tail_visits(path: &str, rope_length: usize) -> usize {
    assert!(rope_length >= 2, "The rope must be at least 2 long");
    let mut rope: Vec<Point> = Vec::new();
    let mut tail_visited: HashSet<Point> = HashSet::new();

    for _ in 0..rope_length {
        rope.push(Point { x: 0, y: 0 });
    }

    tail_visited.insert(rope[rope_length - 1].clone());

    for line in path.trim().split("\n") {
        let line_parts: Vec<&str> = line.split(" ").collect();
        assert!(
            line_parts.len() == 2,
            "Each line should have a direction and length"
        );

        let length: i64 = line_parts[1].parse().unwrap();
        match line_parts[0] {
            "U" => {
                for _ in 0..length {
                    rope[0].y += 1;

                    for i in 0..rope_length - 1 {
                        rope[i + 1] = move_tail(&rope[i], &rope[i + 1]);
                        tail_visited.insert(rope[rope_length - 1].clone());
                    }
                }
            }
            "D" => {
                for _ in 0..length {
                    rope[0].y -= 1;

                    for i in 0..rope_length - 1 {
                        rope[i + 1] = move_tail(&rope[i], &rope[i + 1]);
                        tail_visited.insert(rope[rope_length - 1].clone());
                    }
                }
            }
            "L" => {
                for _ in 0..length {
                    rope[0].x -= 1;

                    for i in 0..rope_length - 1 {
                        rope[i + 1] = move_tail(&rope[i], &rope[i + 1]);
                        tail_visited.insert(rope[rope_length - 1].clone());
                    }
                }
            }
            "R" => {
                for _ in 0..length {
                    rope[0].x += 1;

                    for i in 0..rope_length - 1 {
                        rope[i + 1] = move_tail(&rope[i], &rope[i + 1]);
                        tail_visited.insert(rope[rope_length - 1].clone());
                    }
                }
            }
            _ => assert!(false, "Not a known direction"),
        }
    }
    tail_visited.len()
}

fn main() {
    let input = advent::read_file("data/day9_input.txt");

    println!("Tail Visits: {}", tail_visits(&input, 2));
    println!("Long Tail Visits: {}", tail_visits(&input, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        let input = "R 4\n\
                     U 4\n\
                     L 3\n\
                     D 1\n\
                     R 4\n\
                     D 1\n\
                     L 5\n\
                     R 2";

        assert_eq!(13, tail_visits(&input, 2));
        assert_eq!(1, tail_visits(&input, 10));

        let input2 = "R 5\n\
                      U 8\n\
                      L 8\n\
                      D 3\n\
                      R 17\n\
                      D 10\n\
                      L 25\n\
                      U 20";
        assert_eq!(36, tail_visits(&input2, 10));
    }
}
