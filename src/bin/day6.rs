//! Day 6: Guard Gallivant
//!
//! <https://adventofcode.com/2024/day/6>

use rustc_hash::FxHashSet;
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
    traverse_map(&map, guard_start)
}

fn traverse_map(map: &[Vec<Space>], start: (i32, i32)) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    let (mut row, mut col) = start;
    let mut direction = Direction::Up;
    loop {
        visited[row as usize][col as usize] = true;

        let (next_row, next_col) = direction.step(row, col);
        if !(0..map.len() as i32).contains(&next_row)
            || !(0..map[0].len() as i32).contains(&next_col)
        {
            break;
        }

        if map[next_row as usize][next_col as usize] == Space::Obstacle {
            direction = direction.rotate_right();
        } else {
            (row, col) = (next_row, next_col);
        }
    }

    visited
        .into_iter()
        .map(|row| row.into_iter().filter(|&b| b).count())
        .sum()
}

fn solve_part_2(input: &str) -> usize {
    let Input {
        mut map,
        guard_start,
    } = parse_input(input);

    let mut visited = vec![vec![0; map[0].len()]; map.len()];
    let mut result = FxHashSet::default();
    traverse_part_2(
        &mut map,
        &mut visited,
        guard_start,
        Direction::Up,
        None,
        &mut result,
    );

    result.len()
}

fn traverse_part_2(
    map: &mut [Vec<Space>],
    visited: &mut Vec<Vec<u8>>,
    (mut row, mut col): (i32, i32),
    mut direction: Direction,
    obstacle_location: Option<(i32, i32)>,
    result: &mut FxHashSet<(i32, i32)>,
) {
    let mut visits = Vec::new();

    loop {
        if visited[row as usize][col as usize] & (direction as u8) != 0 {
            result.insert(obstacle_location.unwrap());
            break;
        }
        visited[row as usize][col as usize] |= direction as u8;
        visits.push((row, col, direction));

        let (next_row, next_col) = direction.step(row, col);
        if !(0..map.len() as i32).contains(&next_row)
            || !(0..map[0].len() as i32).contains(&next_col)
        {
            // Went out of bounds
            break;
        }

        if map[next_row as usize][next_col as usize] == Space::Obstacle {
            // Ran into an obstacle; rotate
            direction = direction.rotate_right();
        } else {
            if obstacle_location.is_none()
                && visited[next_row as usize][next_col as usize] == 0
                && !result.contains(&(next_row, next_col))
            {
                // No obstacle has been inserted yet, and the space ahead is:
                //   * Empty
                //   * Has not been visited yet
                //   * Is not already part of the result (i.e. an obstacle was placed there and caused a loop)
                // Insert the obstacle, recurse, then remove the obstacle
                map[next_row as usize][next_col as usize] = Space::Obstacle;
                traverse_part_2(
                    map,
                    visited,
                    (row, col),
                    direction.rotate_right(),
                    Some((next_row, next_col)),
                    result,
                );
                map[next_row as usize][next_col as usize] = Space::Empty;
            }
            (row, col) = (next_row, next_col);
        }
    }

    unwind_visits(visited, &visits);
}

fn unwind_visits(visited: &mut [Vec<u8>], visits: &[(i32, i32, Direction)]) {
    for &(row, col, direction) in visits {
        visited[row as usize][col as usize] &= !(direction as u8);
    }
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
