use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow;
use thiserror::Error;

pub mod solutions_2015;
pub mod solutions_2018;
pub mod solutions_2019;
pub mod solutions_2020;
pub mod solutions_2021;

pub type Result<T> = anyhow::Result<T>;

type SolveFn = fn() -> Result<String>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct DayPart {
    day: u32,
    part: u32,
}

impl DayPart {
    fn new(day: u32, part: u32) -> Self {
        DayPart { day, part }
    }
}

pub struct SolveResult<T> {
    day: u32,
    part: u32,
    answer: Result<T>,
}

impl<T> SolveResult<T> {
    fn new(day: u32, part: u32, answer: Result<T>) -> Self {
        SolveResult { day, part, answer }
    }
}

impl<T> fmt::Display for SolveResult<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.answer {
            Ok(ans) => write!(
                f,
                "Day {} - Part {}\n==> Answer: {}",
                self.day, self.part, ans
            ),
            Err(err) => write!(
                f,
                "Day {} - Part {}\n==> Error: {}",
                self.day, self.part, err
            ),
        }
    }
}

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("Missing command-line argument: {0}")]
    MissingArgument(String),

    #[error("Year: {0} not implemented yet.")]
    NotImplementedYear(u32),

    #[error("Year: {0}, Day: {1} not implemented yet.")]
    NotImplementedDay(u32, u32),

    #[error("Year: {0}, Day: {1}, Part: {2} not implemented yet.")]
    NotImplemented(u32, u32, u32),
}

fn file_input(file_path: &str) -> Result<impl Iterator<Item = String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let it = reader
        .lines()
        .flat_map(|line| line)
        .map(|line| line.trim().to_string());

    Ok(it)
}
