//! I'm just using this file as a launcher :p

use advent_runner::{day_1, day_2, input_to_vec};
fn main() {
    {
        println!("=== Day 1 Solutions ===");
        println!(
            "Part 1 Solution: {}",
            day_1::part_1(&input_to_vec("src/inputs/day_1.txt"))
        );
        println!(
            "Part 2 Solution: {}",
            day_1::part_2(&input_to_vec("src/inputs/day_1.txt"))
        );
        println!();
    }
    {
        println!("=== Day 2 Solutions ===");
        println!(
            "Part 1 Solution: {:?}",
            day_2::part_1(&input_to_vec("src/inputs/day_2.txt"))
        );
        println!(
            "Part 2 Solution: {:?}",
            day_2::part_2(&input_to_vec("src/inputs/day_2.txt"))
        );
        println!();
    }
}
