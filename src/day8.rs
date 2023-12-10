use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Direction {
    Left,
    Right,
}

struct Graph {
    nodes: HashMap<String, Node>,
}

struct Node {
    left_key: String,
    right_key: String,
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

fn parse() -> (Graph, Vec<Direction>) {
    let mut graph = Graph {
        nodes: HashMap::new(),
    };
    let mut iter = lines_from_file("src/day8.txt").unwrap();
    let directions = parse_directions(&iter.next().unwrap());
    iter.next();
    iter.map(|line| parse_node(&line)).for_each(|(key, node)| {
        graph.nodes.insert(key, node);
    });

    (graph, directions)
}

fn parse_directions(direction_str: &str) -> Vec<Direction> {
    direction_str
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("AAAAAAAhhhh"),
        })
        .collect()
}

fn parse_node(node_str: &str) -> (String, Node) {
    let mut assignemt_split = node_str.split('=');
    let key = assignemt_split.next().unwrap().trim().to_owned();
    let mut lr_split = assignemt_split.next().unwrap().split(',');
    let left_key = lr_split.next().unwrap().trim()[1..].to_owned();
    let mut right = lr_split.next().unwrap().trim().chars();
    right.next_back();
    let right_key = right.as_str().to_owned();

    (
        key,
        Node {
            left_key,
            right_key,
        },
    )
}

pub fn task1() {
    let (graph, directions) = parse();

    let start: String = "AAA".to_owned();
    let end: String = "ZZZ".to_owned();
    let mut current_node = &start;
    let mut steps = 0;

    'outer: loop {
        for direction in &directions {
            steps += 1;
            match direction {
                Direction::Left => {
                    current_node = &graph.nodes.get(current_node).unwrap().left_key;
                }
                Direction::Right => {
                    current_node = &graph.nodes.get(current_node).unwrap().right_key;
                }
            };
            if current_node == &end {
                break 'outer;
            }
        }
    }

    println!("Day  8, Task 1:{}", steps);
}
