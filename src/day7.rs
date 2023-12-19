use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardStrengh {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl std::convert::TryFrom<char> for CardStrengh {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use CardStrengh::*;
        match value {
            'A' => Ok(Ace),
            'K' => Ok(King),
            'Q' => Ok(Queen),
            'J' => Ok(Jack),
            'T' => Ok(Ten),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand(
    CardStrengh,
    CardStrengh,
    CardStrengh,
    CardStrengh,
    CardStrengh,
);

impl Hand {
    fn hand_type(&self) -> HandType {
        use HandType::*;
        let mut map = HashMap::new();
        map.entry(&self.0).or_insert(1);
        map.entry(&self.1)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
        map.entry(&self.2)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
        map.entry(&self.3)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
        map.entry(&self.4)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
        match map.len() {
            1 => FiveOfAKind,
            4 => OnePair,
            5 => HighCard,
            2 => match map.values().next().unwrap() {
                4 | 1 => FourOfAKind,
                _ => FullHouse,
            },
            _ => {
                if map.values().contains(&3) {
                    ThreeOfAKind
                } else {
                    TwoPairs
                }
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then_with(|| self.0.cmp(&other.0))
            .then_with(|| self.1.cmp(&other.1))
            .then_with(|| self.2.cmp(&other.2))
            .then_with(|| self.3.cmp(&other.3))
            .then_with(|| self.4.cmp(&other.4))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Bid {
    hand: Hand,
    bid: u32,
}

fn parse() -> Vec<Bid> {
    lines_from_file("src/day7.txt")
        .unwrap()
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> Bid {
    let mut iter = line.split_ascii_whitespace();
    let mut cards = iter.next().unwrap().chars();
    let card1 = CardStrengh::try_from(cards.next().unwrap()).unwrap();
    let card2 = CardStrengh::try_from(cards.next().unwrap()).unwrap();
    let card3 = CardStrengh::try_from(cards.next().unwrap()).unwrap();
    let card4 = CardStrengh::try_from(cards.next().unwrap()).unwrap();
    let card5 = CardStrengh::try_from(cards.next().unwrap()).unwrap();

    let bid = iter.next().unwrap().parse::<u32>().unwrap();

    Bid {
        hand: Hand(card1, card2, card3, card4, card5),
        bid,
    }
}

pub fn task1() -> crate::AOCResult<u32> {
    let mut bids = parse();
    bids.sort_by(|l, r| l.hand.cmp(&r.hand));
    let value: u32 = bids
        .iter()
        .enumerate()
        .map(|(index, bid)| (index as u32 + 1) * bid.bid)
        .sum();

    crate::AOCResult {
        day: 7,
        task: 1,
        r: value,
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardStrenghWithJoker {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl std::convert::TryFrom<char> for CardStrenghWithJoker {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use CardStrenghWithJoker::*;
        match value {
            'A' => Ok(Ace),
            'K' => Ok(King),
            'Q' => Ok(Queen),
            'J' => Ok(Jack),
            'T' => Ok(Ten),
            '9' => Ok(Nine),
            '8' => Ok(Eight),
            '7' => Ok(Seven),
            '6' => Ok(Six),
            '5' => Ok(Five),
            '4' => Ok(Four),
            '3' => Ok(Three),
            '2' => Ok(Two),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HandWithJoker(
    CardStrenghWithJoker,
    CardStrenghWithJoker,
    CardStrenghWithJoker,
    CardStrenghWithJoker,
    CardStrenghWithJoker,
);

impl HandWithJoker {
    fn hand_type(&self) -> HandType {
        use HandType::*;
        let mut map = HashMap::new();
        map.entry(&self.0).or_insert(1);
        map.entry(&self.1)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
        map.entry(&self.2)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
        map.entry(&self.3)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
        map.entry(&self.4)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);

        let jacks = map.remove(&CardStrenghWithJoker::Jack);
        if let Some(num_jacks) = jacks {
            match num_jacks {
                5 => return FiveOfAKind,
                _ => {
                    let max_entry = map.iter_mut().max_by_key(|e| *e.1).unwrap();
                    *max_entry.1 += num_jacks;
                }
            }
        }

        match map.len() {
            1 => FiveOfAKind,
            4 => OnePair,
            5 => HighCard,
            2 => match map.values().next().unwrap() {
                4 | 1 => FourOfAKind,
                _ => FullHouse,
            },
            _ => {
                if map.values().contains(&3) {
                    ThreeOfAKind
                } else {
                    TwoPairs
                }
            }
        }
    }
}

impl Ord for HandWithJoker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then_with(|| self.0.cmp(&other.0))
            .then_with(|| self.1.cmp(&other.1))
            .then_with(|| self.2.cmp(&other.2))
            .then_with(|| self.3.cmp(&other.3))
            .then_with(|| self.4.cmp(&other.4))
    }
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct BidWithJoker {
    hand: HandWithJoker,
    bid: u32,
}

fn parse2() -> Vec<BidWithJoker> {
    lines_from_file("src/day7.txt")
        .unwrap()
        .map(|line| parse_line2(&line))
        .collect()
}

fn parse_line2(line: &str) -> BidWithJoker {
    let mut iter = line.split_ascii_whitespace();
    let mut cards = iter.next().unwrap().chars();
    let card1 = CardStrenghWithJoker::try_from(cards.next().unwrap()).unwrap();
    let card2 = CardStrenghWithJoker::try_from(cards.next().unwrap()).unwrap();
    let card3 = CardStrenghWithJoker::try_from(cards.next().unwrap()).unwrap();
    let card4 = CardStrenghWithJoker::try_from(cards.next().unwrap()).unwrap();
    let card5 = CardStrenghWithJoker::try_from(cards.next().unwrap()).unwrap();

    let bid = iter.next().unwrap().parse::<u32>().unwrap();

    BidWithJoker {
        hand: HandWithJoker(card1, card2, card3, card4, card5),
        bid,
    }
}

pub fn task2() -> crate::AOCResult<u32> {
    let mut bids = parse2();
    bids.sort_by(|l, r| l.hand.cmp(&r.hand));
    let value: u32 = bids
        .iter()
        .enumerate()
        .map(|(index, bid)| (index as u32 + 1) * bid.bid)
        .sum();

    crate::AOCResult {
        day: 7,
        task: 2,
        r: value,
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_card_str_ord() {
        use CardStrengh::*;
        assert!(Two < Three);
    }

    #[test]
    fn test_hand_str_ord() {
        use CardStrengh::*;
        let mut hands = vec![
            Hand(Three, Two, Ten, Three, King),
            Hand(Ten, Five, Five, Jack, Five),
            Hand(King, King, Six, Seven, Seven),
            Hand(King, Ten, Jack, Jack, Ten),
            Hand(Queen, Queen, Queen, Jack, Ace),
        ];

        hands.sort();
        hands.reverse();

        assert_eq!(
            hands,
            vec![
                Hand(Queen, Queen, Queen, Jack, Ace),
                Hand(Ten, Five, Five, Jack, Five),
                Hand(King, King, Six, Seven, Seven),
                Hand(King, Ten, Jack, Jack, Ten),
                Hand(Three, Two, Ten, Three, King),
            ]
        );

        assert!(Hand(Jack, Jack, Jack, Jack, Jack) > Hand(Jack, Jack, King, Nine, Jack))
    }

    #[test]
    fn hand_cmp() {
        use HandType::*;
        assert!(FiveOfAKind >= FiveOfAKind);
        assert!(FiveOfAKind > FourOfAKind);
        assert!(FourOfAKind > FullHouse);
    }
}
