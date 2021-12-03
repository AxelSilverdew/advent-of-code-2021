//! # Day 3's Solutions.
//! You can look at the question [here](https://adventofcode.com/2021/day/3).
//! Needed a LOT of help to do this one. Boolean math is hard. :<

pub fn read_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>()
}

fn max_bit(nums: &[u32], bit: usize) -> u32 {
    let mut c = [0, 0];
    for &x in nums {
        c[(x as usize >> bit) & 1] += 1
    }
    (c[1] >= c[0]) as u32
}

pub fn part_1(nums: &[u32]) -> u32 {
    let x = (0..12).map(|i| max_bit(nums, i) << i).sum::<u32>();
    x * (!x & 0xfff)
}

pub fn part_2(nums: &[u32], oxygen: u32) -> u32 {
    let mut nums = nums.to_vec();
    for i in (0..12).rev() {
        let keep = max_bit(&nums, i) ^ oxygen;
        nums.retain(|x| (x >> i) & 1 == keep);
        if nums.len() == 1 {
            break;
        }
    }
    nums[0]
}
