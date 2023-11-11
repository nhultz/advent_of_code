use aoc_derive::{aoc, aoc_input};
use std::collections::HashMap;

#[aoc_input(day10)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[String]) -> u32 {
    input
        .into_iter()
        .map(|l| Line { val: l.to_string() })
        .filter_map(|l| l.is_corrupt())
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &[String]) -> u64 {
    let mut scores: Vec<u64> = input
        .into_iter()
        .map(|l| Line { val: l.to_string() })
        .filter(|l| match l.is_corrupt() {
            Some(_) => false,
            None => true,
        })
        .map(|l| l.find_missing_sequence())
        .map(|seq| {
            seq.chars().fold(0, |accum, item| {
                let res = accum * 5;
                match item {
                    ')' => res + 1,
                    ']' => res + 2,
                    '}' => res + 3,
                    '>' => res + 4,
                    _ => res,
                }
            })
        })
        .collect();

    scores.sort();

    // return middle score
    scores[scores.len() / 2]
}

#[derive(Debug)]
struct Line {
    val: String,
}

impl Line {
    fn is_corrupt(&self) -> Option<u32> {
        let pairs: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
            .iter()
            .cloned()
            .collect();

        let mut stack = Vec::new();
        let mut points = None;

        for ch in self.val.chars() {
            if pairs.contains_key(&ch) {
                stack.push(ch);
                continue;
            }

            if let Some(prev_open) = stack.pop() {
                if let Some(expected_close) = pairs.get(&prev_open) {
                    if ch != *expected_close {
                        match ch {
                            ')' => points = Some(3),
                            ']' => points = Some(57),
                            '}' => points = Some(1197),
                            '>' => points = Some(25137),
                            _ => continue,
                        }
                    }
                }
            }
        }

        points
    }

    fn find_missing_sequence(&self) -> String {
        let pairs: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
            .iter()
            .cloned()
            .collect();

        let mut stack = Vec::new();

        for ch in self.val.chars() {
            if pairs.contains_key(&ch) {
                stack.push(ch);
                continue;
            }

            stack.pop();
        }

        let mut missing_seq = String::new();
        while let Some(open_char) = stack.pop() {
            if let Some(expected_close) = pairs.get(&open_char) {
                missing_seq.push(*expected_close);
            }
        }

        missing_seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        let entries = parse(INPUT);
        assert_eq!(part1(&entries), 26397);
    }

    #[test]
    fn test_part2() {
        let entries = parse(INPUT);
        assert_eq!(part2(&entries), 288957);
    }
}
