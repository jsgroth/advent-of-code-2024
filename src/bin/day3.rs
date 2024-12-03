//! Day 3: Mull It Over
//!
//! A regex solution would be much shorter and arguably cleaner, but this state machine approach
//! seems to be significantly faster, especially for part 2
//!
//! <https://adventofcode.com/2024/day/3>

use std::error::Error;

enum ParseState {
    Start,                             //
    M,                                 // m
    Mu,                                // mu
    Mul,                               // mul
    MulParen,                          // mul(
    MulParenOp(String),                // mul(\d+
    MulParenOpComma(String),           // mul(\d+,
    MulParenOpCommaOp(String, String), // mul(\d+,\d+
    D,                                 // d
    Do,                                // do
    DoParen,                           // do(
    Don,                               // don
    DonApostrophe,                     // don'
    Dont,                              // don't
    DontParen,                         // don't(
}

fn solve<const PART2: bool>(input: &str) -> i32 {
    use ParseState::*;

    let mut enabled = true;
    let mut sum = 0;
    let mut state = Start;
    for c in input.chars() {
        state = match (state, c) {
            (Start, 'm') => M,
            (M, 'u') => Mu,
            (Mu, 'l') => Mul,
            (Mul, '(') => MulParen,
            (MulParen, '0'..='9') => MulParenOp(c.to_string()),
            (MulParenOp(mut op), '0'..='9') => {
                op.push(c);
                MulParenOp(op)
            }
            (MulParenOp(op), ',') => MulParenOpComma(op),
            (MulParenOpComma(op), '0'..='9') => MulParenOpCommaOp(op, c.to_string()),
            (MulParenOpCommaOp(l_op, mut r_op), '0'..='9') => {
                r_op.push(c);
                MulParenOpCommaOp(l_op, r_op)
            }
            (MulParenOpCommaOp(l_op, r_op), ')') => {
                if enabled || !PART2 {
                    let l: i32 = l_op.parse().unwrap();
                    let r: i32 = r_op.parse().unwrap();
                    sum += l * r;
                }
                Start
            }
            (Start, 'd') => D,
            (D, 'o') => Do,
            (Do, '(') => DoParen,
            (DoParen, ')') => {
                enabled = true;
                Start
            }
            (Do, 'n') => Don,
            (Don, '\'') => DonApostrophe,
            (DonApostrophe, 't') => Dont,
            (Dont, '(') => DontParen,
            (DontParen, ')') => {
                enabled = false;
                Start
            }
            _ => Start,
        };
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
