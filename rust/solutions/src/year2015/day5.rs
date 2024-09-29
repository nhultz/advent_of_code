use aoc_derive::{aoc, aoc_input};

use std::collections::{HashMap, HashSet};

#[aoc_input(day5)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day5, part1)]
fn part1(input: &[String]) -> usize {
    input.into_iter().filter(|s| is_nice_str(s)).count()
}

#[aoc(day5, part2)]
fn part2(input: &[String]) -> usize {
    input.into_iter().filter(|s| is_better_nice_str(s)).count()
}

fn is_nice_str(s: &str) -> bool {
    let vowel_count = s.chars().filter(|c| is_vowel(c)).count();
    let has_double_letter = s.as_bytes().windows(2).any(|chunk| chunk[0] == chunk[1]);
    let contains_bad = is_bad(s);

    vowel_count >= 3 && has_double_letter && !contains_bad
}

fn is_better_nice_str(s: &str) -> bool {
    let mut all_pairs: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut i = 0;
    let mut j = 1;

    while j < s.len() {
        let pair = s[i..=j].to_string();
        let entry = all_pairs.entry(pair).or_insert(HashSet::new());
        entry.insert(i);
        entry.insert(j);
        i += 1;
        j += 1;
    }

    let double_no_overlap = all_pairs.values().any(|p| p.len() >= 4);

    let same_with_middle = s.as_bytes().windows(3).any(|chunk| chunk[0] == chunk[2]);

    double_no_overlap && same_with_middle
}

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn is_bad(s: &str) -> bool {
    s.contains("ab") | s.contains("cd") | s.contains("pq") | s.contains("xy")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(is_nice_str("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_str("aaa"), true);
        assert_eq!(is_nice_str("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_str("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_str("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(is_better_nice_str("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_better_nice_str("xxyxx"), true);
        assert_eq!(is_better_nice_str("uurcxstgmygtbstg"), false);
        assert_eq!(is_better_nice_str("ieodomkazucvgmuy"), false);
    }
}
