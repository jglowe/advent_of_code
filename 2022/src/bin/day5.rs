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
// The file for day5 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

struct Move {
    number: i64,
    start: usize,
    end: usize,
}

struct Ship {
    container_stacks: Vec<Vec<char>>,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            container_stacks: Vec::new(),
        }
    }

    fn top(&self) -> String {
        let mut top = String::new();
        for stack in &self.container_stacks {
            if stack.len() > 0 {
                top.push(stack[stack.len() - 1]);
            } else {
                top.push(' ');
            }
        }

        top
    }

    fn run_moves(&mut self, moves: &Vec<Move>) {
        for m in moves {
            for _ in 0..m.number {
                let item = self.container_stacks[m.start].pop().unwrap();

                self.container_stacks[m.end].push(item);
            }
        }
    }

    fn run_moves_part2(&mut self, moves: &Vec<Move>) {
        for m in moves {
            let mut crain: Vec<char> = Vec::new();

            for _ in 0..m.number {
                crain.push(self.container_stacks[m.start].pop().unwrap());
            }

            for _ in 0..m.number {
                self.container_stacks[m.end].push(crain.pop().unwrap());
            }
        }
    }
}

fn parse_input(input: &str) -> (Ship, Vec<Move>) {
    let mut ship = Ship::new();
    let mut moves = Vec::new();
    let mut found_empty_line = false;

    let mut ship_lines: Vec<&str> = Vec::new();

    for line in input.split("\n") {
        if line == "" {
            found_empty_line = true;
            continue;
        }

        if found_empty_line {
            let move_parts: Vec<&str> = line.split(" ").collect();
            assert!(move_parts.len() == 6, "Move wrongly formatted");

            moves.push(Move {
                number: move_parts[1].parse().unwrap(),
                start: move_parts[3].parse::<usize>().unwrap() - 1,
                end: move_parts[5].parse::<usize>().unwrap() - 1,
            })
        } else {
            ship_lines.push(line)
        }
    }

    let line = ship_lines.pop().unwrap();
    let width = line.split("   ").collect::<Vec<&str>>().len();

    for _ in 0..width {
        ship.container_stacks.push(Vec::new());
    }

    while let Some(line) = ship_lines.pop() {
        for i in 0..width {
            let index = i * 4 + 1;

            if line.as_bytes()[index] != b' ' {
                ship.container_stacks[i].push(line.as_bytes()[index] as char);
            }
        }
    }

    (ship, moves)
}

fn main() {
    let input = aoc::read_file("data/day5_input.txt");
    let (mut ship, moves) = parse_input(&input);

    ship.run_moves(&moves);

    println!("Top containers of the ship: {}", ship.top());

    let (mut ship, moves) = parse_input(&input);

    ship.run_moves_part2(&moves);

    println!("Top containers of the ship part2: {}", ship.top());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        let input = "    [D]    \n\
                     [N] [C]    \n\
                     [Z] [M] [P]\n\
                      1   2   3 \n\
                     \n\
                     move 1 from 2 to 1\n\
                     move 3 from 1 to 3\n\
                     move 2 from 2 to 1\n\
                     move 1 from 1 to 2"
            .to_string();

        let (mut ship, moves) = parse_input(&input);

        assert_eq!("NDP", ship.top());

        ship.run_moves(&moves);

        assert_eq!("CMZ", ship.top());

        let (mut ship, moves) = parse_input(&input);

        ship.run_moves_part2(&moves);
        assert_eq!("MCD", ship.top());
    }
}
