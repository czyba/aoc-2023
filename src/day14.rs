use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Space {
    Empty,
    Rock,
    Wall,
}

fn parse() -> Vec<Vec<Space>> {
    lines_from_file("src/day14.txt")
        .unwrap()
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> Vec<Space> {
    line.chars()
        .map(|c| match c {
            '.' => Space::Empty,
            '#' => Space::Wall,
            'O' => Space::Rock,
            _ => panic!(),
        })
        .collect()
}

trait Tiltable {
    fn row_len(&self) -> usize;
    fn col_len(&self) -> usize;
    fn get(&self, row: usize, col: usize) -> Space;
    fn swap(&mut self, row1: usize, row2: usize, col: usize);
}

struct TiltNorth(Vec<Vec<Space>>);

impl Tiltable for TiltNorth {
    fn row_len(&self) -> usize {
        self.0.len()
    }

    fn col_len(&self) -> usize {
        self.0[0].len()
    }

    fn get(&self, row: usize, col: usize) -> Space {
        self.0[row][col]
    }

    fn swap(&mut self, row1: usize, row2: usize, col: usize) {
        let t = self.0[row1][col];
        self.0[row1][col] = self.0[row2][col];
        self.0[row2][col] = t;
    }
}

struct TiltWest(Vec<Vec<Space>>);

impl Tiltable for TiltWest {
    fn row_len(&self) -> usize {
        self.0[0].len()
    }

    fn col_len(&self) -> usize {
        self.0.len()
    }

    fn get(&self, row: usize, col: usize) -> Space {
        self.0[col][row]
    }

    fn swap(&mut self, row1: usize, row2: usize, col: usize) {
        self.0[col].swap(row1, row2)
    }
}

struct TiltSouth(Vec<Vec<Space>>);

impl Tiltable for TiltSouth {
    fn row_len(&self) -> usize {
        self.0.len()
    }

    fn col_len(&self) -> usize {
        self.0[0].len()
    }

    fn get(&self, row: usize, col: usize) -> Space {
        self.0[self.0.len() - 1 - row][col]
    }

    fn swap(&mut self, row1: usize, row2: usize, col: usize) {
        let start = self.0.len() - 1;
        let t = self.0[start - row1][col];
        self.0[start - row1][col] = self.0[start - row2][col];
        self.0[start - row2][col] = t;
    }
}

struct TiltEast(Vec<Vec<Space>>);

impl Tiltable for TiltEast {
    fn row_len(&self) -> usize {
        self.0[0].len()
    }

    fn col_len(&self) -> usize {
        self.0.len()
    }

    fn get(&self, row: usize, col: usize) -> Space {
        self.0[col][self.row_len() - 1 - row]
    }

    fn swap(&mut self, row1: usize, row2: usize, col: usize) {
        let start = self.row_len() - 1;
        self.0[col].swap(start - row1, start - row2);
    }
}

fn tilt<T: Tiltable>(tiltable: &mut T) {
    for row_index in 0..tiltable.row_len() {
        'loops: for column_index in 0..tiltable.col_len() {
            if tiltable.get(row_index, column_index) != Space::Rock {
                continue;
            }
            for swap_row_index in (0..row_index).rev() {
                if tiltable.get(swap_row_index, column_index) != Space::Empty {
                    tiltable.swap(swap_row_index + 1, row_index, column_index);
                    continue 'loops;
                }
            }
            tiltable.swap(0, row_index, column_index);
        }
    }
}

fn evaluate(platform: &Vec<Vec<Space>>) -> usize {
    let value = platform.len();
    platform
        .iter()
        .enumerate()
        .flat_map(|(index, row)| {
            row.iter()
                .filter(|&s| *s == Space::Rock)
                .map(move |_| value - index)
        })
        .sum()
}

pub fn task1() -> crate::AOCResult<usize> {
    let platform = parse();
    let mut tiltable = TiltNorth(platform);
    tilt(&mut tiltable);
    let result = evaluate(&tiltable.0);

    crate::AOCResult {
        day: 14,
        task: 1,
        r: result,
    }
}

fn tilt_ccw_circles(mut platform: Vec<Vec<Space>>, times: usize) -> Vec<Vec<Space>> {
    let mut cache = HashMap::new();
    let mut iterations = 0usize;
    while !cache.contains_key(&platform) {
        cache.insert(platform.clone(), iterations);
        iterations += 1;
        platform = tilt_ccw(platform);
    }
    let loop_start_index = *cache.get(&platform).unwrap();
    let loop_size = iterations - loop_start_index;
    let loop_steps = times - loop_start_index;
    let num_full_loops = loop_steps / loop_size;
    let loop_index = loop_steps - (num_full_loops * loop_size);

    let (platform, _) = cache
        .into_iter()
        .find(|(_, v)| *v == loop_start_index + loop_index)
        .unwrap();

    platform
}

fn tilt_ccw(platform: Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let mut tiltable = TiltNorth(platform);
    tilt(&mut tiltable);
    let mut tiltable = TiltWest(tiltable.0);
    tilt(&mut tiltable);
    let mut tiltable = TiltSouth(tiltable.0);
    tilt(&mut tiltable);
    let mut tiltable = TiltEast(tiltable.0);
    tilt(&mut tiltable);
    tiltable.0
}

pub fn task2() -> crate::AOCResult<usize> {
    let platform = parse();
    let platform = tilt_ccw_circles(platform, 1000000000);
    let result = evaluate(&platform);

    crate::AOCResult {
        day: 14,
        task: 2,
        r: result,
    }
}
