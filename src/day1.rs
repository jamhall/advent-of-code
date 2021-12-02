use anyhow::Result;

#[aoc_generator(day1)]
pub fn read(input: &str) -> Vec<i32> {
    input.split('\n')
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<i32>>()
}

pub fn count(values: &[i32], window_size: usize) -> usize {
    values
        .windows(window_size + 1)
        .filter(|window| window[0] < window[window_size])
        .count()
}

#[aoc(day1, part1)]
pub fn solve_part_1(inputs: &[i32]) -> usize {
    count(inputs, 1)
}

#[aoc(day1, part2)]
pub fn solve_part_2(inputs: &[i32]) -> usize {
    count(inputs, 3)
}