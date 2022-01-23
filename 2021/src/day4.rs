///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _             _  _
//                              | |           | || |
//                            __| | __ _ _   _| || |_
//                           / _` |/ _` | | | |__   _|
//                          | (_| | (_| | |_| |  | |
//                           \__,_|\__,_|\__, |  |_|
//                                        __/ |
//                                       |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day4 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::fs;

struct BingoBoard {
    numbers: Vec<Vec<i32>>,
    called: Vec<Vec<bool>>,
    won: bool,
}

impl BingoBoard {
    fn new(numbers: Vec<Vec<i32>>) -> BingoBoard {
        let called = Vec::from([
            Vec::from([false, false, false, false, false]),
            Vec::from([false, false, false, false, false]),
            Vec::from([false, false, false, false, false]),
            Vec::from([false, false, false, false, false]),
            Vec::from([false, false, false, false, false]),
        ]);
        let won = false;
        BingoBoard {
            numbers,
            called,
            won,
        }
    }

    fn sum_of_remaining(&self) -> i32 {
        let mut sum = 0;
        for row in 0..5 {
            for column in 0..5 {
                if !self.called[row][column] {
                    sum += self.numbers[row][column];
                }
            }
        }
        sum
    }

    fn won(&self) -> bool {
        let mut won = false;

        // Row win
        for i in 0..5 {
            won = won
                || (self.called[i][4]
                    && self.called[i][3]
                    && self.called[i][2]
                    && self.called[i][1]
                    && self.called[i][0]);
        }

        // Column win
        for i in 0..5 {
            won = won
                || (self.called[4][i]
                    && self.called[3][i]
                    && self.called[2][i]
                    && self.called[1][i]
                    && self.called[0][i]);
        }
        won
    }
}

fn main() {
    let filename = "data/day4_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let mut numbers_called: Vec<i32> = Vec::new();
    let mut numbers_to_locations: HashMap<i32, Vec<(usize, usize, usize)>> = HashMap::new();
    let mut lines_loaded: Vec<Vec<i32>> = Vec::new();
    let mut boards: Vec<BingoBoard> = Vec::new();

    for (i, line) in file_contents.split("\n").enumerate() {
        if i == 0 {
            numbers_called = line
                .split(",")
                .map(|string| string.parse::<i32>().unwrap())
                .collect();
        } else if line.len() != 0 {
            let current_board = boards.len();
            let current_row = lines_loaded.len();

            let row: Vec<i32> = line
                .split(" ")
                .filter(|string| string.len() != 0)
                .map(|string| string.parse::<i32>().unwrap())
                .collect();

            for (current_column, number) in row.iter().enumerate() {
                let list = numbers_to_locations.entry(*number).or_insert(Vec::new());
                list.push((current_board, current_row, current_column));
            }

            lines_loaded.push(row);

            if lines_loaded.len() == 5 {
                boards.push(BingoBoard::new(lines_loaded));
                lines_loaded = Vec::new();
            }
        }
    }

    for number in numbers_called {
        if let Some(locations) = numbers_to_locations.get(&number) {
            for (board, row, column) in locations {
                boards[*board].called[*row][*column] = true;

                if boards[*board].won() && !boards[*board].won {
                    boards[*board].won = true;

                    println!("{}", boards[*board].sum_of_remaining() * number);
                }
            }
        }
    }
}
