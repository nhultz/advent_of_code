use clap::{App, Arg};

use advent_of_code_solutions::{solutions_2018, solutions_2019};
use advent_of_code_solutions::{AdventError, Result};

fn main() {
    let matches = App::new("AdventOfCode")
        .version("1.0.0")
        .author("Nick Hultz <nhultz328@gmail.com>")
        .about("Advent of Code Solutions")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .takes_value(true)
                .help("Which year of solutions to run."),
        )
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

    let year = matches.value_of("year");
    let day = matches.value_of("day");
    let part = matches.value_of("part");

    match run(year, day, part) {
        Ok(v) => println!("Answer: {}", v),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run(year: Option<&str>, day: Option<&str>, part: Option<&str>) -> Result<String> {
    let year: u32 = year
        .ok_or(AdventError::MissingArgument("year".into()))?
        .parse()?;

    let day: u32 = day
        .ok_or(AdventError::MissingArgument("day".into()))?
        .parse()?;

    let part: u32 = part
        .ok_or(AdventError::MissingArgument("part".into()))?
        .parse()?;

    match year {
        2018 => match (day, part) {
            (1, 1) => solutions_2018::day1::part1(),
            (1, 2) => solutions_2018::day1::part2(),
            (2, 1) => solutions_2018::day2::part1(),
            (2, 2) => solutions_2018::day2::part2(),
            (3, 1) => solutions_2018::day3::part1(),
            (3, 2) => solutions_2018::day3::part2(),
            (4, 1) => solutions_2018::day4::part1(),
            (4, 2) => solutions_2018::day4::part2(),
            (5, 1) => solutions_2018::day5::part1(),
            (5, 2) => solutions_2018::day5::part2(),
            (6, 1) => solutions_2018::day6::part1(),
            (6, 2) => solutions_2018::day6::part2(),
            (7, 1) => solutions_2018::day7::part1(),
            (7, 2) => solutions_2018::day7::part2(),
            (8, 1) => solutions_2018::day8::part1(),
            (8, 2) => solutions_2018::day8::part2(),
            (9, 1) => solutions_2018::day9::part1(),
            (9, 2) => solutions_2018::day9::part2(),
            (10, 1) => solutions_2018::day10::part1(),
            (10, 2) => solutions_2018::day10::part2(),
            (11, 1) => solutions_2018::day11::part1(),
            (11, 2) => solutions_2018::day11::part2(),
            (12, 1) => solutions_2018::day12::part1(),
            (12, 2) => solutions_2018::day12::part2(),
            (13, 1) => solutions_2018::day13::part1(),
            (13, 2) => solutions_2018::day13::part2(),
            (14, 1) => solutions_2018::day14::part1(),
            (14, 2) => solutions_2018::day14::part2(),
            (15, 1) => solutions_2018::day15::part1(),
            (15, 2) => solutions_2018::day15::part2(),
            (16, 1) => solutions_2018::day16::part1(),
            (16, 2) => solutions_2018::day16::part2(),
            (17, 1) => solutions_2018::day17::part1(),
            (17, 2) => solutions_2018::day17::part2(),
            (18, 1) => solutions_2018::day18::part1(),
            (18, 2) => solutions_2018::day18::part2(),
            (19, 1) => solutions_2018::day19::part1(),
            (19, 2) => solutions_2018::day19::part2(),
            (20, 1) => solutions_2018::day20::part1(),
            (20, 2) => solutions_2018::day20::part2(),
            (21, 1) => solutions_2018::day21::part1(),
            (21, 2) => solutions_2018::day21::part2(),
            (22, 1) => solutions_2018::day22::part1(),
            (22, 2) => solutions_2018::day22::part2(),
            (23, 1) => solutions_2018::day23::part1(),
            (23, 2) => solutions_2018::day23::part2(),
            (24, 1) => solutions_2018::day24::part1(),
            (24, 2) => solutions_2018::day24::part2(),
            (25, 1) => solutions_2018::day25::part1(),
            (25, 2) => solutions_2018::day25::part2(),
            (_, _) => Err(AdventError::NotImplemented(year, day, part))?,
        },
        2019 => match (day, part) {
            (1, 1) => solutions_2019::day1::part1(),
            (1, 2) => solutions_2019::day1::part2(),
            (2, 1) => solutions_2019::day2::part1(),
            (2, 2) => solutions_2019::day2::part2(),
            (3, 1) => solutions_2019::day3::part1(),
            (3, 2) => solutions_2019::day3::part2(),
            (4, 1) => solutions_2019::day4::part1(),
            (4, 2) => solutions_2019::day4::part2(),
            (5, 1) => solutions_2019::day5::part1(),
            (5, 2) => solutions_2019::day5::part2(),
            (6, 1) => solutions_2019::day6::part1(),
            (6, 2) => solutions_2019::day6::part2(),
            (7, 1) => solutions_2019::day7::part1(),
            (7, 2) => solutions_2019::day7::part2(),
            (8, 1) => solutions_2019::day8::part1(),
            (8, 2) => solutions_2019::day8::part2(),
            (9, 1) => solutions_2019::day9::part1(),
            (9, 2) => solutions_2019::day9::part2(),
            (10, 1) => solutions_2019::day10::part1(),
            (10, 2) => solutions_2019::day10::part2(),
            (11, 1) => solutions_2019::day11::part1(),
            (11, 2) => solutions_2019::day11::part2(),
            (12, 1) => solutions_2019::day12::part1(),
            (12, 2) => solutions_2019::day12::part2(),
            (13, 1) => solutions_2019::day13::part1(),
            (13, 2) => solutions_2019::day13::part2(),
            (14, 1) => solutions_2019::day14::part1(),
            (14, 2) => solutions_2019::day14::part2(),
            (15, 1) => solutions_2019::day15::part1(),
            (15, 2) => solutions_2019::day15::part2(),
            (16, 1) => solutions_2019::day16::part1(),
            (16, 2) => solutions_2019::day16::part2(),
            (17, 1) => solutions_2019::day17::part1(),
            (17, 2) => solutions_2019::day17::part2(),
            (18, 1) => solutions_2019::day18::part1(),
            (18, 2) => solutions_2019::day18::part2(),
            (19, 1) => solutions_2019::day19::part1(),
            (19, 2) => solutions_2019::day19::part2(),
            (20, 1) => solutions_2019::day20::part1(),
            (20, 2) => solutions_2019::day20::part2(),
            (21, 1) => solutions_2019::day21::part1(),
            (21, 2) => solutions_2019::day21::part2(),
            (22, 1) => solutions_2019::day22::part1(),
            (22, 2) => solutions_2019::day22::part2(),
            (23, 1) => solutions_2019::day23::part1(),
            (23, 2) => solutions_2019::day23::part2(),
            (24, 1) => solutions_2019::day24::part1(),
            (24, 2) => solutions_2019::day24::part2(),
            (25, 1) => solutions_2019::day25::part1(),
            (25, 2) => solutions_2019::day25::part2(),
            (_, _) => Err(AdventError::NotImplemented(year, day, part))?,
        },
        _ => Err(AdventError::NotImplemented(year, day, part))?,
    }
}
