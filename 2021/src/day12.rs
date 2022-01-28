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

fn find_path(
    caves: &HashMap<String, Cave>,
    already_visited: Vec<String>,
    name: String,
    twice_visit: String,
    twice_visited: i32,
) -> Option<HashSet<Vec<String>>> {
    if name == "start" && already_visited.len() != 0 {
        None
    } else if name == "end" {
        let mut cloned_array = already_visited.clone();
        cloned_array.push("end".to_string());
        Some(vec![cloned_array].iter().cloned().collect())
    } else if name.chars().collect::<Vec<char>>()[0].is_lowercase()
        && already_visited.contains(&name)
        && (name != twice_visit || twice_visited >= 1)
    {
        None
    } else {
        let cave = match caves.get(&name) {
            Some(cave) => cave,
            None => panic!("Invalid Cave Name"),
        };

        let twice_visited = if name == twice_visit {
            twice_visited + 1
        } else {
            twice_visited
        };

        let mut routes: HashSet<Vec<String>> = HashSet::new();

        for adjcent_cave in cave.adjcent.iter() {
            let mut cloned_path = already_visited.clone();
            cloned_path.push(name.clone());

            if twice_visit == "" && name.chars().collect::<Vec<char>>()[0].is_lowercase() {
                let cloned_path = cloned_path.clone();
                if let Some(paths) = find_path(
                    caves,
                    cloned_path,
                    adjcent_cave.to_string(),
                    name.clone(),
                    twice_visited,
                ) {
                    for path in paths.iter() {
                        routes.insert((*path).clone());
                    }
                }
            }
            if let Some(paths) = find_path(
                caves,
                cloned_path,
                adjcent_cave.to_string(),
                twice_visit.clone(),
                twice_visited,
            ) {
                for path in paths.iter() {
                    routes.insert((*path).clone());
                }
            }
        }

        Some(routes)
    }
}

fn find_paths(caves: &HashMap<String, Cave>, twice_visit: bool) -> HashSet<Vec<String>> {
    if twice_visit {
        match find_path(caves, Vec::new(), "start".to_string(), "".to_string(), 0) {
            Some(v) => v,
            None => HashSet::new(),
        }
    } else {
        match find_path(
            caves,
            Vec::new(),
            "start".to_string(),
            "start".to_string(),
            2,
        ) {
            Some(v) => v,
            None => HashSet::new(),
        }
    }
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

            // Creates the left cave if not there
            let cave = caves.entry(edge.1.to_string()).or_insert(Cave {
                name: edge.1.to_string(),
                is_small: edge.1.chars().collect::<Vec<char>>()[0].is_lowercase(),
                adjcent: HashSet::new(),
            });

            (*cave).adjcent.insert(edge.0.to_string());
        });

    for (_, cave) in caves.iter() {
        println!("Cave name: {}", cave.name);
        println!("is_small_cave: {}", cave.is_small);
        print!("adjcent:");
        for adjcent in cave.adjcent.iter() {
            print!("{},", adjcent);
        }
        println!("");
    }

    let paths = find_paths(&caves, false);

    for path in &paths {
        for cave in path {
            print!("{}->", cave)
        }
        println!("");
    }

    println!("Part 1 {}", paths.len());

    let paths = find_paths(&caves, true);

    for path in &paths {
        for cave in path {
            print!("{}->", cave)
        }
        println!("");
    }

    println!("Part 2 {}", paths.len());
}
