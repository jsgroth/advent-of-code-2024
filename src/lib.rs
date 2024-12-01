use std::error::Error;
use std::fmt::Display;
use std::{env, fs, io};

pub fn read_input() -> io::Result<String> {
    let mut args = env::args();
    args.next();

    let input_filename = args.next().expect("ARGS: <filename>");
    fs::read_to_string(&input_filename)
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

    Ok(())
}
