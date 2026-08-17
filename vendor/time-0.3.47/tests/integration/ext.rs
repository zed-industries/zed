mod numerical_duration {
    use time::ext::NumericalDuration;
    use time::Duration;

    #[test]
    fn unsigned() {
        assert_eq!(5.nanoseconds(), Duration::nanoseconds(5));
        assert_eq!(5.microseconds(), Duration::microseconds(5));
        assert_eq!(5.milliseconds(), Duration::milliseconds(5));
        assert_eq!(5.seconds(), Duration::seconds(5));
        assert_eq!(5.minutes(), Duration::minutes(5));
        assert_eq!(5.hours(), Duration::hours(5));
        assert_eq!(5.days(), Duration::days(5));
        assert_eq!(5.weeks(), Duration::weeks(5));
    }

    #[test]
    fn signed() {
        assert_eq!((-5).nanoseconds(), Duration::nanoseconds(-5));
        assert_eq!((-5).microseconds(), Duration::microseconds(-5));
        assert_eq!((-5).milliseconds(), Duration::milliseconds(-5));
        assert_eq!((-5).seconds(), Duration::seconds(-5));
        assert_eq!((-5).minutes(), Duration::minutes(-5));
        assert_eq!((-5).hours(), Duration::hours(-5));
        assert_eq!((-5).days(), Duration::days(-5));
        assert_eq!((-5).weeks(), Duration::weeks(-5));
    }

    #[test]
    fn float() {
        // Ensure values truncate rather than round.
        assert_eq!(1.9.nanoseconds(), Duration::nanoseconds(1));

        assert_eq!(1.0.nanoseconds(), Duration::nanoseconds(1));
        assert_eq!(1.0.microseconds(), Duration::microseconds(1));
        assert_eq!(1.0.milliseconds(), Duration::milliseconds(1));
        assert_eq!(1.0.seconds(), Duration::seconds(1));
        assert_eq!(1.0.minutes(), Duration::minutes(1));
        assert_eq!(1.0.hours(), Duration::hours(1));
        assert_eq!(1.0.days(), Duration::days(1));
        assert_eq!(1.0.weeks(), Duration::weeks(1));

        assert_eq!(1.5.nanoseconds(), Duration::nanoseconds(1));
        assert_eq!(1.5.microseconds(), Duration::nanoseconds(1_500));
        assert_eq!(1.5.milliseconds(), Duration::microseconds(1_500));
        assert_eq!(1.5.seconds(), Duration::milliseconds(1_500));
        assert_eq!(1.5.minutes(), Duration::seconds(90));
        assert_eq!(1.5.hours(), Duration::minutes(90));
        assert_eq!(1.5.days(), Duration::hours(36));
        assert_eq!(1.5.weeks(), Duration::hours(252));
    }

    #[test]
    fn arithmetic() {
        assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
        assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
    }
}

mod numerical_std_duration {
    use std::time::Duration as StdDuration;

    use time::ext::NumericalStdDuration;

    #[test]
    fn unsigned() {
        assert_eq!(5.std_nanoseconds(), StdDuration::from_nanos(5));
        assert_eq!(5.std_microseconds(), StdDuration::from_micros(5));
        assert_eq!(5.std_milliseconds(), StdDuration::from_millis(5));
        assert_eq!(5.std_seconds(), StdDuration::from_secs(5));
        assert_eq!(5.std_minutes(), StdDuration::from_secs(5 * 60));
        assert_eq!(5.std_hours(), StdDuration::from_secs(5 * 3_600));
        assert_eq!(5.std_days(), StdDuration::from_secs(5 * 86_400));
        assert_eq!(5.std_weeks(), StdDuration::from_secs(5 * 604_800));
    }

    #[test]
    fn float() {
        // Ensure values truncate rather than round.
        assert_eq!(1.9.std_nanoseconds(), StdDuration::from_nanos(1));

        assert_eq!(1.0.std_nanoseconds(), StdDuration::from_nanos(1));
        assert_eq!(1.0.std_microseconds(), StdDuration::from_micros(1));
        assert_eq!(1.0.std_milliseconds(), StdDuration::from_millis(1));
        assert_eq!(1.0.std_seconds(), StdDuration::from_secs(1));
        assert_eq!(1.0.std_minutes(), StdDuration::from_secs(60));
        assert_eq!(1.0.std_hours(), StdDuration::from_secs(3_600));
        assert_eq!(1.0.std_days(), StdDuration::from_secs(86_400));
        assert_eq!(1.0.std_weeks(), StdDuration::from_secs(604_800));

        assert_eq!(1.5.std_nanoseconds(), StdDuration::from_nanos(1));
        assert_eq!(1.5.std_microseconds(), StdDuration::from_nanos(1_500));
        assert_eq!(1.5.std_milliseconds(), StdDuration::from_micros(1_500));
        assert_eq!(1.5.std_seconds(), StdDuration::from_millis(1_500));
        assert_eq!(1.5.std_minutes(), StdDuration::from_secs(90));
        assert_eq!(1.5.std_hours(), StdDuration::from_secs(90 * 60));
        assert_eq!(1.5.std_days(), StdDuration::from_secs(36 * 3_600));
        assert_eq!(1.5.std_weeks(), StdDuration::from_secs(252 * 3_600));
    }

    #[test]
    fn arithmetic() {
        assert_eq!(
            2.std_seconds() + 500.std_milliseconds(),
            2_500.std_milliseconds()
        );
        assert_eq!(
            2.std_seconds() - 500.std_milliseconds(),
            1_500.std_milliseconds()
        );
    }
}
