use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HotSpringStatus {
    Operational,
    Damaged,
    Unknown,
}

fn parse() -> Vec<(Vec<HotSpringStatus>, Vec<usize>)> {
    lines_from_file("src/day12.txt")
        .unwrap()
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> (Vec<HotSpringStatus>, Vec<usize>) {
    let mut iter = line.split_ascii_whitespace();
    let hss = iter
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '?' => HotSpringStatus::Unknown,
            '.' => HotSpringStatus::Operational,
            '#' => HotSpringStatus::Damaged,
            _ => panic!(),
        })
        .collect();

    let operationals = iter
        .next()
        .unwrap()
        .split(',')
        .map(|digits| digits.parse::<usize>().unwrap())
        .collect();

    (hss, operationals)
}

pub fn task1() {
    let data = parse();
    let count = brute_force(&data);
    println!("Day 12, Task 1: {}", count);
}

fn brute_force(data: &Vec<(Vec<HotSpringStatus>, Vec<usize>)>) -> usize {
    let mut sum = 0;

    for (hss, operationals) in data {
        let mut copy = hss.clone();
        let mut num_valid = 0;
        let mut current_index = 0;

        let stack = copy
            .iter_mut()
            .enumerate()
            .filter(|(_, e)| **e == HotSpringStatus::Unknown)
            .map(|(index, _)| index)
            .collect_vec();

        loop {
            if current_index == stack.len() {
                if validate(&copy, operationals) {
                    num_valid += 1;
                }
                current_index -= 1;
            }
            let status = &mut copy[stack[current_index]];
            match status {
                HotSpringStatus::Operational => {
                    *status = HotSpringStatus::Damaged;
                    current_index += 1;
                }
                HotSpringStatus::Damaged => {
                    *status = HotSpringStatus::Unknown;
                    if current_index == 0 {
                        break;
                    }
                    current_index -= 1;
                }
                HotSpringStatus::Unknown => {
                    *status = HotSpringStatus::Operational;
                    current_index += 1;
                }
            }
        }
        sum += num_valid;
    }
    sum
}

fn validate(hss: &[HotSpringStatus], combinations: &Vec<usize>) -> bool {
    let mut iter = hss.iter();
    for chain in combinations {
        let mut cnt = *chain;
        for t in iter.by_ref() {
            if *t == HotSpringStatus::Operational && cnt == *chain {
                continue;
            }
            if *t == HotSpringStatus::Operational && cnt > 0 {
                return false;
            }
            if *t == HotSpringStatus::Operational && cnt == 0 {
                break;
            }
            if *t == HotSpringStatus::Damaged && cnt == 0 {
                return false;
            }
            if *t == HotSpringStatus::Damaged {
                cnt -= 1;
            }
        }

        if cnt > 0 {
            return false;
        }
    }

    iter.all(|s| *s == HotSpringStatus::Operational)
}
