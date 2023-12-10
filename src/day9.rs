use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

fn parse() -> Vec<Vec<i64>> {
    lines_from_file("src/day9.txt")
        .unwrap()
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|word| word.parse::<i64>().unwrap())
        .collect()
}

fn extrapolate_forwards(data: &Vec<i64>) -> i64 {
    let mut stack_data = vec![];
    let mut current_data = data;
    while current_data.iter().any(|num| *num != 0) {
        let mut next_data = Vec::with_capacity(current_data.len() - 1);
        let mut iter = current_data.iter();
        let mut last = *iter.next().unwrap();
        for next in iter {
            next_data.push(next - last);
            last = *next;
        }
        stack_data.push(next_data);
        current_data = stack_data.last().unwrap();
    }

    let mut interpolated_value = 0;

    let mut stack_iter = stack_data.iter();
    while let Some(data) = stack_iter.next_back() {
        interpolated_value += data.last().unwrap();
    }

    interpolated_value + data.last().unwrap()
}

fn extrapolate_backwards(data: &Vec<i64>) -> i64 {
    let mut stack_data = vec![];
    let mut current_data = data;
    while current_data.iter().any(|num| *num != 0) {
        let mut next_data = Vec::with_capacity(current_data.len() - 1);
        let mut iter = current_data.iter();
        let mut last = *iter.next().unwrap();
        for next in iter {
            next_data.push(next - last);
            last = *next;
        }
        stack_data.push(next_data);
        current_data = stack_data.last().unwrap();
    }

    let mut interpolated_value = 0;

    let mut stack_iter = stack_data.iter();
    while let Some(data) = stack_iter.next_back() {
        interpolated_value = data.first().unwrap() - interpolated_value;
    }

    data.first().unwrap() - interpolated_value
}

pub fn task1() {
    let sum: i64 = parse()
        .iter_mut()
        .map(|data| extrapolate_forwards(data))
        .sum();

    println!("Day  9, Task 1: {}", sum);
}

pub fn task2() {
    let sum: i64 = parse()
        .iter_mut()
        .map(|data| extrapolate_backwards(data))
        .sum();

    println!("Day  9, Task 2: {}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extend_with_extrapolattion_test() {
        assert_eq!(18, extrapolate_forwards(&vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(28, extrapolate_forwards(&vec![1, 3, 6, 10, 15, 21]));
        assert_eq!(68, extrapolate_forwards(&vec![10, 13, 16, 21, 30, 45]));
    }

    #[test]
    fn extrapolate_backwards_test() {
        assert_eq!(5, extrapolate_backwards(&vec![10, 13, 16, 21, 30, 45]));
    }
}
