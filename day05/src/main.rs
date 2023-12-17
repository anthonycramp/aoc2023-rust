const DAY_NUMBER: &str = "05";
const INPUT: &str = include_str!("../../inputs/day05.txt");
// const INPUT: &str = "";

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    0
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[derive(Default, Debug)]
struct AlmanacRange {
    destination_range_start: i32,
    source_range_start: i32,
    range_length: i32,
}

impl AlmanacRange {
    fn new(destination_range_start: i32, source_range_start: i32, range_length: i32) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }

    fn in_source_range(&self, source_value: i32) -> bool {
        source_value >= self.source_range_start
            && source_value < self.source_range_start + self.range_length
    }

    fn map(&self, source_value: i32) -> Option<i32> {
        if self.in_source_range(source_value) {
            Some(source_value - (self.source_range_start - self.destination_range_start))
        } else {
            None
        }
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
        assert_eq!(Some(50), almanac_range.map(98));
        assert_eq!(Some(51), almanac_range.map(99));
        assert_eq!(None, almanac_range.map(100));
    }

    #[test]
    #[ignore = "not yet implemented"]
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
    #[ignore = "not yet implemented"]
    fn test_part2() {
        let test_cases = [
            TestCase {
                input: TEST_INPUT,
                expected: 123,
            },
            TestCase {
                input: "abc",
                expected: 345,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
