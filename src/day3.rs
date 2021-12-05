use anyhow::Result;

#[derive(Debug)]
enum Mode {
    MostCommon,
    LeastCommon,
}

fn calculate_rates(values: &[&str], mode: Mode) -> usize {
    (0..12)
        .map(|i| {
            values
                .iter()
                .map(|&line| line.chars().nth(i).unwrap() == '0')
                .map(|is_zero| if is_zero { 1 } else { 0 })
                .sum()
        })
        .map(|summed: usize| {
            if summed > values.len() / 2 {
                match mode {
                    Mode::MostCommon => 0,
                    Mode::LeastCommon => 1
                }
            } else {
                match mode {
                    Mode::MostCommon => 1,
                    Mode::LeastCommon => 0
                }
            }
        })
        .fold(0, |result, bit| {
            (result << 1) ^ bit
        })
}

#[aoc(day3, part1)]
pub fn solve_part_1(input: &str) -> usize {
    let lines: Vec<&str> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect();
    let gamma_rate = calculate_rates(&lines, Mode::MostCommon);
    let epsilon_rate = calculate_rates(&lines, Mode::LeastCommon);
    epsilon_rate * gamma_rate
}