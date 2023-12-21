use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

pub fn parse(filename: &str) -> Vec<String> {
    lines_from_file(filename).unwrap().collect()
}

fn get_starting_pos(input: &mut [String]) -> (usize, usize) {
    for (row, s) in input.iter_mut().enumerate() {
        if let Some((col, _)) = s.chars().enumerate().find(|(_, c)| *c == 'S') {
            unsafe {
                s.as_bytes_mut()[col] = ".".as_bytes()[0];
            }
            return (row, col);
        }
    }
    panic!()
}

#[derive(Debug, PartialEq, Eq)]
struct MinDistancePos {
    pos: (usize, usize),
    distance: usize,
}

impl std::cmp::Ord for MinDistancePos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl std::cmp::PartialOrd for MinDistancePos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_paths_bounded(
    input: &[String],
    starting_pos: (usize, usize),
    max_steps: usize,
) -> HashMap<(usize, usize), usize> {
    let mut shortest_paths = HashMap::new();
    let mut bheap = BinaryHeap::new();
    bheap.push(MinDistancePos {
        pos: starting_pos,
        distance: 0,
    });
    shortest_paths.insert(starting_pos, 0usize);
    let dot = ".".as_bytes()[0];
    while let Some(min_pos) = bheap.pop() {
        if min_pos.distance >= max_steps {
            continue;
        }
        let pos = min_pos.pos;
        if pos.0 > 0
            && !shortest_paths.contains_key(&(pos.0 - 1, pos.1))
            && input[pos.0 - 1].as_bytes()[pos.1] == dot
        {
            shortest_paths.insert((pos.0 - 1, pos.1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0 - 1, pos.1),
                distance: min_pos.distance + 1,
            });
        }
        if pos.1 > 0
            && !shortest_paths.contains_key(&(pos.0, pos.1 - 1))
            && input[pos.0].as_bytes()[pos.1 - 1] == dot
        {
            shortest_paths.insert((pos.0, pos.1 - 1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0, pos.1 - 1),
                distance: min_pos.distance + 1,
            });
        }
        if pos.0 < input.len() - 1
            && !shortest_paths.contains_key(&(pos.0 + 1, pos.1))
            && input[pos.0 + 1].as_bytes()[pos.1] == dot
        {
            shortest_paths.insert((pos.0 + 1, pos.1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0 + 1, pos.1),
                distance: min_pos.distance + 1,
            });
        }
        if pos.1 < input[0].len() - 1
            && !shortest_paths.contains_key(&(pos.0, pos.1 + 1))
            && input[pos.0].as_bytes()[pos.1 + 1] == dot
        {
            shortest_paths.insert((pos.0, pos.1 + 1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0, pos.1 + 1),
                distance: min_pos.distance + 1,
            });
        }
    }
    shortest_paths
}

fn shortest_paths(
    input: &[String],
    starting_pos: (usize, usize),
) -> HashMap<(usize, usize), usize> {
    let mut shortest_paths = HashMap::new();
    let mut bheap = BinaryHeap::new();
    bheap.push(MinDistancePos {
        pos: starting_pos,
        distance: 0,
    });
    shortest_paths.insert(starting_pos, 0usize);
    let dot = ".".as_bytes()[0];
    while let Some(min_pos) = bheap.pop() {
        let pos = min_pos.pos;
        if pos.0 > 0
            && !shortest_paths.contains_key(&(pos.0 - 1, pos.1))
            && input[pos.0 - 1].as_bytes()[pos.1] == dot
        {
            shortest_paths.insert((pos.0 - 1, pos.1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0 - 1, pos.1),
                distance: min_pos.distance + 1,
            });
        }
        if pos.1 > 0
            && !shortest_paths.contains_key(&(pos.0, pos.1 - 1))
            && input[pos.0].as_bytes()[pos.1 - 1] == dot
        {
            shortest_paths.insert((pos.0, pos.1 - 1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0, pos.1 - 1),
                distance: min_pos.distance + 1,
            });
        }
        if pos.0 < input.len() - 1
            && !shortest_paths.contains_key(&(pos.0 + 1, pos.1))
            && input[pos.0 + 1].as_bytes()[pos.1] == dot
        {
            shortest_paths.insert((pos.0 + 1, pos.1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0 + 1, pos.1),
                distance: min_pos.distance + 1,
            });
        }
        if pos.1 < input[0].len() - 1
            && !shortest_paths.contains_key(&(pos.0, pos.1 + 1))
            && input[pos.0].as_bytes()[pos.1 + 1] == dot
        {
            shortest_paths.insert((pos.0, pos.1 + 1), min_pos.distance + 1);
            bheap.push(MinDistancePos {
                pos: (pos.0, pos.1 + 1),
                distance: min_pos.distance + 1,
            });
        }
    }
    shortest_paths
}

pub fn task1() -> crate::AOCResult<usize> {
    let mut input = parse("src/day21.txt");
    let starting_pos = get_starting_pos(&mut input);
    let shortes_paths = shortest_paths_bounded(&input, starting_pos, 64);
    let r = shortes_paths
        .iter()
        .filter(|(pos, _)| {
            ((pos.0.max(starting_pos.0) - pos.0.min(starting_pos.0))
                + (pos.1.max(starting_pos.1) - pos.1.min(starting_pos.1)))
                % 2
                == 64 % 2
        })
        .count();

    crate::AOCResult {
        day: 21,
        task: 1,
        r,
    }
}

fn _print_distances(input: &[String], distances: &HashMap<(usize, usize), usize>) {
    use std::fmt::Write;
    let mut s = String::new();
    for (row, line) in input.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            s.push(' ');
            if char == '.' {
                if let Some(distance) = distances.get(&(row, col)) {
                    write!(&mut s, "{:3}", distance).unwrap();
                } else {
                    s.push_str("   ");
                }
            } else {
                s.push_str("###");
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn transform_to_distance_map(distances: HashMap<(usize, usize), usize>) -> BTreeMap<usize, usize> {
    let map = distances
        .iter()
        .fold(BTreeMap::new(), |mut acc, (_, distance)| {
            acc.entry(*distance)
                .and_modify(|i| *i += 1)
                .or_insert(1usize);
            acc
        });

    let mut res = BTreeMap::new();

    for (steps, mut count) in map {
        if steps > 1 {
            count += res.get(&(steps - 2)).unwrap();
        }
        res.insert(steps, count);
    }

    res
}

#[derive(Debug)]
struct Number {
    straight_odds: usize,
    straight_evens: usize,
    diagonal_odds: usize,
    diagonal_evens: usize,
}

fn numbers(num_full_size: usize, num_steps: usize) -> Number {
    match (num_full_size % 2 == 0, num_steps % 2 == 0) {
        (true, true) => {
            let diagonal_lines = num_full_size - 2;
            let h = diagonal_lines / 2;
            Number {
                straight_odds: num_full_size / 2 - 1,
                straight_evens: num_full_size / 2,
                diagonal_evens: h * h,
                diagonal_odds: h * (h + 1),
            }
        }
        (true, false) => {
            let diagonal_lines = num_full_size - 2;
            let h = diagonal_lines / 2;
            Number {
                straight_odds: num_full_size / 2,
                straight_evens: num_full_size / 2 - 1,
                diagonal_evens: h * (h + 1),
                diagonal_odds: h * h,
            }
        }
        (false, true) => {
            let diagonal_lines = num_full_size - 2;
            let h = (diagonal_lines - 1) / 2;
            Number {
                straight_odds: (num_full_size - 1) / 2,
                straight_evens: (num_full_size - 1) / 2,
                diagonal_evens: h * (h + 1) + 1,
                diagonal_odds: h * (h + 1),
            }
        }
        (false, false) => {
            let diagonal_lines = num_full_size - 2;
            let h = (diagonal_lines - 1) / 2;
            Number {
                straight_odds: (num_full_size - 1) / 2,
                straight_evens: (num_full_size - 1) / 2,
                diagonal_evens: h * (h + 1),
                diagonal_odds: h * (h + 1) + 1,
            }
        }
    }
}

fn get_max_(map: &BTreeMap<usize, usize>) -> (usize, usize) {
    let max = map.iter().max_by_key(|(distance, _)| *distance).unwrap();
    let pre_max = map.get(&(max.0 - 1)).unwrap();
    if max.0 % 2 == 0 {
        (*pre_max, *max.1)
    } else {
        (*max.1, *pre_max)
    }
}

fn calculate_steps_large(input: &Vec<String>, num_steps: usize) -> usize {
    /*
     * Assumptions for input:
     *  1. Start is directly in center
     *  2. In the center there's no rocks vertically or horizontally
     *  3. The outer row and column is also rock-free
     *  4. The input is a square
     *  5. The size is odd
     */
    let len = input.len();
    let mid = len / 2;
    let mut count = 0;

    // TODO: Note that there are some errors when remaining top == input.len();
    let num_full_size = num_steps / len;
    let remaining_top = num_steps - num_full_size * len + mid;
    let remaining_top_corner = num_steps - num_full_size * len - 1;
    let number_full_tiles = numbers(num_full_size, num_steps);

    {
        let distances_from_bottom =
            transform_to_distance_map(shortest_paths(input, (len - 1, mid)));
        count += *distances_from_bottom.get(&remaining_top).unwrap();
        if remaining_top >= len {
            count += *distances_from_bottom.get(&(remaining_top - len)).unwrap();
        }
        let (odd_cnt, even_cnt) = get_max_(&distances_from_bottom);
        count +=
            number_full_tiles.straight_odds * odd_cnt + number_full_tiles.straight_evens * even_cnt;

        let distances_from_bottom_right =
            transform_to_distance_map(shortest_paths(input, (len - 1, len - 1)));
        let size_top_corner = *distances_from_bottom_right
            .get(&remaining_top_corner)
            .unwrap_or(&0);
        let size_top_corner_large = *distances_from_bottom_right
            .get(&(remaining_top_corner + len))
            .unwrap();
        count += size_top_corner * num_full_size + size_top_corner_large * (num_full_size - 1);
        let (odd_cnt, even_cnt) = get_max_(&distances_from_bottom_right);
        count +=
            odd_cnt * number_full_tiles.diagonal_odds + even_cnt * number_full_tiles.diagonal_evens;
    }

    {
        let distance_from_left = transform_to_distance_map(shortest_paths(input, (mid, 0)));
        count += *distance_from_left.get(&remaining_top).unwrap();
        if remaining_top >= len {
            count += *distance_from_left.get(&(remaining_top - len)).unwrap();
        }
        let (odd_cnt, even_cnt) = get_max_(&distance_from_left);
        count +=
            number_full_tiles.straight_odds * odd_cnt + number_full_tiles.straight_evens * even_cnt;

        let distances_from_bottom_left =
            transform_to_distance_map(shortest_paths(input, (len - 1, 0)));
        let size_top_corner = *distances_from_bottom_left
            .get(&remaining_top_corner)
            .unwrap_or(&0);
        let size_top_corner_large = *distances_from_bottom_left
            .get(&(remaining_top_corner + len))
            .unwrap();
        count += size_top_corner * num_full_size + size_top_corner_large * (num_full_size - 1);
        let (odd_cnt, even_cnt) = get_max_(&distances_from_bottom_left);
        count +=
            odd_cnt * number_full_tiles.diagonal_odds + even_cnt * number_full_tiles.diagonal_evens;
    }

    {
        let distance_from_top = transform_to_distance_map(shortest_paths(input, (0, mid)));
        count += *distance_from_top.get(&remaining_top).unwrap();
        if remaining_top >= len {
            count += *distance_from_top.get(&(remaining_top - len)).unwrap();
        }
        let (odd_cnt, even_cnt) = get_max_(&distance_from_top);
        count +=
            number_full_tiles.straight_odds * odd_cnt + number_full_tiles.straight_evens * even_cnt;

        let distance_from_top_left = transform_to_distance_map(shortest_paths(input, (0, 0)));
        let size_top_corner = *distance_from_top_left
            .get(&remaining_top_corner)
            .unwrap_or(&0);
        let size_top_corner_large = *distance_from_top_left
            .get(&(remaining_top_corner + len))
            .unwrap();
        count += size_top_corner * num_full_size + size_top_corner_large * (num_full_size - 1);
        let (odd_cnt, even_cnt) = get_max_(&distance_from_top_left);
        count +=
            odd_cnt * number_full_tiles.diagonal_odds + even_cnt * number_full_tiles.diagonal_evens;
    }

    {
        let distance_from_right = transform_to_distance_map(shortest_paths(input, (mid, len - 1)));
        count += *distance_from_right.get(&remaining_top).unwrap();
        if remaining_top >= len {
            count += *distance_from_right.get(&(remaining_top - len)).unwrap();
        }
        let (odd_cnt, even_cnt) = get_max_(&distance_from_right);
        count +=
            number_full_tiles.straight_odds * odd_cnt + number_full_tiles.straight_evens * even_cnt;

        let distance_from_top_right =
            transform_to_distance_map(shortest_paths(input, (0, len - 1)));
        let size_top_corner = *distance_from_top_right
            .get(&remaining_top_corner)
            .unwrap_or(&0);
        let size_top_corner_large = *distance_from_top_right
            .get(&(remaining_top_corner + len))
            .unwrap();
        count += size_top_corner * num_full_size + size_top_corner_large * (num_full_size - 1);
        let (odd_cnt, even_cnt) = get_max_(&distance_from_top_right);
        count +=
            odd_cnt * number_full_tiles.diagonal_odds + even_cnt * number_full_tiles.diagonal_evens;
    }

    // Center Tile
    let distance_from_center = transform_to_distance_map(shortest_paths(input, (mid, mid)));
    let (odd_cnt, even_cnt) = get_max_(&distance_from_center);
    count += if num_steps % 2 == 0 {
        even_cnt
    } else {
        odd_cnt
    };

    count
}

pub fn task2() -> crate::AOCResult<usize> {
    let steps = 26501365;

    let mut input = parse("src/day21.txt");
    // Remove middle S;
    get_starting_pos(&mut input);
    let r = calculate_steps_large(&input, steps);

    crate::AOCResult {
        day: 21,
        task: 2,
        r,
    }
}
