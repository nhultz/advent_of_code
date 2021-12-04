use crate::{AdventError, DayPart, SolveFn, SolveResult};
use lazy_static::lazy_static;
use std::collections::HashMap;

mod day1;
mod day2;
mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;

// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;

// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

lazy_static! {
    static ref ALL_2021_PROBLEMS: HashMap<DayPart, SolveFn> = {
        let mut m = HashMap::new();
        m.insert(DayPart::new(1, 1), day1::part1 as SolveFn);
        m.insert(DayPart::new(1, 2), day1::part2);
        m.insert(DayPart::new(2, 1), day2::part1);
        m.insert(DayPart::new(2, 2), day2::part2);
        m.insert(DayPart::new(3, 1), day3::part1);
        m.insert(DayPart::new(3, 2), day3::part2);
        m
    };
}

pub fn solve_all() -> Vec<SolveResult<String>> {
    (1..=25).map(|day| solve_day(day)).flatten().collect()
}

pub fn solve_day(day: u32) -> Vec<SolveResult<String>> {
    (1..=2).map(|part| solve_part(day, part)).collect()
}

pub fn solve_part(day: u32, part: u32) -> SolveResult<String> {
    let day_part = DayPart::new(day, part);
    let res = match ALL_2021_PROBLEMS.get(&day_part) {
        Some(f) => f(),
        None => Err(AdventError::NotImplemented(2021, day, part)).map_err(anyhow::Error::msg),
    };

    SolveResult::new(day, part, res)
}
