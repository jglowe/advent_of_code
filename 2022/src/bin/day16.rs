///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                        _            __   __
//                                       | |          /_ | / /
//                                     __| | __ _ _   _| |/ /_
//                                    / _` |/ _` | | | | | '_ \
//                                   | (_| | (_| | |_| | | (_) |
//                                    \__,_|\__,_|\__, |_|\___/
//                                                 __/ |
//                                                |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day16 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use itertools::Itertools;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct Room {
    name: String,
    adjcent: Vec<String>,
    rate: i64,
}

#[derive(PartialEq, Eq, Clone)]
struct State {
    visited: BTreeSet<String>,
    time_left: i64,
    pressure_relieved: i64,
    current: String,
}

type Map = HashMap<String, Room>;

fn parse_input(input: &str) -> (Map, Vec<String>) {
    let mut map = Map::new();
    let mut rooms_of_interest: Vec<String> = Vec::new();
    for line in input.trim().lines() {
        let regex = Regex::new(
            r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z ,]+)",
        )
        .unwrap();
        let captures = regex.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str().to_string();
        let rate = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let adjcent = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let room = Room {
            name: name.clone(),
            adjcent,
            rate,
        };

        if room.rate != 0 {
            rooms_of_interest.push(room.name.clone());
        }

        map.insert(name, room);
    }

    (map, rooms_of_interest)
}

fn compute_distance(map: &Map, start: &String, end: &String) -> i64 {
    let mut visited: HashSet<String> = HashSet::new();
    let mut distances: HashMap<String, i64> = HashMap::new();
    let mut to_process = BinaryHeap::new();

    visited.insert(start.clone());
    distances.insert(start.clone(), 0);
    for node in &map.get(start).unwrap().adjcent {
        to_process.push(Reverse((1, node.clone())));
    }

    while let Some(Reverse((distance, room))) = to_process.pop() {
        visited.insert(room.clone());
        distances.insert(room.clone(), distance);

        if room == *end {
            break;
        }

        for node in &map.get(&room).unwrap().adjcent {
            if !visited.contains(node) {
                to_process.push(Reverse((distance + 1, node.clone())));
            }
        }
    }

    *distances.get(end).unwrap()
}

fn get_distances(
    map: &Map,
    rooms_of_interest: &Vec<String>,
    starting_room: &String,
) -> HashMap<(String, String), i64> {
    let mut distances: HashMap<(String, String), i64> = HashMap::new();

    let mut rooms = rooms_of_interest.clone();

    rooms.push(starting_room.clone());

    while rooms.len() > 1 {
        let room = rooms.pop().unwrap();
        for r in &rooms {
            let min_distance = compute_distance(map, &r, &room);
            distances.insert((room.clone(), r.to_string()), min_distance);
            distances.insert((r.to_string(), room.clone()), min_distance);
        }
    }

    distances
}

fn traverse_graph(
    map: &Map,
    rooms_of_interest: &Vec<String>,
    starting_node: &String,
    time: i64,
) -> i64 {
    let distances: HashMap<(String, String), i64> =
        get_distances(map, rooms_of_interest, starting_node);
    let mut best_pressure_relief = 0;

    for room in rooms_of_interest {
        let mut visited: BTreeSet<String> = BTreeSet::new();
        visited.insert(room.clone());
        let time_left = time
            - distances
                .get(&(starting_node.clone(), room.clone()))
                .unwrap()
            - 1;
        let pressure_relieved = time_left * map.get(room).unwrap().rate;

        let mut queue: VecDeque<State> = VecDeque::new();

        queue.push_back(State {
            visited,
            time_left,
            pressure_relieved,
            current: room.clone(),
        });

        while let Some(state) = queue.pop_front() {
            let left: Vec<String> = rooms_of_interest
                .clone()
                .into_iter()
                .filter(|r| !state.visited.contains(r))
                .collect();

            if left.len() == 0 || state.time_left <= 0 {
                best_pressure_relief = best_pressure_relief.max(state.pressure_relieved);

                continue;
            }

            for room in left {
                let mut visited = state.visited.clone();
                visited.insert(room.clone());
                let time_left = state.time_left
                    - 1
                    - distances
                        .get(&(room.clone(), state.current.clone()))
                        .unwrap();
                let pressure_relieved = if time_left >= 0 {
                    state.pressure_relieved + map.get(&room).unwrap().rate * time_left
                } else {
                    state.pressure_relieved
                };
                queue.push_back(State {
                    visited,
                    current: room.clone(),
                    time_left,
                    pressure_relieved,
                });
            }
        }
    }

    best_pressure_relief
}

fn traverse_graph2(
    map: &Map,
    rooms_of_interest: &Vec<String>,
    starting_node: &String,
    time: i64,
) -> i64 {
    let distances: HashMap<(String, String), i64> =
        get_distances(map, rooms_of_interest, starting_node);

    let mut paths: HashMap<BTreeSet<String>, i64> = HashMap::new();
    for room in rooms_of_interest {
        let mut visited: BTreeSet<String> = BTreeSet::new();
        visited.insert(room.clone());
        let time_left = time
            - distances
                .get(&(starting_node.clone(), room.clone()))
                .unwrap()
            - 1;
        let pressure_relieved = time_left * map.get(room).unwrap().rate;

        let mut queue: VecDeque<State> = VecDeque::new();

        queue.push_back(State {
            visited,
            time_left,
            pressure_relieved,
            current: room.clone(),
        });

        while let Some(state) = queue.pop_front() {
            let left: Vec<String> = rooms_of_interest
                .clone()
                .into_iter()
                .filter(|r| !state.visited.contains(r))
                .collect();

            paths
                .entry(state.visited.clone())
                .and_modify(|s| *s = state.pressure_relieved.max(*s))
                .or_insert(state.pressure_relieved);

            if left.len() == 0 || state.time_left <= 0 {
                continue;
            }

            for room in left {
                let mut visited = state.visited.clone();
                visited.insert(room.clone());
                let time_left = state.time_left
                    - 1
                    - distances
                        .get(&(room.clone(), state.current.clone()))
                        .unwrap();
                let pressure_relieved = if time_left >= 0 {
                    state.pressure_relieved + map.get(&room).unwrap().rate * time_left
                } else {
                    state.pressure_relieved
                };
                queue.push_back(State {
                    visited,
                    current: room.clone(),
                    time_left,
                    pressure_relieved,
                });
            }
        }
    }

    println!("HERE");
    paths
        .iter()
        .tuple_combinations()
        .filter(|(h, e)| h.0.is_disjoint(e.0))
        .map(|(h, e)| h.1 + e.1)
        .max()
        .unwrap()
}

fn main() {
    let input = advent::read_file("data/day16_input.txt");

    let (map, rooms_of_interest) = parse_input(&input);
    println!(
        "{}",
        traverse_graph(&map, &rooms_of_interest, &"AA".to_string(), 30)
    );
    println!(
        "{}",
        traverse_graph2(&map, &rooms_of_interest, &"AA".to_string(), 26)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
                     Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
                     Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
                     Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
                     Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
                     Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
                     Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
                     Valve HH has flow rate=22; tunnel leads to valve GG\n\
                     Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
                     Valve JJ has flow rate=21; tunnel leads to valve II\n";

        let (map, rooms_of_interest) = parse_input(&input);
        assert_eq!(
            1651,
            traverse_graph(&map, &rooms_of_interest, &"AA".to_string(), 30)
        );
        assert_eq!(
            1707,
            traverse_graph2(&map, &rooms_of_interest, &"AA".to_string(), 26)
        );
    }
}
