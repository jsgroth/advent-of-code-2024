//! Day 17: Chronospatial Computer
//!
//! <https://adventofcode.com/2024/day/17>

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{cmp, env};
use winnow::ascii::{digit1, newline};
use winnow::combinator::{opt, preceded, separated, terminated};
use winnow::prelude::*;

#[derive(Debug)]
struct Input {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
}

fn parse_num<T: FromStr>(input: &mut &str) -> PResult<T> {
    digit1.parse_to().parse_next(input)
}

fn parse_register(name: &'static str) -> impl FnMut(&mut &str) -> PResult<u64> {
    move |input| {
        let name_parser = ("Register ", name, ": ");
        terminated(preceded(name_parser, parse_num), newline).parse_next(input)
    }
}

fn parse_program(input: &mut &str) -> PResult<Vec<u8>> {
    preceded("Program: ", separated(1.., parse_num::<u8>, ',')).parse_next(input)
}

fn parse_input(input: &mut &str) -> PResult<Input> {
    let (a, b, c) =
        (parse_register("A"), parse_register("B"), parse_register("C")).parse_next(input)?;
    newline.parse_next(input)?;
    let program = terminated(parse_program, opt(newline)).parse_next(input)?;

    Ok(Input { a, b, c, program })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComboOperand {
    Literal(u8),
    A,
    B,
    C,
}

impl ComboOperand {
    fn from_operand(operand: u8) -> Self {
        match operand {
            0..=3 => Self::Literal(operand),
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            _ => panic!("Invalid combo operand: {operand}"),
        }
    }

    fn value(self, a: u64, b: u64, c: u64) -> u64 {
        match self {
            Self::Literal(literal) => literal.into(),
            Self::A => a,
            Self::B => b,
            Self::C => c,
        }
    }
}

impl Display for ComboOperand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(u8),
    Bst(ComboOperand),
    Jnz(u8),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Adv(operand) => write!(f, "A >>= {operand}"),
            Self::Bxl(operand) => write!(f, "B ^= {operand}"),
            Self::Bst(operand) => write!(f, "B = {operand} & 7"),
            Self::Jnz(operand) => write!(f, "JNZ {operand}"),
            Self::Bxc => write!(f, "B ^= C"),
            Self::Out(operand) => write!(f, "OUT ({operand} & 7)"),
            Self::Bdv(operand) => write!(f, "B = A >> {operand}"),
            Self::Cdv(operand) => write!(f, "C = A >> {operand}"),
        }
    }
}

fn disassemble(program: &[u8]) -> Vec<Instruction> {
    assert!(program.len() % 2 == 0 && program.iter().all(|&opcode| opcode < 8));

    program
        .chunks_exact(2)
        .map(|chunk| {
            let &[opcode, operand] = chunk else { unreachable!() };

            match opcode {
                0 => Instruction::Adv(ComboOperand::from_operand(operand)),
                1 => Instruction::Bxl(operand),
                2 => Instruction::Bst(ComboOperand::from_operand(operand)),
                3 => Instruction::Jnz(operand),
                4 => Instruction::Bxc,
                5 => Instruction::Out(ComboOperand::from_operand(operand)),
                6 => Instruction::Bdv(ComboOperand::from_operand(operand)),
                7 => Instruction::Cdv(ComboOperand::from_operand(operand)),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn run_program(mut a: u64, mut b: u64, mut c: u64, program: &[Instruction]) -> Vec<u8> {
    let mut ip = 0;
    let mut out = Vec::new();
    while ip < program.len() {
        let instruction = program[ip];
        ip += 1;

        match instruction {
            Instruction::Adv(operand) => {
                let shift = operand.value(a, b, c);
                a >>= shift;
            }
            Instruction::Bxl(operand) => {
                b ^= u64::from(operand);
            }
            Instruction::Bst(operand) => {
                b = operand.value(a, b, c) & 7;
            }
            Instruction::Jnz(operand) => {
                if a != 0 {
                    ip = (operand >> 1).into();
                }
            }
            Instruction::Bxc => {
                b ^= c;
            }
            Instruction::Out(operand) => {
                out.push((operand.value(a, b, c) & 7) as u8);
            }
            Instruction::Bdv(operand) => {
                let shift = operand.value(a, b, c);
                b = a >> shift;
            }
            Instruction::Cdv(operand) => {
                let shift = operand.value(a, b, c);
                c = a >> shift;
            }
        }
    }

    out
}

fn solve_part_1(input: &str) -> String {
    let Input { a, b, c, program } = parse_input.parse(input).unwrap();

    assert!(program.iter().all(|&opcode| opcode < 8));

    let instructions = disassemble(&program);
    let out = run_program(a, b, c, &instructions);
    let out: Vec<_> = out.iter().map(u8::to_string).collect();

    out.join(",")
}

fn solve_part_2(input: &str) -> u64 {
    let Input { program, .. } = parse_input.parse(input).unwrap();

    let instructions = disassemble(&program);

    // TODO Do all inputs look like this? What is actually variable between inputs?
    let Instruction::Bxl(first_xor) = instructions[1] else {
        panic!("Unexpected second instruction, expected bxl: {}", instructions[1]);
    };
    let Instruction::Bxl(second_xor) = instructions[3] else {
        panic!("Unexpected fourth instruction, expected bxl: {}", instructions[3]);
    };

    let mut searcher = SolutionSearcher::new(first_xor.into(), second_xor.into());
    searcher.search(0, 0, 10, &program, 0);
    searcher.solutions.into_iter().min().expect("No solution found")
}

struct SolutionSearcher {
    first_xor: u64,
    second_xor: u64,
    solutions: Vec<u64>,
}

impl SolutionSearcher {
    fn new(first_xor: u64, second_xor: u64) -> Self {
        Self { first_xor, second_xor, solutions: Vec::new() }
    }

    fn search(&mut self, a: u64, acc: u64, free_bits: u8, program: &[u8], program_idx: usize) {
        if program_idx == program.len() {
            if a == 0 {
                self.solutions.push(acc);
            }
            return;
        }

        let target: u64 = program[program_idx].into();
        for high_bits in 0..1 << free_bits {
            let new_a = a | (high_bits << (10 - free_bits));
            let shift = (new_a & 7) ^ self.first_xor;

            if (new_a & 7) ^ self.first_xor ^ self.second_xor ^ (new_a >> shift) == target {
                let used_bits = shift;
                let next_free_bits = cmp::min((10 - used_bits) as u8, free_bits + 3);
                let next_acc = acc | ((new_a & 7) << (3 * program_idx));

                self.search(new_a >> 3, next_acc, next_free_bits, program, program_idx + 1);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    if env::args().any(|arg| arg.as_str() == "--print-program") {
        let Input { program, .. } = parse_input.parse(&advent_of_code_2024::read_input()?).unwrap();
        let instructions = disassemble(&program);
        for (i, instruction) in instructions.into_iter().enumerate() {
            println!("{}: {instruction}", 2 * i);
        }
        return Ok(());
    }

    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day17.txt");

    #[test]
    fn part_1() {
        assert_eq!("4,6,3,5,6,3,5,2,1,0", solve_part_1(SAMPLE_INPUT).as_str());
    }
}
