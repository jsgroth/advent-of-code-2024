//! Day 2: Red-Nosed Reports
//!
//! <https://adventofcode.com/2024/day/2>

use std::error::Error;

fn parse_input(input: &str) -> impl Iterator<Item = Vec<i32>> + use<'_> {
    input.lines().filter(|line| !line.is_empty()).map(|line| {
        line.split(' ')
            .map(|level| level.parse::<i32>().unwrap())
            .collect()
    })
}

fn solve_part_1(input: &str) -> usize {
    parse_input(input)
        .filter(|levels| levels_valid(levels))
        .count()
}

fn levels_valid(levels: &[i32]) -> bool {
    if levels.len() <= 1 {
        return true;
    }

    let sign = (levels[1] - levels[0]).signum();
    (1..levels.len()).all(|i| {
        let diff = levels[i] - levels[i - 1];
        diff.signum() == sign && (1..=3).contains(&diff.abs())
    })
}

fn solve_part_2(input: &str) -> usize {
    parse_input(input)
        .filter(|levels| {
            levels_valid(levels)
                || (0..levels.len()).any(|i| {
                    let mut levels_with_skip = levels.clone();
                    levels_with_skip.remove(i);
                    levels_valid(&levels_with_skip)
                })
        })
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day2.txt");

    #[test]
    fn part_1() {
        assert_eq!(2, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(4, solve_part_2(SAMPLE_INPUT));
    }
}
