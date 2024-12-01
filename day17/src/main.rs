use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

const DAY_NUMBER: &str = "17";
const INPUT: &str = include_str!("../../inputs/day17.txt");
// const INPUT: &str = "";

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

#[derive(Debug)]
struct DesertIslandMap {
    map_rows: usize,
    map_cols: usize,
    heat_loss_map: Vec<i32>,
}

impl From<&str> for DesertIslandMap {
    fn from(input: &str) -> Self {
        let lines = input.lines();
        let map_rows = lines.clone().count();
        let map_cols = lines.peekable().next().unwrap().len();

        println!("{}x{}", map_rows, map_cols);
        let heat_loss_map = input
            .lines()
            .flat_map(|s| s.trim().chars())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();

        Self {
            map_rows,
            map_cols,
            heat_loss_map,
        }
    }
}

impl DesertIslandMap {
    fn get_heat_loss(&self, location: &Location) -> i32 {
        self.heat_loss_map[self.get_index(&location)]
    }

    fn get_index(&self, location: &Location) -> usize {
        (location.row * self.map_cols + location.col) as usize
    }

    fn get_location(&self, index: usize) -> Location {
        let row = index / self.map_cols;
        let col = index % self.map_rows;

        Location { row, col }
    }

    fn get_map_size(&self) -> usize {
        self.map_rows * self.map_cols
    }

    fn get_neighbours(&self, location: &Location) -> HashMap<AbsoluteDirection, Location> {
        let mut neighbours = HashMap::new();

        if location.row > 0 {
            neighbours.insert(
                AbsoluteDirection::NORTH,
                Location {
                    row: location.row - 1,
                    col: location.col,
                },
            );
        }

        if location.row < self.map_rows - 1 {
            neighbours.insert(
                AbsoluteDirection::SOUTH,
                Location {
                    row: location.row + 1,
                    col: location.col,
                },
            );
        }

        if location.col > 0 {
            neighbours.insert(
                AbsoluteDirection::WEST,
                Location {
                    row: location.row,
                    col: location.col - 1,
                },
            );
        }

        if location.col < self.map_cols - 1 {
            neighbours.insert(
                AbsoluteDirection::EAST,
                Location {
                    row: location.row,
                    col: location.col + 1,
                },
            );
        }

        neighbours
    }

    fn draw(&self, paths: &Vec<i32>, start: &Location, goal: &Location) {
        let start_index = self.get_index(start);
        let goal_index = self.get_index(goal);

        let mut directions = vec![AbsoluteDirection::NONE; self.get_map_size()];
        let mut current_index = goal_index;
        while current_index != start_index {
            let prev_index = paths[current_index] as usize;
            let current_location = self.get_location(current_index);
            let prev_location = self.get_location(prev_index);
            let direction_from_prev_to_current =
                AbsoluteDirection::compute_direction(&prev_location, &current_location);
            directions[current_index] = direction_from_prev_to_current;
            current_index = prev_index;
        }

        for row in 0..self.map_rows {
            for col in 0..self.map_cols {
                let location = Location { row, col };
                let index = self.get_index(&location);
                print!(
                    "{}{} ",
                    self.heat_loss_map[index],
                    directions[index].get_display_char()
                );
            }
            println!();
        }
    }
}

#[derive(Clone, PartialEq)]
struct Location {
    row: usize,
    col: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            row: Default::default(),
            col: Default::default(),
        }
    }
}

impl Location {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum AbsoluteDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    NONE,
}

impl AbsoluteDirection {
    fn compute_direction(from: &Location, to: &Location) -> Self {
        if from.row != to.row && from.col != to.col {
            Self::NONE
        } else if from.row == to.row && from.col == to.col {
            Self::NONE
        } else if from.row == to.row {
            if from.col < to.col {
                Self::EAST
            } else {
                Self::WEST
            }
        } else if from.col == to.col {
            if from.row < to.row {
                Self::SOUTH
            } else {
                Self::NORTH
            }
        } else {
            Self::NONE
        }
    }

    fn get_opposite_direction(&self) -> AbsoluteDirection {
        match self {
            AbsoluteDirection::EAST => AbsoluteDirection::WEST,
            AbsoluteDirection::WEST => AbsoluteDirection::EAST,
            AbsoluteDirection::SOUTH => AbsoluteDirection::NORTH,
            AbsoluteDirection::NORTH => AbsoluteDirection::SOUTH,
            _ => AbsoluteDirection::NONE,
        }
    }

    fn get_display_char(&self) -> char {
        match self {
            AbsoluteDirection::EAST => '>',
            AbsoluteDirection::WEST => '<',
            AbsoluteDirection::NORTH => '^',
            AbsoluteDirection::SOUTH => 'v',
            _ => ' ',
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct State {
    cost: i32,
    index: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let desert_island_map = DesertIslandMap::from(input);

    let mut start = Location::default();
    let goal = Location::new(
        desert_island_map.map_rows - 1,
        desert_island_map.map_cols - 1,
    );

    let mut distance = vec![i32::max_value(); desert_island_map.get_map_size()];
    let mut last: Vec<i32> = vec![-1; desert_island_map.get_map_size()];

    distance[desert_island_map.get_index(&start)] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        index: desert_island_map.get_index(&start),
    });

    while let Some(State { cost, index }) = heap.pop() {
        // println!("{}: {}", index, cost);
        // if we've reached our goal, print all the paths found and
        // return the shortest distance from start to goal
        // if index == desert_island_map.get_index(&goal) {
        //     println!("{:?}", &last);
        //     return distance[index];
        // }

        // if the cost to reach this location exceeds a cost on
        // a path we've already explored, ignore this node
        if cost > distance[index] {
            continue;
        }

        // next step processes each of the neighbours of the current location
        let all_neighbours =
            desert_island_map.get_neighbours(&desert_island_map.get_location(index));

        // However, for part 1, we can't move in the same direction
        // more than three times. We also can't move backwards (although,
        // this constraint would be picked up in the shortest path
        // calculation). Thus, we need to check the recent direction
        // of the shortest path to the current node and check whether
        // each neighbour would exceed the forward or reverse progress
        // constraints, and prune those neighbours if so (doesn't
        // apply to the start location)
        let mut direction_to_previous_location = AbsoluteDirection::NONE;
        let mut three_previous_directions = AbsoluteDirection::NONE;

        if index != desert_island_map.get_index(&start) {
            let current_location = desert_island_map.get_location(index);
            let prev_index = last[index];
            let prev_location = desert_island_map.get_location(prev_index as usize);
            direction_to_previous_location =
                AbsoluteDirection::compute_direction(&current_location, &prev_location);
            let prev_prev_index = last[prev_index as usize];
            if prev_prev_index >= 0 {
                let prev_prev_prev_index = last[prev_prev_index as usize];
                if prev_prev_prev_index >= 0 {
                    let first_location =
                        desert_island_map.get_location(prev_prev_prev_index as usize);
                    let second_location = desert_island_map.get_location(prev_prev_index as usize);
                    let first_direction =
                        AbsoluteDirection::compute_direction(&first_location, &second_location);
                    let third_location = prev_location;
                    let second_direction =
                        AbsoluteDirection::compute_direction(&second_location, &third_location);
                    let fourth_location = current_location;
                    let third_direction =
                        AbsoluteDirection::compute_direction(&third_location, &fourth_location);
                    if first_direction == second_direction && second_direction == third_direction {
                        three_previous_directions = first_direction;
                    }
                }
            }
        }

        let mut neighbours = vec![];
        for (direction, location) in all_neighbours {
            if direction != direction_to_previous_location && direction != three_previous_directions
            {
                neighbours.push(location);
            }
        }

        // for each of the valid neighbours
        // check if they'll be on the shortest path and update
        for location in neighbours {
            let next = State {
                cost: cost + desert_island_map.get_heat_loss(&location),
                index: desert_island_map.get_index(&location),
            };

            // println!("\tChecking {:?}", &next);
            // println!(
            //     "\tCurrent distance to {}: {}",
            //     next.index, distance[next.index]
            // );
            if next.cost < distance[next.index] {
                heap.push(next);
                distance[next.index] = next.cost;
                last[next.index] = index as i32;
                // println!("\tAdding {:?}", &next);
            }
        }
    }

    // println!("{:?}", &last);
    desert_island_map.draw(&last, &start, &goal);
    distance[desert_island_map.get_index(&goal)]
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 102,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_neighbours() {
        let desert_island_map = DesertIslandMap::from(TEST_INPUT);

        let test_cases = [
            TestCase {
                input: Location::new(0, 0),
                expected: 2,
            },
            TestCase {
                input: Location::new(0, 1),
                expected: 3,
            },
            TestCase {
                input: Location::new(1, 1),
                expected: 4,
            },
            TestCase {
                input: Location::new(
                    desert_island_map.map_rows - 1,
                    desert_island_map.map_cols - 1,
                ),
                expected: 2,
            },
            TestCase {
                input: Location::new(
                    desert_island_map.map_rows / 2,
                    desert_island_map.map_cols - 1,
                ),
                expected: 3,
            },
        ];

        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(*expected, desert_island_map.get_neighbours(input).len());
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
