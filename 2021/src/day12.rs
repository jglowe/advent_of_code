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
use std::rc::Rc;

struct Path {
    value: String,
    next: Option<Rc<Path>>,
}

impl Path {
    fn new(value: String) -> Path {
        Path { value, next: None }
    }

    fn push(path: Rc<Path>, value: String) -> Path {
        Path {
            value,
            next: Some(path),
        }
    }

    fn contains(&self, value: &String) -> bool {
        if self.value == *value {
            return true;
        }

        let mut current_node = &self.next;
        while let Some(next_node) = current_node {
            if next_node.value == *value {
                return true;
            }

            current_node = &next_node.next;
        }

        return false;
    }

    fn to_string(&self) -> String {
        let mut string = "".to_string();

        match self.next {
            Some(_) => {
                string.push_str(&self.value);
                string.push_str("<-")
            }
            None => string.push_str(&self.value),
        }

        let mut current_node = &self.next;
        while let Some(node) = current_node {
            match node.next {
                Some(_) => {
                    string.push_str(&node.value);
                    string.push_str("<-")
                }
                None => string.push_str(&node.value),
            }

            current_node = &node.next;
        }

        return string;
    }
}

struct Cave {
    name: String,
    is_small: bool,
    adjcent: HashSet<String>,
}

fn find_path(
    cave: &Cave,
    caves: &HashMap<String, Cave>,
    already_visited: Rc<Path>,
    twice_visit: String,
    visited_times: i32,
    max_visit_count: i32,
) -> Option<Vec<Path>> {
    if cave.name == "start" {
        return None;
    }

    if cave.name == "end" {
        let path = Path::push(already_visited, "end".to_string());
        return Some(vec![path]);
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

    let mut routes: Vec<Path> = Vec::new();

    for adjcent_cave_name in cave.adjcent.iter() {
        let adjcent_cave = caves.get(adjcent_cave_name).unwrap();
        let path = Rc::new(Path::push(Rc::clone(&already_visited), cave.name.clone()));

        if twice_visit == "" && adjcent_cave.is_small {
            if let Some(mut paths) = find_path(
                adjcent_cave,
                caves,
                Rc::clone(&path),
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
            path,
            twice_visit.clone(),
            visited_times,
            max_visit_count,
        ) {
            routes.append(&mut paths);
        }
    }

    Some(routes)
}

fn find_paths(caves: &HashMap<String, Cave>, max_visits: i32) -> HashSet<String> {
    let start_cave = caves.get("start").unwrap();

    let mut results = HashSet::new();

    for cave_name in start_cave.adjcent.iter() {
        let cave = caves.get(cave_name).unwrap();
        let paths = match find_path(
            cave,
            caves,
            Rc::new(Path::new("start".to_string())),
            "".to_string(),
            0,
            max_visits,
        ) {
            Some(v) => v,
            None => Vec::new(),
        };

        for path in paths.into_iter() {
            results.insert(path.to_string());
        }
    }

    results
}

fn parse_graph(input: &str) -> HashMap<String, Cave> {
    let mut caves: HashMap<String, Cave> = HashMap::new();
    println!("{}", input);
    input
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
    caves
}

fn main() {
    let filename = "data/day12_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let caves = parse_graph(&file_contents);

    let paths = find_paths(&caves, 1);

    // for path in paths.iter() {
    //     println!("{}", path);
    // }

    println!("Part 1 {}", paths.len());

    let paths = find_paths(&caves, 2);

    // for path in paths.iter() {
    //     println!("{}", path);
    // }

    println!("Part 2 {}", paths.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "start-A\n\
                     start-b\n\
                     A-c\n\
                     A-b\n\
                     b-d\n\
                     A-end\n\
                     b-end";

        let caves = parse_graph(input);

        let paths = find_paths(&caves, 1);

        assert_eq!(10, paths.len());

        let paths = find_paths(&caves, 2);

        assert_eq!(36, paths.len());
    }

}
