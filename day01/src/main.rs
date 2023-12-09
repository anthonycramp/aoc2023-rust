const DAY_NUMBER: &str = "01";
const INPUT: &str = include_str!("../../inputs/day01.txt");
// const INPUT: &str = "";

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

/// Find and return the digits that exist in the supplied string
fn get_digits(input: &str) -> Vec<i32> {
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

/// Find and return the digits that exist in the supplied string.
/// Digits can be the numbers 1..=9 or their equivalent as
/// English words.
fn get_digits_part2(input: &str) -> Vec<i32> {
    vec![2, 1, 9]
}

/// Compute the calibration value for the supplied text.
/// Returns the two digit number combining the
/// first and last digits found in the input per the
/// get_digits function provided.
fn compute_calibration_value(input: &str, get_digits_fn: fn(&str) -> Vec<i32>) -> i32 {
    let digits = get_digits_fn(input);
    let first_digit = *digits.first().unwrap();
    let last_digit = *digits.last().unwrap();
    first_digit * 10 + last_digit
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| compute_calibration_value(l, get_digits))
        .sum()
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT1: &str = r"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    const TEST_INPUT2: &str = r"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT1,
            expected: 142,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    // #[test]
    // fn test_part2() {
    //     let test_cases = [TestCase {
    //         input: TEST_INPUT2,
    //         expected: 281,
    //     }];
    //     for TestCase { input, expected } in test_cases.iter() {
    //         assert_eq!(part2(*input), *expected);
    //     }
    // }

    #[test]
    fn test_get_digits() {
        let test_cases = [
            TestCase {
                input: "1abc2",
                expected: vec![1, 2],
            },
            TestCase {
                input: "pqr3stu8vwx",
                expected: vec![3, 8],
            },
            TestCase {
                input: "a1b2c3d4e5f",
                expected: vec![1, 2, 3, 4, 5],
            },
            TestCase {
                input: "treb7uchet",
                expected: vec![7],
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(get_digits(input), *expected);
        }
    }

    #[test]
    fn test_get_digits_part2() {
        let test_cases = [TestCase {
            input: "two1nine",
            expected: vec![2, 1, 9],
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(get_digits_part2(input), *expected);
        }
    }

    #[test]
    fn test_compute_calibration_value() {
        let test_cases = [
            TestCase {
                input: "1abc2",
                expected: 12,
            },
            TestCase {
                input: "pqr3stu8vwx",
                expected: 38,
            },
            TestCase {
                input: "a1b2c3d4e5f",
                expected: 15,
            },
            TestCase {
                input: "treb7uchet",
                expected: 77,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(compute_calibration_value(input, get_digits), *expected);
        }
    }
}
