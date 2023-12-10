use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

pub fn task1() {
    let configuration = Configuration {
        total_red: 12,
        total_green: 13,
        total_blue: 14,
    };
    let games: Vec<Game> = lines_from_file("src/day2.txt")
        .unwrap()
        .map(|l| line_to_game(&l))
        .collect();

    let sum = games
        .iter()
        .filter(|game| game.is_possible(&configuration))
        .map(|game| game.id)
        .fold(0, u32::wrapping_add);

    println!("{}", sum);
}

pub fn task2() {
    let games: Vec<Game> = lines_from_file("src/day2.txt")
        .unwrap()
        .map(|l| line_to_game(&l))
        .collect();

    let sum: u32 = games
        .iter()
        .map(|game| game.min_round())
        .map(|r| r.red * r.blue * r.green)
        .sum();

    println!("{}", sum);
}

fn line_to_game(line: &str) -> Game {
    let index = &line[5..];
    let mut iter = index.split(':');
    let id = iter.next().unwrap().parse::<u32>().unwrap();
    let games_str = iter.next().unwrap();
    let game_str_iter = games_str.split(';');
    let rounds = game_str_iter
        .map(str::trim)
        .map(substring_to_round)
        .collect();

    Game { id, rounds }
}

fn substring_to_round(substr: &str) -> Round {
    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    substr.split(',').for_each(|colors| {
        let mut iter = colors.split_ascii_whitespace();
        let num = iter.next().unwrap().parse::<u32>().unwrap();
        let color_str = iter.next().unwrap();
        if color_str == "red" {
            round.red = num;
        } else if color_str == "blue" {
            round.blue = num;
        } else if color_str == "green" {
            round.green = num;
        } else {
            print!("Logical error: {}", color_str);
        }
    });
    round
}

#[derive(Debug)]
struct Configuration {
    total_red: u32,
    total_blue: u32,
    total_green: u32,
}

#[derive(Debug)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

impl Round {
    pub fn is_possible(&self, configuration: &Configuration) -> bool {
        self.red <= configuration.total_red
            && self.blue <= configuration.total_blue
            && self.green <= configuration.total_green
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_possible(&self, configuration: &Configuration) -> bool {
        self.rounds
            .iter()
            .all(|round| round.is_possible(configuration))
    }

    fn min_round(&self) -> Round {
        let mut min_round = Round {
            red: 0,
            blue: 0,
            green: 0,
        };
        self.rounds.iter().for_each(|r| {
            if r.blue > min_round.blue {
                min_round.blue = r.blue;
            }
            if r.red > min_round.red {
                min_round.red = r.red;
            }
            if r.green > min_round.green {
                min_round.green = r.green;
            }
        });
        min_round
    }
}
