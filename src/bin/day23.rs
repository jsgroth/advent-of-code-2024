//! Day 23: LAN Party
//!
//! <https://adventofcode.com/2024/day/23>

use rustc_hash::{FxHashMap, FxHashSet};
use std::error::Error;
use std::iter;

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

    // Accumulate all unique computer strings into a Vec
    let computers: FxHashSet<_> = connections.iter().flat_map(|&(a, b)| [a, b]).collect();
    let computers: Vec<_> = computers.into_iter().collect();

    // Convert everything to u32s because that is significantly faster
    // Replace each computer string with its index in `computers`
    let computer_idx_map: FxHashMap<&str, u32> =
        computers.iter().enumerate().map(|(i, &computer)| (computer, i as u32)).collect();

    let mut connections_map: FxHashMap<u32, Vec<u32>> = FxHashMap::default();
    let mut connections_set: FxHashSet<(u32, u32)> = FxHashSet::default();
    for &(a, b) in &connections {
        let a_idx = *computer_idx_map.get(&a).unwrap();
        let b_idx = *computer_idx_map.get(&b).unwrap();

        for (aa, bb) in [(a_idx, b_idx), (b_idx, a_idx)] {
            connections_map.entry(aa).or_default().push(bb);
            connections_set.insert((aa, bb));
        }
    }

    // Sort the connections map values in reverse order to make it possible to avoid needing to
    // scan the entire Vec later
    for value in connections_map.values_mut() {
        value.sort_by(|a, b| a.cmp(b).reverse());
    }

    let max_group = find_max_group(computers.len() as u32, &connections_map, &connections_set);
    let mut max_group_str: Vec<_> =
        max_group.into_iter().map(|idx| computers[idx as usize]).collect();
    max_group_str.sort();

    max_group_str.join(",")
}

fn find_max_group(
    num_computers: u32,
    connections_map: &FxHashMap<u32, Vec<u32>>,
    connections_set: &FxHashSet<(u32, u32)>,
) -> Vec<u32> {
    // Initialize with a single group for each computer
    let mut groups: Vec<_> = (0..num_computers).map(|computer| vec![computer]).collect();

    // Loop until there is only 1 group left
    // In each iteration, replace `groups` with all groups that are 1 larger
    let mut solution = Vec::new();
    while !groups.is_empty() {
        let mut next_groups = Vec::new();

        // This silliness is necessary because consuming `groups` in the following loop slightly
        // improves performance compared to not consuming it (i.e. `for group in &groups`)
        if groups.len() == 1 {
            solution = groups[0].clone();
        }

        for group in groups {
            let last = *group.last().unwrap();
            for &connection in connections_map.get(&group[0]).unwrap() {
                if connection <= last {
                    break;
                }

                if group[1..].iter().all(|&other| connections_set.contains(&(other, connection))) {
                    next_groups.push(group.iter().copied().chain(iter::once(connection)).collect());
                }
            }
        }

        groups = next_groups;
    }

    assert!(!solution.is_empty(), "More than 1 group of max length");
    solution
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
