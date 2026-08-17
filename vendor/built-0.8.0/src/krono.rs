use crate::{environment, util, write_str_variable, write_variable};
use std::{fs, io};

impl<'a> util::ParseFromEnv<'a> for chrono::DateTime<chrono::offset::Utc> {
    type Err = chrono::ParseError;

    fn parse_from_env(s: &'a str) -> Result<Self, Self::Err> {
        Ok(chrono::DateTime::parse_from_rfc2822(s)?.with_timezone(&chrono::offset::Utc))
    }
}

/// Parse a time-string as formatted by `built`.
///
/// ```
/// use chrono::Datelike;
///
/// pub mod build_info {
///     pub static BUILT_TIME_UTC: &'static str = "Tue, 14 Feb 2017 05:21:41 GMT";
/// }
///
/// assert_eq!(built::util::strptime(&build_info::BUILT_TIME_UTC).year(), 2017);
/// ```
///
/// # Panics
/// If the string can't be parsed. This should never happen with input provided
/// by `built`.
#[must_use]
pub fn strptime(s: &str) -> chrono::DateTime<chrono::offset::Utc> {
    chrono::DateTime::parse_from_rfc2822(s)
        .unwrap()
        .with_timezone(&chrono::offset::Utc)
}

fn get_source_date_epoch_from_env() -> Option<chrono::DateTime<chrono::offset::Utc>> {
    match std::env::var(crate::SOURCE_DATE_EPOCH) {
        Ok(val) => {
            let ts = match val.parse::<i64>() {
                Ok(ts) => ts,
                Err(_) => {
                    eprintln!("SOURCE_DATE_EPOCH defined, but not a i64");
                    return None;
                }
            };
            match chrono::DateTime::from_timestamp(ts, 0) {
                Some(now) => Some(now),
                None => {
                    eprintln!("SOURCE_DATE_EPOCH can't be represented as a UTC-time");
                    None
                }
            }
        }
        Err(_) => None,
    }
}

pub fn write_time(mut w: &fs::File, envmap: &environment::EnvironmentMap) -> io::Result<()> {
    use io::Write;

    let now = match envmap.get_override_var("BUILT_TIME_UTC") {
        Some(v) => v,
        None => get_source_date_epoch_from_env().unwrap_or_else(chrono::offset::Utc::now),
    };
    write_str_variable!(
        w,
        "BUILT_TIME_UTC",
        now.to_rfc2822(),
        "The build time in RFC2822, UTC."
    );
    Ok(())
}
