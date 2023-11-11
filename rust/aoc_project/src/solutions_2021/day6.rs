use crate::{file_input, Result};

pub fn part1() -> Result<String> {
    let input = file_input("data/2021/day6_input.txt")?;
    solve_part1(input)
}

pub fn part2() -> Result<String> {
    let input = file_input("data/2021/day6_input.txt")?;
    solve_part2(input)
}

fn solve_part1<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let initial_fishes = input
        .map(|line| {
            line.split(",")
                .filter_map(|val| val.parse::<u8>().ok())
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    let mut population = Population::new(initial_fishes);
    for _ in 1..=80 {
        population.next_generation();
    }

    Ok(format!("{}", population.size()))
}

fn solve_part2<T>(input: T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let initial_fishes = input
        .map(|line| {
            line.split(",")
                .filter_map(|val| val.parse::<u8>().ok())
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    let mut population = Population::new(initial_fishes);
    for _ in 1..=256 {
        population.next_generation();
    }

    Ok(format!("{}", population.size()))
}

#[derive(Debug)]
struct Population {
    pop_counts: [u64; 9],
}

impl Population {
    fn new(initial_pop: Vec<u8>) -> Self {
        let mut pop_counts = [0; 9];

        for f in initial_pop {
            pop_counts[f as usize] += 1;
        }

        Population { pop_counts }
    }

    fn size(&self) -> u64 {
        self.pop_counts.iter().sum()
    }

    fn next_generation(&mut self) {
        let mut new_counts = [0; 9];

        for i in 0..=8 {
            if i == 0 {
                new_counts[6] = new_counts[6] + self.pop_counts[i];
                new_counts[8] = new_counts[8] + self.pop_counts[i];
            } else {
                new_counts[i - 1] = new_counts[i - 1] + self.pop_counts[i];
            }
        }

        self.pop_counts = new_counts;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() -> Result<()> {
        let input = vec!["3,4,3,1,2"];
        let answer = solve_part1(input.iter().map(|line| line.to_string()))?;

        assert_eq!(answer, "5934");

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<()> {
        let input = vec!["3,4,3,1,2"];
        let answer = solve_part2(input.iter().map(|line| line.to_string()))?;

        assert_eq!(answer, "26984457539");

        Ok(())
    }
}
