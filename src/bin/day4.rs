//! Day 4: Ceres Search
//!
//! <https://adventofcode.com/2024/day/4>

use std::error::Error;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn solve_part_1(input: &str) -> u32 {
    let grid = parse_input(input);

    (0..grid.len())
        .map(|y| {
            (0..grid[0].len())
                .map(|x| count_xmas_starting_at_point(&grid, y as i32, x as i32))
                .sum::<u32>()
        })
        .sum()
}

fn count_xmas_starting_at_point(grid: &[Vec<u8>], y: i32, x: i32) -> u32 {
    if grid[y as usize][x as usize] != b'X' {
        return 0;
    }

    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let mut yy = y + dy;
            let mut xx = x + dx;
            let mut remaining: &[u8] = b"MAS";

            while !remaining.is_empty()
                && (0..grid.len() as i32).contains(&yy)
                && (0..grid[0].len() as i32).contains(&xx)
                && grid[yy as usize][xx as usize] == remaining[0]
            {
                yy += dy;
                xx += dx;
                remaining = &remaining[1..];
            }

            if remaining.is_empty() {
                count += 1;
            }
        }
    }

    count
}

fn solve_part_2(input: &str) -> usize {
    let grid = parse_input(input);

    (0..grid.len())
        .map(|y| {
            (0..grid[0].len())
                .filter(|&x| check_mas_x_centered_at_point(&grid, y, x))
                .count()
        })
        .sum()
}

fn check_mas_x_centered_at_point(grid: &[Vec<u8>], y: usize, x: usize) -> bool {
    if grid[y][x] != b'A' || y == 0 || x == 0 || y == grid.len() - 1 || x == grid[0].len() - 1 {
        // Can't be centered at a boundary row or column
        return false;
    }

    let top_left = grid[y - 1][x - 1];
    if ![b'M', b'S'].contains(&top_left) {
        return false;
    }

    let other = if top_left == b'M' { b'S' } else { b'M' };

    if grid[y - 1][x + 1] == top_left {
        // Top right matches top left; bottom left and bottom right must both be other
        grid[y + 1][x - 1] == other && grid[y + 1][x + 1] == other
    } else if grid[y + 1][x - 1] == top_left {
        // Bottom left matches top left; top right and bottom right must both be other
        grid[y - 1][x + 1] == other && grid[y + 1][x + 1] == other
    } else {
        // Not a match
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day4.txt");

    #[test]
    fn part_1() {
        assert_eq!(18, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(9, solve_part_2(SAMPLE_INPUT));
    }
}
