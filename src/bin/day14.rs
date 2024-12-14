//! Day 14: Restroom Redoubt
//!
//! <https://adventofcode.com/2024/day/14>

use advent_of_code_2024::Pos2;
use rustc_hash::FxHashSet;
use std::error::Error;
use std::iter;
use winnow::ascii::{digit1, newline};
use winnow::combinator::{opt, preceded, separated, separated_pair, terminated};
use winnow::prelude::*;

type Position = Pos2<i64>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Robot {
    position: Position,
    velocity: Position,
}

impl Robot {
    fn clamp_position(&mut self, width: i64, height: i64) {
        clamp_coordinate(&mut self.position.x, width);
        clamp_coordinate(&mut self.position.y, height);
    }
}

fn clamp_coordinate(coordinate: &mut i64, bound: i64) {
    while *coordinate < 0 {
        *coordinate += bound;
    }

    while *coordinate >= bound {
        *coordinate -= bound;
    }
}

fn parse_i64(input: &mut &str) -> PResult<i64> {
    let sign = opt('-').parse_next(input)?;
    let magnitude: i64 = digit1.parse_to().parse_next(input)?;
    Ok(match sign {
        Some(_) => -magnitude,
        None => magnitude,
    })
}

fn parse_position(input: &mut &str) -> PResult<Position> {
    let (x, y) = separated_pair(parse_i64, ',', parse_i64).parse_next(input)?;
    Ok(Position { x, y })
}

fn parse_robot(input: &mut &str) -> PResult<Robot> {
    let (position, velocity) =
        separated_pair(preceded("p=", parse_position), ' ', preceded("v=", parse_position))
            .parse_next(input)?;
    Ok(Robot { position, velocity })
}

fn parse_input(input: &mut &str) -> PResult<Vec<Robot>> {
    terminated(separated(1.., parse_robot, newline), opt(newline)).parse_next(input)
}

const REAL_WIDTH: i64 = 101;
const REAL_HEIGHT: i64 = 103;

fn solve_part_1(input: &str, width: i64, height: i64) -> i32 {
    let mut robots = parse_input.parse(input).unwrap();

    for _ in 0..100 {
        for robot in &mut robots {
            robot.position += robot.velocity;
            robot.clamp_position(width, height);
        }
    }

    let mut quadrant_counts = [0; 4];
    for robot in &robots {
        if robot.position.x == width / 2 || robot.position.y == height / 2 {
            continue;
        }

        let quadrant = 2 * usize::from(robot.position.x < width / 2)
            + usize::from(robot.position.y < height / 2);
        quadrant_counts[quadrant] += 1;
    }

    quadrant_counts.into_iter().product()
}

// General idea for part 2: repeatedly move the robots until a cycle is detected. At each second,
// score the robot layout by summing the distance squared of every robot from the center of the room
// (assuming the final picture will be clustered near-ish the center of the room). Once a cycle is
// detected, the robot layout with the min score is _probably_ the solution.
//
// The winning layout is printed to stdout for visual verification.
fn solve_part_2(input: &str) -> i32 {
    let mut robots = parse_input.parse(input).unwrap();

    let mut previous_layouts_set: FxHashSet<_> = iter::once(robot_positions(&robots)).collect();
    let mut previous_layouts_vec = vec![robot_positions(&robots)];

    let mut min_score = score(&robots);
    let mut min_time = 0;
    for second in 1.. {
        for robot in &mut robots {
            robot.position += robot.velocity;
            robot.clamp_position(REAL_WIDTH, REAL_HEIGHT);
        }

        let second_score = score(&robots);
        if second_score < min_score {
            min_score = second_score;
            min_time = second;
        }

        if !previous_layouts_set.insert(robot_positions(&robots)) {
            break;
        }
        previous_layouts_vec.push(robot_positions(&robots));
    }

    let mut grid = [[0; REAL_WIDTH as usize]; REAL_HEIGHT as usize];
    for position in &previous_layouts_vec[min_time as usize] {
        grid[position.y as usize][position.x as usize] += 1;
    }

    for row in grid {
        for robot_count in row {
            let c = match robot_count {
                0 => ' ',
                _ => '█',
            };
            print!("{c}");
        }
        println!();
    }
    println!();

    min_time
}

fn robot_positions(robots: &[Robot]) -> Vec<Position> {
    robots.iter().map(|robot| robot.position).collect()
}

fn score(robots: &[Robot]) -> i64 {
    robots
        .iter()
        .map(|robot| {
            let x_delta = (robot.position.x - REAL_WIDTH / 2).abs().pow(2);
            let y_delta = (robot.position.y - REAL_HEIGHT / 2).abs().pow(2);
            x_delta + y_delta
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(|input| solve_part_1(input, REAL_WIDTH, REAL_HEIGHT), solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day14.txt");

    #[test]
    fn part_1() {
        const TEST_WIDTH: i64 = 11;
        const TEST_HEIGHT: i64 = 7;

        assert_eq!(12, solve_part_1(SAMPLE_INPUT, TEST_WIDTH, TEST_HEIGHT));
    }
}
