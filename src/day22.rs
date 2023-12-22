use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
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
                .or_insert_with(Vec::new)
                .push(brick.clone());
        }
    }
    map
}

struct HelperMaps {
    _layer_block_start_map: BTreeMap<usize, Vec<usize>>,
    layer_block_end_map: BTreeMap<usize, Vec<usize>>,
    supports_bricks_map: HashMap<usize, BTreeSet<usize>>,
}

fn calculate_supporting_layers(stack: &BTreeMap<usize, Vec<Brick>>) -> HelperMaps {
    let iter = stack.iter().rev();
    let mut supports_bricks_map = HashMap::new();
    let mut seen_bricks = HashSet::new();
    let mut layer_block_end_map = BTreeMap::new();
    let mut layer_block_start_map = BTreeMap::new();
    for (layer_num, layer_bricks) in iter {
        for brick in layer_bricks {
            if supports_bricks_map.contains_key(&brick.id) {
                continue;
            }
            let mut hs = BTreeSet::new();

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
            layer_block_end_map
                .entry(*layer_num)
                .or_insert_with(Vec::new)
                .push(brick_to_check.id);
            layer_block_start_map
                .entry(*layer_num - (brick_to_check.end.2 - brick_to_check.start.2))
                .or_insert_with(Vec::new)
                .push(brick_to_check.id);
            seen_bricks.insert(brick_to_check.id);
        }
    }
    HelperMaps {
        _layer_block_start_map: layer_block_start_map,
        layer_block_end_map,
        supports_bricks_map,
    }
}

fn calculate_disintigratable_bricks(
    layer_pillar_map: &BTreeMap<usize, Vec<usize>>,
    supports_bricks_map: &HashMap<usize, BTreeSet<usize>>,
) -> usize {
    let mut cnt = 0;

    for pillars in layer_pillar_map.values() {
        for brick_to_check in pillars {
            let mut hs: BTreeSet<usize> = BTreeSet::new();
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
    let helper_maps = calculate_supporting_layers(&stack);
    let r = calculate_disintigratable_bricks(
        &helper_maps.layer_block_end_map,
        &helper_maps.supports_bricks_map,
    );

    crate::AOCResult {
        day: 22,
        task: 1,
        r,
    }
}

fn calculate_disintigration_falling_map(
    stack: &BTreeMap<usize, Vec<Brick>>,
) -> HashMap<usize, usize> {
    let mut disintigrate_falling_map = HashMap::new();
    let iter = stack.iter().rev();
    let mut last_layer_falls: HashMap<BTreeSet<usize>, usize> = HashMap::new();
    last_layer_falls.insert(BTreeSet::new(), 0);

    for (layer, bricks) in iter {
        // println!("{:?}", last_layer_falls);

        // A vertical brick may support itself.
        let mut supports_bricks_map = HashMap::new();

        for brick in bricks {
            if supports_bricks_map.contains_key(&brick.id) {
                continue;
            }
            let mut supports_set = BTreeSet::new();

            if let Some(layer_above) = stack.get(&(layer + 1)) {
                for brick_above in layer_above {
                    if brick_above.overlaps_x_y(brick) {
                        supports_set.insert(brick_above.id);
                    }
                }
            }
            supports_bricks_map.insert(brick.id, supports_set);
        }

        let mut supports_itself = HashSet::new();
        // Calculate toppling blocks, if necessary
        for brick in bricks {
            if disintigrate_falling_map.contains_key(&brick.id) {
                supports_itself.insert(brick.id);
                continue;
            }
            let mut other_supports: BTreeSet<usize> = BTreeSet::new();
            for other in bricks {
                if brick.id == other.id {
                    continue;
                }
                other_supports.extend(supports_bricks_map.get(&other.id).unwrap());
            }
            let supports = supports_bricks_map.get(&brick.id).unwrap();
            let exclusive_supports = supports.difference(&other_supports).cloned().collect();
            let topples =
                last_layer_falls.get(&exclusive_supports).unwrap() + exclusive_supports.len();
            disintigrate_falling_map.insert(brick.id, topples);
        }

        // Add layer combinations
        let mut current_layer_falls: HashMap<BTreeSet<usize>, usize> = HashMap::new();
        for bit_mask in 0..(1usize << bricks.len()) {
            let mut falling_blocks = BTreeSet::new();
            let mut supports: BTreeSet<usize> = BTreeSet::new();
            let mut other_supports: BTreeSet<usize> = BTreeSet::new();

            for (bit_index, brick) in bricks.iter().enumerate() {
                if bit_mask & 1 << bit_index != 0 {
                    falling_blocks.insert(brick.id);
                    supports.extend(supports_bricks_map.get(&brick.id).unwrap());
                } else {
                    other_supports.extend(supports_bricks_map.get(&brick.id).unwrap());
                }
            }

            let exclusive_supports = supports.difference(&other_supports).cloned().collect();

            let topples = last_layer_falls.get(&exclusive_supports).unwrap()
                + exclusive_supports
                    .iter()
                    .map(|block| {
                        if supports_itself.contains(block) {
                            0
                        } else {
                            1
                        }
                    })
                    .sum::<usize>();
            current_layer_falls.insert(falling_blocks, topples);
        }

        last_layer_falls = current_layer_falls;
    }
    disintigrate_falling_map
}

pub fn task2() -> crate::AOCResult<usize> {
    let mut bricks = parse();
    bricks.sort_by(|l, r| {
        l.start
            .2
            .cmp(&r.start.2)
            .then_with(|| l.start.0.cmp(&r.start.0))
            .then_with(|| l.start.1.cmp(&r.start.1))
    });
    let stack = settle_bricks(&bricks);
    let m = calculate_disintigration_falling_map(&stack);
    let r = m.values().sum();

    crate::AOCResult {
        day: 22,
        task: 1,
        r,
    }
}
