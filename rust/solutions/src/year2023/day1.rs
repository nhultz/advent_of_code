use aoc_derive::{aoc, aoc_input};

#[aoc_input(day1)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.trim().to_string()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> u32 {
    input.into_iter().flat_map(|s| extract_numbers(s)).sum()
}

#[aoc(day1, part2)]
fn part2(input: &[String]) -> u32 {
    input
        .into_iter()
        .map(|s| convert_words_to_numbers(s))
        .flat_map(|s| extract_numbers(&s))
        .sum()
}

fn extract_numbers(s: &str) -> Result<u32, String> {
    let first_digit = s.chars().filter(|c| c.is_numeric()).next();
    let last_digit = s.chars().filter(|c| c.is_numeric()).last();

    let num = match (first_digit, last_digit) {
        (Some(n1), Some(n2)) => format!("{}{}", n1, n2),
        (_, _) => return Err("couldn't find two numbers in input string".to_string()),
    };

    num.parse()
        .map_err(|_| "unable to parse number from input string".to_string())
}

fn convert_words_to_numbers(s: &str) -> String {
    s.replace("one", "on1e")
        .replace("two", "tw2o")
        .replace("three", "thre3e")
        .replace("four", "fou4r")
        .replace("five", "fiv5e")
        .replace("six", "si6x")
        .replace("seven", "seve7n")
        .replace("eight", "eigh8t")
        .replace("nine", "nin9e")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    const INPUT_2: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn test_part1() {
        let entries = parse(INPUT_1);
        assert_eq!(part1(&entries), 142);
    }

    #[test]
    fn test_part2() {
        let entries = parse(INPUT_2);
        assert_eq!(part2(&entries), 281);
    }
}
