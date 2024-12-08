//! Day 8: Resonant Collinearity
//!
//! <https://adventofcode.com/2024/day/8>

use rustc_hash::{FxHashMap, FxHashSet};
use std::error::Error;
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Antenna(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { row: self.row + rhs.row, col: self.col + rhs.col }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { row: self.row - rhs.row, col: self.col - rhs.col }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    _ => Space::Antenna(c as u8),
                })
                .collect()
        })
        .collect()
}

fn solve<const PART2: bool>(input: &str) -> usize {
    let map = parse_input(input);
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    let antenna_positions = build_positions_map(&map);

    let mut result: FxHashSet<Position> = FxHashSet::default();
    for positions in antenna_positions.values() {
        if positions.len() < 2 {
            // Doesn't seem to happen in the input, but there can't be an antinode for a character
            // with only one antenna
            continue;
        }

        for i in 0..positions.len() {
            if PART2 {
                // For part 2, every antenna position is a valid antinode location
                result.insert(positions[i]);
            }

            for j in i + 1..positions.len() {
                for (p1, p2) in [(positions[i], positions[j]), (positions[j], positions[i])] {
                    let delta = p2 - p1;

                    let mut current_pos = p2 + delta;
                    while (0..rows).contains(&current_pos.row)
                        && (0..cols).contains(&current_pos.col)
                    {
                        result.insert(current_pos);
                        current_pos += delta;

                        if !PART2 {
                            // For part 1, only the first position on the line is a valid antinode location
                            break;
                        }
                    }
                }
            }
        }
    }

    result.len()
}

fn build_positions_map(map: &[Vec<Space>]) -> FxHashMap<u8, Vec<Position>> {
    let mut antenna_positions: FxHashMap<u8, Vec<Position>> = FxHashMap::default();
    for (i, row) in map.iter().enumerate() {
        for (j, &space) in row.iter().enumerate() {
            let Space::Antenna(c) = space else { continue };
            antenna_positions.entry(c).or_default().push(Position::new(i as i32, j as i32));
        }
    }
    antenna_positions
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve::<false>, solve::<true>)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day8.txt");

    #[test]
    fn part_1() {
        assert_eq!(14, solve::<false>(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(34, solve::<true>(SAMPLE_INPUT));
    }
}
