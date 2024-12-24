//! Day 24: Crossed Wires
//!
//! <https://adventofcode.com/2024/day/24>

use rustc_hash::{FxHashMap, FxHashSet};
use std::error::Error;
use std::hash::Hash;
use std::rc::Rc;
use winnow::ascii::{alphanumeric1, newline};
use winnow::combinator::{alt, opt, repeat, separated, separated_pair, terminated};
use winnow::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Logic {
    And,
    Or,
    Xor,
}

impl Logic {
    fn apply(self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    input: (Rc<str>, Rc<str>),
    logic: Logic,
    output: Rc<str>,
}

#[derive(Debug)]
struct Input {
    start_wires: Vec<(Rc<str>, bool)>,
    gates: Vec<Gate>,
}

fn parse_bit(input: &mut &str) -> PResult<bool> {
    let digit = alt(('0', '1')).parse_next(input)?;
    Ok(digit == '1')
}

fn parse_start_wire(input: &mut &str) -> PResult<(Rc<str>, bool)> {
    let (wire, bit) = separated_pair(alphanumeric1, ": ", parse_bit).parse_next(input)?;
    Ok((wire.into(), bit))
}

fn parse_and(input: &mut &str) -> PResult<Logic> {
    " AND ".parse_next(input)?;
    Ok(Logic::And)
}

fn parse_or(input: &mut &str) -> PResult<Logic> {
    " OR ".parse_next(input)?;
    Ok(Logic::Or)
}

fn parse_xor(input: &mut &str) -> PResult<Logic> {
    " XOR ".parse_next(input)?;
    Ok(Logic::Xor)
}

fn parse_logic(input: &mut &str) -> PResult<Logic> {
    alt((parse_and, parse_or, parse_xor)).parse_next(input)
}

fn parse_gate(input: &mut &str) -> PResult<Gate> {
    let ((input0, logic, input1), output) =
        separated_pair((alphanumeric1, parse_logic, alphanumeric1), " -> ", alphanumeric1)
            .parse_next(input)?;

    Ok(Gate { input: (input0.into(), input1.into()), logic, output: output.into() })
}

fn parse_input(input: &mut &str) -> PResult<Input> {
    let start_wires = repeat(1.., terminated(parse_start_wire, newline)).parse_next(input)?;
    newline.parse_next(input)?;
    let gates = separated(1.., parse_gate, newline).parse_next(input)?;
    opt(newline).parse_next(input)?;

    Ok(Input { start_wires, gates })
}

fn solve_part_1(input: &str) -> u64 {
    let Input { start_wires, gates } = parse_input.parse(input).unwrap();

    let mut wires_map: FxHashMap<Rc<str>, bool> = start_wires.into_iter().collect();
    let gate_map = build_gate_map(&gates);

    let mut result: u64 = 0;
    for z_wire in gate_map.keys().filter(|wire| wire.starts_with('z')) {
        let bit =
            evaluate_wire(z_wire, &gate_map, &mut wires_map, &mut FxHashSet::default()).unwrap();
        let bit_idx: u32 = z_wire[1..].parse().unwrap();
        result |= u64::from(bit) << bit_idx;
    }

    result
}

fn build_gate_map(gates: &[Gate]) -> FxHashMap<Rc<str>, Gate> {
    gates.iter().map(|gate| (gate.output.clone(), gate.clone())).collect()
}

// Returns None if there is a cycle that prevents evaluation
fn evaluate_wire(
    wire: &Rc<str>,
    gates: &FxHashMap<Rc<str>, Gate>,
    wires: &mut FxHashMap<Rc<str>, bool>,
    evaluating: &mut FxHashSet<Rc<str>>,
) -> Option<bool> {
    if let Some(&output) = wires.get(wire) {
        return Some(output);
    }

    if !evaluating.insert(wire.clone()) {
        // There is a cycle; can happen after swapping outputs
        return None;
    }

    let gate = gates.get(wire).unwrap();

    let input0 = evaluate_wire(&gate.input.0, gates, wires, evaluating)?;
    let input1 = evaluate_wire(&gate.input.1, gates, wires, evaluating)?;
    let output = gate.logic.apply(input0, input1);

    wires.insert(wire.clone(), output);
    Some(output)
}

fn solve_part_2(input: &str, op: impl Copy + Fn(u64, u64) -> u64) -> String {
    let Input { start_wires, gates } = parse_input.parse(input).unwrap();

    let output_wires: Vec<_> = gates.iter().map(|gate| gate.output.clone()).collect();
    let mut gate_map = build_gate_map(&gates);

    let start_wire_keys = start_wires.iter().map(|(key, _)| key);
    let x_strs = all_keys_with_prefix('x', start_wire_keys.clone());
    let y_strs = all_keys_with_prefix('y', start_wire_keys);
    let z_strs = all_keys_with_prefix('z', gate_map.keys());

    let mut swapped = Vec::new();
    for bit in 0..z_strs.len() {
        if !is_valid_for_bit(bit, op, &gate_map, &x_strs, &y_strs, &z_strs) {
            swap_to_fix_bit(
                bit,
                op,
                &output_wires,
                &mut gate_map,
                &x_strs,
                &y_strs,
                &z_strs,
                &mut swapped,
            );
        }
    }

    swapped.sort();
    swapped.join(",")
}

fn all_keys_with_prefix<'a>(prefix: char, keys: impl Iterator<Item = &'a Rc<str>>) -> Vec<Rc<str>> {
    let mut keys: Vec<_> = keys.filter(|&wire| wire.starts_with(prefix)).cloned().collect();
    keys.sort();
    keys
}

fn is_valid_for_bit(
    bit: usize,
    op: impl Fn(u64, u64) -> u64,
    gate_map: &FxHashMap<Rc<str>, Gate>,
    x_strs: &[Rc<str>],
    y_strs: &[Rc<str>],
    z_strs: &[Rc<str>],
) -> bool {
    let mut input_wires: FxHashMap<Rc<str>, bool> = FxHashMap::default();

    // There is almost definitely a better way to do this than testing 100 random sums, but this seems to work
    for _ in 0..100 {
        let x = rand::random::<u64>() & ((1 << x_strs.len()) - 1);
        for (i, x_str) in x_strs.iter().enumerate() {
            input_wires.insert(x_str.clone(), x & (1 << i) != 0);
        }

        let y = rand::random::<u64>() & ((1 << y_strs.len()) - 1);
        for (i, y_str) in y_strs.iter().enumerate() {
            input_wires.insert(y_str.clone(), y & (1 << i) != 0);
        }

        let Some(z_bit) = evaluate_wire(
            &z_strs[bit],
            gate_map,
            &mut input_wires.clone(),
            &mut FxHashSet::default(),
        ) else {
            // Cycle exists; definitely not valid
            return false;
        };

        let expected_z_bit = op(x, y) & (1 << bit) != 0;
        if expected_z_bit != z_bit {
            return false;
        }
    }

    true
}

#[allow(clippy::too_many_arguments)]
fn swap_to_fix_bit(
    bit: usize,
    op: impl Copy + Fn(u64, u64) -> u64,
    output_wires: &[Rc<str>],
    gate_map: &mut FxHashMap<Rc<str>, Gate>,
    x_strs: &[Rc<str>],
    y_strs: &[Rc<str>],
    z_strs: &[Rc<str>],
    swapped: &mut Vec<Rc<str>>,
) {
    for i in 0..output_wires.len() {
        for j in i + 1..output_wires.len() {
            let mut swapped_gate_map = gate_map.clone();
            hashmap_swap(&mut swapped_gate_map, output_wires[i].clone(), output_wires[j].clone());

            if is_valid_for_bit(bit, op, &swapped_gate_map, x_strs, y_strs, z_strs) {
                *gate_map = swapped_gate_map;
                swapped.extend([output_wires[i].clone(), output_wires[j].clone()]);
                return;
            }
        }
    }

    panic!("No valid swap found for bit {bit}");
}

fn hashmap_swap<K: Eq + Hash, V>(map: &mut FxHashMap<K, V>, k1: K, k2: K) {
    let mut t = map.remove(&k1).unwrap();
    t = map.insert(k2, t).unwrap();
    map.insert(k1, t);
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, |input| solve_part_2(input, |a, b| a + b))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day24.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample/day24-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("../../sample/day24-3.txt");

    #[test]
    fn part_1() {
        assert_eq!(4, solve_part_1(SAMPLE_INPUT));
        assert_eq!(2024, solve_part_1(SAMPLE_INPUT_2));
    }

    #[test]
    fn part_2() {
        assert_eq!("z00,z01,z02,z05", solve_part_2(SAMPLE_INPUT_3, |a, b| a & b));
    }
}
