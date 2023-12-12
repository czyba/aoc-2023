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
struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
struct NumberMatch {
    pub start: Position,
    pub end: Position,
    pub value: i64,
}

impl NumberMatch {
    fn is_adjacent_to(&self, position: &Position) -> bool {
        let num_line = self.start.line;
        let symbol_line = position.line;

        if symbol_line + 1 < num_line || num_line + 1 < symbol_line {
            return false;
        }

        let num_start_col = self.start.column;
        let num_end_col = self.end.column;
        let symbol_col = position.column;

        symbol_col + 1 >= num_start_col && symbol_col <= num_end_col + 1
    }
}

fn parse_numbers(input: &str) -> Vec<NumberMatch> {
    let mut result = Vec::new();
    let mut start = None;
    let mut line = 0;
    let mut column = 0;
    for (i, c) in input.char_indices() {
        match (c.is_ascii_digit(), start) {
            (true, None) => start = Some((line, column, i)),
            (false, Some((start_line, start_column, start_index))) => {
                if let Ok(value) = input[start_index..i].parse() {
                    result.push(NumberMatch {
                        start: Position {
                            line: start_line,
                            column: start_column,
                        },
                        end: Position {
                            line,
                            column: column - 1,
                        },
                        value,
                    });
                }
                start = None;
            }
            _ => {}
        }
        if c == '\n' {
            line += 1;
            column = 0;
            continue;
        }
        column += 1;
    }
    if let Some((start_line, start_column, start_index)) = start {
        if let Ok(value) = input[start_index..].parse() {
            result.push(NumberMatch {
                start: Position {
                    line: start_line,
                    column: start_column,
                },
                end: Position { line, column },
                value,
            });
        }
    }
    result
}

fn find_special_chars(input: &str) -> Vec<(char, Position)> {
    let mut result = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if !c.is_ascii_digit() && c != '.' && c != '\n' {
                result.push((c, Position { line: i, column: j }));
            }
        }
    }
    result
}

pub fn task1() {
    let schematic = read_file_to_string("src/day3.txt").unwrap();
    let numbers = parse_numbers(&schematic);
    let special_chars = find_special_chars(&schematic);

    let part_num: i64 = numbers
        .iter()
        .filter(|num_match| {
            special_chars
                .iter()
                .any(|(_, pos)| num_match.is_adjacent_to(pos))
        })
        .map(|num_match| num_match.value)
        .sum();

    println!("Day  3, Task 1: {}", part_num);
}

pub fn task2() {
    let schematic = read_file_to_string("src/day3.txt").unwrap();
    let numbers = parse_numbers(&schematic);
    let special_chars = find_special_chars(&schematic);

    let sum: i64 = special_chars
        .iter()
        .filter(|(c, _)| *c == '*')
        .map(|(_, pos)| {
            numbers
                .iter()
                .filter(|num_match| num_match.is_adjacent_to(pos))
                .collect::<Vec<&NumberMatch>>()
        })
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers.into_iter().map(|num_match| num_match.value))
        .map(|numbers| numbers.product::<i64>())
        .sum();

    println!("Day  3, Task 2: {}", sum);
}
