use crate::{AdventError, Result};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn solve(day: u32, part: u32) -> Result<String> {
    match (day, part) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        (2, 1) => day2::part1(),
        (2, 2) => day2::part2(),
        (3, 1) => day3::part1(),
        (3, 2) => day3::part2(),
        (4, 1) => day4::part1(),
        (4, 2) => day4::part2(),
        (5, 1) => day5::part1(),
        (5, 2) => day5::part2(),
        (6, 1) => day6::part1(),
        (6, 2) => day6::part2(),
        (7, 1) => day7::part1(),
        (7, 2) => day7::part2(),
        (8, 1) => day8::part1(),
        (8, 2) => day8::part2(),
        (9, 1) => day9::part1(),
        (9, 2) => day9::part2(),
        (10, 1) => day10::part1(),
        (10, 2) => day10::part2(),
        (11, 1) => day11::part1(),
        (11, 2) => day11::part2(),
        (12, 1) => day12::part1(),
        (12, 2) => day12::part2(),
        (13, 1) => day13::part1(),
        (13, 2) => day13::part2(),
        (14, 1) => day14::part1(),
        (14, 2) => day14::part2(),
        (15, 1) => day15::part1(),
        (15, 2) => day15::part2(),
        (16, 1) => day16::part1(),
        (16, 2) => day16::part2(),
        (17, 1) => day17::part1(),
        (17, 2) => day17::part2(),
        (18, 1) => day18::part1(),
        (18, 2) => day18::part2(),
        (19, 1) => day19::part1(),
        (19, 2) => day19::part2(),
        (20, 1) => day20::part1(),
        (20, 2) => day20::part2(),
        (21, 1) => day21::part1(),
        (21, 2) => day21::part2(),
        (22, 1) => day22::part1(),
        (22, 2) => day22::part2(),
        (23, 1) => day23::part1(),
        (23, 2) => day23::part2(),
        (24, 1) => day24::part1(),
        (24, 2) => day24::part2(),
        (25, 1) => day25::part1(),
        (25, 2) => day25::part2(),
        _ => Err(AdventError::NotImplemented(2019, day, part))?,
    }
}
