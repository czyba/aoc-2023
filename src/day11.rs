use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpaceType {
    Empty,
    Galaxy,
}

type Space = Vec<Vec<SpaceType>>;

fn parse_line(line: &str) -> Vec<SpaceType> {
    use SpaceType::*;
    line.chars()
        .map(|c| match c {
            '#' => Galaxy,
            '.' => Empty,
            x => panic!("Char {}", x),
        })
        .collect()
}

fn parse() -> Space {
    lines_from_file("src/day11.txt")
        .unwrap()
        .map(|l| parse_line(&l))
        .collect()
}

pub fn task1() -> crate::AOCResult<usize> {
    let mut space = parse();
    expand_rows(&mut space);
    expand_cols(&mut space);
    let galaxies = find_galaxies(&space);
    let sum_distance = calulate_distances(&galaxies);

    crate::AOCResult {
        day: 11,
        task: 1,
        r: sum_distance,
    }
}

fn calulate_distances(galaxies: &[(usize, usize)]) -> usize {
    let len = galaxies.len();

    let mut res = 0;

    for i in 0..len {
        let start = galaxies[i];
        for end in galaxies.iter().take(len).skip(i + 1) {
            let distance =
                start.0.max(end.0) - start.0.min(end.0) + start.1.max(end.1) - start.1.min(end.1);
            res += distance;
        }
    }
    res
}

fn find_galaxies(space: &Space) -> Vec<(usize, usize)> {
    space
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, t)| **t == SpaceType::Galaxy)
                .map(move |(col, _)| (row, col))
        })
        .collect()
}

fn expand_rows(space: &mut Space) {
    let mut len = space.len();
    let mut index = 0;
    while index < len {
        if space[index].iter().all(|t| *t == SpaceType::Empty) {
            space.insert(index, space[index].clone());
            index += 1;
            len += 1;
        }
        index += 1;
    }
}

fn expand_cols(space: &mut Space) {
    let mut len = space[0].len();
    let mut index = 0;
    while index < len {
        let mut all_empty = true;
        for row in space.iter() {
            all_empty &= row[index] == SpaceType::Empty;
        }

        if all_empty {
            for row in space.iter_mut() {
                row.insert(index, SpaceType::Empty);
            }

            index += 1;
            len += 1;
        }
        index += 1;
    }
}

pub fn task2() -> crate::AOCResult<u64> {
    let space = parse();
    let row_values = calculate_row_values(&space);
    let col_values = calculate_column_values(&space);
    let galaxies = find_galaxies(&space);
    let sum_distance = calulate_distances_2(&galaxies, row_values, col_values);

    crate::AOCResult {
        day: 11,
        task: 2,
        r: sum_distance,
    }
}

fn calulate_distances_2(
    galaxies: &[(usize, usize)],
    row_values: Vec<u64>,
    col_values: Vec<u64>,
) -> u64 {
    let len = galaxies.len();

    let mut res = 0;

    for i in 0..len {
        let start = galaxies[i];
        for end in galaxies.iter().take(len).skip(i + 1) {
            let mut distance: u64 = row_values[start.0.min(end.0)..start.0.max(end.0)]
                .iter()
                .sum();
            distance += col_values[start.1.min(end.1)..start.1.max(end.1)]
                .iter()
                .sum::<u64>();
            res += distance;
        }
    }
    res
}

fn calculate_row_values(space: &Space) -> Vec<u64> {
    space
        .iter()
        .map(|line| {
            if line.iter().all(|t| *t == SpaceType::Empty) {
                1000000
            } else {
                1
            }
        })
        .collect()
}

fn calculate_column_values(space: &Space) -> Vec<u64> {
    (0..space.len())
        .map(|i| {
            if space.iter().all(|line| line[i] == SpaceType::Empty) {
                1000000
            } else {
                1
            }
        })
        .collect()
}
