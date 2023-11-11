use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use chrono_tz::EST;
use std::fs;
use std::path::{Path, PathBuf};

use clap::ArgMatches;

pub struct CredentialsManager {
    session_token: String,
}

impl CredentialsManager {
    fn get_credentials_file() -> PathBuf {
        let mut current_path = std::env::current_dir().expect("Could not find current path");
        current_path.set_file_name("credentials.toml");
        current_path
    }

    pub fn new() -> Self {
        let creds_path = CredentialsManager::get_credentials_file();

        let content = fs::read_to_string(creds_path).expect("Failed to parse credentials.toml");
        let creds: toml::Value = content.parse().expect("Failed to parse credentials.toml");
        let token = creds["session"]
            .as_str()
            .expect("Unable to get `session` key")
            .to_string();

        CredentialsManager {
            session_token: token,
        }
    }

    pub fn get_session_token(&self) -> String {
        self.session_token.clone()
    }
}

pub struct DateManager {
    pub dates: Vec<AocDate>,
    pub current_year: i32,
}

impl DateManager {
    pub fn new(args: &ArgMatches) -> Self {
        let utc_today = Utc::now().naive_utc();
        let today = EST.from_utc_datetime(&utc_today);

        let year: i32 = args
            .value_of("year")
            .map(|d| d.parse::<i32>().expect("Year not formatted correctly"))
            .unwrap_or_else(|| today.year());

        if year < 2015 {
            panic!("Advent of Code only goes back to 2015.");
        }

        let min_date = NaiveDate::from_ymd(year, 12, 1);
        let max_date = NaiveDate::from_ymd(year, 12, 25);

        let day = if today.date().naive_local() < min_date {
            min_date.day()
        } else if today.date().naive_local() > max_date {
            max_date.day()
        } else {
            today.day()
        };

        let valid_dates: Vec<AocDate> = (1..=day as u8)
            .map(|d| AocDate { year: year, day: d })
            .collect();

        DateManager {
            current_year: year,
            dates: valid_dates,
        }
    }
}

#[derive(Debug)]
pub struct AocDate {
    pub year: i32,
    pub day: u8,
}

impl AocDate {
    pub fn directory(&self) -> PathBuf {
        Path::new(&format!("input/{}", self.year)).to_path_buf()
    }

    pub fn file_name(&self) -> PathBuf {
        Path::new(&format!("input/{}/day{}.txt", self.year, self.day)).to_path_buf()
    }

    pub fn request_url(&self) -> String {
        format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        )
    }
}
