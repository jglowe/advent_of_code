///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                _            __ __
//                               | |          /_ /_ |
//                             __| | __ _ _   _| || |
//                            / _` |/ _` | | | | || |
//                           | (_| | (_| | |_| | || |
//                            \__,_|\__,_|\__, |_||_|
//                                         __/ |
//                                        |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day11 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;

fn handle_nearby_flashing_fish(
    grid: &mut Vec<Vec<u32>>,
    flashing_fish: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
) -> () {
    if grid[x][y] != 0 {
        grid[x][y] += 1;
        if grid[x][y] > 9 {
            flashing_fish.push((x, y));
        }
    }
}

fn main() {
    let filename = "data/day11_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };
    let lines = file_contents.trim().split("\n").collect::<Vec<&str>>();

    let mut total_flashes = 0;

    let mut grid = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|string| string.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut iterations = 0;
    loop {
        iterations += 1;
        let mut flashing_fish: Vec<(usize, usize)> = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                grid[i][j] += 1;
                if grid[i][j] > 9 {
                    flashing_fish.push((i, j));
                }
            }
        }

        while flashing_fish.len() > 0 {
            let (x, y) = flashing_fish.pop().unwrap();

            // Already counted case
            if grid[x][y] == 0 {
                continue;
            }

            if iterations <= 100 {
                total_flashes += 1;
            }
            grid[x][y] = 0;

            // Cardinal directions
            if x != 0 {
                handle_nearby_flashing_fish(&mut grid, &mut flashing_fish, x - 1, y);
            }
            if y != 0 {
                handle_nearby_flashing_fish(&mut grid, &mut flashing_fish, x, y - 1);
            }
            if x + 1 < grid.len() {
                handle_nearby_flashing_fish(&mut grid, &mut flashing_fish, x + 1, y);
            }
            if y + 1 < grid[0].len() {
                handle_nearby_flashing_fish(&mut grid, &mut flashing_fish, x, y + 1);
            }
            // Check diagonals
            if x != 0 && y != 0 {
                handle_nearby_flashing_fish(&mut grid, &mut flashing_fish, x - 1, y - 1);
            }
            if x + 1 < grid.len() && y != 0 {
                handle_nearby_flashing_fish(&mut grid, &mut flashing_fish, x + 1, y - 1);
            }
            if x != 0 && y + 1 < grid[0].len() {
                handle_nearby_flashing_fish(&mut grid, &mut flashing_fish, x - 1, y + 1);
            }
            if x + 1 < grid.len() && y + 1 < grid[0].len() {
                handle_nearby_flashing_fish(&mut grid, &mut flashing_fish, x + 1, y + 1);
            }
        }

        let mut all_flashed = true;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                all_flashed = all_flashed && grid[i][j] == 0;
            }
        }
        if all_flashed {
            break;
        }
    }

    println!("{}", total_flashes);
    println!("{}", iterations);
}
