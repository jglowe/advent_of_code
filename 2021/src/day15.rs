///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                        _            __ _____
//                                       | |          /_ | ____|
//                                     __| | __ _ _   _| | |__
//                                    / _` |/ _` | | | | |___ \
//                                   | (_| | (_| | |_| | |___) |
//                                    \__,_|\__,_|\__, |_|____/
//                                                 __/ |
//                                                |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day15 advent of code 2021
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs;

struct Node {
    risk_level: u32,
    distance: Option<u32>,
    path: Option<(usize, usize)>,
    been_visited: bool,
}

fn parse_input(input: String) -> Vec<Vec<Node>> {
    let mut result = Vec::new();

    for line in input.trim().split("\n") {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(Node {
                risk_level: c.to_digit(10).unwrap(),
                distance: None,
                been_visited: false,
                path: None,
            });
        }

        result.push(row);
    }

    result
}

fn wrap_risk(left: u32, right: u32) -> u32 {
    if left + right > 9 {
        left + right - 9
    } else {
        left + right
    }
}

fn extend_graph(graph: &mut Vec<Vec<Node>>) {
    let number_of_rows = graph.len();
    let number_of_columns = graph[0].len();
    // Extend to the right
    for row in graph.iter_mut() {
        for i in 1..5 {
            for j in 0..number_of_columns {
                row.push(Node {
                    risk_level: wrap_risk(row[j].risk_level, i),
                    distance: None,
                    path: None,
                    been_visited: false,
                })
            }
        }
    }

    // Extend down
    for i in 1..5 {
        for j in 0..number_of_rows {
            let mut row = Vec::new();
            for column in graph[j].iter() {
                row.push(Node {
                    risk_level: wrap_risk(column.risk_level, i),
                    distance: None,
                    path: None,
                    been_visited: false,
                })
            }
            graph.push(row);
        }
    }
}

fn find_shorted_path_length(graph: &mut Vec<Vec<Node>>) -> u32 {
    graph[0][0].distance = Some(0);
    graph[0][0].path = Some((0, 0));

    let mut unknown_nodes = BinaryHeap::new();
    let mut visited = HashSet::new();

    unknown_nodes.push(Reverse((0, (0, 0))));

    while let Some(Reverse((_, (x, y)))) = unknown_nodes.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));
        graph[x][y].been_visited = true;
        let dist = graph[x][y].distance.unwrap();

        let mut adjcent = Vec::new();
        if x != 0 {
            adjcent.push((x - 1, y));
        }
        if y != 0 {
            adjcent.push((x, y - 1));
        }
        if x + 1 < graph.len() {
            adjcent.push((x + 1, y));
        }
        if y + 1 < graph[0].len() {
            adjcent.push((x, y + 1));
        }

        for (i, j) in adjcent.iter() {
            if !graph[*i][*j].been_visited {
                match graph[*i][*j].distance {
                    Some(d) => {
                        if dist + graph[*i][*j].risk_level < d {
                            graph[*i][*j].distance = Some(dist + graph[*i][*j].risk_level);
                            graph[*i][*j].path = Some((x, y));
                        }
                    }
                    None => {
                        graph[*i][*j].distance = Some(dist + graph[*i][*j].risk_level);
                        graph[*i][*j].path = Some((x, y));
                    }
                }
                unknown_nodes.push(Reverse((graph[*i][*j].distance.unwrap(), (*i, *j))));
            }
        }
    }

    graph[graph.len() - 1][graph[0].len() - 1].distance.unwrap()
}

fn main() {
    let filename = "data/day15_input.txt";

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => panic!("Error {} {}", e, filename),
    };

    let mut graph = parse_input(file_contents.clone());
    let total_risk = find_shorted_path_length(&mut graph);
    println!("Part 1 : {}", total_risk);

    let mut graph = parse_input(file_contents);
    extend_graph(&mut graph);
    let total_risk = find_shorted_path_length(&mut graph);
    println!("Part 2 : {}", total_risk);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "1163751742\n\
                     1381373672\n\
                     2136511328\n\
                     3694931569\n\
                     7463417111\n\
                     1319128137\n\
                     1359912421\n\
                     3125421639\n\
                     1293138521\n\
                     2311944581\n";

        let mut graph = parse_input(input.to_string());

        assert_eq!(40, find_shorted_path_length(&mut graph));

        let mut graph = parse_input(input.to_string());
        extend_graph(&mut graph);

        assert_eq!(315, find_shorted_path_length(&mut graph));
    }
}
