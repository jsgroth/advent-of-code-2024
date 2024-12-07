//! Day 1: Historian Hysteria
//!
//! <https://adventofcode.com/2024/day/1>

use rustc_hash::FxHashMap;
use std::error::Error;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split = line.split_ascii_whitespace();
            let l = split.next().unwrap().parse::<i32>().unwrap();
            let r = split.next().unwrap().parse::<i32>().unwrap();
            (l, r)
        })
        .unzip()
}

fn solve_part_1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    left.into_iter().zip(right).map(|(a, b)| (a - b).abs()).sum()
}

fn solve_part_2(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    let mut right_counts: FxHashMap<i32, i32> = FxHashMap::default();
    for n in right {
        *right_counts.entry(n).or_default() += 1;
    }

    left.into_iter().map(|n| n * right_counts.get(&n).copied().unwrap_or_default()).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day1.txt");

    #[test]
    fn part_1() {
        assert_eq!(11, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(31, solve_part_2(SAMPLE_INPUT));
    }
}
