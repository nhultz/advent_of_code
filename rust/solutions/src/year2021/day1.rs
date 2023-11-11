use aoc_derive::{aoc, aoc_input};
use std::num::ParseIntError;

#[aoc_input(day1)]
fn parse(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(freqs: &[i32]) -> usize {
    freqs.windows(2).filter(|n| n[0] < n[1]).count()
}

#[aoc(day1, part2)]
fn part2(freqs: &[i32]) -> usize {
    let sums: Vec<i32> = freqs.windows(3).map(|n| n[0] + n[1] + n[2]).collect();
    sums.windows(2).filter(|n| n[0] < n[1]).count()
}
