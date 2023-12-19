use std::collections::{BTreeMap, BTreeSet};
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
        x => panic!("{:?}", x),
    };
    let len = iter.next().unwrap().parse::<u32>().unwrap();
    let t = iter.next().unwrap();
    let hex_code = t[1..t.len() - 1].to_owned();
    DigCommand { len, dir, hex_code }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct DigCommand {
    len: u32,
    dir: Direction,
    hex_code: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    row: i64,
    col_1: i64,
    col_2: i64,
}

fn calculate_surroundings(commands: &Vec<DigCommand>) -> BTreeSet<Range> {
    let mut pos = (0i64, 0i64);
    let mut range_set = BTreeSet::new();
    for command in commands {
        pos = match command.dir {
            Direction::North => (pos.0 - command.len as i64, pos.1),
            Direction::East => {
                range_set.insert(Range {
                    row: pos.0,
                    col_1: pos.1,
                    col_2: pos.1 + command.len as i64,
                });
                (pos.0, pos.1 + command.len as i64)
            }
            Direction::South => (pos.0 + command.len as i64, pos.1),
            Direction::West => {
                range_set.insert(Range {
                    row: pos.0,
                    col_1: pos.1 - command.len as i64,
                    col_2: pos.1,
                });
                (pos.0, pos.1 - command.len as i64)
            }
        }
    }
    range_set
}

pub fn task1() -> crate::AOCResult<i64> {
    let commands = parse();
    let ranges = calculate_surroundings(&commands);
    let r2 = imscared(ranges);
    crate::AOCResult {
        day: 18,
        task: 1,
        r: r2,
    }
}

trait Helpers {
    fn len(&self) -> i64;
    fn contains_completely(&self, other: &Self) -> bool;
}

impl Helpers for (i64, i64) {
    fn len(&self) -> i64 {
        self.1 - self.0 + 1
    }

    fn contains_completely(&self, other: &Self) -> bool {
        self.0 < other.0 && other.1 < self.1
    }
}

impl From<&Range> for (i64, i64) {
    fn from(value: &Range) -> Self {
        (value.col_1, value.col_2)
    }
}

fn imscared(range_set: BTreeSet<Range>) -> i64 {
    let a: BTreeMap<i64, Vec<Range>> = range_set
        .into_iter()
        .into_group_map_by(|r| r.row)
        .into_iter()
        .collect();

    let mut last_ranges = BTreeSet::new();
    let mut start = a.iter();
    let first = start.next().unwrap();
    let mut last_index = *first.0;
    let mut count = 0;
    // let row_offset = last_index;

    for range in first.1 {
        let r = (range.col_1, range.col_2);
        count += r.len();
        last_ranges.insert(r);
    }
    let mut row_count = count;

    for (index, ranges) in start {
        // Add count inbetween the last two horizontals
        count += row_count * (index - last_index - 1);
        last_index = *index;

        // println!("count: {}", count);
        // println!("actual_row: {}", index - row_offset);
        // println!("last: {:?}", last_ranges);
        // println!("new: {:?}", ranges);

        let mut iter_new = ranges.iter().map(<&Range as Into<(i64, i64)>>::into);
        let mut iter_old = last_ranges.iter();
        let mut old = *iter_old.next().unwrap();
        let mut new: (i64, i64) = iter_new.next().unwrap();
        let mut next_ranges = BTreeSet::new();
        let mut this_row_count = 0;
        row_count = 0;

        loop {
            // println!("old: {:?}, new: {:?}", old, new);
            // old: |---|
            // new:         |---|
            if old.1 < new.0 {
                this_row_count += old.len();
                row_count += old.len();
                next_ranges.insert(old);
                if let Some(next_old) = iter_old.next() {
                    old = *next_old;
                    continue;
                }
                this_row_count += new.len();
                row_count += new.len();
                next_ranges.insert(new);
                iter_new.for_each(|e| {
                    this_row_count += e.len();
                    row_count += e.len();
                    next_ranges.insert(e);
                });
                break;
            // old:         |---|
            // new: |---|
            } else if new.1 < old.0 {
                this_row_count += new.len();
                row_count += new.len();
                next_ranges.insert(new);
                if let Some(next_new) = iter_new.next() {
                    new = next_new;
                    continue;
                }
                this_row_count += old.len();
                row_count += old.len();
                next_ranges.insert(old);
                iter_old.for_each(|e| {
                    this_row_count += e.len();
                    row_count += e.len();
                    next_ranges.insert(*e);
                });
                break;
            }
            // old: |--|
            // new: |--|
            if old == new {
                this_row_count += new.len();

                if let Some(next_old) = iter_old.next() {
                    old = *next_old;
                    if let Some(next_new) = iter_new.next() {
                        new = next_new;
                        continue;
                    }
                    this_row_count += old.len();
                    row_count += old.len();
                    next_ranges.insert(old);
                    iter_old.for_each(|e| {
                        this_row_count += e.len();
                        row_count += e.len();
                        next_ranges.insert(*e);
                    });
                    break;
                }
                iter_new.for_each(|e| {
                    this_row_count += e.len();
                    row_count += e.len();
                    next_ranges.insert(e);
                });
                break;
            // old: |-------------|
            // new:     |----|
            // res: |--|      |---|
            } else if old.contains_completely(&new) {
                let start_range = (old.0, new.0);
                this_row_count += start_range.len() + new.len() - 2;
                row_count += start_range.len();
                next_ranges.insert(start_range);
                old = (new.1, old.1);

                if let Some(next_new) = iter_new.next() {
                    new = next_new;
                    continue;
                }
                this_row_count += old.len();
                row_count += old.len();
                next_ranges.insert(old);
                iter_old.for_each(|e| {
                    this_row_count += e.len();
                    row_count += e.len();
                    next_ranges.insert(*e);
                });
                break;
            // old: |-------------|
            // new: |----|
            } else if old.0 == new.0 {
                this_row_count += new.len() - 1;
                old = (new.1, old.1);

                if let Some(next_new) = iter_new.next() {
                    new = next_new;
                    continue;
                }
                this_row_count += old.len();
                row_count += old.len();
                next_ranges.insert(old);
                iter_old.for_each(|e| {
                    this_row_count += e.len();
                    row_count += e.len();
                    next_ranges.insert(*e);
                });
                break;
            // old: |-------------|
            // new:          |----|
            } else if old.1 == new.1 {
                let start_range = (old.0, new.0);
                this_row_count += start_range.len() + new.len() - 1;
                row_count += start_range.len();
                next_ranges.insert(start_range);

                if let Some(next_old) = iter_old.next() {
                    old = *next_old;
                    if let Some(next_new) = iter_new.next() {
                        new = next_new;
                        continue;
                    }
                    this_row_count += old.len();
                    row_count += old.len();
                    next_ranges.insert(old);
                    iter_old.for_each(|e| {
                        this_row_count += e.len();
                        row_count += e.len();
                        next_ranges.insert(*e);
                    });
                    break;
                }
                iter_new.for_each(|e| {
                    this_row_count += e.len();
                    row_count += e.len();
                    next_ranges.insert(e);
                });
                break;
            // old:      |----|
            // new:  |---|
            } else if old.0 == new.1 {
                let merged_range = (new.0, old.1);
                old = merged_range;

                if let Some(next_new) = iter_new.next() {
                    new = next_new;
                    continue;
                }
                this_row_count += old.len();
                row_count += old.len();
                next_ranges.insert(old);
                iter_old.for_each(|e| {
                    this_row_count += e.len();
                    row_count += e.len();
                    next_ranges.insert(*e);
                });
                break;
            // old: |----|
            // new:      |---|
            } else if old.1 == new.0 {
                let merged_range = (old.0, new.1);
                new = merged_range;

                if let Some(next_old) = iter_old.next() {
                    old = *next_old;
                    continue;
                }
                this_row_count += new.len();
                row_count += new.len();
                next_ranges.insert(new);
                iter_new.for_each(|e| {
                    this_row_count += e.len();
                    row_count += e.len();
                    next_ranges.insert(e);
                });
                break;
            }
            panic!("old: {:?}, new: {:?}", old, new);
        }

        // println!("{}, {}", this_row_count, row_count);
        count += this_row_count;
        last_ranges = next_ranges;
    }
    count
}

pub fn task2() -> crate::AOCResult<i64> {
    let mut commands = parse();
    for command in commands.iter_mut() {
        let distance =
            usize::from_str_radix(&command.hex_code[1..command.hex_code.len() - 1], 16).unwrap();
        let dir = match &command.hex_code[(command.hex_code.len() - 1)..command.hex_code.len()] {
            "0" => Direction::East,
            "1" => Direction::South,
            "2" => Direction::West,
            "3" => Direction::North,
            _ => panic!(),
        };
        command.len = distance as u32;
        command.dir = dir;
    }

    let ranges = calculate_surroundings(&commands);

    let r2 = imscared(ranges);
    crate::AOCResult {
        day: 18,
        task: 2,
        r: r2,
    }
}
