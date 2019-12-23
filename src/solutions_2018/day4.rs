use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Context;
use chrono::{NaiveDate, NaiveDateTime, Timelike};

use crate::Result;

pub fn part1() -> Result<String> {
    let obs = observations()?;

    let mut guard_obs = observations_grouped(obs);
    guard_obs.sort_by(|a, b| b.total_minutes.cmp(&a.total_minutes));

    let guard = &guard_obs[0];
    let minute = *guard.minute_asleep_most().0 as i64;
    Ok(format!("{}", guard.id * minute))
}

pub fn part2() -> Result<String> {
    let obs = observations()?;

    let mut guard_obs = observations_grouped(obs);
    guard_obs.sort_by(|a, b| b.minute_asleep_most().1.cmp(a.minute_asleep_most().1));

    let guard = &guard_obs[0];
    let minute = *guard.minute_asleep_most().0 as i64;
    Ok(format!("{}", guard.id * minute))
}

// Types

#[derive(Debug)]
struct GuardObservation {
    id: i64,
    total_minutes: i64,
    minutes_asleep: HashMap<u32, u32>,
}

impl GuardObservation {
    fn new(id: i64) -> Self {
        GuardObservation {
            id,
            total_minutes: 0,
            minutes_asleep: HashMap::new(),
        }
    }

    fn record_sleep(&mut self, fall_asleep: NaiveDateTime, wake_up: NaiveDateTime) {
        self.total_minutes += (wake_up - fall_asleep).num_minutes();

        for minute in fall_asleep.minute()..wake_up.minute() {
            let entry = self.minutes_asleep.entry(minute).or_insert(0);
            *entry += 1;
        }
    }

    fn minute_asleep_most(&self) -> (&u32, &u32) {
        self.minutes_asleep
            .iter()
            .max_by_key(|val| val.1)
            .expect("Guard minutes hashmap empty")
    }
}

#[derive(Debug)]
enum ObserveState {
    BeginShift(i64),
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Observation {
    timestamp: NaiveDateTime,
    observe_state: ObserveState,
}

impl Observation {
    fn from(observation_str: String) -> Result<Self> {
        let left_bracket = observation_str
            .find("[")
            .with_context(|| "Missing left bracket")?;

        let right_bracket = observation_str
            .find("]")
            .with_context(|| "Missing right bracket")?;

        let timestamp = NaiveDateTime::parse_from_str(
            &observation_str[left_bracket + 1..right_bracket],
            "%Y-%m-%d %H:%M",
        )?;

        let observe_state = match observation_str {
            s if s.contains("wakes up") => ObserveState::WakeUp,
            s if s.contains("falls asleep") => ObserveState::FallAsleep,
            s => {
                let parts: Vec<_> = s.split_whitespace().collect();
                let guard_id: i64 = parts[3].replace("#", "").parse().unwrap();
                ObserveState::BeginShift(guard_id)
            }
        };

        Ok(Observation {
            timestamp,
            observe_state,
        })
    }
}

// Helper Functions

fn observations() -> Result<Vec<Observation>> {
    let file = File::open("data/2018/day4_input.txt")?;
    let reader = BufReader::new(file);

    let mut records: Vec<Observation> = reader
        .lines()
        .flat_map(|line| line)
        .map(|line| Observation::from(line))
        .flat_map(|line| line)
        .collect();

    records.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    Ok(records)
}

fn observations_grouped(observations: Vec<Observation>) -> Vec<GuardObservation> {
    let mut guard_obs = HashMap::new();

    let mut current_guard_id = 0;
    let mut fall_asleep_time = NaiveDate::from_ymd(1900, 1, 1).and_hms(0, 0, 0);

    for observe in observations {
        match observe.observe_state {
            ObserveState::BeginShift(id) => current_guard_id = id,
            ObserveState::FallAsleep => fall_asleep_time = observe.timestamp,
            ObserveState::WakeUp => {
                let cur_obs = guard_obs
                    .entry(current_guard_id)
                    .or_insert(GuardObservation::new(current_guard_id));

                cur_obs.record_sleep(fall_asleep_time, observe.timestamp);
            }
        };
    }

    guard_obs.drain().map(|e| e.1).collect()
}
