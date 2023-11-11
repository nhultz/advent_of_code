use aoc_derive::{aoc, aoc_input};
use std::cmp::Ordering;
use std::num::ParseIntError;

#[aoc_input(day3)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[String]) -> Result<u64, ParseIntError> {
    let size = input[0].len();

    let mut gamma_bits = String::new();

    for i in 0..size {
        let common_bit = most_common_bit_in_pos(input, i);
        match common_bit {
            CommonBit::One => gamma_bits.push_str("1"),
            CommonBit::Zero => gamma_bits.push_str("0"),
            _ => continue,
        }
    }

    let gamma = u64::from_str_radix(&gamma_bits, 2)?;
    let epsilon = u64::from_str_radix(&flip_bits(&gamma_bits), 2)?;

    Ok(gamma * epsilon)
}

#[aoc(day3, part2)]
fn part2(input: &[String]) -> Result<i64, ParseIntError> {
    let mut oxy_bits: Vec<String> = input.to_vec();
    let mut co2_bits: Vec<String> = oxy_bits.clone();

    let mut cur_pos = 0;
    let size = oxy_bits[0].len();

    while oxy_bits.len() > 1 && cur_pos < size {
        let common_bit = most_common_bit_in_pos(&oxy_bits, cur_pos);
        oxy_bits = oxy_bits
            .into_iter()
            .filter(
                |bit_pattern| match (&common_bit, bit_pattern.chars().nth(cur_pos)) {
                    (CommonBit::One, Some('1')) => true,
                    (CommonBit::Zero, Some('0')) => true,
                    (CommonBit::Equal, Some('1')) => true,
                    _ => false,
                },
            )
            .collect();
        cur_pos += 1;
    }

    cur_pos = 0;
    while co2_bits.len() > 1 && cur_pos < size {
        let common_bit = most_common_bit_in_pos(&co2_bits, cur_pos);
        co2_bits = co2_bits
            .into_iter()
            .filter(
                |bit_pattern| match (&common_bit, bit_pattern.chars().nth(cur_pos)) {
                    (CommonBit::One, Some('0')) => true,
                    (CommonBit::Zero, Some('1')) => true,
                    (CommonBit::Equal, Some('0')) => true,
                    _ => false,
                },
            )
            .collect();

        cur_pos += 1;
    }

    let oxy_rating = i64::from_str_radix(&oxy_bits[0], 2)?;
    let co2_rating = i64::from_str_radix(&co2_bits[0], 2)?;

    Ok(oxy_rating * co2_rating)
}

fn flip_bits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '1' => "0",
            '0' => "1",
            _ => "",
        })
        .collect()
}

enum CommonBit {
    One,
    Zero,
    Equal,
}

fn most_common_bit_in_pos(t: &[String], pos: usize) -> CommonBit {
    let mut counter: usize = 0;

    for bit_pattern in t {
        let current_bit = &bit_pattern[pos..=pos];
        match current_bit {
            "1" => counter += 1,
            _ => continue,
        }
    }

    match (counter * 2).cmp(&t.len()) {
        Ordering::Equal => CommonBit::Equal,
        Ordering::Less => CommonBit::Zero,
        Ordering::Greater => CommonBit::One,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        let entries = parse(INPUT);
        assert_eq!(part1(&entries).unwrap(), 198);
    }

    #[test]
    fn test_part2() {
        let entries = parse(INPUT);
        assert_eq!(part2(&entries).unwrap(), 230);
    }
}
