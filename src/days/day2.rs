use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

use crate::Result;

pub fn part1() -> Result<i64> {
    let file = File::open("data/day2_input.txt")?;
    let reader = BufReader::new(file);

    let mut num_two = 0;
    let mut num_three = 0;

    for line in reader.lines() {
        let line = line?;
        if has_n_characters(2, &line) {
            num_two += 1;
        }

        if has_n_characters(3, &line) {
            num_three += 1;
        }
    }

    Ok(num_two * num_three)
}

fn has_n_characters(n: u8, s: &str) -> bool {
    let mut characters = HashMap::new();

    for char in s.chars() {
        let cur_count = characters.entry(char).or_insert(0);
        *cur_count += 1;
    }

    characters.values().any(|val| *val == n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two() {
        assert_eq!(has_n_characters(2, "abcdef"), false);
        assert_eq!(has_n_characters(2, "bababc"), true);
        assert_eq!(has_n_characters(2, "abbcde"), true);
        assert_eq!(has_n_characters(2, "abcccd"), false);
        assert_eq!(has_n_characters(2, "aabcdd"), true);
        assert_eq!(has_n_characters(2, "abcdee"), true);
        assert_eq!(has_n_characters(2, "ababab"), false);
    }

    #[test]
    fn test_three() {
        assert_eq!(has_n_characters(3, "abcdef"), false);
        assert_eq!(has_n_characters(3, "bababc"), true);
        assert_eq!(has_n_characters(3, "abbcde"), false);
        assert_eq!(has_n_characters(3, "abcccd"), true);
        assert_eq!(has_n_characters(3, "aabcdd"), false);
        assert_eq!(has_n_characters(3, "abcdee"), false);
        assert_eq!(has_n_characters(3, "ababab"), true);
    }
}
