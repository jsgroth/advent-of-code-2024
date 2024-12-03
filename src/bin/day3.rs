//! Day 3: Mull It Over
//!
//! <https://adventofcode.com/2024/day/3>

use std::error::Error;

enum ParseState {
    Start,
    M,
    U,
    L,
    LParen,
    LOperand(String),
    Comma(String),
    ROperand(String, String),
    D,
    O,
    DoLParen,
    N,
    Apostrophe,
    T,
    DontLParen,
}

fn solve<const PART2: bool>(input: &str) -> i32 {
    use ParseState::*;

    let mut enabled = true;
    let mut sum = 0;
    let mut state = Start;
    for c in input.chars() {
        state = match (state, c) {
            (Start, 'm') => M,
            (M, 'u') => U,
            (U, 'l') => L,
            (L, '(') => LParen,
            (LParen, '0'..='9') => LOperand(c.to_string()),
            (LOperand(mut op), '0'..='9') => {
                op.push(c);
                LOperand(op)
            }
            (LOperand(op), ',') => Comma(op),
            (Comma(op), '0'..='9') => ROperand(op, c.to_string()),
            (ROperand(l_op, mut r_op), '0'..='9') => {
                r_op.push(c);
                ROperand(l_op, r_op)
            }
            (ROperand(l_op, r_op), ')') => {
                if enabled || !PART2 {
                    let l: i32 = l_op.parse().unwrap();
                    let r: i32 = r_op.parse().unwrap();
                    sum += l * r;
                }
                Start
            }
            (Start, 'd') => D,
            (D, 'o') => O,
            (O, '(') => DoLParen,
            (DoLParen, ')') => {
                enabled = true;
                Start
            }
            (O, 'n') => N,
            (N, '\'') => Apostrophe,
            (Apostrophe, 't') => T,
            (T, '(') => DontLParen,
            (DontLParen, ')') => {
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
