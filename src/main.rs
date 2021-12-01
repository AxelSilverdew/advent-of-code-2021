//! I'm just using this file as a launcher :p

use advent_runner::{day_1, parse_input};
fn main() {
    {
        println!("=== Day 1 Solutions ===");
        println!(
            "Part 1 Solution: {}",
            day_1::part_1(&parse_input(include_str!("inputs/day_1.txt")))
        );
        println!(
            "Part 2 Solution: {}",
            day_1::part_2(&parse_input(include_str!("inputs/day_1.txt")))
        );
        println!();
    }
}
