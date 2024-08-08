use std::time::Duration;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, NaiveDateTime, NaiveTime, TimeDelta, Timelike};

pub fn parse_at(input: &str) -> Result<ParseResult> {
    let now = Local::now();

    if let Ok(utc_time) = dateparser::parse(input) {
        let time: DateTime<Local> = DateTime::from(utc_time);

        let time_of_day = time.time();
        let time_of_day_now = now.time();

        let duration = if time_of_day > time_of_day_now {
            time_of_day - time_of_day_now
        } else {
            TimeDelta::days(1) + (time_of_day - time_of_day_now)
        }
        .to_std()?;

        let time = now + duration;
        let result = ParseResult::new(duration, time);

        return Ok(result);
    }

    Err(anyhow!("Failed to parse time: {}", input))
}

pub fn parse_in(input: &str) -> Result<ParseResult> {
    let now = Local::now();

    if let Ok(seconds) = input.parse::<f64>() {
        let duration = Duration::from_secs_f64(seconds);
        let time = now + duration;
        let result = ParseResult::new(duration, time);

        return Ok(result);
    }

    if let Ok(duration) = humantime::parse_duration(input) {
        let time = now + duration;
        let result = ParseResult::new(duration, time);

        return Ok(result);
    }

    Err(anyhow!("Failed to parse duration: {}", input))
}

pub struct ParseResult {
    pub duration: Duration,
    pub duration_str: String,

    pub time: DateTime<Local>,
    pub time_str: String,
}

impl ParseResult {
    pub fn new(duration: Duration, time: DateTime<Local>) -> Self {
        let duration_str = Self::humanize_duration(duration);
        let time_str = Self::humanize_time(time);
        Self {
            duration,
            duration_str,
            time,
            time_str,
        }
    }

    fn humanize_duration(duration: Duration) -> String {
        let duration_without_milli = Duration::from_secs(duration.as_secs());
        humantime::format_duration(duration_without_milli).to_string()
    }

    fn humanize_time(time: DateTime<Local>) -> String {
        let time_of_day = time.naive_local();
        let time_of_day_without_milli = NaiveTime::from_hms_opt(
            time_of_day.hour(),
            time_of_day.minute(),
            time_of_day.second(),
        )
        .unwrap();

        let time_without_milli = NaiveDateTime::new(time.date_naive(), time_of_day_without_milli);
        time_without_milli.to_string()
    }
}
