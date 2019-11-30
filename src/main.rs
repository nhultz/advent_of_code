use clap::{App, Arg};

use advent_solutions::days;
use advent_solutions::{AdventError, Result};

fn main() {
    let matches = App::new("AdventOfCode2018")
        .version("1.0.0")
        .author("Nick Hultz <nhultz328@gmail.com>")
        .about("Advent of Code 2018 Solutions")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .help("The challenge day to run."),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .takes_value(true)
                .help("The part to run."),
        )
        .get_matches();

    let day = matches.value_of("day");
    let part = matches.value_of("part");

    match run(day, part) {
        Ok(v) => println!("Answer: {}", v),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run(day: Option<&str>, part: Option<&str>) -> Result<String> {
    let day: u32 = day
        .ok_or(AdventError::MissingArgument("day".into()))?
        .parse()?;

    let part: u32 = part
        .ok_or(AdventError::MissingArgument("part".into()))?
        .parse()?;

    match (day, part) {
        (1, 1) => days::day1::part1(),
        (1, 2) => days::day1::part2(),
        (2, 1) => days::day2::part1(),
        (2, 2) => days::day2::part2(),
        (3, 1) => days::day3::part1(),
        (3, 2) => days::day3::part2(),
        (4, 1) => days::day4::part1(),
        (4, 2) => days::day4::part2(),
        (5, 1) => days::day5::part1(),
        (5, 2) => days::day5::part2(),
        (6, 1) => days::day6::part1(),
        (6, 2) => days::day6::part2(),
        (7, 1) => days::day7::part1(),
        (7, 2) => days::day7::part2(),
        (8, 1) => days::day8::part1(),
        (8, 2) => days::day8::part2(),
        (9, 1) => days::day9::part1(),
        (9, 2) => days::day9::part2(),
        (10, 1) => days::day10::part1(),
        (10, 2) => days::day10::part2(),
        (11, 1) => days::day11::part1(),
        (11, 2) => days::day11::part2(),
        (12, 1) => days::day12::part1(),
        (12, 2) => days::day12::part2(),
        (13, 1) => days::day13::part1(),
        (13, 2) => days::day13::part2(),
        (14, 1) => days::day14::part1(),
        (14, 2) => days::day14::part2(),
        (15, 1) => days::day15::part1(),
        (15, 2) => days::day15::part2(),
        (16, 1) => days::day16::part1(),
        (16, 2) => days::day16::part2(),
        (17, 1) => days::day17::part1(),
        (17, 2) => days::day17::part2(),
        (18, 1) => days::day18::part1(),
        (18, 2) => days::day18::part2(),
        (19, 1) => days::day19::part1(),
        (19, 2) => days::day19::part2(),
        (20, 1) => days::day20::part1(),
        (20, 2) => days::day20::part2(),
        (21, 1) => days::day21::part1(),
        (21, 2) => days::day21::part2(),
        (22, 1) => days::day22::part1(),
        (22, 2) => days::day22::part2(),
        (23, 1) => days::day23::part1(),
        (23, 2) => days::day23::part2(),
        (24, 1) => days::day24::part1(),
        (24, 2) => days::day24::part2(),
        (25, 1) => days::day25::part1(),
        (25, 2) => days::day25::part2(),
        (_, _) => Err(AdventError::NotImplemented(day, part)),
    }
}
