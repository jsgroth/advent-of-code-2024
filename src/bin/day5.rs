//! Day 5: Print Queue
//!
//! <https://adventofcode.com/2024/day/5>

use std::error::Error;
use winnow::ascii::{digit1, newline};
use winnow::combinator::{opt, repeat, separated, separated_pair, terminated};
use winnow::prelude::*;

#[derive(Debug)]
struct Input {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

fn parse_u32(input: &mut &str) -> PResult<u32> {
    digit1.parse_to().parse_next(input)
}

fn parse_rule(input: &mut &str) -> PResult<(u32, u32)> {
    separated_pair(parse_u32, '|', parse_u32).parse_next(input)
}

fn parse_rules(input: &mut &str) -> PResult<Vec<(u32, u32)>> {
    repeat(1.., terminated(parse_rule, newline)).parse_next(input)
}

fn parse_update(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., parse_u32, ',').parse_next(input)
}

fn parse_input(input: &mut &str) -> PResult<Input> {
    let rules = parse_rules.parse_next(input)?;
    newline.parse_next(input)?;

    let updates: Vec<_> = separated(1.., parse_update, newline).parse_next(input)?;
    opt(newline).parse_next(input)?;

    Ok(Input { rules, updates })
}

fn solve_part_1(input: &str) -> u32 {
    let Input { rules, updates } = parse_input.parse(input).unwrap();

    let mut sum = 0;
    'outer: for update in &updates {
        for &rule in &rules {
            if let Some((before_idx, after_idx)) = find_rule_indices(rule, update) {
                if before_idx > after_idx {
                    continue 'outer;
                }
            }
        }

        sum += update[update.len() / 2];
    }

    sum
}

fn find_rule_indices(rule: (u32, u32), update: &[u32]) -> Option<(usize, usize)> {
    let before_idx = update.iter().position(|&value| value == rule.0);
    let after_idx = update.iter().position(|&value| value == rule.1);
    match (before_idx, after_idx) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

fn solve_part_2(input: &str) -> u32 {
    let Input { rules, updates } = parse_input.parse(input).unwrap();

    let mut sum = 0;
    for mut update in updates {
        let mut rule_idx = 0;
        let mut reordered = false;
        while rule_idx < rules.len() {
            match find_rule_indices(rules[rule_idx], &update) {
                Some((before_idx, after_idx)) if before_idx > after_idx => {
                    update.swap(before_idx, after_idx);
                    rule_idx = 0;
                    reordered = true;
                }
                _ => {
                    rule_idx += 1;
                }
            }
        }

        if reordered {
            sum += update[update.len() / 2];
        }
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day5.txt");

    #[test]
    fn part_1() {
        assert_eq!(143, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(123, solve_part_2(SAMPLE_INPUT));
    }
}
