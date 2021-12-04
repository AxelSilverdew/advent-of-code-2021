//! I'm just using this file as a launcher :p

use advent_runner::{
    day_1, day_2,
    day_3::{self, read_input},
    day_4, input_to_vec,
};
fn main() {
    {
        let day_1_input = &input_to_vec("src/inputs/day_1.txt");
        println!("=== Day 1 Solutions ===");
        println!("Part 1 Solution: {}", day_1::part_1(&day_1_input));
        println!("Part 2 Solution: {}", day_1::part_2(&day_1_input));
        println!();
    }
    {
        let day_2_input = &input_to_vec("src/inputs/day_2.txt");
        println!("=== Day 2 Solutions ===");
        println!("Part 1 Solution: {:?}", day_2::part_1(&day_2_input));
        println!("Part 2 Solution: {:?}", day_2::part_2(&day_2_input));
        println!();
    }
    {
        let day_3_input = &read_input(include_str!("inputs/day_3.txt"));
        println!("=== Day 3 Solutions ===");
        println!("Part 1 Solution: {:?}", day_3::part_1(&day_3_input));
        println!(
            "Part 2 Solution: {:?}",
            day_3::part_2(&day_3_input, 1) * day_3::part_2(&day_3_input, 0)
        );
        println!();
    }
    {
        let day_4_input = include_str!("inputs/day_4.txt");
        println!("=== Day 4 Solutions ===");
        println!("Part 1 Solution: {:?}", day_4::part_1(&day_4_input));
        println!("Part 2 Solution: {:?}", day_4::part_2(&day_4_input));
        println!();
    }
}
