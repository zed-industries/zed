use lazy_static::lazy_static;
use time::{OffsetDateTime, UtcOffset};

/// Formats a timestamp, which respects the user's date and time preferences/custom format.
pub fn format_localized_timestamp(
    reference: OffsetDateTime,
    timestamp: OffsetDateTime,
    timezone: UtcOffset,
) -> String {
    #[cfg(target_os = "macos")]
    {
        let timestamp_local = timestamp.to_offset(timezone);
        let reference_local = reference.to_offset(timezone);
        let reference_local_date = reference_local.date();
        let timestamp_local_date = timestamp_local.date();

        let native_fmt = if timestamp_local_date == reference_local_date {
            macos::format_time(&timestamp)
        } else if reference_local_date.previous_day() == Some(timestamp_local_date) {
            macos::format_time(&timestamp).map(|t| format!("yesterday at {}", t).to_string())
        } else {
            macos::format_date(&timestamp)
        };
        native_fmt.unwrap_or_else(|_| format_timestamp_fallback(reference, timestamp, timezone))
    }
    #[cfg(not(target_os = "macos"))]
    {
        //todo!(linux) respect user's date/time preferences
        //todo!(windows) respect user's date/time preferences
        format_timestamp_fallback(reference, timestamp, timezone)
    }
}

fn format_timestamp_fallback(
    reference: OffsetDateTime,
    timestamp: OffsetDateTime,
    timezone: UtcOffset,
) -> String {
    lazy_static! {
        static ref CURRENT_LOCALE: String =
            sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    }
    let is_12_hour_time = is_12_hour_time_by_locale(CURRENT_LOCALE.as_str());
    format_timestamp_naive(reference, timestamp, timezone, is_12_hour_time)
}

/// Formats a timestamp, which is either in 12-hour or 24-hour time format.
/// Note:
/// This function does not respect the user's date and time preferences.
/// This should only be used as a fallback mechanism when the os time formatting fails.
pub fn format_timestamp_naive(
    reference: OffsetDateTime,
    timestamp: OffsetDateTime,
    timezone: UtcOffset,
    is_12_hour_time: bool,
) -> String {
    let timestamp_local = timestamp.to_offset(timezone);
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

    let reference_local = reference.to_offset(timezone);
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

/// Returns true if the locale is recognized as a 12-hour time locale.
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
    use anyhow::Result;
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

    pub fn format_time(timestamp: &time::OffsetDateTime) -> Result<String> {
        format_with_date_formatter(timestamp, TIME_FORMATTER.with(|f| *f))
    }

    pub fn format_date(timestamp: &time::OffsetDateTime) -> Result<String> {
        format_with_date_formatter(timestamp, DATE_FORMATTER.with(|f| *f))
    }

    fn format_with_date_formatter(
        timestamp: &time::OffsetDateTime,
        fmt: CFDateFormatterRef,
    ) -> Result<String> {
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
            Ok(CFString::wrap_under_create_rule(s).to_string())
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

        assert_eq!(
            format_timestamp_naive(reference, timestamp, test_timezone(), false),
            "15:30"
        );
    }

    #[test]
    fn test_format_today() {
        let reference = create_offset_datetime(1990, 4, 12, 16, 45, 0);
        let timestamp = create_offset_datetime(1990, 4, 12, 15, 30, 0);

        assert_eq!(
            format_timestamp_naive(reference, timestamp, test_timezone(), true),
            "03:30 pm"
        );
    }

    #[test]
    fn test_format_yesterday() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 9, 0, 0);

        assert_eq!(
            format_timestamp_naive(reference, timestamp, test_timezone(), true),
            "yesterday at 09:00 am"
        );
    }

    #[test]
    fn test_format_yesterday_less_than_24_hours_ago() {
        let reference = create_offset_datetime(1990, 4, 12, 19, 59, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 20, 0, 0);

        assert_eq!(
            format_timestamp_naive(reference, timestamp, test_timezone(), true),
            "yesterday at 08:00 pm"
        );
    }

    #[test]
    fn test_format_yesterday_more_than_24_hours_ago() {
        let reference = create_offset_datetime(1990, 4, 12, 19, 59, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 18, 0, 0);

        assert_eq!(
            format_timestamp_naive(reference, timestamp, test_timezone(), true),
            "yesterday at 06:00 pm"
        );
    }

    #[test]
    fn test_format_yesterday_over_midnight() {
        let reference = create_offset_datetime(1990, 4, 12, 0, 5, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 23, 55, 0);

        assert_eq!(
            format_timestamp_naive(reference, timestamp, test_timezone(), true),
            "yesterday at 11:55 pm"
        );
    }

    #[test]
    fn test_format_yesterday_over_month() {
        let reference = create_offset_datetime(1990, 4, 2, 9, 0, 0);
        let timestamp = create_offset_datetime(1990, 4, 1, 20, 0, 0);

        assert_eq!(
            format_timestamp_naive(reference, timestamp, test_timezone(), true),
            "yesterday at 08:00 pm"
        );
    }

    #[test]
    fn test_format_before_yesterday() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 10, 20, 20, 0);

        assert_eq!(
            format_timestamp_naive(reference, timestamp, test_timezone(), true),
            "04/10/1990"
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
        date.with_time(time).assume_utc() // Assume UTC for simplicity
    }
}
