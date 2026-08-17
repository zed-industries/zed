use crate::impl_type_with_repr;

impl_type_with_repr! {
    std::time::SystemTime => (u64, u32) {
        system_time {
            samples = [std::time::SystemTime::now()],
            repr(t) = {
                let since_epoch = t.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();
                (since_epoch.as_secs(), since_epoch.subsec_nanos())
            },
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Date => (i32, u16) {
        time_date {
            samples = [time::Date::MIN, time::Date::MAX, time::Date::from_calendar_date(2011, time::Month::June, 21).unwrap()],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L92
            repr(d) = (d.year(), d.ordinal()),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Duration => (i64, i32) {
        time_duration {
            samples = [time::Duration::MIN, time::Duration::MAX, time::Duration::new(42, 123456789)],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L119
            repr(d) = (d.whole_seconds(), d.subsec_nanoseconds()),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::OffsetDateTime => (i32, u16, u8, u8, u8, u32, i8, i8, i8) {
        time_offset_date_time {
            samples = [
                time::OffsetDateTime::now_utc(),
                time::OffsetDateTime::new_in_offset(
                    time::Date::from_calendar_date(2024, time::Month::May, 4).unwrap(),
                    time::Time::from_hms_nano(15, 32, 43, 2_000).unwrap(),
                    time::UtcOffset::from_hms(1, 2, 3).unwrap())
            ],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L155
            repr(d) = (
                d.year(),
                d.ordinal(),
                d.hour(),
                d.minute(),
                d.second(),
                d.nanosecond(),
                d.offset().whole_hours(),
                d.offset().minutes_past_hour(),
                d.offset().seconds_past_minute()
            ),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::PrimitiveDateTime => (i32, u16, u8, u8, u8, u32) {
        time_primitive_date_time {
            samples = [
                time::PrimitiveDateTime::MIN,
                time::PrimitiveDateTime::MAX,
                time::PrimitiveDateTime::new(
                    time::Date::from_calendar_date(2024, time::Month::May, 4).unwrap(),
                    time::Time::from_hms_nano(15, 32, 43, 2_000).unwrap())
            ],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L200
            repr(d) = (
                d.year(),
                d.ordinal(),
                d.hour(),
                d.minute(),
                d.second(),
                d.nanosecond()
            ),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Time => (u8, u8, u8, u32) {
        time_time {
            samples = [time::Time::MIDNIGHT, time::Time::from_hms(23, 42, 59).unwrap(), time::Time::from_hms_nano(15, 32, 43, 2_000).unwrap()],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L246
            repr(t) = (t.hour(), t.minute(), t.second(), t.nanosecond()),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::UtcOffset => (i8, i8, i8) {
        time_utc_offset {
            samples = [time::UtcOffset::UTC, time::UtcOffset::from_hms(1, 2, 3).unwrap()],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L282
            repr(offset) = (offset.whole_hours(), offset.minutes_past_hour(), offset.seconds_past_minute()),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Weekday => u8 {
        time_weekday {
            samples = [time::Weekday::Monday, time::Weekday::Wednesday, time::Weekday::Friday],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L312
            repr(weekday) = weekday.number_from_monday(),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Month => u8 {
        time_month {
            samples = [time::Month::January, time::Month::July, time::Month::December],
            // Serialized as month number:
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L337
            repr(month) = month as u8,
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::DateTime<Tz: chrono::TimeZone> => &str {
        chrono_date_time <Tz = chrono::offset::Utc> {
            samples = [chrono::DateTime::<Tz>::MIN_UTC, chrono::DateTime::<Tz>::MAX_UTC],
            repr(date) = &date.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::Month => &str {
        chrono_month {
            samples = [chrono::Month::January, chrono::Month::December],
            repr(month) = month.name(),
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::NaiveDate => &str {
        chrono_naive_date {
            samples = [chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap()],
            repr(d) = &format!("{d:?}"),
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::NaiveDateTime => &str {
        chrono_naive_date_time {
            samples = [chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap()],
            repr(dt) = &format!("{dt:?}"),
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::NaiveTime => &str {
        chrono_naive_time {
            samples = [chrono::NaiveTime::from_hms_opt(9, 10, 11).unwrap()],
            repr(t) = &format!("{t:?}"),
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::Weekday => &str {
        chrono_weekday {
            samples = [chrono::Weekday::Mon, chrono::Weekday::Fri],
            // Serialized as the weekday's name.
            repr(weekday) = &weekday.to_string(),
        }
    }
}
