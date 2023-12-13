use core::num;
use std::collections::HashSet;

const DAY_NUMBER: &str = "04";
const INPUT: &str = include_str!("../../inputs/day04.txt");
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

#[derive(Default)]
struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    selected_numbers: HashSet<i32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let mut card = Self::default();

        let first_split: Vec<_> = value.split(":").collect();
        let card_header_split: Vec<_> = first_split[0].split_ascii_whitespace().collect();
        let card_id = card_header_split[1].parse::<i32>().unwrap();

        let numbers_split: Vec<_> = first_split[1].trim().split(" | ").collect();
        println!("{:?}", numbers_split);
        let winning_numbers: HashSet<_> = numbers_split[0]
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let selected_numbers: HashSet<_> = numbers_split[1]
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        card.id = card_id;
        card.winning_numbers = winning_numbers;
        card.selected_numbers = selected_numbers;

        card
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_parse_card() {
        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(1, card.id);
        assert_eq!(5, card.winning_numbers.len());
        assert!(card.winning_numbers.contains(&41));
        assert!(card.winning_numbers.contains(&48));
        assert!(card.winning_numbers.contains(&83));
        assert!(card.winning_numbers.contains(&86));
        assert!(card.winning_numbers.contains(&17));

        assert_eq!(8, card.selected_numbers.len());
        assert!(card.selected_numbers.contains(&83));
        assert!(card.selected_numbers.contains(&86));
        assert!(card.selected_numbers.contains(&6));
        assert!(card.selected_numbers.contains(&31));
        assert!(card.selected_numbers.contains(&17));
        assert!(card.selected_numbers.contains(&9));
        assert!(card.selected_numbers.contains(&48));
        assert!(card.selected_numbers.contains(&53));
    }

    #[test]
    fn test_card_score() {
        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(8, card.get_score());
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 13,
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
