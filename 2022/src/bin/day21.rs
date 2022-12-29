///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                        _             ___  __
//                                       | |           |__ \/_ |
//                                     __| | __ _ _   _   ) || |
//                                    / _` |/ _` | | | | / / | |
//                                   | (_| | (_| | |_| |/ /_ | |
//                                    \__,_|\__,_|\__, |____||_|
//                                                 __/ |
//                                                |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day21 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

#[derive(Clone)]
enum Job {
    Number(i64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

fn parse_input(input: &str) -> HashMap<String, Job> {
    let mut jobs = HashMap::new();

    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        assert!(parts.len() == 2, "Wrong number of parts");
        let monkey = parts[0].to_string();

        let parts: Vec<&str> = parts[1].split(" ").collect();

        if parts.len() == 1 {
            jobs.insert(monkey, Job::Number(parts[0].parse::<i64>().unwrap()));
        } else if parts.len() == 3 {
            match parts[1] {
                "+" => jobs.insert(monkey, Job::Add(parts[0].to_string(), parts[2].to_string())),
                "-" => jobs.insert(
                    monkey,
                    Job::Subtract(parts[0].to_string(), parts[2].to_string()),
                ),
                "*" => jobs.insert(
                    monkey,
                    Job::Multiply(parts[0].to_string(), parts[2].to_string()),
                ),
                "/" => jobs.insert(
                    monkey,
                    Job::Divide(parts[0].to_string(), parts[2].to_string()),
                ),
                _ => panic!("Invalid operation"),
            };
        } else {
            panic!("Wrong number of parts");
        }
    }

    jobs
}

fn resolve(name: &String, jobs: &HashMap<String, Job>) -> i64 {
    match jobs.get(name).unwrap() {
        Job::Number(n) => *n,
        Job::Add(x, y) => resolve(x, &jobs) + resolve(y, &jobs),
        Job::Subtract(x, y) => resolve(x, &jobs) - resolve(y, &jobs),
        Job::Multiply(x, y) => resolve(x, &jobs) * resolve(y, &jobs),
        Job::Divide(x, y) => resolve(x, &jobs) / resolve(y, &jobs),
    }
}

fn human_found(name: &String, jobs: &HashMap<String, Job>) -> bool {
    if name == "humn" {
        return true;
    }
    match jobs.get(name).unwrap() {
        Job::Number(_) => false,
        Job::Add(x, y) => human_found(x, &jobs) || human_found(y, &jobs),
        Job::Subtract(x, y) => human_found(x, &jobs) || human_found(y, &jobs),
        Job::Multiply(x, y) => human_found(x, &jobs) || human_found(y, &jobs),
        Job::Divide(x, y) => human_found(x, &jobs) || human_found(y, &jobs),
    }
}

fn part1(jobs: &HashMap<String, Job>) -> i64 {
    resolve(&"root".to_string(), &jobs)
}

fn part2(jobs: &HashMap<String, Job>) -> i64 {
    let (root_left, root_right) = match jobs.get("root").unwrap() {
        Job::Number(_) => panic!("Root shouldn't be just a number"),
        Job::Add(x, y) => (x, y),
        Job::Subtract(x, y) => (x, y),
        Job::Multiply(x, y) => (x, y),
        Job::Divide(x, y) => (x, y),
    };
    let mut eq = if human_found(root_left, &jobs) {
        resolve(root_right, jobs)
    } else {
        resolve(root_left, jobs)
    };
    let mut current_monkey = if human_found(root_left, jobs) {
        root_left
    } else {
        root_right
    };

    while current_monkey != "humn" {
        match jobs.get(current_monkey).unwrap() {
            Job::Number(_) => panic!("Shouldn't ever reach this"),
            Job::Add(x, y) => {
                if human_found(x, jobs) {
                    current_monkey = x;
                    eq = eq - resolve(y, jobs);
                } else {
                    current_monkey = y;
                    eq = eq - resolve(x, jobs);
                }
            }
            Job::Subtract(x, y) => {
                if human_found(x, jobs) {
                    current_monkey = x;
                    eq = eq + resolve(y, jobs);
                } else {
                    current_monkey = y;
                    eq = resolve(x, jobs) - eq;
                }
            }
            Job::Multiply(x, y) => {
                if human_found(x, jobs) {
                    current_monkey = x;
                    eq = eq / resolve(y, jobs);
                } else {
                    current_monkey = y;
                    eq = eq / resolve(x, jobs);
                }
            }
            Job::Divide(x, y) => {
                if human_found(x, jobs) {
                    current_monkey = x;
                    eq = eq * resolve(y, jobs);
                } else {
                    current_monkey = y;
                    eq = resolve(x, jobs) / eq;
                }
            }
        };
    }

    eq
}

fn main() {
    let input = advent::read_file("data/day21_input.txt");
    let jobs = parse_input(&input);
    println!("Part 1 : {}", part1(&jobs));
    println!("Part 2 : {}", part2(&jobs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "root: pppw + sjmn\n\
                     dbpl: 5\n\
                     cczh: sllz + lgvd\n\
                     zczc: 2\n\
                     ptdq: humn - dvpt\n\
                     dvpt: 3\n\
                     lfqf: 4\n\
                     humn: 5\n\
                     ljgn: 2\n\
                     sjmn: drzm * dbpl\n\
                     sllz: 4\n\
                     pppw: cczh / lfqf\n\
                     lgvd: ljgn * ptdq\n\
                     drzm: hmdt - zczc\n\
                     hmdt: 32\n";
        let jobs = parse_input(&input);

        assert_eq!(152, part1(&jobs));
    }

    #[test]
    fn part2_test() {
        let input = "root: pppw + sjmn\n\
                     dbpl: 5\n\
                     cczh: sllz + lgvd\n\
                     zczc: 2\n\
                     ptdq: humn - dvpt\n\
                     dvpt: 3\n\
                     lfqf: 4\n\
                     humn: 5\n\
                     ljgn: 2\n\
                     sjmn: drzm * dbpl\n\
                     sllz: 4\n\
                     pppw: cczh / lfqf\n\
                     lgvd: ljgn * ptdq\n\
                     drzm: hmdt - zczc\n\
                     hmdt: 32\n";
        let jobs = parse_input(&input);

        assert_eq!(301, part2(&jobs));
    }
}
