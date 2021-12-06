//! # Day 6's Solutions.
//! You can look at the question [here](https://adventofcode.com/2021/day/6).

use crate::string_to_vec;

pub fn calculate(input: &str, days: usize) -> usize {
    let input = string_to_vec(input);
    let mut ages = [0; 9];
    for age in input {
        ages[age] += 1;
    }

    (0..days).for_each(|_| {
        ages.rotate_left(1);
        ages[6] += ages[8];
    });
    ages.iter().sum()
}
