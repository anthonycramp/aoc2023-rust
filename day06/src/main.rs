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
    // the distance travelled by the boat is given by t * (R - t)
    // where t is the time spent charging and R is the total race time
    // We are looking for those times where t * (R - t) > D
    // where D is the record distance travelled
    // we can do this by finding the solutions to the
    // equation t^2 - R.t + D = 0
    // using the quadratic formula with a = 1, b = -R, c = D

    // we want these to be floating point to accommodate the
    // sqrt and division
    let a = 1 as f64;
    let b = -race_time as f64;
    let c = record_distance as f64;

    let quadratic_numerator_lower = -b - (b * b - 4.0 * a * c).sqrt();
    let quadratic_numerator_upper = -b + (b * b - 4.0 * a * c).sqrt();
    let quadratic_denominator = 2.0 * a;

    let lower_intercept = quadratic_numerator_lower / quadratic_denominator;
    let upper_intercept = quadratic_numerator_upper / quadratic_denominator;

    let mut first_record_breaking_time = lower_intercept.ceil() as i64;
    let mut last_record_breaking_time = upper_intercept.floor() as i64;

    // have to check the boundaries because we must be strictly greater than,
    // not equal to, the record distance
    let distance = first_record_breaking_time * (race_time as i64 - first_record_breaking_time);
    if distance == record_distance as i64 {
        first_record_breaking_time += 1;
    }

    let distance = last_record_breaking_time * (race_time as i64 - last_record_breaking_time);
    if distance == record_distance as i64 {
        last_record_breaking_time -= 1;
    }

    (last_record_breaking_time - first_record_breaking_time + 1) as i32
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
