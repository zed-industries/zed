use std::cmp::Ordering;
use std::time::{Duration as StdDuration, SystemTime};

use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::{date, datetime, offset, time, utc_datetime};
use time::{Duration, Month, OffsetDateTime, UtcDateTime, Weekday};

#[test]
fn new() {
    let dt = UtcDateTime::new(date!(2023-12-18), time!(10:13:44.250 AM));
    assert_eq!(dt.year(), 2023);
    assert_eq!(dt.millisecond(), 250);
}

#[test]
fn now() {
    assert!(UtcDateTime::now().year() >= 2019);
}

#[test]
fn to_offset() {
    assert_eq!(
        utc_datetime!(2000-01-01 0:00).to_offset(offset!(-1)),
        datetime!(1999-12-31 23:00 -1),
    );
    assert_eq!(
        utc_datetime!(0000-001 0:00).to_offset(offset!(UTC)),
        datetime!(0000-001 0:00 UTC),
    );
}

#[test]
fn to_offset_panic() {
    assert_panic!(UtcDateTime::MAX.to_offset(offset!(+1)));
    assert_panic!(UtcDateTime::MIN.to_offset(offset!(-1)));
}

#[test]
fn checked_to_offset() {
    assert_eq!(
        utc_datetime!(2000-01-01 0:00)
            .checked_to_offset(offset!(-1))
            .map(|odt| odt.year()),
        Some(1999),
    );
    assert_eq!(UtcDateTime::MAX.checked_to_offset(offset!(+1)), None);
    assert_eq!(UtcDateTime::MIN.checked_to_offset(offset!(-1)), None);
}

#[test]
fn from_unix_timestamp() {
    assert_eq!(
        UtcDateTime::from_unix_timestamp(0),
        Ok(UtcDateTime::UNIX_EPOCH),
    );
    assert_eq!(
        UtcDateTime::from_unix_timestamp(1_546_300_800),
        Ok(utc_datetime!(2019-01-01 0:00)),
    );
}

#[test]
fn from_unix_timestamp_nanos() {
    assert_eq!(
        UtcDateTime::from_unix_timestamp_nanos(0),
        Ok(UtcDateTime::UNIX_EPOCH),
    );
    assert_eq!(
        UtcDateTime::from_unix_timestamp_nanos(1_546_300_800_000_000_000),
        Ok(utc_datetime!(2019-01-01 0:00)),
    );
    assert!(UtcDateTime::from_unix_timestamp_nanos(i128::MAX).is_err());
}

#[test]
fn unix_timestamp() {
    assert_eq!(UtcDateTime::UNIX_EPOCH.unix_timestamp(), 0);
}

#[test]
fn unix_timestamp_nanos() {
    assert_eq!(UtcDateTime::UNIX_EPOCH.unix_timestamp_nanos(), 0);
}

#[test]
fn date() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).date(), date!(2019-01-01));
}

#[test]
fn time() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).time(), time!(0:00));
}

#[test]
fn year() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).year(), 2019);
}

#[test]
fn month() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).month(), Month::January);
}

#[test]
fn day() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).day(), 1);
}

#[test]
fn ordinal() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).ordinal(), 1);
}

#[test]
fn iso_week() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).iso_week(), 1);
    assert_eq!(utc_datetime!(2020-01-01 0:00).iso_week(), 1);
    assert_eq!(utc_datetime!(2020-12-31 0:00).iso_week(), 53);
    assert_eq!(utc_datetime!(2021-01-01 0:00).iso_week(), 53);
}

#[test]
fn sunday_based_week() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).sunday_based_week(), 0);
    assert_eq!(utc_datetime!(2020-01-01 0:00).sunday_based_week(), 0);
    assert_eq!(utc_datetime!(2020-12-31 0:00).sunday_based_week(), 52);
    assert_eq!(utc_datetime!(2021-01-01 0:00).sunday_based_week(), 0);
}

#[test]
fn monday_based_week() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).monday_based_week(), 0);
    assert_eq!(utc_datetime!(2020-01-01 0:00).monday_based_week(), 0);
    assert_eq!(utc_datetime!(2020-12-31 0:00).monday_based_week(), 52);
    assert_eq!(utc_datetime!(2021-01-01 0:00).monday_based_week(), 0);
}

#[test]
fn to_calendar_date() {
    assert_eq!(
        utc_datetime!(2019-01-02 0:00).to_calendar_date(),
        (2019, Month::January, 2)
    );
}

#[test]
fn to_ordinal_date() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).to_ordinal_date(), (2019, 1));
}

#[test]
fn to_iso_week_date() {
    use Weekday::*;
    assert_eq!(
        utc_datetime!(2019-01-01 0:00).to_iso_week_date(),
        (2019, 1, Tuesday)
    );
    assert_eq!(
        utc_datetime!(2019-10-04 0:00).to_iso_week_date(),
        (2019, 40, Friday)
    );
    assert_eq!(
        utc_datetime!(2020-01-01 0:00).to_iso_week_date(),
        (2020, 1, Wednesday)
    );
    assert_eq!(
        utc_datetime!(2020-12-31 0:00).to_iso_week_date(),
        (2020, 53, Thursday)
    );
    assert_eq!(
        utc_datetime!(2021-01-01 0:00).to_iso_week_date(),
        (2020, 53, Friday)
    );
}

#[test]
fn weekday() {
    use Weekday::*;
    assert_eq!(utc_datetime!(2019-01-01 0:00).weekday(), Tuesday);
    assert_eq!(utc_datetime!(2019-02-01 0:00).weekday(), Friday);
    assert_eq!(utc_datetime!(2019-03-01 0:00).weekday(), Friday);
}

#[test]
fn to_julian_day() {
    assert_eq!(
        utc_datetime!(-999_999-01-01 0:00).to_julian_day(),
        -363_521_074
    );
    assert_eq!(utc_datetime!(-4713-11-24 0:00).to_julian_day(), 0);
    assert_eq!(utc_datetime!(2000-01-01 0:00).to_julian_day(), 2_451_545);
    assert_eq!(utc_datetime!(2019-01-01 0:00).to_julian_day(), 2_458_485);
    assert_eq!(utc_datetime!(2019-12-31 0:00).to_julian_day(), 2_458_849);
}

#[test]
fn as_hms() {
    assert_eq!(utc_datetime!(2020-01-01 1:02:03).as_hms(), (1, 2, 3));
}

#[test]
fn as_hms_milli() {
    assert_eq!(
        utc_datetime!(2020-01-01 1:02:03.004).as_hms_milli(),
        (1, 2, 3, 4)
    );
}

#[test]
fn as_hms_micro() {
    assert_eq!(
        utc_datetime!(2020-01-01 1:02:03.004_005).as_hms_micro(),
        (1, 2, 3, 4_005)
    );
}

#[test]
fn as_hms_nano() {
    assert_eq!(
        utc_datetime!(2020-01-01 1:02:03.004_005_006).as_hms_nano(),
        (1, 2, 3, 4_005_006)
    );
}

#[test]
fn hour() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).hour(), 0);
}

#[test]
fn minute() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).minute(), 0);
}

#[test]
fn second() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).second(), 0);
}

#[test]
fn millisecond() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).millisecond(), 0);
    assert_eq!(utc_datetime!(2019-01-01 23:59:59.999).millisecond(), 999);
}

#[test]
fn microsecond() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).microsecond(), 0);
    assert_eq!(
        utc_datetime!(2019-01-01 23:59:59.999_999).microsecond(),
        999_999,
    );
}

#[test]
fn nanosecond() {
    assert_eq!(utc_datetime!(2019-01-01 0:00).nanosecond(), 0);
    assert_eq!(
        utc_datetime!(2019-01-01 23:59:59.999_999_999).nanosecond(),
        999_999_999,
    );
}

#[test]
fn replace_time() {
    assert_eq!(
        utc_datetime!(2020-01-01 5:00).replace_time(time!(12:00)),
        utc_datetime!(2020-01-01 12:00)
    );
}

#[test]
fn replace_date() {
    assert_eq!(
        utc_datetime!(2020-01-01 12:00).replace_date(date!(2020-01-30)),
        utc_datetime!(2020-01-30 12:00)
    );
}

#[test]
fn replace_year() {
    assert_eq!(
        utc_datetime!(2022-02-18 12:00).replace_year(2019),
        Ok(utc_datetime!(2019-02-18 12:00))
    );
    assert!(
        utc_datetime!(2022-02-18 12:00)
            .replace_year(-1_000_000_000)
            .is_err()
    ); // -1_000_000_000 isn't a valid year
    assert!(
        utc_datetime!(2022-02-18 12:00)
            .replace_year(1_000_000_000)
            .is_err()
    ); // 1_000_000_000 isn't a valid year
}

#[test]
fn replace_month() {
    assert_eq!(
        utc_datetime!(2022-02-18 12:00).replace_month(Month::January),
        Ok(utc_datetime!(2022-01-18 12:00))
    );
    assert!(
        utc_datetime!(2022-01-30 12:00)
            .replace_month(Month::February)
            .is_err()
    ); // 30 isn't a valid day in February
}

#[test]
fn replace_day() {
    assert_eq!(
        utc_datetime!(2022-02-18 12:00).replace_day(1),
        Ok(utc_datetime!(2022-02-01 12:00))
    );
    // 00 isn't a valid day
    assert!(utc_datetime!(2022-02-18 12:00).replace_day(0).is_err());
    // 30 isn't a valid day in February
    assert!(utc_datetime!(2022-02-18 12:00).replace_day(30).is_err());
}

#[test]
fn replace_ordinal() {
    assert_eq!(
        utc_datetime!(2022-02-18 12:00).replace_ordinal(1),
        Ok(utc_datetime!(2022-001 12:00))
    );
    assert_eq!(
        utc_datetime!(2024-02-29 12:00).replace_ordinal(366),
        Ok(utc_datetime!(2024-366 12:00))
    );
    assert!(utc_datetime!(2022-049 12:00).replace_ordinal(0).is_err()); // 0 isn't a valid day
    assert!(utc_datetime!(2022-049 12:00).replace_ordinal(366).is_err()); // 2022 isn't a leap year
    assert!(utc_datetime!(2022-049 12:00).replace_ordinal(367).is_err()); // 367 isn't a valid day
}

#[test]
fn replace_hour() {
    assert_eq!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006).replace_hour(7),
        Ok(utc_datetime!(2022-02-18 07:02:03.004_005_006))
    );
    assert!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_hour(24)
            .is_err()
    ); // 24 isn't a valid hour
}

#[test]
fn replace_minute() {
    assert_eq!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006).replace_minute(7),
        Ok(utc_datetime!(2022-02-18 01:07:03.004_005_006))
    );
    assert!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_minute(60)
            .is_err()
    ); // 60 isn't a valid minute
}

#[test]
fn replace_second() {
    assert_eq!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006).replace_second(7),
        Ok(utc_datetime!(2022-02-18 01:02:07.004_005_006))
    );
    assert!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_second(60)
            .is_err()
    ); // 60 isn't a valid second
}

#[test]
fn replace_millisecond() {
    assert_eq!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006).replace_millisecond(7),
        Ok(utc_datetime!(2022-02-18 01:02:03.007))
    );
    assert!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_millisecond(1_000)
            .is_err()
    ); // 1_000 isn't a valid millisecond
}

#[test]
fn replace_microsecond() {
    assert_eq!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006).replace_microsecond(7_008),
        Ok(utc_datetime!(2022-02-18 01:02:03.007_008))
    );
    assert!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_microsecond(1_000_000)
            .is_err()
    ); // 1_000_000 isn't a valid microsecond
}

#[test]
fn replace_nanosecond() {
    assert_eq!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006).replace_nanosecond(7_008_009),
        Ok(utc_datetime!(2022-02-18 01:02:03.007_008_009))
    );
    assert!(
        utc_datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_nanosecond(1_000_000_000)
            .is_err()
    ); // 1_000_000_000 isn't a valid nanosecond
}

#[test]
fn truncate_to_day() {
    assert_eq!(
        utc_datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_day(),
        utc_datetime!(2021-11-12 0:00)
    );
    assert_eq!(
        utc_datetime!(2021-11-12 0:00).truncate_to_day(),
        utc_datetime!(2021-11-12 0:00)
    );
}

#[test]
fn truncate_to_hour() {
    assert_eq!(
        utc_datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_hour(),
        utc_datetime!(2021-11-12 17:00)
    );
    assert_eq!(
        utc_datetime!(2021-11-12 0:00).truncate_to_hour(),
        utc_datetime!(2021-11-12 0:00)
    );
}

#[test]
fn truncate_to_minute() {
    assert_eq!(
        utc_datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_minute(),
        utc_datetime!(2021-11-12 17:47)
    );
    assert_eq!(
        utc_datetime!(2021-11-12 0:00).truncate_to_minute(),
        utc_datetime!(2021-11-12 0:00)
    );
}

#[test]
fn truncate_to_second() {
    assert_eq!(
        utc_datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_second(),
        utc_datetime!(2021-11-12 17:47:53)
    );
    assert_eq!(
        utc_datetime!(2021-11-12 0:00).truncate_to_second(),
        utc_datetime!(2021-11-12 0:00)
    );
}

#[test]
fn truncate_to_millisecond() {
    assert_eq!(
        utc_datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_millisecond(),
        utc_datetime!(2021-11-12 17:47:53.123)
    );
    assert_eq!(
        utc_datetime!(2021-11-12 0:00).truncate_to_millisecond(),
        utc_datetime!(2021-11-12 0:00)
    );
}

#[test]
fn truncate_to_microsecond() {
    assert_eq!(
        utc_datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_microsecond(),
        utc_datetime!(2021-11-12 17:47:53.123_456)
    );
    assert_eq!(
        utc_datetime!(2021-11-12 0:00).truncate_to_microsecond(),
        utc_datetime!(2021-11-12 0:00)
    );
}

#[test]
fn partial_eq() {
    assert_eq!(
        utc_datetime!(2000-01-01 0:00),
        utc_datetime!(2000-01-01 0:00),
    );
}

#[test]
fn partial_ord() {
    let t1 = utc_datetime!(2019-01-01 0:00);
    let t2 = utc_datetime!(2019-01-01 0:00);
    assert_eq!(t1.partial_cmp(&t2), Some(Ordering::Equal));
}

#[test]
fn ord() {
    let t1 = utc_datetime!(2019-01-01 0:00);
    let t2 = utc_datetime!(2019-01-01 0:00);
    assert_eq!(t1, t2);

    let t1 = utc_datetime!(2019-01-01 0:00);
    let t2 = utc_datetime!(2019-01-01 0:00:00.000_000_001);
    assert!(t2 > t1);
}

#[test]
fn hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    assert_eq!(
        {
            let mut hasher = DefaultHasher::new();
            utc_datetime!(2019-01-01 0:00).hash(&mut hasher);
            hasher.finish()
        },
        {
            let mut hasher = DefaultHasher::new();
            utc_datetime!(2019-01-01 0:00).hash(&mut hasher);
            hasher.finish()
        }
    );
}

#[test]
fn add_duration() {
    assert_eq!(
        utc_datetime!(2019-01-01 0:00) + 5.days(),
        utc_datetime!(2019-01-06 0:00),
    );
    assert_eq!(
        utc_datetime!(2019-12-31 0:00) + 1.days(),
        utc_datetime!(2020-01-01 0:00),
    );
    assert_eq!(
        utc_datetime!(2019-12-31 23:59:59) + 2.seconds(),
        utc_datetime!(2020-01-01 0:00:01),
    );
    assert_eq!(
        utc_datetime!(2020-01-01 0:00:01) + (-2).seconds(),
        utc_datetime!(2019-12-31 23:59:59),
    );
    assert_eq!(
        utc_datetime!(1999-12-31 23:00) + 1.hours(),
        utc_datetime!(2000-01-01 0:00),
    );
}

#[test]
fn add_std_duration() {
    assert_eq!(
        utc_datetime!(2019-01-01 0:00) + 5.std_days(),
        utc_datetime!(2019-01-06 0:00),
    );
    assert_eq!(
        utc_datetime!(2019-12-31 0:00) + 1.std_days(),
        utc_datetime!(2020-01-01 0:00),
    );
    assert_eq!(
        utc_datetime!(2019-12-31 23:59:59) + 2.std_seconds(),
        utc_datetime!(2020-01-01 0:00:01),
    );
}

#[test]
fn add_assign_duration() {
    let mut new_years_day_2019 = utc_datetime!(2019-01-01 0:00);
    new_years_day_2019 += 5.days();
    assert_eq!(new_years_day_2019, utc_datetime!(2019-01-06 0:00));

    let mut new_years_eve_2020_days = utc_datetime!(2019-12-31 0:00);
    new_years_eve_2020_days += 1.days();
    assert_eq!(new_years_eve_2020_days, utc_datetime!(2020-01-01 0:00));

    let mut new_years_eve_2020_seconds = utc_datetime!(2019-12-31 23:59:59);
    new_years_eve_2020_seconds += 2.seconds();
    assert_eq!(
        new_years_eve_2020_seconds,
        utc_datetime!(2020-01-01 0:00:01)
    );

    let mut new_years_day_2020_seconds = utc_datetime!(2020-01-01 0:00:01);
    new_years_day_2020_seconds += (-2).seconds();
    assert_eq!(
        new_years_day_2020_seconds,
        utc_datetime!(2019-12-31 23:59:59)
    );
}

#[test]
fn add_assign_std_duration() {
    let mut new_years_day_2019 = utc_datetime!(2019-01-01 0:00);
    new_years_day_2019 += 5.std_days();
    assert_eq!(new_years_day_2019, utc_datetime!(2019-01-06 0:00));

    let mut new_years_eve_2020_days = utc_datetime!(2019-12-31 0:00);
    new_years_eve_2020_days += 1.std_days();
    assert_eq!(new_years_eve_2020_days, utc_datetime!(2020-01-01 0:00));

    let mut new_years_eve_2020_seconds = utc_datetime!(2019-12-31 23:59:59);
    new_years_eve_2020_seconds += 2.std_seconds();
    assert_eq!(
        new_years_eve_2020_seconds,
        utc_datetime!(2020-01-01 0:00:01)
    );
}

#[test]
fn sub_duration() {
    assert_eq!(
        utc_datetime!(2019-01-06 0:00) - 5.days(),
        utc_datetime!(2019-01-01 0:00),
    );
    assert_eq!(
        utc_datetime!(2020-01-01 0:00) - 1.days(),
        utc_datetime!(2019-12-31 0:00),
    );
    assert_eq!(
        utc_datetime!(2020-01-01 0:00:01) - 2.seconds(),
        utc_datetime!(2019-12-31 23:59:59),
    );
    assert_eq!(
        utc_datetime!(2019-12-31 23:59:59) - (-2).seconds(),
        utc_datetime!(2020-01-01 0:00:01),
    );
    assert_eq!(
        utc_datetime!(1999-12-31 23:00) - (-1).hours(),
        utc_datetime!(2000-01-01 0:00),
    );
}

#[test]
fn sub_std_duration() {
    assert_eq!(
        utc_datetime!(2019-01-06 0:00) - 5.std_days(),
        utc_datetime!(2019-01-01 0:00),
    );
    assert_eq!(
        utc_datetime!(2020-01-01 0:00) - 1.std_days(),
        utc_datetime!(2019-12-31 0:00),
    );
    assert_eq!(
        utc_datetime!(2020-01-01 0:00:01) - 2.std_seconds(),
        utc_datetime!(2019-12-31 23:59:59),
    );
}

#[test]
fn sub_assign_duration() {
    let mut new_years_day_2019 = utc_datetime!(2019-01-06 0:00);
    new_years_day_2019 -= 5.days();
    assert_eq!(new_years_day_2019, utc_datetime!(2019-01-01 0:00));

    let mut new_years_day_2020_days = utc_datetime!(2020-01-01 0:00);
    new_years_day_2020_days -= 1.days();
    assert_eq!(new_years_day_2020_days, utc_datetime!(2019-12-31 0:00));

    let mut new_years_day_2020_seconds = utc_datetime!(2020-01-01 0:00:01);
    new_years_day_2020_seconds -= 2.seconds();
    assert_eq!(
        new_years_day_2020_seconds,
        utc_datetime!(2019-12-31 23:59:59)
    );

    let mut new_years_eve_2020_seconds = utc_datetime!(2019-12-31 23:59:59);
    new_years_eve_2020_seconds -= (-2).seconds();
    assert_eq!(
        new_years_eve_2020_seconds,
        utc_datetime!(2020-01-01 0:00:01)
    );
}

#[test]
fn sub_assign_std_duration() {
    let mut ny19 = utc_datetime!(2019-01-06 0:00);
    ny19 -= 5.std_days();
    assert_eq!(ny19, utc_datetime!(2019-01-01 0:00));

    let mut ny20 = utc_datetime!(2020-01-01 0:00);
    ny20 -= 1.std_days();
    assert_eq!(ny20, utc_datetime!(2019-12-31 0:00));

    let mut ny20t = utc_datetime!(2020-01-01 0:00:01);
    ny20t -= 2.std_seconds();
    assert_eq!(ny20t, utc_datetime!(2019-12-31 23:59:59));
}

#[test]
fn std_add_duration() {
    assert_eq!(
        SystemTime::from(utc_datetime!(2019-01-01 0:00)) + 0.seconds(),
        SystemTime::from(utc_datetime!(2019-01-01 0:00)),
    );
    assert_eq!(
        SystemTime::from(utc_datetime!(2019-01-01 0:00)) + 5.days(),
        SystemTime::from(utc_datetime!(2019-01-06 0:00)),
    );
    assert_eq!(
        SystemTime::from(utc_datetime!(2019-12-31 0:00)) + 1.days(),
        SystemTime::from(utc_datetime!(2020-01-01 0:00)),
    );
    assert_eq!(
        SystemTime::from(utc_datetime!(2019-12-31 23:59:59)) + 2.seconds(),
        SystemTime::from(utc_datetime!(2020-01-01 0:00:01)),
    );
    assert_eq!(
        SystemTime::from(utc_datetime!(2020-01-01 0:00:01)) + (-2).seconds(),
        SystemTime::from(utc_datetime!(2019-12-31 23:59:59)),
    );
}

#[test]
fn std_add_assign_duration() {
    let mut new_years_day_2019 = SystemTime::from(utc_datetime!(2019-01-01 0:00));
    new_years_day_2019 += 5.days();
    assert_eq!(new_years_day_2019, utc_datetime!(2019-01-06 0:00));

    let mut new_years_eve_2020_days = SystemTime::from(utc_datetime!(2019-12-31 0:00));
    new_years_eve_2020_days += 1.days();
    assert_eq!(new_years_eve_2020_days, utc_datetime!(2020-01-01 0:00));

    let mut new_years_eve_2020_seconds = SystemTime::from(utc_datetime!(2019-12-31 23:59:59));
    new_years_eve_2020_seconds += 2.seconds();
    assert_eq!(
        new_years_eve_2020_seconds,
        utc_datetime!(2020-01-01 0:00:01)
    );

    let mut new_years_day_2020_seconds = SystemTime::from(utc_datetime!(2020-01-01 0:00:01));
    new_years_day_2020_seconds += (-2).seconds();
    assert_eq!(
        new_years_day_2020_seconds,
        utc_datetime!(2019-12-31 23:59:59)
    );
}

#[test]
fn std_sub_duration() {
    assert_eq!(
        SystemTime::from(utc_datetime!(2019-01-06 0:00)) - 5.days(),
        SystemTime::from(utc_datetime!(2019-01-01 0:00)),
    );
    assert_eq!(
        SystemTime::from(utc_datetime!(2020-01-01 0:00)) - 1.days(),
        SystemTime::from(utc_datetime!(2019-12-31 0:00)),
    );
    assert_eq!(
        SystemTime::from(utc_datetime!(2020-01-01 0:00:01)) - 2.seconds(),
        SystemTime::from(utc_datetime!(2019-12-31 23:59:59)),
    );
    assert_eq!(
        SystemTime::from(utc_datetime!(2019-12-31 23:59:59)) - (-2).seconds(),
        SystemTime::from(utc_datetime!(2020-01-01 0:00:01)),
    );
}

#[test]
fn std_sub_assign_duration() {
    let mut new_years_day_2019 = SystemTime::from(utc_datetime!(2019-01-06 0:00));
    new_years_day_2019 -= 5.days();
    assert_eq!(new_years_day_2019, utc_datetime!(2019-01-01 0:00));

    let mut new_years_day_2020 = SystemTime::from(utc_datetime!(2020-01-01 0:00));
    new_years_day_2020 -= 1.days();
    assert_eq!(new_years_day_2020, utc_datetime!(2019-12-31 0:00));

    let mut new_years_day_2020_seconds = SystemTime::from(utc_datetime!(2020-01-01 0:00:01));
    new_years_day_2020_seconds -= 2.seconds();
    assert_eq!(
        new_years_day_2020_seconds,
        utc_datetime!(2019-12-31 23:59:59)
    );

    let mut new_years_eve_2020_seconds = SystemTime::from(utc_datetime!(2019-12-31 23:59:59));
    new_years_eve_2020_seconds -= (-2).seconds();
    assert_eq!(
        new_years_eve_2020_seconds,
        utc_datetime!(2020-01-01 0:00:01)
    );
}

#[test]
fn sub_self() {
    assert_eq!(
        utc_datetime!(2019-01-02 0:00) - utc_datetime!(2019-01-01 0:00),
        1.days(),
    );
    assert_eq!(
        utc_datetime!(2019-01-01 0:00) - utc_datetime!(2019-01-02 0:00),
        (-1).days(),
    );
    assert_eq!(
        utc_datetime!(2020-01-01 0:00) - utc_datetime!(2019-12-31 0:00),
        1.days(),
    );
    assert_eq!(
        utc_datetime!(2019-12-31 0:00) - utc_datetime!(2020-01-01 0:00),
        (-1).days(),
    );
}

#[test]
fn std_sub() {
    assert_eq!(
        SystemTime::from(utc_datetime!(2019-01-02 0:00)) - utc_datetime!(2019-01-01 0:00),
        1.days()
    );
    assert_eq!(
        SystemTime::from(utc_datetime!(2019-01-01 0:00)) - utc_datetime!(2019-01-02 0:00),
        (-1).days()
    );
    assert_eq!(
        SystemTime::from(utc_datetime!(2020-01-01 0:00)) - utc_datetime!(2019-12-31 0:00),
        1.days()
    );
    assert_eq!(
        SystemTime::from(utc_datetime!(2019-12-31 0:00)) - utc_datetime!(2020-01-01 0:00),
        (-1).days()
    );
}

#[test]
fn sub_std() {
    assert_eq!(
        utc_datetime!(2019-01-02 0:00) - SystemTime::from(utc_datetime!(2019-01-01 0:00)),
        1.days()
    );
    assert_eq!(
        utc_datetime!(2019-01-01 0:00) - SystemTime::from(utc_datetime!(2019-01-02 0:00)),
        (-1).days()
    );
    assert_eq!(
        utc_datetime!(2020-01-01 0:00) - SystemTime::from(utc_datetime!(2019-12-31 0:00)),
        1.days()
    );
    assert_eq!(
        utc_datetime!(2019-12-31 0:00) - SystemTime::from(utc_datetime!(2020-01-01 0:00)),
        (-1).days()
    );
}

#[test]
fn odt_sub() {
    assert_eq!(
        datetime!(2019-01-02 0:00 UTC) - utc_datetime!(2019-01-01 0:00),
        1.days()
    );
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC) - utc_datetime!(2019-01-02 0:00),
        (-1).days()
    );
    assert_eq!(
        datetime!(2020-01-01 0:00 UTC) - utc_datetime!(2019-12-31 0:00),
        1.days()
    );
    assert_eq!(
        datetime!(2019-12-31 0:00 UTC) - utc_datetime!(2020-01-01 0:00),
        (-1).days()
    );
}

#[test]
fn sub_odt() {
    assert_eq!(
        utc_datetime!(2019-01-02 0:00) - datetime!(2019-01-01 0:00 UTC),
        1.days()
    );
    assert_eq!(
        utc_datetime!(2019-01-01 0:00) - datetime!(2019-01-02 0:00 UTC),
        (-1).days()
    );
    assert_eq!(
        utc_datetime!(2020-01-01 0:00) - datetime!(2019-12-31 0:00 UTC),
        1.days()
    );
    assert_eq!(
        utc_datetime!(2019-12-31 0:00) - datetime!(2020-01-01 0:00 UTC),
        (-1).days()
    );
}

#[test]
fn eq_std() {
    let now_datetime = UtcDateTime::now();
    let now_systemtime = SystemTime::from(now_datetime);
    assert_eq!(now_datetime, now_systemtime);
}

#[test]
fn std_eq() {
    let now_datetime = UtcDateTime::now();
    let now_systemtime = SystemTime::from(now_datetime);
    assert_eq!(now_systemtime, now_datetime);
}

#[test]
fn eq_odt() {
    let now_datetime = UtcDateTime::now();
    let now_odt = OffsetDateTime::from(now_datetime);
    assert_eq!(now_datetime, now_odt);
}

#[test]
fn odt_eq() {
    let now_datetime = UtcDateTime::now();
    let now_odt = OffsetDateTime::from(now_datetime);
    assert_eq!(now_odt, now_datetime);
}

#[test]
fn ord_std() {
    assert_eq!(
        utc_datetime!(2019-01-01 0:00),
        SystemTime::from(utc_datetime!(2019-01-01 0:00))
    );
    assert!(utc_datetime!(2019-01-01 0:00) < SystemTime::from(utc_datetime!(2020-01-01 0:00)));
    assert!(utc_datetime!(2019-01-01 0:00) < SystemTime::from(utc_datetime!(2019-02-01 0:00)));
    assert!(utc_datetime!(2019-01-01 0:00) < SystemTime::from(utc_datetime!(2019-01-02 0:00)));
    assert!(utc_datetime!(2019-01-01 0:00) < SystemTime::from(utc_datetime!(2019-01-01 1:00:00)));
    assert!(utc_datetime!(2019-01-01 0:00) < SystemTime::from(utc_datetime!(2019-01-01 0:01:00)));
    assert!(utc_datetime!(2019-01-01 0:00) < SystemTime::from(utc_datetime!(2019-01-01 0:00:01)));
    assert!(
        utc_datetime!(2019-01-01 0:00) < SystemTime::from(utc_datetime!(2019-01-01 0:00:00.001))
    );
    assert!(utc_datetime!(2020-01-01 0:00) > SystemTime::from(utc_datetime!(2019-01-01 0:00)));
    assert!(utc_datetime!(2019-02-01 0:00) > SystemTime::from(utc_datetime!(2019-01-01 0:00)));
    assert!(utc_datetime!(2019-01-02 0:00) > SystemTime::from(utc_datetime!(2019-01-01 0:00)));
    assert!(utc_datetime!(2019-01-01 1:00:00) > SystemTime::from(utc_datetime!(2019-01-01 0:00)));
    assert!(utc_datetime!(2019-01-01 0:01:00) > SystemTime::from(utc_datetime!(2019-01-01 0:00)));
    assert!(utc_datetime!(2019-01-01 0:00:01) > SystemTime::from(utc_datetime!(2019-01-01 0:00)));
    assert!(
        utc_datetime!(2019-01-01 0:00:00.000_000_001)
            > SystemTime::from(utc_datetime!(2019-01-01 0:00))
    );
}

#[test]
fn std_ord() {
    assert_eq!(
        SystemTime::from(utc_datetime!(2019-01-01 0:00)),
        utc_datetime!(2019-01-01 0:00)
    );
    assert!(SystemTime::from(utc_datetime!(2019-01-01 0:00)) < utc_datetime!(2020-01-01 0:00));
    assert!(SystemTime::from(utc_datetime!(2019-01-01 0:00)) < utc_datetime!(2019-02-01 0:00));
    assert!(SystemTime::from(utc_datetime!(2019-01-01 0:00)) < utc_datetime!(2019-01-02 0:00));
    assert!(SystemTime::from(utc_datetime!(2019-01-01 0:00)) < utc_datetime!(2019-01-01 1:00:00));
    assert!(SystemTime::from(utc_datetime!(2019-01-01 0:00)) < utc_datetime!(2019-01-01 0:01:00));
    assert!(SystemTime::from(utc_datetime!(2019-01-01 0:00)) < utc_datetime!(2019-01-01 0:00:01));
    assert!(
        SystemTime::from(utc_datetime!(2019-01-01 0:00))
            < utc_datetime!(2019-01-01 0:00:00.000_000_001)
    );
    assert!(SystemTime::from(utc_datetime!(2020-01-01 0:00)) > utc_datetime!(2019-01-01 0:00));
    assert!(SystemTime::from(utc_datetime!(2019-02-01 0:00)) > utc_datetime!(2019-01-01 0:00));
    assert!(SystemTime::from(utc_datetime!(2019-01-02 0:00)) > utc_datetime!(2019-01-01 0:00));
    assert!(SystemTime::from(utc_datetime!(2019-01-01 1:00:00)) > utc_datetime!(2019-01-01 0:00));
    assert!(SystemTime::from(utc_datetime!(2019-01-01 0:01:00)) > utc_datetime!(2019-01-01 0:00));
    assert!(SystemTime::from(utc_datetime!(2019-01-01 0:00:01)) > utc_datetime!(2019-01-01 0:00));
    assert!(
        SystemTime::from(utc_datetime!(2019-01-01 0:00:00.001)) > utc_datetime!(2019-01-01 0:00)
    );
}

#[test]
fn ord_odt() {
    assert_eq!(
        utc_datetime!(2019-01-01 0:00),
        datetime!(2019-01-01 0:00 UTC)
    );
    assert!(utc_datetime!(2019-01-01 0:00) < datetime!(2020-01-01 0:00 UTC));
    assert!(utc_datetime!(2019-01-01 0:00) < datetime!(2019-02-01 0:00 UTC));
    assert!(utc_datetime!(2019-01-01 0:00) < datetime!(2019-01-02 0:00 UTC));
    assert!(utc_datetime!(2019-01-01 0:00) < datetime!(2019-01-01 1:00:00 UTC));
    assert!(utc_datetime!(2019-01-01 0:00) < datetime!(2019-01-01 0:01:00 UTC));
    assert!(utc_datetime!(2019-01-01 0:00) < datetime!(2019-01-01 0:00:01 UTC));
    assert!(utc_datetime!(2019-01-01 0:00) < datetime!(2019-01-01 0:00:00.001 UTC));
    assert!(utc_datetime!(2020-01-01 0:00) > datetime!(2019-01-01 0:00 UTC));
    assert!(utc_datetime!(2019-02-01 0:00) > datetime!(2019-01-01 0:00 UTC));
    assert!(utc_datetime!(2019-01-02 0:00) > datetime!(2019-01-01 0:00 UTC));
    assert!(utc_datetime!(2019-01-01 1:00:00) > datetime!(2019-01-01 0:00 UTC));
    assert!(utc_datetime!(2019-01-01 0:01:00) > datetime!(2019-01-01 0:00 UTC));
    assert!(utc_datetime!(2019-01-01 0:00:01) > datetime!(2019-01-01 0:00 UTC));
    assert!(utc_datetime!(2019-01-01 0:00:00.000_000_001) > datetime!(2019-01-01 0:00 UTC));
}

#[test]
fn odt_ord() {
    assert_eq!(
        datetime!(2019-01-01 0:00 UTC),
        utc_datetime!(2019-01-01 0:00)
    );
    assert!(datetime!(2019-01-01 0:00 UTC) < utc_datetime!(2020-01-01 0:00));
    assert!(datetime!(2019-01-01 0:00 UTC) < utc_datetime!(2019-02-01 0:00));
    assert!(datetime!(2019-01-01 0:00 UTC) < utc_datetime!(2019-01-02 0:00));
    assert!(datetime!(2019-01-01 0:00 UTC) < utc_datetime!(2019-01-01 1:00:00));
    assert!(datetime!(2019-01-01 0:00 UTC) < utc_datetime!(2019-01-01 0:01:00));
    assert!(datetime!(2019-01-01 0:00 UTC) < utc_datetime!(2019-01-01 0:00:01));
    assert!(datetime!(2019-01-01 0:00 UTC) < utc_datetime!(2019-01-01 0:00:00.000_000_001));
    assert!(datetime!(2020-01-01 0:00 UTC) > utc_datetime!(2019-01-01 0:00));
    assert!(datetime!(2019-02-01 0:00 UTC) > utc_datetime!(2019-01-01 0:00));
    assert!(datetime!(2019-01-02 0:00 UTC) > utc_datetime!(2019-01-01 0:00));
    assert!(datetime!(2019-01-01 1:00:00 UTC) > utc_datetime!(2019-01-01 0:00));
    assert!(datetime!(2019-01-01 0:01:00 UTC) > utc_datetime!(2019-01-01 0:00));
    assert!(datetime!(2019-01-01 0:00:01 UTC) > utc_datetime!(2019-01-01 0:00));
    assert!(datetime!(2019-01-01 0:00:00.001 UTC) > utc_datetime!(2019-01-01 0:00));
}

#[test]
fn from_std() {
    assert_eq!(
        UtcDateTime::from(SystemTime::UNIX_EPOCH),
        UtcDateTime::UNIX_EPOCH
    );
    assert_eq!(
        UtcDateTime::from(SystemTime::UNIX_EPOCH - 1.std_days()),
        UtcDateTime::UNIX_EPOCH - 1.days()
    );
    assert_eq!(
        UtcDateTime::from(SystemTime::UNIX_EPOCH + 1.std_days()),
        UtcDateTime::UNIX_EPOCH + 1.days()
    );
}

#[test]
fn to_std() {
    assert_eq!(
        SystemTime::from(UtcDateTime::UNIX_EPOCH),
        SystemTime::UNIX_EPOCH
    );
    assert_eq!(
        SystemTime::from(UtcDateTime::UNIX_EPOCH + 1.days()),
        SystemTime::UNIX_EPOCH + 1.std_days()
    );
    assert_eq!(
        SystemTime::from(UtcDateTime::UNIX_EPOCH - 1.days()),
        SystemTime::UNIX_EPOCH - 1.std_days()
    );
}

#[test]
fn from_odt() {
    assert_eq!(
        UtcDateTime::from(OffsetDateTime::UNIX_EPOCH),
        UtcDateTime::UNIX_EPOCH
    );
    assert_eq!(
        UtcDateTime::from(OffsetDateTime::UNIX_EPOCH - 1.std_days()),
        UtcDateTime::UNIX_EPOCH - 1.days()
    );
    assert_eq!(
        UtcDateTime::from(OffsetDateTime::UNIX_EPOCH + 1.std_days()),
        UtcDateTime::UNIX_EPOCH + 1.days()
    );
}

#[test]
fn to_odt() {
    assert_eq!(
        OffsetDateTime::from(UtcDateTime::UNIX_EPOCH),
        OffsetDateTime::UNIX_EPOCH
    );
    assert_eq!(
        OffsetDateTime::from(UtcDateTime::UNIX_EPOCH + 1.days()),
        OffsetDateTime::UNIX_EPOCH + 1.std_days()
    );
    assert_eq!(
        OffsetDateTime::from(UtcDateTime::UNIX_EPOCH - 1.days()),
        OffsetDateTime::UNIX_EPOCH - 1.std_days()
    );
}

#[test]
fn checked_add_duration() {
    // Successful addition
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_add(5.nanoseconds()),
        Some(utc_datetime!(2021-10-25 14:01:53.450_000_005))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_add(4.seconds()),
        Some(utc_datetime!(2021-10-25 14:01:57.45))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_add(2.days()),
        Some(utc_datetime!(2021-10-27 14:01:53.45))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_add(1.weeks()),
        Some(utc_datetime!(2021-11-01 14:01:53.45))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_add((-5).nanoseconds()),
        Some(utc_datetime!(2021-10-25 14:01:53.449_999_995))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_add((-4).seconds()),
        Some(utc_datetime!(2021-10-25 14:01:49.45))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_add((-2).days()),
        Some(utc_datetime!(2021-10-23 14:01:53.45))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_add((-1).weeks()),
        Some(utc_datetime!(2021-10-18 14:01:53.45))
    );

    // Addition with underflow
    assert_eq!(
        utc_datetime!(-999_999-01-01 0:00).checked_add((-1).nanoseconds()),
        None
    );
    assert_eq!(
        utc_datetime!(-999_999-01-01 0:00).checked_add(Duration::MIN),
        None
    );
    assert_eq!(
        utc_datetime!(-999_990-01-01 0:00).checked_add((-530).weeks()),
        None
    );

    // Addition with overflow
    assert_eq!(
        utc_datetime!(+999_999-12-31 23:59:59.999_999_999).checked_add(1.nanoseconds()),
        None
    );
    assert_eq!(
        utc_datetime!(+999_999-12-31 23:59:59.999_999_999).checked_add(Duration::MAX),
        None
    );
    assert_eq!(
        utc_datetime!(+999_990-12-31 23:59:59.999_999_999).checked_add(530.weeks()),
        None
    );
}

#[test]
fn checked_sub_duration() {
    // Successful subtraction
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_sub((-5).nanoseconds()),
        Some(utc_datetime!(2021-10-25 14:01:53.450_000_005))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_sub((-4).seconds()),
        Some(utc_datetime!(2021-10-25 14:01:57.45))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_sub((-2).days()),
        Some(utc_datetime!(2021-10-27 14:01:53.45))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_sub((-1).weeks()),
        Some(utc_datetime!(2021-11-01 14:01:53.45))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_sub(5.nanoseconds()),
        Some(utc_datetime!(2021-10-25 14:01:53.449_999_995))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_sub(4.seconds()),
        Some(utc_datetime!(2021-10-25 14:01:49.45))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_sub(2.days()),
        Some(utc_datetime!(2021-10-23 14:01:53.45))
    );
    assert_eq!(
        utc_datetime!(2021-10-25 14:01:53.45).checked_sub(1.weeks()),
        Some(utc_datetime!(2021-10-18 14:01:53.45))
    );

    // Subtraction with underflow
    assert_eq!(
        utc_datetime!(-999_999-01-01 0:00).checked_sub(1.nanoseconds()),
        None
    );
    assert_eq!(
        utc_datetime!(-999_999-01-01 0:00).checked_sub(Duration::MAX),
        None
    );
    assert_eq!(
        utc_datetime!(-999_990-01-01 0:00).checked_sub(530.weeks()),
        None
    );

    // Subtraction with overflow
    assert_eq!(
        utc_datetime!(+999_999-12-31 23:59:59.999_999_999).checked_sub((-1).nanoseconds()),
        None
    );
    assert_eq!(
        utc_datetime!(+999_999-12-31 23:59:59.999_999_999).checked_sub(Duration::MIN),
        None
    );
    assert_eq!(
        utc_datetime!(+999_990-12-31 23:59:59.999_999_999).checked_sub((-530).weeks()),
        None
    );

    // Subtracting 0 duration at MIN/MAX values with non-zero offset
    assert_eq!(
        utc_datetime!(+999_999-12-31 23:59:59.999_999_999).checked_sub(Duration::ZERO),
        Some(utc_datetime!(+999_999-12-31 23:59:59.999_999_999))
    );
    assert_eq!(
        utc_datetime!(-999_999-01-01 0:00).checked_sub(Duration::ZERO),
        Some(utc_datetime!(-999_999-01-01 0:00))
    );
}

#[test]
fn saturating_add_duration() {
    assert_eq!(
        utc_datetime!(2021-11-12 17:47).saturating_add(2.days()),
        utc_datetime!(2021-11-14 17:47)
    );
    assert_eq!(
        utc_datetime!(2021-11-12 17:47).saturating_add((-2).days()),
        utc_datetime!(2021-11-10 17:47)
    );

    // Adding with underflow
    assert_eq!(
        utc_datetime!(-999999-01-01 0:00).saturating_add((-10).days()),
        utc_datetime!(-999999-01-01 0:00)
    );

    // Adding with overflow
    assert_eq!(
        utc_datetime!(+999999-12-31 23:59:59.999_999_999).saturating_add(10.days()),
        utc_datetime!(+999999-12-31 23:59:59.999_999_999)
    );

    // Adding zero duration at boundaries
    assert_eq!(
        utc_datetime!(-999999-01-01 0:00).saturating_add(Duration::ZERO),
        utc_datetime!(-999999-01-01 0:00)
    );
    assert_eq!(
        utc_datetime!(+999999-12-31 23:59:59.999_999_999).saturating_add(Duration::ZERO),
        utc_datetime!(+999999-12-31 23:59:59.999_999_999)
    );
}

#[test]
fn saturating_sub_duration() {
    assert_eq!(
        utc_datetime!(2021-11-12 17:47).saturating_sub(2.days()),
        utc_datetime!(2021-11-10 17:47)
    );
    assert_eq!(
        utc_datetime!(2021-11-12 17:47).saturating_sub((-2).days()),
        utc_datetime!(2021-11-14 17:47)
    );

    // Subtracting with underflow
    assert_eq!(
        utc_datetime!(-999999-01-01 0:00).saturating_sub(10.days()),
        utc_datetime!(-999999-01-01 0:00)
    );

    // Subtracting with overflow
    assert_eq!(
        utc_datetime!(+999999-12-31 23:59:59.999_999_999).saturating_sub((-10).days()),
        utc_datetime!(+999999-12-31 23:59:59.999_999_999)
    );

    // Subtracting zero duration at boundaries
    assert_eq!(
        utc_datetime!(-999999-01-01 0:00).saturating_sub(Duration::ZERO),
        utc_datetime!(-999999-01-01 0:00)
    );
    assert_eq!(
        utc_datetime!(+999999-12-31 23:59:59.999_999_999).saturating_sub(Duration::ZERO),
        utc_datetime!(+999999-12-31 23:59:59.999_999_999)
    );
}

#[test]
#[should_panic = "overflow adding duration to date"]
fn issue_621() {
    let _ = UtcDateTime::UNIX_EPOCH + StdDuration::from_secs(18_157_382_926_370_278_155);
}

#[test]
fn to_offset_regression() {
    let value = utc_datetime!(0000-01-01 23:59).to_offset(offset!(+24:59));
    assert_eq!(value, datetime!(0000-01-03 0:58 +24:59));
}
