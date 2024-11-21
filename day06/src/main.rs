const DAY_NUMBER: &str = "06";
// const INPUT: &str = include_str!("../../inputs/dayNN.txt");
const INPUT: &str = r#"Time:        58     99     64     69
Distance:   478   2232   1019   1071"#;

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let mut input_lines = input.lines();
    let race_times: Vec<_> = input_lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|t| t.parse::<i32>().unwrap())
        .collect();

    let record_distances: Vec<_> = input_lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|d| d.parse::<i32>().unwrap())
        .collect();

    (0..race_times.len())
        .map(|race_index| {
            compute_record_breaking_combinations_for_race(
                race_times[race_index],
                record_distances[race_index],
            )
        })
        .product::<i32>()
}

fn compute_record_breaking_combinations_for_race(race_time: i32, record_distance: i32) -> i32 {
    (0..=race_time)
        .map(|charge_time| charge_time * (race_time - charge_time))
        .filter(|race_distance| *race_distance > record_distance)
        .count() as i32
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 288,
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
