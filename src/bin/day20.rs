//! Day 20: Race Condition
//!
//! <https://adventofcode.com/2024/day/20>

use advent_of_code_2024::{Grid, Pos2};
use std::collections::VecDeque;
use std::error::Error;

type Position = Pos2<i32>;

#[derive(Debug)]
struct Input {
    walls: Vec<Vec<bool>>,
    start: Position,
    end: Position,
}

fn parse_input(input: &str) -> Input {
    let mut start: Option<Position> = None;
    let mut end: Option<Position> = None;
    let mut walls = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut walls_row = Vec::new();
        for c in line.chars() {
            match c {
                '#' => walls_row.push(true),
                '.' => walls_row.push(false),
                'S' => {
                    start = Some(Position { x: walls_row.len() as i32, y: walls.len() as i32 });
                    walls_row.push(false);
                }
                'E' => {
                    end = Some(Position { x: walls_row.len() as i32, y: walls.len() as i32 });
                    walls_row.push(false);
                }
                _ => panic!("Invalid input character: {c}"),
            }
        }
        walls.push(walls_row);
    }

    Input {
        walls,
        start: start.expect("No start position in input"),
        end: end.expect("No end position in input"),
    }
}

fn solve(input: &str, min_save: u32, max_cheat_time: u32) -> u32 {
    let Input { walls, start, end } = parse_input(input);
    let walls = Grid(walls);

    let distances_from_end = build_distances_from_end(&walls, end);
    let max_path_len = distances_from_end[start] - min_save;

    count_possible_cheats(&walls, start, &distances_from_end, max_cheat_time, max_path_len)
}

#[rustfmt::skip]
const DELTAS: [Position; 4] =
    [
        Position { x: -1, y: 0 },
        Position { x: 0, y: -1 },
        Position { x: 1, y: 0 },
        Position { x: 0, y: 1, },
    ];

#[derive(Debug, Clone, Copy)]
struct QueueEntry {
    pos: Position,
    distance: u32,
}

// BFS from the end position to build a grid of the min distance from each position to the end
fn build_distances_from_end(walls: &Grid<bool>, end: Position) -> Grid<u32> {
    let mut distances = Grid::same_size_as(walls);
    let mut visited = Grid::same_size_as(walls);

    let mut queue = VecDeque::new();
    queue.push_back(QueueEntry { pos: end, distance: 0 });
    visited[end] = true;

    while let Some(QueueEntry { pos, distance }) = queue.pop_front() {
        distances[pos] = distance;

        for delta in DELTAS {
            let new_pos = pos + delta;
            if walls[new_pos] || visited[new_pos] {
                continue;
            }

            queue.push_back(QueueEntry { pos: new_pos, distance: distance + 1 });
            visited[new_pos] = true;
        }
    }

    distances
}

// BFS from the start position, and at each position, check if it's possible to use a cheat starting
// at that position to reach the end in less than `min_path_len`
fn count_possible_cheats(
    walls: &Grid<bool>,
    start: Position,
    distances_from_end: &Grid<u32>,
    max_cheat_time: u32,
    max_path_len: u32,
) -> u32 {
    let mut visited = Grid::same_size_as(walls);

    let mut queue = VecDeque::new();
    queue.push_back(QueueEntry { pos: start, distance: 0 });
    visited[start] = true;

    let mut count = 0;
    while let Some(QueueEntry { pos, distance }) = queue.pop_front() {
        if distance > max_path_len - 2 {
            // Every useful cheat must take at least 2 steps: one to step on a wall and one to step
            // onto an open space
            break;
        }

        for cheat_distance in 2..=max_cheat_time {
            if distance + cheat_distance > max_path_len {
                break;
            }

            // Traverse the diamond formed by all spaces `cheat_distance` away from `pos`
            let mut cdx = -(cheat_distance as i32);
            let mut cdy = 0;
            let mut cdx_delta = 1;
            let mut cdy_delta = -1;
            loop {
                let cheat_pos = pos + Position { x: cdx, y: cdy };
                if (0..walls.cols() as i32).contains(&cheat_pos.x)
                    && (0..walls.rows() as i32).contains(&cheat_pos.y)
                    && !walls[cheat_pos]
                    && distance + cheat_distance + distances_from_end[cheat_pos] <= max_path_len
                {
                    count += 1;
                }

                cdx += cdx_delta;
                cdy += cdy_delta;
                if cdx == 0 || cdy == 0 {
                    // Rotate right
                    let t = -cdy_delta;
                    cdy_delta = cdx_delta;
                    cdx_delta = t;
                }

                if cdx == -(cheat_distance as i32) {
                    break;
                }
            }
        }

        for delta in DELTAS {
            let new_pos = pos + delta;
            if walls[new_pos] || visited[new_pos] {
                continue;
            }

            queue.push_back(QueueEntry { pos: new_pos, distance: distance + 1 });
            visited[new_pos] = true;
        }
    }

    count
}

const P1_CHEAT_DISTANCE: u32 = 2;
const P2_CHEAT_DISTANCE: u32 = 20;

fn solve_part_1(input: &str, min_save: u32) -> u32 {
    solve(input, min_save, P1_CHEAT_DISTANCE)
}

fn solve_part_2(input: &str, min_save: u32) -> u32 {
    solve(input, min_save, P2_CHEAT_DISTANCE)
}

const REAL_MIN_SAVE: u32 = 100;

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(
        |input| solve_part_1(input, REAL_MIN_SAVE),
        |input| solve_part_2(input, REAL_MIN_SAVE),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day20.txt");

    #[test]
    fn part_1() {
        assert_eq!(1, solve_part_1(SAMPLE_INPUT, 64));
        assert_eq!(2, solve_part_1(SAMPLE_INPUT, 40));
        assert_eq!(3, solve_part_1(SAMPLE_INPUT, 38));
        assert_eq!(4, solve_part_1(SAMPLE_INPUT, 36));
        assert_eq!(5, solve_part_1(SAMPLE_INPUT, 20));
        assert_eq!(8, solve_part_1(SAMPLE_INPUT, 12));
        assert_eq!(10, solve_part_1(SAMPLE_INPUT, 10));
    }

    #[test]
    fn part_2() {
        assert_eq!(3, solve_part_2(SAMPLE_INPUT, 76));
        assert_eq!(7, solve_part_2(SAMPLE_INPUT, 74));
        assert_eq!(29, solve_part_2(SAMPLE_INPUT, 72));
        assert_eq!(41, solve_part_2(SAMPLE_INPUT, 70));
        assert_eq!(55, solve_part_2(SAMPLE_INPUT, 68));
    }
}
