use itertools::Itertools;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::Read;
use std::io::Result;
use std::ops::Range;

struct ParsedInput {
    seeds: Vec<u64>,
    seed_soil_map: RangeMap,
    soil_fertilizer_map: RangeMap,
    fertilizer_waper_map: RangeMap,
    water_light_map: RangeMap,
    light_temperator_map: RangeMap,
    temperature_humidity_map: RangeMap,
    humidity_location_map: RangeMap,
}

impl ParsedInput {
    fn translate_seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_soil_map.map(seed);
        let fertilizer = self.soil_fertilizer_map.map(soil);
        let water = self.fertilizer_waper_map.map(fertilizer);
        let light = self.water_light_map.map(water);
        let temperature = self.light_temperator_map.map(light);
        let humidity = self.temperature_humidity_map.map(temperature);
        self.humidity_location_map.map(humidity)
    }

    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        self.seed_soil_map
            .map_range(range)
            .iter()
            .flat_map(|r| self.soil_fertilizer_map.map_range(r))
            .flat_map(|r| self.fertilizer_waper_map.map_range(&r))
            .flat_map(|r| self.water_light_map.map_range(&r))
            .flat_map(|r| self.light_temperator_map.map_range(&r))
            .flat_map(|r| self.temperature_humidity_map.map_range(&r))
            .flat_map(|r| self.humidity_location_map.map_range(&r))
            .collect()
    }
}

pub struct RangeMap {
    map: BTreeMap<u64, u64>,
}

#[cfg(windows)]
const DOUBLE_LINE_ENDING: &str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_LINE_ENDING: &str = "\n\n";

impl RangeMap {
    fn new() -> Self {
        let mut map = BTreeMap::new();
        map.insert(0, 0);
        Self { map }
    }

    fn add_range(&mut self, start: u64, size: u64, mapped_start: u64) {
        self.map.insert(start, mapped_start);
        let end = start + size;
        self.map.entry(end).or_insert(end);
    }

    fn map(&self, value: u64) -> u64 {
        let entry = self
            .map
            .iter()
            .filter(|(&key, _)| key <= value)
            .last()
            .unwrap();

        (entry.1 + value) - entry.0
    }

    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let mut r = Vec::new();

        let mut current_value = range.start;
        let mut iter = self.map.iter();
        let mut last_entry = iter.next().unwrap();
        for current_entry in iter {
            if *current_entry.0 < range.start {
                last_entry = current_entry;
                continue;
            }
            let start = current_value;
            let end = u64::min(range.end, *current_entry.0);
            r.push(Range {
                start: (last_entry.1 + start) - last_entry.0,
                end: (last_entry.1 + end) - last_entry.0,
            });
            last_entry = current_entry;
            current_value = end;
            if current_value >= range.end {
                return r;
            }
        }

        let start = current_value;
        let end = range.end;
        r.push(Range {
            start: (last_entry.1 + start) - last_entry.0,
            end: (last_entry.1 + end) - last_entry.0,
        });

        r
    }
}

fn read_file_to_string(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse() -> ParsedInput {
    let text = read_file_to_string("src/day5.txt").unwrap();
    let mut splits = text.split(DOUBLE_LINE_ENDING);
    let seeds = parse_seeds(splits.next().unwrap().lines().next().unwrap());
    let seed_soil_map = parse_map(splits.next().unwrap());
    let soil_fertilizer_map = parse_map(splits.next().unwrap());
    let fertilizer_waper_map = parse_map(splits.next().unwrap());
    let water_light_map = parse_map(splits.next().unwrap());
    let light_temperator_map = parse_map(splits.next().unwrap());
    let temperature_humidity_map = parse_map(splits.next().unwrap());
    let humidity_location_map = parse_map(splits.next().unwrap());
    ParsedInput {
        seeds,
        seed_soil_map,
        soil_fertilizer_map,
        fertilizer_waper_map,
        water_light_map,
        light_temperator_map,
        temperature_humidity_map,
        humidity_location_map,
    }
}

fn parse_seeds(seed_line: &str) -> Vec<u64> {
    seed_line[7..]
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_map(map_str: &str) -> RangeMap {
    let mut rm = RangeMap::new();
    let iter = map_str.lines().skip(1);
    for line in iter {
        let mut line_iter = line.split_ascii_whitespace();
        let mapped_start = line_iter.next().unwrap().parse::<u64>().unwrap();
        let start = line_iter.next().unwrap().parse::<u64>().unwrap();
        let size = line_iter.next().unwrap().parse::<u64>().unwrap();
        rm.add_range(start, size, mapped_start);
    }

    rm
}

pub fn task1() {
    let parsed_input = parse();
    let min_location = parsed_input
        .seeds
        .iter()
        .map(|seed| parsed_input.translate_seed_to_location(*seed))
        .min()
        .unwrap();
    println!("Day 5, Task 1: {}", min_location);
}

pub fn task2() {
    let parsed_input = parse();

    let min_location = parsed_input
        .seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = *chunk.next().unwrap();
            let size = chunk.next().unwrap();
            Range {
                start,
                end: start + size,
            }
        })
        .flat_map(|range| parsed_input.map_range(&range))
        .map(|range| range.start)
        .min()
        .unwrap();

    println!("Day 5, Task 2: {}", min_location);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let mut rm = RangeMap::new();
        rm.add_range(98, 2, 50);
        rm.add_range(50, 48, 52);

        assert_eq!(10, rm.map(10));
        assert_eq!(49, rm.map(49));
        assert_eq!(50, rm.map(98));
        assert_eq!(51, rm.map(99));
        assert_eq!(52, rm.map(50));
        assert_eq!(55, rm.map(53));
    }

    #[test]
    fn test_range() {
        let mut rm = RangeMap::new();
        rm.add_range(10, 10, 110);
        rm.add_range(90, 10, 10);

        let mut mapped = rm.map_range(&Range { start: 0, end: 110 });
        mapped.sort_by(|l, r| l.start.cmp(&r.start));

        assert_eq!(
            mapped,
            vec![
                Range { start: 0, end: 10 },
                Range { start: 10, end: 20 },
                Range { start: 20, end: 90 },
                Range {
                    start: 100,
                    end: 110
                },
                Range {
                    start: 110,
                    end: 120
                },
            ]
        );

        let mapped = rm.map_range(&Range { start: 13, end: 17 });
        assert_eq!(
            mapped,
            vec![Range {
                start: 113,
                end: 117
            }]
        )
    }
}
