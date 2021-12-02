//! # Day 2's Solutions.
//! You can look at the question [here](https://adventofcode.com/2021/day/2).

use std::str::FromStr;

#[derive(Debug)]
pub struct Commands {
    direction: Direction,
    value: isize,
}
impl FromStr for Commands {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<_> = s.split_whitespace().collect();
        let direction = data[0].parse()?;
        let value = data[1].parse().map_err(|_| "can't parse amount")?;

        Ok(Self { direction, value })
    }
}

#[derive(Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}
impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err("Bad Input"),
        }
    }
}

pub fn part_1(input: &Vec<Commands>) -> isize {
    let (mut horizontal, mut depth) = (0, 0);

    for command in input {
        match command.direction {
            Direction::Forward => horizontal += command.value,
            Direction::Down => depth += command.value,
            Direction::Up => depth -= command.value,
        }
    }
    horizontal * depth
}

pub fn part_2(input: &Vec<Commands>) -> isize {
    let (mut horizontal, mut depth, mut aim) = (0, 0, 0);

    for command in input {
        match command.direction {
            Direction::Forward => {
                horizontal += command.value;
                depth += command.value * aim;
            }
            Direction::Down => aim += command.value,
            Direction::Up => aim -= command.value,
        }
    }
    horizontal * depth
}
