# advent-of-code-2024

[Advent of Code 2024](https://adventofcode.com/2024) solutions

Each day's solution is an individual binary located in `src/bin/`, with shared code in `src/lib.rs` (and its submodules).

Correctness for sample inputs is verified via unit tests:
```shell
cargo test
```

To run on an actual input file, specify the day using `--bin` and pass the filename as a CLI arg, for example:
```shell
cargo run --release --bin day1 -- /path/to/input1.txt
```

These solutions use minimal third-party dependencies:
* [rustc-hash](https://crates.io/crates/rustc-hash): Faster hash function for use with the standard library HashMap and HashSet
* [winnow](https://crates.io/crates/winnow): Parser combinator library
