use chrono::{DateTime, SecondsFormat};
use chrono_tz::UTC;
use rrule::{DateFilter, RRule};
use rustler::NifStruct;

#[derive(Debug, NifStruct)]
#[module = "Properties"]
struct Properties {
    freq: String,
    interval: u16,
    count: String,
    until: String,
    week_start: String,
    by_set_pos: String,
    by_month: Vec<u8>,
    by_month_day: Vec<i8>,
    by_n_month_day: String,
    by_year_day: Vec<i16>,
    by_week_no: Vec<i8>,
    by_weekday: Vec<String>,
    by_hour: Vec<u8>,
    by_minute: Vec<u8>,
    by_second: Vec<u8>,
    by_easter: String,
}

#[rustler::nif]
fn properties(rrule_string: String) -> Properties {
    let rrule: RRule = rrule_string.parse().unwrap();
    let p = rrule.get_properties();
    Properties {
        freq: format!("{:?}", p.freq),
        interval: p.interval.clone(),
        count: format!("{:?}", p.count),
        until: format!("{:?}", p.until),
        week_start: format!("{:?}", p.week_start),
        by_set_pos: format!("{:?}", p.by_set_pos),
        by_month: p.by_month.clone(),
        by_month_day: p.by_month_day.clone(),
        by_n_month_day: format!("{:?}", p.by_n_month_day),
        by_year_day: p.by_year_day.clone(),
        by_week_no: p.by_week_no.clone(),
        by_weekday: p
            .by_weekday
            .clone()
            .iter()
            .map(|x| format!("{:?}", x))
            .collect(),
        by_hour: p.by_hour.clone(),
        by_minute: p.by_minute.clone(),
        by_second: p.by_second.clone(),
        by_easter: format!("{:?}", p.by_easter),
    }
}

#[rustler::nif]
/// `next` takes a string that represents a recurrence rule and a limit, and returns a vector of strings
/// that represent the next `limit` number of recurrences
///
/// Arguments:
///
/// * `rrule_string`: The RRULE string to parse.
/// * `limit`: The number of recurrences to return.
///
/// Returns:
///
/// A vector of strings.
fn next(rrule_string: String, limit: u16) -> Vec<String> {
    let rrule: RRule = rrule_string.parse().unwrap();
    let recurrences = rrule.all(limit).unwrap();
    let iter = recurrences.iter();
    let mut return_vec: Vec<String> = Vec::new();
    for val in iter {
        return_vec.push(val.to_rfc3339_opts(SecondsFormat::Millis, true))
    }

    return return_vec;
}

/// It takes a rrule string, a start date, an end date, and a boolean indicating whether to include the
/// start date in the results, and returns a vector of strings representing the dates that fall between
/// the start and end dates
///
/// Arguments:
///
/// * `rrule_string`: The RRule string to parse.
/// * `start_date`: The start date of the range to check for recurrences.
/// * `end_date`: The end date of the recurrence.
/// * `inc`: Whether to include the start date in the results.
///
/// Returns:
///
/// A vector of strings
#[rustler::nif]
fn between(rrule_string: String, start_date: String, end_date: String, inc: bool) -> Vec<String> {
    let start = DateTime::parse_from_rfc3339(&start_date)
        .unwrap()
        .with_timezone(&UTC);
    let end = DateTime::parse_from_rfc3339(&end_date)
        .unwrap()
        .with_timezone(&UTC);
    let rrule: RRule = rrule_string.parse().unwrap();
    let recurrences = rrule.all_between(start, end, inc).unwrap();
    let iter = recurrences.iter();
    let mut return_vec: Vec<String> = Vec::new();
    for val in iter {
        return_vec.push(val.to_rfc3339_opts(SecondsFormat::Millis, true))
    }

    return return_vec;
}

/// It takes a RRule string, a date string, and a boolean, and returns a vector of date strings
///
/// Arguments:
///
/// * `rrule_string`: The RRule string to parse.
/// * `before`: The date to find the recurrence before.
/// * `inc`: If true, the before date will be included in the results.
///
/// Returns:
///
/// A vector of strings.
#[rustler::nif]
fn just_before(rrule_string: String, before: String, inc: bool) -> Vec<String> {
    let start = DateTime::parse_from_rfc3339(&before)
        .unwrap()
        .with_timezone(&UTC);
    let rrule: RRule = rrule_string.parse().unwrap();
    let recurrences = rrule.just_before(start, inc).unwrap();
    let iter = recurrences.iter();
    let mut return_vec: Vec<String> = Vec::new();
    for val in iter {
        return_vec.push(val.to_rfc3339_opts(SecondsFormat::Millis, true))
    }

    return return_vec;
}

/// `just_after` takes a `String` containing an RRule, a `String` containing a date, and a boolean, and
/// returns a `Vec<String>` containing the dates that match the RRule after the given date
///
/// Arguments:
///
/// * `rrule_string`: The string representation of the RRule.
/// * `after_date`: The date to start looking for recurrences after.
/// * `inc`: If true, include the start date in the results.
///
/// Returns:
///
/// A vector of strings.
#[rustler::nif]
fn just_after(rrule_string: String, after_date: String, inc: bool) -> Vec<String> {
    let start = DateTime::parse_from_rfc3339(&after_date)
        .unwrap()
        .with_timezone(&UTC);
    let rrule: RRule = rrule_string.parse().unwrap();
    let recurrences = rrule.just_after(start, inc).unwrap();
    let iter = recurrences.iter();
    let mut return_vec: Vec<String> = Vec::new();
    for val in iter {
        return_vec.push(val.to_rfc3339_opts(SecondsFormat::Millis, true))
    }

    return return_vec;
}

rustler::init!("Elixir.ExDateUtil.Rrule");
