use aoc_derive::{aoc, aoc_input};

use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::map_res,
    sequence::{separated_pair, tuple},
    IResult,
};

#[aoc_input(day6)]
fn parse(input: &str) -> Result<Vec<Instruction>, String> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Instruction]) -> u64 {
    let mut lights: [[u8; 1000]; 1000] = [[0; 1000]; 1000];

    for inst in input {
        for r in inst.row() {
            for c in inst.col() {
                match inst.cmd {
                    Command::On => lights[r][c] = 1,
                    Command::Off => lights[r][c] = 0,
                    Command::Toggle => lights[r][c] ^= 1, // XOR
                }
            }
        }
    }

    lights
        .iter()
        .map(|row| row.iter().map(|n| *n as u64).sum::<u64>())
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &[Instruction]) -> u64 {
    let mut lights: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

    for inst in input {
        for r in inst.row() {
            for c in inst.col() {
                match inst.cmd {
                    Command::On => lights[r][c] += 1,
                    Command::Off => lights[r][c] = (lights[r][c] - 1).max(0),
                    Command::Toggle => lights[r][c] += 2,
                }
            }
        }
    }

    lights
        .iter()
        .map(|row| row.iter().map(|n| *n as u64).sum::<u64>())
        .sum()
}

#[derive(Debug)]
enum Command {
    On,
    Off,
    Toggle,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "turn on" => Ok(Command::On),
            "turn off" => Ok(Command::Off),
            "toggle" => Ok(Command::Toggle),
            _ => Err(format!("Can't parse Command: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    cmd: Command,
    top_left_corner: (usize, usize),
    bottom_right_corner: (usize, usize),
}

impl Instruction {
    fn row(&self) -> std::ops::RangeInclusive<usize> {
        self.top_left_corner.0..=self.bottom_right_corner.0
    }

    fn col(&self) -> std::ops::RangeInclusive<usize> {
        self.top_left_corner.1..=self.bottom_right_corner.1
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let mut parser = tuple((command, coord, tag(" through "), coord));
        let (_, (cmd, top_corner, _, bottom_corner)) =
            parser(val).map_err(|e| format!("unable to parse line: {}", e))?;

        Ok(Instruction {
            cmd,
            top_left_corner: (top_corner.0 as usize, top_corner.1 as usize),
            bottom_right_corner: (bottom_corner.0 as usize, bottom_corner.1 as usize),
        })
    }
}

fn command(input: &str) -> IResult<&str, Command> {
    map_res(
        alt((tag("turn on "), tag("turn off "), tag("toggle "))),
        |s: &str| s.parse::<Command>(),
    )(input)
}

fn coord(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag(","), u32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&vec![Instruction {
                cmd: Command::On,
                top_left_corner: (0, 0),
                bottom_right_corner: (999, 999),
            }]),
            1000000
        );
        assert_eq!(
            part1(&vec![Instruction {
                cmd: Command::Toggle,
                top_left_corner: (0, 0),
                bottom_right_corner: (999, 0),
            }]),
            1000
        );
        assert_eq!(
            part1(&vec![Instruction {
                cmd: Command::Off,
                top_left_corner: (499, 499),
                bottom_right_corner: (500, 500),
            }]),
            0
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&vec![Instruction {
                cmd: Command::On,
                top_left_corner: (0, 0),
                bottom_right_corner: (0, 0),
            }]),
            1
        );
        assert_eq!(
            part2(&vec![Instruction {
                cmd: Command::Toggle,
                top_left_corner: (0, 0),
                bottom_right_corner: (999, 999),
            }]),
            2000000
        );
    }
}
