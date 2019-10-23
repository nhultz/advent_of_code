use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

use crate::Result;

pub fn part1() -> Result<i64> {
    let file = File::open("data/day1_input.txt")?;
    let reader = BufReader::new(file);

    let mut freq = 0;

    for line in reader.lines() {
        let val: i64 = line?.trim().parse()?;
        freq = freq + val;
    }

    Ok(freq)
}

pub fn part2() -> Result<i64> {
    let file = File::open("data/day1_input.txt")?;
    let reader = BufReader::new(file);

    let mut freqs = HashSet::new();
    let mut freq = 0;

    let numbers: Vec<i64> = reader.lines()
        .flat_map(|line| line)
        .map(|line| line.trim().parse())
        .flat_map(|num| num)
        .collect();

    loop {
        for num in &numbers {
            freq = freq + num;
            if freqs.contains(&freq) {
                return Ok(freq)
            } else {
                freqs.insert(freq);
            }
        }
    }
}
