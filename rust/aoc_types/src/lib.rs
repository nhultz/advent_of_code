use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use void::Void;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl Day {
    pub fn as_u8(self) -> u8 {
        <Self as Into<u8>>::into(self)
    }
}

impl FromStr for Day {
    type Err = &'static str;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(match val {
            "1" | "day1" => Day::Day1,
            "2" | "day2" => Day::Day2,
            "3" | "day3" => Day::Day3,
            "4" | "day4" => Day::Day4,
            "5" | "day5" => Day::Day5,
            "6" | "day6" => Day::Day6,
            "7" | "day7" => Day::Day7,
            "8" | "day8" => Day::Day8,
            "9" | "day9" => Day::Day9,
            "10" | "day10" => Day::Day10,
            "11" | "day11" => Day::Day11,
            "12" | "day12" => Day::Day12,
            "13" | "day13" => Day::Day13,
            "14" | "day14" => Day::Day14,
            "15" | "day15" => Day::Day15,
            "16" | "day16" => Day::Day16,
            "17" | "day17" => Day::Day17,
            "18" | "day18" => Day::Day18,
            "19" | "day19" => Day::Day19,
            "20" | "day20" => Day::Day20,
            "21" | "day21" => Day::Day21,
            "22" | "day22" => Day::Day22,
            "23" | "day23" => Day::Day23,
            "24" | "day24" => Day::Day24,
            "25" | "day25" => Day::Day25,
            _ => return Err("Failed to parse day, allowed pattern: dayX with X in 1..=25"),
        })
    }
}

impl TryFrom<u8> for Day {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        Ok(match val {
            1 => Day::Day1,
            2 => Day::Day2,
            3 => Day::Day3,
            4 => Day::Day4,
            5 => Day::Day5,
            6 => Day::Day6,
            7 => Day::Day7,
            8 => Day::Day8,
            9 => Day::Day9,
            10 => Day::Day10,
            11 => Day::Day11,
            12 => Day::Day12,
            13 => Day::Day13,
            14 => Day::Day14,
            15 => Day::Day15,
            16 => Day::Day16,
            17 => Day::Day17,
            18 => Day::Day18,
            19 => Day::Day19,
            20 => Day::Day20,
            21 => Day::Day21,
            22 => Day::Day22,
            23 => Day::Day23,
            24 => Day::Day24,
            25 => Day::Day25,
            _ => return Err("Day must be in range 1..=25"),
        })
    }
}

impl Into<u8> for Day {
    fn into(self) -> u8 {
        match self {
            Day::Day1 => 1,
            Day::Day2 => 2,
            Day::Day3 => 3,
            Day::Day4 => 4,
            Day::Day5 => 5,
            Day::Day6 => 6,
            Day::Day7 => 7,
            Day::Day8 => 8,
            Day::Day9 => 9,
            Day::Day10 => 10,
            Day::Day11 => 11,
            Day::Day12 => 12,
            Day::Day13 => 13,
            Day::Day14 => 14,
            Day::Day15 => 15,
            Day::Day16 => 16,
            Day::Day17 => 17,
            Day::Day18 => 18,
            Day::Day19 => 19,
            Day::Day20 => 20,
            Day::Day21 => 21,
            Day::Day22 => 22,
            Day::Day23 => 23,
            Day::Day24 => 24,
            Day::Day25 => 25,
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "day{}", self.as_u8())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Part {
    Part1,
    Part2,
}

impl Part {
    pub fn as_u8(self) -> u8 {
        <Self as Into<u8>>::into(self)
    }
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(match val {
            "1" | "part1" => Part::Part1,
            "2" | "part2" => Part::Part2,
            _ => return Err("Failed to parse part, allowed pattern: partX with X in 1..=2"),
        })
    }
}

impl TryFrom<u8> for Part {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        Ok(match val {
            1 => Part::Part1,
            2 => Part::Part2,
            _ => return Err("Part must be in range 1..=2"),
        })
    }
}

impl Into<u8> for Part {
    fn into(self) -> u8 {
        match self {
            Part::Part1 => 1,
            Part::Part2 => 2,
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "part{}", self.as_u8())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct DayPart {
    pub day: Day,
    pub part: Part,
}

impl PartialOrd for DayPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for DayPart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.day.cmp(&other.day).then(self.part.cmp(&other.part))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ReturnWrapperType {
    Result,
    Option,
}

pub trait InputReader<'a> {
    type Output;

    fn read(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>>;

    fn is_default(&self) -> bool {
        false
    }
}

pub trait InputReaderDefault {}

impl<'a, T> InputReader<'a> for &T
where
    T: InputReaderDefault,
{
    type Output = &'a str;

    fn read(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
        Ok(input)
    }

    fn is_default(&self) -> bool {
        true
    }
}

pub trait Runner<'a, I> {
    type Output;

    fn run(&self, input: I) -> Result<Self::Output, Box<dyn Error>>;

    fn is_implemented(&self) -> bool {
        true
    }
}

pub trait RunnerDefault {
    type Input;
}

impl<'a, I, T> Runner<'a, I> for &T
where
    T: RunnerDefault<Input = I>,
{
    type Output = Void;

    fn run(&self, _input: I) -> Result<Self::Output, Box<dyn Error>> {
        Err(Box::new(NotImplemented))
    }

    fn is_implemented(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct NotImplemented;

impl Display for NotImplemented {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Not Implemented")
    }
}

impl Error for NotImplemented {}

#[derive(Debug)]
pub struct InputReaderFailed;

impl Display for InputReaderFailed {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Input Reader Failed")
    }
}

impl Error for InputReaderFailed {}
