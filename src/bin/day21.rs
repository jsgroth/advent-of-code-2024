//! Day 21: Keypad Conundrum
//!
//! <https://adventofcode.com/2024/day/21>

use advent_of_code_2024::Pos2;
use rustc_hash::FxHashMap;
use std::cmp;
use std::error::Error;

type Position = Pos2<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumericKey {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Activate = 10,
}

impl NumericKey {
    const GAP: Position = Position::xy(0, 3);

    const fn position(self) -> Position {
        match self {
            Self::Zero => Position::xy(1, 3),
            Self::One => Position::xy(0, 2),
            Self::Two => Position::xy(1, 2),
            Self::Three => Position::xy(2, 2),
            Self::Four => Position::xy(0, 1),
            Self::Five => Position::xy(1, 1),
            Self::Six => Position::xy(2, 1),
            Self::Seven => Position::xy(0, 0),
            Self::Eight => Position::xy(1, 0),
            Self::Nine => Position::xy(2, 0),
            Self::Activate => Position::xy(2, 3),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl DirectionalKey {
    const GAP: Position = Position::xy(0, 0);

    const fn position(self) -> Position {
        match self {
            Self::Up => Position::xy(1, 0),
            Self::Down => Position::xy(1, 1),
            Self::Left => Position::xy(0, 1),
            Self::Right => Position::xy(2, 1),
            Self::Activate => Position::xy(2, 0),
        }
    }

    const fn x_direction(delta: Position) -> Self {
        if delta.x < 0 { Self::Left } else { Self::Right }
    }

    const fn y_direction(delta: Position) -> Self {
        if delta.y < 0 { Self::Up } else { Self::Down }
    }
}

#[derive(Debug, Clone)]
struct Code {
    keys: Vec<NumericKey>,
    value: u64,
}

fn parse_input(input: &str) -> Vec<Code> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let keys = line
                .chars()
                .map(|c| match c {
                    '0' => NumericKey::Zero,
                    '1' => NumericKey::One,
                    '2' => NumericKey::Two,
                    '3' => NumericKey::Three,
                    '4' => NumericKey::Four,
                    '5' => NumericKey::Five,
                    '6' => NumericKey::Six,
                    '7' => NumericKey::Seven,
                    '8' => NumericKey::Eight,
                    '9' => NumericKey::Nine,
                    'A' => NumericKey::Activate,
                    _ => panic!("Invalid input character: {c}"),
                })
                .collect();

            let value = line[..3].parse::<u64>().unwrap();

            Code { keys, value }
        })
        .collect()
}

fn solve(input: &str, middle_robots: u32) -> u64 {
    let codes = parse_input(input);

    let mut cache = FxHashMap::default();
    codes
        .into_iter()
        .map(|code| {
            let min_distance =
                find_min_distance(&code.keys, NumericKey::Activate, middle_robots, &mut cache);
            min_distance * code.value
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey {
    start: Position,
    target: Position,
    depth_remaining: u32,
    gap: Position,
}

fn find_min_distance(
    code: &[NumericKey],
    start: NumericKey,
    middle_robots: u32,
    cache: &mut FxHashMap<CacheKey, u64>,
) -> u64 {
    if code.is_empty() {
        return 0;
    }

    find_min_distance_rec(
        CacheKey {
            start: start.position(),
            target: code[0].position(),
            depth_remaining: middle_robots + 1,
            gap: NumericKey::GAP,
        },
        cache,
    ) + find_min_distance(&code[1..], code[0], middle_robots, cache)
}

fn find_min_distance_rec(key: CacheKey, cache: &mut FxHashMap<CacheKey, u64>) -> u64 {
    if let Some(&min_distance) = cache.get(&key) {
        return min_distance;
    }

    let CacheKey { start, target, depth_remaining, gap } = key;

    let target_delta = target - start;
    if depth_remaining == 0 || target_delta == Position::xy(0, 0) {
        // Next level only needs to press A, which it is already on
        return 1;
    }

    let mut min_distance = u64::MAX;

    if start.y != gap.y || target.x != gap.x {
        // Move horizontally then vertically, then back to A
        let mut sub_distance = 0;
        let mut sub_pos = DirectionalKey::Activate.position();

        move_horizontally(target_delta, depth_remaining, &mut sub_distance, &mut sub_pos, cache);
        move_vertically(target_delta, depth_remaining, &mut sub_distance, &mut sub_pos, cache);

        sub_distance += find_min_distance_rec(
            CacheKey {
                start: sub_pos,
                target: DirectionalKey::Activate.position(),
                depth_remaining: depth_remaining - 1,
                gap: DirectionalKey::GAP,
            },
            cache,
        );
        min_distance = cmp::min(min_distance, sub_distance);
    }

    if start.x != gap.x || target.y != gap.y {
        // Move vertically then horizontally, then back to A
        let mut sub_distance = 0;
        let mut sub_pos = DirectionalKey::Activate.position();

        move_vertically(target_delta, depth_remaining, &mut sub_distance, &mut sub_pos, cache);
        move_horizontally(target_delta, depth_remaining, &mut sub_distance, &mut sub_pos, cache);

        sub_distance += find_min_distance_rec(
            CacheKey {
                start: sub_pos,
                target: DirectionalKey::Activate.position(),
                depth_remaining: depth_remaining - 1,
                gap: DirectionalKey::GAP,
            },
            cache,
        );
        min_distance = cmp::min(min_distance, sub_distance);
    }

    cache.insert(key, min_distance);
    min_distance
}

fn move_horizontally(
    target_delta: Position,
    depth_remaining: u32,
    sub_distance: &mut u64,
    sub_pos: &mut Position,
    cache: &mut FxHashMap<CacheKey, u64>,
) {
    if target_delta.x == 0 {
        return;
    }

    let new_sub_pos = DirectionalKey::x_direction(target_delta).position();
    *sub_distance += find_min_distance_rec(
        CacheKey {
            start: *sub_pos,
            target: new_sub_pos,
            depth_remaining: depth_remaining - 1,
            gap: DirectionalKey::GAP,
        },
        cache,
    );
    *sub_distance += (target_delta.x.abs() - 1) as u64;
    *sub_pos = new_sub_pos;
}

fn move_vertically(
    target_delta: Position,
    depth_remaining: u32,
    sub_distance: &mut u64,
    sub_pos: &mut Position,
    cache: &mut FxHashMap<CacheKey, u64>,
) {
    if target_delta.y == 0 {
        return;
    }

    let new_sub_pos = DirectionalKey::y_direction(target_delta).position();
    *sub_distance += find_min_distance_rec(
        CacheKey {
            start: *sub_pos,
            target: new_sub_pos,
            depth_remaining: depth_remaining - 1,
            gap: DirectionalKey::GAP,
        },
        cache,
    );
    *sub_distance += (target_delta.y.abs() - 1) as u64;
    *sub_pos = new_sub_pos;
}

const P1_ROBOTS: u32 = 2;
const P2_ROBOTS: u32 = 25;

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(|input| solve(input, P1_ROBOTS), |input| solve(input, P2_ROBOTS))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day21.txt");

    #[test]
    fn part_1() {
        assert_eq!(126384, solve(SAMPLE_INPUT, P1_ROBOTS));
    }
}
