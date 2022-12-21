///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                       _            __ _  _
//                                      | |          /_ | || |
//                                    __| | __ _ _   _| | || |_
//                                   / _` |/ _` | | | | |__   _|
//                                  | (_| | (_| | |_| | |  | |
//                                   \__,_|\__,_|\__, |_|  |_|
//                                                __/ |
//                                               |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day14 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
enum CavePoint {
    Air,
    Rock,
    Sand,
}

fn parse_cave(input: &str) -> (Vec<Vec<CavePoint>>, usize) {
    let mut cave: Vec<Vec<CavePoint>> = Vec::new();

    for _ in 0..500 {
        let mut row: Vec<CavePoint> = Vec::new();
        for _ in 0..1000 {
            row.push(CavePoint::Air);
        }
        cave.push(row);
    }

    let mut largest_row = 0;
    for line in input.trim().lines() {
        let sections: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|s| {
                let t: Vec<usize> = s.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
                (t[0], t[1])
            })
            .collect();
        assert!(
            sections.len() >= 2,
            "Each section should have at least two points"
        );

        let (mut column, mut row) = sections[0];
        cave[row][column] = CavePoint::Rock;
        for i in 1..sections.len() {
            while (column, row) != sections[i] {
                if row < sections[i].1 {
                    row += 1;
                    cave[row][column] = CavePoint::Rock;
                } else if row > sections[i].1 {
                    row -= 1;
                    cave[row][column] = CavePoint::Rock;
                } else if column < sections[i].0 {
                    column += 1;
                    cave[row][column] = CavePoint::Rock;
                } else if column > sections[i].0 {
                    column -= 1;
                    cave[row][column] = CavePoint::Rock;
                } else {
                    panic!("Shouldn't happen");
                }
                if row > largest_row {
                    largest_row = row;
                }
            }
        }
    }

    (cave, largest_row)
}

fn fill_till_void(cave: &mut Vec<Vec<CavePoint>>) -> usize {
    let mut not_filled = true;
    let mut sand_count = 0;
    while not_filled {
        let (mut sand_row, mut sand_column) = (0, 500);
        let mut not_settled = true;

        while not_settled {
            if sand_row + 1 >= cave.len() {
                not_settled = false;
                not_filled = false;
            } else if cave[sand_row + 1][sand_column] == CavePoint::Air {
                sand_row += 1;
            } else if cave[sand_row + 1][sand_column - 1] == CavePoint::Air {
                sand_row += 1;
                sand_column -= 1;
            } else if cave[sand_row + 1][sand_column + 1] == CavePoint::Air {
                sand_row += 1;
                sand_column += 1;
            } else {
                not_settled = false;
                cave[sand_row][sand_column] = CavePoint::Sand;
            }
        }
        if not_filled {
            sand_count += 1;
        }
    }

    sand_count
}

fn fill_cave(cave: &mut Vec<Vec<CavePoint>>, largest_row: usize) -> usize {
    let mut row: Vec<CavePoint> = Vec::new();
    for _ in 0..1000 {
        row.push(CavePoint::Rock);
    }

    cave[largest_row + 2] = row;

    let mut not_filled = true;
    let mut sand_count = 0;
    while not_filled {
        let (mut sand_row, mut sand_column) = (0, 500);
        let mut not_settled = true;

        if cave[sand_row][sand_column] == CavePoint::Sand {
            break;
        }

        while not_settled {
            if sand_row + 1 >= cave.len() {
                not_settled = false;
                not_filled = false;
            } else if cave[sand_row + 1][sand_column] == CavePoint::Air {
                sand_row += 1;
            } else if cave[sand_row + 1][sand_column - 1] == CavePoint::Air {
                sand_row += 1;
                sand_column -= 1;
            } else if cave[sand_row + 1][sand_column + 1] == CavePoint::Air {
                sand_row += 1;
                sand_column += 1;
            } else {
                not_settled = false;
                cave[sand_row][sand_column] = CavePoint::Sand;
            }
        }
        if not_filled {
            sand_count += 1;
        }
    }

    sand_count
}

fn main() {
    let input = advent::read_file("data/day14_input.txt");
    let (mut cave, _) = parse_cave(&input);
    println!("Cave Filled after: {} ", fill_till_void(&mut cave));
    let (mut cave, largest_row) = parse_cave(&input);
    println!("Cave Filled after: {} ", fill_cave(&mut cave, largest_row));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario() {
        let input = "498,4 -> 498,6 -> 496,6\n\
                     503,4 -> 502,4 -> 502,9 -> 494,9";

        let (mut cave, _) = parse_cave(&input);
        assert_eq!(24, fill_till_void(&mut cave));
        let (mut cave, largest_row) = parse_cave(&input);
        assert_eq!(93, fill_cave(&mut cave, largest_row));
    }
}
