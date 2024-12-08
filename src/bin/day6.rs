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

use advent_of_code_2024::Pos2;
use std::error::Error;

type Position = Pos2<i32>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up = 1 << 0,
    Left = 1 << 1,
    Right = 1 << 2,
    Down = 1 << 3,
}

impl Direction {
    const fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    const fn delta(self) -> Position {
        match self {
            Self::Up => Position { x: 0, y: -1 },
            Self::Left => Position { x: -1, y: 0 },
            Self::Right => Position { x: 1, y: 0 },
            Self::Down => Position { x: 0, y: 1 },
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
    guard_start: Position,
}

fn parse_input(input: &str) -> Input {
    let mut map = Vec::new();
    let mut guard_start: Option<Position> = None;
    for (row, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut map_row = Vec::new();
        for (col, c) in line.chars().enumerate() {
            map_row.push(if c == '#' { Space::Obstacle } else { Space::Empty });
            if c == '^' {
                guard_start = Some(Position { x: col as i32, y: row as i32 });
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

fn traverse_map(map: &[Vec<Space>], start: Position) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    let mut current_pos = start;
    let mut direction = Direction::Up;
    loop {
        visited[current_pos.y as usize][current_pos.x as usize] = true;

        let next_pos = current_pos + direction.delta();
        if !(0..map.len() as i32).contains(&next_pos.y)
            || !(0..map[0].len() as i32).contains(&next_pos.x)
        {
            break;
        }

        if map[next_pos.y as usize][next_pos.x as usize] == Space::Obstacle {
            direction = direction.rotate_right();
        } else {
            current_pos = next_pos;
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
    visits: Vec<(Position, Direction)>,
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
        for &(pos, direction) in &self.visits[i..] {
            visited[pos.y as usize][pos.x as usize] &= !(direction as u8);
        }
        self.visits.truncate(i);
    }

    fn push(&mut self, pos: Position, direction: Direction) {
        self.visits.push((pos, direction));
    }
}

fn traverse_part_2(
    map: &mut [Vec<Space>],
    visited: &mut Vec<Vec<u8>>,
    mut current_pos: Position,
    mut direction: Direction,
    obstacle_placed: bool,
    visits: &mut VisitsBuffer,
) -> u32 {
    visits.checkpoint();

    let mut loops = 0;
    loop {
        if visited[current_pos.y as usize][current_pos.x as usize] & (direction as u8) != 0 {
            loops += 1;
            break;
        }
        visited[current_pos.y as usize][current_pos.x as usize] |= direction as u8;
        visits.push(current_pos, direction);

        let next_pos = current_pos + direction.delta();
        if !(0..map.len() as i32).contains(&next_pos.y)
            || !(0..map[0].len() as i32).contains(&next_pos.x)
        {
            // Went out of bounds
            break;
        }

        if map[next_pos.y as usize][next_pos.x as usize] == Space::Obstacle {
            // Ran into an obstacle; rotate
            direction = direction.rotate_right();
        } else {
            if !obstacle_placed && visited[next_pos.y as usize][next_pos.x as usize] == 0 {
                // No obstacle has been inserted yet, and the space ahead is:
                //   * Empty
                //   * Has not been visited yet
                // Insert the obstacle, recurse, then remove the obstacle
                map[next_pos.y as usize][next_pos.x as usize] = Space::Obstacle;
                loops += traverse_part_2(
                    map,
                    visited,
                    current_pos,
                    direction.rotate_right(),
                    true,
                    visits,
                );
                map[next_pos.y as usize][next_pos.x as usize] = Space::Empty;
            }
            current_pos = next_pos;
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
