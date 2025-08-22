// This won't be documented further as it is intended to be removed, or merged with the `time_format` crate.

use chrono::{DateTime, Local, NaiveDateTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateTimeType {
    Naive(NaiveDateTime),
    Local(DateTime<Local>),
}

impl DateTimeType {
    /// Converts the [`DateTimeType`] to a [`NaiveDateTime`].
    ///
    /// If the [`DateTimeType`] is already a [`NaiveDateTime`], it will be returned as is.
    /// If the [`DateTimeType`] is a [`DateTime<Local>`], it will be converted to a [`NaiveDateTime`].
    pub fn to_naive(self) -> NaiveDateTime {
        match self {
            DateTimeType::Naive(naive) => naive,
            DateTimeType::Local(local) => local.naive_local(),
        }
    }
}

pub struct FormatDistance {
    date: DateTimeType,
    base_date: DateTimeType,
    include_seconds: bool,
    add_suffix: bool,
    hide_prefix: bool,
}

impl FormatDistance {
    pub fn new(date: DateTimeType, base_date: DateTimeType) -> Self {
        Self {
            date,
            base_date,
            include_seconds: false,
            add_suffix: false,
            hide_prefix: false,
        }
    }

    pub fn from_now(date: DateTimeType) -> Self {
        Self::new(date, DateTimeType::Local(Local::now()))
    }

    pub fn include_seconds(mut self, include_seconds: bool) -> Self {
        self.include_seconds = include_seconds;
        self
    }

    pub fn add_suffix(mut self, add_suffix: bool) -> Self {
        self.add_suffix = add_suffix;
        self
    }

    pub fn hide_prefix(mut self, hide_prefix: bool) -> Self {
        self.hide_prefix = hide_prefix;
        self
    }
}

impl std::fmt::Display for FormatDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format_distance(
                self.date,
                self.base_date.to_naive(),
                self.include_seconds,
                self.add_suffix,
                self.hide_prefix,
            )
        )
    }
}
/// Calculates the distance in seconds between two [`NaiveDateTime`] objects.
/// It returns a signed integer denoting the difference. If `date` is earlier than `base_date`, the returned value will be negative.
///
/// ## Arguments
///
/// * `date` - A [NaiveDateTime`] object representing the date of interest
/// * `base_date` - A [NaiveDateTime`] object representing the base date against which the comparison is made
fn distance_in_seconds(date: NaiveDateTime, base_date: NaiveDateTime) -> i64 {
    let duration = date.signed_duration_since(base_date);
    -duration.num_seconds()
}

/// Generates a string describing the time distance between two dates in a human-readable way.
fn distance_string(
    distance: i64,
    include_seconds: bool,
    add_suffix: bool,
    hide_prefix: bool,
) -> String {
    let suffix = if distance < 0 { " from now" } else { " ago" };

    let distance = distance.abs();

    let minutes = distance / 60;
    let hours = distance / 3_600;
    let days = distance / 86_400;
    let months = distance / 2_592_000;

    let string = if distance < 5 && include_seconds {
        if hide_prefix {
            "5 seconds"
        } else {
            "less than 5 seconds"
        }
        .to_string()
    } else if distance < 10 && include_seconds {
        if hide_prefix {
            "10 seconds"
        } else {
            "less than 10 seconds"
        }
        .to_string()
    } else if distance < 20 && include_seconds {
        if hide_prefix {
            "20 seconds"
        } else {
            "less than 20 seconds"
        }
        .to_string()
    } else if distance < 40 && include_seconds {
        "half a minute".to_string()
    } else if distance < 60 && include_seconds {
        if hide_prefix {
            "a minute"
        } else {
            "less than a minute"
        }
        .to_string()
    } else if distance < 90 && include_seconds {
        "1 minute".to_string()
    } else if distance < 30 {
        if hide_prefix {
            "a minute"
        } else {
            "less than a minute"
        }
        .to_string()
    } else if distance < 90 {
        "1 minute".to_string()
    } else if distance < 2_700 {
        format!("{} minutes", minutes)
    } else if distance < 5_400 {
        if hide_prefix {
            "1 hour"
        } else {
            "about 1 hour"
        }
        .to_string()
    } else if distance < 86_400 {
        if hide_prefix {
            format!("{} hours", hours)
        } else {
            format!("about {} hours", hours)
        }
    } else if distance < 172_800 {
        "1 day".to_string()
    } else if distance < 2_592_000 {
        format!("{} days", days)
    } else if distance < 5_184_000 {
        if hide_prefix {
            "1 month"
        } else {
            "about 1 month"
        }
        .to_string()
    } else if distance < 7_776_000 {
        if hide_prefix {
            "2 months"
        } else {
            "about 2 months"
        }
        .to_string()
    } else if distance < 31_540_000 {
        format!("{} months", months)
    } else if distance < 39_425_000 {
        if hide_prefix {
            "1 year"
        } else {
            "about 1 year"
        }
        .to_string()
    } else if distance < 55_195_000 {
        if hide_prefix { "1 year" } else { "over 1 year" }.to_string()
    } else if distance < 63_080_000 {
        if hide_prefix {
            "2 years"
        } else {
            "almost 2 years"
        }
        .to_string()
    } else {
        let years = distance / 31_536_000;
        let remaining_months = (distance % 31_536_000) / 2_592_000;

        if remaining_months < 3 {
            if hide_prefix {
                format!("{} years", years)
            } else {
                format!("about {} years", years)
            }
        } else if remaining_months < 9 {
            if hide_prefix {
                format!("{} years", years)
            } else {
                format!("over {} years", years)
            }
        } else if hide_prefix {
            format!("{} years", years + 1)
        } else {
            format!("almost {} years", years + 1)
        }
    };

    if add_suffix {
        format!("{}{}", string, suffix)
    } else {
        string
    }
}

/// Get the time difference between two dates into a relative human readable string.
///
/// For example, "less than a minute ago", "about 2 hours ago", "3 months from now", etc.
///
/// Use [`format_distance_from_now`] to compare a NaiveDateTime against now.
pub fn format_distance(
    date: DateTimeType,
    base_date: NaiveDateTime,
    include_seconds: bool,
    add_suffix: bool,
    hide_prefix: bool,
) -> String {
    let distance = distance_in_seconds(date.to_naive(), base_date);

    distance_string(distance, include_seconds, add_suffix, hide_prefix)
}

/// Get the time difference between a date and now as relative human readable string.
///
/// For example, "less than a minute ago", "about 2 hours ago", "3 months from now", etc.
pub fn format_distance_from_now(
    datetime: DateTimeType,
    include_seconds: bool,
    add_suffix: bool,
    hide_prefix: bool,
) -> String {
    let now = chrono::offset::Local::now().naive_local();

    format_distance(datetime, now, include_seconds, add_suffix, hide_prefix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn test_format_distance() {
        let date = DateTimeType::Naive(
            #[allow(deprecated)]
            NaiveDateTime::from_timestamp_opt(9600, 0).expect("Invalid NaiveDateTime for date"),
        );
        let base_date = DateTimeType::Naive(
            #[allow(deprecated)]
            NaiveDateTime::from_timestamp_opt(0, 0).expect("Invalid NaiveDateTime for base_date"),
        );

        assert_eq!(
            "about 2 hours",
            format_distance(date, base_date.to_naive(), false, false, false)
        );
    }

    #[test]
    fn test_format_distance_with_suffix() {
        let date = DateTimeType::Naive(
            #[allow(deprecated)]
            NaiveDateTime::from_timestamp_opt(9600, 0).expect("Invalid NaiveDateTime for date"),
        );
        let base_date = DateTimeType::Naive(
            #[allow(deprecated)]
            NaiveDateTime::from_timestamp_opt(0, 0).expect("Invalid NaiveDateTime for base_date"),
        );

        assert_eq!(
            "about 2 hours from now",
            format_distance(date, base_date.to_naive(), false, true, false)
        );
    }

    #[test]
    fn test_format_distance_from_hms() {
        let date = DateTimeType::Naive(
            NaiveDateTime::parse_from_str("1969-07-20T11:22:33Z", "%Y-%m-%dT%H:%M:%SZ")
                .expect("Invalid NaiveDateTime for date"),
        );
        let base_date = DateTimeType::Naive(
            NaiveDateTime::parse_from_str("2024-02-01T00:00:00Z", "%Y-%m-%dT%H:%M:%SZ")
                .expect("Invalid NaiveDateTime for base_date"),
        );

        assert_eq!(
            "over 54 years ago",
            format_distance(date, base_date.to_naive(), false, true, false)
        );
    }

    #[test]
    fn test_format_distance_string() {
        assert_eq!(
            distance_string(3, false, false, false),
            "less than a minute"
        );
        assert_eq!(
            distance_string(7, false, false, false),
            "less than a minute"
        );
        assert_eq!(
            distance_string(13, false, false, false),
            "less than a minute"
        );
        assert_eq!(
            distance_string(21, false, false, false),
            "less than a minute"
        );
        assert_eq!(distance_string(45, false, false, false), "1 minute");
        assert_eq!(distance_string(61, false, false, false), "1 minute");
        assert_eq!(distance_string(1920, false, false, false), "32 minutes");
        assert_eq!(distance_string(3902, false, false, false), "about 1 hour");
        assert_eq!(distance_string(18002, false, false, false), "about 5 hours");
        assert_eq!(distance_string(86470, false, false, false), "1 day");
        assert_eq!(distance_string(345880, false, false, false), "4 days");
        assert_eq!(
            distance_string(2764800, false, false, false),
            "about 1 month"
        );
        assert_eq!(
            distance_string(5184000, false, false, false),
            "about 2 months"
        );
        assert_eq!(distance_string(10368000, false, false, false), "4 months");
        assert_eq!(
            distance_string(34694000, false, false, false),
            "about 1 year"
        );
        assert_eq!(
            distance_string(47310000, false, false, false),
            "over 1 year"
        );
        assert_eq!(
            distance_string(61503000, false, false, false),
            "almost 2 years"
        );
        assert_eq!(
            distance_string(160854000, false, false, false),
            "about 5 years"
        );
        assert_eq!(
            distance_string(236550000, false, false, false),
            "over 7 years"
        );
        assert_eq!(
            distance_string(249166000, false, false, false),
            "almost 8 years"
        );
    }

    #[test]
    fn test_format_distance_string_include_seconds() {
        assert_eq!(
            distance_string(3, true, false, false),
            "less than 5 seconds"
        );
        assert_eq!(
            distance_string(7, true, false, false),
            "less than 10 seconds"
        );
        assert_eq!(
            distance_string(13, true, false, false),
            "less than 20 seconds"
        );
        assert_eq!(distance_string(21, true, false, false), "half a minute");
        assert_eq!(
            distance_string(45, true, false, false),
            "less than a minute"
        );
        assert_eq!(distance_string(61, true, false, false), "1 minute");
    }
}
