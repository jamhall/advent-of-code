use anyhow::Result;

#[derive(Debug)]
pub enum Direction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug)]
pub struct Location {
    depth: usize,
    horizontal: usize,
    aim: usize,
}

impl Location {
    fn new() -> Self {
        Self {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }

    fn result(self) -> usize {
        self.horizontal * self.depth
    }
}

#[aoc_generator(day2)]
fn read(input: &str) -> Vec<Direction> {
    input
        .split('\n')
        .into_iter()
        .map(|line| {
            let (direction, distance) = line.trim().split_once(" ").unwrap();
            let distance = distance.parse::<usize>().unwrap();
            return match direction {
                "forward" => Direction::Forward(distance),
                "up" => Direction::Up(distance),
                "down" => Direction::Down(distance),
                _ => panic!("Unknown command")
            };
        })
        .collect::<Vec<Direction>>()
}

#[aoc(day2, part1)]
pub fn solve_part_1(commands: &[Direction]) -> usize {
    commands
        .iter()
        .fold(Location::new(), |location, direction| match direction {
            Direction::Forward(distance) => Location {
                horizontal: location.horizontal + distance,
                ..location
            },
            Direction::Down(depth) => Location {
                depth: location.depth + depth,
                ..location
            },
            Direction::Up(depth) => Location {
                depth: location.depth - depth,
                ..location
            },
        })
        .result()
}


#[aoc(day2, part2)]
pub fn solve_part_2(commands: &[Direction]) -> usize {
    commands
        .iter()
        .fold(Location::new(), |location, direction| match direction {
            Direction::Forward(distance) => Location {
                horizontal: location.horizontal + distance,
                depth: location.depth + distance * location.aim,
                ..location
            },
            Direction::Down(depth) => Location {
                aim: location.aim + depth,
                ..location
            },
            Direction::Up(depth) => Location {
                aim: location.aim - depth,
                ..location
            },
        })
        .result()
}