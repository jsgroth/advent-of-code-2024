//! Day 15: Warehouse Woes
//!
//! <https://adventofcode.com/2024/day/15>

use advent_of_code_2024::Pos2;
use std::error::Error;

type Position = Pos2<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Wall,
    Box,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    const fn delta(self) -> Position {
        match self {
            Self::Up => Position { x: 0, y: -1 },
            Self::Left => Position { x: -1, y: 0 },
            Self::Right => Position { x: 1, y: 0 },
            Self::Down => Position { x: 0, y: 1 },
        }
    }
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<Space>>,
    robot_start: Position,
    moves: Vec<Direction>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let (map, robot_start) = parse_map(&mut lines);
    let moves = parse_moves(&mut lines);

    Input { map, robot_start, moves }
}

fn parse_map<'a>(lines: &mut impl Iterator<Item = &'a str>) -> (Vec<Vec<Space>>, Position) {
    let mut map: Vec<Vec<Space>> = Vec::new();
    let mut robot_start: Option<Position> = None;
    for map_line in lines.by_ref().take_while(|line| !line.is_empty()) {
        let mut map_row = Vec::with_capacity(map_line.len());
        for c in map_line.chars() {
            match c {
                '.' => map_row.push(Space::Empty),
                '#' => map_row.push(Space::Wall),
                'O' => map_row.push(Space::Box),
                '@' => {
                    robot_start = Some(Position { x: map_row.len() as i32, y: map.len() as i32 });
                    map_row.push(Space::Empty);
                }
                _ => panic!("Unexpected map character: '{c}'"),
            }
        }
        map.push(map_row);
    }

    let robot_start = robot_start.expect("No robot location in map input");
    (map, robot_start)
}

fn parse_moves<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<Direction> {
    lines
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => Direction::Up,
                '<' => Direction::Left,
                '>' => Direction::Right,
                'v' => Direction::Down,
                _ => panic!("Unexpected direction character: '{c}'"),
            })
        })
        .collect()
}

fn solve_part_1(input: &str) -> usize {
    let Input { mut map, robot_start, moves } = parse_input(input);

    let mut robot_pos = robot_start;
    for &direction in &moves {
        let delta = direction.delta();
        let new_pos = robot_pos + delta;
        match map[new_pos.y as usize][new_pos.x as usize] {
            Space::Empty => {
                robot_pos = new_pos;
            }
            Space::Wall => {}
            Space::Box => {
                if try_push_boxes(&mut map, new_pos, delta, Space::Empty, |space| {
                    space == Space::Box
                }) {
                    robot_pos = new_pos;
                }
            }
        }
    }

    score_map(&map, Space::Box)
}

fn try_push_boxes<T: Copy + Eq>(
    map: &mut [Vec<T>],
    pos: Position,
    delta: Position,
    empty: T,
    is_box: impl Fn(T) -> bool,
) -> bool {
    // Advance until end_pos hits an empty space or a wall
    let mut end_pos = pos;
    while is_box(map[end_pos.y as usize][end_pos.x as usize]) {
        end_pos += delta;
    }

    if map[end_pos.y as usize][end_pos.x as usize] != empty {
        // Hit a wall; can't move
        return false;
    }

    // Shift all boxes over and fill the last space with empty
    while end_pos != pos {
        map[end_pos.y as usize][end_pos.x as usize] =
            map[(end_pos.y - delta.y) as usize][(end_pos.x - delta.x) as usize];
        end_pos -= delta;
    }
    map[pos.y as usize][pos.x as usize] = empty;

    true
}

fn score_map<T: Copy + Eq>(map: &[Vec<T>], target: T) -> usize {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &space)| if space == target { 100 * y + x } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoxSide {
    Left,
    Right,
}

impl BoxSide {
    fn other(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn other_x_adjust(self) -> i32 {
        match self {
            Self::Left => 1,
            Self::Right => -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space2 {
    Empty,
    Wall,
    Box(BoxSide),
}

impl Space2 {
    fn is_box(self) -> bool {
        matches!(self, Self::Box(_))
    }
}

fn solve_part_2(input: &str) -> usize {
    let Input { map, robot_start, moves } = parse_input(input);

    let mut map = expand_map(&map);
    let mut robot_pos = Position { x: 2 * robot_start.x, y: robot_start.y };

    for &direction in &moves {
        let delta = direction.delta();
        let new_pos = robot_pos + delta;

        let space = map[new_pos.y as usize][new_pos.x as usize];
        match space {
            Space2::Empty => {
                robot_pos = new_pos;
            }
            Space2::Wall => {}
            Space2::Box(_) => {
                match direction {
                    Direction::Left | Direction::Right => {
                        // Horizontal push; easy case, basically the same as part 1
                        if try_push_boxes(&mut map, new_pos, delta, Space2::Empty, Space2::is_box) {
                            robot_pos = new_pos;
                        }
                    }
                    Direction::Up | Direction::Down => {
                        // Vertical push; trickier case
                        if can_move(&map, new_pos, delta) {
                            do_move(&mut map, new_pos, delta, Space2::Empty);
                            robot_pos = new_pos;
                        }
                    }
                }
            }
        }
    }

    score_map(&map, Space2::Box(BoxSide::Left))
}

fn expand_map(map: &[Vec<Space>]) -> Vec<Vec<Space2>> {
    map.iter()
        .map(|row| {
            row.iter()
                .flat_map(|&space| match space {
                    Space::Empty => [Space2::Empty; 2],
                    Space::Wall => [Space2::Wall; 2],
                    Space::Box => [Space2::Box(BoxSide::Left), Space2::Box(BoxSide::Right)],
                })
                .collect()
        })
        .collect()
}

fn can_move(map: &[Vec<Space2>], pos: Position, delta: Position) -> bool {
    let space = map[pos.y as usize][pos.x as usize];
    match space {
        Space2::Empty => true,
        Space2::Wall => false,
        Space2::Box(side) => {
            let x_adjustment = side.other_x_adjust();
            can_move(map, pos + delta, delta)
                && can_move(map, pos + Position { x: x_adjustment, y: delta.y }, delta)
        }
    }
}

fn do_move(map: &mut [Vec<Space2>], pos: Position, delta: Position, new_space: Space2) {
    let space = map[pos.y as usize][pos.x as usize];
    match space {
        Space2::Empty => {}
        Space2::Box(side) => {
            // Push this half of the box up/down
            do_move(map, pos + delta, delta, space);

            // Push the other half of the box up/down
            let x_adjustment = side.other_x_adjust();
            do_move(
                map,
                pos + Position { x: x_adjustment, y: delta.y },
                delta,
                Space2::Box(side.other()),
            );

            // Mark empty the space occupied by the other half of the box
            map[pos.y as usize][(pos.x + x_adjustment) as usize] = Space2::Empty;
        }
        Space2::Wall => panic!("Attempted to move a box into a wall at {pos:?}"),
    }

    map[pos.y as usize][pos.x as usize] = new_space;
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day15.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample/day15-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("../../sample/day15-3.txt");

    #[test]
    fn part_1() {
        assert_eq!(2028, solve_part_1(SAMPLE_INPUT_2));
        assert_eq!(10092, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(618, solve_part_2(SAMPLE_INPUT_3));
        assert_eq!(9021, solve_part_2(SAMPLE_INPUT));
    }
}
