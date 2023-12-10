use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::Read;
use std::io::Result;

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
}
