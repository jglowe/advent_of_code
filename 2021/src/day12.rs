///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _            __ ___
//                              | |          /_ |__ \
//                            __| | __ _ _   _| |  ) |
//                           / _` |/ _` | | | | | / /
//                          | (_| | (_| | |_| | |/ /_
//                           \__,_|\__,_|\__, |_|____|
//                                        __/ |
//                                       |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day12 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

struct Cave {
    name: String,
    is_small: bool,
    adjcent: HashSet<String>,
}

// struct Visits {
//     max_count: i32,
//     max_count_item: Option<String>,
//     counts: HashMap<String, i32>,
// }

fn find_path(
    cave: &Cave,
    caves: &HashMap<String, Cave>,
    mut already_visited: Vec<String>,
    twice_visit: String,
    visited_times: i32,
    max_visit_count: i32,
) -> Option<Vec<Vec<String>>> {
    if cave.name == "start" {
        return None;
    }

    if cave.name == "end" {
        already_visited.push("end".to_string());
        return Some(vec![already_visited]);
    }

    if cave.is_small
        && already_visited.contains(&cave.name)
        && (cave.name != twice_visit || visited_times >= max_visit_count - 1)
    {
        return None;
    }

    let visited_times = if cave.name == twice_visit {
        visited_times + 1
    } else {
        visited_times
    };

    let mut routes: Vec<Vec<String>> = Vec::new();

    for adjcent_cave_name in cave.adjcent.iter() {
        let adjcent_cave = caves.get(adjcent_cave_name).unwrap();
        let mut cloned_path = already_visited.clone();
        cloned_path.push(cave.name.clone());

        if twice_visit == "" && adjcent_cave.is_small {
            let cloned_path = cloned_path.clone();
            if let Some(mut paths) = find_path(
                adjcent_cave,
                caves,
                cloned_path,
                (*adjcent_cave.name).to_string(),
                visited_times,
                max_visit_count,
            ) {
                routes.append(&mut paths);
            }
        }
        if let Some(mut paths) = find_path(
            adjcent_cave,
            caves,
            cloned_path,
            twice_visit.clone(),
            visited_times,
            max_visit_count,
        ) {
            routes.append(&mut paths);
        }
    }

    Some(routes)
}

fn find_paths(caves: &HashMap<String, Cave>, max_visits: i32) -> HashSet<Vec<String>> {
    let start_cave = caves.get("start").unwrap();

    let mut results = HashSet::new();

    for cave_name in start_cave.adjcent.iter() {
        let cave = caves.get(cave_name).unwrap();
        let paths = match find_path(
            cave,
            caves,
            vec!["start".to_string()],
            "".to_string(),
            0,
            max_visits,
        ) {
            Some(v) => v,
            None => Vec::new(),
        };

        for path in paths.into_iter() {
            results.insert(path);
        }
    }

    results
}

fn main() {
    let filename = "data/day12_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };
    let mut caves: HashMap<String, Cave> = HashMap::new();
    file_contents
        .trim()
        .split("\n")
        .map(|line| line.split_once("-").unwrap())
        .for_each(|edge| {
            // Creates the left cave if not there
            let cave = caves.entry(edge.0.to_string()).or_insert(Cave {
                name: edge.0.to_string(),
                is_small: edge.0.chars().collect::<Vec<char>>()[0].is_lowercase(),
                adjcent: HashSet::new(),
            });

            (*cave).adjcent.insert(edge.1.to_string());

            // Creates the right cave if not there
            let cave = caves.entry(edge.1.to_string()).or_insert(Cave {
                name: edge.1.to_string(),
                is_small: edge.1.chars().collect::<Vec<char>>()[0].is_lowercase(),
                adjcent: HashSet::new(),
            });

            (*cave).adjcent.insert(edge.0.to_string());
        });

    let paths = find_paths(&caves, 1);

    for path in paths.iter() {
        println!("{}", path.join(","));
    }

    println!("Part 1 {}", paths.len());

    let paths = find_paths(&caves, 2);

    for path in paths.iter() {
        println!("{}", path.join(","));
    }

    println!("Part 2 {}", paths.len());
}
