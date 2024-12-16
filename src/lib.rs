mod pos;

pub use pos::Pos2;

use std::error::Error;
use std::fmt::Display;
use std::time::Instant;
use std::{env, fs, hint, io};

pub fn read_input() -> io::Result<String> {
    let mut args = env::args();
    args.next();

    let input_filename = args.next().expect("ARGS: <filename>");
    fs::read_to_string(&input_filename)
}

const TIME_ITERATIONS: u128 = 100;

fn should_time() -> bool {
    env::var("AOCTIME").is_ok_and(|var| !var.is_empty())
}

fn time_micros<T>(f: impl Fn() -> T) -> u128 {
    let mut elapsed_sum = 0;
    for _ in 0..TIME_ITERATIONS {
        let start = Instant::now();
        hint::black_box(f());
        elapsed_sum += (Instant::now() - start).as_micros();
    }
    elapsed_sum / TIME_ITERATIONS
}

pub fn run<T1, T2>(
    solve1: impl Fn(&str) -> T1,
    solve2: impl Fn(&str) -> T2,
) -> Result<(), Box<dyn Error>>
where
    T1: Display,
    T2: Display,
{
    let input = read_input()?;

    let solution1 = solve1(&input);
    println!("{solution1}");

    let solution2 = solve2(&input);
    println!("{solution2}");

    if should_time() {
        let duration1 = time_micros(|| solve1(&input));
        println!("Part 1 time: {duration1}μs");

        let duration2 = time_micros(|| solve2(&input));
        println!("Part 2 time: {duration2}μs");
    }

    Ok(())
}

pub fn run_single_fn<T1, T2>(solve: impl Fn(&str) -> (T1, T2)) -> Result<(), Box<dyn Error>>
where
    T1: Display,
    T2: Display,
{
    let input = read_input()?;
    let (solution1, solution2) = solve(&input);
    println!("{solution1}");
    println!("{solution2}");

    if should_time() {
        let duration = time_micros(|| solve(&input));
        println!("Solution time: {duration}μs");
    }

    Ok(())
}
