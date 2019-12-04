use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Result;

pub fn part1() -> Result<String> {
    Ok("part1".into())
}

pub fn part2() -> Result<String> {
    Ok("part2".into())
}

fn polymer() -> Result<Vec<char>> {
    let file = File::open("data/day5_input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let chars = line.trim().chars().collect();
    Ok(chars)
}

fn is_reactive(c1: &char, c2: &char) -> bool {
    (*c1 as i8 - *c2 as i8).abs() == 32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_reactive() {
        assert_eq!(is_reactive(&'a', &'A'), true);
        assert_eq!(is_reactive(&'A', &'a'), true);
        assert_eq!(is_reactive(&'a', &'a'), false);
        assert_eq!(is_reactive(&'a', &'b'), false);
        assert_eq!(is_reactive(&'B', &'a'), false);
        assert_eq!(is_reactive(&'z', &'Z'), true);
    }
}
