//! Day 11: Plutonian Pebbles
//!
//! <https://adventofcode.com/2024/day/11>

use rustc_hash::FxHashMap;
use std::error::Error;

fn solve(input: &str, blinks: u32) -> u64 {
    let initial_stones: Vec<_> =
        input.lines().next().unwrap().split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

    let mut stones: FxHashMap<u64, u64> = FxHashMap::default();
    for stone in initial_stones {
        *stones.entry(stone).or_default() += 1;
    }

    for _ in 0..blinks {
        let mut next_stones = FxHashMap::default();

        for (&stone, &count) in &stones {
            if stone == 0 {
                // All 0s become 1
                *next_stones.entry(1).or_default() += count;
            } else {
                let log10 = stone.ilog10();
                if log10 % 2 == 0 {
                    // Odd number of digits; multiply by 2024
                    *next_stones.entry(stone * 2024).or_default() += count;
                } else {
                    // Even number of digits; split into left half of digits and right half of digits
                    let split_pow10 = 10_u64.pow((log10 + 1) / 2);
                    let l = stone / split_pow10;
                    let r = stone % split_pow10;
                    for next_stone in [l, r] {
                        *next_stones.entry(next_stone).or_default() += count;
                    }
                }
            }
        }

        stones = next_stones;
    }

    stones.values().sum()
}

const P1_BLINKS: u32 = 25;
const P2_BLINKS: u32 = 75;

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2024::run(|input| solve(input, P1_BLINKS), |input| solve(input, P2_BLINKS))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "0 1 10 99 999";
    const SAMPLE_INPUT_2: &str = "125 17";

    #[test]
    fn solution() {
        assert_eq!(7, solve(SAMPLE_INPUT, 1));
        assert_eq!(22, solve(SAMPLE_INPUT_2, 6));
        assert_eq!(55312, solve(SAMPLE_INPUT_2, 25));
    }
}
