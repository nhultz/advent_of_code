use crate::{file_input, Result};

pub fn part1() -> Result<String> {
    let input = file_input("data/2015/day1_input.txt")?;
    solve_part1(input)
}

pub fn part2() -> Result<String> {
    let input = file_input("data/2015/day1_input.txt")?;
    solve_part2(input)
}

fn solve_part1<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let directions: Vec<char> = input
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect();

    let mut floor = 0;
    for c in directions {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
    }

    Ok(format!("{}", floor))
}

fn solve_part2<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let directions: Vec<char> = input
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect();

    let mut floor = 0;
    let mut basement_pos = 0;

    for (idx, c) in directions.iter().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }

        if floor == -1 {
            basement_pos = idx + 1;
            break;
        }
    }

    Ok(format!("{}", basement_pos))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() -> Result<()> {
        let input = vec!["(()(()("];
        let answer = solve_part1(input.iter().map(|line| line.to_string()))?;

        assert_eq!(answer, "3");

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<()> {
        let input = vec!["()())"];
        let answer = solve_part2(input.iter().map(|line| line.to_string()))?;

        assert_eq!(answer, "5");

        Ok(())
    }
}
