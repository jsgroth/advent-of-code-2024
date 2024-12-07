//! Day 7: Bridge Repair
//!
//! <https://adventofcode.com/2024/day/7>

use std::error::Error;
use winnow::ascii::{digit1, newline};
use winnow::combinator::{opt, separated, terminated};
use winnow::prelude::*;

#[derive(Debug)]
struct Equation {
    test: u64,
    operands: Vec<u64>,
}

fn parse_u64(input: &mut &str) -> PResult<u64> {
    digit1.parse_to().parse_next(input)
}

fn parse_operands(input: &mut &str) -> PResult<Vec<u64>> {
    separated(1.., parse_u64, ' ').parse_next(input)
}

fn parse_equation(input: &mut &str) -> PResult<Equation> {
    let test = terminated(parse_u64, ": ").parse_next(input)?;
    let operands = parse_operands.parse_next(input)?;
    Ok(Equation { test, operands })
}

fn parse_input(input: &mut &str) -> PResult<Vec<Equation>> {
    let equations = separated(1.., parse_equation, newline).parse_next(input)?;
    opt(newline).parse_next(input)?;
    Ok(equations)
}

fn solve<const PART2: bool>(input: &str) -> u64 {
    let equations = parse_input.parse(input).unwrap();

    equations
        .into_iter()
        .filter(|equation| {
            test_equation::<PART2>(equation.test, equation.operands[0], &equation.operands[1..])
        })
        .map(|equation| equation.test)
        .sum()
}

fn test_equation<const PART2: bool>(test: u64, acc: u64, remaining: &[u64]) -> bool {
    if remaining.is_empty() {
        return acc == test;
    }

    test_add::<PART2>(test, acc, remaining)
        || test_mul::<PART2>(test, acc, remaining)
        || (PART2 && test_concat(test, acc, remaining))
}

fn test_add<const PART2: bool>(test: u64, acc: u64, remaining: &[u64]) -> bool {
    test_equation::<PART2>(test, acc + remaining[0], &remaining[1..])
}

fn test_mul<const PART2: bool>(test: u64, acc: u64, remaining: &[u64]) -> bool {
    test_equation::<PART2>(test, acc * remaining[0], &remaining[1..])
}

fn test_concat(test: u64, acc: u64, remaining: &[u64]) -> bool {
    let operand = remaining[0];
    if operand == 0 {
        return test_equation::<true>(test, 10 * acc, &remaining[1..]);
    }

    let next_acc = acc * 10_u64.pow(operand.ilog10() + 1) + operand;
    test_equation::<true>(test, next_acc, &remaining[1..])
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve::<false>, solve::<true>)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day7.txt");

    #[test]
    fn part_1() {
        assert_eq!(3749, solve::<false>(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(11387, solve::<true>(SAMPLE_INPUT));
    }
}
