use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

use crate::{AdventError, Result};

pub fn part1() -> Result<String> {
    let file = File::open("data/2018/day2_input.txt")?;
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

    let checksum = num_two * num_three;
    Ok(checksum.to_string())
}

pub fn part2() -> Result<String> {
    let file = File::open("data/2018/day2_input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().flat_map(|line| line).collect();
    let lines_clone = lines.clone();

    let correct_boxes = lines
        .into_iter()
        .cartesian_product(lines_clone.into_iter())
        .filter(|product| check_one_char_difference(&product.0, &product.1))
        .nth(1)
        .ok_or(AdventError::UnexpectedError(
            "No matching boxes found.".into(),
        ))?;

    let mut answer = String::new();
    for (f, s) in correct_boxes.0.chars().zip(correct_boxes.1.chars()) {
        if f == s {
            answer.push(f);
        }
    }
    Ok(answer)
}

fn has_n_characters(n: u8, s: &str) -> bool {
    let mut characters = HashMap::new();

    for char in s.chars() {
        let cur_count = characters.entry(char).or_insert(0);
        *cur_count += 1;
    }

    characters.values().any(|val| *val == n)
}

fn check_one_char_difference(first: &str, second: &str) -> bool {
    let mut different_characters = 0;

    for (f, s) in first.chars().zip(second.chars()) {
        if f != s {
            different_characters += 1;
        }
    }

    // We only want a difference of 1 character
    different_characters == 1
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

    #[test]
    fn test_one_char_difference() {
        assert_eq!(check_one_char_difference("abcde", "fghij"), false);
        assert_eq!(check_one_char_difference("fghij", "fguij"), true);
        assert_eq!(check_one_char_difference("abcde", "axcye"), false);
    }
}
