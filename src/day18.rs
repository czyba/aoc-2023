use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

fn parse() -> Vec<DigCommand> {
    lines_from_file("src/day18.txt")
        .unwrap()
        .map(|l| parse_line(&l))
        .collect()
}

fn parse_line(line: &str) -> DigCommand {
    let mut iter = line.split_ascii_whitespace();
    let dir = match iter.next().unwrap() {
        "U" => Direction::North,
        "R" => Direction::East,
        "D" => Direction::South,
        "L" => Direction::West,
        x => panic!("{:?}" , x),
    };
    let len = iter.next().unwrap().parse::<u32>().unwrap();
    let t = iter.next().unwrap();
    let hex_code = t[1..t.len() - 1].to_owned();
    DigCommand {
        len, dir, hex_code
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {

    fn next(self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::West => (pos.0, pos.1 - 1),
        }
    }

}

struct DigCommand {
    len: u32,
    dir: Direction,
    hex_code: String,
}

fn calculate_surroundings(commands: &Vec<DigCommand>) -> BTreeSet<(i32, i32)> {
    let mut pos = (0,0);
    let mut set = BTreeSet::new();
    set.insert(pos.clone());
    for command in commands {
        for _ in 0..command.len {
            pos = command.dir.next(pos);
            set.insert(pos);
        }
    }
    set
}

pub fn task1() {
    let commands = parse();
    let outline = calculate_surroundings(&commands);
    let a = outline
        .iter()
        .into_group_map_by(|(x,_)| x);

        // TODO Incorrect, since it does not cover "spikes"
    let r : i32 = a.values()
        .map(|v| {
            let (min, max) = v.iter()
                .map(|(x,y)| y)
                .fold((i32::MAX, i32::MIN), |acc, n| {
                    (acc.0.min(*n), acc.1.max(*n))
                });
            return max - min + 1;
        })
        .sum();

    println!("Day 18, Task 1: {}", r);
}


