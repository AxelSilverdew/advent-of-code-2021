use itertools::Itertools;

pub fn parse_line(line: &str) -> ((usize, usize), (usize, usize)) {
    line.split(" -> ")
        .map(|s| {
            s.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

fn count_overlaps(input: &Vec<((usize, usize), (usize, usize))>, include_diagonals: bool) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    for (start, end) in input.iter() {
        if start.0 == end.0 {
            let min = std::cmp::min(start.1, end.1);
            let max = std::cmp::max(start.1, end.1);
            for i in min..=max {
                grid[i][start.0] += 1;
            }
        } else if start.1 == end.1 {
            let min = std::cmp::min(start.0, end.0);
            let max = std::cmp::max(start.0, end.0);
            for i in min..=max {
                grid[start.1][i] += 1;
            }
        } else if include_diagonals {
            let x_range: Box<dyn DoubleEndedIterator<Item = usize>> = if start.0 < end.0 {
                Box::new(start.0..=end.0)
            } else {
                Box::new((end.0..=start.0).rev())
            };

            let y_range: Box<dyn DoubleEndedIterator<Item = usize>> = if start.1 < end.1 {
                Box::new(start.1..=end.1)
            } else {
                Box::new((end.1..=start.1).rev())
            };
            for (i, j) in x_range.zip(y_range) {
                grid[j][i] += 1;
            }
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|n| **n > 1).count())
        .sum::<usize>()
}

pub fn part_1(input: &str) -> usize {
    let input: Vec<_> = input.lines().map(parse_line).collect();
    count_overlaps(&input, false)
}

pub fn part_2(input: &str) -> usize {
    let input: Vec<_> = input.lines().map(parse_line).collect();
    count_overlaps(&input, true)
}
