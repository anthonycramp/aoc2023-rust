use std::collections::{BinaryHeap, HashMap};

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

#[derive(PartialEq, Eq, Hash)]
enum AbsoluteDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST,
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
    let mut last = vec![-1; desert_island_map.get_map_size()];

    distance[desert_island_map.get_index(&start)] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        index: desert_island_map.get_index(&start),
    });

    while let Some(State { cost, index }) = heap.pop() {
        if index == desert_island_map.get_index(&goal) {
            println!("{:?}", &last);
            return distance[index];
        }

        if cost > distance[index] {
            continue;
        }

        let neighbours = desert_island_map.get_neighbours(&desert_island_map.get_location(index));

        for (_, location) in neighbours {
            let next = State {
                cost: cost + desert_island_map.get_heat_loss(&location),
                index: desert_island_map.get_index(&location),
            };

            if next.cost < distance[next.index] {
                heap.push(next);
                distance[next.index] = next.cost;
                last[next.index] = index as i32;
            }
        }
    }

    println!("{:?}", &last);
    0
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
