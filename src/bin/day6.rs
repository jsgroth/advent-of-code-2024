//! Day 6: Guard Gallivant
//!
//! <https://adventofcode.com/2024/day/6>

use std::error::Error;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up = 1 << 0,
    Left = 1 << 1,
    Right = 1 << 2,
    Down = 1 << 3,
}

impl Direction {
    fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn step(self, row: i32, col: i32) -> (i32, i32) {
        match self {
            Self::Up => (row - 1, col),
            Self::Right => (row, col + 1),
            Self::Down => (row + 1, col),
            Self::Left => (row, col - 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Obstacle,
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<Space>>,
    guard_start: (i32, i32),
}

fn parse_input(input: &str) -> Input {
    let mut map = Vec::new();
    let mut guard_start: Option<(i32, i32)> = None;
    for (row, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut map_row = Vec::new();
        for (col, c) in line.chars().enumerate() {
            map_row.push(if c == '#' {
                Space::Obstacle
            } else {
                Space::Empty
            });
            if c == '^' {
                guard_start = Some((row as i32, col as i32));
            }
        }
        map.push(map_row);
    }

    Input {
        map,
        guard_start: guard_start.expect("No guard position in input"),
    }
}

fn solve_part_1(input: &str) -> usize {
    let Input { map, guard_start } = parse_input(input);

    let TraverseResult::Escaped { visited } = traverse_map(&map, guard_start) else {
        panic!("Failed to escape the map");
    };
    visited
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TraverseResult {
    Escaped { visited: usize },
    Looped,
}

fn traverse_map(map: &[Vec<Space>], start: (i32, i32)) -> TraverseResult {
    let mut visited = vec![vec![0_u8; map[0].len()]; map.len()];

    let (mut row, mut col) = start;
    let mut direction = Direction::Up;
    while (0..map.len() as i32).contains(&row) && (0..map[0].len() as i32).contains(&col) {
        if visited[row as usize][col as usize] & (direction as u8) != 0 {
            return TraverseResult::Looped;
        }
        visited[row as usize][col as usize] |= direction as u8;

        let (next_row, next_col) = direction.step(row, col);
        if (0..map.len() as i32).contains(&next_row)
            && (0..map[0].len() as i32).contains(&next_col)
            && map[next_row as usize][next_col as usize] == Space::Obstacle
        {
            direction = direction.rotate_right();
        } else {
            (row, col) = (next_row, next_col);
        }
    }

    let visited_count = visited
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|&directions| directions != 0)
                .count()
        })
        .sum();

    TraverseResult::Escaped {
        visited: visited_count,
    }
}

fn solve_part_2(input: &str) -> u32 {
    let Input {
        mut map,
        guard_start,
    } = parse_input(input);

    let mut loop_positions = 0;
    for test_row in 0..map.len() {
        for test_col in 0..map[0].len() {
            if map[test_row][test_col] == Space::Obstacle
                || (test_row as i32, test_col as i32) == guard_start
            {
                continue;
            }

            map[test_row][test_col] = Space::Obstacle;
            if traverse_map(&map, guard_start) == TraverseResult::Looped {
                loop_positions += 1;
            }
            map[test_row][test_col] = Space::Empty;
        }
    }

    loop_positions
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day6.txt");

    #[test]
    fn part_1() {
        assert_eq!(41, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(6, solve_part_2(SAMPLE_INPUT));
    }
}
