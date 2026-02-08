use time::{OffsetDateTime, UtcOffset};

/// The formatting style for a timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampFormat {
    /// Formats the timestamp as an absolute time, e.g. "2021-12-31 3:00AM".
    Absolute,
    /// Formats the timestamp as an absolute time.
    /// If the message is from today or yesterday the date will be replaced with "Today at x" or "Yesterday at x" respectively.
    /// E.g. "Today at 12:00 PM", "Yesterday at 11:00 AM", "2021-12-31 3:00AM".
    EnhancedAbsolute,
    /// Formats the timestamp as an absolute time, using month name, day of month, year. e.g. "Feb. 24, 2024".
    MediumAbsolute,
    /// Formats the timestamp as a relative time, e.g. "just now", "1 minute ago", "2 hours ago", "2 months ago".
    Relative,
}

/// Formats a timestamp, which respects the user's date and time preferences/custom format.
pub fn format_localized_timestamp(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
    timezone: UtcOffset,
    format: TimestampFormat,
) -> String {
    let timestamp_local = timestamp.to_offset(timezone);
    let reference_local = reference.to_offset(timezone);
    format_local_timestamp(timestamp_local, reference_local, format)
}

/// Formats a timestamp, which respects the user's date and time preferences/custom format.
pub fn format_local_timestamp(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
    format: TimestampFormat,
) -> String {
    match format {
        TimestampFormat::Absolute => format_absolute_timestamp(timestamp, reference, false),
        TimestampFormat::EnhancedAbsolute => format_absolute_timestamp(timestamp, reference, true),
        TimestampFormat::MediumAbsolute => format_absolute_timestamp_medium(timestamp, reference),
        TimestampFormat::Relative => format_relative_time(timestamp, reference)
            .unwrap_or_else(|| format_relative_date(timestamp, reference)),
    }
}

/// Formats the date component of a timestamp
pub fn format_date(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
    enhanced_formatting: bool,
) -> String {
    format_absolute_date(timestamp, reference, enhanced_formatting)
}

/// Formats the time component of a timestamp
pub fn format_time(timestamp: OffsetDateTime) -> String {
    format_absolute_time(timestamp)
}

/// Formats the date component of a timestamp in medium style
pub fn format_date_medium(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
    enhanced_formatting: bool,
) -> String {
    format_absolute_date_medium(timestamp, reference, enhanced_formatting)
}

fn format_absolute_date(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
    #[allow(unused_variables)] enhanced_date_formatting: bool,
) -> String {
    #[cfg(target_os = "macos")]
    {
        if !enhanced_date_formatting {
            return macos::format_date(&timestamp);
        }

        let timestamp_date = timestamp.date();
        let reference_date = reference.date();
        if timestamp_date == reference_date {
            "Today".to_string()
        } else if reference_date.previous_day() == Some(timestamp_date) {
            "Yesterday".to_string()
        } else {
            macos::format_date(&timestamp)
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        // todo(linux) respect user's date/time preferences
        // todo(windows) respect user's date/time preferences
        let current_locale = CURRENT_LOCALE
            .get_or_init(|| sys_locale::get_locale().unwrap_or_else(|| String::from("en-US")));
        format_timestamp_naive_date(
            timestamp,
            reference,
            is_12_hour_time_by_locale(current_locale.as_str()),
        )
    }
}

fn format_absolute_time(timestamp: OffsetDateTime) -> String {
    #[cfg(target_os = "macos")]
    {
        macos::format_time(&timestamp)
    }
    #[cfg(not(target_os = "macos"))]
    {
        // todo(linux) respect user's date/time preferences
        // todo(windows) respect user's date/time preferences
        let current_locale = CURRENT_LOCALE
            .get_or_init(|| sys_locale::get_locale().unwrap_or_else(|| String::from("en-US")));
        format_timestamp_naive_time(
            timestamp,
            is_12_hour_time_by_locale(current_locale.as_str()),
        )
    }
}

fn format_absolute_timestamp(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
    #[allow(unused_variables)] enhanced_date_formatting: bool,
) -> String {
    #[cfg(target_os = "macos")]
    {
        if !enhanced_date_formatting {
            return format!(
                "{} {}",
                format_absolute_date(timestamp, reference, enhanced_date_formatting),
                format_absolute_time(timestamp)
            );
        }

        let timestamp_date = timestamp.date();
        let reference_date = reference.date();
        if timestamp_date == reference_date {
            format!("Today at {}", format_absolute_time(timestamp))
        } else if reference_date.previous_day() == Some(timestamp_date) {
            format!("Yesterday at {}", format_absolute_time(timestamp))
        } else {
            format!(
                "{} {}",
                format_absolute_date(timestamp, reference, enhanced_date_formatting),
                format_absolute_time(timestamp)
            )
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        // todo(linux) respect user's date/time preferences
        // todo(windows) respect user's date/time preferences
        format_timestamp_fallback(timestamp, reference)
    }
}

fn format_absolute_date_medium(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
    enhanced_formatting: bool,
) -> String {
    #[cfg(target_os = "macos")]
    {
        if !enhanced_formatting {
            return macos::format_date_medium(&timestamp);
        }

        let timestamp_date = timestamp.date();
        let reference_date = reference.date();
        if timestamp_date == reference_date {
            "Today".to_string()
        } else if reference_date.previous_day() == Some(timestamp_date) {
            "Yesterday".to_string()
        } else {
            macos::format_date_medium(&timestamp)
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        // todo(linux) respect user's date/time preferences
        // todo(windows) respect user's date/time preferences
        let current_locale = CURRENT_LOCALE
            .get_or_init(|| sys_locale::get_locale().unwrap_or_else(|| String::from("en-US")));
        if !enhanced_formatting {
            return format_timestamp_naive_date_medium(
                timestamp,
                is_12_hour_time_by_locale(current_locale.as_str()),
            );
        }

        let timestamp_date = timestamp.date();
        let reference_date = reference.date();
        if timestamp_date == reference_date {
            "Today".to_string()
        } else if reference_date.previous_day() == Some(timestamp_date) {
            "Yesterday".to_string()
        } else {
            format_timestamp_naive_date_medium(
                timestamp,
                is_12_hour_time_by_locale(current_locale.as_str()),
            )
        }
    }
}

fn format_absolute_timestamp_medium(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
) -> String {
    #[cfg(target_os = "macos")]
    {
        format_absolute_date_medium(timestamp, reference, false)
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
                    let month_diff = calculate_month_difference(timestamp, reference);
                    match month_diff {
                        0..=1 => "1 month ago".to_string(),
                        2..=11 => format!("{} months ago", month_diff),
                        months => {
                            let years = months / 12;
                            match years {
                                1 => "1 year ago".to_string(),
                                _ => format!("{years} years ago"),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Calculates the difference in months between two timestamps.
/// The reference timestamp should always be greater than the timestamp.
fn calculate_month_difference(timestamp: OffsetDateTime, reference: OffsetDateTime) -> usize {
    let timestamp_year = timestamp.year();
    let reference_year = reference.year();
    let timestamp_month: u8 = timestamp.month().into();
    let reference_month: u8 = reference.month().into();

    let month_diff = if reference_month >= timestamp_month {
        reference_month as usize - timestamp_month as usize
    } else {
        12 - timestamp_month as usize + reference_month as usize
    };

    let year_diff = (reference_year - timestamp_year) as usize;
    if year_diff == 0 {
        reference_month as usize - timestamp_month as usize
    } else if month_diff == 0 {
        year_diff * 12
    } else if timestamp_month > reference_month {
        (year_diff - 1) * 12 + month_diff
    } else {
        year_diff * 12 + month_diff
    }
}

/// Formats a timestamp, which is either in 12-hour or 24-hour time format.
/// Note:
/// This function does not respect the user's date and time preferences.
/// This should only be used as a fallback mechanism when the OS time formatting fails.
fn format_timestamp_naive_time(timestamp_local: OffsetDateTime, is_12_hour_time: bool) -> String {
    let timestamp_local_hour = timestamp_local.hour();
    let timestamp_local_minute = timestamp_local.minute();

    let (hour, meridiem) = if is_12_hour_time {
        let meridiem = if timestamp_local_hour >= 12 {
            "PM"
        } else {
            "AM"
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

    match meridiem {
        Some(meridiem) => format!("{}:{:02} {}", hour, timestamp_local_minute, meridiem),
        None => format!("{:02}:{:02}", hour, timestamp_local_minute),
    }
}

#[cfg(not(target_os = "macos"))]
fn format_timestamp_naive_date(
    timestamp_local: OffsetDateTime,
    reference_local: OffsetDateTime,
    is_12_hour_time: bool,
) -> String {
    let reference_local_date = reference_local.date();
    let timestamp_local_date = timestamp_local.date();

    if timestamp_local_date == reference_local_date {
        "Today".to_string()
    } else if reference_local_date.previous_day() == Some(timestamp_local_date) {
        "Yesterday".to_string()
    } else {
        match is_12_hour_time {
            true => format!(
                "{:02}/{:02}/{}",
                timestamp_local_date.month() as u32,
                timestamp_local_date.day(),
                timestamp_local_date.year()
            ),
            false => format!(
                "{:02}/{:02}/{}",
                timestamp_local_date.day(),
                timestamp_local_date.month() as u32,
                timestamp_local_date.year()
            ),
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn format_timestamp_naive_date_medium(
    timestamp_local: OffsetDateTime,
    is_12_hour_time: bool,
) -> String {
    let timestamp_local_date = timestamp_local.date();

    match is_12_hour_time {
        true => format!(
            "{:02}/{:02}/{}",
            timestamp_local_date.month() as u32,
            timestamp_local_date.day(),
            timestamp_local_date.year()
        ),
        false => format!(
            "{:02}/{:02}/{}",
            timestamp_local_date.day(),
            timestamp_local_date.month() as u32,
            timestamp_local_date.year()
        ),
    }
}

pub fn format_timestamp_naive(
    timestamp_local: OffsetDateTime,
    reference_local: OffsetDateTime,
    is_12_hour_time: bool,
) -> String {
    let formatted_time = format_timestamp_naive_time(timestamp_local, is_12_hour_time);
    let reference_local_date = reference_local.date();
    let timestamp_local_date = timestamp_local.date();

    if timestamp_local_date == reference_local_date {
        format!("Today at {}", formatted_time)
    } else if reference_local_date.previous_day() == Some(timestamp_local_date) {
        format!("Yesterday at {}", formatted_time)
    } else {
        let formatted_date = match is_12_hour_time {
            true => format!(
                "{:02}/{:02}/{}",
                timestamp_local_date.month() as u32,
                timestamp_local_date.day(),
                timestamp_local_date.year()
            ),
            false => format!(
                "{:02}/{:02}/{}",
                timestamp_local_date.day(),
                timestamp_local_date.month() as u32,
                timestamp_local_date.year()
            ),
        };
        format!("{} {}", formatted_date, formatted_time)
    }
}

#[cfg(not(target_os = "macos"))]
static CURRENT_LOCALE: std::sync::OnceLock<String> = std::sync::OnceLock::new();

#[cfg(not(target_os = "macos"))]
fn format_timestamp_fallback(timestamp: OffsetDateTime, reference: OffsetDateTime) -> String {
    let current_locale = CURRENT_LOCALE
        .get_or_init(|| sys_locale::get_locale().unwrap_or_else(|| String::from("en-US")));

    let is_12_hour_time = is_12_hour_time_by_locale(current_locale.as_str());
    format_timestamp_naive(timestamp, reference, is_12_hour_time)
}

/// Returns `true` if the locale is recognized as a 12-hour time locale.
#[cfg(not(target_os = "macos"))]
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
            CFDateFormatterCreate, kCFDateFormatterMediumStyle, kCFDateFormatterNoStyle,
            kCFDateFormatterShortStyle,
        },
        locale::CFLocaleCopyCurrent,
    };

    pub fn format_time(timestamp: &time::OffsetDateTime) -> String {
        format_with_date_formatter(timestamp, TIME_FORMATTER.with(|f| *f))
    }

    pub fn format_date(timestamp: &time::OffsetDateTime) -> String {
        format_with_date_formatter(timestamp, DATE_FORMATTER.with(|f| *f))
    }

    pub fn format_date_medium(timestamp: &time::OffsetDateTime) -> String {
        format_with_date_formatter(timestamp, MEDIUM_DATE_FORMATTER.with(|f| *f))
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

        static MEDIUM_DATE_FORMATTER: CFDateFormatterRef = unsafe {
            CFDateFormatterCreate(
                kCFAllocatorDefault,
                CURRENT_LOCALE.with(|locale| *locale),
                kCFDateFormatterMediumStyle,
                kCFDateFormatterNoStyle,
            )
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);

        // Test with same date (today)
        let timestamp_today = create_offset_datetime(1990, 4, 12, 9, 30, 0);
        assert_eq!(format_date(timestamp_today, reference, true), "Today");

        // Test with previous day (yesterday)
        let timestamp_yesterday = create_offset_datetime(1990, 4, 11, 9, 30, 0);
        assert_eq!(
            format_date(timestamp_yesterday, reference, true),
            "Yesterday"
        );

        // Test with other date
        let timestamp_other = create_offset_datetime(1990, 4, 10, 9, 30, 0);
        let result = format_date(timestamp_other, reference, true);
        assert!(!result.is_empty());
        assert_ne!(result, "Today");
        assert_ne!(result, "Yesterday");
    }

    #[test]
    fn test_format_time() {
        let timestamp = create_offset_datetime(1990, 4, 12, 9, 30, 0);

        // We can't assert the exact output as it depends on the platform and locale
        // But we can at least confirm it doesn't panic and returns a non-empty string
        let result = format_time(timestamp);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_format_date_medium() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 12, 9, 30, 0);

        // Test with enhanced formatting (today)
        let result_enhanced = format_date_medium(timestamp, reference, true);
        assert_eq!(result_enhanced, "Today");

        // Test with standard formatting
        let result_standard = format_date_medium(timestamp, reference, false);
        assert!(!result_standard.is_empty());

        // Test yesterday with enhanced formatting
        let timestamp_yesterday = create_offset_datetime(1990, 4, 11, 9, 30, 0);
        let result_yesterday = format_date_medium(timestamp_yesterday, reference, true);
        assert_eq!(result_yesterday, "Yesterday");

        // Test other date with enhanced formatting
        let timestamp_other = create_offset_datetime(1990, 4, 10, 9, 30, 0);
        let result_other = format_date_medium(timestamp_other, reference, true);
        assert!(!result_other.is_empty());
        assert_ne!(result_other, "Today");
        assert_ne!(result_other, "Yesterday");
    }

    #[test]
    fn test_format_absolute_time() {
        let timestamp = create_offset_datetime(1990, 4, 12, 9, 30, 0);

        // We can't assert the exact output as it depends on the platform and locale
        // But we can at least confirm it doesn't panic and returns a non-empty string
        let result = format_absolute_time(timestamp);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_format_absolute_date() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);

        // Test with same date (today)
        let timestamp_today = create_offset_datetime(1990, 4, 12, 9, 30, 0);
        assert_eq!(
            format_absolute_date(timestamp_today, reference, true),
            "Today"
        );

        // Test with previous day (yesterday)
        let timestamp_yesterday = create_offset_datetime(1990, 4, 11, 9, 30, 0);
        assert_eq!(
            format_absolute_date(timestamp_yesterday, reference, true),
            "Yesterday"
        );

        // Test with other date
        let timestamp_other = create_offset_datetime(1990, 4, 10, 9, 30, 0);
        let result = format_absolute_date(timestamp_other, reference, true);
        assert!(!result.is_empty());
        assert_ne!(result, "Today");
        assert_ne!(result, "Yesterday");
    }

    #[test]
    fn test_format_absolute_date_medium() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 12, 9, 30, 0);

        // Test with enhanced formatting (today)
        let result_enhanced = format_absolute_date_medium(timestamp, reference, true);
        assert_eq!(result_enhanced, "Today");

        // Test with standard formatting
        let result_standard = format_absolute_date_medium(timestamp, reference, false);
        assert!(!result_standard.is_empty());

        // Test yesterday with enhanced formatting
        let timestamp_yesterday = create_offset_datetime(1990, 4, 11, 9, 30, 0);
        let result_yesterday = format_absolute_date_medium(timestamp_yesterday, reference, true);
        assert_eq!(result_yesterday, "Yesterday");
    }

    #[test]
    fn test_format_timestamp_naive_time() {
        let timestamp = create_offset_datetime(1990, 4, 12, 9, 30, 0);
        assert_eq!(format_timestamp_naive_time(timestamp, true), "9:30 AM");
        assert_eq!(format_timestamp_naive_time(timestamp, false), "09:30");

        let timestamp_pm = create_offset_datetime(1990, 4, 12, 15, 45, 0);
        assert_eq!(format_timestamp_naive_time(timestamp_pm, true), "3:45 PM");
        assert_eq!(format_timestamp_naive_time(timestamp_pm, false), "15:45");
    }

    #[test]
    fn test_format_24_hour_time() {
        let reference = create_offset_datetime(1990, 4, 12, 16, 45, 0);
        let timestamp = create_offset_datetime(1990, 4, 12, 15, 30, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, false),
            "Today at 15:30"
        );
    }

    #[test]
    fn test_format_today() {
        let reference = create_offset_datetime(1990, 4, 12, 16, 45, 0);
        let timestamp = create_offset_datetime(1990, 4, 12, 15, 30, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "Today at 3:30 PM"
        );
    }

    #[test]
    fn test_format_yesterday() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 9, 0, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "Yesterday at 9:00 AM"
        );
    }

    #[test]
    fn test_format_yesterday_less_than_24_hours_ago() {
        let reference = create_offset_datetime(1990, 4, 12, 19, 59, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 20, 0, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "Yesterday at 8:00 PM"
        );
    }

    #[test]
    fn test_format_yesterday_more_than_24_hours_ago() {
        let reference = create_offset_datetime(1990, 4, 12, 19, 59, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 18, 0, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "Yesterday at 6:00 PM"
        );
    }

    #[test]
    fn test_format_yesterday_over_midnight() {
        let reference = create_offset_datetime(1990, 4, 12, 0, 5, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 23, 55, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "Yesterday at 11:55 PM"
        );
    }

    #[test]
    fn test_format_yesterday_over_month() {
        let reference = create_offset_datetime(1990, 4, 2, 9, 0, 0);
        let timestamp = create_offset_datetime(1990, 4, 1, 20, 0, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "Yesterday at 8:00 PM"
        );
    }

    #[test]
    fn test_format_before_yesterday() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 10, 20, 20, 0);

        assert_eq!(
            format_timestamp_naive(timestamp, reference, true),
            "04/10/1990 8:20 PM"
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
            "4 weeks ago".to_string()
        );

        for i in 2..=11 {
            assert_eq!(
                format_relative_date(next_month(), reference),
                format!("{} months ago", i)
            );
        }

        assert_eq!(format_relative_date(next_month(), reference), "1 year ago");
    }

    #[test]
    fn test_relative_format_years() {
        let reference = create_offset_datetime(1990, 4, 12, 23, 0, 0);

        // 12 months
        assert_eq!(
            format_relative_date(create_offset_datetime(1989, 4, 12, 23, 0, 0), reference),
            "1 year ago"
        );

        // 13 months
        assert_eq!(
            format_relative_date(create_offset_datetime(1989, 3, 12, 23, 0, 0), reference),
            "1 year ago"
        );

        // 23 months
        assert_eq!(
            format_relative_date(create_offset_datetime(1988, 5, 12, 23, 0, 0), reference),
            "1 year ago"
        );

        // 24 months
        assert_eq!(
            format_relative_date(create_offset_datetime(1988, 4, 12, 23, 0, 0), reference),
            "2 years ago"
        );

        // 25 months
        assert_eq!(
            format_relative_date(create_offset_datetime(1988, 3, 12, 23, 0, 0), reference),
            "2 years ago"
        );

        // 35 months
        assert_eq!(
            format_relative_date(create_offset_datetime(1987, 5, 12, 23, 0, 0), reference),
            "2 years ago"
        );

        // 36 months
        assert_eq!(
            format_relative_date(create_offset_datetime(1987, 4, 12, 23, 0, 0), reference),
            "3 years ago"
        );

        // 37 months
        assert_eq!(
            format_relative_date(create_offset_datetime(1987, 3, 12, 23, 0, 0), reference),
            "3 years ago"
        );

        // 120 months
        assert_eq!(
            format_relative_date(create_offset_datetime(1980, 4, 12, 23, 0, 0), reference),
            "10 years ago"
        );
    }

    #[test]
    fn test_calculate_month_difference() {
        let reference = create_offset_datetime(1990, 4, 12, 23, 0, 0);

        assert_eq!(calculate_month_difference(reference, reference), 0);

        assert_eq!(
            calculate_month_difference(create_offset_datetime(1990, 1, 12, 23, 0, 0), reference),
            3
        );

        assert_eq!(
            calculate_month_difference(create_offset_datetime(1989, 11, 12, 23, 0, 0), reference),
            5
        );

        assert_eq!(
            calculate_month_difference(create_offset_datetime(1989, 4, 12, 23, 0, 0), reference),
            12
        );

        assert_eq!(
            calculate_month_difference(create_offset_datetime(1989, 3, 12, 23, 0, 0), reference),
            13
        );

        assert_eq!(
            calculate_month_difference(create_offset_datetime(1987, 5, 12, 23, 0, 0), reference),
            35
        );

        assert_eq!(
            calculate_month_difference(create_offset_datetime(1987, 4, 12, 23, 0, 0), reference),
            36
        );

        assert_eq!(
            calculate_month_difference(create_offset_datetime(1987, 3, 12, 23, 0, 0), reference),
            37
        );
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
