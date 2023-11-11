use aoc_derive::{aoc, aoc_input};

#[aoc_input(day1)]
fn parse(input: &str) -> Vec<char> {
    input.lines().map(|l| l.chars()).flatten().collect()
}

#[aoc(day1, part1)]
fn part1(input: &[char]) -> i32 {
    let mut floor = 0;
    for c in input {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
    }
    floor
}

#[aoc(day1, part2)]
fn part2(input: &[char]) -> usize {
    let mut floor = 0;
    let mut basement_pos = 0;

    for (idx, c) in input.iter().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }

        if floor == -1 {
            basement_pos = idx + 1;
            break;
        }
    }

    basement_pos
}
