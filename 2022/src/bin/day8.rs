///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _              ___
//                              | |            / _ \
//                            __| | __ _ _   _| (_) |
//                           / _` |/ _` | | | |> _ <
//                          | (_| | (_| | |_| | (_) |
//                           \__,_|\__,_|\__, |\___/
//                                        __/ |
//                                       |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day8 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_input(input: &String) -> Vec<Vec<u8>> {
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn is_visible(map: &Vec<Vec<u8>>, row: usize, column: usize) -> bool {
    let num_rows = map.len();
    let num_columns = map[0].len();

    if row == 0 || row == num_rows - 1 || column == 0 || column == num_columns - 1 {
        return true;
    }

    let mut all_smaller = true;

    for i in 0..row {
        if map[row][column] <= map[i][column] {
            all_smaller = false;
            break;
        }
    }

    if all_smaller {
        return true;
    }

    all_smaller = true;

    for i in row + 1..num_rows {
        if map[row][column] <= map[i][column] {
            all_smaller = false;
            break;
        }
    }

    if all_smaller {
        return true;
    }

    all_smaller = true;
    for i in 0..column {
        if map[row][column] <= map[row][i] {
            all_smaller = false;
            break;
        }
    }

    if all_smaller {
        return true;
    }

    all_smaller = true;
    for i in column + 1..num_columns {
        if map[row][column] <= map[row][i] {
            all_smaller = false;
            break;
        }
    }

    all_smaller
}

fn scenic_score(map: &Vec<Vec<u8>>, row: usize, column: usize) -> i64 {
    let num_rows = map.len();
    let num_columns = map[0].len();

    if row == 0 || row == num_rows - 1 || column == 0 || column == num_columns - 1 {
        return 0;
    }

    let mut top_count = 0;

    for i in (0..row).rev() {
        top_count += 1;
        if map[row][column] <= map[i][column] {
            break;
        }
    }

    let mut down_count = 0;

    for i in row + 1..num_rows {
        down_count += 1;
        if map[row][column] <= map[i][column] {
            break;
        }
    }

    let mut left_count = 0;

    for i in (0..column).rev() {
        left_count += 1;
        if map[row][column] <= map[row][i] {
            break;
        }
    }

    let mut right_count = 0;
    for i in column + 1..num_columns {
        right_count += 1;
        if map[row][column] <= map[row][i] {
            break;
        }
    }

    top_count * left_count * right_count * down_count
}

fn visible_count(map: &Vec<Vec<u8>>) -> i64 {
    let mut count = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_visible(&map, i, j) {
                count += 1;
            }
        }
    }

    count
}

fn highest_senic_score(map: &Vec<Vec<u8>>) -> i64 {
    let mut highest_score = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let score = scenic_score(map, i, j);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score
}

fn main() {
    let input = advent::read_file("data/day8_input.txt").trim().to_string();
    let map = parse_input(&input);

    println!("Visible Trees: {}", visible_count(&map));
    println!("Highest Scenic Score: {}", highest_senic_score(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        let input = "30373\n\
                     25512\n\
                     65332\n\
                     33549\n\
                     35390"
            .to_string();

        let map = parse_input(&input);

        assert_eq!(21, visible_count(&map));
        assert_eq!(8, highest_senic_score(&map));
    }
}
