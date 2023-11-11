use std::collections::HashSet;

use crate::{file_input, Result};

use anyhow::anyhow;

pub fn part1() -> Result<String> {
    let entries: Vec<i64> = file_input("data/2020/day1_input.txt")?
        .map(|line| line.parse::<i64>())
        .flat_map(|line| line)
        .collect();

    let mut seen = HashSet::new();

    for n in entries {
        let matching_num = 2020 - n;

        if seen.contains(&matching_num) {
            let answer = n * matching_num;
            return Ok(format!("{}", answer));
        } else {
            seen.insert(n);
        }
    }

    Err(anyhow!("No answer found"))
}

pub fn part2() -> Result<String> {
    let entries: Vec<i64> = file_input("data/2020/day1_input.txt")?
        .map(|line| line.parse::<i64>())
        .flat_map(|line| line)
        .collect();

    let mut seen = HashSet::new();

    for n1 in entries.iter() {
        for n2 in entries.iter() {
            let matching_num = 2020 - n1 - n2;

            if seen.contains(&matching_num) {
                let answer = n1 * n2 * matching_num;
                return Ok(format!("{}", answer));
            } else {
                seen.insert(n1);
            }
        }
    }

    Err(anyhow!("No answer found"))
}

