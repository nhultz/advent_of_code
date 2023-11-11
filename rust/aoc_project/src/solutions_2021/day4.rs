use crate::{file_input, Result};
use anyhow::anyhow;

pub fn part1() -> Result<String> {
    let mut input = file_input("data/2021/day4_input.txt")?;
    solve_part1(&mut input)
}

pub fn part2() -> Result<String> {
    let mut input = file_input("data/2021/day4_input.txt")?;
    solve_part2(&mut input)
}

fn solve_part1<T>(nums: &mut T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let first_line = nums.next().ok_or(anyhow!("Can't get first line of nums"))?;

    let drawn_nums: Vec<u32> = first_line
        .split(",")
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut boards = vec![];

    let mut rows = vec![];
    while let Some(line) = nums.next() {
        match &line {
            l if l.is_empty() => {
                if !rows.is_empty() {
                    let board = Board::from_rows(rows)?;
                    boards.push(board);
                }
                rows = vec![];
            }
            l => rows.push(l.clone()),
        }
    }

    if !rows.is_empty() {
        let board = Board::from_rows(rows)?;
        boards.push(board);
    }

    for num in drawn_nums {
        for b in &mut boards {
            b.mark(num);
            if b.is_winner() {
                let sum = b.sum_unmarked();
                return Ok(format!("{}", num * sum));
            }
        }
    }

    Err(anyhow!("No winner found"))
}

fn solve_part2<T>(nums: &mut T) -> Result<String>
where
    T: Iterator<Item = String>,
{
    let first_line = nums.next().ok_or(anyhow!("Can't get first line of nums"))?;

    let drawn_nums: Vec<u32> = first_line
        .split(",")
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut boards = vec![];

    let mut rows = vec![];
    while let Some(line) = nums.next() {
        match &line {
            l if l.is_empty() => {
                if !rows.is_empty() {
                    let board = Board::from_rows(rows)?;
                    boards.push(board);
                }
                rows = vec![];
            }
            l => rows.push(l.clone()),
        }
    }

    if !rows.is_empty() {
        let board = Board::from_rows(rows)?;
        boards.push(board);
    }

    let mut num_winners = 0;
    let num_boards = boards.len();

    for num in drawn_nums {
        for b in &mut boards {
            if b.is_winner() {
                continue;
            }

            b.mark(num);

            if b.is_winner() {
                num_winners += 1;
            }

            if num_winners == num_boards {
                let sum = b.sum_unmarked();
                return Ok(format!("{}", num * sum));
            }
        }
    }

    Err(anyhow!("No winner found"))
}

#[derive(Debug, Clone)]
struct Space {
    value: u32,
    is_marked: bool,
}

impl Space {
    fn new(value: u32) -> Self {
        let is_marked = false;
        Space { value, is_marked }
    }
}

#[derive(Debug, Clone)]
struct Board {
    rows: Vec<Vec<Space>>,
}

impl Board {
    fn from_rows(raw_rows: Vec<String>) -> Result<Self> {
        let rows = raw_rows
            .iter()
            .map(|line| {
                let r: Vec<Space> = line
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .map(|n| Space::new(n))
                    .collect();
                r
            })
            .collect();

        Ok(Board { rows })
    }

    fn mark(&mut self, value: u32) {
        for row in &mut self.rows {
            for space in row {
                if space.value == value {
                    space.is_marked = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        for row in &self.rows {
            if row.iter().all(|p| p.is_marked) {
                return true;
            }
        }

        let mut num_marked = 0;
        for i in 0..self.rows[0].len() {
            for row in &self.rows {
                if row[i].is_marked {
                    num_marked += 1;
                }
            }

            if num_marked == self.rows.len() {
                return true;
            }
            num_marked = 0;
        }

        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.rows
            .iter()
            .map(|r| {
                r.iter().fold(0, |acc, p| {
                    acc + match p.is_marked {
                        true => 0,
                        false => p.value,
                    }
                })
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() -> Result<()> {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11 0",
            "8 2 23 4 24",
            "21 9 14 16 7",
            "6 10 3 18 5",
            "1 12 20 15 19",
            "",
            "3 15 0 2 22",
            "9 18 13 17 5",
            "19 8 7 25 23",
            "20 11 10 24 4",
            "14 21 16 12 6",
            "",
            "14 21 17 24 4",
            "10 16 15 9 19",
            "18 8 23 26 20",
            "22 11 13 6 5",
            "2 0 12 3 7",
        ];

        let mut i = input.iter().map(|line| line.to_string());

        let answer = solve_part1(&mut i)?;
        assert_eq!(answer, "4512");

        Ok(())
    }

    #[test]
    fn test_input_part2() -> Result<()> {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11 0",
            "8 2 23 4 24",
            "21 9 14 16 7",
            "6 10 3 18 5",
            "1 12 20 15 19",
            "",
            "3 15 0 2 22",
            "9 18 13 17 5",
            "19 8 7 25 23",
            "20 11 10 24 4",
            "14 21 16 12 6",
            "",
            "14 21 17 24 4",
            "10 16 15 9 19",
            "18 8 23 26 20",
            "22 11 13 6 5",
            "2 0 12 3 7",
        ];

        let mut i = input.iter().map(|line| line.to_string());

        let answer = solve_part2(&mut i)?;
        assert_eq!(answer, "1924");

        Ok(())
    }
}
