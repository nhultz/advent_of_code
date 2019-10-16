use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::Result;

pub fn part1() -> Result<i64> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut freq = 0;

    for line in reader.lines() {
        let val: i64 = line?.trim().parse().unwrap();
        freq = freq + val;
    }

    Ok(freq)
}
