use crate::{file_input, Result};

pub fn part1() -> Result<String> {
    let entries: Vec<i64> = file_input("data/2021/day1_input.txt")?
        .map(|line| line.parse::<i64>())
        .flat_map(|line| line)
        .collect();

    let answer = entries.windows(2).filter(|n| n[0] < n[1]).count();
    Ok(format!("{}", answer))
}

pub fn part2() -> Result<String> {
    let entries: Vec<i64> = file_input("data/2021/day1_input.txt")?
        .map(|line| line.parse::<i64>())
        .flat_map(|line| line)
        .collect();

    let sums: Vec<i64> = entries.windows(3).map(|n| n[0] + n[1] + n[2]).collect();
    let answer = sums.windows(2).filter(|n| n[0] < n[1]).count();
    Ok(format!("{}", answer))
}
