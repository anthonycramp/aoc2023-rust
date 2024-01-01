use std::collections::HashMap;

const DAY_NUMBER: &str = "05";
const INPUT: &str = include_str!("../../inputs/day05.txt");
// const INPUT: &str = "";

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i64 {
    let source_parameter = String::from("seed");
    let destination_parameter = String::from("location");
    let input_value_string = input.lines().next().unwrap();
    let input_values: Vec<_> = parse_seeds_1(input_value_string);

    let almanac = Almanac::from(input);

    input_values
        .iter()
        .map(|v| almanac.map(&source_parameter, *v, &destination_parameter))
        .min()
        .unwrap()
}

// replace return type as required by the problem
fn part2(input: &str) -> i64 {
    let source_parameter = String::from("seed");
    let destination_parameter = String::from("location");
    let input_value_string = input.lines().next().unwrap();
    let input_values: Vec<_> = parse_seeds_2(input_value_string);

    let almanac = Almanac::from(input);

    input_values
        .iter()
        .map(|v| almanac.map(&source_parameter, *v, &destination_parameter))
        .min()
        .unwrap()
}

fn parse_seeds_1(seeds: &str) -> Vec<i64> {
    seeds
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}

fn parse_seeds_2(seeds: &str) -> Vec<i64> {
    let seeds = parse_seeds_1(seeds);
    let mut seeds_iter = seeds.iter();

    let mut seeds: Vec<i64> = vec![];
    loop {
        let seed_base_n = seeds_iter.next();
        if seed_base_n.is_none() {
            break;
        }
        let seed_base_n = *seed_base_n.unwrap();
        let seed_base_l = *seeds_iter.next().unwrap();

        for seed_n in seed_base_n..(seed_base_n + seed_base_l) {
            seeds.push(seed_n);
        }
    }

    seeds
}

#[derive(Debug, PartialEq)]
enum DestinationValue {
    In(i64),
    Out(i64),
}

#[derive(Default, Debug)]
struct AlmanacRange {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl AlmanacRange {
    fn new(destination_range_start: i64, source_range_start: i64, range_length: i64) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }

    fn in_source_range(&self, source_value: i64) -> bool {
        source_value >= self.source_range_start
            && source_value < self.source_range_start + self.range_length
    }

    fn map(&self, source_value: i64) -> DestinationValue {
        if self.in_source_range(source_value) {
            DestinationValue::In(
                source_value - (self.source_range_start - self.destination_range_start),
            )
        } else {
            DestinationValue::Out(source_value)
        }
    }
}

impl From<&str> for AlmanacRange {
    fn from(value: &str) -> Self {
        let values: Vec<_> = value.split_ascii_whitespace().collect();
        Self::new(
            values[0].parse().unwrap(),
            values[1].parse().unwrap(),
            values[2].parse().unwrap(),
        )
    }
}

#[derive(Default, Debug)]
struct AlmanacEntry {
    ranges: Vec<AlmanacRange>,
    source_parameter: String,
    destination_parameter: String,
}

impl AlmanacEntry {
    fn new(source_parameter: &str, destination_parameter: &str) -> Self {
        let mut entry = Self::default();
        entry.source_parameter = String::from(source_parameter);
        entry.destination_parameter = String::from(destination_parameter);

        entry
    }

    fn add_range(mut self, range: AlmanacRange) -> Self {
        self.ranges.push(range);
        self
    }

    fn map(&self, source_value: i64) -> i64 {
        let destination_value_possibilities: Vec<_> =
            self.ranges.iter().map(|r| r.map(source_value)).collect();

        for destination_value in destination_value_possibilities {
            if let DestinationValue::In(val) = destination_value {
                return val;
            }
        }

        source_value
    }
}

impl From<&str> for AlmanacEntry {
    fn from(value: &str) -> Self {
        let mut entry = AlmanacEntry::default();

        let header_string = value.lines().next().unwrap();
        let parameters: Vec<_> = header_string
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .split('-')
            .collect();
        entry.source_parameter = String::from(parameters[0]);
        entry.destination_parameter = String::from(parameters[2]);

        for almanac_range_str in value.lines().skip(1) {
            if almanac_range_str.trim().is_empty() {
                continue;
            }
            entry = entry.add_range(AlmanacRange::from(almanac_range_str));
        }
        entry
    }
}

#[derive(Default, Debug)]
struct Almanac {
    entries: HashMap<String, AlmanacEntry>,
}

impl Almanac {
    fn add_entry(mut self, entry: AlmanacEntry) -> Self {
        self.entries.insert(entry.source_parameter.clone(), entry);
        self
    }

    fn map(&self, source_parameter: &str, source_value: i64, destination_parameter: &str) -> i64 {
        let mut current_parameter = String::from(source_parameter);
        let mut current_value = source_value;

        while current_parameter != destination_parameter {
            if let Some(entry) = self.entries.get(&current_parameter) {
                current_parameter = entry.destination_parameter.clone();
                current_value = entry.map(current_value);
            } else {
                panic!(
                    "Unknown alamanac entry source parameter: {}",
                    current_parameter
                );
            }
        }

        current_value
    }
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut almanac = Self::default();

        let almanac_entries = value.lines().skip(2);
        let almanac_entries = almanac_entries
            .map(|e| e.trim())
            .collect::<Vec<_>>()
            .join("\n");
        let almanac_entries = almanac_entries.split("\n\n");
        for almanac_entry_str in almanac_entries {
            almanac = almanac.add_entry(AlmanacEntry::from(almanac_entry_str));
        }
        almanac
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r"seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4";

    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_map_range() {
        let almanac_range = AlmanacRange::new(50, 98, 2);
        assert_eq!(DestinationValue::In(50), almanac_range.map(98));
        assert_eq!(DestinationValue::In(51), almanac_range.map(99));
        assert_eq!(DestinationValue::Out(100), almanac_range.map(100));
        assert_eq!(DestinationValue::Out(97), almanac_range.map(97));
        let almanac_range = AlmanacRange::new(52, 50, 48);
        assert_eq!(DestinationValue::Out(98), almanac_range.map(98));
        assert_eq!(DestinationValue::Out(99), almanac_range.map(99));
        assert_eq!(DestinationValue::Out(100), almanac_range.map(100));
        assert_eq!(DestinationValue::In(97), almanac_range.map(95));
        assert_eq!(DestinationValue::In(52), almanac_range.map(50));
        assert_eq!(DestinationValue::In(99), almanac_range.map(97));
    }

    #[test]
    fn test_map_entry() {
        let almanac_entry = AlmanacEntry::default()
            .add_range(AlmanacRange::new(50, 98, 2))
            .add_range(AlmanacRange::new(52, 50, 48));
        assert_eq!(49, almanac_entry.map(49));
        assert_eq!(52, almanac_entry.map(50));
        assert_eq!(99, almanac_entry.map(97));
        assert_eq!(50, almanac_entry.map(98));
        assert_eq!(51, almanac_entry.map(99));
        assert_eq!(100, almanac_entry.map(100));
        assert_eq!(81, almanac_entry.map(79));
        assert_eq!(14, almanac_entry.map(14));
        assert_eq!(57, almanac_entry.map(55));
        assert_eq!(13, almanac_entry.map(13));
    }

    #[test]
    fn test_map_almanac() {
        let almanac = Almanac::default()
            .add_entry(
                AlmanacEntry::new("seed", "soil")
                    .add_range(AlmanacRange::new(50, 98, 2))
                    .add_range(AlmanacRange::new(52, 50, 48)),
            )
            .add_entry(
                AlmanacEntry::new("soil", "fertilizer")
                    .add_range(AlmanacRange::new(0, 15, 37))
                    .add_range(AlmanacRange::new(37, 52, 2))
                    .add_range(AlmanacRange::new(39, 0, 15)),
            );
        assert_eq!(81, almanac.map("seed", 79, "soil"));
        assert_eq!(81, almanac.map("seed", 79, "fertilizer"));
        assert_eq!(53, almanac.map("seed", 14, "fertilizer"));
    }

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 35,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 46,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
