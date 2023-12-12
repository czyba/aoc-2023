use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

const DIGITS: [(&str, i32); 10] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

const DIGITS_INCLUDING_WRITTEN_OUT_DIGITS: [(&str, i32); 19] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub struct InOrderReplacer<'a, T> {
    input: &'a str,
    replacements: &'a [(&'a str, T)],
    current: usize,
}

impl<'a, T: Clone> Iterator for InOrderReplacer<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.input.len() {
            return None;
        }
        for start_index in self.current..self.input.len() {
            let substring = &self.input[start_index..self.input.len()];
            for (digit, value) in self.replacements {
                if substring.starts_with(*digit) {
                    self.current = start_index + 1;
                    return Some(value.clone());
                }
            }
        }
        None
    }
}

fn replacer<'a, T>(input: &'a str, replacements: &'a [(&'a str, T)]) -> InOrderReplacer<'a, T> {
    InOrderReplacer {
        input,
        replacements,
        current: 0,
    }
}

fn calculate_value(replacements: &[(&str, i32)], task_id: i32) {
    let c = lines_from_file("src/day1_task1.txt")
        .unwrap()
        .map(|line| {
            replacer(&line, replacements).fold(None, |acc, val| {
                acc.or(Some((val, val))).map(|v| (v.0, val))
            })
        })
        .map(|p| p.map(|(l, r)| l * 10 + r).unwrap_or(0))
        .fold(0, i32::wrapping_add);

    println!("Day  1, Task {}: {}", task_id, c);
}

pub fn task1() {
    calculate_value(&DIGITS, 1);
}

pub fn task2() {
    calculate_value(&DIGITS_INCLUDING_WRITTEN_OUT_DIGITS, 2);
}
