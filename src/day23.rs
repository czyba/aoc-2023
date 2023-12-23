use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

fn parse() -> Vec<String> {
    lines_from_file("src/day23.txt")
        .unwrap()
        .collect()
}

const H : u8 = 35;
const E : u8 = 46;
const D : u8 = 118;
const R : u8 = 62;

fn i(input: &[String], row: usize, col: usize) -> u8 {
    input[row].as_bytes()[col]
}

fn find_all_paths(input: &[String], start: (usize, usize)) -> usize {
    // TODO: Loop detection at intersections
    let mut prev = HashSet::new();
    let mut cur = HashSet::new();
    let mut next = HashSet::new();
    cur.insert(start);

    let mut steps = 0;
    while !cur.is_empty() {
        steps += 1;
        for pos in &cur {
            // Up
            if pos.0 > 0 && !prev.contains(&(pos.0 - 1, pos.1)) && i(input, pos.0 - 1, pos.1) != H && i(input, pos.0, pos.1) == E {
                next.insert((pos.0 - 1, pos.1));
            }
            // Right
            if pos.1 < input[0].len() - 1 && !prev.contains(&(pos.0, pos.1 + 1)) && i(input, pos.0, pos.1 + 1) != H && (i(input, pos.0, pos.1) == E || i(input, pos.0, pos.1) == R) {
                next.insert((pos.0, pos.1 + 1));
            }
            // Down
            if pos.0 < input.len() - 1 && !prev.contains(&(pos.0 + 1, pos.1)) && i(input, pos.0 + 1, pos.1) != H && (i(input, pos.0, pos.1) == E || i(input, pos.0, pos.1) == D){
                next.insert((pos.0 + 1, pos.1));
            }
            // Left
            if pos.1 > 0 && !prev.contains(&(pos.0, pos.1 - 1)) && i(input, pos.0, pos.1 - 1) != H && i(input, pos.0, pos.1) == E{
                next.insert((pos.0, pos.1 - 1));
            }
        }
        prev = cur;
        cur = next;
        next = HashSet::new();
    }


    steps - 1
}

pub fn task1() -> crate::AOCResult<usize> {
    let input = parse();
    let start = (0,1);
    let r = find_all_paths(&input, start);


    crate::AOCResult {
        day: 23,
        task: 1,
        r,
    }
}

pub fn task2() -> crate::AOCResult<usize> {
    let mut input = parse();
    let start = (0,1);
    input.iter_mut()
        .for_each(|s| { *s = s.replace(">", ".").replace("v", "."); });
    find_all_paths(&input, start);


    crate::AOCResult {
        day: 23,
        task: 1,
        r: 0,
    }
}