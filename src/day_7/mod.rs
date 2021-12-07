use itertools::Itertools;

use crate::string_to_vec;

fn cost_at(pos: usize, crabs: &[usize]) -> usize {
    crabs
        .iter()
        .map(|&crab_pos| {
            if crab_pos > pos {
                crab_pos - pos
            } else {
                pos - crab_pos
            }
        })
        .sum()
}

fn read_input(input: &str) -> Vec<usize> {
    let mut crabs: Vec<usize> = string_to_vec(input);
    crabs.sort_unstable();
    crabs
}

fn ordered<O: PartialOrd>(a: O, b: O) -> (O, O) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn sum_until(end: usize) -> usize {
    (end * (1 + end)) / 2
}

fn cost_at_part_2(pos: usize, groups: &[(usize, usize)]) -> usize {
    groups
        .iter()
        .map(|&(number, new_pos)| {
            let (first, last) = ordered(pos, new_pos);
            number * sum_until(last - first)
        })
        .sum()
}

fn ternary_search(mut min: usize, mut max: usize, callback: impl Fn(usize) -> usize) -> usize {
    while max - min > 6 {
        let mid1 = min + (max - min) / 3;
        let mid2 = max - (max - min) / 3;

        let cost1 = callback(mid1);
        let cost2 = callback(mid2);

        if cost1 < cost2 {
            max = mid2 - 1
        } else {
            min = mid1 + 1
        }
    }

    // Ternary search isn't effective at such small intervals so we iterate the remaining part
    (min..=max).map(callback).min().unwrap()
}

pub fn part_1(input: &str) -> usize {
    let crabs = read_input(input);
    let median = crabs[crabs.len() / 2 + (crabs.len() % 2)];
    cost_at(median, &crabs)
}

pub fn part_2(input: &str) -> usize {
    let groups: Vec<_> = read_input(input).into_iter().dedup_with_count().collect();

    let min = groups.first().unwrap().1;
    let max = groups.last().unwrap().1;

    ternary_search(min, max, |pos| cost_at_part_2(pos, &groups))
}
