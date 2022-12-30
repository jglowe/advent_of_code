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
// The file for day19 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::{BinaryHeap, HashSet};

use regex::Regex;

struct Blueprint {
    id: u8,
    ore_cost: u8,
    clay_cost: u8,
    obsidian_cost: (u8, u8),
    geode_cost: (u8, u8),
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
                id: captures.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                ore_cost: captures.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                clay_cost: captures.get(3).unwrap().as_str().parse::<u8>().unwrap(),
                obsidian_cost: (
                    captures.get(4).unwrap().as_str().parse::<u8>().unwrap(),
                    captures.get(5).unwrap().as_str().parse::<u8>().unwrap(),
                ),
                geode_cost: (
                    captures.get(6).unwrap().as_str().parse::<u8>().unwrap(),
                    captures.get(7).unwrap().as_str().parse::<u8>().unwrap(),
                ),
            }
        })
        .collect()
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    current_ore: u8,
    current_clay: u8,
    current_obsidian: u8,
    current_geodes: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
}

fn find_max_geodes(blueprint: &Blueprint, time: u8) -> usize {
    let mut max_geodes_collected = 0;
    let mut queue = BinaryHeap::new();
    let mut already_there = HashSet::new();

    let max_ore_cost = *vec![
        blueprint.ore_cost,
        blueprint.clay_cost,
        blueprint.obsidian_cost.0,
        blueprint.geode_cost.0,
    ]
    .iter()
    .max()
    .unwrap();

    queue.push((
        time,
        State {
            current_ore: 0,
            current_clay: 0,
            current_obsidian: 0,
            current_geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        },
    ));

    while let Some((time_left, state)) = queue.pop() {
        if time_left == 0 {
            max_geodes_collected = max_geodes_collected.max(state.current_geodes);
            continue;
        }

        let theoretical_possible_max =
            state.current_geodes + state.geode_robots * time_left + (1..time_left).sum::<u8>();

        if theoretical_possible_max < max_geodes_collected {
            continue;
        }

        if already_there.contains(&state) {
            continue;
        }

        already_there.insert(state.clone());
        if state.ore_robots >= blueprint.geode_cost.0
            && state.obsidian_robots >= blueprint.geode_cost.1
        {
            max_geodes_collected = max_geodes_collected.max(theoretical_possible_max);
            continue;
        }

        // If you have enough robots to build a geode robot every time, calculate the end cost
        //     max_geodes_collected = max_geodes_collected.max(theoretical_possible_max);
        //     continue;
        // }

        // Only one robot can be built at a time and if you can build a geode robot, build it.
        if state.current_ore >= blueprint.geode_cost.0
            && state.current_obsidian >= blueprint.geode_cost.1
        {
            let mut state1 = state.clone();

            state1.current_ore += state1.ore_robots;
            state1.current_clay += state1.clay_robots;
            state1.current_obsidian += state1.obsidian_robots;
            state1.current_geodes += state1.geode_robots;
            state1.geode_robots += 1;
            state1.current_ore -= blueprint.geode_cost.0;
            state1.current_obsidian -= blueprint.geode_cost.1;
            queue.push((time_left - 1, state1));
        }

        // Ore is used for building all robots. If you don't build a robot in 5 minutes, trim path.
        if state.current_ore > (state.ore_robots * 5).max(max_ore_cost) {
            continue;
        }

        // Obsidian Robot
        if state.current_ore >= blueprint.obsidian_cost.0
            && state.current_clay >= blueprint.obsidian_cost.1
            && state.obsidian_robots < blueprint.geode_cost.1
        {
            let mut state = state.clone();

            state.current_ore += state.ore_robots;
            state.current_clay += state.clay_robots;
            state.current_obsidian += state.obsidian_robots;
            state.current_geodes += state.geode_robots;
            state.obsidian_robots += 1;
            state.current_ore -= blueprint.obsidian_cost.0;
            state.current_clay -= blueprint.obsidian_cost.1;
            queue.push((time_left - 1, state));
        }

        // Ore Robot
        if state.current_ore >= blueprint.ore_cost && state.ore_robots < max_ore_cost {
            let mut state = state.clone();

            state.current_ore += state.ore_robots;
            state.current_clay += state.clay_robots;
            state.current_obsidian += state.obsidian_robots;
            state.current_geodes += state.geode_robots;
            state.ore_robots += 1;
            state.current_ore -= blueprint.ore_cost;
            queue.push((time_left - 1, state));
        }

        // Clay robot
        if state.current_ore >= blueprint.clay_cost && state.clay_robots < blueprint.obsidian_cost.1
        {
            let mut state = state.clone();

            state.current_ore += state.ore_robots;
            state.current_clay += state.clay_robots;
            state.current_obsidian += state.obsidian_robots;
            state.current_geodes += state.geode_robots;
            state.clay_robots += 1;
            state.current_ore -= blueprint.clay_cost;
            queue.push((time_left - 1, state));
        }

        // If you can build all robots, don't wait
        if state.current_ore >= max_ore_cost
            && state.current_clay >= blueprint.obsidian_cost.1
            && state.current_obsidian >= blueprint.geode_cost.1
        {
            continue;
        }

        let mut state = state.clone();
        state.current_ore += state.ore_robots;
        state.current_clay += state.clay_robots;
        state.current_obsidian += state.obsidian_robots;
        state.current_geodes += state.geode_robots;
        queue.push((time_left - 1, state));
    }

    max_geodes_collected as usize
}

fn part1(blueprints: &Vec<Blueprint>, time: u8) -> usize {
    blueprints
        .iter()
        .map(|blueprint| find_max_geodes(blueprint, time) * (blueprint.id as usize))
        .sum()
}

fn part2(blueprints: &Vec<Blueprint>, time: u8) -> usize {
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
        assert_eq!(33, part1(&blueprints, 24));
    }

    #[test]
    fn part2_test() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\n\
                     Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.\n";
        let blueprints = parse_input(&input);
        assert_eq!(56, find_max_geodes(&blueprints[0], 32));
        assert_eq!(62, find_max_geodes(&blueprints[1], 32));
    }
}
