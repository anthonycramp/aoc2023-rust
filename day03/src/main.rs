const DAY_NUMBER: &str = "03";
const INPUT: &str = include_str!("../../inputs/day03.txt");
// const INPUT: &str = "";

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let schematic = Schematic::from(input);

    let mut sum = 0;

    for row in 0..schematic.schematic.len() {
        for col in 0..schematic.schematic[0].len() {
            let location = Location(row, col);
            let symbol_at_location = schematic.get_symbol_at_location(&location);
            if symbol_at_location == Symbol::SPECIAL || symbol_at_location == Symbol::GEAR {
                sum += schematic
                    .get_part_numbers_adjacent_to_location(&location)
                    .iter()
                    .sum::<u32>();
            }
        }
    }

    sum as i32
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

/// Represent a row and column in the engine schematic
#[derive(Debug, PartialEq, Clone)]
struct Location(usize, usize);

fn get_neighbours_of_location(location: &Location, rows: usize, columns: usize) -> Vec<Location> {
    let row_lower_bound = if location.0 == 0 {
        location.0
    } else {
        location.0 - 1
    };
    let row_upper_bound = if location.0 == rows - 1 {
        location.0
    } else {
        location.0 + 1
    };
    let column_lower_bound = if location.1 == 0 {
        location.1
    } else {
        location.1 - 1
    };
    let column_upper_bound = if location.1 == columns - 1 {
        location.1
    } else {
        location.1 + 1
    };

    let mut neighbours: Vec<Location> = vec![];
    for row in row_lower_bound..=row_upper_bound {
        for column in column_lower_bound..=column_upper_bound {
            if row == location.0 && column == location.1 {
                // source location is not a neighbour
                continue;
            }
            neighbours.push(Location(row, column));
        }
    }

    neighbours
}

fn distance(loc1: &Location, loc2: &Location) -> i32 {
    (loc1.0 as i32 - loc2.0 as i32).abs() + (loc1.1 as i32 - loc2.1 as i32).abs()
}

#[derive(Debug, PartialEq)]
enum Symbol {
    EMPTY,
    SPECIAL,
    DIGIT,
    GEAR,
}

#[derive(Default)]
struct Schematic {
    schematic: Vec<String>,
}

impl From<&str> for Schematic {
    fn from(input: &str) -> Self {
        let mut schematic = Schematic::default();

        input
            .lines()
            .for_each(|l| schematic.schematic.push(String::from(l.trim())));

        schematic
    }
}

impl Schematic {
    fn get_symbol_at_location(&self, location: &Location) -> Symbol {
        let char_at_location = self.schematic[location.0].chars().nth(location.1).unwrap();

        if char_at_location == '.' {
            Symbol::EMPTY
        } else if char_at_location == '*' {
            Symbol::GEAR
        } else if char_at_location.is_ascii_digit() {
            Symbol::DIGIT
        } else {
            Symbol::SPECIAL
        }
    }

    fn get_part_numbers_adjacent_to_location(&self, location: &Location) -> Vec<u32> {
        let mut part_numbers: Vec<_> = vec![];
        let neighbours =
            get_neighbours_of_location(location, self.schematic.len(), self.schematic[0].len());
        let mut last_location = None;
        let mut in_digit = false;

        for (index, neighbour) in neighbours.iter().enumerate() {
            if let Some(loc) = last_location {
                if distance(&neighbour, loc) > 1 {
                    in_digit = false;
                }
            }

            last_location = Some(&neighbours[index]);
            match self.get_symbol_at_location(&neighbour) {
                Symbol::DIGIT => {
                    if !in_digit {
                        part_numbers.push(get_integer_at_location(
                            self.schematic[neighbour.0].as_str(),
                            neighbour.1,
                        ));
                        in_digit = true;
                    }
                }
                _ => in_digit = false,
            }
        }

        part_numbers
    }

    fn is_gear(&self, location: &Location) -> bool {
        self.get_symbol_at_location(location) == Symbol::GEAR
            && self.get_part_numbers_adjacent_to_location(location).len() == 2
    }
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
        // test upper left corner
        let neighbours = get_neighbours_of_location(&Location(0, 0), 10, 10);
        assert_eq!(3, neighbours.len());
        assert!(neighbours.contains(&Location(0, 1)));
        assert!(neighbours.contains(&Location(1, 0)));
        assert!(neighbours.contains(&Location(1, 1)));

        // test lower right corner
        let neighbours = get_neighbours_of_location(&Location(9, 9), 10, 10);
        assert_eq!(3, neighbours.len());
        assert!(neighbours.contains(&Location(8, 9)));
        assert!(neighbours.contains(&Location(9, 8)));
        assert!(neighbours.contains(&Location(8, 8)));

        // test upper right corner
        let neighbours = get_neighbours_of_location(&Location(0, 9), 10, 10);
        assert_eq!(3, neighbours.len());
        assert!(neighbours.contains(&Location(0, 8)));
        assert!(neighbours.contains(&Location(1, 9)));
        assert!(neighbours.contains(&Location(1, 8)));

        // test lower left corner
        let neighbours = get_neighbours_of_location(&Location(9, 0), 10, 10);
        assert_eq!(3, neighbours.len());
        assert!(neighbours.contains(&Location(8, 0)));
        assert!(neighbours.contains(&Location(9, 1)));
        assert!(neighbours.contains(&Location(8, 1)));

        // test left edge
        let neighbours = get_neighbours_of_location(&Location(5, 0), 10, 10);
        assert_eq!(5, neighbours.len());
        assert!(neighbours.contains(&Location(4, 0)));
        assert!(neighbours.contains(&Location(6, 0)));
        assert!(neighbours.contains(&Location(4, 1)));
        assert!(neighbours.contains(&Location(5, 1)));
        assert!(neighbours.contains(&Location(6, 1)));

        // test right edge
        let neighbours = get_neighbours_of_location(&Location(5, 9), 10, 10);
        assert_eq!(5, neighbours.len());
        assert!(neighbours.contains(&Location(4, 9)));
        assert!(neighbours.contains(&Location(6, 9)));
        assert!(neighbours.contains(&Location(4, 8)));
        assert!(neighbours.contains(&Location(5, 8)));
        assert!(neighbours.contains(&Location(6, 8)));

        // test top edge
        let neighbours = get_neighbours_of_location(&Location(0, 5), 10, 10);
        assert_eq!(5, neighbours.len());
        assert!(neighbours.contains(&Location(0, 4)));
        assert!(neighbours.contains(&Location(0, 6)));
        assert!(neighbours.contains(&Location(1, 4)));
        assert!(neighbours.contains(&Location(1, 5)));
        assert!(neighbours.contains(&Location(1, 6)));

        // test bottom edge
        let neighbours = get_neighbours_of_location(&Location(9, 5), 10, 10);
        assert_eq!(5, neighbours.len());
        assert!(neighbours.contains(&Location(9, 4)));
        assert!(neighbours.contains(&Location(9, 6)));
        assert!(neighbours.contains(&Location(8, 4)));
        assert!(neighbours.contains(&Location(8, 5)));
        assert!(neighbours.contains(&Location(8, 6)));

        // test a centre location
        let neighbours = get_neighbours_of_location(&Location(5, 5), 10, 10);
        assert_eq!(8, neighbours.len());
        assert!(neighbours.contains(&Location(4, 4)));
        assert!(neighbours.contains(&Location(4, 5)));
        assert!(neighbours.contains(&Location(4, 6)));
        assert!(neighbours.contains(&Location(5, 4)));
        assert!(neighbours.contains(&Location(5, 6)));
        assert!(neighbours.contains(&Location(6, 4)));
        assert!(neighbours.contains(&Location(6, 5)));
        assert!(neighbours.contains(&Location(6, 6)));
    }

    #[test]
    fn test_get_symbol_at_schematic_location() {
        let schematic = Schematic::from(TEST_INPUT);
        let test_cases = [
            TestCase {
                input: Location(0, 0),
                expected: Symbol::DIGIT,
            },
            TestCase {
                input: Location(0, 4),
                expected: Symbol::EMPTY,
            },
            TestCase {
                input: Location(1, 3),
                expected: Symbol::GEAR,
            },
            TestCase {
                input: Location(3, 6),
                expected: Symbol::SPECIAL,
            },
        ];

        for TestCase { input, expected } in test_cases {
            assert_eq!(expected, schematic.get_symbol_at_location(&input));
        }
    }

    #[test]
    fn test_get_part_numbers_adjacent_to_location() {
        let schematic = Schematic::from(TEST_INPUT);
        let part_numbers = schematic.get_part_numbers_adjacent_to_location(&Location(1, 3));
        assert_eq!(2, part_numbers.len());
        assert!(part_numbers.contains(&467));
        assert!(part_numbers.contains(&35));

        let schematic = Schematic::from(".....\n35.35\n..*..\n.....\n..*..");
        let part_numbers = schematic.get_part_numbers_adjacent_to_location(&Location(2, 2));
        assert_eq!(2, part_numbers.len());
        assert!(part_numbers.iter().all(|pn| *pn == 35));

        let schematic = Schematic::from("...\n3*5\n...");
        let part_numbers = schematic.get_part_numbers_adjacent_to_location(&Location(1, 1));
        assert_eq!(2, part_numbers.len());
        assert!(part_numbers.contains(&3));
        assert!(part_numbers.contains(&5));
    }

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 4361,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_is_gear() {
        let schematic = Schematic::from(TEST_INPUT);
        assert!(schematic.is_gear(&Location(1, 3)));
        assert!(schematic.is_gear(&Location(8, 5)));
        assert!(!schematic.is_gear(&Location(4, 3)));
        assert!(!schematic.is_gear(&Location(3, 6)));
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
