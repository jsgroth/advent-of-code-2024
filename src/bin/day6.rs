//! Day 6: Guard Gallivant
//!
//! <https://adventofcode.com/2024/day/6>
//!
//! -------
//! Part 1
//! -------
//! Straightforward 2D grid walking
//!
//! -------
//! Part 2
//! -------
//! Walk the grid as in part 1, but at each step, check if it's possible to place an obstacle at
//! the next position that the guard would step onto. This is possible if the following are true:
//!   - There is not already an obstacle in that position
//!   - No obstacle has been placed yet
//!   - The guard has not already stepped on the potential obstacle position
//!
//! If an obstacle can be placed then snapshot the current visited state, place the obstacle, and
//! recursively check if the guard will enter a loop when starting from the current state. When the
//! recursive call returns then restore visited state, remove the obstacle, and continue on normally.
//!
//! Loops are detected based on (row, column, direction) triples. If the guard ever steps on a
//! position twice while facing the same direction, there is a loop.

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
            map_row.push(if c == '#' { Space::Obstacle } else { Space::Empty });
            if c == '^' {
                guard_start = Some((row as i32, col as i32));
            }
        }
        map.push(map_row);
    }

    Input { map, guard_start: guard_start.expect("No guard position in input") }
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

    visited.into_iter().map(|row| row.into_iter().filter(|&b| b).count()).sum()
}

fn solve_part_2(input: &str) -> u32 {
    let Input { mut map, guard_start } = parse_input(input);

    let mut visited = vec![vec![0; map[0].len()]; map.len()];
    traverse_part_2(
        &mut map,
        &mut visited,
        guard_start,
        Direction::Up,
        false,
        &mut VisitsBuffer::new(),
    )
}

struct VisitsBuffer {
    visits: Vec<(i32, i32, Direction)>,
    indices: Vec<usize>,
}

impl VisitsBuffer {
    fn new() -> Self {
        Self { visits: Vec::new(), indices: Vec::new() }
    }

    fn checkpoint(&mut self) {
        self.indices.push(self.visits.len());
    }

    fn unwind(&mut self, visited: &mut [Vec<u8>]) {
        let i = self.indices.pop().unwrap();
        for &(row, col, direction) in &self.visits[i..] {
            visited[row as usize][col as usize] &= !(direction as u8);
        }
        self.visits.truncate(i);
    }

    fn push(&mut self, row: i32, col: i32, direction: Direction) {
        self.visits.push((row, col, direction));
    }
}

fn traverse_part_2(
    map: &mut [Vec<Space>],
    visited: &mut Vec<Vec<u8>>,
    (mut row, mut col): (i32, i32),
    mut direction: Direction,
    obstacle_placed: bool,
    visits: &mut VisitsBuffer,
) -> u32 {
    visits.checkpoint();

    let mut loops = 0;
    loop {
        if visited[row as usize][col as usize] & (direction as u8) != 0 {
            loops += 1;
            break;
        }
        visited[row as usize][col as usize] |= direction as u8;
        visits.push(row, col, direction);

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
            if !obstacle_placed && visited[next_row as usize][next_col as usize] == 0 {
                // No obstacle has been inserted yet, and the space ahead is:
                //   * Empty
                //   * Has not been visited yet
                // Insert the obstacle, recurse, then remove the obstacle
                map[next_row as usize][next_col as usize] = Space::Obstacle;
                loops += traverse_part_2(
                    map,
                    visited,
                    (row, col),
                    direction.rotate_right(),
                    true,
                    visits,
                );
                map[next_row as usize][next_col as usize] = Space::Empty;
            }
            (row, col) = (next_row, next_col);
        }
    }

    visits.unwind(visited);

    loops
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
