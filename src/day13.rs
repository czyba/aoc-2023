use std::fs::File;
use std::io::prelude::Read;
use std::io::Result;

fn read_file_to_string(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
struct BitField {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

#[cfg(windows)]
const DOUBLE_LINE_ENDING: &str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_LINE_ENDING: &str = "\n\n";

fn parse() -> Vec<BitField> {
    read_file_to_string("src/day13.txt")
        .unwrap()
        .split(DOUBLE_LINE_ENDING)
        .map(|split| {
            let rows: Vec<u64> = split.lines().map(parse_line).collect();
            let len = split.lines().next().unwrap().len();

            let cols = (0..len)
                .map(|col| {
                    rows.iter().enumerate().fold(0, |acc, (row, n)| {
                        acc + if (n & 1 << col) != 0 { 1 << row } else { 0 }
                    })
                })
                .collect();

            BitField { rows, cols }
        })
        .collect()
}

fn parse_line(line: &str) -> u64 {
    line.char_indices().fold(0, |acc, c| {
        if let (index, '.') = c {
            return acc + (1 << index);
        }
        acc
    })
}

fn find_reflection_index(field: &Vec<u64>) -> usize {
    for i in 1..field.len() {
        let mut mirrors = true;
        for j in 0..i.min(field.len() - i) {
            mirrors &= field[i + j] == field[i - 1 - j]
        }
        if mirrors {
            return i;
        }
    }
    0
}

pub fn task1() {
    let bit_fields = parse();
    let num = bit_fields
        .iter()
        .map(|bf| {
            (
                find_reflection_index(&bf.rows),
                find_reflection_index(&bf.cols),
            )
        })
        .fold(0, |acc, (r, c)| acc + r * 100 + c);

    println!("Day 13, Task 1: {}", num);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mirror_index() {}
}
