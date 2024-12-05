//! Day 5: Print Queue
//!
//! <https://adventofcode.com/2024/day/5>

use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};
use std::collections::HashSet;
use std::error::Error;
use winnow::ascii::{digit1, newline};
use winnow::combinator::{opt, repeat, separated, separated_pair, terminated};
use winnow::prelude::*;

#[derive(Debug)]
struct Input {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

fn parse_u32(input: &mut &str) -> PResult<u32> {
    digit1.parse_to().parse_next(input)
}

fn parse_rule(input: &mut &str) -> PResult<(u32, u32)> {
    separated_pair(parse_u32, '|', parse_u32).parse_next(input)
}

fn parse_rules(input: &mut &str) -> PResult<Vec<(u32, u32)>> {
    repeat(1.., terminated(parse_rule, newline)).parse_next(input)
}

fn parse_update(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., parse_u32, ',').parse_next(input)
}

fn parse_updates(input: &mut &str) -> PResult<Vec<Vec<u32>>> {
    separated(1.., parse_update, newline).parse_next(input)
}

fn parse_input(input: &mut &str) -> PResult<Input> {
    let rules = parse_rules.parse_next(input)?;
    newline.parse_next(input)?;
    let updates = parse_updates.parse_next(input)?;
    opt(newline).parse_next(input)?;

    Ok(Input { rules, updates })
}

fn solve_part_1(input: &str) -> u32 {
    let Input { rules, updates } = parse_input.parse(input).unwrap();

    let rules_graph = make_rules_graph(&rules);

    let mut sum = 0;
    let mut seen: FxHashSet<u32> = FxHashSet::default();
    'outer: for update in updates {
        seen.clear();
        for &page in &update {
            if let Some(edges) = rules_graph.get(&page) {
                for &edge in edges {
                    if seen.contains(&edge) {
                        continue 'outer;
                    }
                }
            }

            seen.insert(page);
        }

        sum += update[update.len() / 2];
    }

    sum
}

fn make_rules_graph(rules: &[(u32, u32)]) -> FxHashMap<u32, Vec<u32>> {
    let mut graph: FxHashMap<u32, Vec<u32>> = FxHashMap::default();
    for &(before, after) in rules {
        graph.entry(before).or_default().push(after);
    }

    graph
}

fn solve_part_2(input: &str) -> u32 {
    let Input { rules, updates } = parse_input.parse(input).unwrap();

    let rules_graph = make_rules_graph(&rules);

    let mut sum = 0;
    for update in updates {
        let sorted = topological_sort(&rules_graph, &update);
        if sorted != update {
            sum += sorted[sorted.len() / 2];
        }
    }

    sum
}

fn topological_sort(graph: &FxHashMap<u32, Vec<u32>>, update: &[u32]) -> Vec<u32> {
    let update_set: FxHashSet<_> = update.iter().copied().collect();

    let mut visited = HashSet::with_capacity_and_hasher(update.len(), FxBuildHasher);
    let mut sorted = Vec::with_capacity(update.len());
    for &page in update {
        if !visited.contains(&page) {
            topological_sort_visit(graph, page, &update_set, &mut visited, &mut sorted);
        }
    }
    sorted.reverse();

    sorted
}

fn topological_sort_visit(
    graph: &FxHashMap<u32, Vec<u32>>,
    page: u32,
    update: &FxHashSet<u32>,
    visited: &mut FxHashSet<u32>,
    sorted: &mut Vec<u32>,
) {
    visited.insert(page);

    if let Some(edges) = graph.get(&page) {
        for &edge in edges {
            if !visited.contains(&edge) && update.contains(&edge) {
                topological_sort_visit(graph, edge, update, visited, sorted);
            }
        }
    }

    sorted.push(page);
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day5.txt");

    #[test]
    fn part_1() {
        assert_eq!(143, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(123, solve_part_2(SAMPLE_INPUT));
    }
}
