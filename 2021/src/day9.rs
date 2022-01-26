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
// The file for day9 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn bason_size(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> i32 {
    fn count_size(
        grid: &Vec<Vec<u32>>,
        aleady_counted: &mut Vec<(usize, usize)>,
        x: usize,
        y: usize,
    ) -> i32 {
        if !aleady_counted.contains(&(x, y)) && grid[x][y] != 9 {
            println!("{},{}", x, y);
            let mut count = 1;

            aleady_counted.push((x, y));

            if x != 0 {
                count += count_size(grid, aleady_counted, x - 1, y);
            }
            if x + 1 < grid.len() {
                count += count_size(grid, aleady_counted, x + 1, y);
            }
            if y != 0 {
                count += count_size(grid, aleady_counted, x, y - 1);
            }
            if y + 1 < grid[0].len() {
                count += count_size(grid, aleady_counted, x, y + 1);
            }

            count
        } else {
            0
        }
    }

    let size = count_size(&grid, &mut Vec::new(), x, y);

    println!("{}", size);
    size
}

fn main() {
    let filename = "data/day9_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };
    let lines = file_contents.trim().split("\n").collect::<Vec<&str>>();

    let grid = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|string| string.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut total_risk = 0;

    let mut low_points: Vec<(usize, usize)> = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut is_lower_than_neighbers = true;

            if i != 0 {
                is_lower_than_neighbers = is_lower_than_neighbers && grid[i - 1][j] > grid[i][j];
            }
            if i + 1 < grid.len() {
                is_lower_than_neighbers = is_lower_than_neighbers && grid[i + 2][j] > grid[i][j];
            }
            if j != 0 {
                is_lower_than_neighbers = is_lower_than_neighbers && grid[i][j - 1] > grid[i][j];
            }
            if j + 1 < grid[0].len() {
                is_lower_than_neighbers = is_lower_than_neighbers && grid[i][j + 1] > grid[i][j];
            }

            if is_lower_than_neighbers {
                total_risk += grid[i][j] + 1;
                low_points.push((i, j));
            }
        }
    }

    let mut bason_risks = low_points
        .iter()
        .map(|(x, y)| bason_size(&grid, *x, *y)).collect::<Vec<i32>>();

    bason_risks.sort();
    bason_risks.reverse();

    let total_bason_risk = bason_risks[0] * bason_risks[1] * bason_risks[2];
    println!("{}", total_risk);
    println!("{}", total_bason_risk);
}
