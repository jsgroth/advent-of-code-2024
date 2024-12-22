//! Day 22: Monkey Market
//!
//! <https://adventofcode.com/2024/day/22>

use rustc_hash::{FxHashMap, FxHashSet};
use std::error::Error;

fn solve_part_1(input: &str) -> i64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut number: i64 = line.parse().unwrap();
            for _ in 0..2000 {
                number = next_secret_number(number);
            }
            number
        })
        .sum()
}

fn next_secret_number(mut number: i64) -> i64 {
    const MODULO: i64 = 16_777_216;

    number ^= number * 64;
    number %= MODULO;

    number ^= number / 32;
    number %= MODULO;

    number ^= number * 2048;
    number %= MODULO;

    number
}

fn solve_part_2(input: &str) -> i64 {
    let numbers: Vec<i64> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut changes_to_bananas: FxHashMap<[i64; 4], i64> = FxHashMap::default();
    let mut changes_for_number: FxHashSet<[i64; 4]> = FxHashSet::default();
    for &start_number in &numbers {
        changes_for_number.clear();

        let mut number = start_number;
        let mut changes = [i64::MAX; 4];
        for _ in 0..2000 {
            let next_number = next_secret_number(number);
            let difference = (next_number % 10) - (number % 10);
            push_change(&mut changes, difference);

            if changes[0] != i64::MAX && changes_for_number.insert(changes) {
                let bananas = next_number % 10;
                *changes_to_bananas.entry(changes).or_default() += bananas;
            }

            number = next_number;
        }
    }

    *changes_to_bananas.values().max().unwrap()
}

fn push_change(numbers: &mut [i64; 4], number: i64) {
    numbers[0] = numbers[1];
    numbers[1] = numbers[2];
    numbers[2] = numbers[3];
    numbers[3] = number;
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day22.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample/day22-2.txt");

    #[test]
    fn part_1() {
        assert_eq!(37327623, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(23, solve_part_2(SAMPLE_INPUT_2));
    }
}
