//! Day 12: Garden Groups
//!
//! <https://adventofcode.com/2024/day/12>

use rustc_hash::FxHashMap;
use std::cmp;
use std::error::Error;

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().filter(|line| !line.is_empty()).map(|line| line.as_bytes()).collect()
}

fn solve_part_1(input: &str) -> u32 {
    let map = parse_input(input);
    let (regions, region_to_area) = build_region_and_area_maps(&map);

    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let area = *region_to_area.get(&regions[i][j]).unwrap();

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

fn build_region_and_area_maps(map: &[&[u8]]) -> (Vec<Vec<u32>>, FxHashMap<u32, u32>) {
    let mut regions = vec![vec![0; map[0].len()]; map.len()];

    let mut current_region = 1;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if regions[i][j] == 0 {
                floodfill(map, i, j, current_region, &mut regions);
                current_region += 1;
            }
        }
    }

    let mut region_to_area: FxHashMap<u32, u32> = FxHashMap::default();
    for row in &regions {
        for &value in row {
            *region_to_area.entry(value).or_default() += 1;
        }
    }

    (regions, region_to_area)
}

fn floodfill(map: &[&[u8]], i: usize, j: usize, current_region: u32, regions: &mut [Vec<u32>]) {
    regions[i][j] = current_region;

    for (di, dj) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let ii = i as i32 + di;
        let jj = j as i32 + dj;
        if (0..map.len() as i32).contains(&ii)
            && (0..map[0].len() as i32).contains(&jj)
            && regions[ii as usize][jj as usize] == 0
            && map[ii as usize][jj as usize] == map[i][j]
        {
            floodfill(map, ii as usize, jj as usize, current_region, regions);
        }
    }
}

fn solve_part_2(input: &str) -> u32 {
    let map = parse_input(input);
    let (regions, region_to_area) = build_region_and_area_maps(&map);

    let mut side_count: FxHashMap<u32, u32> = FxHashMap::default();

    // Count vertical edges
    for j in 0..map[0].len() {
        let first_col = j == 0;
        let last_col = j == map[0].len() - 1;

        // Count edges to the left of this column
        let mut i = 0;
        while i < map.len() {
            let region = regions[i][j];

            let mut ii = i;
            while ii < map.len()
                && regions[ii][j] == region
                && (first_col || regions[ii][j - 1] != region)
            {
                ii += 1;
            }
            if ii != i {
                *side_count.entry(region).or_default() += 1;
            }
            i = cmp::max(ii, i + 1);
        }

        // Count edges to the right of this column
        let mut i = 0;
        while i < map.len() {
            let region = regions[i][j];

            let mut ii = i;
            while ii < map.len()
                && regions[ii][j] == region
                && (last_col || regions[ii][j + 1] != region)
            {
                ii += 1;
            }
            if ii != i {
                *side_count.entry(region).or_default() += 1;
            }
            i = cmp::max(ii, i + 1);
        }
    }

    // Count horizontal edges
    for i in 0..map.len() {
        let first_row = i == 0;
        let last_row = i == map.len() - 1;

        // Count edges above this row
        let mut j = 0;
        while j < map[0].len() {
            let region = regions[i][j];

            let mut jj = j;
            while jj < map[0].len()
                && regions[i][jj] == region
                && (first_row || regions[i - 1][jj] != region)
            {
                jj += 1;
            }
            if jj != j {
                *side_count.entry(region).or_default() += 1;
            }
            j = cmp::max(jj, j + 1);
        }

        // Count edges below this row
        let mut j = 0;
        while j < map[0].len() {
            let region = regions[i][j];

            let mut jj = j;
            while jj < map[0].len()
                && regions[i][jj] == region
                && (last_row || regions[i + 1][jj] != region)
            {
                jj += 1;
            }
            if jj != j {
                *side_count.entry(region).or_default() += 1;
            }
            j = cmp::max(jj, j + 1);
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
