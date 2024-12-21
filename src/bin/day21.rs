//! Day 21: Keypad Conundrum
//!
//! <https://adventofcode.com/2024/day/21>

use advent_of_code_2024::Pos2;
use rustc_hash::FxHashMap;
use std::cmp;
use std::cmp::Ordering;
use std::error::Error;

type Position = Pos2<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumericKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

// Numeric keypad layout:
//
//   | 0 1 2
//  --------
// 0 | 7 8 9
// 1 | 4 5 6
// 2 | 1 2 3
// 3 |   0 A
//
// The bottom left corner (0, 3) is a gap that must not be touched
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

    fn from_char(c: char) -> Self {
        match c {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'A' => Self::Activate,
            _ => panic!("Invalid input character: {c}"),
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

// Directional keypad layout:
//
//   | 0 1 2
//  --------
// 0 |   ^ A
// 1 | < v >
//
// The top left corner (0, 0) is a gap that must not be touched
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

    fn x_direction(delta: Position) -> Option<Self> {
        match delta.x.cmp(&0) {
            Ordering::Less => Some(Self::Left),
            Ordering::Greater => Some(Self::Right),
            Ordering::Equal => None,
        }
    }

    fn y_direction(delta: Position) -> Option<Self> {
        match delta.y.cmp(&0) {
            Ordering::Less => Some(Self::Up),
            Ordering::Greater => Some(Self::Down),
            Ordering::Equal => None,
        }
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
            let keys = line.chars().map(NumericKey::from_char).collect();
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

    find_min_distance_key(
        start.position(),
        code[0].position(),
        middle_robots + 1,
        NumericKey::GAP,
        cache,
    ) + find_min_distance(&code[1..], code[0], middle_robots, cache)
}

// Find the min distance of the path from `start` to `target` at the specified depth
fn find_min_distance_key(
    start: Position,
    target: Position,
    depth_remaining: u32,
    gap: Position,
    cache: &mut FxHashMap<CacheKey, u64>,
) -> u64 {
    if depth_remaining == 0 {
        // At the bottom level, any key takes 1 step
        return 1;
    }

    let target_delta = target - start;
    if target_delta == Position::xy(0, 0) {
        // The next level only needs to press A, which it is already on
        return 1;
    }

    let cache_key = CacheKey { start, target, depth_remaining, gap };
    if let Some(&min_distance) = cache.get(&cache_key) {
        return min_distance;
    }

    let mut min_distance = u64::MAX;

    // Check if the path will cross the gap if moving horizontally then vertically
    if start.y != gap.y || target.x != gap.x {
        // At depth (depth-1), move horizontally then vertically to reach the arrow, then move back to A
        let distance =
            move_to_key_and_back(target_delta, depth_remaining - 1, MoveDirections::HThenV, cache);
        min_distance = cmp::min(min_distance, distance);
    }

    // Check if the path will cross the gap if moving vertically then horizontally
    if start.x != gap.x || target.y != gap.y {
        // At depth (depth-1), move vertically then horizontally to reach the arrow, then move back to A
        let distance =
            move_to_key_and_back(target_delta, depth_remaining - 1, MoveDirections::VThenH, cache);
        min_distance = cmp::min(min_distance, distance);
    }

    cache.insert(cache_key, min_distance);
    min_distance
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveDirections {
    HThenV,
    VThenH,
}

// Find the min distance of the path to move the distance specified by `target_delta` at the specified
// depth, and then back to the A
fn move_to_key_and_back(
    target_delta: Position,
    depth: u32,
    directions: MoveDirections,
    cache: &mut FxHashMap<CacheKey, u64>,
) -> u64 {
    let mut distance = 0;
    let mut pos = DirectionalKey::Activate.position();

    // Move the specified distance (to the arrow key)
    let move_directions = match directions {
        MoveDirections::HThenV => [MoveDirection::Horizontal, MoveDirection::Vertical],
        MoveDirections::VThenH => [MoveDirection::Vertical, MoveDirection::Horizontal],
    };
    for direction in move_directions {
        move_direction(target_delta, direction, depth, &mut distance, &mut pos, cache);
    }

    // Move back to A
    distance += find_min_distance_key(
        pos,
        DirectionalKey::Activate.position(),
        depth,
        DirectionalKey::GAP,
        cache,
    );
    distance
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveDirection {
    Horizontal,
    Vertical,
}

// Find the min distance of the path required to move by the specified distance in a single direction
// (either horizontal or vertical)
fn move_direction(
    target_delta: Position,
    direction: MoveDirection,
    depth: u32,
    distance: &mut u64,
    pos: &mut Position,
    cache: &mut FxHashMap<CacheKey, u64>,
) {
    let (delta_component, direction_key) = match direction {
        MoveDirection::Horizontal => (target_delta.x, DirectionalKey::x_direction(target_delta)),
        MoveDirection::Vertical => (target_delta.y, DirectionalKey::y_direction(target_delta)),
    };
    let Some(direction_key) = direction_key else { return };
    let new_pos = direction_key.position();

    *distance += find_min_distance_key(*pos, new_pos, depth, DirectionalKey::GAP, cache);

    // Account for next level pressing A multiple times if abs(distance) > 1
    *distance += (delta_component.abs() - 1) as u64;

    *pos = new_pos;
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

    #[test]
    fn part_2() {
        assert_eq!(154115708116294, solve(SAMPLE_INPUT, P2_ROBOTS));
    }
}
