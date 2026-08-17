use std::cmp::Ordering;

use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::{date, datetime, offset, time};
use time::{Duration, Month, PrimitiveDateTime, Weekday};

#[test]
fn new() {
    assert_eq!(
        PrimitiveDateTime::new(date!(2019-01-01), time!(0:00)),
        datetime!(2019-01-01 0:00),
    );
}

#[test]
fn date() {
    assert_eq!(datetime!(2019-01-01 0:00).date(), date!(2019-01-01));
}

#[test]
fn time() {
    assert_eq!(datetime!(2019-01-01 0:00).time(), time!(0:00));
}

#[test]
fn year() {
    assert_eq!(datetime!(2019-01-01 0:00).year(), 2019);
    assert_eq!(datetime!(2019-12-31 0:00).year(), 2019);
    assert_eq!(datetime!(2020-01-01 0:00).year(), 2020);
}

#[test]
fn month() {
    assert_eq!(datetime!(2019-01-01 0:00).month(), Month::January);
    assert_eq!(datetime!(2019-12-31 0:00).month(), Month::December);
}

#[test]
fn day() {
    assert_eq!(datetime!(2019-01-01 0:00).day(), 1);
    assert_eq!(datetime!(2019-12-31 0:00).day(), 31);
}

#[test]
fn ordinal() {
    assert_eq!(datetime!(2019-01-01 0:00).ordinal(), 1);
    assert_eq!(datetime!(2019-12-31 0:00).ordinal(), 365);
}

#[test]
fn iso_week() {
    assert_eq!(datetime!(2019-01-01 0:00).iso_week(), 1);
    assert_eq!(datetime!(2019-10-04 0:00).iso_week(), 40);
    assert_eq!(datetime!(2020-01-01 0:00).iso_week(), 1);
    assert_eq!(datetime!(2020-12-31 0:00).iso_week(), 53);
    assert_eq!(datetime!(2021-01-01 0:00).iso_week(), 53);
}

#[test]
fn sunday_based_week() {
    assert_eq!(datetime!(2019-01-01 0:00).sunday_based_week(), 0);
    assert_eq!(datetime!(2020-01-01 0:00).sunday_based_week(), 0);
    assert_eq!(datetime!(2020-12-31 0:00).sunday_based_week(), 52);
    assert_eq!(datetime!(2021-01-01 0:00).sunday_based_week(), 0);
}

#[test]
fn monday_based_week() {
    assert_eq!(datetime!(2019-01-01 0:00).monday_based_week(), 0);
    assert_eq!(datetime!(2020-01-01 0:00).monday_based_week(), 0);
    assert_eq!(datetime!(2020-12-31 0:00).monday_based_week(), 52);
    assert_eq!(datetime!(2021-01-01 0:00).monday_based_week(), 0);
}

#[test]
fn to_calendar_date() {
    assert_eq!(
        datetime!(2019-01-02 0:00).to_calendar_date(),
        (2019, Month::January, 2)
    );
}

#[test]
fn to_ordinal_date() {
    assert_eq!(datetime!(2019-01-01 0:00).to_ordinal_date(), (2019, 1));
}

#[test]
fn to_iso_week_date() {
    use Weekday::*;
    assert_eq!(
        datetime!(2019-01-01 0:00).to_iso_week_date(),
        (2019, 1, Tuesday)
    );
    assert_eq!(
        datetime!(2019-10-04 0:00).to_iso_week_date(),
        (2019, 40, Friday)
    );
    assert_eq!(
        datetime!(2020-01-01 0:00).to_iso_week_date(),
        (2020, 1, Wednesday)
    );
    assert_eq!(
        datetime!(2020-12-31 0:00).to_iso_week_date(),
        (2020, 53, Thursday)
    );
    assert_eq!(
        datetime!(2021-01-01 0:00).to_iso_week_date(),
        (2020, 53, Friday)
    );
}

#[test]
fn weekday() {
    use Weekday::*;
    assert_eq!(datetime!(2019-01-01 0:00).weekday(), Tuesday);
    assert_eq!(datetime!(2019-02-01 0:00).weekday(), Friday);
    assert_eq!(datetime!(2019-03-01 0:00).weekday(), Friday);
    assert_eq!(datetime!(2019-04-01 0:00).weekday(), Monday);
    assert_eq!(datetime!(2019-05-01 0:00).weekday(), Wednesday);
    assert_eq!(datetime!(2019-06-01 0:00).weekday(), Saturday);
    assert_eq!(datetime!(2019-07-01 0:00).weekday(), Monday);
    assert_eq!(datetime!(2019-08-01 0:00).weekday(), Thursday);
    assert_eq!(datetime!(2019-09-01 0:00).weekday(), Sunday);
    assert_eq!(datetime!(2019-10-01 0:00).weekday(), Tuesday);
    assert_eq!(datetime!(2019-11-01 0:00).weekday(), Friday);
    assert_eq!(datetime!(2019-12-01 0:00).weekday(), Sunday);
}

#[test]
fn to_julian_day() {
    assert_eq!(datetime!(-999_999-01-01 0:00).to_julian_day(), -363_521_074);
    assert_eq!(datetime!(-4713-11-24 0:00).to_julian_day(), 0);
    assert_eq!(datetime!(2000-01-01 0:00).to_julian_day(), 2_451_545);
    assert_eq!(datetime!(2019-01-01 0:00).to_julian_day(), 2_458_485);
    assert_eq!(datetime!(2019-12-31 0:00).to_julian_day(), 2_458_849);
}

#[test]
fn as_hms() {
    assert_eq!(datetime!(2020-01-01 1:02:03).as_hms(), (1, 2, 3));
}

#[test]
fn as_hms_milli() {
    assert_eq!(
        datetime!(2020-01-01 1:02:03.004).as_hms_milli(),
        (1, 2, 3, 4)
    );
}

#[test]
fn as_hms_micro() {
    assert_eq!(
        datetime!(2020-01-01 1:02:03.004_005).as_hms_micro(),
        (1, 2, 3, 4_005)
    );
}

#[test]
fn as_hms_nano() {
    assert_eq!(
        datetime!(2020-01-01 1:02:03.004_005_006).as_hms_nano(),
        (1, 2, 3, 4_005_006)
    );
}

#[test]
fn hour() {
    assert_eq!(datetime!(2019-01-01 0:00).hour(), 0);
    assert_eq!(datetime!(2019-01-01 23:59:59).hour(), 23);
}

#[test]
fn minute() {
    assert_eq!(datetime!(2019-01-01 0:00).minute(), 0);
    assert_eq!(datetime!(2019-01-01 23:59:59).minute(), 59);
}

#[test]
fn second() {
    assert_eq!(datetime!(2019-01-01 0:00).second(), 0);
    assert_eq!(datetime!(2019-01-01 23:59:59).second(), 59);
}

#[test]
fn millisecond() {
    assert_eq!(datetime!(2019-01-01 0:00).millisecond(), 0);
    assert_eq!(datetime!(2019-01-01 23:59:59.999).millisecond(), 999);
}

#[test]
fn microsecond() {
    assert_eq!(datetime!(2019-01-01 0:00).microsecond(), 0);
    assert_eq!(
        datetime!(2019-01-01 23:59:59.999_999).microsecond(),
        999_999
    );
}

#[test]
fn nanosecond() {
    assert_eq!(datetime!(2019-01-01 0:00).nanosecond(), 0);
    assert_eq!(
        datetime!(2019-01-01 23:59:59.999_999_999).nanosecond(),
        999_999_999
    );
}

#[test]
fn assume_offset() {
    assert_eq!(
        datetime!(2019-01-01 0:00)
            .assume_offset(offset!(UTC))
            .unix_timestamp(),
        1_546_300_800,
    );
    assert_eq!(
        datetime!(2019-01-01 0:00)
            .assume_offset(offset!(-1))
            .unix_timestamp(),
        1_546_304_400,
    );
}

#[test]
fn assume_utc() {
    assert_eq!(
        datetime!(2019-01-01 0:00).assume_utc().unix_timestamp(),
        1_546_300_800,
    );
}

#[test]
fn replace_time() {
    assert_eq!(
        datetime!(2020-01-01 12:00).replace_time(time!(5:00)),
        datetime!(2020-01-01 5:00)
    );
}

#[test]
fn replace_date() {
    assert_eq!(
        datetime!(2020-01-01 12:00).replace_date(date!(2020-01-30)),
        datetime!(2020-01-30 12:00)
    );
}

#[test]
fn replace_year() {
    assert_eq!(
        datetime!(2022-02-18 12:00).replace_year(2019),
        Ok(datetime!(2019-02-18 12:00))
    );
    assert!(
        datetime!(2022-02-18 12:00)
            .replace_year(-1_000_000_000)
            .is_err()
    ); // -1_000_000_000 isn't a valid year
    assert!(
        datetime!(2022-02-18 12:00)
            .replace_year(1_000_000_000)
            .is_err()
    ); // 1_000_000_000 isn't a valid year
}

#[test]
fn replace_month() {
    assert_eq!(
        datetime!(2022-02-18 12:00).replace_month(Month::January),
        Ok(datetime!(2022-01-18 12:00))
    );
    assert!(
        datetime!(2022-01-30 12:00)
            .replace_month(Month::February)
            .is_err()
    ); // 30 isn't a valid day in February
}

#[test]
fn replace_day() {
    assert_eq!(
        datetime!(2022-02-18 12:00).replace_day(1),
        Ok(datetime!(2022-02-01 12:00))
    );
    // 00 isn't a valid day
    assert!(datetime!(2022-02-18 12:00).replace_day(0).is_err());
    // 30 isn't a valid day in February
    assert!(datetime!(2022-02-18 12:00).replace_day(30).is_err());
}

#[test]
fn replace_ordinal() {
    assert_eq!(
        datetime!(2022-02-18 12:00).replace_ordinal(1),
        Ok(datetime!(2022-001 12:00))
    );
    assert_eq!(
        datetime!(2024-02-29 12:00).replace_ordinal(366),
        Ok(datetime!(2024-366 12:00))
    );
    assert!(datetime!(2022-049 12:00).replace_ordinal(0).is_err()); // 0 isn't a valid day
    assert!(datetime!(2022-049 12:00).replace_ordinal(366).is_err()); // 2022 isn't a leap year
    assert!(datetime!(2022-049 12:00).replace_ordinal(367).is_err()); // 367 isn't a valid day
}

#[test]
fn replace_hour() {
    assert_eq!(
        datetime!(2022-02-18 01:02:03.004_005_006).replace_hour(7),
        Ok(datetime!(2022-02-18 07:02:03.004_005_006))
    );
    assert!(
        datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_hour(24)
            .is_err()
    ); // 24 isn't a valid hour
}

#[test]
fn replace_minute() {
    assert_eq!(
        datetime!(2022-02-18 01:02:03.004_005_006).replace_minute(7),
        Ok(datetime!(2022-02-18 01:07:03.004_005_006))
    );
    assert!(
        datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_minute(60)
            .is_err()
    ); // 60 isn't a valid minute
}

#[test]
fn replace_second() {
    assert_eq!(
        datetime!(2022-02-18 01:02:03.004_005_006).replace_second(7),
        Ok(datetime!(2022-02-18 01:02:07.004_005_006))
    );
    assert!(
        datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_second(60)
            .is_err()
    ); // 60 isn't a valid second
}

#[test]
fn replace_millisecond() {
    assert_eq!(
        datetime!(2022-02-18 01:02:03.004_005_006).replace_millisecond(7),
        Ok(datetime!(2022-02-18 01:02:03.007))
    );
    assert!(
        datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_millisecond(1_000)
            .is_err()
    ); // 1_000 isn't a valid millisecond
}

#[test]
fn replace_microsecond() {
    assert_eq!(
        datetime!(2022-02-18 01:02:03.004_005_006).replace_microsecond(7_008),
        Ok(datetime!(2022-02-18 01:02:03.007_008))
    );
    assert!(
        datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_microsecond(1_000_000)
            .is_err()
    ); // 1_000_000 isn't a valid microsecond
}

#[test]
fn replace_nanosecond() {
    assert_eq!(
        datetime!(2022-02-18 01:02:03.004_005_006).replace_nanosecond(7_008_009),
        Ok(datetime!(2022-02-18 01:02:03.007_008_009))
    );
    assert!(
        datetime!(2022-02-18 01:02:03.004_005_006)
            .replace_nanosecond(1_000_000_000)
            .is_err()
    ); // 1_000_000_000 isn't a valid nanosecond
}

#[test]
fn truncate_to_day() {
    assert_eq!(
        datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_day(),
        datetime!(2021-11-12 0:00)
    );
    assert_eq!(
        datetime!(2021-11-12 0:00).truncate_to_day(),
        datetime!(2021-11-12 0:00)
    );
}

#[test]
fn truncate_to_hour() {
    assert_eq!(
        datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_hour(),
        datetime!(2021-11-12 17:00)
    );
    assert_eq!(
        datetime!(2021-11-12 0:00).truncate_to_hour(),
        datetime!(2021-11-12 0:00)
    );
}

#[test]
fn truncate_to_minute() {
    assert_eq!(
        datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_minute(),
        datetime!(2021-11-12 17:47)
    );
    assert_eq!(
        datetime!(2021-11-12 0:00).truncate_to_minute(),
        datetime!(2021-11-12 0:00)
    );
}

#[test]
fn truncate_to_second() {
    assert_eq!(
        datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_second(),
        datetime!(2021-11-12 17:47:53)
    );
    assert_eq!(
        datetime!(2021-11-12 0:00).truncate_to_second(),
        datetime!(2021-11-12 0:00)
    );
}

#[test]
fn truncate_to_millisecond() {
    assert_eq!(
        datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_millisecond(),
        datetime!(2021-11-12 17:47:53.123)
    );
    assert_eq!(
        datetime!(2021-11-12 0:00).truncate_to_millisecond(),
        datetime!(2021-11-12 0:00)
    );
}

#[test]
fn truncate_to_microsecond() {
    assert_eq!(
        datetime!(2021-11-12 17:47:53.123_456_789).truncate_to_microsecond(),
        datetime!(2021-11-12 17:47:53.123_456)
    );
    assert_eq!(
        datetime!(2021-11-12 0:00).truncate_to_microsecond(),
        datetime!(2021-11-12 0:00)
    );
}

#[test]
fn add_duration() {
    assert_eq!(
        datetime!(2019-01-01 0:00) + 5.days(),
        datetime!(2019-01-06 0:00),
    );
    assert_eq!(
        datetime!(2019-12-31 0:00) + 1.days(),
        datetime!(2020-01-01 0:00),
    );
    assert_eq!(
        datetime!(2019-12-31 23:59:59) + 2.seconds(),
        datetime!(2020-01-01 0:00:01),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00:01) + (-2).seconds(),
        datetime!(2019-12-31 23:59:59),
    );
    assert_eq!(
        datetime!(1999-12-31 23:00) + 1.hours(),
        datetime!(2000-01-01 0:00),
    );
}

#[test]
fn add_std_duration() {
    assert_eq!(
        datetime!(2019-01-01 0:00) + 5.std_days(),
        datetime!(2019-01-06 0:00),
    );
    assert_eq!(
        datetime!(2019-12-31 0:00) + 1.std_days(),
        datetime!(2020-01-01 0:00),
    );
    assert_eq!(
        datetime!(2019-12-31 23:59:59) + 2.std_seconds(),
        datetime!(2020-01-01 0:00:01),
    );
}

#[test]
fn add_assign_duration() {
    let mut new_years_day_2019 = datetime!(2019-01-01 0:00);
    new_years_day_2019 += 5.days();
    assert_eq!(new_years_day_2019, datetime!(2019-01-06 0:00));

    let mut new_years_eve_2020_days = datetime!(2019-12-31 0:00);
    new_years_eve_2020_days += 1.days();
    assert_eq!(new_years_eve_2020_days, datetime!(2020-01-01 0:00));

    let mut new_years_eve_2020_seconds = datetime!(2019-12-31 23:59:59);
    new_years_eve_2020_seconds += 2.seconds();
    assert_eq!(new_years_eve_2020_seconds, datetime!(2020-01-01 0:00:01));

    let mut new_years_day_2020_days = datetime!(2020-01-01 0:00:01);
    new_years_day_2020_days += (-2).seconds();
    assert_eq!(new_years_day_2020_days, datetime!(2019-12-31 23:59:59));
}

#[test]
fn add_assign_std_duration() {
    let mut ny19 = datetime!(2019-01-01 0:00);
    ny19 += 5.std_days();
    assert_eq!(ny19, datetime!(2019-01-06 0:00));

    let mut nye20 = datetime!(2019-12-31 0:00);
    nye20 += 1.std_days();
    assert_eq!(nye20, datetime!(2020-01-01 0:00));

    let mut nye20t = datetime!(2019-12-31 23:59:59);
    nye20t += 2.std_seconds();
    assert_eq!(nye20t, datetime!(2020-01-01 0:00:01));
}

#[test]
fn sub_duration() {
    assert_eq!(
        datetime!(2019-01-06 0:00) - 5.days(),
        datetime!(2019-01-01 0:00),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00) - 1.days(),
        datetime!(2019-12-31 0:00),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00:01) - 2.seconds(),
        datetime!(2019-12-31 23:59:59),
    );
    assert_eq!(
        datetime!(2019-12-31 23:59:59) - (-2).seconds(),
        datetime!(2020-01-01 0:00:01),
    );
    assert_eq!(
        datetime!(1999-12-31 23:00) - (-1).hours(),
        datetime!(2000-01-01 0:00),
    );
}

#[test]
fn sub_std_duration() {
    assert_eq!(
        datetime!(2019-01-06 0:00) - 5.std_days(),
        datetime!(2019-01-01 0:00),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00) - 1.std_days(),
        datetime!(2019-12-31 0:00),
    );
    assert_eq!(
        datetime!(2020-01-01 0:00:01) - 2.std_seconds(),
        datetime!(2019-12-31 23:59:59),
    );
}

#[test]
fn sub_assign_duration() {
    let mut new_years_day_2019 = datetime!(2019-01-06 0:00);
    new_years_day_2019 -= 5.days();
    assert_eq!(new_years_day_2019, datetime!(2019-01-01 0:00));

    let mut new_years_day_2020_days = datetime!(2020-01-01 0:00);
    new_years_day_2020_days -= 1.days();
    assert_eq!(new_years_day_2020_days, datetime!(2019-12-31 0:00));

    let mut new_years_day_2020_seconds = datetime!(2020-01-01 0:00:01);
    new_years_day_2020_seconds -= 2.seconds();
    assert_eq!(new_years_day_2020_seconds, datetime!(2019-12-31 23:59:59));

    let mut new_years_eve_2020_seconds = datetime!(2019-12-31 23:59:59);
    new_years_eve_2020_seconds -= (-2).seconds();
    assert_eq!(new_years_eve_2020_seconds, datetime!(2020-01-01 0:00:01));
}

#[test]
fn sub_assign_std_duration() {
    let mut ny19 = datetime!(2019-01-06 0:00);
    ny19 -= 5.std_days();
    assert_eq!(ny19, datetime!(2019-01-01 0:00));

    let mut ny20 = datetime!(2020-01-01 0:00);
    ny20 -= 1.std_days();
    assert_eq!(ny20, datetime!(2019-12-31 0:00));

    let mut ny20t = datetime!(2020-01-01 0:00:01);
    ny20t -= 2.std_seconds();
    assert_eq!(ny20t, datetime!(2019-12-31 23:59:59));
}

#[test]
fn sub_datetime() {
    assert_eq!(
        datetime!(2019-01-02 0:00) - datetime!(2019-01-01 0:00),
        1.days()
    );
    assert_eq!(
        datetime!(2019-01-01 0:00) - datetime!(2019-01-02 0:00),
        (-1).days()
    );
    assert_eq!(
        datetime!(2020-01-01 0:00) - datetime!(2019-12-31 0:00),
        1.days()
    );
    assert_eq!(
        datetime!(2019-12-31 0:00) - datetime!(2020-01-01 0:00),
        (-1).days()
    );
}

#[test]
fn ord() {
    use Ordering::*;
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Equal)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2020-01-01 0:00)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-02-01 0:00)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-02 0:00)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 1:00)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:01)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00:01)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00:00.000_000_001)),
        Some(Less)
    );
    assert_eq!(
        datetime!(2020-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-02-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-01-02 0:00).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-01-01 1:00).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-01-01 0:01).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00:01).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(2019-01-01 0:00:00.000_000_001).partial_cmp(&datetime!(2019-01-01 0:00)),
        Some(Greater)
    );
    assert_eq!(
        datetime!(-0001-01-01 0:00).partial_cmp(&datetime!(0001-01-01 0:00)),
        Some(Less)
    );
}

#[test]
fn checked_add_duration() {
    // Successful addition
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_add(5.nanoseconds()),
        Some(datetime!(2021-10-25 14:01:53.450_000_005))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_add(4.seconds()),
        Some(datetime!(2021-10-25 14:01:57.45))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_add(2.days()),
        Some(datetime!(2021-10-27 14:01:53.45))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_add(1.weeks()),
        Some(datetime!(2021-11-01 14:01:53.45))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_add((-5).nanoseconds()),
        Some(datetime!(2021-10-25 14:01:53.449_999_995))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_add((-4).seconds()),
        Some(datetime!(2021-10-25 14:01:49.45))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_add((-2).days()),
        Some(datetime!(2021-10-23 14:01:53.45))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_add((-1).weeks()),
        Some(datetime!(2021-10-18 14:01:53.45))
    );

    // Addition with underflow
    assert_eq!(
        datetime!(-999_999-01-01 0:00).checked_add((-1).nanoseconds()),
        None
    );
    assert_eq!(
        datetime!(-999_999-01-01 0:00).checked_add(Duration::MIN),
        None
    );
    assert_eq!(
        datetime!(-999_990-01-01 0:00).checked_add((-530).weeks()),
        None
    );

    // Addition with overflow
    assert_eq!(
        datetime!(+999_999-12-31 23:59:59.999_999_999).checked_add(1.nanoseconds()),
        None
    );
    assert_eq!(
        datetime!(+999_999-12-31 23:59:59.999_999_999).checked_add(Duration::MAX),
        None
    );
    assert_eq!(
        datetime!(+999_990-12-31 23:59:59.999_999_999).checked_add(530.weeks()),
        None
    );
}

#[test]
fn checked_sub_duration() {
    // Successful subtraction
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_sub((-5).nanoseconds()),
        Some(datetime!(2021-10-25 14:01:53.450_000_005))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_sub((-4).seconds()),
        Some(datetime!(2021-10-25 14:01:57.45))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_sub((-2).days()),
        Some(datetime!(2021-10-27 14:01:53.45))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_sub((-1).weeks()),
        Some(datetime!(2021-11-01 14:01:53.45))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_sub(5.nanoseconds()),
        Some(datetime!(2021-10-25 14:01:53.449_999_995))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_sub(4.seconds()),
        Some(datetime!(2021-10-25 14:01:49.45))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_sub(2.days()),
        Some(datetime!(2021-10-23 14:01:53.45))
    );
    assert_eq!(
        datetime!(2021-10-25 14:01:53.45).checked_sub(1.weeks()),
        Some(datetime!(2021-10-18 14:01:53.45))
    );

    // Subtraction with underflow
    assert_eq!(
        datetime!(-999_999-01-01 0:00).checked_sub(1.nanoseconds()),
        None
    );
    assert_eq!(
        datetime!(-999_999-01-01 0:00).checked_sub(Duration::MAX),
        None
    );
    assert_eq!(
        datetime!(-999_990-01-01 0:00).checked_sub(530.weeks()),
        None
    );

    // Subtraction with overflow
    assert_eq!(
        datetime!(+999_999-12-31 23:59:59.999_999_999).checked_sub((-1).nanoseconds()),
        None
    );
    assert_eq!(
        datetime!(+999_999-12-31 23:59:59.999_999_999).checked_sub(Duration::MIN),
        None
    );
    assert_eq!(
        datetime!(+999_990-12-31 23:59:59.999_999_999).checked_sub((-530).weeks()),
        None
    );
}

#[test]
fn saturating_add_duration() {
    assert_eq!(
        datetime!(2021-11-12 17:47).saturating_add(2.days()),
        datetime!(2021-11-14 17:47)
    );
    assert_eq!(
        datetime!(2021-11-12 17:47).saturating_add((-2).days()),
        datetime!(2021-11-10 17:47)
    );

    // Adding with underflow
    assert_eq!(
        PrimitiveDateTime::MIN.saturating_add((-10).days()),
        PrimitiveDateTime::MIN
    );

    // Adding with overflow
    assert_eq!(
        PrimitiveDateTime::MAX.saturating_add(10.days()),
        PrimitiveDateTime::MAX
    );

    // Adding zero duration at boundaries
    assert_eq!(
        PrimitiveDateTime::MIN.saturating_add(Duration::ZERO),
        PrimitiveDateTime::MIN
    );
    assert_eq!(
        PrimitiveDateTime::MAX.saturating_add(Duration::ZERO),
        PrimitiveDateTime::MAX
    );
}

#[test]
fn saturating_sub_duration() {
    assert_eq!(
        datetime!(2021-11-12 17:47).saturating_sub(2.days()),
        datetime!(2021-11-10 17:47)
    );
    assert_eq!(
        datetime!(2021-11-12 17:47).saturating_sub((-2).days()),
        datetime!(2021-11-14 17:47)
    );

    // Subtracting with underflow
    assert_eq!(
        PrimitiveDateTime::MIN.saturating_sub(10.days()),
        PrimitiveDateTime::MIN
    );

    // Subtracting with overflow
    assert_eq!(
        PrimitiveDateTime::MAX.saturating_sub((-10).days()),
        PrimitiveDateTime::MAX
    );

    // Subtracting zero duration at boundaries
    assert_eq!(
        PrimitiveDateTime::MIN.saturating_sub(Duration::ZERO),
        PrimitiveDateTime::MIN
    );
    assert_eq!(
        PrimitiveDateTime::MAX.saturating_sub(Duration::ZERO),
        PrimitiveDateTime::MAX
    );
}
