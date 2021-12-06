//! # Advent of Code 2021 Solutions
//!
//! I'm attempting this year's AoC in Rust.
//! I don't know how far I'll get, but I'm gonna try to have fun with it.
//!
//! Each day's solutions are in their own module and they're all being executed from the main.rs.
//!
//! To see the output, just `git clone` this repo and do a `cargo run`.

use std::{fs::read_to_string, path::Path, str::FromStr};

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;

pub fn input_to_vec<T, P>(input: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let data = read_to_string(&input).expect("Input file missing");
    data.lines()
        .map(|x| x.parse().expect("Unable to parse line"))
        .collect()
}

pub fn string_to_vec(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}
