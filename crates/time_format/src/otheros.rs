use icu::calendar::Date as IcuDate;
use icu::datetime::{DateTimeFormatter, fieldsets};
use icu::locale::{Locale, locale};
use icu::time::Time as IcuTime;
use time::OffsetDateTime;

#[cfg(not(all(unix, not(any(target_vendor = "apple", target_os = "android")))))]
use sys_locale;

static CURRENT_LOCALE: std::sync::OnceLock<Locale> = std::sync::OnceLock::new();

// Test-only locale override - allows setting a fixed locale for testing
#[cfg(test)]
static TEST_LOCALE_OVERRIDE: std::sync::OnceLock<Locale> = std::sync::OnceLock::new();

#[cfg(test)]
pub fn set_test_locale(locale_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_locale = locale_str.parse::<Locale>()?;
    // Use set() to ensure it's only set once for the entire test run
    let _ = TEST_LOCALE_OVERRIDE.set(parsed_locale);
    Ok(())
}

#[cfg(all(unix, not(any(target_vendor = "apple", target_os = "android"))))]
fn posix_to_bcp47(locale: &str) -> String {
    if locale.is_empty() {
        return String::new();
    }

    // Find the end of the language/region part
    let end = locale
        .find('.')
        .or_else(|| locale.find('@'))
        .unwrap_or(locale.len());
    let lang_region = &locale[..end];

    lang_region.replace('_', "-")
}

// Unix-specific locale detection using environment variables
#[cfg(all(unix, not(any(target_vendor = "apple", target_os = "android"))))]
fn get_locale_string_unix() -> String {
    // Priority: LC_TIME > LC_ALL > LANG > default
    let locale_str = std::env::var("LC_TIME")
        .or_else(|_| std::env::var("LC_ALL"))
        .or_else(|_| std::env::var("LANG"))
        .unwrap_or_else(|_| "en-US".to_string());

    posix_to_bcp47(&locale_str)
}

// Non-Unix platform locale detection using sys_locale
#[cfg(not(all(unix, not(any(target_vendor = "apple", target_os = "android")))))]
fn get_locale_string_non_unix() -> String {
    let sys_locale_result = sys_locale::get_locale();
    let locale_str = sys_locale_result.unwrap_or_else(|| "en-US".to_string());

    locale_str
}

pub fn get_locale() -> &'static Locale {
    // Check for test override first (only available in test builds)
    #[cfg(test)]
    {
        if let Some(test_locale) = TEST_LOCALE_OVERRIDE.get() {
            return test_locale;
        }
    }

    CURRENT_LOCALE.get_or_init(|| {
        // Use platform-specific locale detection
        #[cfg(all(unix, not(any(target_vendor = "apple", target_os = "android"))))]
        let locale_str = get_locale_string_unix();

        #[cfg(not(all(unix, not(any(target_vendor = "apple", target_os = "android")))))]
        let locale_str = get_locale_string_non_unix();

        // Try to parse with ICU and log the result
        match locale_str.parse::<Locale>() {
            Ok(parsed_locale) => parsed_locale,
            Err(_) => {
                let fallback_locale = locale!("en-US");
                fallback_locale
            }
        }
    })
}

pub fn convert_to_icu_date(timestamp: OffsetDateTime) -> IcuDate<icu::calendar::Iso> {
    IcuDate::try_new_iso(timestamp.year(), timestamp.month() as u8, timestamp.day()).unwrap()
}

pub fn convert_to_icu_time(timestamp: OffsetDateTime) -> IcuTime {
    IcuTime::try_new(timestamp.hour(), timestamp.minute(), timestamp.second(), 0).unwrap()
}

#[allow(dead_code)]
pub fn format_timestamp_with_icu(timestamp: OffsetDateTime, reference: OffsetDateTime) -> String {
    let locale = get_locale();
    let icu_time = convert_to_icu_time(timestamp);
    let icu_date = convert_to_icu_date(timestamp);

    let timestamp_date = timestamp.date();
    let reference_date = reference.date();

    if timestamp_date == reference_date {
        let time_formatter =
            DateTimeFormatter::try_new(locale.into(), fieldsets::T::short()).unwrap();
        format!("Today at {}", time_formatter.format(&icu_time).to_string())
    } else if reference_date.previous_day() == Some(timestamp_date) {
        let time_formatter =
            DateTimeFormatter::try_new(locale.into(), fieldsets::T::short()).unwrap();
        format!(
            "Yesterday at {}",
            time_formatter.format(&icu_time).to_string()
        )
    } else {
        let datetime_formatter =
            DateTimeFormatter::try_new(locale.into(), fieldsets::YMD::medium()).unwrap();
        datetime_formatter.format(&icu_date).to_string()
    }
}

#[allow(dead_code)]
pub fn format_timestamp_fallback(timestamp: OffsetDateTime, reference: OffsetDateTime) -> String {
    format_timestamp_with_icu(timestamp, reference)
}

pub fn format_timestamp_naive_date(
    timestamp_local: OffsetDateTime,
    reference_local: OffsetDateTime,
) -> String {
    let locale = get_locale();
    let icu_date = convert_to_icu_date(timestamp_local);
    let reference_local_date = reference_local.date();
    let timestamp_local_date = timestamp_local.date();

    if timestamp_local_date == reference_local_date {
        "Today".to_string()
    } else if reference_local_date.previous_day() == Some(timestamp_local_date) {
        "Yesterday".to_string()
    } else {
        let date_formatter =
            DateTimeFormatter::try_new(locale.into(), fieldsets::YMD::medium()).unwrap();
        date_formatter.format(&icu_date).to_string()
    }
}

pub fn format_timestamp_naive_date_medium(timestamp_local: OffsetDateTime) -> String {
    let locale = get_locale();
    let icu_date = convert_to_icu_date(timestamp_local);

    let date_formatter =
        DateTimeFormatter::try_new(locale.into(), fieldsets::YMD::medium()).unwrap();
    date_formatter.format(&icu_date).to_string()
}

pub fn format_timestamp_naive_time(timestamp_local: OffsetDateTime) -> String {
    let locale = get_locale();
    let icu_time = convert_to_icu_time(timestamp_local);

    let time_formatter = DateTimeFormatter::try_new(locale.into(), fieldsets::T::short()).unwrap();
    time_formatter.format(&icu_time).to_string()
}

pub fn format_absolute_timestamp(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
    enhanced_date_formatting: bool,
) -> String {
    if !enhanced_date_formatting {
        return format!(
            "{} {}",
            format_timestamp_naive_date_medium(timestamp),
            format_timestamp_naive_time(timestamp)
        );
    }

    let timestamp_date = timestamp.date();
    let reference_date = reference.date();
    if timestamp_date == reference_date {
        format!("Today at {}", format_timestamp_naive_time(timestamp))
    } else if reference_date.previous_day() == Some(timestamp_date) {
        format!("Yesterday at {}", format_timestamp_naive_time(timestamp))
    } else {
        format!(
            "{} {}",
            format_timestamp_naive_date(timestamp, reference),
            format_timestamp_naive_time(timestamp)
        )
    }
}

pub fn format_absolute_timestamp_medium(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
) -> String {
    let timestamp_date = timestamp.date();
    let reference_date = reference.date();
    if timestamp_date == reference_date {
        "Today".to_string()
    } else if reference_date.previous_day() == Some(timestamp_date) {
        "Yesterday".to_string()
    } else {
        format_timestamp_naive_date_medium(timestamp)
    }
}

#[allow(dead_code)]
pub fn format_timestamp_naive(
    timestamp_local: OffsetDateTime,
    reference_local: OffsetDateTime,
) -> String {
    let formatted_time = format_timestamp_naive_time(timestamp_local);
    let reference_local_date = reference_local.date();
    let timestamp_local_date = timestamp_local.date();

    if timestamp_local_date == reference_local_date {
        format!("Today at {}", formatted_time)
    } else if reference_local_date.previous_day() == Some(timestamp_local_date) {
        format!("Yesterday at {}", formatted_time)
    } else {
        let locale = get_locale();
        let icu_date = convert_to_icu_date(timestamp_local);
        let date_formatter =
            DateTimeFormatter::try_new(locale.into(), fieldsets::YMD::medium()).unwrap();
        let formatted_date = date_formatter.format(&icu_date).to_string();
        format!("{} {}", formatted_date, formatted_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Date, Month, Time};

    fn create_offset_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> OffsetDateTime {
        let date = Date::from_calendar_date(year, Month::try_from(month).unwrap(), day).unwrap();
        let time = Time::from_hms(hour, minute, second).unwrap();
        let date = date.with_time(time).assume_utc();
        date.to_offset(time::UtcOffset::from_hms(0, 0, 0).unwrap())
    }

    #[test]
    fn test_format_absolute_timestamp_basic() {
        let timestamp = create_offset_datetime(2024, 1, 15, 14, 30, 0);
        let reference = create_offset_datetime(2024, 1, 15, 16, 0, 0);

        let result = format_absolute_timestamp(timestamp, reference, false);

        // Should include both date and time
        assert!(result.contains("14:30") || result.contains("2:30"));
        assert!(!result.is_empty());
    }

    #[test]
    fn test_format_absolute_timestamp_enhanced_today() {
        let timestamp = create_offset_datetime(2024, 1, 15, 14, 30, 0);
        let reference = create_offset_datetime(2024, 1, 15, 16, 0, 0);

        let result = format_absolute_timestamp(timestamp, reference, true);

        // Should say "Today at" with time
        assert!(result.starts_with("Today at"));
        assert!(result.contains("14:30") || result.contains("2:30"));
    }

    #[test]
    fn test_format_absolute_timestamp_enhanced_yesterday() {
        let timestamp = create_offset_datetime(2024, 1, 14, 14, 30, 0);
        let reference = create_offset_datetime(2024, 1, 15, 16, 0, 0);

        let result = format_absolute_timestamp(timestamp, reference, true);

        // Should say "Yesterday at" with time
        assert!(result.starts_with("Yesterday at"));
        assert!(result.contains("14:30") || result.contains("2:30"));
    }

    #[test]
    fn test_format_absolute_timestamp_enhanced_other_date() {
        let timestamp = create_offset_datetime(2024, 1, 10, 14, 30, 0);
        let reference = create_offset_datetime(2024, 1, 15, 16, 0, 0);

        let result = format_absolute_timestamp(timestamp, reference, true);

        // Should include both date and time, not "Today" or "Yesterday"
        assert!(!result.contains("Today"));
        assert!(!result.contains("Yesterday"));
        assert!(result.contains("14:30") || result.contains("2:30"));
        assert!(!result.is_empty());
    }

    #[test]
    fn test_format_absolute_timestamp_medium() {
        let timestamp = create_offset_datetime(2024, 1, 15, 14, 30, 0);
        let reference = create_offset_datetime(2024, 1, 15, 16, 0, 0);

        let result = format_absolute_timestamp_medium(timestamp, reference);

        // Should return a date string in medium format
        assert!(!result.is_empty());
        // Should not contain time for medium format
        assert!(!result.contains("14:30") && !result.contains("2:30"));
    }
}
