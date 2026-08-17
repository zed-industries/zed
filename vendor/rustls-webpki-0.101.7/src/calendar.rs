// Copyright 2015-2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use super::{time::Time, Error};

pub(crate) fn time_from_ymdhms_utc(
    year: u64,
    month: u64,
    day_of_month: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
) -> Result<Time, Error> {
    let days_before_year_since_unix_epoch = days_before_year_since_unix_epoch(year)?;

    const JAN: u64 = 31;
    let feb = days_in_feb(year);
    const MAR: u64 = 31;
    const APR: u64 = 30;
    const MAY: u64 = 31;
    const JUN: u64 = 30;
    const JUL: u64 = 31;
    const AUG: u64 = 31;
    const SEP: u64 = 30;
    const OCT: u64 = 31;
    const NOV: u64 = 30;
    let days_before_month_in_year = match month {
        1 => 0,
        2 => JAN,
        3 => JAN + feb,
        4 => JAN + feb + MAR,
        5 => JAN + feb + MAR + APR,
        6 => JAN + feb + MAR + APR + MAY,
        7 => JAN + feb + MAR + APR + MAY + JUN,
        8 => JAN + feb + MAR + APR + MAY + JUN + JUL,
        9 => JAN + feb + MAR + APR + MAY + JUN + JUL + AUG,
        10 => JAN + feb + MAR + APR + MAY + JUN + JUL + AUG + SEP,
        11 => JAN + feb + MAR + APR + MAY + JUN + JUL + AUG + SEP + OCT,
        12 => JAN + feb + MAR + APR + MAY + JUN + JUL + AUG + SEP + OCT + NOV,
        _ => unreachable!(), // `read_two_digits` already bounds-checked it.
    };

    let days_before =
        days_before_year_since_unix_epoch + days_before_month_in_year + day_of_month - 1;

    let seconds_since_unix_epoch =
        (days_before * 24 * 60 * 60) + (hours * 60 * 60) + (minutes * 60) + seconds;

    Ok(Time::from_seconds_since_unix_epoch(
        seconds_since_unix_epoch,
    ))
}

fn days_before_year_since_unix_epoch(year: u64) -> Result<u64, Error> {
    // We don't support dates before January 1, 1970 because that is the
    // Unix epoch. It is likely that other software won't deal well with
    // certificates that have dates before the epoch.
    if year < UNIX_EPOCH_YEAR {
        return Err(Error::BadDerTime);
    }
    let days_before_year_ad = days_before_year_ad(year);
    debug_assert!(days_before_year_ad >= DAYS_BEFORE_UNIX_EPOCH_AD);
    Ok(days_before_year_ad - DAYS_BEFORE_UNIX_EPOCH_AD)
}

const UNIX_EPOCH_YEAR: u64 = 1970;

fn days_before_year_ad(year: u64) -> u64 {
    ((year - 1) * 365)
        + ((year - 1) / 4)    // leap years are every 4 years,
        - ((year - 1) / 100)  // except years divisible by 100,
        + ((year - 1) / 400) // except years divisible by 400.
}

pub(crate) fn days_in_month(year: u64, month: u64) -> u64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => days_in_feb(year),
        _ => unreachable!(), // `read_two_digits` already bounds-checked it.
    }
}

fn days_in_feb(year: u64) -> u64 {
    if (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0)) {
        29
    } else {
        28
    }
}

#[allow(clippy::unreadable_literal)] // TODO: Make this clear.
const DAYS_BEFORE_UNIX_EPOCH_AD: u64 = 719162;

#[cfg(test)]
mod tests {
    #[test]
    fn test_days_before_unix_epoch() {
        use super::{days_before_year_ad, DAYS_BEFORE_UNIX_EPOCH_AD, UNIX_EPOCH_YEAR};
        assert_eq!(
            DAYS_BEFORE_UNIX_EPOCH_AD,
            days_before_year_ad(UNIX_EPOCH_YEAR)
        );
    }

    #[test]
    fn test_days_before_year_since_unix_epoch() {
        use super::{days_before_year_since_unix_epoch, Error, UNIX_EPOCH_YEAR};
        assert_eq!(Ok(0), days_before_year_since_unix_epoch(UNIX_EPOCH_YEAR));
        assert_eq!(
            Ok(365),
            days_before_year_since_unix_epoch(UNIX_EPOCH_YEAR + 1)
        );
        assert_eq!(
            Err(Error::BadDerTime),
            days_before_year_since_unix_epoch(UNIX_EPOCH_YEAR - 1)
        );
    }

    #[test]
    fn test_days_in_month() {
        use super::days_in_month;
        assert_eq!(days_in_month(2017, 1), 31);
        assert_eq!(days_in_month(2017, 2), 28);
        assert_eq!(days_in_month(2017, 3), 31);
        assert_eq!(days_in_month(2017, 4), 30);
        assert_eq!(days_in_month(2017, 5), 31);
        assert_eq!(days_in_month(2017, 6), 30);
        assert_eq!(days_in_month(2017, 7), 31);
        assert_eq!(days_in_month(2017, 8), 31);
        assert_eq!(days_in_month(2017, 9), 30);
        assert_eq!(days_in_month(2017, 10), 31);
        assert_eq!(days_in_month(2017, 11), 30);
        assert_eq!(days_in_month(2017, 12), 31);

        // leap cases
        assert_eq!(days_in_month(2000, 2), 29);
        assert_eq!(days_in_month(2004, 2), 29);
        assert_eq!(days_in_month(2016, 2), 29);
        assert_eq!(days_in_month(2100, 2), 28);
    }

    #[allow(clippy::unreadable_literal)] // TODO: Make this clear.
    #[test]
    fn test_time_from_ymdhms_utc() {
        use super::{time_from_ymdhms_utc, Error, Time, UNIX_EPOCH_YEAR};

        // 1969-12-31 00:00:00
        assert_eq!(
            Err(Error::BadDerTime),
            time_from_ymdhms_utc(UNIX_EPOCH_YEAR - 1, 1, 1, 0, 0, 0)
        );

        // 1969-12-31 23:59:59
        assert_eq!(
            Err(Error::BadDerTime),
            time_from_ymdhms_utc(UNIX_EPOCH_YEAR - 1, 12, 31, 23, 59, 59)
        );

        // 1970-01-01 00:00:00
        assert_eq!(
            Time::from_seconds_since_unix_epoch(0),
            time_from_ymdhms_utc(UNIX_EPOCH_YEAR, 1, 1, 0, 0, 0).unwrap()
        );

        // 1970-01-01 00:00:01
        assert_eq!(
            Time::from_seconds_since_unix_epoch(1),
            time_from_ymdhms_utc(UNIX_EPOCH_YEAR, 1, 1, 0, 0, 1).unwrap()
        );

        // 1971-01-01 00:00:00
        assert_eq!(
            Time::from_seconds_since_unix_epoch(365 * 86400),
            time_from_ymdhms_utc(UNIX_EPOCH_YEAR + 1, 1, 1, 0, 0, 0).unwrap()
        );

        // year boundary
        assert_eq!(
            Time::from_seconds_since_unix_epoch(1483228799),
            time_from_ymdhms_utc(2016, 12, 31, 23, 59, 59).unwrap()
        );
        assert_eq!(
            Time::from_seconds_since_unix_epoch(1483228800),
            time_from_ymdhms_utc(2017, 1, 1, 0, 0, 0).unwrap()
        );

        // not a leap year
        assert_eq!(
            Time::from_seconds_since_unix_epoch(1492449162),
            time_from_ymdhms_utc(2017, 4, 17, 17, 12, 42).unwrap()
        );

        // leap year, post-feb
        assert_eq!(
            Time::from_seconds_since_unix_epoch(1460913162),
            time_from_ymdhms_utc(2016, 4, 17, 17, 12, 42).unwrap()
        );
    }
}
