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
    let mut s = map(&outline);
    dfs(&mut s);
    let r = s.iter()
        .flat_map(|v|
            v.iter()
        )
        .filter(|&&c| c != 'X')
        .count();

    println!("Day 18, Task 1: {}", r);
    
}

fn dfs(s: &mut Vec<Vec<char>>) {
    let mut worklist : Vec<(usize, usize)> = Vec::new();
    worklist.push((0,0));
    let row_len = s.len();
    let col_len = s[0].len();

    while let Some((row, col)) = worklist.pop() {
        if s[row][col] != '.' {
            continue;
        }
        s[row][col] = 'X';
        if row + 1 < row_len {
            worklist.push((row + 1, col));
        }
        if col + 1 < col_len {
            worklist.push((row, col + 1));
        }
        if row > 0 {
            worklist.push((row - 1, col));
        }
        if col > 0 {
            worklist.push((row, col - 1));
        }
    }
}

fn map(data: &BTreeSet<(i32, i32)>) -> Vec<Vec<char>> {
    let bounds = data.iter()
        .fold(((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)), |acc, e| {
            ((acc.0.0.min(e.0), acc.0.1.max(e.0)), (acc.1.0.min(e.1),  acc.1.1.max(e.1)))
        });
    
    let rows = (bounds.0.1 - bounds.0.0 + 1) as usize;
    let cols = (bounds.1.1 - bounds.1.0 + 1) as usize;
    let mut v = Vec::new();
    let mut s = String::with_capacity((rows + 2) * (cols + 5));
    let t = vec!['.'; cols  + 2];
    v.push(t);

    for i in 0..(rows as i32) {
        let mut t = Vec::with_capacity(cols + 2);
        t.push('.');
        for j in 0..(cols as i32) {
            if data.contains(&(i + bounds.0.0, j + bounds.1.0)) {
                t.push('#');
            } else {
                t.push('.');
            }
        }
        t.push('.');
        v.push(t);
    }

    let t = vec!['.'; cols  + 2];
    v.push(t);

    v
}
