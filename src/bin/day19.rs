//! Day 19: Linen Layout
//!
//! <https://adventofcode.com/2024/day/19>

use rustc_hash::FxHashMap;
use std::error::Error;

#[derive(Debug)]
struct Input<'a> {
    towels: Vec<&'a [u8]>,
    designs: Vec<&'a [u8]>,
}

fn parse_input(input: &str) -> Input<'_> {
    let mut lines = input.lines();

    let towels: Vec<_> = lines.next().unwrap().split(", ").map(|s| s.as_bytes()).collect();
    lines.next();
    let designs: Vec<_> =
        lines.filter(|line| !line.is_empty()).map(|line| line.as_bytes()).collect();

    Input { towels, designs }
}

fn solve_part_1(input: &str) -> usize {
    let Input { towels, designs } = parse_input(input);

    designs.into_iter().filter(|&design| is_design_possible(&towels, design)).count()
}

fn is_design_possible(towels: &[&[u8]], design: &[u8]) -> bool {
    if design.is_empty() {
        return true;
    }

    towels.iter().any(|&towel| {
        towel.len() <= design.len()
            && towel == &design[..towel.len()]
            && is_design_possible(towels, &design[towel.len()..])
    })
}

fn solve_part_2(input: &str) -> u64 {
    let Input { towels, designs } = parse_input(input);

    let mut cache = FxHashMap::default();
    designs.into_iter().map(|design| ways_to_make_design(&towels, design, &mut cache)).sum()
}

fn ways_to_make_design<'a>(
    towels: &[&[u8]],
    design: &'a [u8],
    cache: &mut FxHashMap<&'a [u8], u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(&count) = cache.get(&design) {
        return count;
    }

    let sum = towels
        .iter()
        .map(|&towel| {
            if towel.len() <= design.len() && towel == &design[..towel.len()] {
                ways_to_make_design(towels, &design[towel.len()..], cache)
            } else {
                0
            }
        })
        .sum::<u64>();

    cache.insert(design, sum);
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day19.txt");

    #[test]
    fn part_1() {
        assert_eq!(6, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(16, solve_part_2(SAMPLE_INPUT));
    }
}
