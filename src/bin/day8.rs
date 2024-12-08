//! Day 8: Resonant Collinearity
//!
//! <https://adventofcode.com/2024/day/8>

use rustc_hash::{FxHashMap, FxHashSet};
use std::error::Error;
use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Antenna(NonZeroU8),
}

fn parse_input(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '.' {
                        Space::Empty
                    } else {
                        Space::Antenna(NonZeroU8::new(c as u8).unwrap())
                    }
                })
                .collect()
        })
        .collect()
}

fn solve_part_1(input: &str) -> usize {
    let grid = parse_input(input);

    let mut positions: FxHashSet<(usize, usize)> = FxHashSet::default();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for ii in 0..grid.len() {
                for jj in 0..grid[ii].len() {
                    let Space::Antenna(c) = grid[ii][jj] else { continue };

                    let di = (ii as i32) - (i as i32);
                    let dj = (jj as i32) - (j as i32);
                    if di == 0 || dj == 0 {
                        continue;
                    }

                    let iii = (i as i32) + 2 * di;
                    let jjj = (j as i32) + 2 * dj;
                    if (0..grid.len() as i32).contains(&iii)
                        && (0..grid[iii as usize].len() as i32).contains(&jjj)
                        && grid[iii as usize][jjj as usize] == Space::Antenna(c)
                    {
                        positions.insert((i, j));
                    }
                }
            }
        }
    }

    positions.len()
}

fn solve_part_2(input: &str) -> usize {
    let grid = parse_input(input);

    let mut char_to_positions: FxHashMap<u8, Vec<(usize, usize)>> = FxHashMap::default();
    for (i, row) in grid.iter().enumerate() {
        for (j, &space) in row.iter().enumerate() {
            let Space::Antenna(c) = space else { continue };
            char_to_positions.entry(c.get()).or_default().push((i, j));
        }
    }

    let mut result: FxHashSet<(usize, usize)> = FxHashSet::default();
    for positions in char_to_positions.values() {
        for i in 0..positions.len() - 1 {
            result.insert(positions[i]);
            for j in i..positions.len() {
                result.insert(positions[j]);

                let ipos = positions[i];
                let jpos = positions[j];

                let drow = (jpos.0 as i32) - (ipos.0 as i32);
                let dcol = (jpos.1 as i32) - (ipos.1 as i32);
                if drow == 0 && dcol == 0 {
                    continue;
                }

                for direction in [-1, 1] {
                    let (mut row, mut col) = if direction == -1 {
                        (ipos.0 as i32 - drow, ipos.1 as i32 - dcol)
                    } else {
                        (jpos.0 as i32 + drow, jpos.1 as i32 + dcol)
                    };

                    while (0..grid.len() as i32).contains(&row)
                        && (0..grid[0].len() as i32).contains(&col)
                    {
                        result.insert((row as usize, col as usize));
                        row += direction * drow;
                        col += direction * dcol;
                    }
                }
            }
        }
    }

    result.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day8.txt");

    #[test]
    fn part_1() {
        assert_eq!(14, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(34, solve_part_2(SAMPLE_INPUT));
    }
}
