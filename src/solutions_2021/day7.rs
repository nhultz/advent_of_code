use crate::{file_input, Result};
use anyhow::anyhow;

pub fn part1() -> Result<String> {
    let input = file_input("data/2021/day7_input.txt")?;
    solve_part1(input)
}

pub fn part2() -> Result<String> {
    let input = file_input("data/2021/day7_input.txt")?;
    solve_part2(input)
}

fn solve_part1<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let starting_positions: Vec<i64> = input
        .map(|line| {
            line.split(",")
                .filter_map(|val| val.parse().ok())
                .collect::<Vec<i64>>()
        })
        .flatten()
        .collect();

    let cheapest_fuel = calc_cheapest_fuel(&starting_positions, |dist| dist)?;
    Ok(format!("{:?}", cheapest_fuel))
}

fn solve_part2<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let starting_positions: Vec<i64> = input
        .map(|line| {
            line.split(",")
                .filter_map(|val| val.parse().ok())
                .collect::<Vec<i64>>()
        })
        .flatten()
        .collect();

    let cheapest_fuel = calc_cheapest_fuel(&starting_positions, |dist| (dist * (dist + 1)) / 2)?;
    Ok(format!("{:?}", cheapest_fuel))
}

fn calc_cheapest_fuel(starting_positions: &[i64], fuel_fn: fn(i64) -> i64) -> Result<i64> {
    let min_pos = starting_positions
        .iter()
        .min()
        .ok_or(anyhow!("No min found"))?
        .clone();

    let max_pos = starting_positions
        .iter()
        .max()
        .ok_or(anyhow!("No max found"))?
        .clone();

    (min_pos..=max_pos)
        .map(|target_pos| {
            starting_positions
                .iter()
                .map(|p| {
                    let distance = (p - target_pos).abs();
                    fuel_fn(distance)
                })
                .sum()
        })
        .min()
        .ok_or(anyhow!("No answer found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() -> Result<()> {
        let input = vec!["16,1,2,0,4,2,7,1,2,14"];
        let answer = solve_part1(input.iter().map(|line| line.to_string()))?;

        assert_eq!(answer, "37");

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<()> {
        let input = vec!["16,1,2,0,4,2,7,1,2,14"];
        let answer = solve_part2(input.iter().map(|line| line.to_string()))?;

        assert_eq!(answer, "168");

        Ok(())
    }
}
