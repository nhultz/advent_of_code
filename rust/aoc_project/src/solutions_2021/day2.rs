use crate::{file_input, Result};

pub fn part1() -> Result<String> {
    let input = file_input("data/2021/day2_input.txt")?;
    solve_part1(input)
}

pub fn part2() -> Result<String> {
    let input = file_input("data/2021/day2_input.txt")?;
    solve_part2(input)
}

fn solve_part1<T>(directions: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let d: Vec<(String, i64)> = directions
        .map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            (parts[0].to_string(), parts[1].parse::<i64>().unwrap())
        })
        .collect();

    let mut horizontal_dir = 0;
    let mut depth = 0;

    for (dir, amt) in d {
        match dir.as_ref() {
            "forward" => horizontal_dir += amt,
            "up" => depth -= amt,
            "down" => depth += amt,
            _ => continue,
        }
    }

    Ok(format!("{}", horizontal_dir * depth))
}

fn solve_part2<T>(directions: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let d: Vec<(String, i64)> = directions
        .map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            (parts[0].to_string(), parts[1].parse::<i64>().unwrap())
        })
        .collect();

    let mut horizontal_dir = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (dir, amt) in d {
        match dir.as_ref() {
            "forward" => {
                horizontal_dir += amt;
                depth += aim * amt;
            }
            "up" => {
                aim -= amt;
            }
            "down" => aim += amt,
            _ => continue,
        }
    }

    Ok(format!("{}", horizontal_dir * depth))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() -> Result<()> {
        let input = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let answer = solve_part1(input.into_iter())?;

        assert_eq!(answer, "150");

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<()> {
        let input = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let answer = solve_part2(input.into_iter())?;

        assert_eq!(answer, "900");

        Ok(())
    }
}
