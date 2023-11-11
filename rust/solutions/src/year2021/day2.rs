use aoc_derive::{aoc, aoc_input};
use std::num::ParseIntError;

#[aoc_input(day2)]
fn parse(input: &str) -> Result<Vec<(String, i64)>, ParseIntError> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();

            let direction = parts[0].to_string();
            let val = parts[1].parse();

            val.map(|n| (direction, n))
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(directions: &[(String, i64)]) -> i64 {
    let mut horizontal_dir = 0;
    let mut depth = 0;

    for (dir, amt) in directions {
        match dir.as_ref() {
            "forward" => horizontal_dir += amt,
            "up" => depth -= amt,
            "down" => depth += amt,
            _ => continue,
        }
    }

    horizontal_dir * depth
}

#[aoc(day2, part2)]
fn part2(directions: &[(String, i64)]) -> i64 {
    let mut horizontal_dir = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (dir, amt) in directions {
        match dir.as_ref() {
            "forward" => {
                horizontal_dir += amt;
                depth += aim * amt;
            }
            "up" => {
                aim -= amt;
            }
            "down" => aim += amt,
            _ => continue,
        }
    }

    horizontal_dir * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part1() {
        let entries = parse(INPUT).unwrap();
        assert_eq!(part1(&entries), 150);
    }

    #[test]
    fn test_part2() {
        let entries = parse(INPUT).unwrap();
        assert_eq!(part2(&entries), 900);
    }
}
