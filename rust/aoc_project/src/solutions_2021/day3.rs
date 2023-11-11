use crate::{file_input, Result};
use std::cmp::Ordering;

pub fn part1() -> Result<String> {
    let input = file_input("data/2021/day3_input.txt")?;
    solve_part1(input)
}

pub fn part2() -> Result<String> {
    let input = file_input("data/2021/day3_input.txt")?;
    solve_part2(input)
}

fn solve_part1<T>(nums: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let bits: Vec<String> = nums.collect();
    let size = bits[0].len();

    let mut gamma_bits = String::new();

    for i in 0..size {
        let common_bit = most_common_bit_in_pos(&bits, i);
        match common_bit {
            CommonBit::One => gamma_bits.push_str("1"),
            CommonBit::Zero => gamma_bits.push_str("0"),
            _ => continue,
        }
    }

    let gamma = u64::from_str_radix(&gamma_bits, 2)?;
    let epsilon = u64::from_str_radix(&flip_bits(&gamma_bits), 2)?;

    Ok(format!("{}", gamma * epsilon))
}

fn solve_part2<T>(nums: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let mut oxy_bits: Vec<String> = nums.collect();
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

    Ok(format!("{}", oxy_rating * co2_rating))
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

fn most_common_bit_in_pos(t: &Vec<String>, pos: usize) -> CommonBit {
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

    #[test]
    fn test_input_part1() -> Result<()> {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let answer = solve_part1(input.into_iter())?;
        assert_eq!(answer, "198");

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<()> {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let answer = solve_part2(input.into_iter())?;
        assert_eq!(answer, "230");

        Ok(())
    }
}
