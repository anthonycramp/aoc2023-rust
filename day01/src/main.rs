const INPUT: &str = include_str!("../../inputs/day01.txt");
// const INPUT: &str = "";

fn main() {
    const day_number: &str = "01";
    println!("Day {} Part 1: {:?}", day_number, part1(INPUT));
    println!("Day {} Part 2: {:?}", day_number, part2(INPUT));
}

/// Find the first digit, [1,9], in the supplied string
fn find_first_digit(input: &str) -> i32 {
    1
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    0
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

    // #[test]
    // fn test_part1() {
    //     let test_cases = [TestCase {
    //         input: TEST_INPUT1,
    //         expected: 142,
    //     }];
    //     for TestCase { input, expected } in test_cases.iter() {
    //         assert_eq!(part1(*input), *expected);
    //     }
    // }

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
    fn test_find_first_digit() {
        let test_cases = [TestCase {
            input: "1abc2",
            expected: 1,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(find_first_digit(input), *expected);
        }
    }
}
