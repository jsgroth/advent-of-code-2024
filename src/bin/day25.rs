//! Day 25: Code Chronicle
//!
//! <https://adventofcode.com/2024/day/25>

use std::error::Error;
use std::iter;

#[derive(Debug)]
struct Input {
    total_height: u32,
    lock_heights: Vec<Vec<u32>>,
    key_heights: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let mut total_height: Option<u32> = None;
    let mut lock_heights = Vec::new();
    let mut key_heights = Vec::new();
    loop {
        let schematic: Vec<Vec<_>> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        if schematic.is_empty() {
            break;
        }

        let schematic_height = schematic.len() as u32;
        assert!(
            total_height.is_none_or(|total_height| total_height == schematic_height),
            "Mismatched total heights in input"
        );
        total_height = Some(schematic_height);

        if schematic[0].iter().all(|&b| b) {
            lock_heights.push(convert_to_heights(schematic));
        } else {
            // Convert heights to (schematic_height - height) because keys go up from the bottom
            key_heights.push(
                convert_to_heights(schematic)
                    .into_iter()
                    .map(|height| schematic_height - height)
                    .collect(),
            );
        }
    }

    Input { total_height: total_height.expect("Input is empty"), lock_heights, key_heights }
}

fn convert_to_heights(schematic: Vec<Vec<bool>>) -> Vec<u32> {
    let mut heights = Vec::new();

    let first = schematic[0][0];
    for col in 0..schematic[0].len() {
        let mut row = 1;
        while row < schematic.len() && schematic[row][col] == first {
            row += 1;
        }

        heights.push(row as u32);
    }

    heights
}

fn solve_part_1(input: &str) -> usize {
    let Input { total_height, lock_heights, key_heights } = parse_input(input);

    lock_heights
        .iter()
        .map(|lock| {
            key_heights.iter().filter(|&key| lock_matches_key(total_height, lock, key)).count()
        })
        .sum()
}

fn lock_matches_key(total_height: u32, lock_heights: &[u32], key_heights: &[u32]) -> bool {
    iter::zip(lock_heights, key_heights)
        .all(|(&lock_height, &key_height)| lock_height + key_height <= total_height)
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, |_input| String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day25.txt");

    #[test]
    fn part_1() {
        assert_eq!(3, solve_part_1(SAMPLE_INPUT));
    }
}
