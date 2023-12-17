use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

fn parse() -> Vec<String> {
    lines_from_file("src/day16.txt")
        .unwrap()
        .map(|l| l.to_owned())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    row: usize,
    col: usize,
    dir: Direction,
}

trait DirectBeam {
    fn direct(&self, state: &State) -> Vec<State>;
}

impl DirectBeam for u8 {
    fn direct(&self, state: &State) -> Vec<State> {
        match self {
            // '.'
            46 => match state.dir {
                Direction::North => vec![State {
                    row: state.row + 1,
                    col: state.col,
                    dir: state.dir,
                }],
                Direction::East => vec![State {
                    row: state.row,
                    col: state.col.wrapping_sub(1),
                    dir: state.dir,
                }],
                Direction::South => vec![State {
                    row: state.row.wrapping_sub(1),
                    col: state.col,
                    dir: state.dir,
                }],
                Direction::West => vec![State {
                    row: state.row,
                    col: state.col + 1,
                    dir: state.dir,
                }],
            },
            // '/'
            47 => match state.dir {
                Direction::North => vec![State {
                    row: state.row,
                    col: state.col.wrapping_sub(1),
                    dir: Direction::East,
                }],
                Direction::East => vec![State {
                    row: state.row + 1,
                    col: state.col,
                    dir: Direction::North,
                }],
                Direction::South => vec![State {
                    row: state.row,
                    col: state.col + 1,
                    dir: Direction::West,
                }],
                Direction::West => vec![State {
                    row: state.row.wrapping_sub(1),
                    col: state.col,
                    dir: Direction::South,
                }],
            },
            // '\'
            92 => match state.dir {
                Direction::North => vec![State {
                    row: state.row,
                    col: state.col + 1,
                    dir: Direction::West,
                }],
                Direction::East => vec![State {
                    row: state.row.wrapping_sub(1),
                    col: state.col,
                    dir: Direction::South,
                }],
                Direction::South => vec![State {
                    row: state.row,
                    col: state.col.wrapping_sub(1),
                    dir: Direction::East,
                }],
                Direction::West => vec![State {
                    row: state.row + 1,
                    col: state.col,
                    dir: Direction::North,
                }],
            },
            // '-'
            45 => match state.dir {
                Direction::North => vec![
                    State {
                        row: state.row,
                        col: state.col + 1,
                        dir: Direction::West,
                    },
                    State {
                        row: state.row,
                        col: state.col.wrapping_sub(1),
                        dir: Direction::East,
                    },
                ],
                Direction::East => vec![State {
                    row: state.row,
                    col: state.col.wrapping_sub(1),
                    dir: state.dir,
                }],
                Direction::South => vec![
                    State {
                        row: state.row,
                        col: state.col + 1,
                        dir: Direction::West,
                    },
                    State {
                        row: state.row,
                        col: state.col.wrapping_sub(1),
                        dir: Direction::East,
                    },
                ],
                Direction::West => vec![State {
                    row: state.row,
                    col: state.col + 1,
                    dir: state.dir,
                }],
            },
            // '|'
            124 => match state.dir {
                Direction::North => vec![State {
                    row: state.row + 1,
                    col: state.col,
                    dir: state.dir,
                }],
                Direction::East => vec![
                    State {
                        row: state.row + 1,
                        col: state.col,
                        dir: Direction::North,
                    },
                    State {
                        row: state.row.wrapping_sub(1),
                        col: state.col,
                        dir: Direction::South,
                    },
                ],
                Direction::South => vec![State {
                    row: state.row.wrapping_sub(1),
                    col: state.col,
                    dir: state.dir,
                }],
                Direction::West => vec![
                    State {
                        row: state.row + 1,
                        col: state.col,
                        dir: Direction::North,
                    },
                    State {
                        row: state.row.wrapping_sub(1),
                        col: state.col,
                        dir: Direction::South,
                    },
                ],
            },
            _ => panic!(),
        }
    }
}

fn calculate_energized_tiles(input: &[String], start: State) -> usize {
    let mut seen = HashSet::new();
    let mut worklist = vec![start.clone()];
    seen.insert(start);
    while let Some(s) = worklist.pop() {
        let next_states = input[s.row].as_bytes()[s.col].direct(&s);
        worklist.extend(
            next_states
                .into_iter()
                .filter(|n| n.row < input.len() && n.col < input[n.row].len())
                .filter(|n| seen.insert(n.clone())),
        );
    }

    seen.iter().map(|s| (s.row, s.col)).unique().count()
}

pub fn task1() {
    let input = parse();
    let start = State {
        row: 0,
        col: 0,
        dir: Direction::West,
    };

    let result = calculate_energized_tiles(&input, start);

    println!("Day 16, Task 1: {}", result);
}

pub fn task2() {
    let input = parse();

    let mut result = 0;
    for row in 0..(input.len()) {
        let start = State {
            row,
            col: 0,
            dir: Direction::West,
        };
        result = result.max(calculate_energized_tiles(&input, start));
        let start = State {
            row,
            col: input[row].len() - 1,
            dir: Direction::East,
        };
        result = result.max(calculate_energized_tiles(&input, start));
    }
    for col in 0..(input[0].len()) {
        let start = State {
            row: 0,
            col,
            dir: Direction::North,
        };
        result = result.max(calculate_energized_tiles(&input, start));
        let start = State {
            row: input.len() - 1,
            col,
            dir: Direction::South,
        };
        result = result.max(calculate_energized_tiles(&input, start));
    }

    println!("Day 16, Task 2: {}", result);
}
