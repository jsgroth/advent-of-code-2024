use std::error::Error;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::time::Instant;
use std::{env, fs, hint, io};

pub fn read_input() -> io::Result<String> {
    let mut args = env::args();
    args.next();

    let input_filename = args.next().expect("ARGS: <filename>");
    fs::read_to_string(&input_filename)
}

const TIME_ITERATIONS: u128 = 100;

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

    if env::var("AOCTIME").is_ok_and(|var| !var.is_empty()) {
        let duration1 = time_micros(|| solve1(&input));
        println!("Part 1 time: {duration1}μs");

        let duration2 = time_micros(|| solve2(&input));
        println!("Part 2 time: {duration2}μs");
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Add<Output = T>> Add for Pos2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Copy + AddAssign> AddAssign for Pos2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Pos2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: Copy + SubAssign> SubAssign for Pos2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Pos2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Pos2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
