use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

pub fn parse() -> Vec<String> {
    lines_from_file("src/day21.txt")
        .unwrap()
        .collect()
}

fn get_starting_pos(input: &mut [String]) -> (usize, usize) {
    for (row, s) in input.iter_mut().enumerate() {
        if let Some((col, _)) = s.chars().enumerate().find(|(_, c)| *c == 'S') {
            unsafe {
                s.as_bytes_mut()[col] = ".".as_bytes()[0];
            }
            return (row, col);
        }
    }
    panic!()
}

#[derive(Debug, PartialEq, Eq)]
struct MinDistancePos {
    pos: (usize, usize),
    distance: usize,
}

impl std::cmp::Ord for MinDistancePos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl std::cmp::PartialOrd for MinDistancePos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_paths(
    input: &[String],
    starting_pos: (usize, usize),
    max_steps: usize,
) -> HashMap<(usize, usize), usize> {
    let mut shortest_paths = HashMap::new();
    let mut bheap = BinaryHeap::new();
    bheap.push(MinDistancePos {
        pos: starting_pos,
        distance: 0,
    });
    shortest_paths.insert(starting_pos, 0usize);
    let dot = ".".as_bytes()[0];
    while let Some(min_pos) = bheap.pop() {
        if min_pos.distance >= max_steps {
            continue;
        }
        let pos = min_pos.pos;
        if pos.0 > 0
            && !shortest_paths.contains_key(&(pos.0 - 1, pos.1))
            && input[pos.0 - 1].as_bytes()[pos.1] == dot
        {
            shortest_paths.insert((pos.0 - 1, pos.1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0 - 1, pos.1),
                distance: min_pos.distance + 1,
            });
        }
        if pos.1 > 0
            && !shortest_paths.contains_key(&(pos.0, pos.1 - 1))
            && input[pos.0].as_bytes()[pos.1 - 1] == dot
        {
            shortest_paths.insert((pos.0, pos.1 - 1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0, pos.1 - 1),
                distance: min_pos.distance + 1,
            });
        }
        if pos.0 < input.len() - 1
            && !shortest_paths.contains_key(&(pos.0 + 1, pos.1))
            && input[pos.0 + 1].as_bytes()[pos.1] == dot
        {
            shortest_paths.insert((pos.0 + 1, pos.1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0 + 1, pos.1),
                distance: min_pos.distance + 1,
            });
        }
        if pos.1 < input[0].len() - 1
            && !shortest_paths.contains_key(&(pos.0, pos.1 + 1))
            && input[pos.0].as_bytes()[pos.1 + 1] == dot
        {
            shortest_paths.insert((pos.0, pos.1 + 1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0, pos.1 + 1),
                distance: min_pos.distance + 1,
            });
        }
    }
    shortest_paths
}

pub fn task1() -> crate::AOCResult<usize> {
    let mut input = parse();
    let starting_pos = get_starting_pos(&mut input);
    let shortes_paths = shortest_paths(&input, starting_pos, 64);
    let r = shortes_paths
        .iter()
        .filter(|(pos, _)| {
            ((pos.0.max(starting_pos.0) - pos.0.min(starting_pos.0))
                + (pos.1.max(starting_pos.1) - pos.1.min(starting_pos.1)))
                % 2
                == 6 % 2
        })
        .count();

    crate::AOCResult {
        day: 21,
        task: 1,
        r,
    }
}
