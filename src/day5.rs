use std::collections::BTreeMap;

pub struct RangeMap {
    map: BTreeMap<u64, u64>,
}

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
