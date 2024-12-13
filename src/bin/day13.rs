//! Day 13: Claw Contraption
//!
//! <https://adventofcode.com/2024/day/13>

use advent_of_code_2024::Pos2;
use std::error::Error;
use winnow::ascii::{digit1, newline};
use winnow::combinator::{opt, preceded, separated, separated_pair, terminated};
use winnow::prelude::*;

type Position = Pos2<i64>;

#[derive(Debug, Clone)]
struct Machine {
    a: Position,
    b: Position,
    prize: Position,
}

fn parse_i64(input: &mut &str) -> PResult<i64> {
    digit1.parse_to().parse_next(input)
}

fn parse_button(button: &'static str) -> impl FnMut(&mut &str) -> PResult<Position> {
    move |input| {
        ("Button ", button, ": ").parse_next(input)?;

        let (x, y) = separated_pair(preceded("X+", parse_i64), ", ", preceded("Y+", parse_i64))
            .parse_next(input)?;

        Ok(Position { x, y })
    }
}

fn parse_prize(input: &mut &str) -> PResult<Position> {
    "Prize: ".parse_next(input)?;

    let (x, y) = separated_pair(preceded("X=", parse_i64), ", ", preceded("Y=", parse_i64))
        .parse_next(input)?;

    Ok(Position { x, y })
}

fn parse_machine(input: &mut &str) -> PResult<Machine> {
    let a = terminated(parse_button("A"), newline).parse_next(input)?;
    let b = terminated(parse_button("B"), newline).parse_next(input)?;
    let prize = terminated(parse_prize, opt(newline)).parse_next(input)?;

    Ok(Machine { a, b, prize })
}

fn parse_input(input: &mut &str) -> PResult<Vec<Machine>> {
    separated(1.., parse_machine, newline).parse_next(input)
}

// 10 trillion
const PART_2_ADJUSTMENT: i64 = 10_000_000_000_000;

fn solve<const PART2: bool>(input: &str) -> i64 {
    let machines = parse_input.parse(input).unwrap();

    // Assert no 0s in input
    assert!(machines.iter().all(|machine| machine.a.x != 0
        && machine.a.y != 0
        && machine.b.x != 0
        && machine.b.y != 0));

    let mut total = 0;
    for machine in machines {
        let prize = if PART2 {
            machine.prize + Position { x: PART_2_ADJUSTMENT, y: PART_2_ADJUSTMENT }
        } else {
            machine.prize
        };

        if let Some((a, b)) = solve_equation(machine.a, machine.b, prize) {
            total += 3 * a + b;
        }
    }

    total
}

// The problem can be represented as a system of 2 linear equations:
//   A * ax + B * bx = px
//   A * ay + B * by = py
// Where A and B are unknown variables, and ax/ay/bx/by/px/py are constants (the inputs).
//
// If we solve both sides for A and then set both sides equal to each other, we get:
//   (px - B * bx) / ax = (py - B * by) / ay
//
// Solving this for B, we ultimately get:
//   B = (ax * py - ay * px) / (ax * by - bx * ay)
//
// From the original equations, we can also derive a formula for A from B:
//   A = (px - B * bx) / ax
//     OR
//   A = (py - B * by) / ay
// Either of these will produce the same result.
//
// Given the constraints of the problem, a solution is only valid if A and B are both integers, so
// this function checks for that and will return None if either is not an integer.
fn solve_equation(a: Position, b: Position, p: Position) -> Option<(i64, i64)> {
    let b_numerator = a.x * p.y - a.y * p.x;
    let b_denominator = a.x * b.y - b.x * a.y;

    assert_ne!(
        b_denominator, 0,
        "unexpected input; equation has infinite solutions for a={a:?} b={b:?} p={p:?}"
    );
    if b_numerator % b_denominator != 0 {
        // B is not an integer
        return None;
    }

    let b_solution = b_numerator / b_denominator;
    let a_numerator = p - b * b_solution;
    if a_numerator.x % a.x != 0 || a_numerator.y % a.y != 0 {
        // A is not an integer
        return None;
    }

    let a_solution = a_numerator.x / a.x;
    Some((a_solution, b_solution))
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve::<false>, solve::<true>)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day13.txt");

    #[test]
    fn part_1() {
        assert_eq!(480, solve::<false>(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(875318608908, solve::<true>(SAMPLE_INPUT));
    }
}
