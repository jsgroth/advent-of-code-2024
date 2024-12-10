//! Day 10: Hoof It
//!
//! <https://adventofcode.com/2024/day/10>

use advent_of_code_2024::Pos2;
use rustc_hash::FxHashSet;
use std::error::Error;
use std::iter;

type Position = Pos2<i32>;

trait Accumulator {
    fn new() -> Self;

    fn new_for_pos(pos: Position) -> Self;

    fn accumulate(&mut self, other: &Self);

    fn score(&self) -> usize;
}

// Part 1: Accumulate unique number of 9s reachable
impl Accumulator for FxHashSet<Position> {
    fn new() -> Self {
        FxHashSet::default()
    }

    fn new_for_pos(pos: Position) -> Self {
        iter::once(pos).collect()
    }

    fn accumulate(&mut self, other: &Self) {
        self.extend(other.iter().copied());
    }

    fn score(&self) -> usize {
        self.len()
    }
}

// Part 2: Accumulate the total number of ways to reach a 9
impl Accumulator for usize {
    fn new() -> Self {
        0
    }

    fn new_for_pos(_pos: Position) -> Self {
        1
    }

    fn accumulate(&mut self, other: &Self) {
        *self += *other;
    }

    fn score(&self) -> usize {
        *self
    }
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

fn solve<Acc: Clone + Accumulator>(input: &str) -> usize {
    let map = parse_input(input);
    let mut cache: Vec<Vec<Option<Acc>>> = vec![vec![None; map[0].len()]; map.len()];

    let mut total = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let pos = Position { y: y as i32, x: x as i32 };

            if map[y][x] == 0 {
                search(&map, &mut cache, pos);
                total += cache[y][x].as_ref().unwrap().score();
            }
        }
    }

    total
}

fn search<Acc: Accumulator>(map: &[Vec<i32>], cache: &mut [Vec<Option<Acc>>], pos: Position) {
    if cache[pos.y as usize][pos.x as usize].is_some() {
        return;
    }

    if map[pos.y as usize][pos.x as usize] == 9 {
        cache[pos.y as usize][pos.x as usize] = Some(Acc::new_for_pos(pos));
        return;
    }

    let n = map[pos.y as usize][pos.x as usize];
    let mut acc = Acc::new();
    for (dy, dx) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let new_pos = pos + Position { y: dy, x: dx };
        if !(0..map.len() as i32).contains(&new_pos.y)
            || !(0..map[0].len() as i32).contains(&new_pos.x)
        {
            continue;
        }

        if map[new_pos.y as usize][new_pos.x as usize] != n + 1 {
            continue;
        }

        search(map, cache, new_pos);
        acc.accumulate(cache[new_pos.y as usize][new_pos.x as usize].as_ref().unwrap());
    }

    cache[pos.y as usize][pos.x as usize] = Some(acc);
}

fn solve_part_1(input: &str) -> usize {
    solve::<FxHashSet<Position>>(input)
}

fn solve_part_2(input: &str) -> usize {
    solve::<usize>(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day10.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample/day10-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("../../sample/day10-3.txt");

    #[test]
    fn part_1() {
        assert_eq!(1, solve_part_1(SAMPLE_INPUT));
        assert_eq!(36, solve_part_1(SAMPLE_INPUT_2));
    }

    #[test]
    fn part_2() {
        assert_eq!(227, solve_part_2(SAMPLE_INPUT_3));
        assert_eq!(81, solve_part_2(SAMPLE_INPUT_2));
    }
}
