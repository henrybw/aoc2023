use itertools::Itertools;
use rangemap::RangeMap;
use std::collections::HashMap;
use std::ops::Range;
use std::str::Lines;

#[derive(Debug)]
struct CategoryMap {
    source: String,
    dest: String,
    map: RangeMap<u64, Range<u64>>,
}

impl CategoryMap {
    fn new(lines: &mut Lines) -> Option<Self> {
        let header: Vec<_> = lines.next()?.split_whitespace().collect();
        assert!(header[1] == "map:");
        let (source, dest) =
            header[0].split("-to-").map(String::from).collect_tuple()?;

        let mut map = RangeMap::new();
        lines.take_while(|line| !line.is_empty()).for_each(|line| {
            let nums: Vec<u64> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            assert!(nums.len() == 3);
            map.insert(nums[1]..nums[1] + nums[2], nums[0]..nums[0] + nums[2]);
        });

        Some(Self { source, dest, map })
    }

    fn convert(&self, source: u64) -> u64 {
        if self.map.contains_key(&source) {
            let (source_range, dest_range) =
                self.map.get_key_value(&source).unwrap();
            dest_range.start + (source - source_range.start)
        } else {
            source
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<(String, String), CategoryMap>,
}

impl Almanac {
    fn new(input: String) -> Self {
        let mut lines = input.lines();

        let seeds = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        lines.next();

        let mut maps = HashMap::new();
        while let Some(map) = CategoryMap::new(&mut lines) {
            maps.insert((map.source.clone(), map.dest.clone()), map);
        }

        Self { seeds, maps }
    }

    fn convert(&self, dest: &str, source: &str, value: u64) -> u64 {
        self.maps
            .get(&(source.to_string(), dest.to_string()))
            .expect("unknown map: {source}-to-{dest}")
            .convert(value)
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        // XXX this chain would be nicer if implemented as a builder pattern...
        self.convert(
            "location",
            "humidity",
            self.convert(
                "humidity",
                "temperature",
                self.convert(
                    "temperature",
                    "light",
                    self.convert(
                        "light",
                        "water",
                        self.convert(
                            "water",
                            "fertilizer",
                            self.convert(
                                "fertilizer",
                                "soil",
                                self.convert("soil", "seed", seed),
                            ),
                        ),
                    ),
                ),
            ),
        )
    }
}

fn example_input() -> String {
    String::from_utf8_lossy(include_bytes!("day5_example.txt")).to_string()
}

pub fn part1(input: Option<String>) -> u32 {
    let almanac = Almanac::new(input.unwrap_or_else(example_input));
    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.seed_to_location(seed))
        .min()
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(None), 35);
    }

    #[test]
    fn seed_to_soil() {
        let input = vec!["seed-to-soil map:", "50 98 2", "52 50 48"].join("\n");
        let mut lines = input.lines();
        let maybe_map = CategoryMap::new(&mut lines);
        assert!(maybe_map.is_some());

        let map = maybe_map.unwrap();
        assert_eq!(map.convert(0), 0);
        assert_eq!(map.convert(1), 1);
        assert_eq!(map.convert(50), 52);
        assert_eq!(map.convert(51), 53);
        assert_eq!(map.convert(98), 50);
        assert_eq!(map.convert(99), 51);
    }
}
