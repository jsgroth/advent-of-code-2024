//! Day 16: Reindeer Maze
//!
//! <https://adventofcode.com/2024/day/16>

use advent_of_code_2024::Pos2;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::ops::Index;

type Position = Pos2<i32>;

#[derive(Debug, Clone)]
struct Walls(Vec<Vec<bool>>);

impl Index<Position> for Walls {
    type Output = bool;

    fn index(&self, index: Position) -> &Self::Output {
        &self.0[index.y as usize][index.x as usize]
    }
}

#[derive(Debug)]
struct Input {
    walls: Vec<Vec<bool>>,
    start: Position,
    end: Position,
}

fn parse_input(input: &str) -> Input {
    let mut walls = Vec::new();
    let mut start: Option<Position> = None;
    let mut end: Option<Position> = None;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut walls_row = Vec::new();
        for c in line.chars() {
            match c {
                '.' => walls_row.push(false),
                '#' => walls_row.push(true),
                'S' => {
                    start = Some(Position { x: walls_row.len() as i32, y: walls.len() as i32 });
                    walls_row.push(false);
                }
                'E' => {
                    end = Some(Position { x: walls_row.len() as i32, y: walls.len() as i32 });
                    walls_row.push(false);
                }
                _ => panic!("Invalid input character: '{c}"),
            }
        }

        walls.push(walls_row);
    }

    let start = start.expect("No start position in map");
    let end = end.expect("No end position in map");
    Input { walls, start, end }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn delta(self) -> Position {
        match self {
            Self::North => Position { x: 0, y: -1 },
            Self::South => Position { x: 0, y: 1 },
            Self::East => Position { x: 1, y: 0 },
            Self::West => Position { x: -1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeapEntry {
    score: u32,
    pos: Position,
    direction: Direction,
    path: Vec<Position>,
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse cmp for min heap
        other.score.cmp(&self.score)
    }
}

fn solve<const PART2: bool>(input: &str) -> u32 {
    let Input { walls, start, end } = parse_input(input);
    let walls = Walls(walls);

    let mut min_scores: FxHashMap<(Position, Direction), u32> = FxHashMap::default();

    let mut heap = BinaryHeap::new();
    heap.push(HeapEntry { score: 0, pos: start, direction: Direction::East, path: vec![] });

    let mut good_seats: FxHashSet<Position> = [start, end].into_iter().collect();

    let mut min_score_to_end: Option<u32> = None;

    while let Some(HeapEntry { score, pos, direction, mut path }) = heap.pop() {
        if pos == end {
            match min_score_to_end {
                Some(min_score) if min_score == score => {
                    // This is a min-distance path to the end; all positions on this path are good places to sit
                    good_seats.extend(path.into_iter());
                }
                None => {
                    // First path to reach the end is guaranteed to have the min possible score
                    min_score_to_end = Some(score);
                    good_seats.extend(path.into_iter());
                }
                Some(_) => {
                    // This is not a min-distance path to the end position; do nothing
                }
            }
            continue;
        }

        if min_scores.get(&(pos, direction)).is_some_and(|&min_score| min_score < score) {
            continue;
        }
        min_scores.insert((pos, direction), score);

        path.push(pos);

        let forward_pos = pos + direction.delta();
        let forward_score = score + 1;
        if !walls[forward_pos]
            && min_scores
                .get(&(forward_pos, direction))
                .is_none_or(|&min_score| min_score >= forward_score)
        {
            heap.push(HeapEntry {
                score: forward_score,
                pos: forward_pos,
                direction,
                path: path.clone(),
            });
        }

        let rotate_score = score + 1000;
        for rotate_direction in [direction.rotate_left(), direction.rotate_right()] {
            if min_scores
                .get(&(pos, rotate_direction))
                .is_none_or(|&min_score| min_score >= rotate_score)
            {
                heap.push(HeapEntry {
                    score: rotate_score,
                    pos,
                    direction: rotate_direction,
                    path: path.clone(),
                });
            }
        }
    }

    if PART2 { good_seats.len() as u32 } else { min_score_to_end.expect("No solution found") }
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve::<false>, solve::<true>)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day16.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample/day16-2.txt");

    #[test]
    fn part_1() {
        assert_eq!(7036, solve::<false>(SAMPLE_INPUT));
        assert_eq!(11048, solve::<false>(SAMPLE_INPUT_2));
    }

    #[test]
    fn part_2() {
        assert_eq!(45, solve::<true>(SAMPLE_INPUT));
        assert_eq!(64, solve::<true>(SAMPLE_INPUT_2));
    }
}
