use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.expect("Could not parse line")))
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    id: usize,
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    fn overlaps_x_y(&self, other: &Brick) -> bool {
        self.start.0 <= other.end.0
            && other.start.0 <= self.end.0
            && self.start.1 <= other.end.1
            && other.start.1 <= self.end.1
    }
}

fn parse() -> Vec<Brick> {
    lines_from_file("src/day22.txt")
        .unwrap()
        .enumerate()
        .map(|(index, line)| parse_brick(&line, index))
        .collect()
}

fn parse_brick(line: &str, id: usize) -> Brick {
    let mut iter = line.split('~');
    let mut iter2 = iter.next().unwrap().split(',');
    let start = (
        iter2.next().unwrap().parse::<usize>().unwrap(),
        iter2.next().unwrap().parse::<usize>().unwrap(),
        iter2.next().unwrap().parse::<usize>().unwrap(),
    );
    let mut iter2 = iter.next().unwrap().split(',');
    let end = (
        iter2.next().unwrap().parse::<usize>().unwrap(),
        iter2.next().unwrap().parse::<usize>().unwrap(),
        iter2.next().unwrap().parse::<usize>().unwrap(),
    );
    Brick { id, start, end }
}

fn settle_bricks(snapshot: &Vec<Brick>) -> BTreeMap<usize, Vec<Brick>> {
    let mut map = BTreeMap::new();
    for brick in snapshot {
        let mut index = 0;
        'stack: for (k, v) in map.iter().rev() {
            for last_brick in v {
                if brick.overlaps_x_y(last_brick) {
                    index = k + 1;
                    break 'stack;
                }
            }
        }
        for i in 0..(brick.end.2 - brick.start.2 + 1) {
            map.entry(index + i)
                .or_insert_with(|| vec![brick.clone()])
                .push(brick.clone());
        }
    }
    map
}

fn calculate_supporting_layers(
    stack: &BTreeMap<usize, Vec<Brick>>,
) -> (BTreeMap<usize, Vec<usize>>, HashMap<usize, HashSet<usize>>) {
    let iter = stack.iter().rev();
    let mut supports_bricks_map = HashMap::new();
    let mut seen_bricks = HashSet::new();
    let mut layer_pillar_map = BTreeMap::new();
    for (layer_num, layer_bricks) in iter {
        for brick in layer_bricks {
            if supports_bricks_map.contains_key(&brick.id) {
                continue;
            }
            let mut hs = HashSet::new();

            if let Some(layer_above) = stack.get(&(layer_num + 1)) {
                for brick_above in layer_above {
                    if brick_above.overlaps_x_y(brick) {
                        hs.insert(brick_above.id);
                    }
                }
            }
            supports_bricks_map.insert(brick.id, hs);
        }

        for brick_to_check in layer_bricks {
            if seen_bricks.contains(&brick_to_check.id) {
                continue;
            }
            layer_pillar_map
                .entry(*layer_num)
                .or_insert_with(Vec::new)
                .push(brick_to_check.id);
            seen_bricks.insert(brick_to_check.id);
        }
    }
    (layer_pillar_map, supports_bricks_map)
}

fn calculate_disintigratable_bricks(
    layer_pillar_map: &BTreeMap<usize, Vec<usize>>,
    supports_bricks_map: &HashMap<usize, HashSet<usize>>,
) -> usize {
    let mut cnt = 0;

    for pillars in layer_pillar_map.values() {
        for brick_to_check in pillars {
            let mut hs: HashSet<usize> = HashSet::new();
            for other in pillars {
                if brick_to_check == other {
                    continue;
                }
                hs.extend(supports_bricks_map.get(other).unwrap());
            }
            let a = supports_bricks_map.get(brick_to_check).unwrap();
            if a.difference(&hs).count() == 0 {
                cnt += 1;
            }
        }
    }

    cnt
}

pub fn task1() -> crate::AOCResult<usize> {
    let mut bricks = parse();
    bricks.sort_by(|l, r| {
        l.start
            .2
            .cmp(&r.start.2)
            .then_with(|| l.start.0.cmp(&r.start.0))
            .then_with(|| l.start.1.cmp(&r.start.1))
    });
    let stack = settle_bricks(&bricks);
    let (layer_pillar_map, supports_bricks_map) = calculate_supporting_layers(&stack);
    let r = calculate_disintigratable_bricks(&layer_pillar_map, &supports_bricks_map);

    crate::AOCResult {
        day: 22,
        task: 1,
        r,
    }
}
