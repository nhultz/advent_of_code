use clap::{App, Arg, SubCommand};

mod app;
mod helpers;
mod year2015;
mod year2021;
mod year2023;

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
                .help("The year to run."),
        )
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .help("The day to run."),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .takes_value(true)
                .help("The part to run."),
        )
        .subcommand(
            SubCommand::with_name("input")
                .about("Download the input for a specified date")
                .arg(
                    Arg::with_name("year")
                        .short("y")
                        .long("year")
                        .takes_value(true)
                        .help("The year to download."),
                ),
        )
        .get_matches();

    let app = app::AocApp::new();

    match matches.subcommand() {
        ("input", Some(cmd)) => app.execute_input(&cmd),
        (cmd, Some(_)) => eprintln!("Unknown Command: `{}`", cmd),
        _ => {
            if let Err(e) = app.execute_run_problem(&matches) {
                eprintln!("An error occured: {}", e.to_string());
                std::process::exit(-1);
            }
        }
    };
}
