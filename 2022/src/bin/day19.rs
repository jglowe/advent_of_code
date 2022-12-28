///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                        _            __  ___
//                                       | |          /_ |/ _ \
//                                     __| | __ _ _   _| | (_) |
//                                    / _` |/ _` | | | | |\__, |
//                                   | (_| | (_| | |_| | |  / /
//                                    \__,_|\__,_|\__, |_| /_/
//                                                 __/ |
//                                                |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day18 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::{HashSet, VecDeque};

use regex::Regex;

struct Blueprint {
    id: usize,
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost: (usize, usize),
    geode_cost: (usize, usize),
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let regex = Regex::new(
            r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."
        )
        .unwrap();

    input
        .trim()
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            Blueprint {
                id: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                ore_cost: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                clay_cost: captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                obsidian_cost: (
                    captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                    captures.get(5).unwrap().as_str().parse::<usize>().unwrap(),
                ),
                geode_cost: (
                    captures.get(6).unwrap().as_str().parse::<usize>().unwrap(),
                    captures.get(7).unwrap().as_str().parse::<usize>().unwrap(),
                ),
            }
        })
        .collect()
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    current_ore: usize,
    current_clay: usize,
    current_obsidian: usize,
    current_geodes: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    time_left: usize,
}

fn find_max_geodes(blueprint: &Blueprint, time: usize) -> usize {
    let mut max_geodes_collected = 0;
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut already_there: HashSet<State> = HashSet::new();

    queue.push_back(State {
        current_ore: 0,
        current_clay: 0,
        current_obsidian: 0,
        current_geodes: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
        time_left: time,
    });

    while let Some(state) = queue.pop_back() {
        if state.time_left == 0 {
            max_geodes_collected = max_geodes_collected.max(state.current_geodes);
            continue;
        }

        if already_there.contains(&state) {
            continue;
        } else {
            already_there.insert(state.clone());
        }

        // Only one robot can be built at a time and if you can build a geode robot, build it.
        if state.current_ore >= blueprint.geode_cost.0
            && state.current_obsidian >= blueprint.geode_cost.1
        {
            let mut state = state.clone();

            state.current_ore += state.ore_robots;
            state.current_clay += state.clay_robots;
            state.current_obsidian += state.obsidian_robots;
            state.current_geodes += state.geode_robots;
            state.geode_robots += 1;
            state.current_ore -= blueprint.geode_cost.0;
            state.current_obsidian -= blueprint.geode_cost.1;
            state.time_left -= 1;
            queue.push_back(state);
            continue;
        }

        let max_ore_cost = *vec![
            blueprint.ore_cost,
            blueprint.clay_cost,
            blueprint.obsidian_cost.0,
            blueprint.geode_cost.0,
        ].iter().max().unwrap();

        // Ore is used for building all robots. If you don't build a robot in 5 minutes, trim path.
        if state.current_ore > (state.ore_robots * 5).max(max_ore_cost) {
            continue;
        }

        // Knowing you can only build one robot at time, you only need at most the cost of building
        // a single robot.
        if state.ore_robots > max_ore_cost {
            continue;
        }

        // Knowing you can only build one robot at time, you only need at most the cost of building
        // a single robot.
        if state.clay_robots > blueprint.obsidian_cost.1 {
            continue;
        }

        // Knowing you can only build one robot at time, you only need at most the cost of building
        // a single robot.
        if state.obsidian_robots > blueprint.geode_cost.1 {
            continue;
        }

        if state.current_ore >= blueprint.obsidian_cost.0
            && state.current_clay >= blueprint.obsidian_cost.1
        {
            let mut state = state.clone();

            state.current_ore += state.ore_robots;
            state.current_clay += state.clay_robots;
            state.current_obsidian += state.obsidian_robots;
            state.current_geodes += state.geode_robots;
            state.obsidian_robots += 1;
            state.current_ore -= blueprint.obsidian_cost.0;
            state.current_clay -= blueprint.obsidian_cost.1;
            state.time_left -= 1;
            queue.push_back(state);
        }

        if state.current_ore >= blueprint.ore_cost {
            let mut state = state.clone();

            state.current_ore += state.ore_robots;
            state.current_clay += state.clay_robots;
            state.current_obsidian += state.obsidian_robots;
            state.current_geodes += state.geode_robots;
            state.ore_robots += 1;
            state.current_ore -= blueprint.ore_cost;
            state.time_left -= 1;
            queue.push_back(state);
        }

        if state.current_ore >= blueprint.clay_cost {
            let mut state = state.clone();

            state.current_ore += state.ore_robots;
            state.current_clay += state.clay_robots;
            state.current_obsidian += state.obsidian_robots;
            state.current_geodes += state.geode_robots;
            state.clay_robots += 1;
            state.current_ore -= blueprint.clay_cost;
            state.time_left -= 1;
            queue.push_back(state);
        }

        let mut state = state.clone();
        state.current_ore += state.ore_robots;
        state.current_clay += state.clay_robots;
        state.current_obsidian += state.obsidian_robots;
        state.current_geodes += state.geode_robots;
        state.time_left -= 1;
        queue.push_back(state);
    }

    max_geodes_collected
}

fn part1(blueprints: &Vec<Blueprint>, time: usize) -> usize {
    blueprints
        .iter()
        .map(|blueprint| find_max_geodes(blueprint, time) * blueprint.id)
        .sum()
}

fn part2(blueprints: &Vec<Blueprint>, time: usize) -> usize {
    let first = find_max_geodes(&blueprints[0], time);
    let second = find_max_geodes(&blueprints[1], time);
    let third = find_max_geodes(&blueprints[2], time);
    first * second * third
}

fn main() {
    let input = advent::read_file("data/day19_input.txt");
    let blueprints = parse_input(&input);
    println!("Part1 : {}", part1(&blueprints, 24));
    println!("Part2 : {}", part2(&blueprints, 32));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\n\
                     Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.\n";
        let blueprints = parse_input(&input);
        assert_eq!(9, find_max_geodes(&blueprints[0], 24));
        assert_eq!(12, find_max_geodes(&blueprints[1], 24));
        assert_eq!(33, part1(&blueprints, 24));
    }

    #[test]
    fn part2_test() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\n\
                     Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.\n";
        let blueprints = parse_input(&input);
        assert_eq!(56, find_max_geodes(&blueprints[0], 32));
        assert_eq!(62, find_max_geodes(&blueprints[1], 32));
        assert_eq!(56 * 62, part2(&blueprints, 32));
    }
}
