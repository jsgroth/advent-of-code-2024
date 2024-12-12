//! Day 12: Garden Groups
//!
//! <https://adventofcode.com/2024/day/12>

use rustc_hash::FxHashMap;
use std::error::Error;

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().filter(|line| !line.is_empty()).map(|line| line.as_bytes()).collect()
}

fn solve_part_1(input: &str) -> u32 {
    let map = parse_input(input);
    let (_, area_map) = build_region_and_area_maps(&map);

    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let area = area_map[i][j];

            for (di, dj) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let ii = i as i32 + di;
                let jj = j as i32 + dj;

                if !(0..map.len() as i32).contains(&ii)
                    || !(0..map[0].len() as i32).contains(&jj)
                    || map[ii as usize][jj as usize] != map[i][j]
                {
                    total += area;
                }
            }
        }
    }

    total
}

fn build_region_and_area_maps(map: &[&[u8]]) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut regions = vec![vec![0; map[0].len()]; map.len()];

    let mut current_region = 1;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if regions[i][j] == 0 {
                floodfill(map, i, j, current_region, map[i][j], &mut regions);
                current_region += 1;
            }
        }
    }

    let mut counts: FxHashMap<u32, u32> = FxHashMap::default();
    for row in &regions {
        for &value in row {
            *counts.entry(value).or_default() += 1;
        }
    }

    let area_map = regions
        .iter()
        .map(|row| row.iter().map(|&region| *counts.get(&region).unwrap()).collect())
        .collect();

    (regions, area_map)
}

fn floodfill(
    map: &[&[u8]],
    i: usize,
    j: usize,
    current_region: u32,
    current_char: u8,
    regions: &mut [Vec<u32>],
) {
    regions[i][j] = current_region;

    for (di, dj) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let ii = i as i32 + di;
        let jj = j as i32 + dj;
        if (0..map.len() as i32).contains(&ii)
            && (0..map[0].len() as i32).contains(&jj)
            && regions[ii as usize][jj as usize] == 0
            && map[ii as usize][jj as usize] == current_char
        {
            floodfill(map, ii as usize, jj as usize, current_region, current_char, regions);
        }
    }
}

fn solve_part_2(input: &str) -> u32 {
    let map = parse_input(input);
    let (region_map, area_map) = build_region_and_area_maps(&map);

    let mut side_count: FxHashMap<u32, u32> = FxHashMap::default();

    // Count vertical edges
    let mut counted_left = vec![vec![false; map[0].len()]; map.len()];
    let mut counted_right = vec![vec![false; map[0].len()]; map.len()];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let region = region_map[i][j];

            // Check for an edge to the left
            let mut ii = i;
            while ii < map.len()
                && !counted_left[ii][j]
                && region_map[ii][j] == region
                && (j == 0 || region_map[ii][j - 1] != region)
            {
                counted_left[ii][j] = true;
                ii += 1;
            }
            if ii != i {
                *side_count.entry(region).or_default() += 1;
            }

            // Check for an edge to the right
            ii = i;
            while ii < map.len()
                && !counted_right[ii][j]
                && region_map[ii][j] == region
                && (j == map[0].len() - 1 || region_map[ii][j + 1] != region)
            {
                counted_right[ii][j] = true;
                ii += 1;
            }
            if ii != i {
                *side_count.entry(region).or_default() += 1;
            }
        }
    }

    // Count horizontal edges
    let mut counted_top = vec![vec![false; map[0].len()]; map.len()];
    let mut counted_bottom = vec![vec![false; map[0].len()]; map.len()];
    for j in 0..map[0].len() {
        for i in 0..map.len() {
            let region = region_map[i][j];

            // Check for an edge above
            let mut jj = j;
            while jj < map[i].len()
                && !counted_top[i][jj]
                && region_map[i][jj] == region
                && (i == 0 || region_map[i - 1][jj] != region)
            {
                counted_top[i][jj] = true;
                jj += 1;
            }
            if jj != j {
                *side_count.entry(region).or_default() += 1;
            }

            // Check for an edge below
            jj = j;
            while jj < map[i].len()
                && !counted_bottom[i][jj]
                && region_map[i][jj] == region
                && (i == map.len() - 1 || region_map[i + 1][jj] != region)
            {
                counted_bottom[i][jj] = true;
                jj += 1;
            }
            if jj != j {
                *side_count.entry(region).or_default() += 1;
            }
        }
    }

    let mut region_to_area: FxHashMap<u32, u32> = FxHashMap::default();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            region_to_area.insert(region_map[i][j], area_map[i][j]);
        }
    }

    let mut total = 0;
    for (&region, &area) in &region_to_area {
        let count = *side_count.get(&region).unwrap();
        total += count * area;
    }

    total
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day12.txt");
    const SAMPLE_INPUT_2: &str = include_str!("../../sample/day12-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("../../sample/day12-3.txt");
    const SAMPLE_INPUT_4: &str = include_str!("../../sample/day12-4.txt");
    const SAMPLE_INPUT_5: &str = include_str!("../../sample/day12-5.txt");

    #[test]
    fn part_1() {
        assert_eq!(140, solve_part_1(SAMPLE_INPUT));
        assert_eq!(772, solve_part_1(SAMPLE_INPUT_2));
        assert_eq!(1930, solve_part_1(SAMPLE_INPUT_3));
    }

    #[test]
    fn part_2() {
        assert_eq!(80, solve_part_2(SAMPLE_INPUT));
        assert_eq!(436, solve_part_2(SAMPLE_INPUT_2));
        assert_eq!(236, solve_part_2(SAMPLE_INPUT_4));
        assert_eq!(368, solve_part_2(SAMPLE_INPUT_5));
        assert_eq!(1206, solve_part_2(SAMPLE_INPUT_3));
    }
}
