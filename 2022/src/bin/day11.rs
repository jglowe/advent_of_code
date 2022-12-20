///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                         _            __ __
//                                        | |          /_ /_ |
//                                      __| | __ _ _   _| || |
//                                     / _` |/ _` | | | | || |
//                                    | (_| | (_| | |_| | || |
//                                     \__,_|\__,_|\__, |_||_|
//                                                  __/ |
//                                                 |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day11 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

enum Operation {
    Multiply(i128),
    Square,
    Add(i128),
}

struct Monkey {
    items: Vec<i128>,
    operation: Operation,
    divisible_test: i128,
    test_true: usize,
    test_false: usize,
    inspections: i128,
}

impl Monkey {
    fn parse(input: &mut dyn Iterator<Item = &str>) -> Monkey {
        input.next(); // First line has monkey number not needed as they are in order

        let items = input.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .split(", ")
            .map(|item| item.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        let operation_line = input.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .collect::<Vec<&str>>();

        let operation = match (operation_line[3], operation_line[4]) {
            ("+", x) => Operation::Add(x.parse::<i128>().unwrap()),
            ("*", "old") => Operation::Square,
            ("*", x) => Operation::Multiply(x.parse::<i128>().unwrap()),
            (_, _) => panic!("Badly formatted operation"),
        };

        let divisible_test = input
            .next()
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<i128>()
            .unwrap();

        let test_true = input
            .next()
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let test_false = input
            .next()
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        input.next(); // Empty line between the the next monkey

        Monkey {
            items,
            operation,
            divisible_test,
            test_true,
            test_false,
            inspections: 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut lines = input.trim().lines().peekable();
    let mut monkeys: Vec<Monkey> = Vec::new();

    while let Some(_) = lines.peek() {
        let monkey = Monkey::parse(&mut lines);
        monkeys.push(monkey);
    }

    monkeys
}

fn find_divisor(monkeys: &Vec<Monkey>)-> i128 {
    let mut divisor = 1;

    for monkey in monkeys {
        divisor *= monkey.divisible_test;
    }

    divisor
}

fn most_active(monkeys: &mut Vec<Monkey>) -> i128 {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                monkeys[i].inspections += 1;
                let item = match monkeys[i].operation {
                    Operation::Multiply(x) => item * x,
                    Operation::Square => item * item,
                    Operation::Add(x) => item + x,
                };

                let item = item / 3;

                if item % monkeys[i].divisible_test == 0 {
                    let true_test = monkeys[i].test_true;
                    monkeys[true_test].items.push(item);
                } else {
                    let false_test = monkeys[i].test_false;
                    monkeys[false_test].items.push(item);
                }
            }
        }
    }

    let mut inspections: Vec<i128> = Vec::new();

    for monkey in monkeys.iter() {
        inspections.push(monkey.inspections);
    }

    inspections.sort_by(|a, b| b.cmp(a));

    inspections[0] * inspections[1]
}

fn most_active_10k(monkeys: &mut Vec<Monkey>) -> i128 {
    let modulo = find_divisor(&monkeys);

    for _ in 0..10000{
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                monkeys[i].inspections += 1;
                let item = match monkeys[i].operation {
                    Operation::Multiply(x) => item * x,
                    Operation::Square => item * item,
                    Operation::Add(x) => item + x,
                };

                let item = item % modulo;

                if item % monkeys[i].divisible_test == 0 {
                    let true_test = monkeys[i].test_true;
                    monkeys[true_test].items.push(item);
                } else {
                    let false_test = monkeys[i].test_false;
                    monkeys[false_test].items.push(item);
                }
            }
        }
    }

    let mut inspections: Vec<i128> = Vec::new();

    for monkey in monkeys.iter() {
        inspections.push(monkey.inspections);
    }

    inspections.sort_by(|a, b| b.cmp(a));

    inspections[0] * inspections[1]
}

fn main() {
    let input = advent::read_file("data/day11_input.txt");
    let mut monkeys = parse_input(&input);

    println!("{}", most_active(&mut monkeys));

    let mut monkeys = parse_input(&input);
    println!("{}", most_active_10k(&mut monkeys));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        let mut monkeys = parse_input(&input);
        assert_eq!(10605, most_active(&mut monkeys));

        let mut monkeys = parse_input(&input);
        assert_eq!(2713310158, most_active_10k(&mut monkeys));
    }
}
