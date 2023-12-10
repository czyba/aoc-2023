use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

trait Task1 {
    fn calculate_value(&self) -> u32;
}

trait Task2 {
    fn calculate_num_matches(&self) -> u32;
}

#[derive(Debug)]
struct Card {
    _id: u32,
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
}

impl Task1 for Card {
    fn calculate_value(&self) -> u32 {
        let num_matches = self
            .own_numbers
            .iter()
            .filter(|&num| self.winning_numbers.contains(num))
            .count();

        if num_matches == 0 {
            return 0;
        }

        let result: u32 = 2;

        result.pow(num_matches as u32 - 1)
    }
}

impl Task2 for Card {
    fn calculate_num_matches(&self) -> u32 {
        self.own_numbers
            .iter()
            .filter(|&num| self.winning_numbers.contains(num))
            .count() as u32
    }
}

fn parse_card(line: &str) -> Card {
    let mut iter = line.split(':');
    let id = iter
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let card_content = iter.next().unwrap();
    let (winning_numbers, own_numbers) = parse_card_content(card_content);
    Card {
        _id: id,
        winning_numbers,
        own_numbers,
    }
}

fn parse_card_content(card_content: &str) -> (Vec<u32>, Vec<u32>) {
    let mut iter = card_content.split('|').map(str::trim);

    (
        parse_numbers(iter.next().unwrap()),
        parse_numbers(iter.next().unwrap()),
    )
}

fn parse_numbers(numbers: &str) -> Vec<u32> {
    numbers
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

pub fn task1() {
    let value: u32 = lines_from_file("src/day4.txt")
        .unwrap()
        .map(|line| parse_card(&line))
        .map(|c| c.calculate_value())
        .sum();

    println!("{}", value);
}

pub fn task2() {
    let cards: Vec<Card> = lines_from_file("src/day4.txt")
        .unwrap()
        .map(|line| parse_card(&line))
        .collect();

    let mut card_counts = vec![1_u64; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let value = card.calculate_num_matches() as usize;
        for current_index in (index + 1)..=(usize::min(index + value, card_counts.len())) {
            card_counts[current_index] += card_counts[index];
        }
    }

    println!("{:?}", card_counts.iter().sum::<u64>());
}
