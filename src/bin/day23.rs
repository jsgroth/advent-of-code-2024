//! Day 23: LAN Party
//!
//! <https://adventofcode.com/2024/day/23>

use rustc_hash::{FxHashMap, FxHashSet};
use std::error::Error;

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .collect()
}

fn solve_part_1(input: &str) -> usize {
    let connections = parse_input(input);
    let connections_set = build_connections_set(&connections);
    let connections_map = build_connections_map(&connections);

    let mut t_triples: FxHashSet<[&str; 3]> = FxHashSet::default();
    for &(a, b) in &connections {
        for &c in connections_map.get(&b).unwrap() {
            if a == c {
                continue;
            }

            if !a.starts_with('t') && !b.starts_with('t') && !c.starts_with('t') {
                continue;
            }

            if connections_set.contains(&(a, c)) {
                let mut triple = [a, b, c];
                triple.sort();
                t_triples.insert(triple);
            }
        }
    }

    t_triples.len()
}

fn build_connections_set<'a>(connections: &[(&'a str, &'a str)]) -> FxHashSet<(&'a str, &'a str)> {
    let mut set = FxHashSet::default();
    for &(a, b) in connections {
        set.insert((a, b));
        set.insert((b, a));
    }
    set
}

fn build_connections_map<'a>(
    connections: &[(&'a str, &'a str)],
) -> FxHashMap<&'a str, Vec<&'a str>> {
    let mut map: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
    for &(a, b) in connections {
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
    }
    map
}

fn solve_part_2(input: &str) -> String {
    let connections = parse_input(input);
    let connections_set = build_connections_set(&connections);
    let connections_map = build_connections_map(&connections);
    let computers: Vec<_> = connections_map.keys().copied().collect();

    let mut max = max_group(&computers, &connections_set, &[]);
    max.sort();

    max.join(",")
}

fn max_group<'a>(
    computers: &[&'a str],
    connections: &FxHashSet<(&str, &str)>,
    group: &[&'a str],
) -> Vec<&'a str> {
    if computers.is_empty() {
        return group.to_vec();
    }

    let mut max = max_group(&computers[1..], connections, group);
    if group.iter().all(|computer| connections.contains(&(computer, computers[0]))) {
        let mut new_group = group.to_vec();
        new_group.push(computers[0]);
        let sub_result = max_group(&computers[1..], connections, &new_group);
        if sub_result.len() > max.len() {
            max = sub_result;
        }
    }

    max
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day23.txt");

    #[test]
    fn part_1() {
        assert_eq!(7, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!("co,de,ka,ta", solve_part_2(SAMPLE_INPUT).as_str());
    }
}
