use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HotSpringStatus {
    Operational,
    Damaged,
    Unknown,
}

fn parse() -> Vec<(Vec<HotSpringStatus>, Vec<usize>)> {
    lines_from_file("src/day12.txt")
        .unwrap()
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> (Vec<HotSpringStatus>, Vec<usize>) {
    let mut iter = line.split_ascii_whitespace();
    let hss = iter
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '?' => HotSpringStatus::Unknown,
            '.' => HotSpringStatus::Operational,
            '#' => HotSpringStatus::Damaged,
            _ => panic!(),
        })
        .collect();

    let operationals = iter
        .next()
        .unwrap()
        .split(',')
        .map(|digits| digits.parse::<usize>().unwrap())
        .collect();

    (hss, operationals)
}

pub fn task1() {
    // 7541
    let data = parse();
    let count: i64 = data
        .iter()
        .map(|(hss, counts)| calculate_possibilites(hss, counts))
        .sum();
    // let count = brute_force(&data);
    println!("Day 12, Task 1: {}", count);
}

fn calculate_possibilites(hss: &[HotSpringStatus], combinations: &[usize]) -> i64 {
    use HotSpringStatus::*;

    let mut status_orders = Vec::new();
    for combination in combinations.iter() {
        status_orders.push(HotSpringStatus::Operational);
        status_orders.extend(vec![HotSpringStatus::Damaged; *combination]);
    }
    status_orders.push(HotSpringStatus::Operational);

    let mut last: Vec<i64> = vec![1];
    let mut next: Vec<i64> = vec![0; 2];

    for (index, status) in hss.iter().enumerate() {
        for j in 0..(index + 1).min(status_orders.len() - 1) {
            let prev = status_orders[j];
            let curr = status_orders[j + 1];

            match (prev, curr, *status) {
                (Operational, Damaged, Operational) => next[j] += last[j],
                (Operational, Damaged, Unknown) => {
                    next[j] += last[j];
                    next[j + 1] += last[j];
                }
                (_, Damaged, Damaged) => next[j + 1] += last[j],
                (_, Damaged, Unknown) => next[j + 1] += last[j],
                (Damaged, Operational, Operational) => {
                    if j + 2 == status_orders.len() {
                        next[j] += last[j];
                    } else {
                        next[j + 1] += last[j];
                    }
                }
                (Damaged, Operational, Unknown) => {
                    if j + 2 == status_orders.len() {
                        next[j] += last[j];
                    } else {
                        next[j + 1] += last[j];
                    }
                }
                _ => (),
            }
        }

        last = next;
        next = vec![0; (index + 3).min(status_orders.len())];
    }
    last[status_orders.len() - 2]
}

pub fn task2() {
    let data = parse();
    let count: i64 = data
        .iter()
        .map(|(hss, combinations)| {
            let mut extended_hss = hss.clone();
            (0..4).for_each(|_| {
                extended_hss.push(HotSpringStatus::Unknown);
                extended_hss.extend(hss.iter());
            });

            let mut extended_combinations = combinations.clone();
            (0..4).for_each(|_| {
                extended_combinations.extend(combinations);
            });

            (extended_hss, extended_combinations)
        })
        .map(|(hss, counts)| calculate_possibilites(&hss, &counts))
        .sum();

    println!("Day 12, Task 2: {}", count);
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    use HotSpringStatus::*;

    #[test]
    fn test() {
        assert_eq!(
            3,
            calculate_possibilites(&vec![Unknown, Unknown, Unknown, Unknown], &vec![1, 1])
        );

        assert_eq!(
            4,
            calculate_possibilites(
                &vec![
                    Operational,
                    Unknown,
                    Unknown,
                    Operational,
                    Operational,
                    Unknown,
                    Unknown,
                    Operational,
                    Operational,
                    Operational,
                    Unknown,
                    Damaged,
                    Damaged,
                ],
                &vec![1, 1, 3]
            )
        );

        assert_eq!(
            10,
            calculate_possibilites(
                &vec![
                    Unknown, Damaged, Damaged, Damaged, Unknown, Unknown, Unknown, Unknown,
                    Unknown, Unknown, Unknown, Unknown,
                ],
                &vec![3, 2, 1]
            )
        );

        // ??#??#????## 2,7
        assert_eq!(
            2,
            calculate_possibilites(
                &vec![
                    Unknown, Unknown, Damaged, Unknown, Unknown, Damaged, Unknown, Unknown,
                    Unknown, Unknown, Damaged, Damaged
                ],
                &vec![2, 7]
            )
        );
    }
}
