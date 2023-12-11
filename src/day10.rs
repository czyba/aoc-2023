use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Field {
    NE,
    NS,
    NW,
    ES,
    EW,
    SW,
    Start,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct State {
    line: usize,
    col: usize,
    direction: Direction,
}

impl State {
    fn next(self, field: Field) -> State {
        match (field, self.direction) {
            (Field::NE, Direction::North) => Self {
                line: self.line,
                col: self.col + 1,
                direction: Direction::West,
            },
            (Field::NE, Direction::East) => Self {
                line: self.line - 1,
                col: self.col,
                direction: Direction::South,
            },
            (Field::NS, Direction::North) => Self {
                line: self.line + 1,
                col: self.col,
                direction: Direction::North,
            },
            (Field::NS, Direction::South) => Self {
                line: self.line - 1,
                col: self.col,
                direction: Direction::South,
            },
            (Field::NW, Direction::North) => Self {
                line: self.line,
                col: self.col - 1,
                direction: Direction::East,
            },
            (Field::NW, Direction::West) => Self {
                line: self.line - 1,
                col: self.col,
                direction: Direction::South,
            },
            (Field::ES, Direction::East) => Self {
                line: self.line + 1,
                col: self.col,
                direction: Direction::North,
            },
            (Field::ES, Direction::South) => Self {
                line: self.line,
                col: self.col + 1,
                direction: Direction::West,
            },
            (Field::EW, Direction::East) => Self {
                line: self.line,
                col: self.col - 1,
                direction: Direction::East,
            },
            (Field::EW, Direction::West) => Self {
                line: self.line,
                col: self.col + 1,
                direction: Direction::West,
            },
            (Field::SW, Direction::South) => Self {
                line: self.line,
                col: self.col - 1,
                direction: Direction::East,
            },
            (Field::SW, Direction::West) => Self {
                line: self.line + 1,
                col: self.col,
                direction: Direction::North,
            },
            (Field::Start, _) => Self {
                line: self.line,
                col: self.col,
                direction: Direction::None,
            },
            (Field::None, _) => Self {
                line: self.line,
                col: self.col,
                direction: Direction::None,
            },
            _ => Self {
                line: self.line,
                col: self.col,
                direction: Direction::None,
            },
        }
    }
}

fn parse_line(line: &str) -> Vec<Field> {
    use Field::*;
    line.chars()
        .map(|c| match c {
            'L' => NE,
            '|' => NS,
            'J' => NW,
            'F' => ES,
            '-' => EW,
            '7' => SW,
            'S' => Start,
            _ => None,
        })
        .collect()
}

fn parse() -> Vec<Vec<Field>> {
    lines_from_file("src/day10.txt")
        .unwrap()
        .map(|l| parse_line(&l))
        .collect()
}

pub fn task1() {
    let grid = parse();

    let start_pos = grid
        .iter()
        .enumerate()
        .map(|(line, vec)| {
            vec.iter()
                .enumerate()
                .find(|(_, &connection)| connection == Field::Start)
                .map(|x| (line, x.0))
        })
        .find(Option::is_some)
        .flatten()
        .unwrap();

    let num_steps = find_start_or_loop_or_empty(
        &grid,
        State {
            line: start_pos.0 - 1,
            col: start_pos.1,
            direction: Direction::South,
        },
    )
    .or_else(|| {
        find_start_or_loop_or_empty(
            &grid,
            State {
                line: start_pos.0,
                col: start_pos.1 + 1,
                direction: Direction::West,
            },
        )
    })
    .or_else(|| {
        find_start_or_loop_or_empty(
            &grid,
            State {
                line: start_pos.0 + 1,
                col: start_pos.1,
                direction: Direction::North,
            },
        )
    })
    .or_else(|| {
        find_start_or_loop_or_empty(
            &grid,
            State {
                line: start_pos.0,
                col: start_pos.1 - 1,
                direction: Direction::East,
            },
        )
    })
    .unwrap();

    println!("Day 10, Task 1: {}", num_steps / 2);
}

fn find_start_or_loop_or_empty(grid: &[Vec<Field>], start_state: State) -> Option<usize> {
    let mut one_step = start_state;
    let mut two_step = start_state;
    two_step = two_step.next(grid[two_step.line][two_step.col]);

    let mut count_steps = 1;

    loop {
        one_step = one_step.next(grid[one_step.line][one_step.col]);
        two_step = two_step.next(grid[two_step.line][two_step.col]);
        two_step = two_step.next(grid[two_step.line][two_step.col]);
        let field = grid[one_step.line][one_step.col];
        count_steps += 1;
        if field == Field::Start {
            return Some(count_steps);
        }
        if field == Field::None {
            return None;
        }
        if one_step == two_step {
            return None;
        }
    }
}

pub fn task2() {
    let grid = parse();

    let start_pos = grid
        .iter()
        .enumerate()
        .map(|(line, vec)| {
            vec.iter()
                .enumerate()
                .find(|(_, &connection)| connection == Field::Start)
                .map(|x| (line, x.0))
        })
        .find(Option::is_some)
        .flatten()
        .unwrap();

    // Already know which one is the correct direction.
    let mut enclosure_grid = create_initial_enclosure_map(&grid, start_pos);
    calculate_outside_fields(&mut enclosure_grid);

    let mut count = 0;
    for line in 0..grid.len() {
        for col in 0..grid.len() {
            count += if enclosure_grid[line * 2 + 1][col * 2 + 1] == Enclosure::Unknown {
                1
            } else {
                0
            }
        }
    }
    println!("Day 10, Task 2: {}", count);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Enclosure {
    Unknown,
    Wall,
    Outside,
}

impl From<Field> for Enclosure {
    fn from(value: Field) -> Self {
        match value {
            Field::NE => Enclosure::Wall,
            Field::NS => Enclosure::Wall,
            Field::NW => Enclosure::Wall,
            Field::ES => Enclosure::Wall,
            Field::EW => Enclosure::Wall,
            Field::SW => Enclosure::Wall,
            Field::Start => Enclosure::Wall,
            Field::None => Enclosure::Unknown,
        }
    }
}

fn create_initial_enclosure_map(
    grid: &[Vec<Field>],
    start_pos: (usize, usize),
) -> Vec<Vec<Enclosure>> {
    let mut one_step = State {
        line: start_pos.0 + 1,
        col: start_pos.1,
        direction: Direction::North,
    };

    let mut enclosure = Vec::with_capacity(grid.len() * 2 + 3);
    for _ in 0..grid.len() * 2 + 3 {
        enclosure.push(vec![Enclosure::Unknown; grid.len() * 2 + 3]);
    }

    enclosure[start_pos.0 * 2 + 1][start_pos.1 * 2 + 1] = Enclosure::Wall;
    enclosure[start_pos.0 * 2 + 2][start_pos.1 * 2 + 1] = Enclosure::Wall;

    loop {
        enclosure[one_step.line * 2 + 1][one_step.col * 2 + 1] = Enclosure::Wall;

        let now = one_step;
        one_step = one_step.next(grid[one_step.line][one_step.col]);
        enclosure[now.line + one_step.line + 1][now.col + one_step.col + 1] = Enclosure::Wall;

        let field = grid[one_step.line][one_step.col];

        if field == Field::Start {
            return enclosure;
        }
    }
}

fn calculate_outside_fields(enclosure: &mut [Vec<Enclosure>]) {
    let mut worklist = Vec::new();
    worklist.push((0, 0));

    // Is Square
    let length = enclosure.len();

    while let Some((line, col)) = worklist.pop() {
        if enclosure[line][col] != Enclosure::Unknown {
            continue;
        }
        enclosure[line][col] = Enclosure::Outside;
        if line > 0 {
            worklist.push((line - 1, col));
        }
        if line < length - 1 {
            worklist.push((line + 1, col));
        }
        if col > 0 {
            worklist.push((line, col - 1));
        }
        if col < length - 1 {
            worklist.push((line, col + 1));
        }
    }
}
