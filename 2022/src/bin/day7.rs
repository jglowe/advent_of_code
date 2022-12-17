///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _            ______
//                              | |          |____  |
//                            __| | __ _ _   _   / /
//                           / _` |/ _` | | | | / /
//                          | (_| | (_| | |_| |/ /
//                           \__,_|\__,_|\__, /_/
//                                        __/ |
//                                       |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day7 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

fn parse(input: &str) -> HashMap<String, usize> {
    let mut path: Vec<&str> = Vec::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    for line in input.split("\n") {
        if line.len() == 0 {
            continue
        }
        if line.chars().nth(0).unwrap() == '$' {
            if &line[2..4] == "cd" {
                if &line[5..] == ".." {
                    path.pop();
                } else {
                    path.push(&line[5..])
                }
            }
        } else {
            let line_parts: Vec<&str> = line.split(" ").collect();

            assert!(line_parts.len() == 2, "Lines must have 2 parts");

            if line_parts[0] != "dir" {
                let mut p = String::new();
                for part in &path {
                    p.push_str(part);
                    dir_sizes
                        .entry(p.clone())
                        .and_modify(|x| *x += line_parts[0].parse::<usize>().unwrap())
                        .or_insert(line_parts[0].parse().unwrap());
                }

            }
        }
    }

    dir_sizes
}

fn find_sum_under_limit(dir_sizes: &HashMap<String, usize>) -> usize {
    let mut size = 0;

    for value in dir_sizes.values() {
        if *value <= 100000 {
            size += value;
        }
    }

    size
}

fn find_smallest_to_delete(dir_sizes: &HashMap<String, usize>) -> usize {
    let total_disk_space = 70000000;
    let total_used = dir_sizes.get("/").unwrap();
    let free = total_disk_space - total_used;
    let needed = 30000000 - free;

    let mut min_needed = total_disk_space;

    for value in dir_sizes.values() {
        if *value >= needed && *value < min_needed {
            min_needed = *value;
        }
    }

    min_needed
}

fn main() {
    let input = advent::read_file("data/day7_input.txt");

    let file_system = parse(&input);
    println!("{}", find_sum_under_limit(&file_system));
    println!("{}", find_smallest_to_delete(&file_system));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        let input = "$ cd /\n\
                     $ ls\n\
                     dir a\n\
                     14848514 b.txt\n\
                     8504156 c.dat\n\
                     dir d\n\
                     $ cd a\n\
                     $ ls\n\
                     dir e\n\
                     29116 f\n\
                     2557 g\n\
                     62596 h.lst\n\
                     $ cd e\n\
                     $ ls\n\
                     584 i\n\
                     $ cd ..\n\
                     $ cd ..\n\
                     $ cd d\n\
                     $ ls\n\
                     4060174 j\n\
                     8033020 d.log\n\
                     5626152 d.ext\n\
                     7214296 k"
            .to_string();

        let file_system = parse(&input);
        assert_eq!(95437, find_sum_under_limit(&file_system));
        assert_eq!(24933642, find_smallest_to_delete(&file_system));
    }
}
