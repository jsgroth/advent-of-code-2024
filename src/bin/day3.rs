//! Day 3: Mull It Over
//!
//! <https://adventofcode.com/2024/day/3>

use std::error::Error;
use winnow::ascii::digit1;
use winnow::combinator::{separated_pair, terminated};
use winnow::prelude::*;

fn parse_i32(input: &mut &str) -> PResult<i32> {
    digit1.parse_to().parse_next(input)
}

fn parse_mul_suffix(input: &mut &str) -> PResult<(i32, i32)> {
    terminated(separated_pair(parse_i32, ',', parse_i32), ')').parse_next(input)
}

fn solve<const PART2: bool>(mut input: &str) -> i32 {
    let mut enabled = true;
    let mut sum = 0;

    while input.len() >= 4 {
        if enabled && input.starts_with("mul(") {
            input = &input["mul(".len()..];
            if let Ok((l, r)) = parse_mul_suffix(&mut input) {
                sum += l * r;
            }
        } else if PART2 && input.starts_with("do()") {
            enabled = true;
            input = &input["do()".len()..];
        } else if PART2 && input.starts_with("don't()") {
            enabled = false;
            input = &input["don't()".len()..];
        } else {
            input = &input[1..];
        }
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve::<false>, solve::<true>)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day3.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample/day3-2.txt");

    #[test]
    fn part_1() {
        assert_eq!(161, solve::<false>(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(48, solve::<true>(SAMPLE_INPUT_2));
    }
}
