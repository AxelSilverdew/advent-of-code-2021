// I'm confident there's a better way of doing this.
// I'm 100% gonna refactor this later.

use itertools::Itertools;

static DIGITS: [&str; 10] = [
    "abcdeg", "ab", "acdfg", "abcdf", "abef", "bcdef", "bcdefg", "abd", "abcdefg", "abcdef",
];

pub fn read_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" | ").unwrap();
            let x = a.split_whitespace().collect();
            let y = b.split_whitespace().collect();
            (x, y)
        })
        .collect::<Vec<_>>()
}

pub fn part_1((_, b): &(Vec<&str>, Vec<&str>)) -> usize {
    b.iter().filter(|x| [2, 3, 4, 7].contains(&x.len())).count()
}

fn display_digit(perm: &[char], s: &str) -> Option<usize> {
    let decoded = s
        .chars()
        .map(|c| perm[(c as u8 - b'a') as usize])
        .sorted()
        .collect::<String>();
    DIGITS.iter().position(|&s| s == decoded)
}

fn try_permutation(perm: &[char], (a, b): &(Vec<&str>, Vec<&str>)) -> Option<usize> {
    let invalid = a
        .iter()
        .map(|s| display_digit(&perm, s))
        .any(|o| o.is_none());
    if invalid {
        return None;
    }

    let ans = b
        .iter()
        .rev()
        .enumerate()
        .map(|(i, s)| display_digit(&perm, s).unwrap() * 10usize.pow(i as u32))
        .sum();
    Some(ans)
}

pub fn part_2(display: &(Vec<&str>, Vec<&str>)) -> usize {
    "abcdefg"
        .chars()
        .permutations(7)
        .find_map(|perm| try_permutation(&perm, display))
        .unwrap()
}
