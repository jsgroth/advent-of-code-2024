//! Day 9: Disk Fragmenter
//!
//! <https://adventofcode.com/2024/day/9>

use std::error::Error;
use std::iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Occupied(u64),
}

fn parse_input(input: &str) -> Vec<Space> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .enumerate()
        .flat_map(|(i, c)| {
            let size = c.to_digit(10).unwrap();
            match i % 2 {
                0 => {
                    let id = (i / 2) as u64;
                    iter::repeat_n(Space::Occupied(id), size as usize)
                }
                1 => iter::repeat_n(Space::Empty, size as usize),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn solve_part_1(input: &str) -> u64 {
    let mut disk = parse_input(input);

    let mut i = 0;
    let mut j = disk.len() - 1;
    loop {
        while i < disk.len() && matches!(disk[i], Space::Occupied(..)) {
            i += 1;
        }

        while j > 0 && disk[j] == Space::Empty {
            j -= 1;
        }

        if i > j {
            break;
        }

        disk[i] = disk[j];
        disk[j] = Space::Empty;
    }

    evaluate_disk(&disk)
}

fn evaluate_disk(disk: &[Space]) -> u64 {
    disk.iter()
        .enumerate()
        .map(|(i, &space)| match space {
            Space::Occupied(id) => id * i as u64,
            Space::Empty => 0,
        })
        .sum()
}

fn solve_part_2(input: &str) -> u64 {
    let mut disk = parse_input(input);
    let mut empty_spaces = find_empty_spaces(&disk);

    let mut max_id = u64::MAX;
    let mut j = disk.len() - 1;
    loop {
        // Move pointer left one-by-one until a file is hit
        while j > 0 && disk[j] == Space::Empty {
            j -= 1;
        }

        if j == 0 {
            // Reached the leftmost edge of the disk
            break;
        }

        let Space::Occupied(id) = disk[j] else { unreachable!() };

        // Find the left edge of this file
        let mut jj = j;
        while jj > 0 && disk[jj - 1] == disk[j] {
            jj -= 1;
        }

        if id >= max_id {
            // This file was moved left in a previous iteration of the loop - skip it
            j = jj.saturating_sub(1);
            continue;
        }
        max_id = id;

        let occupied_len = j - jj + 1;
        for empty_space in &mut empty_spaces {
            if empty_space.start > j {
                // No sufficiently large empty spaces to the left of this file
                break;
            }

            if empty_space.len >= occupied_len {
                // Sufficient empty space found; move the file and shrink the empty space
                disk[empty_space.start..empty_space.start + occupied_len].fill(Space::Occupied(id));
                disk[jj..=j].fill(Space::Empty);

                empty_space.start += occupied_len;
                empty_space.len -= occupied_len;

                // Don't bother removing empty spaces of length 0; that's an O(N) operation without
                // a fancier data structure

                break;
            }
        }

        // Move pointer to the left of the file
        j = jj.saturating_sub(1);
    }

    evaluate_disk(&disk)
}

#[derive(Debug)]
struct EmptySpace {
    start: usize,
    len: usize,
}

fn find_empty_spaces(disk: &[Space]) -> Vec<EmptySpace> {
    let mut empty_spaces = Vec::new();
    let mut i = 0;
    loop {
        while i < disk.len() && matches!(disk[i], Space::Occupied(..)) {
            i += 1;
        }

        if i == disk.len() {
            return empty_spaces;
        }

        let mut ii = i;
        while ii < disk.len() && disk[ii] == Space::Empty {
            ii += 1;
        }
        empty_spaces.push(EmptySpace { start: i, len: ii - i });

        i = ii;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day9.txt");

    #[test]
    fn part_1() {
        assert_eq!(1928, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(2858, solve_part_2(SAMPLE_INPUT));
    }
}
