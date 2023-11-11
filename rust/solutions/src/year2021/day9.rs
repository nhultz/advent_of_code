use aoc_derive::{aoc, aoc_input};
use std::collections::VecDeque;

type Point = (usize, usize);

#[aoc_input(day9)]
fn parse(input: &str) -> Result<Vec<Vec<u32>>, String> {
    input
        .lines()
        .map(|l| {
            let nums = l
                .chars()
                .map(|c| c.to_digit(10).ok_or(format!("unable to parse")))
                .collect();
            nums
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    find_low_points(input)
        .into_iter()
        .map(|(r, c)| input[r][c] + 1)
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &Vec<Vec<u32>>) -> u64 {
    let mut sizes: Vec<u64> = find_low_points(input)
        .into_iter()
        .map(|p| find_basin_size(p, input))
        .collect();

    // Sort descending to find biggest
    sizes.sort_by(|a, b| b.cmp(a));

    // Take top 3 and multiply
    sizes.iter().take(3).product()
}

fn find_low_points(input: &[Vec<u32>]) -> Vec<Point> {
    let mut low_points = Vec::new();

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let left_bigger = check_left(input, row, col);
            let right_bigger = check_right(input, row, col);
            let up_bigger = check_up(input, row, col);
            let down_bigger = check_down(input, row, col);

            if left_bigger && right_bigger && up_bigger && down_bigger {
                let point = (row, col);
                low_points.push(point);
            }
        }
    }

    low_points
}

fn check_left(input: &[Vec<u32>], row: usize, col: usize) -> bool {
    col == 0 || input[row][col] < input[row][col - 1]
}

fn check_right(input: &[Vec<u32>], row: usize, col: usize) -> bool {
    col + 1 == input[row].len() || input[row][col] < input[row][col + 1]
}

fn check_up(input: &[Vec<u32>], row: usize, col: usize) -> bool {
    row == 0 || input[row][col] < input[row - 1][col]
}

fn check_down(input: &[Vec<u32>], row: usize, col: usize) -> bool {
    row + 1 == input.len() || input[row][col] < input[row + 1][col]
}

fn find_basin_size(p: Point, input: &[Vec<u32>]) -> u64 {
    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut visited: Vec<Vec<bool>> = Vec::with_capacity(num_rows);
    for _ in 0..num_rows {
        visited.push(vec![false; num_cols]);
    }

    let mut queue = VecDeque::new();
    queue.push_back(p);

    let mut cur_size = 0;
    while let Some(cur_point) = queue.pop_front() {
        let (row, col) = cur_point;

        if visited[row][col] {
            continue;
        }

        if input[row][col] == 9 {
            continue;
        }

        cur_size += 1;
        visited[row][col] = true;

        if row > 0 {
            queue.push_back((row - 1, col));
        }

        if row + 1 < input.len() {
            queue.push_back((row + 1, col));
        }

        if col > 0 {
            queue.push_back((row, col - 1));
        }

        if col + 1 < input[row].len() {
            queue.push_back((row, col + 1));
        }
    }

    cur_size
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        let entries = parse(INPUT).unwrap();
        assert_eq!(part1(&entries), 15);
    }

    #[test]
    fn test_part2() {
        let entries = parse(INPUT).unwrap();
        assert_eq!(part2(&entries), 1134);
    }
}
