use crate::helpers::{AocDate, CredentialsManager, DateManager};
use crate::{year2015, year2021, year2023};
use aoc_types::{Day, Part};
use std::error;
use std::fs::{self, File};
use std::io::Write;

use clap::ArgMatches;
use ureq;

pub struct AocApp {}

impl AocApp {
    pub fn new() -> Self {
        AocApp {}
    }

    fn download_input(&self, token: &str, d: &AocDate) -> Result<(), Box<dyn error::Error>> {
        let input_file = d.file_name();
        if input_file.exists() {
            println!(
                "Input for year {}, day {} already exists. Skipping.",
                d.year, d.day
            );
            return Ok(());
        }

        print!("Requesting input for year {}, day {}... ", d.year, d.day);

        let cookie_token = format!("session={}", token);
        let req_result = ureq::get(&d.request_url())
            .set("Cookie", &cookie_token)
            .call();

        match req_result {
            Ok(response) => match response.status() {
                200 => {
                    let input_dir = d.directory();
                    fs::create_dir_all(&input_dir)?;

                    let body = response.into_string()?;
                    let mut file = File::create(&input_file)?;
                    file.write_all(body.as_bytes())?;
                }
                status_code => {
                    println!(
                        "Unable to download input for {} - Day {}. Status Code: {}",
                        d.year, d.day, status_code
                    )
                }
            },
            Err(e) => eprintln!("Download request failed. Error: {}", e),
        };

        println!("Done.");
        Ok(())
    }

    pub fn execute_input(&self, sub_args: &ArgMatches) {
        let token = CredentialsManager::new().get_session_token();

        let date_mgr = DateManager::new(sub_args);

        date_mgr.dates.iter().for_each(|d| {
            self.download_input(&token, &d).expect(&format!(
                "Unable to download input for year {} day {}.",
                d.year, d.day
            ));
        });
    }

    pub fn execute_run_problem(&self, args: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
        let date_mgr = DateManager::new(args);

        let day: Option<Day> = args
            .value_of("day")
            .map(|d| d.parse().expect("Failed to parse day"));

        // TODO: Maybe later
        let _part: Option<Part> = args
            .value_of("part")
            .map(|p| p.parse().expect("Failed to parse part"));

        let day = day.unwrap_or_else(|| {
            date_mgr
                .dates
                .last()
                .expect("No implementation found")
                .day
                .try_into()
                .expect("Failed to parse day")
        });

        match date_mgr.current_year {
            2015 => year2015::__aoc::run_problem(day),
            2021 => year2021::__aoc::run_problem(day),
            2023 => year2023::__aoc::run_problem(day),
            n => return Err(format!("{} not implemented yet.", n).into()),
        }

        Ok(())
    }
}
