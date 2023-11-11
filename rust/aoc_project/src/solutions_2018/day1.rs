use std::collections::HashSet;

use crate::{file_input, Result};

pub fn part1() -> Result<String> {
    let freq: i64 = file_input("data/2018/day1_input.txt")?
        .map(|line| line.parse::<i64>())
        .flat_map(|line| line)
        .sum();

    Ok(freq.to_string())
}

pub fn part2() -> Result<String> {
    let numbers: Vec<i64> = file_input("data/2018/day1_input.txt")?
        .map(|line| line.parse::<i64>())
        .flat_map(|line| line)
        .collect();

    let mut freqs = HashSet::new();
    let mut freq = 0;

    loop {
        for num in &numbers {
            freq = freq + num;
            if freqs.contains(&freq) {
                return Ok(freq.to_string());
            } else {
                freqs.insert(freq);
            }
        }
    }
}
