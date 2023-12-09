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

    fn is_game_possible(&self, game: &Game) -> bool {
        let mut is_game_possible = true;

        if let Some(red) = game.red {
            is_game_possible = is_game_possible && (red <= self.red);
        }

        if let Some(green) = game.green {
            is_game_possible = is_game_possible && (green <= self.green);
        }

        if let Some(blue) = game.blue {
            is_game_possible = is_game_possible && (blue <= self.blue);
        }

        is_game_possible
    }
}

struct Game {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
        }
    }
}

impl Game {
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
    fn test_is_game_possible() {
        let game_bag = GameBag::new(12, 13, 14);
        assert!(game_bag.is_game_possible(&Game::default().set_blue(3).set_red(4)));
        assert!(!game_bag.is_game_possible(&Game::default().set_green(8).set_blue(6).set_red(20)));
        assert!(game_bag.is_game_possible(&Game::default().set_green(2)));
        assert!(game_bag.is_game_possible(&Game::default().set_blue(5).set_red(4).set_green(13)));
        assert!(!game_bag.is_game_possible(&Game::default().set_green(3).set_blue(15).set_red(14)));
        assert!(game_bag.is_game_possible(&Game::default()));
        assert!(game_bag.is_game_possible(
            &Game::default()
                .set_red(game_bag.red)
                .set_green(game_bag.green)
                .set_blue(game_bag.blue)
        ));
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
