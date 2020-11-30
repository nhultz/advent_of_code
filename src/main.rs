use clap::{App, Arg};

use advent_of_code_solutions::{solutions_2018, solutions_2019, solutions_2020};
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
        2018 => solutions_2018::solve(day, part),
        2019 => solutions_2019::solve(day, part),
        2020 => solutions_2020::solve(day, part),
        _ => Err(AdventError::NotImplemented(year, day, part))?,
    }
}
