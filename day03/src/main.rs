const DAY_NUMBER: &str = "03";
const INPUT: &str = include_str!("../../inputs/day03.txt");
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

/// row[location] should contain an ASCII digit which may be part
/// of a larger sequence of ASCII digits making up an integer number.
/// This function finds and returns the integer containing the digit
/// at row[location].
fn get_integer_at_location(row: &str, location: usize) -> u32 {
    assert!(row.chars().nth(location).unwrap().is_ascii_digit());
    let row_bytes = row.as_bytes();

    // need to walk backward from location until we hit start of row
    // or we find a non-digit location
    let mut number_start_index = location;
    loop {
        if !row_bytes[number_start_index].is_ascii_digit() {
            // we've gone one step before the integer, step forward once
            number_start_index += 1;
            break;
        }

        if number_start_index == 0 {
            // can't step back further, so break
            break;
        }

        number_start_index -= 1;
    }

    // then walk forward from location until we hit end of row or
    // we find a non-digit location
    let mut number_end_index = location;
    while number_end_index < row_bytes.len() {
        if !row_bytes[number_end_index].is_ascii_digit() {
            // we're passed the end of the integer, no need to
            // step back because ranges are non-inclusive of their
            // upper bound.
            break;
        }
        number_end_index += 1;
    }

    row[number_start_index..number_end_index]
        .parse::<u32>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r"467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_get_integer_at_location() {
        let row1 = "467..114..";
        assert_eq!(467, get_integer_at_location(row1, 0));
        assert_eq!(467, get_integer_at_location(row1, 1));
        assert_eq!(467, get_integer_at_location(row1, 2));
        assert_eq!(114, get_integer_at_location(row1, 5));
        assert_eq!(114, get_integer_at_location(row1, 6));
        assert_eq!(114, get_integer_at_location(row1, 7));

        let row2 = ".664.598..";
        assert_eq!(664, get_integer_at_location(row2, 2));
        assert_eq!(598, get_integer_at_location(row2, 7));

        let row3 = ".......45";
        assert_eq!(45, get_integer_at_location(row3, 7));
        assert_eq!(45, get_integer_at_location(row3, 8));
    }

    #[test]
    fn test_get_neighbours_of_location() {
        let neighbours = get_neighbours_of_location(Location(0, 0), 10, 10);
        assert_eq!(3, neighbours.len());
        assert!(neighbours.contains(Location(0, 1)));
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_part1() {
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
