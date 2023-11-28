use chrono::NaiveDateTime;

/// Calculates the distance in seconds between two NaiveDateTime objects.
/// It returns a signed integer denoting the difference. If `date` is earlier than `base_date`, the returned value will be negative.
///
/// ## Arguments
///
/// * `date` - A NaiveDateTime object representing the date of interest
/// * `base_date` - A NaiveDateTime object representing the base date against which the comparison is made
fn distance_in_seconds(date: NaiveDateTime, base_date: NaiveDateTime) -> i64 {
    let duration = date.signed_duration_since(base_date);
    -duration.num_seconds()
}

/// Generates a string describing the time distance between two dates in a human-readable way.
fn distance_string(distance: i64, include_seconds: bool, add_suffix: bool) -> String {
    let suffix = if distance < 0 { " from now" } else { " ago" };

    let distance = distance.abs();

    let minutes = distance / 60;
    let hours = distance / 3_600;
    let days = distance / 86_400;
    let months = distance / 2_592_000;

    let string = if distance < 5 && include_seconds {
        "less than 5 seconds".to_string()
    } else if distance < 10 && include_seconds {
        "less than 10 seconds".to_string()
    } else if distance < 20 && include_seconds {
        "less than 20 seconds".to_string()
    } else if distance < 40 && include_seconds {
        "half a minute".to_string()
    } else if distance < 60 && include_seconds {
        "less than a minute".to_string()
    } else if distance < 90 && include_seconds {
        "1 minute".to_string()
    } else if distance < 30 {
        "less than a minute".to_string()
    } else if distance < 90 {
        "1 minute".to_string()
    } else if distance < 2_700 {
        format!("{} minutes", minutes)
    } else if distance < 5_400 {
        "about 1 hour".to_string()
    } else if distance < 86_400 {
        format!("about {} hours", hours)
    } else if distance < 172_800 {
        "1 day".to_string()
    } else if distance < 2_592_000 {
        format!("{} days", days)
    } else if distance < 5_184_000 {
        "about 1 month".to_string()
    } else if distance < 7_776_000 {
        "about 2 months".to_string()
    } else if distance < 31_540_000 {
        format!("{} months", months)
    } else if distance < 39_425_000 {
        "about 1 year".to_string()
    } else if distance < 55_195_000 {
        "over 1 year".to_string()
    } else if distance < 63_080_000 {
        "almost 2 years".to_string()
    } else {
        let years = distance / 31_536_000;
        let remaining_months = (distance % 31_536_000) / 2_592_000;

        if remaining_months < 3 {
            format!("about {} years", years)
        } else if remaining_months < 9 {
            format!("over {} years", years)
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
/// Use [naive_format_distance_from_now] to compare a NaiveDateTime against now.
///
/// # Arguments
///
/// * `date` - The NaiveDateTime to compare.
/// * `base_date` - The NaiveDateTime to compare against.
/// * `include_seconds` - A boolean. If true, distances less than a minute are more detailed
/// * `add_suffix` - A boolean. If true, result indicates if the time is in the past or future
///
/// # Example
///
/// ```rust
/// use chrono::DateTime;
/// use ui2::utils::naive_format_distance;
///
/// fn time_between_moon_landings() -> String {
///     let date = DateTime::parse_from_rfc3339("1969-07-20T00:00:00Z").unwrap().naive_local();
///     let base_date = DateTime::parse_from_rfc3339("1972-12-14T00:00:00Z").unwrap().naive_local();
///     format!("There was {} between the first and last crewed moon landings.", naive_format_distance(date, base_date, false, false))
/// }
/// ```
///
/// Output: `"There was about 3 years between the first and last crewed moon landings."`
pub fn naive_format_distance(
    date: NaiveDateTime,
    base_date: NaiveDateTime,
    include_seconds: bool,
    add_suffix: bool,
) -> String {
    let distance = distance_in_seconds(date, base_date);

    distance_string(distance, include_seconds, add_suffix)
}

/// Get the time difference between a date and now as relative human readable string.
///
/// For example, "less than a minute ago", "about 2 hours ago", "3 months from now", etc.
///
/// # Arguments
///
/// * `datetime` - The NaiveDateTime to compare with the current time.
/// * `include_seconds` - A boolean. If true, distances less than a minute are more detailed
/// * `add_suffix` - A boolean. If true, result indicates if the time is in the past or future
///
/// # Example
///
/// ```rust
/// use chrono::DateTime;
/// use ui2::utils::naive_format_distance_from_now;
///
/// fn time_since_first_moon_landing() -> String {
///     let date = DateTime::parse_from_rfc3339("1969-07-20T00:00:00Z").unwrap().naive_local();
///     format!("It's been {} since Apollo 11 first landed on the moon.", naive_format_distance_from_now(date, false, false))
/// }
/// ```
///
/// Output: `It's been over 54 years since Apollo 11 first landed on the moon.`
pub fn naive_format_distance_from_now(
    datetime: NaiveDateTime,
    include_seconds: bool,
    add_suffix: bool,
) -> String {
    let now = chrono::offset::Local::now().naive_local();

    naive_format_distance(datetime, now, include_seconds, add_suffix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn test_naive_format_distance() {
        let date =
            NaiveDateTime::from_timestamp_opt(9600, 0).expect("Invalid NaiveDateTime for date");
        let base_date =
            NaiveDateTime::from_timestamp_opt(0, 0).expect("Invalid NaiveDateTime for base_date");

        assert_eq!(
            "about 2 hours",
            naive_format_distance(date, base_date, false, false)
        );
    }

    #[test]
    fn test_naive_format_distance_with_suffix() {
        let date =
            NaiveDateTime::from_timestamp_opt(9600, 0).expect("Invalid NaiveDateTime for date");
        let base_date =
            NaiveDateTime::from_timestamp_opt(0, 0).expect("Invalid NaiveDateTime for base_date");

        assert_eq!(
            "about 2 hours from now",
            naive_format_distance(date, base_date, false, true)
        );
    }

    #[test]
    fn test_naive_format_distance_from_now() {
        let date = NaiveDateTime::parse_from_str("1969-07-20T00:00:00Z", "%Y-%m-%dT%H:%M:%SZ")
            .expect("Invalid NaiveDateTime for date");

        assert_eq!(
            "over 54 years ago",
            naive_format_distance_from_now(date, false, true)
        );
    }

    #[test]
    fn test_naive_format_distance_string() {
        assert_eq!(distance_string(3, false, false), "less than a minute");
        assert_eq!(distance_string(7, false, false), "less than a minute");
        assert_eq!(distance_string(13, false, false), "less than a minute");
        assert_eq!(distance_string(21, false, false), "less than a minute");
        assert_eq!(distance_string(45, false, false), "1 minute");
        assert_eq!(distance_string(61, false, false), "1 minute");
        assert_eq!(distance_string(1920, false, false), "32 minutes");
        assert_eq!(distance_string(3902, false, false), "about 1 hour");
        assert_eq!(distance_string(18002, false, false), "about 5 hours");
        assert_eq!(distance_string(86470, false, false), "1 day");
        assert_eq!(distance_string(345880, false, false), "4 days");
        assert_eq!(distance_string(2764800, false, false), "about 1 month");
        assert_eq!(distance_string(5184000, false, false), "about 2 months");
        assert_eq!(distance_string(10368000, false, false), "4 months");
        assert_eq!(distance_string(34694000, false, false), "about 1 year");
        assert_eq!(distance_string(47310000, false, false), "over 1 year");
        assert_eq!(distance_string(61503000, false, false), "almost 2 years");
        assert_eq!(distance_string(160854000, false, false), "about 5 years");
        assert_eq!(distance_string(236550000, false, false), "over 7 years");
        assert_eq!(distance_string(249166000, false, false), "almost 8 years");
    }

    #[test]
    fn test_naive_format_distance_string_include_seconds() {
        assert_eq!(distance_string(3, true, false), "less than 5 seconds");
        assert_eq!(distance_string(7, true, false), "less than 10 seconds");
        assert_eq!(distance_string(13, true, false), "less than 20 seconds");
        assert_eq!(distance_string(21, true, false), "half a minute");
        assert_eq!(distance_string(45, true, false), "less than a minute");
        assert_eq!(distance_string(61, true, false), "1 minute");
    }
}
