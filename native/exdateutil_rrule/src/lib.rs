use chrono::{DateTime, SecondsFormat};
use rrule::{RRule, RRuleSet, Tz, Unvalidated};
use rustler::{Error, NifResult, NifStruct};
use std::fmt::Display;

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

fn raise(error: impl Display) -> Error {
    Error::RaiseTerm(Box::new(error.to_string()))
}

fn parse_rrule_set(rrule_string: &str) -> NifResult<RRuleSet> {
    let rrule_set: RRuleSet = rrule_string.parse().map_err(raise)?;
    let tzid_of_utc_dtstart = rrule_string
        .lines()
        .find_map(|line| line.strip_prefix("DTSTART;TZID="))
        .and_then(|value| value.split_once(':'))
        .filter(|(_tzid, date)| date.ends_with('Z'))
        .map(|(tzid, _date)| tzid);

    match tzid_of_utc_dtstart {
        None => Ok(rrule_set),
        Some(tzid) => {
            let tz: chrono_tz::Tz = tzid.parse().map_err(raise)?;
            let dt_start = rrule_set.get_dt_start().with_timezone(&Tz::from(tz));
            let lines_without_dtstart: Vec<&str> = rrule_string
                .lines()
                .filter(|line| !line.starts_with("DTSTART"))
                .collect();
            RRuleSet::new(dt_start)
                .set_from_string(&lines_without_dtstart.join("\n"))
                .map_err(raise)
        }
    }
}

fn parse_date(date: &str) -> NifResult<DateTime<Tz>> {
    Ok(DateTime::parse_from_rfc3339(date)
        .map_err(raise)?
        .with_timezone(&Tz::UTC))
}

fn occurrences(rrule_set: RRuleSet, limit: u16) -> NifResult<Vec<DateTime<Tz>>> {
    let result = rrule_set.all(limit);
    if result.limited && result.dates.len() < usize::from(limit) {
        return Err(raise(
            "Reached the rrule iteration limit before the next occurrence. \
            See 'validator limits' in the rrule docs.",
        ));
    }
    Ok(result.dates)
}

fn to_strings(dates: impl IntoIterator<Item = DateTime<Tz>>) -> Vec<String> {
    dates
        .into_iter()
        .map(|date| date.to_rfc3339_opts(SecondsFormat::Millis, true))
        .collect()
}

#[rustler::nif]
fn properties(rrule_string: String) -> NifResult<Properties> {
    let rrule_set = parse_rrule_set(&rrule_string)?;
    let rrule = rrule_set
        .get_rrule()
        .first()
        .ok_or_else(|| raise("The string has no RRULE line."))?;
    let rrule_line = rrule_string
        .lines()
        .find(|line| line.starts_with("RRULE"))
        .unwrap_or_default();
    let unvalidated_rrule: RRule<Unvalidated> = rrule_line.parse().map_err(raise)?;
    let mut by_n_month_day: Vec<i8> = unvalidated_rrule
        .get_by_month_day()
        .iter()
        .copied()
        .filter(|day| *day < 0)
        .collect();
    by_n_month_day.sort_unstable();
    by_n_month_day.dedup();

    Ok(Properties {
        freq: format!("{:?}", rrule.get_freq()),
        interval: rrule.get_interval(),
        count: format!("{:?}", rrule.get_count()),
        until: format!("{:?}", rrule.get_until()),
        week_start: format!("{:?}", rrule.get_week_start()),
        by_set_pos: format!("{:?}", rrule.get_by_set_pos()),
        by_month: rrule.get_by_month().to_vec(),
        by_month_day: rrule.get_by_month_day().to_vec(),
        by_n_month_day: format!("{:?}", by_n_month_day),
        by_year_day: rrule.get_by_year_day().to_vec(),
        by_week_no: rrule.get_by_week_no().to_vec(),
        by_weekday: rrule
            .get_by_weekday()
            .iter()
            .map(|weekday| format!("{:?}", weekday))
            .collect(),
        by_hour: rrule.get_by_hour().to_vec(),
        by_minute: rrule.get_by_minute().to_vec(),
        by_second: rrule.get_by_second().to_vec(),
        by_easter: format!("{:?}", rrule.get_by_easter()),
    })
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
fn next(rrule_string: String, limit: u16) -> NifResult<Vec<String>> {
    let rrule_set = parse_rrule_set(&rrule_string)?;
    Ok(to_strings(occurrences(rrule_set, limit)?))
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
fn between(
    rrule_string: String,
    start_date: String,
    end_date: String,
    inc: bool,
) -> NifResult<Vec<String>> {
    let start = parse_date(&start_date)?;
    let end = parse_date(&end_date)?;
    let rrule_set = parse_rrule_set(&rrule_string)?.after(start).before(end);
    let recurrences = occurrences(rrule_set, u16::MAX)?;
    Ok(to_strings(
        recurrences
            .into_iter()
            .filter(|date| inc || (*date != start && *date != end)),
    ))
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
fn just_before(rrule_string: String, before: String, inc: bool) -> NifResult<Vec<String>> {
    let end = parse_date(&before)?;
    let rrule_set = parse_rrule_set(&rrule_string)?.before(end);
    let recurrences = occurrences(rrule_set, u16::MAX)?;
    Ok(to_strings(
        recurrences.into_iter().rfind(|date| inc || *date != end),
    ))
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
fn just_after(rrule_string: String, after_date: String, inc: bool) -> NifResult<Vec<String>> {
    let start = parse_date(&after_date)?;
    let rrule_set = parse_rrule_set(&rrule_string)?.after(start);
    let recurrences = occurrences(rrule_set, 2)?;
    Ok(to_strings(
        recurrences.into_iter().find(|date| inc || *date != start),
    ))
}

rustler::init!("Elixir.ExDateUtil.Rrule");
