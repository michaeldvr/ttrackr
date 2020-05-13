// helper file
use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime, Utc};
use inflector::Inflector;

pub type BoxError = Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

pub fn fmt_duration(duration: i32, short: bool, zero_text: &str) -> String {
    if duration == 0 {
        return String::from(zero_text);
    }
    let d = duration;
    let mut res = Vec::<String>::new();
    let units = ["day", "hour", "minute", "second"];

    let (d, text) = calc_duration_step(d, 86400, units[0]);
    if !text.is_empty() {
        res.push(text);
    }

    let (d, text) = calc_duration_step(d, 3600, units[1]);
    if !text.is_empty() {
        res.push(text);
    }

    let (d, text) = calc_duration_step(d, 60, units[2]);
    if !text.is_empty() {
        res.push(text);
    }

    // hide second(s) if `short` set true and `duration` is
    // larger than one minute
    if !short || duration <= 60 {
        let (_, text) = calc_duration_step(d, 1, units[3]);
        if !text.is_empty() {
            res.push(text);
        }
    }

    res.join(" ")
}

pub fn unwrap_string(val: Option<&String>, default: &str) -> String {
    match val {
        Some(txt) => String::from(txt),
        None => String::from(default),
    }
}

fn calc_duration_step(val: i32, mul: i32, unit: &str) -> (i32, String) {
    let mut remainder = val;
    let mut txt = String::from("");
    if val >= mul {
        remainder = val % mul;
        let stepval = val / mul;
        txt.push_str(&stepval.to_string());
        txt.push(' ');
        txt.push_str(&pluralize(unit, stepval));
    }

    (remainder, txt)
}

fn pluralize(text: &str, val: i32) -> String {
    if val == 1 {
        text.to_owned()
    } else {
        text.to_plural()
    }
}

pub fn open_naivedate(data: Option<chrono::NaiveDate>) -> Option<String> {
    match data {
        Some(d) => Some(d.to_string()),
        None => None,
    }
}

pub fn utc_to_local_naive(utc: &str) -> Result<String, BoxError> {
    let naive_dt = NaiveDateTime::parse_from_str(utc, "%Y-%m-%d %H:%M:%S")?;
    let utc_dt: DateTime<Utc> = DateTime::from_utc(naive_dt, Utc);
    let local_dt = utc_dt.with_timezone(&chrono::offset::Local);
    Ok(local_dt.format("%Y-%m-%d %H:%M:%S").to_string())
}

#[allow(dead_code)]
pub fn local_to_utc(local: &str) -> Result<String, BoxError> {
    let localnaive = NaiveDateTime::parse_from_str(local, "%Y-%m-%d %H:%M:%S")?;
    let localdt: DateTime<Local> = chrono::Local.from_local_datetime(&localnaive).unwrap();
    let utcdt = localdt.naive_utc();
    Ok(utcdt.format("%Y-%m-%d %H:%M:%S").to_string())
}

pub fn get_timestamp() -> String {
    let nowstamp = Utc::now().naive_local();
    nowstamp.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[test]
fn formatted_seconds() {
    assert_eq!(fmt_duration(0, false, "zero"), "zero");
    assert_eq!(fmt_duration(1, false, "zero"), "1 second");
    assert_eq!(fmt_duration(5, false, "zero"), "5 seconds");
    assert_eq!(fmt_duration(5, true, "zero"), "5 seconds");
    assert_eq!(fmt_duration(70, false, "zero"), "1 minute 10 seconds");
    assert_eq!(fmt_duration(130, true, "zero"), "2 minutes");
    assert_eq!(fmt_duration(130, true, "zero"), "2 minutes");
    assert_eq!(fmt_duration(8228, true, "zero"), "2 hours 17 minutes");
    assert_eq!(
        fmt_duration(8228, false, "zero"),
        "2 hours 17 minutes 8 seconds"
    );
    assert_eq!(
        fmt_duration(104520, true, "zero"),
        "1 day 5 hours 2 minutes"
    );
    assert_eq!(
        fmt_duration(104520, false, "zero"),
        "1 day 5 hours 2 minutes"
    );
    assert_eq!(
        fmt_duration(1473120, false, "zero"),
        "17 days 1 hour 12 minutes"
    );
}
