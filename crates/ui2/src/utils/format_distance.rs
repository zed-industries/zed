use chrono::NaiveDateTime;

fn distance_in_seconds(date: NaiveDateTime, base_date: NaiveDateTime) -> i64 {
    let duration = date.signed_duration_since(base_date);
    duration.num_seconds()
}

fn distance_string(distance: i64, include_seconds: bool, add_suffix: bool) -> String {
    let suffix = if distance < 0 { " from now" } else { " ago" };

    let d = distance.abs();

    let minutes = d / 60;
    let hours = d / 3600;
    let days = d / 86400;
    let months = d / 2592000;
    let years = d / 31536000;

    let string = if d < 5 && include_seconds {
        "less than 5 seconds".to_string()
    } else if d < 10 && include_seconds {
        "less than 10 seconds".to_string()
    } else if d < 20 && include_seconds {
        "less than 20 seconds".to_string()
    } else if d < 40 && include_seconds {
        "half a minute".to_string()
    } else if d < 60 && include_seconds {
        "less than a minute".to_string()
    } else if d < 90 && include_seconds {
        "1 minute".to_string()
    } else if d < 30 {
        "less than a minute".to_string()
    } else if d < 90 {
        "1 minute".to_string()
    } else if d < 2700 {
        format!("{} minutes", minutes)
    } else if d < 5400 {
        "about 1 hour".to_string()
    } else if d < 86400 {
        format!("about {} hours", hours)
    } else if d < 172800 {
        "1 day".to_string()
    } else if d < 2592000 {
        format!("{} days", days)
    } else if d < 5184000 {
        "about 1 month".to_string()
    } else if d < 7776000 {
        "about 2 months".to_string()
    } else if d < 31536000 {
        format!("{} months", months)
    } else if d < 39408000 {
        "about 1 year".to_string()
    } else if d < 47318400 {
        "over 1 year".to_string()
    } else if d < 63072000 {
        "almost 2 years".to_string()
    } else {
        format!("{} years", years)
    };

    if add_suffix {
        return format!("{}{}", string, suffix);
    } else {
        string
    }
}

pub fn naive_format_distance(
    date: NaiveDateTime,
    base_date: NaiveDateTime,
    include_seconds: bool,
    add_suffix: bool,
) -> String {
    let distance = distance_in_seconds(date, base_date);

    distance_string(distance, include_seconds, add_suffix)
}

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

    // #[test]
    // fn test_naive_format_distance() {
    //     let date = NaiveDateTime::from_timestamp_opt(0, 0);
    //     let base_date = NaiveDateTime::from_timestamp_opt(9400, 0);

    //     assert_eq!(
    //         "about 2 hours ago",
    //         naive_format_distance(
    //             date.expect("invalid datetime"),
    //             base_date.expect("invalid datetime"),
    //             false,
    //             true
    //         )
    //     );
    // }

    // #[test]
    // fn test_naive_format_distance_with_seconds() {
    //     let date = NaiveDateTime::from_timestamp_opt(0, 0);
    //     let base_date = NaiveDateTime::from_timestamp_opt(10, 0);

    //     assert_eq!(
    //         "less than 30 seconds ago",
    //         naive_format_distance(
    //             date.expect("invalid datetime"),
    //             base_date.expect("invalid datetime"),
    //             true,
    //             true
    //         )
    //     );
    // }

    // #[test]
    // fn test_naive_format_distance_with_future_date() {
    //     let date = NaiveDateTime::from_timestamp_opt(3400, 0);
    //     let base_date = NaiveDateTime::from_timestamp_opt(00, 0);

    //     assert_eq!(
    //         "about 56 from now",
    //         naive_format_distance(
    //             date.expect("invalid datetime"),
    //             base_date.expect("invalid datetime"),
    //             false,
    //             true
    //         )
    //     );
    // }

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
