//! # Advent of Code 2021 Solutions
//!
//! I'm attempting this year's AoC in Rust.
//! I don't know how far I'll get, but I'm gonna try to have fun with it.
//!
//! Each day's solutions are in their own module and they're all being executed from the main.rs.
//!
//! To see the output, just `git clone` this repo and do a `cargo run`.

pub mod day_1;

pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[test]
pub fn test_parse_input() {
    let input = parse_input("199 200 208 210 200 207 240 269 260 263");
    assert_eq!(input.len(), 10);
}
