//! # Day 1's Solutions.
//! You can look at the question [here](https://adventofcode.com/2021/day/1).
//!
//! I'm using the [`itermore`](https://lib.rs/crates/itermore) crate here. It's a pretty nifty lib.

use itermore::IterMore;

pub fn part_1(input: &Vec<i64>) -> usize {
    input.iter().windows().filter(|[a, b]| b > a).count()
}

pub fn part_2(input: &Vec<i64>) -> usize {
    input.iter().windows().filter(|[a, _, _, b]| b > a).count()
}
