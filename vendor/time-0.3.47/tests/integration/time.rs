use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::time;
use time::{Result, Time};

#[test]
fn from_hms() -> Result<()> {
    let time = Time::from_hms(1, 2, 3)?;
    assert_eq!(time.hour(), 1);
    assert_eq!(time.minute(), 2);
    assert_eq!(time.second(), 3);
    assert_eq!(time.nanosecond(), 0);

    assert!(Time::from_hms(24, 0, 0).is_err());
    assert!(Time::from_hms(0, 60, 0).is_err());
    assert!(Time::from_hms(0, 0, 60).is_err());
    Ok(())
}

#[test]
fn from_hms_milli() -> Result<()> {
    let time = Time::from_hms_milli(1, 2, 3, 4)?;
    assert_eq!(time.hour(), 1);
    assert_eq!(time.minute(), 2);
    assert_eq!(time.second(), 3);
    assert_eq!(time.millisecond(), 4);
    assert_eq!(time.nanosecond(), 4_000_000);

    assert!(Time::from_hms_milli(24, 0, 0, 0).is_err());
    assert!(Time::from_hms_milli(0, 60, 0, 0).is_err());
    assert!(Time::from_hms_milli(0, 0, 60, 0).is_err());
    assert!(Time::from_hms_milli(0, 0, 0, 1_000).is_err());
    Ok(())
}

#[test]
fn from_hms_micro() -> Result<()> {
    let time = Time::from_hms_micro(1, 2, 3, 4)?;
    assert_eq!(time.hour(), 1);
    assert_eq!(time.minute(), 2);
    assert_eq!(time.second(), 3);
    assert_eq!(time.microsecond(), 4);
    assert_eq!(time.nanosecond(), 4_000);

    assert!(Time::from_hms_micro(24, 0, 0, 0).is_err());
    assert!(Time::from_hms_micro(0, 60, 0, 0).is_err());
    assert!(Time::from_hms_micro(0, 0, 60, 0).is_err());
    assert!(Time::from_hms_micro(0, 0, 0, 1_000_000).is_err());
    Ok(())
}

#[test]
fn from_hms_nano() -> Result<()> {
    let time = Time::from_hms_nano(1, 2, 3, 4)?;
    assert_eq!(time.hour(), 1);
    assert_eq!(time.minute(), 2);
    assert_eq!(time.second(), 3);
    assert_eq!(time.nanosecond(), 4);

    assert!(Time::from_hms_nano(24, 0, 0, 0).is_err());
    assert!(Time::from_hms_nano(0, 60, 0, 0).is_err());
    assert!(Time::from_hms_nano(0, 0, 60, 0).is_err());
    assert!(Time::from_hms_nano(0, 0, 0, 1_000_000_000).is_err());
    Ok(())
}

#[test]
fn as_hms() {
    assert_eq!(time!(1:02:03).as_hms(), (1, 2, 3));
}

#[test]
fn as_hms_milli() {
    assert_eq!(time!(1:02:03.004).as_hms_milli(), (1, 2, 3, 4));
}

#[test]
fn as_hms_micro() {
    assert_eq!(time!(1:02:03.004_005).as_hms_micro(), (1, 2, 3, 4_005));
}

#[test]
fn as_hms_nano() {
    assert_eq!(
        time!(1:02:03.004_005_006).as_hms_nano(),
        (1, 2, 3, 4_005_006)
    );
}

#[test]
fn hour() -> Result<()> {
    for hour in 0..24 {
        assert_eq!(Time::from_hms(hour, 0, 0)?.hour(), hour);
        assert_eq!(Time::from_hms(hour, 59, 59)?.hour(), hour);
    }
    Ok(())
}

#[test]
fn minute() -> Result<()> {
    for minute in 0..60 {
        assert_eq!(Time::from_hms(0, minute, 0)?.minute(), minute);
        assert_eq!(Time::from_hms(23, minute, 59)?.minute(), minute);
    }
    Ok(())
}

#[test]
fn second() -> Result<()> {
    for second in 0..60 {
        assert_eq!(Time::from_hms(0, 0, second)?.second(), second);
        assert_eq!(Time::from_hms(23, 59, second)?.second(), second);
    }
    Ok(())
}

#[test]
fn millisecond() -> Result<()> {
    for milli in 0..1_000 {
        assert_eq!(Time::from_hms_milli(0, 0, 0, milli)?.millisecond(), milli);
        assert_eq!(
            Time::from_hms_milli(23, 59, 59, milli)?.millisecond(),
            milli
        );
    }
    Ok(())
}

#[test]
fn microsecond() -> Result<()> {
    for micro in (0..1_000_000).step_by(1_000) {
        assert_eq!(Time::from_hms_micro(0, 0, 0, micro)?.microsecond(), micro);
        assert_eq!(
            Time::from_hms_micro(23, 59, 59, micro)?.microsecond(),
            micro
        );
    }
    Ok(())
}

#[test]
fn nanosecond() -> Result<()> {
    for nano in (0..1_000_000_000).step_by(1_000_000) {
        assert_eq!(Time::from_hms_nano(0, 0, 0, nano)?.nanosecond(), nano);
        assert_eq!(Time::from_hms_nano(23, 59, 59, nano)?.nanosecond(), nano);
    }
    Ok(())
}

#[test]
fn duration_until() -> Result<()> {
    assert_eq!(time!(18:00).duration_until(Time::MIDNIGHT), 6.hours());
    assert_eq!(time!(23:00).duration_until(time!(1:00)), 2.hours());
    assert_eq!(time!(12:30).duration_until(time!(14:00)), 90.minutes());
    Ok(())
}

#[test]
fn duration_since() -> Result<()> {
    assert_eq!(Time::MIDNIGHT.duration_since(time!(18:00)), 6.hours());
    assert_eq!(time!(1:00).duration_since(time!(23:00)), 2.hours());
    assert_eq!(time!(14:00).duration_since(time!(12:30)), 90.minutes());
    Ok(())
}

#[test]
fn replace_hour() {
    assert_eq!(
        time!(1:02:03.004_005_006).replace_hour(7),
        Ok(time!(7:02:03.004_005_006))
    );
    assert!(time!(1:02:03.004_005_006).replace_hour(24).is_err());
}

#[test]
fn replace_minute() {
    assert_eq!(
        time!(1:02:03.004_005_006).replace_minute(7),
        Ok(time!(1:07:03.004_005_006))
    );
    assert!(time!(1:02:03.004_005_006).replace_minute(60).is_err());
}

#[test]
fn replace_second() {
    assert_eq!(
        time!(1:02:03.004_005_006).replace_second(7),
        Ok(time!(1:02:07.004_005_006))
    );
    assert!(time!(1:02:03.004_005_006).replace_second(60).is_err());
}

#[test]
fn replace_millisecond() {
    assert_eq!(
        time!(1:02:03.004_005_006).replace_millisecond(7),
        Ok(time!(1:02:03.007))
    );
    assert!(
        time!(1:02:03.004_005_006)
            .replace_millisecond(1_000)
            .is_err()
    );
}

#[test]
fn replace_millisecond_regression() {
    assert!(Time::MIDNIGHT.replace_millisecond(9999).is_err());
    assert!(Time::MIDNIGHT.replace_millisecond(4294).is_err());
    assert!(Time::MIDNIGHT.replace_millisecond(4295).is_err());
}

#[test]
fn replace_microsecond() {
    assert_eq!(
        time!(1:02:03.004_005_006).replace_microsecond(7_008),
        Ok(time!(1:02:03.007_008))
    );
    assert!(
        time!(1:02:03.004_005_006)
            .replace_microsecond(1_000_000)
            .is_err()
    );
}

#[test]
fn replace_nanosecond() {
    assert_eq!(
        time!(1:02:03.004_005_006).replace_nanosecond(7_008_009),
        Ok(time!(1:02:03.007_008_009))
    );
    assert!(
        time!(1:02:03.004_005_006)
            .replace_nanosecond(1_000_000_000)
            .is_err()
    );
}

#[test]
fn truncate_to_hour() {
    assert_eq!(time!(1:02:03.004_005_006).truncate_to_hour(), time!(1:00));
    assert_eq!(Time::MIDNIGHT.truncate_to_hour(), Time::MIDNIGHT);
}

#[test]
fn truncate_to_minute() {
    assert_eq!(time!(1:02:03.004_005_006).truncate_to_minute(), time!(1:02));
    assert_eq!(Time::MIDNIGHT.truncate_to_minute(), Time::MIDNIGHT);
}

#[test]
fn truncate_to_second() {
    assert_eq!(
        time!(1:02:03.004_005_006).truncate_to_second(),
        time!(1:02:03)
    );
    assert_eq!(Time::MIDNIGHT.truncate_to_second(), Time::MIDNIGHT);
}

#[test]
fn truncate_to_millisecond() {
    assert_eq!(
        time!(1:02:03.004_005_006).truncate_to_millisecond(),
        time!(1:02:03.004)
    );
    assert_eq!(Time::MIDNIGHT.truncate_to_millisecond(), Time::MIDNIGHT);
}

#[test]
fn truncate_to_microsecond() {
    assert_eq!(
        time!(1:02:03.004_005_006).truncate_to_microsecond(),
        time!(1:02:03.004_005)
    );
    assert_eq!(Time::MIDNIGHT.truncate_to_microsecond(), Time::MIDNIGHT);
}

#[test]
fn add_duration() {
    assert_eq!(time!(0:00) + 1.seconds(), time!(0:00:01));
    assert_eq!(time!(0:00) + 1.minutes(), time!(0:01));
    assert_eq!(time!(0:00) + 1.hours(), time!(1:00));
    assert_eq!(time!(0:00) + 1.days(), time!(0:00));
}

#[test]
fn add_assign_duration() {
    let mut time = time!(0:00);

    time += 1.seconds();
    assert_eq!(time, time!(0:00:01));

    time += 1.minutes();
    assert_eq!(time, time!(0:01:01));

    time += 1.hours();
    assert_eq!(time, time!(1:01:01));

    time += 1.days();
    assert_eq!(time, time!(1:01:01));
}

#[test]
fn sub_duration() {
    assert_eq!(time!(12:00) - 1.hours(), time!(11:00));

    // Underflow
    assert_eq!(time!(0:00) - 1.seconds(), time!(23:59:59));
    assert_eq!(time!(0:00) - 1.minutes(), time!(23:59));
    assert_eq!(time!(0:00) - 1.hours(), time!(23:00));
    assert_eq!(time!(0:00) - 1.days(), time!(0:00));
}

#[test]
fn sub_assign_duration() {
    let mut time = time!(0:00);

    time -= 1.seconds();
    assert_eq!(time, time!(23:59:59));

    time -= 1.minutes();
    assert_eq!(time, time!(23:58:59));

    time -= 1.hours();
    assert_eq!(time, time!(22:58:59));

    time -= 1.days();
    assert_eq!(time, time!(22:58:59));
}

#[test]
fn add_std_duration() {
    assert_eq!(time!(0:00) + 1.std_milliseconds(), time!(0:00:00.001));
    assert_eq!(time!(0:00) + 1.std_seconds(), time!(0:00:01));
    assert_eq!(time!(0:00) + 1.std_minutes(), time!(0:01));
    assert_eq!(time!(0:00) + 1.std_hours(), time!(1:00));
    assert_eq!(time!(0:00) + 1.std_days(), time!(0:00));
}

#[test]
fn add_assign_std_duration() {
    let mut time = time!(0:00);

    time += 1.std_seconds();
    assert_eq!(time, time!(0:00:01));

    time += 1.std_minutes();
    assert_eq!(time, time!(0:01:01));

    time += 1.std_hours();
    assert_eq!(time, time!(1:01:01));

    time += 1.std_days();
    assert_eq!(time, time!(1:01:01));
}

#[test]
fn sub_std_duration() {
    assert_eq!(time!(12:00) - 1.std_hours(), time!(11:00));

    // Underflow
    assert_eq!(time!(0:00) - 1.std_milliseconds(), time!(23:59:59.999));
    assert_eq!(time!(0:00) - 1.std_seconds(), time!(23:59:59));
    assert_eq!(time!(0:00) - 1.std_minutes(), time!(23:59));
    assert_eq!(time!(0:00) - 1.std_hours(), time!(23:00));
    assert_eq!(time!(0:00) - 1.std_days(), time!(0:00));
}

#[test]
fn sub_assign_std_duration() {
    let mut time = time!(0:00);

    time -= 1.std_seconds();
    assert_eq!(time, time!(23:59:59));

    time -= 1.std_minutes();
    assert_eq!(time, time!(23:58:59));

    time -= 1.std_hours();
    assert_eq!(time, time!(22:58:59));

    time -= 1.std_days();
    assert_eq!(time, time!(22:58:59));
}

#[test]
fn sub_time() {
    assert_eq!(time!(0:00) - time!(0:00), 0.seconds());
    assert_eq!(time!(1:00) - time!(0:00), 1.hours());
    assert_eq!(time!(1:00) - time!(0:00:01), 59.minutes() + 59.seconds());
}

#[test]
fn ordering() {
    assert!(time!(0:00) < time!(0:00:00.000_000_001));
    assert!(time!(0:00) < time!(0:00:01));
    assert!(time!(12:00) > time!(11:00));
    assert_eq!(time!(0:00), time!(0:00));
}

#[test]
fn ordering_lexico_endianness() {
    // Endianness of nanoseconds
    assert!(time!(00:00:00.4) > time!(00:00:00.1));
    // Endianness of one field wrt the others
    // Hourt wrt others
    assert!(time!(01:00:00) > time!(00:01:00.0));
    assert!(time!(01:00:00) > time!(00:00:01.0));
    assert!(time!(01:00:00) > time!(00:00:00.1));
    // Minutes wrt to others
    assert!(time!(00:01:00) > time!(00:00:01.0));
    assert!(time!(00:01:00) > time!(00:00:00.1));
    // Second wrt to others
    assert!(time!(00:00:01) > time!(00:00:00.1));
}

#[test]
fn issue_481() {
    assert_eq!(time!(0:00) - time!(01:00:00.1), (-3600.1).seconds());
    assert_eq!(
        time!(0:00) - time!(23:59:59.999_999_999),
        (-86_399.999_999_999).seconds()
    );
}
