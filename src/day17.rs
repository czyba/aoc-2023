use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

fn parse() -> Vec<String> {
    lines_from_file("src/day17.txt")
        .unwrap()
        .map(|l| l.to_owned())
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    row: usize,
    col: usize,
    num_straight: usize,
    direction: Direction,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct StateWithCost {
    state: State,
    cost: u32,
}

impl Ord for StateWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for StateWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn calulate_shortest_distance<F: Fn(&Vec<String>, &State) -> Vec<State>>(
    input: &Vec<String>,
    succ_fn: F,
    max_consecutive: usize,
) -> u32 {
    let mut costs: HashMap<State, u32> = HashMap::new();
    let mut next = BinaryHeap::new();
    for (i, _) in input.iter().enumerate() {
        for j in 0..input[i].len() {
            for k in 0..(max_consecutive + 1) {
                costs.insert(
                    State {
                        row: i,
                        col: j,
                        num_straight: k,
                        direction: Direction::North,
                    },
                    u32::MAX,
                );
                costs.insert(
                    State {
                        row: i,
                        col: j,
                        num_straight: k,
                        direction: Direction::East,
                    },
                    u32::MAX,
                );
                costs.insert(
                    State {
                        row: i,
                        col: j,
                        num_straight: k,
                        direction: Direction::South,
                    },
                    u32::MAX,
                );
                costs.insert(
                    State {
                        row: i,
                        col: j,
                        num_straight: k,
                        direction: Direction::West,
                    },
                    u32::MAX,
                );
            }
        }
    }
    let s1 = StateWithCost {
        state: State {
            row: 0,
            col: 0,
            direction: Direction::South,
            num_straight: 0,
        },
        cost: 0,
    };
    let s2 = StateWithCost {
        state: State {
            row: 0,
            col: 0,
            direction: Direction::East,
            num_straight: 0,
        },
        cost: 0,
    };
    costs.insert(s1.state.clone(), 0);
    costs.insert(s2.state.clone(), 0);
    next.push(s1);
    next.push(s2);

    while let Some(s) = next.pop() {
        // Already something with lower cost found
        if *costs.get(&s.state).unwrap() < s.cost {
            continue;
        }

        let cost = *costs.get(&s.state).unwrap();
        for succ in succ_fn(input, &s.state) {
            let edge_cost = (input[succ.row].as_bytes()[succ.col] - "0".as_bytes()[0]) as u32;
            let next_cost = cost + edge_cost;
            let last_cost = costs.get_mut(&succ).unwrap();
            if *last_cost > next_cost {
                *last_cost = next_cost;
                next.push(StateWithCost {
                    state: succ,
                    cost: next_cost,
                });
            }
        }
    }

    let r = costs
        .iter()
        .filter(|(s, _)| s.row == input.len() - 1 && s.col == input[s.row].len() - 1)
        .map(|(_, v)| *v)
        .min()
        .unwrap();
    r
}

fn get_successors_task1(input: &Vec<String>, state: &State) -> Vec<State> {
    let mut res = Vec::new();
    // straight
    if state.num_straight < 3 {
        if let Some(next) = get_straight_successor(input, state) {
            res.push(next);
        }
    };

    if let Some(next) = get_left_successor(input, state) {
        res.push(next);
    }

    if let Some(next) = get_right_successor(input, state) {
        res.push(next);
    }

    res
}

fn get_straight_successor(input: &Vec<String>, state: &State) -> Option<State> {
    match state.direction {
        Direction::North => {
            if state.row > 0 {
                Some(State {
                    row: state.row - 1,
                    col: state.col,
                    direction: state.direction,
                    num_straight: state.num_straight + 1,
                })
            } else {
                None
            }
        }
        Direction::East => {
            if state.col < input[state.row].len() - 1 {
                Some(State {
                    row: state.row,
                    col: state.col + 1,
                    direction: state.direction,
                    num_straight: state.num_straight + 1,
                })
            } else {
                None
            }
        }
        Direction::South => {
            if state.row < input.len() - 1 {
                Some(State {
                    row: state.row + 1,
                    col: state.col,
                    direction: state.direction,
                    num_straight: state.num_straight + 1,
                })
            } else {
                None
            }
        }
        Direction::West => {
            if state.col > 0 {
                Some(State {
                    row: state.row,
                    col: state.col - 1,
                    direction: state.direction,
                    num_straight: state.num_straight + 1,
                })
            } else {
                None
            }
        }
    }
}

fn get_left_successor(input: &Vec<String>, state: &State) -> Option<State> {
    match state.direction {
        Direction::North => {
            if state.col > 0 {
                Some(State {
                    row: state.row,
                    col: state.col - 1,
                    direction: Direction::West,
                    num_straight: 1,
                })
            } else {
                None
            }
        }
        Direction::East => {
            if state.row > 0 {
                Some(State {
                    row: state.row - 1,
                    col: state.col,
                    direction: Direction::North,
                    num_straight: 1,
                })
            } else {
                None
            }
        }
        Direction::South => {
            if state.col < input[state.row].len() - 1 {
                Some(State {
                    row: state.row,
                    col: state.col + 1,
                    direction: Direction::East,
                    num_straight: 1,
                })
            } else {
                None
            }
        }
        Direction::West => {
            if state.row < input.len() - 1 {
                Some(State {
                    row: state.row + 1,
                    col: state.col,
                    direction: Direction::South,
                    num_straight: 1,
                })
            } else {
                None
            }
        }
    }
}

fn get_right_successor(input: &Vec<String>, state: &State) -> Option<State> {
    match state.direction {
        Direction::North => {
            if state.col < input[state.row].len() - 1 {
                Some(State {
                    row: state.row,
                    col: state.col + 1,
                    direction: Direction::East,
                    num_straight: 1,
                })
            } else {
                None
            }
        }
        Direction::East => {
            if state.row < input.len() - 1 {
                Some(State {
                    row: state.row + 1,
                    col: state.col,
                    direction: Direction::South,
                    num_straight: 1,
                })
            } else {
                None
            }
        }
        Direction::South => {
            if state.col > 0 {
                Some(State {
                    row: state.row,
                    col: state.col - 1,
                    direction: Direction::West,
                    num_straight: 1,
                })
            } else {
                None
            }
        }
        Direction::West => {
            if state.row > 0 {
                Some(State {
                    row: state.row - 1,
                    col: state.col,
                    direction: Direction::North,
                    num_straight: 1,
                })
            } else {
                None
            }
        }
    }
}

pub fn task1() {
    let input = parse();
    let r = calulate_shortest_distance(&input, get_successors_task1, 3);
    println!("Day 17, Task 1: {}", r);
}

pub fn task2() {
    let input = parse();
    let r = calulate_shortest_distance(&input, get_successors_task2, 10);
    println!("Day 17, Task 2: {}", r);
}

fn get_successors_task2(input: &Vec<String>, state: &State) -> Vec<State> {
    let mut res = Vec::new();
    // straight
    if state.num_straight < 10 {
        if let Some(next) = get_straight_successor(input, state) {
            res.push(next);
        }
    };

    if state.num_straight >= 4 {
        if let Some(next) = get_left_successor(input, state) {
            res.push(next);
        }
    }

    if state.num_straight >= 4 {
        if let Some(next) = get_right_successor(input, state) {
            res.push(next);
        }
    }
    res
}
