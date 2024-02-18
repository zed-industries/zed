use anyhow::Result;
use lazy_static::lazy_static;
use time::{OffsetDateTime, UtcOffset};

/// Formats a timestamp, which respects the user's 12-hour clock preference/current locale.
pub fn format_localized_timestamp(
    reference: OffsetDateTime,
    timestamp: OffsetDateTime,
    timezone: UtcOffset,
) -> String {
    format_timestamp(reference, timestamp, timezone, is_12_hour_clock())
}

/// Formats a timestamp, which is either in 12-hour or 24-hour clock format.
pub fn format_timestamp(
    reference: OffsetDateTime,
    timestamp: OffsetDateTime,
    timezone: UtcOffset,
    is_12_hour_clock: bool,
) -> String {
    let timestamp_local = timestamp.to_offset(timezone);
    let timestamp_local_hour = timestamp_local.hour();
    let timestamp_local_minute = timestamp_local.minute();

    let (hour, meridiem) = if is_12_hour_clock {
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

/// Returns true if the users configuration is set to 12-hour clock,
/// or if the locale is recognized as a 12-hour clock locale.
pub fn is_12_hour_clock() -> bool {
    IS_12_HOUR_CLOCK_USER_PREFERENCE
        .as_ref()
        .map(|c| *c)
        .unwrap_or_else(|_| is_12_hour_clock_by_locale(CURRENT_LOCALE.as_str()))
}

/// Returns true if the locale is recognized as a 12-hour clock locale.
pub fn is_12_hour_clock_by_locale(locale: &str) -> bool {
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

lazy_static! {
    static ref CURRENT_LOCALE: String =
        sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    static ref IS_12_HOUR_CLOCK_USER_PREFERENCE: Result<bool> =
        read_is_12_hour_clock_from_user_preferences();
}

fn read_is_12_hour_clock_from_user_preferences() -> Result<bool> {
    #[cfg(target_os = "macos")]
    {
        let mut file_path = util::paths::HOME.clone();
        file_path.push("Library");
        file_path.push("Preferences");
        file_path.push(".GlobalPreferences.plist");

        let value: plist::Value = plist::from_file(file_path.as_path())?;
        if let Some(root_dict) = value.as_dictionary() {
            if let Some(plist::Value::Boolean(force_12_hour_format)) =
                root_dict.get("AppleICUForce12HourTime")
            {
                return Ok(*force_12_hour_format);
            }
        }
        Ok(false)
    }
    #[cfg(not(target_os = "macos"))]
    {
        //todo!(linux)
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_24_hour_clock() {
        let reference = create_offset_datetime(1990, 4, 12, 16, 45, 0);
        let timestamp = create_offset_datetime(1990, 4, 12, 15, 30, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone(), false),
            "15:30"
        );
    }

    #[test]
    fn test_format_today() {
        let reference = create_offset_datetime(1990, 4, 12, 16, 45, 0);
        let timestamp = create_offset_datetime(1990, 4, 12, 15, 30, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone(), true),
            "03:30 pm"
        );
    }

    #[test]
    fn test_format_yesterday() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 9, 0, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone(), true),
            "yesterday at 09:00 am"
        );
    }

    #[test]
    fn test_format_yesterday_less_than_24_hours_ago() {
        let reference = create_offset_datetime(1990, 4, 12, 19, 59, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 20, 0, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone(), true),
            "yesterday at 08:00 pm"
        );
    }

    #[test]
    fn test_format_yesterday_more_than_24_hours_ago() {
        let reference = create_offset_datetime(1990, 4, 12, 19, 59, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 18, 0, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone(), true),
            "yesterday at 06:00 pm"
        );
    }

    #[test]
    fn test_format_yesterday_over_midnight() {
        let reference = create_offset_datetime(1990, 4, 12, 0, 5, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 23, 55, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone(), true),
            "yesterday at 11:55 pm"
        );
    }

    #[test]
    fn test_format_yesterday_over_month() {
        let reference = create_offset_datetime(1990, 4, 2, 9, 0, 0);
        let timestamp = create_offset_datetime(1990, 4, 1, 20, 0, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone(), true),
            "yesterday at 08:00 pm"
        );
    }

    #[test]
    fn test_format_before_yesterday() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 10, 20, 20, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone(), true),
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
