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
        .ok_or(AdventError::MissingArgument("day".to_owned()))?
        .parse()?;

    let part: u32 = part
        .ok_or(AdventError::MissingArgument("part".to_owned()))?
        .parse()?;

    match (day, part) {
        (1, 1) => days::day1::part1(),
        (1, 2) => days::day1::part2(),
        (2, 1) => days::day2::part1(),
        (2, 2) => days::day2::part2(),
        (3, 1) => days::day3::part1(),
        (_, _) => Err(AdventError::NotImplemented(day, part)),
    }
}
