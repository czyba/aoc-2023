use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

fn parse() -> Vec<String> {
    lines_from_file("src/day23.txt").unwrap().collect()
}

const H: u8 = 35;
const E: u8 = 46;
const D: u8 = 118;
const R: u8 = 62;

fn i(input: &[String], row: usize, col: usize) -> u8 {
    input[row].as_bytes()[col]
}

fn find_intersections(input: &[String]) -> HashSet<(usize, usize)> {
    let mut intersections = HashSet::new();
    for (row, s) in input.iter().enumerate() {
        for (col, s) in s.as_bytes().iter().enumerate() {
            if *s == H {
                continue;
            }
            let mut cnt = 0;
            if row > 0 && i(input, row - 1, col) != H {
                cnt += 1;
            }
            if col < input[0].len() - 1 && i(input, row, col + 1) != H {
                cnt += 1;
            }
            if row < input.len() - 1 && i(input, row + 1, col) != H {
                cnt += 1;
            }
            if col > 0 && i(input, row, col - 1) != H {
                cnt += 1;
            }
            if cnt != 2 {
                intersections.insert((row, col));
            }
        }
    }
    intersections
}

fn follow_path(
    input: &[String],
    intersections: &HashSet<(usize, usize)>,
    start: (usize, usize),
    mut last: (usize, usize),
) -> Option<((usize, usize), usize)> {
    let mut cur = start;
    let mut steps = 0;
    while !intersections.contains(&cur) {
        if cur.0 > 0
            && (cur.0 - 1, cur.1) != last
            && i(input, cur.0 - 1, cur.1) != H
            && i(input, cur.0, cur.1) == E
        {
            last = cur;
            cur = (cur.0 - 1, cur.1);
        } else if cur.1 < input[0].len() - 1
            && last != (cur.0, cur.1 + 1)
            && i(input, cur.0, cur.1 + 1) != H
            && (i(input, cur.0, cur.1) == E || i(input, cur.0, cur.1) == R)
        {
            last = cur;
            cur = (cur.0, cur.1 + 1);
        } else if cur.0 < input.len() - 1
            && last != (cur.0 + 1, cur.1)
            && i(input, cur.0 + 1, cur.1) != H
            && (i(input, cur.0, cur.1) == E || i(input, cur.0, cur.1) == D)
        {
            last = cur;
            cur = (cur.0 + 1, cur.1);
        } else if cur.1 > 0
            && last != (cur.0, cur.1 - 1)
            && i(input, cur.0, cur.1 - 1) != H
            && i(input, cur.0, cur.1) == E
        {
            last = cur;
            cur = (cur.0, cur.1 - 1);
        } else {
            return None;
        }
        steps += 1;
    }
    Some((cur, steps))
}

fn steps_between_intersections(
    input: &[String],
    intersections: &HashSet<(usize, usize)>,
) -> (
    HashMap<(usize, usize), HashSet<(usize, usize)>>,
    HashMap<((usize, usize), (usize, usize)), usize>,
) {
    let mut successors = HashMap::new();
    let mut steps_between_intersections = HashMap::new();
    for intersection in intersections {
        if intersection.0 > 0
            && i(input, intersection.0 - 1, intersection.1) != H
            && i(input, intersection.0, intersection.1) == E
        {
            if let Some(path) = follow_path(
                input,
                intersections,
                (intersection.0 - 1, intersection.1),
                *intersection,
            ) {
                successors
                    .entry(*intersection)
                    .or_insert_with(HashSet::new)
                    .insert(path.0);
                steps_between_intersections.insert((*intersection, path.0), path.1 + 1);
            }
        }
        if intersection.1 < input[0].len() - 1
            && i(input, intersection.0, intersection.1 + 1) != H
            && (i(input, intersection.0, intersection.1) == E
                || i(input, intersection.0, intersection.1) == R)
        {
            if let Some(path) = follow_path(
                input,
                intersections,
                (intersection.0, intersection.1 + 1),
                *intersection,
            ) {
                successors
                    .entry(*intersection)
                    .or_insert_with(HashSet::new)
                    .insert(path.0);
                steps_between_intersections.insert((*intersection, path.0), path.1 + 1);
            }
        }
        if intersection.0 < input.len() - 1
            && i(input, intersection.0 + 1, intersection.1) != H
            && (i(input, intersection.0, intersection.1) == E
                || i(input, intersection.0, intersection.1) == D)
        {
            if let Some(path) = follow_path(
                input,
                intersections,
                (intersection.0 + 1, intersection.1),
                *intersection,
            ) {
                successors
                    .entry(*intersection)
                    .or_insert_with(HashSet::new)
                    .insert(path.0);
                steps_between_intersections.insert((*intersection, path.0), path.1 + 1);
            }
        }
        if intersection.1 > 0
            && i(input, intersection.0, intersection.1 - 1) != H
            && i(input, intersection.0, intersection.1) == E
        {
            if let Some(path) = follow_path(
                input,
                intersections,
                (intersection.0, intersection.1 - 1),
                *intersection,
            ) {
                successors
                    .entry(*intersection)
                    .or_insert_with(HashSet::new)
                    .insert(path.0);
                steps_between_intersections.insert((*intersection, path.0), path.1 + 1);
            }
        }
    }
    (successors, steps_between_intersections)
}

fn find_all_paths(input: &[String], start: (usize, usize)) -> usize {
    let intersections = find_intersections(input);
    let (successors, steps_between_intersections) =
        steps_between_intersections(input, &intersections);
    let mut start_path = HashSet::new();
    start_path.insert(start);

    let mut worklist = vec![(start_path, start, 0)];

    let mut max_steps = 0;

    while let Some((path, intersection, steps)) = worklist.pop() {
        for succ in successors.get(&intersection).unwrap() {
            if path.contains(succ) {
                continue;
            }
            let steps = steps
                + steps_between_intersections
                    .get(&(intersection, *succ))
                    .unwrap();
            if succ.0 == input.len() - 1 {
                max_steps = max_steps.max(steps);
                continue;
            }
            let mut new_path = path.clone();
            new_path.insert(*succ);
            worklist.push((new_path, *succ, steps));
        }
    }
    max_steps
}

pub fn task1() -> crate::AOCResult<usize> {
    let input = parse();
    let start = (0, 1);
    let r = find_all_paths(&input, start);

    crate::AOCResult {
        day: 23,
        task: 1,
        r,
    }
}

pub fn task2() -> crate::AOCResult<usize> {
    let mut input = parse();
    let start = (0, 1);
    input.iter_mut().for_each(|s| {
        *s = s.replace(['>', 'v'], ".");
    });
    let r = find_all_paths(&input, start);

    crate::AOCResult {
        day: 23,
        task: 2,
        r,
    }
}
