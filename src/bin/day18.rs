//! Day 18: RAM Run
//!
//! <https://adventofcode.com/2024/day/18>

use advent_of_code_2024::Pos2;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Display, Formatter};

type Position = Pos2<i32>;

fn parse_input(input: &str) -> Vec<Pos2<usize>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            Pos2 { x: l.parse().unwrap(), y: r.parse().unwrap() }
        })
        .collect()
}

#[derive(Debug)]
struct QueueEntry {
    pos: Position,
    len: u32,
}

fn solve_part_1(input: &str, bytes: usize, size: usize) -> u32 {
    let bytes_list = parse_input(input);
    let mut bytes_map = vec![vec![false; size]; size];

    for &byte_pos in &bytes_list[..bytes] {
        bytes_map[byte_pos.y][byte_pos.x] = true;
    }

    bfs_path_search(&bytes_map, size as i32).expect("No solution found")
}

fn bfs_path_search(bytes_map: &[Vec<bool>], size: i32) -> Option<u32> {
    let end_pos = Position { x: size - 1, y: size - 1 };

    let mut visited = vec![vec![false; size as usize]; size as usize];
    let mut queue = VecDeque::new();
    queue.push_back(QueueEntry { pos: Position { x: 0, y: 0 }, len: 0 });
    visited[0][0] = true;

    while let Some(QueueEntry { pos, len }) = queue.pop_front() {
        for (dy, dx) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let new_pos = pos + Position { x: dx, y: dy };
            if !(0..size).contains(&new_pos.y) || !(0..size).contains(&new_pos.x) {
                continue;
            }

            if !bytes_map[new_pos.y as usize][new_pos.x as usize]
                && !visited[new_pos.y as usize][new_pos.x as usize]
            {
                if new_pos == end_pos {
                    return Some(len + 1);
                }

                visited[new_pos.y as usize][new_pos.x as usize] = true;
                queue.push_back(QueueEntry { pos: new_pos, len: len + 1 });
            }
        }
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part2Solution(usize, usize);

impl Display for Part2Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

fn solve_part_2(input: &str, start_bytes: usize, size: usize) -> Part2Solution {
    let bytes_list = parse_input(input);
    let mut bytes_map = vec![vec![false; size]; size];

    for &byte_pos in &bytes_list[..start_bytes] {
        bytes_map[byte_pos.y][byte_pos.x] = true;
    }

    let mut b = start_bytes;
    let mut e = bytes_list.len();
    while b < e {
        let m = (b + e) / 2;

        let mut bytes_map = bytes_map.clone();
        for &byte_pos in &bytes_list[start_bytes..=m] {
            bytes_map[byte_pos.y][byte_pos.x] = true;
        }

        let can_reach_end = bfs_path_search(&bytes_map, size as i32).is_some();
        if can_reach_end {
            b = m + 1;
        } else {
            e = m;
        }
    }

    assert_eq!(b, e);
    let byte_pos = bytes_list[b];
    Part2Solution(byte_pos.x, byte_pos.y)
}

const REAL_START_BYTES: usize = 1024;
const REAL_SIZE: usize = 71;

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(
        |input| solve_part_1(input, REAL_START_BYTES, REAL_SIZE),
        |input| solve_part_2(input, REAL_START_BYTES, REAL_SIZE),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day18.txt");
    const SAMPLE_START_BYTES: usize = 12;
    const SAMPLE_SIZE: usize = 7;

    #[test]
    fn part_1() {
        assert_eq!(22, solve_part_1(SAMPLE_INPUT, SAMPLE_START_BYTES, SAMPLE_SIZE));
    }

    #[test]
    fn part_2() {
        assert_eq!(
            Part2Solution(6, 1),
            solve_part_2(SAMPLE_INPUT, SAMPLE_START_BYTES, SAMPLE_SIZE)
        );
    }
}
