use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn count_possible_wins(&self) -> u32 {
        // sqrt(-d + (t/2)^2) + t/2
        let time_halved = (self.time as f64) / 2f64;
        let time_halved_squared = time_halved * time_halved;
        let distance = self.distance as f64;
        let root = (time_halved_squared - distance).sqrt();
        let lower_bound = time_halved - root;
        let ceil_lower = lower_bound.ceil();
        let upper_bound = time_halved + root;
        let floor_upper = upper_bound.floor();
        (floor_upper - ceil_lower) as u32 + 1
    }
}

fn parse() -> Vec<Race> {
    let mut lines = lines_from_file("src/day6.txt").unwrap();
    let times = parse_numbers(&lines.next().unwrap());
    let distance = parse_numbers(&lines.next().unwrap());

    times
        .into_iter()
        .zip(distance)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_numbers(line: &str) -> Vec<u64> {
    line.split_ascii_whitespace()
        .skip(1)
        .filter(|&num| !num.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

pub fn task1() {
    let races = parse();
    let possibilities: u32 = races.iter().map(Race::count_possible_wins).product();

    println!("Day  6, Task 1: {:?}", possibilities);
}

pub fn task2() {
    let race = Race {
        time: 54817088,
        distance: 446129210351007,
    };
    let possibilities = race.count_possible_wins();

    println!("Day  6, Task 2: {:?}", possibilities);
}
