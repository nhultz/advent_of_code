use crate::{file_input, Result};
use std::collections::HashMap;

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
    pop_counts: HashMap<u8, u64>,
}

impl Population {
    fn new(initial_pop: Vec<u8>) -> Self {
        let mut counter: HashMap<u8, u64> = HashMap::default();
        for f in initial_pop {
            let count = counter.entry(f).or_insert(0);
            *count += 1;
        }

        Population {
            pop_counts: counter,
        }
    }

    fn size(&self) -> u64 {
        self.pop_counts.values().sum()
    }

    fn next_generation(&mut self) {
        let mut counter: HashMap<u8, u64> = HashMap::default();

        for (k, count) in &self.pop_counts {
            if *k == 0 {
                let cur_6 = counter.entry(6).or_insert(0);
                *cur_6 += *count;

                let cur_8 = counter.entry(8).or_insert(0);
                *cur_8 += *count;
            } else {
                let cur_k = counter.entry(k - 1).or_insert(0);
                *cur_k += *count;
            }
        }

        self.pop_counts = counter;
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
