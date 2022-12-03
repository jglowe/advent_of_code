///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                _             ___
//                               | |           |__ \
//                             __| | __ _ _   _   ) |
//                            / _` |/ _` | | | | / /
//                           | (_| | (_| | |_| |/ /_
//                            \__,_|\__,_|\__, |____|
//                                         __/ |
//                                        |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day2 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Choice {
    X,
    Y,
    Z,
}

struct Round {
    opponent: Hand,
    you: Choice,
}

fn parse_rounds(input: String) -> Vec<Round> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let choices: Vec<&str> = line.split(" ").collect();

            assert!(
                choices.len() == 2,
                "There should only be two hands in a match"
            );

            let opponent_choice = match choices[0] {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                    "C" => Hand::Scissors,
                _ => panic!("Opponent didn't choose rock, paper, or scissors"),
            };

            let your_choice = match choices[1] {
                "X" => Choice::X,
                "Y" => Choice::Y,
                "Z" => Choice::Z,
                _ => panic!("You didn't choose rock, paper, or scissors"),
            };

            Round {
                opponent: opponent_choice,
                you: your_choice,
            }
        })
        .collect()
}

fn compute_score(rounds: &Vec<Round>) -> i64 {
    rounds
        .into_iter()
        .map(|round| {
            let choice_score = match &round.you {
                Choice::X => 1,
                Choice::Y => 2,
                Choice::Z => 3,
            };

            let win_score = match &round {
                Round {
                    opponent: Hand::Rock,
                    you: Choice::X,
                } => 3,
                Round {
                    opponent: Hand::Rock,
                    you: Choice::Y,
                } => 6,
                Round {
                    opponent: Hand::Rock,
                    you: Choice::Z,
                } => 0,
                Round {
                    opponent: Hand::Paper,
                    you: Choice::X,
                } => 0,
                Round {
                    opponent: Hand::Paper,
                    you: Choice::Y,
                } => 3,
                Round {
                    opponent: Hand::Paper,
                    you: Choice::Z,
                } => 6,
                Round {
                    opponent: Hand::Scissors,
                    you: Choice::X,
                } => 6,
                Round {
                    opponent: Hand::Scissors,
                    you: Choice::Y,
                } => 0,
                Round {
                    opponent: Hand::Scissors,
                    you: Choice::Z,
                } => 3,
            };

            choice_score + win_score
        })
        .sum()
}

fn compute_score2(rounds: &Vec<Round>) -> i64 {
    rounds
        .into_iter()
        .map(|round| {
            let choice_score = match &round {
                Round {
                    opponent: Hand::Rock,
                    you: Choice::X,
                } => 3,
                Round {
                    opponent: Hand::Rock,
                    you: Choice::Y,
                } => 1,
                Round {
                    opponent: Hand::Rock,
                    you: Choice::Z,
                } => 2,
                Round {
                    opponent: Hand::Paper,
                    you: Choice::X,
                } => 1,
                Round {
                    opponent: Hand::Paper,
                    you: Choice::Y,
                } => 2,
                Round {
                    opponent: Hand::Paper,
                    you: Choice::Z,
                } => 3,
                Round {
                    opponent: Hand::Scissors,
                    you: Choice::X,
                } => 2,
                Round {
                    opponent: Hand::Scissors,
                    you: Choice::Y,
                } => 3,
                Round {
                    opponent: Hand::Scissors,
                    you: Choice::Z,
                } => 1,
            };

            let win_score = match &round.you {
                Choice::X => 0,
                Choice::Y => 3,
                Choice::Z => 6,
            };

            choice_score + win_score
        })
        .sum()
}

fn main() {
    let filename = "data/day2_input.txt";

    let input = aoc::read_file(filename);

    let rounds = parse_rounds(input);

    println!("Score: {}", compute_score(&rounds));
    println!("Score2: {}", compute_score2(&rounds));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_paper_scissors_scores() {
        let input = "A Y\n\
                     B X\n\
                     C Z";

        let rounds = parse_rounds(input.to_string());

        assert_eq!(15, compute_score(&rounds));
        assert_eq!(12, compute_score2(&rounds));
    }
}
