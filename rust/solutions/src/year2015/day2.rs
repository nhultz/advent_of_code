use aoc_derive::{aoc, aoc_input};

use nom::{
    bytes::complete::tag,
    character::complete::u32,
    sequence::{terminated, tuple},
    IResult,
};

use std::cmp;
use std::str::FromStr;

#[aoc_input(day2)]
fn parse(input: &str) -> Result<Vec<Dimension>, String> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Dimension]) -> u32 {
    input.iter().map(|d| d.wrapping_area()).sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Dimension]) -> u32 {
    input.iter().map(|d| d.ribbon_length()).sum()
}

#[derive(Debug)]
pub struct Dimension {
    length: u32,
    width: u32,
    height: u32,
}

impl Dimension {
    fn wrapping_area(&self) -> u32 {
        let x_area = self.length * self.width;
        let y_area = self.width * self.height;
        let z_area = self.height * self.length;

        let smallest_side = cmp::min(x_area, cmp::min(y_area, z_area));

        (2 * x_area + 2 * y_area + 2 * z_area) + smallest_side
    }

    fn ribbon_length(&self) -> u32 {
        let x_perim = (self.length * 2) + (self.width * 2);
        let y_perim = (self.width * 2) + (self.height * 2);
        let z_perim = (self.height * 2) + (self.length * 2);

        let smallest_perim = cmp::min(x_perim, cmp::min(y_perim, z_perim));

        smallest_perim + (self.length * self.height * self.width)
    }
}

impl FromStr for Dimension {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let mut parser = tuple((u32_with_comma, u32_with_comma, u32));
        let (_, (length, width, height)) =
            parser(val).map_err(|e| format!("unable to parse line: {}", e))?;

        Ok(Dimension {
            length,
            width,
            height,
        })
    }
}

fn u32_with_comma(input: &str) -> IResult<&str, u32> {
    terminated(u32, tag("x"))(input)
}
