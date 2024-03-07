use time::{OffsetDateTime, UtcOffset};

/// The formatting style for a timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampFormat {
    /// Formats the timestamp as an absolute time, e.g. "12:00 PM", "yesterday at 11:00 AM", "2021-12-31".
    Absolute,
    /// Formats the timestamp as a relative time, e.g. "just now", "1 minute ago", "2 hours ago", "2 months ago".
    Relative,
}

/// Formats a timestamp, which respects the user's date and time preferences/custom format.
pub fn format_localized_timestamp(
    reference: OffsetDateTime,
    timestamp: OffsetDateTime,
    timezone: UtcOffset,
    format: TimestampFormat,
) -> String {
    let timestamp_local = timestamp.to_offset(timezone);
    let reference_local = reference.to_offset(timezone);

    match format {
        TimestampFormat::Absolute => format_absolute_timestamp(timestamp_local, reference_local),
        TimestampFormat::Relative => format_relative_time(timestamp_local, reference_local)
            .unwrap_or_else(|| format_relative_date(timestamp_local, reference_local)),
    }
}

fn format_absolute_timestamp(timestamp: OffsetDateTime, reference: OffsetDateTime) -> String {
    #[cfg(target_os = "macos")]
    {
        let timestamp_date = timestamp.date();
        let reference_date = reference.date();
        if timestamp_date == reference_date {
            macos::format_time(&timestamp)
        } else if reference_date.previous_day() == Some(timestamp_date) {
            format!("yesterday at {}", macos::format_time(&timestamp))
        } else {
            macos::format_date(&timestamp)
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        // todo(linux) respect user's date/time preferences
        // todo(windows) respect user's date/time preferences
        format_timestamp_fallback(timestamp, reference)
    }
}

fn format_relative_time(timestamp: OffsetDateTime, reference: OffsetDateTime) -> Option<String> {
    let difference = reference - timestamp;
    let minutes = difference.whole_minutes();
    match minutes {
        0 => Some("Just now".to_string()),
        1 => Some("1 minute ago".to_string()),
        2..=59 => Some(format!("{} minutes ago", minutes)),
        _ => {
            let hours = difference.whole_hours();
            match hours {
                1 => Some("1 hour ago".to_string()),
                2..=23 => Some(format!("{} hours ago", hours)),
                _ => None,
            }
        }
    }
}

fn format_relative_date(timestamp: OffsetDateTime, reference: OffsetDateTime) -> String {
    let timestamp_date = timestamp.date();
    let reference_date = reference.date();
    let difference = reference_date - timestamp_date;
    let days = difference.whole_days();
    match days {
        0 => "Today".to_string(),
        1 => "Yesterday".to_string(),
        2..=6 => format!("{} days ago", days),
        _ => {
            let weeks = difference.whole_weeks();
            match weeks {
                1 => "1 week ago".to_string(),
                2..=4 => format!("{} weeks ago", weeks),
                _ => {
                    let timestamp_month_diff: u8 = timestamp_date.month().into();
                    let reference_month_diff: u8 = reference_date.month().into();
                    let month_diff = if timestamp_month_diff > reference_month_diff {
                        timestamp_month_diff - reference_month_diff
                    } else {
                        reference_month_diff - timestamp_month_diff
                    };
                    match month_diff {
                        0..=1 => "1 month ago".to_string(),
                        2..=11 => format!("{} months ago", month_diff),
                        _ => {
                            let timestamp_year = timestamp_date.year();
                            let reference_year = reference_date.year();
                            let years = reference_year - timestamp_year;
                            match years {
                                1 => "1 year ago".to_string(),
                                _ => format!("{} years ago", years),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Formats a timestamp, which is either in 12-hour or 24-hour time format.
/// Note:
/// This function does not respect the user's date and time preferences.
/// This should only be used as a fallback mechanism when the OS time formatting fails.
pub fn format_timestamp_naive(
    timestamp_local: OffsetDateTime,
    reference_local: OffsetDateTime,
    is_12_hour_time: bool,
) -> String {
    let timestamp_local_hour = timestamp_local.hour();
    let timestamp_local_minute = timestamp_local.minute();

    let (hour, meridiem) = if is_12_hour_time {
        let meridiem = if timestamp_local_hour >= 12 {
            "pm"
        } else {
            "am"
        };

        let hour_12 = match timestamp_local_hour {
            0 => 12,                              // Midnight
            13..=23 => timestamp_local_hour - 12, // PM hours
            _ => timestamp_local_hour,            // AM hours
        };

        (hour_12, Some(meridiem))
    } else {
        (timestamp_local_hour, None)
    };

    let formatted_time = match meridiem {
        Some(meridiem) => format!("{:02}:{:02} {}", hour, timestamp_local_minute, meridiem),
        None => format!("{:02}:{:02}", hour, timestamp_local_minute),
    };

    let reference_local_date = reference_local.date();
    let timestamp_local_date = timestamp_local.date();

    if timestamp_local_date == reference_local_date {
        return formatted_time;
    }

    if reference_local_date.previous_day() == Some(timestamp_local_date) {
        return format!("yesterday at {}", formatted_time);
    }

    match meridiem {
        Some(_) => format!(
            "{:02}/{:02}/{}",
            timestamp_local_date.month() as u32,
            timestamp_local_date.day(),
            timestamp_local_date.year()
        ),
        None => format!(
            "{:02}/{:02}/{}",
            timestamp_local_date.day(),
            timestamp_local_date.month() as u32,
            timestamp_local_date.year()
        ),
    }
}

#[cfg(not(target_os = "macos"))]
fn format_timestamp_fallback(timestamp: OffsetDateTime, reference: OffsetDateTime) -> String {
    static CURRENT_LOCALE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    let current_locale = CURRENT_LOCALE
        .get_or_init(|| sys_locale::get_locale().unwrap_or_else(|| String::from("en-US")));

    let is_12_hour_time = is_12_hour_time_by_locale(current_locale.as_str());
    format_timestamp_naive(timestamp, reference, is_12_hour_time)
}

#[cfg(not(target_os = "macos"))]
/// Returns `true` if the locale is recognized as a 12-hour time locale.
fn is_12_hour_time_by_locale(locale: &str) -> bool {
    [
        "es-MX", "es-CO", "es-SV", "es-NI",
        "es-HN", // Mexico, Colombia, El Salvador, Nicaragua, Honduras
        "en-US", "en-CA", "en-AU", "en-NZ", // U.S, Canada, Australia, New Zealand
        "ar-SA", "ar-EG", "ar-JO", // Saudi Arabia, Egypt, Jordan
        "en-IN", "hi-IN", // India, Hindu
        "en-PK", "ur-PK", // Pakistan, Urdu
        "en-PH", "fil-PH", // Philippines, Filipino
        "bn-BD", "ccp-BD", // Bangladesh, Chakma
        "en-IE", "ga-IE", // Ireland, Irish
        "en-MY", "ms-MY", // Malaysia, Malay
    ]
    .contains(&locale)
}

#[cfg(target_os = "macos")]
mod macos {
    use core_foundation::base::TCFType;
    use core_foundation::date::CFAbsoluteTime;
    use core_foundation::string::CFString;
    use core_foundation_sys::date_formatter::CFDateFormatterCreateStringWithAbsoluteTime;
    use core_foundation_sys::date_formatter::CFDateFormatterRef;
    use core_foundation_sys::locale::CFLocaleRef;
    use core_foundation_sys::{
        base::kCFAllocatorDefault,
        date_formatter::{
            kCFDateFormatterNoStyle, kCFDateFormatterShortStyle, CFDateFormatterCreate,
        },
        locale::CFLocaleCopyCurrent,
    };

    pub fn format_time(timestamp: &time::OffsetDateTime) -> String {
        format_with_date_formatter(timestamp, TIME_FORMATTER.with(|f| *f))
    }

    pub fn format_date(timestamp: &time::OffsetDateTime) -> String {
        format_with_date_formatter(timestamp, DATE_FORMATTER.with(|f| *f))
    }

    fn format_with_date_formatter(
        timestamp: &time::OffsetDateTime,
        fmt: CFDateFormatterRef,
    ) -> String {
        const UNIX_TO_CF_ABSOLUTE_TIME_OFFSET: i64 = 978307200;
        // Convert timestamp to macOS absolute time
        let timestamp_macos = timestamp.unix_timestamp() - UNIX_TO_CF_ABSOLUTE_TIME_OFFSET;
        let cf_absolute_time = timestamp_macos as CFAbsoluteTime;
        unsafe {
            let s = CFDateFormatterCreateStringWithAbsoluteTime(
                kCFAllocatorDefault,
                fmt,
                cf_absolute_time,
            );
            CFString::wrap_under_create_rule(s).to_string()
        }
    }

    thread_local! {
        static CURRENT_LOCALE: CFLocaleRef = unsafe { CFLocaleCopyCurrent() };
        static TIME_FORMATTER: CFDateFormatterRef = unsafe {
            CFDateFormatterCreate(
                kCFAllocatorDefault,
                CURRENT_LOCALE.with(|locale| *locale),
                kCFDateFormatterNoStyle,
                kCFDateFormatterShortStyle,
            )
        };
        static DATE_FORMATTER: CFDateFormatterRef = unsafe {
            CFDateFormatterCreate(
                kCFAllocatorDefault,
                CURRENT_LOCALE.with(|locale| *locale),
                kCFDateFormatterShortStyle,
                kCFDateFormatterNoStyle,
            )
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_24_hour_time() {
        let reference = create_offset_datetime(1990, 4, 12, 16, 45, 0);
        let timestamp = create_offset_datetime(1990, 4, 12, 15, 30, 0);

        assert_eq!(format_timestamp_naive(timestamp, reference, false), "15:30");
    }

    #[test]
    fn test_format_today() {
        let reference = create_offset_datetime(1990, 4, 12, 16, 45, 0);
        let timestamp = create_offset_datetime(1990, 4, 12, 15, 30, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "03:30 pm"
        );
    }

    #[test]
    fn test_format_yesterday() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 9, 0, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "yesterday at 09:00 am"
        );
    }

    #[test]
    fn test_format_yesterday_less_than_24_hours_ago() {
        let reference = create_offset_datetime(1990, 4, 12, 19, 59, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 20, 0, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "yesterday at 08:00 pm"
        );
    }

    #[test]
    fn test_format_yesterday_more_than_24_hours_ago() {
        let reference = create_offset_datetime(1990, 4, 12, 19, 59, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 18, 0, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "yesterday at 06:00 pm"
        );
    }

    #[test]
    fn test_format_yesterday_over_midnight() {
        let reference = create_offset_datetime(1990, 4, 12, 0, 5, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 23, 55, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "yesterday at 11:55 pm"
        );
    }

    #[test]
    fn test_format_yesterday_over_month() {
        let reference = create_offset_datetime(1990, 4, 2, 9, 0, 0);
        let timestamp = create_offset_datetime(1990, 4, 1, 20, 0, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "yesterday at 08:00 pm"
        );
    }

    #[test]
    fn test_format_before_yesterday() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 10, 20, 20, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "04/10/1990"
        );
    }

    #[test]
    fn test_relative_format_minutes() {
        let reference = create_offset_datetime(1990, 4, 12, 23, 0, 0);
        let mut current_timestamp = reference;

        let mut next_minute = || {
            current_timestamp = if current_timestamp.minute() == 0 {
                current_timestamp
                    .replace_hour(current_timestamp.hour() - 1)
                    .unwrap()
                    .replace_minute(59)
                    .unwrap()
            } else {
                current_timestamp
                    .replace_minute(current_timestamp.minute() - 1)
                    .unwrap()
            };
            current_timestamp
        };

        assert_eq!(
            format_relative_time(reference, reference),
            Some("Just now".to_string())
        );

        assert_eq!(
            format_relative_time(next_minute(), reference),
            Some("1 minute ago".to_string())
        );

        for i in 2..=59 {
            assert_eq!(
                format_relative_time(next_minute(), reference),
                Some(format!("{} minutes ago", i))
            );
        }

        assert_eq!(
            format_relative_time(next_minute(), reference),
            Some("1 hour ago".to_string())
        );
    }

    #[test]
    fn test_relative_format_hours() {
        let reference = create_offset_datetime(1990, 4, 12, 23, 0, 0);
        let mut current_timestamp = reference;

        let mut next_hour = || {
            current_timestamp = if current_timestamp.hour() == 0 {
                let date = current_timestamp.date().previous_day().unwrap();
                current_timestamp.replace_date(date)
            } else {
                current_timestamp
                    .replace_hour(current_timestamp.hour() - 1)
                    .unwrap()
            };
            current_timestamp
        };

        assert_eq!(
            format_relative_time(next_hour(), reference),
            Some("1 hour ago".to_string())
        );

        for i in 2..=23 {
            assert_eq!(
                format_relative_time(next_hour(), reference),
                Some(format!("{} hours ago", i))
            );
        }

        assert_eq!(format_relative_time(next_hour(), reference), None);
    }

    #[test]
    fn test_relative_format_days() {
        let reference = create_offset_datetime(1990, 4, 12, 23, 0, 0);
        let mut current_timestamp = reference;

        let mut next_day = || {
            let date = current_timestamp.date().previous_day().unwrap();
            current_timestamp = current_timestamp.replace_date(date);
            current_timestamp
        };

        assert_eq!(
            format_relative_date(reference, reference),
            "Today".to_string()
        );

        assert_eq!(
            format_relative_date(next_day(), reference),
            "Yesterday".to_string()
        );

        for i in 2..=6 {
            assert_eq!(
                format_relative_date(next_day(), reference),
                format!("{} days ago", i)
            );
        }

        assert_eq!(format_relative_date(next_day(), reference), "1 week ago");
    }

    #[test]
    fn test_relative_format_weeks() {
        let reference = create_offset_datetime(1990, 4, 12, 23, 0, 0);
        let mut current_timestamp = reference;

        let mut next_week = || {
            for _ in 0..7 {
                let date = current_timestamp.date().previous_day().unwrap();
                current_timestamp = current_timestamp.replace_date(date);
            }
            current_timestamp
        };

        assert_eq!(
            format_relative_date(next_week(), reference),
            "1 week ago".to_string()
        );

        for i in 2..=4 {
            assert_eq!(
                format_relative_date(next_week(), reference),
                format!("{} weeks ago", i)
            );
        }

        assert_eq!(format_relative_date(next_week(), reference), "1 month ago");
    }

    #[test]
    fn test_relative_format_months() {
        let reference = create_offset_datetime(1990, 4, 12, 23, 0, 0);
        let mut current_timestamp = reference;

        let mut next_month = || {
            if current_timestamp.month() == time::Month::January {
                current_timestamp = current_timestamp
                    .replace_month(time::Month::December)
                    .unwrap()
                    .replace_year(current_timestamp.year() - 1)
                    .unwrap();
            } else {
                current_timestamp = current_timestamp
                    .replace_month(current_timestamp.month().previous())
                    .unwrap();
            }
            current_timestamp
        };

        assert_eq!(
            format_relative_date(next_month(), reference),
            "1 months ago".to_string()
        );

        for i in 2..=12 {
            assert_eq!(
                format_relative_date(next_month(), reference),
                format!("{} months ago", i)
            );
        }

        assert_eq!(format_relative_date(next_month(), reference), "1 year ago");
    }

    fn test_timezone() -> UtcOffset {
        UtcOffset::from_hms(0, 0, 0).expect("Valid timezone offset")
    }

    fn create_offset_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> OffsetDateTime {
        let date = time::Date::from_calendar_date(year, time::Month::try_from(month).unwrap(), day)
            .unwrap();
        let time = time::Time::from_hms(hour, minute, second).unwrap();
        let date = date.with_time(time).assume_utc(); // Assume UTC for simplicity
        date.to_offset(test_timezone())
    }
}
