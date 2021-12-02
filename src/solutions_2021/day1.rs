use crate::{file_input, Result};

pub fn part1() -> Result<String> {
    let input = file_input("data/2021/day1_input.txt")?
        .map(|line| line.parse::<i64>())
        .flat_map(|line| line);

    solve_part1(input)
}

pub fn part2() -> Result<String> {
    let input = file_input("data/2021/day1_input.txt")?
        .map(|line| line.parse::<i64>())
        .flat_map(|line| line);

    solve_part2(input)
}

fn solve_part1<T>(nums: T) -> Result<String>
where
    T: Iterator<Item = i64>,
{
    let answer = nums
        .collect::<Vec<i64>>()
        .windows(2)
        .filter(|n| n[0] < n[1])
        .count();

    Ok(format!("{}", answer))
}

fn solve_part2<T>(nums: T) -> Result<String>
where
    T: Iterator<Item = i64>,
{
    let sums: Vec<i64> = nums
        .collect::<Vec<i64>>()
        .windows(3)
        .map(|n| n[0] + n[1] + n[2])
        .collect();

    let answer = sums.windows(2).filter(|n| n[0] < n[1]).count();
    Ok(format!("{}", answer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() -> Result<()> {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let answer = solve_part1(input.into_iter())?;

        assert_eq!(answer, "7");

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<()> {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let answer = solve_part2(input.into_iter())?;

        assert_eq!(answer, "5");

        Ok(())
    }
}
