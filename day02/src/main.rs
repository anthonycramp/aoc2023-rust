const DAY_NUMBER: &str = "02";
const INPUT: &str = include_str!("../../inputs/day02.txt");
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

struct GameBag {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameBag {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn is_hand_possible(&self, hand: &Hand) -> bool {
        let mut is_hand_possible = true;

        if let Some(red) = hand.red {
            is_hand_possible = is_hand_possible && (red <= self.red);
        }

        if let Some(green) = hand.green {
            is_hand_possible = is_hand_possible && (green <= self.green);
        }

        if let Some(blue) = hand.blue {
            is_hand_possible = is_hand_possible && (blue <= self.blue);
        }

        is_hand_possible
    }

    fn is_game_possible(&self, game: &Game) -> bool {
        game.hands.iter().all(|h| self.is_hand_possible(h))
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
        }
    }
}

impl Hand {
    fn set_red(mut self, red: u32) -> Self {
        self.red = Some(red);
        self
    }

    fn set_green(mut self, green: u32) -> Self {
        self.green = Some(green);
        self
    }

    fn set_blue(mut self, blue: u32) -> Self {
        self.blue = Some(blue);
        self
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut hand = Hand::default();
        let cubes = value.split(", ");
        for cube in cubes {
            let cube_components: Vec<_> = cube.split_ascii_whitespace().collect();
            assert_eq!(2, cube_components.len());
            let number: u32 = cube_components[0].parse().unwrap();
            let colour = cube_components[1];
            match colour {
                "red" => hand = hand.set_red(number),
                "green" => hand = hand.set_green(number),
                "blue" => hand = hand.set_blue(number),
                _ => panic!("Unknown cube colour: {}", colour),
            }
        }

        hand
    }
}

struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn new(id: u32) -> Self {
        Self { id, hands: vec![] }
    }

    fn add_hand(mut self, hand: Hand) -> Self {
        self.hands.push(hand);
        self
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_is_hand_possible() {
        let game_bag = GameBag::new(12, 13, 14);
        assert!(game_bag.is_hand_possible(&Hand::default().set_blue(3).set_red(4)));
        assert!(!game_bag.is_hand_possible(&Hand::default().set_green(8).set_blue(6).set_red(20)));
        assert!(game_bag.is_hand_possible(&Hand::default().set_green(2)));
        assert!(game_bag.is_hand_possible(&Hand::default().set_blue(5).set_red(4).set_green(13)));
        assert!(!game_bag.is_hand_possible(&Hand::default().set_green(3).set_blue(15).set_red(14)));
        assert!(game_bag.is_hand_possible(&Hand::default()));
        assert!(game_bag.is_hand_possible(
            &Hand::default()
                .set_red(game_bag.red)
                .set_green(game_bag.green)
                .set_blue(game_bag.blue)
        ));
    }

    #[test]
    fn test_is_game_possible() {
        let game_bag = GameBag::new(12, 13, 14);
        assert!(game_bag.is_game_possible(
            &Game::new(1)
                .add_hand(Hand::default().set_blue(3).set_red(4))
                .add_hand(Hand::default().set_red(1).set_green(2).set_blue(6))
                .add_hand(Hand::default().set_green(2))
        ));
        assert!(game_bag.is_game_possible(
            &Game::new(2)
                .add_hand(Hand::default().set_blue(1).set_green(2))
                .add_hand(Hand::default().set_green(3).set_blue(4).set_red(1))
                .add_hand(Hand::default().set_green(1).set_blue(1))
        ));
        assert!(!game_bag.is_game_possible(
            &Game::new(3)
                .add_hand(Hand::default().set_green(8).set_blue(6).set_red(20))
                .add_hand(Hand::default().set_blue(5).set_red(4).set_green(13))
                .add_hand(Hand::default().set_green(5).set_red(1))
        ));
        assert!(!game_bag.is_game_possible(
            &Game::new(4)
                .add_hand(Hand::default().set_green(1).set_red(3).set_blue(6))
                .add_hand(Hand::default().set_green(3).set_red(6))
                .add_hand(Hand::default().set_green(3).set_blue(15).set_red(14))
        ));
        assert!(game_bag.is_game_possible(
            &Game::new(5)
                .add_hand(Hand::default().set_red(6).set_blue(1).set_green(3))
                .add_hand(Hand::default().set_blue(2).set_red(1).set_green(2))
        ));
    }

    #[test]
    fn test_parse_hand() {
        let test_cases = [
            TestCase {
                input: "3 blue, 4 red",
                expected: Hand::default().set_blue(3).set_red(4),
            },
            TestCase {
                input: "8 green, 6 blue, 20 red",
                expected: Hand::default().set_green(8).set_blue(6).set_red(20),
            },
        ];
        for TestCase { input, expected } in test_cases {
            assert_eq!(expected, Hand::from(input));
        }
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 8,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 2286,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
