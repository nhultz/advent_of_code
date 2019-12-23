use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow;
use thiserror::Error;

pub mod solutions_2018;
pub mod solutions_2019;

pub type Result<T> = anyhow::Result<T>;

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("Missing command-line argument: {0}")]
    MissingArgument(String),

    #[error("Year: {0}, Day: {1}, Part: {2} not implemented yet.")]
    NotImplemented(u32, u32, u32),
}

fn file_input(file_path: &str) -> Result<impl Iterator<Item = String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().flat_map(|line| line))
}
