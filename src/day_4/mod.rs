//! # Day 4's Solutions.
//! You can look at the question [here](https://adventofcode.com/2021/day/4).
//! TIL about nalgebra. It's a pretty cool crate!!

use nalgebra::Matrix5;

type Board = Matrix5<(u32, bool)>;

struct Input {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

impl Input {
    fn new(input: &str) -> Self {
        let mut lines = input.split("\n\n");
        let draws = lines
            .next()
            .unwrap()
            .split(',')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>();

        let boards = lines
            .map(|board| {
                Matrix5::from_iterator(board.lines().flat_map(|line| {
                    line.split_whitespace()
                        .filter_map(|x| x.parse::<u32>().ok())
                        .map(|x| (x, false))
                }))
            })
            .collect::<Vec<Board>>();

        Self { draws, boards }
    }
}

fn update_board(board: &mut Board, num: u32) {
    board.iter_mut().for_each(|x| {
        if x.0 == num {
            x.1 = true;
        }
    });
}

fn check_win(board: &Board) -> bool {
    let column_won = board
        .column_iter()
        .any(|col| col.iter().all(|(_, marked)| *marked));
    let row_won = board
        .row_iter()
        .any(|row| row.iter().all(|(_, marked)| *marked));

    row_won || column_won
}

fn calc_unmarked_sum(board: &Board) -> u32 {
    board
        .iter()
        .map(|(x, marked)| if !marked { *x } else { 0 })
        .sum()
}

pub fn part_1(input: &str) -> u32 {
    let Input { draws, mut boards } = Input::new(input);

    for draw in draws {
        for board in boards.iter_mut() {
            update_board(board, draw);
            if check_win(&board) {
                return draw * calc_unmarked_sum(&board);
            }
        }
    }
    0
}

pub fn part_2(input: &str) -> u32 {
    let Input { draws, mut boards } = Input::new(input);

    let mut boards_won = vec![false; boards.len()];

    for draw in draws {
        for (i, board) in boards.iter_mut().enumerate() {
            if boards_won[i] {
                continue;
            }
            update_board(board, draw);

            if check_win(&board) {
                boards_won[i] = true;
                if boards_won.iter().all(|x| *x) {
                    return draw * calc_unmarked_sum(&board);
                }
            }
        }
    }
    0
}
