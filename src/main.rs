use clap::{App, Arg};

use advent_of_code_solutions::{solutions_2018, solutions_2019, solutions_2020, solutions_2021};
use advent_of_code_solutions::{AdventError, Result, SolveResult};

fn main() -> Result<()> {
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

    let year: u32 = year
        .ok_or(AdventError::MissingArgument("year".into()))?
        .parse()?;

    let run_result = run(year, day, part)?;

    println!("Advent of Code {}", year);
    for solve in run_result {
        println!("{}", solve);
    }

    Ok(())
}

fn run(year: u32, day: Option<&str>, part: Option<&str>) -> Result<Vec<SolveResult<String>>> {
    let day: Option<u32> = day.map(|d| d.parse().unwrap());
    let part: Option<u32> = part.map(|p| p.parse().unwrap());

    let solve_results = match (year, day, part) {
        (y, None, None) => match y {
            2018 => solutions_2018::solve_all(),
            2019 => solutions_2019::solve_all(),
            2020 => solutions_2020::solve_all(),
            2021 => solutions_2021::solve_all(),
            _ => return Err(AdventError::NotImplementedYear(year))?,
        },
        (y, Some(d), None) => match (y, d) {
            (2018, d) => solutions_2018::solve_day(d),
            (2019, d) => solutions_2019::solve_day(d),
            (2020, d) => solutions_2020::solve_day(d),
            (2021, d) => solutions_2021::solve_day(d),
            _ => return Err(AdventError::NotImplementedDay(year, d))?,
        },
        (y, Some(d), Some(p)) => match (y, d, p) {
            (2018, d, p) => vec![solutions_2018::solve_part(d, p)],
            (2019, d, p) => vec![solutions_2019::solve_part(d, p)],
            (2020, d, p) => vec![solutions_2020::solve_part(d, p)],
            (2021, d, p) => vec![solutions_2021::solve_part(d, p)],
            _ => return Err(AdventError::NotImplementedDay(year, d))?,
        },
        _ => return Err(AdventError::NotImplementedYear(year))?,
    };

    Ok(solve_results)
}
