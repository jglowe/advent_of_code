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
    numbers: Vec<Vec<i64>>,
    called: Vec<Vec<bool>>,
    won: bool,
}

impl BingoBoard {
    fn new(numbers: Vec<Vec<i64>>) -> BingoBoard {
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

    fn sum_of_remaining(&self) -> i64 {
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

fn find_wins(input: &str) -> Vec<i64> {
    let mut wins: Vec<i64> = Vec::new();
    let mut numbers_called: Vec<i64> = Vec::new();
    let mut numbers_to_locations: HashMap<i64, Vec<(usize, usize, usize)>> = HashMap::new();
    let mut lines_loaded: Vec<Vec<i64>> = Vec::new();
    let mut boards: Vec<BingoBoard> = Vec::new();

    for (i, line) in input.split("\n").enumerate() {
        if i == 0 {
            numbers_called = line
                .split(",")
                .map(|string| string.parse::<i64>().unwrap())
                .collect();
        } else if line.len() != 0 {
            let current_board = boards.len();
            let current_row = lines_loaded.len();

            let row: Vec<i64> = line
                .split(" ")
                .filter(|string| string.len() != 0)
                .map(|string| string.parse::<i64>().unwrap())
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

                    wins.push(boards[*board].sum_of_remaining() * number);
                }
            }
        }
    }
    wins
}

fn main() {
    let input = match fs::read_to_string("data/day4_input.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("Error {}", e),
    };

    let wins = find_wins(&input);
    println!("Part 1 : {}", wins[0]);
    println!("Part 2 : {}", wins[wins.len() - 1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let wins = find_wins(&input);
        assert_eq!(4512, wins[0]);
    }

    #[test]
    fn part2_test() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let wins = find_wins(&input);
        assert_eq!(1924, wins[wins.len() - 1]);
    }
}
