use crate::{file_input, Result};
use nom::{
    bytes::complete::tag,
    character::complete::u32,
    sequence::{terminated, tuple},
    IResult,
};
use std::cmp;

pub fn part1() -> Result<String> {
    let input = file_input("data/2015/day2_input.txt")?;
    solve_part1(input)
}

pub fn part2() -> Result<String> {
    let input = file_input("data/2015/day2_input.txt")?;
    solve_part2(input)
}

fn solve_part1<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let dims: Vec<Dimension> = input.map(|line| parse_dimension(&line)).collect();
    let total_area: u32 = dims.iter().map(|d| d.wrapping_area()).sum();

    Ok(format!("{}", total_area))
}

fn solve_part2<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let dims: Vec<Dimension> = input.map(|line| parse_dimension(&line)).collect();
    let total_length: u32 = dims.iter().map(|d| d.ribbon_length()).sum();

    Ok(format!("{}", total_length))
}

fn u32_with_comma(input: &str) -> IResult<&str, u32> {
    terminated(u32, tag("x"))(input)
}

fn parse_dimension(input: &str) -> Dimension {
    let (_, (length, width, height)) =
        tuple((u32_with_comma, u32_with_comma, u32))(input).expect("Got bad input");

    Dimension {
        length,
        width,
        height,
    }
}

#[derive(Debug)]
struct Dimension {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() -> Result<()> {
        let input = vec!["2x3x4", "1x1x10"];
        let answer = solve_part1(input.iter().map(|line| line.to_string()))?;

        assert_eq!(answer, "101");

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<()> {
        let input = vec!["2x3x4", "1x1x10"];
        let answer = solve_part2(input.iter().map(|line| line.to_string()))?;

        assert_eq!(answer, "48");

        Ok(())
    }
}
