use std::io;
use std::num::NonZero;

use time::format_description::well_known::iso8601::{DateKind, OffsetPrecision, TimePrecision};
use time::format_description::well_known::{Iso8601, Rfc2822, Rfc3339, iso8601};
use time::format_description::{self, BorrowedFormatItem, OwnedFormatItem};
use time::macros::{date, datetime, format_description as fd, offset, time, utc_datetime};
use time::{OffsetDateTime, Time};

#[test]
fn rfc_2822() -> time::Result<()> {
    assert_eq!(
        datetime!(2021-01-02 03:04:05 UTC).format(&Rfc2822)?,
        "Sat, 02 Jan 2021 03:04:05 +0000"
    );
    assert_eq!(
        utc_datetime!(2021-01-02 03:04:05).format(&Rfc2822)?,
        "Sat, 02 Jan 2021 03:04:05 +0000"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05 +06:07).format(&Rfc2822)?,
        "Sat, 02 Jan 2021 03:04:05 +0607"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05 -06:07).format(&Rfc2822)?,
        "Sat, 02 Jan 2021 03:04:05 -0607"
    );

    assert!(matches!(
        datetime!(1885-01-01 01:01:01 UTC).format(&Rfc2822),
        Err(time::error::Format::InvalidComponent("year"))
    ));
    assert!(matches!(
        utc_datetime!(1885-01-01 01:01:01).format(&Rfc2822),
        Err(time::error::Format::InvalidComponent("year"))
    ));
    assert!(matches!(
        datetime!(2000-01-01 00:00:00 +00:00:01).format(&Rfc2822),
        Err(time::error::Format::InvalidComponent("offset_second"))
    ));

    Ok(())
}

#[test]
fn rfc_3339() -> time::Result<()> {
    assert_eq!(
        datetime!(2021-01-02 03:04:05 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.1 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.1Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.12 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.12Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.123Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_4 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.1234Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_45 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.12345Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.123456Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456_7 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.1234567Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456_78 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.12345678Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456_789 UTC).format(&Rfc3339)?,
        "2021-01-02T03:04:05.123456789Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456_789 -01:02).format(&Rfc3339)?,
        "2021-01-02T03:04:05.123456789-01:02"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05.123_456_789 +01:02).format(&Rfc3339)?,
        "2021-01-02T03:04:05.123456789+01:02"
    );

    assert!(matches!(
        datetime!(-0001-01-01 0:00 UTC).format(&Rfc3339),
        Err(time::error::Format::InvalidComponent("year"))
    ));
    assert!(matches!(
        datetime!(0000-01-01 0:00 +00:00:01).format(&Rfc3339),
        Err(time::error::Format::InvalidComponent("offset_second"))
    ));

    Ok(())
}

#[test]
fn iso_8601() -> time::Result<()> {
    macro_rules! assert_format_config {
        ($formatted:literal $(, $($config:tt)+)?) => {
            assert_eq!(
                datetime!(2021-01-02 03:04:05 UTC).format(
                    &Iso8601::<{ iso8601::Config::DEFAULT$($($config)+)?.encode() }>
                )?,
                $formatted
            );
        };
    }

    assert_eq!(
        datetime!(-123_456-01-02 03:04:05 UTC).format(
            &Iso8601::<
                {
                    iso8601::Config::DEFAULT
                        .set_year_is_six_digits(true)
                        .encode()
                },
            >
        )?,
        "-123456-01-02T03:04:05.000000000Z"
    );
    assert_eq!(
        datetime!(-123_456-01-02 03:04:05 UTC).format(
            &Iso8601::<
                {
                    iso8601::Config::DEFAULT
                        .set_date_kind(DateKind::Ordinal)
                        .set_year_is_six_digits(true)
                        .encode()
                },
            >
        )?,
        "-123456-002T03:04:05.000000000Z"
    );
    assert_eq!(
        datetime!(-123_456-01-02 03:04:05 UTC).format(
            &Iso8601::<
                {
                    iso8601::Config::DEFAULT
                        .set_date_kind(DateKind::Week)
                        .set_year_is_six_digits(true)
                        .encode()
                },
            >
        )?,
        "-123456-W01-4T03:04:05.000000000Z"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05+1:00).format(&Iso8601::DEFAULT)?,
        "2021-01-02T03:04:05.000000000+01:00"
    );
    assert_eq!(
        datetime!(2021-01-02 03:04:05+1:00).format(
            &Iso8601::<
                {
                    iso8601::Config::DEFAULT
                        .set_offset_precision(OffsetPrecision::Hour)
                        .encode()
                },
            >
        )?,
        "2021-01-02T03:04:05.000000000+01"
    );
    assert_format_config!("2021-01-02T03:04:05.000000000Z");
    assert_format_config!("20210102T030405.000000000Z", .set_use_separators(false));
    assert_format_config!("+002021-01-02T03:04:05.000000000Z", .set_year_is_six_digits(true));
    assert_format_config!("2021-01-02T03Z", .set_time_precision(TimePrecision::Hour { decimal_digits: None }));
    assert_format_config!("2021-01-02T03:04Z", .set_time_precision(TimePrecision::Minute { decimal_digits: None }));
    assert_format_config!("2021-01-02T03:04:05Z", .set_time_precision(TimePrecision::Second { decimal_digits: None }));
    assert_format_config!("2021-002T03:04:05.000000000Z", .set_date_kind(DateKind::Ordinal));
    assert_format_config!("2020-W53-6T03:04:05.000000000Z", .set_date_kind(DateKind::Week));

    assert!(matches!(
        datetime!(+10_000-01-01 0:00 UTC).format(&Iso8601::DEFAULT),
        Err(time::error::Format::InvalidComponent("year"))
    ));
    assert!(matches!(
        datetime!(+10_000-W01-1 0:00 UTC).format(
            &Iso8601::<
                {
                    iso8601::Config::DEFAULT
                        .set_date_kind(DateKind::Week)
                        .encode()
                },
            >
        ),
        Err(time::error::Format::InvalidComponent("year"))
    ));
    assert!(matches!(
        datetime!(+10_000-001 0:00 UTC).format(
            &Iso8601::<
                {
                    iso8601::Config::DEFAULT
                        .set_date_kind(DateKind::Ordinal)
                        .encode()
                },
            >
        ),
        Err(time::error::Format::InvalidComponent("year"))
    ));
    assert!(matches!(
        datetime!(2021-01-02 03:04:05 +0:00:01).format(&Iso8601::DEFAULT),
        Err(time::error::Format::InvalidComponent("offset_second"))
    ));
    assert!(matches!(
        datetime!(2021-01-02 03:04:05 +0:01).format(
            &Iso8601::<
                {
                    iso8601::Config::DEFAULT
                        .set_offset_precision(OffsetPrecision::Hour)
                        .encode()
                },
            >
        ),
        Err(time::error::Format::InvalidComponent("offset_minute"))
    ));

    Ok(())
}

#[test]
fn iso_8601_issue_678() -> time::Result<()> {
    macro_rules! assert_format_config {
        ($formatted:literal $(, $($config:tt)+)?) => {
            assert_eq!(
                datetime!(2021-01-02 03:04:05.999_999_999 UTC).format(
                    &Iso8601::<{ iso8601::Config::DEFAULT$($($config)+)?.encode() }>
                )?,
                $formatted
            );
        };
    }

    assert_format_config!("2021-01-02T03:04:05.999999999Z", .set_time_precision(TimePrecision::Second { decimal_digits: NonZero::new(9) }));
    assert_format_config!("2021-01-02T03:04:05.999999Z", .set_time_precision(TimePrecision::Second { decimal_digits: NonZero::new(6) }));
    assert_format_config!("2021-01-02T03:04:05.999Z", .set_time_precision(TimePrecision::Second { decimal_digits: NonZero::new(3) }));

    Ok(())
}

#[test]
fn format_time() -> time::Result<()> {
    let format_output = [
        (fd!("[hour]"), "13"),
        (fd!("[hour repr:12]"), "01"),
        (fd!("[hour repr:12 padding:none]"), "1"),
        (fd!("[hour repr:12 padding:space]"), " 1"),
        (fd!("[hour repr:24]"), "13"),
        (fd!("[hour repr:24]"), "13"),
        (fd!("[hour repr:24 padding:none]"), "13"),
        (fd!("[hour repr:24 padding:space]"), "13"),
        (fd!("[minute]"), "02"),
        (fd!("[minute padding:none]"), "2"),
        (fd!("[minute padding:space]"), " 2"),
        (fd!("[minute padding:zero]"), "02"),
        (fd!("[period]"), "PM"),
        (fd!("[period case:upper]"), "PM"),
        (fd!("[period case:lower]"), "pm"),
        (fd!("[second]"), "03"),
        (fd!("[second padding:none]"), "3"),
        (fd!("[second padding:space]"), " 3"),
        (fd!("[second padding:zero]"), "03"),
        (fd!("[subsecond]"), "456789012"),
        (fd!("[subsecond digits:1]"), "4"),
        (fd!("[subsecond digits:2]"), "45"),
        (fd!("[subsecond digits:3]"), "456"),
        (fd!("[subsecond digits:4]"), "4567"),
        (fd!("[subsecond digits:5]"), "45678"),
        (fd!("[subsecond digits:6]"), "456789"),
        (fd!("[subsecond digits:7]"), "4567890"),
        (fd!("[subsecond digits:8]"), "45678901"),
        (fd!("[subsecond digits:9]"), "456789012"),
        (fd!("[subsecond digits:1+]"), "456789012"),
    ];

    for &(format_description, output) in &format_output {
        assert_eq!(
            time!(13:02:03.456_789_012).format(format_description)?,
            output
        );
        assert!(
            time!(13:02:03.456_789_012)
                .format_into(&mut io::sink(), format_description)
                .is_ok()
        );
        assert_eq!(
            time!(13:02:03.456_789_012).format(&OwnedFormatItem::from(format_description))?,
            output
        );
        assert!(
            time!(13:02:03.456_789_012)
                .format_into(&mut io::sink(), &OwnedFormatItem::from(format_description))
                .is_ok()
        );
    }

    assert_eq!(
        time!(1:02:03).format(fd!("[hour repr:12][period]"))?,
        "01AM"
    );
    assert_eq!(
        Time::MIDNIGHT.format(fd!("[hour repr:12][period case:lower]"))?,
        "12am"
    );
    assert_eq!(Time::MIDNIGHT.format(fd!("[subsecond digits:1+]"))?, "0");
    assert_eq!(
        time!(0:00:00.01).format(fd!("[subsecond digits:1+]"))?,
        "01"
    );
    assert_eq!(
        time!(0:00:00.001).format(fd!("[subsecond digits:1+]"))?,
        "001"
    );
    assert_eq!(
        time!(0:00:00.0001).format(fd!("[subsecond digits:1+]"))?,
        "0001"
    );
    assert_eq!(
        time!(0:00:00.00001).format(fd!("[subsecond digits:1+]"))?,
        "00001"
    );
    assert_eq!(
        time!(0:00:00.000001).format(fd!("[subsecond digits:1+]"))?,
        "000001"
    );
    assert_eq!(
        time!(0:00:00.0000001).format(fd!("[subsecond digits:1+]"))?,
        "0000001"
    );
    assert_eq!(
        time!(0:00:00.00000001).format(fd!("[subsecond digits:1+]"))?,
        "00000001"
    );
    assert_eq!(
        time!(0:00:00.000000001).format(fd!("[subsecond digits:1+]"))?,
        "000000001"
    );

    Ok(())
}

#[test]
fn display_time() {
    assert_eq!(time!(0:00).to_string(), "0:00:00.0");
    assert_eq!(time!(23:59).to_string(), "23:59:00.0");
    assert_eq!(time!(23:59:59).to_string(), "23:59:59.0");
    assert_eq!(time!(0:00:01).to_string(), "0:00:01.0");
    assert_eq!(time!(0:00:00.1).to_string(), "0:00:00.1");
    assert_eq!(time!(0:00:00.01).to_string(), "0:00:00.01");
    assert_eq!(time!(0:00:00.001).to_string(), "0:00:00.001");
    assert_eq!(time!(0:00:00.000_1).to_string(), "0:00:00.0001");
    assert_eq!(time!(0:00:00.000_01).to_string(), "0:00:00.00001");
    assert_eq!(time!(0:00:00.000_001).to_string(), "0:00:00.000001");
    assert_eq!(time!(0:00:00.000_000_1).to_string(), "0:00:00.0000001");
    assert_eq!(time!(0:00:00.000_000_01).to_string(), "0:00:00.00000001");
    assert_eq!(time!(0:00:00.000_000_001).to_string(), "0:00:00.000000001");

    assert_eq!(format!("{:>12}", time!(0:00)), "   0:00:00.0");
    assert_eq!(format!("{:x^14}", time!(0:00)), "xx0:00:00.0xxx");
}

#[test]
fn format_date() -> time::Result<()> {
    let format_output = [
        (fd!("[day]"), "31"),
        (fd!("[month]"), "12"),
        (fd!("[month repr:short]"), "Dec"),
        (fd!("[month repr:long]"), "December"),
        (fd!("[ordinal]"), "365"),
        (fd!("[weekday]"), "Tuesday"),
        (fd!("[weekday repr:short]"), "Tue"),
        (fd!("[weekday repr:sunday]"), "3"),
        (fd!("[weekday repr:sunday one_indexed:false]"), "2"),
        (fd!("[weekday repr:monday]"), "2"),
        (fd!("[weekday repr:monday one_indexed:false]"), "1"),
        (fd!("[week_number]"), "01"),
        (fd!("[week_number padding:none]"), "1"),
        (fd!("[week_number padding:space]"), " 1"),
        (fd!("[week_number repr:sunday]"), "52"),
        (fd!("[week_number repr:monday]"), "52"),
        (fd!("[year]"), "2019"),
        (fd!("[year base:iso_week]"), "2020"),
        (fd!("[year sign:mandatory]"), "+2019"),
        (fd!("[year base:iso_week sign:mandatory]"), "+2020"),
        (fd!("[year repr:century]"), "20"),
        (fd!("[year repr:last_two]"), "19"),
        (fd!("[year base:iso_week repr:last_two]"), "20"),
        (fd!("[year range:standard]"), "2019"),
        (fd!("[year range:standard repr:century]"), "20"),
        (fd!("[year range:standard repr:last_two]"), "19"),
    ];

    for &(format_description, output) in &format_output {
        assert_eq!(date!(2019-12-31).format(format_description)?, output);
        assert!(
            date!(2019-12-31)
                .format_into(&mut io::sink(), format_description)
                .is_ok()
        );
        assert_eq!(
            date!(2019-12-31).format(&OwnedFormatItem::from(format_description))?,
            output
        );
        assert!(
            date!(2019-12-31)
                .format_into(&mut io::sink(), &OwnedFormatItem::from(format_description))
                .is_ok()
        );
    }

    Ok(())
}

#[test]
fn format_date_err() {
    assert!(matches!(
        date!(+10_000-01-01).format(fd!("[year range:standard]")),
        Err(time::error::Format::ComponentRange(cr)) if cr.name() == "year"
    ));
    assert!(matches!(
        date!(+10_000-01-01).format(fd!("[year repr:century range:standard]")),
        Err(time::error::Format::ComponentRange(cr)) if cr.name() == "year"
    ));
}

#[test]
fn display_date() {
    assert_eq!(date!(2019-01-01).to_string(), "2019-01-01");
    assert_eq!(date!(2019-12-31).to_string(), "2019-12-31");
    assert_eq!(date!(-4713-11-24).to_string(), "-4713-11-24");
    assert_eq!(date!(-0001-01-01).to_string(), "-0001-01-01");

    assert_eq!(date!(+10_000-01-01).to_string(), "+10000-01-01");
    assert_eq!(date!(+100_000-01-01).to_string(), "+100000-01-01");
    assert_eq!(date!(-10_000-01-01).to_string(), "-10000-01-01");
    assert_eq!(date!(-100_000-01-01).to_string(), "-100000-01-01");
}

#[test]
fn format_offset() -> time::Result<()> {
    let value_format_output = [
        (
            offset!(+01:02:03),
            fd!("[offset_hour sign:automatic]"),
            "01",
        ),
        (
            offset!(+01:02:03),
            fd!("[offset_hour sign:mandatory]"),
            "+01",
        ),
        (
            offset!(-01:02:03),
            fd!("[offset_hour sign:automatic]"),
            "-01",
        ),
        (
            offset!(-01:02:03),
            fd!("[offset_hour sign:mandatory]"),
            "-01",
        ),
        (offset!(+01:02:03), fd!("[offset_minute]"), "02"),
        (offset!(+01:02:03), fd!("[offset_second]"), "03"),
    ];

    for &(value, format_description, output) in &value_format_output {
        assert_eq!(value.format(format_description)?, output);
        assert!(
            value
                .format_into(&mut io::sink(), format_description)
                .is_ok()
        );
        assert_eq!(
            value.format(&OwnedFormatItem::from(format_description))?,
            output
        );
        assert!(
            value
                .format_into(&mut io::sink(), &OwnedFormatItem::from(format_description))
                .is_ok()
        );
    }

    Ok(())
}

#[test]
fn display_offset() {
    assert_eq!(offset!(UTC).to_string(), "+00:00:00");
    assert_eq!(offset!(+0:00:01).to_string(), "+00:00:01");
    assert_eq!(offset!(-0:00:01).to_string(), "-00:00:01");
    assert_eq!(offset!(+1).to_string(), "+01:00:00");
    assert_eq!(offset!(-1).to_string(), "-01:00:00");
    assert_eq!(offset!(+23:59).to_string(), "+23:59:00");
    assert_eq!(offset!(-23:59).to_string(), "-23:59:00");
    assert_eq!(offset!(+23:59:59).to_string(), "+23:59:59");
    assert_eq!(offset!(-23:59:59).to_string(), "-23:59:59");

    assert_eq!(format!("{:>10}", offset!(UTC)), " +00:00:00");
    assert_eq!(format!("{:x^14}", offset!(UTC)), "xx+00:00:00xxx");
}

#[test]
fn format_pdt() -> time::Result<()> {
    let format_description = fd!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]");

    assert_eq!(
        datetime!(1970-01-01 0:00).format(format_description)?,
        "1970-01-01 00:00:00.0"
    );
    assert!(
        datetime!(1970-01-01 0:00)
            .format_into(&mut io::sink(), format_description)
            .is_ok()
    );
    assert_eq!(
        datetime!(1970-01-01 0:00).format(&OwnedFormatItem::from(format_description))?,
        "1970-01-01 00:00:00.0"
    );
    assert!(
        datetime!(1970-01-01 0:00)
            .format_into(&mut io::sink(), &OwnedFormatItem::from(format_description))
            .is_ok()
    );

    Ok(())
}

#[test]
fn display_pdt() {
    assert_eq!(
        datetime!(1970-01-01 0:00).to_string(),
        String::from("1970-01-01 0:00:00.0")
    );
    assert_eq!(
        datetime!(1970-01-01 0:00:01).to_string(),
        String::from("1970-01-01 0:00:01.0")
    );
}

#[test]
fn format_odt() -> time::Result<()> {
    let format_description = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:1+] [offset_hour \
         sign:mandatory]:[offset_minute]:[offset_second]",
    )?;

    assert_eq!(
        datetime!(1970-01-01 0:00 UTC).format(&format_description)?,
        "1970-01-01 00:00:00.0 +00:00:00"
    );
    assert!(
        datetime!(1970-01-01 0:00 UTC)
            .format_into(&mut io::sink(), &format_description)
            .is_ok()
    );
    assert_eq!(
        datetime!(1970-01-01 0:00 UTC).format(&OwnedFormatItem::from(&format_description))?,
        "1970-01-01 00:00:00.0 +00:00:00"
    );
    assert!(
        datetime!(1970-01-01 0:00 UTC)
            .format_into(&mut io::sink(), &OwnedFormatItem::from(format_description))
            .is_ok()
    );

    Ok(())
}

#[test]
fn display_odt() {
    assert_eq!(
        datetime!(1970-01-01 0:00 UTC).to_string(),
        "1970-01-01 0:00:00.0 +00:00:00"
    );
}

#[test]
fn format_udt() -> time::Result<()> {
    let format_description = fd!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]");

    assert_eq!(
        utc_datetime!(1970-01-01 0:00).format(format_description)?,
        "1970-01-01 00:00:00.0"
    );
    assert!(
        utc_datetime!(1970-01-01 0:00)
            .format_into(&mut io::sink(), format_description)
            .is_ok()
    );
    assert_eq!(
        utc_datetime!(1970-01-01 0:00).format(&OwnedFormatItem::from(format_description))?,
        "1970-01-01 00:00:00.0"
    );
    assert!(
        utc_datetime!(1970-01-01 0:00)
            .format_into(&mut io::sink(), &OwnedFormatItem::from(format_description))
            .is_ok()
    );

    Ok(())
}

#[test]
fn display_udt() {
    assert_eq!(
        utc_datetime!(1970-01-01 0:00).to_string(),
        "1970-01-01 0:00:00.0 +00"
    );
}

#[test]
fn insufficient_type_information() {
    let assert_insufficient_type_information = |res| {
        assert!(matches!(
            res,
            Err(time::error::Format::InsufficientTypeInformation { .. })
        ));
    };
    assert_insufficient_type_information(Time::MIDNIGHT.format(fd!("[year]")));
    assert_insufficient_type_information(Time::MIDNIGHT.format(&BorrowedFormatItem::First(&[
        BorrowedFormatItem::Compound(fd!("[year]")),
    ])));
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn failed_write() -> time::Result<()> {
    macro_rules! assert_err {
        ($val:expr, $format:expr) => {{
            let val = $val;
            let format = $format;
            let success_len = val.format(&format)?.len();
            for len in 0..success_len {
                let mut buf = &mut vec![0; len][..];
                let res = val.format_into(&mut buf, &format);
                assert!(matches!(
                    res,
                    Err(time::error::Format::StdIo(e)) if e.kind() == io::ErrorKind::WriteZero)
                );
            }
        }};
    }

    assert_err!(Time::MIDNIGHT, fd!("foo"));
    assert_err!(Time::MIDNIGHT, OwnedFormatItem::from(fd!("foo")));
    assert_err!(Time::MIDNIGHT, BorrowedFormatItem::Compound(fd!("foo")));
    assert_err!(
        Time::MIDNIGHT,
        BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(fd!("foo")))
    );
    assert_err!(
        Time::MIDNIGHT,
        OwnedFormatItem::from(BorrowedFormatItem::Optional(&BorrowedFormatItem::Compound(
            fd!("foo")
        )))
    );
    assert_err!(OffsetDateTime::UNIX_EPOCH, Rfc3339);
    assert_err!(datetime!(2021-001 0:00:00.1 UTC), Rfc3339);
    assert_err!(datetime!(2021-001 0:00 +0:01), Rfc3339);
    assert_err!(OffsetDateTime::UNIX_EPOCH, Rfc2822);
    assert_err!(OffsetDateTime::UNIX_EPOCH, Iso8601::DEFAULT);
    assert_err!(datetime!(2021-001 0:00 +0:01), Iso8601::DEFAULT);
    assert_err!(
        OffsetDateTime::UNIX_EPOCH,
        Iso8601::<
            {
                iso8601::Config::DEFAULT
                    .set_year_is_six_digits(true)
                    .encode()
            },
        >
    );
    assert_err!(
        OffsetDateTime::UNIX_EPOCH,
        Iso8601::<
            {
                iso8601::Config::DEFAULT
                    .set_date_kind(DateKind::Ordinal)
                    .encode()
            },
        >
    );
    assert_err!(
        OffsetDateTime::UNIX_EPOCH,
        Iso8601::<
            {
                iso8601::Config::DEFAULT
                    .set_year_is_six_digits(true)
                    .set_date_kind(DateKind::Ordinal)
                    .encode()
            },
        >
    );
    assert_err!(
        OffsetDateTime::UNIX_EPOCH,
        Iso8601::<
            {
                iso8601::Config::DEFAULT
                    .set_year_is_six_digits(true)
                    .set_date_kind(DateKind::Week)
                    .encode()
            },
        >
    );
    assert_err!(
        OffsetDateTime::UNIX_EPOCH,
        Iso8601::<
            {
                iso8601::Config::DEFAULT
                    .set_date_kind(DateKind::Week)
                    .encode()
            },
        >
    );
    assert_err!(
        OffsetDateTime::UNIX_EPOCH,
        Iso8601::<
            {
                iso8601::Config::DEFAULT
                    .set_time_precision(TimePrecision::Minute {
                        decimal_digits: None,
                    })
                    .encode()
            },
        >
    );
    assert_err!(
        OffsetDateTime::UNIX_EPOCH,
        Iso8601::<
            {
                iso8601::Config::DEFAULT
                    .set_time_precision(TimePrecision::Hour {
                        decimal_digits: None,
                    })
                    .encode()
            },
        >
    );

    assert_err!(Time::MIDNIGHT, fd!("[hour padding:space]"));
    assert_err!(offset!(+1), fd!("[offset_hour sign:mandatory]"));
    assert_err!(offset!(-1), fd!("[offset_hour]"));
    assert_err!(date!(-1-001), fd!("[year]"));
    assert_err!(date!(2021-001), fd!("[year sign:mandatory]"));
    assert_err!(date!(+999_999-001), fd!("[year]"));
    assert_err!(date!(+99_999-001), fd!("[year]"));

    let component_names = [
        "day",
        "month",
        "ordinal",
        "weekday",
        "week_number",
        "year",
        "hour",
        "minute",
        "period",
        "second",
        "subsecond",
        "offset_hour",
        "offset_minute",
        "offset_second",
    ];
    for component in &component_names {
        let component = format!("[{component}]");
        assert_err!(
            OffsetDateTime::UNIX_EPOCH,
            format_description::parse(&component)?
        );
    }

    Ok(())
}

#[test]
fn first() -> time::Result<()> {
    assert_eq!(Time::MIDNIGHT.format(&BorrowedFormatItem::First(&[]))?, "");
    assert_eq!(
        Time::MIDNIGHT.format(&BorrowedFormatItem::First(&[BorrowedFormatItem::Compound(
            fd!("[hour]")
        )]))?,
        "00"
    );
    assert_eq!(
        Time::MIDNIGHT.format(&OwnedFormatItem::First(Box::new([])))?,
        ""
    );
    assert_eq!(
        Time::MIDNIGHT.format(&OwnedFormatItem::from(BorrowedFormatItem::First(&[
            BorrowedFormatItem::Compound(fd!("[hour]"))
        ])))?,
        "00"
    );

    Ok(())
}

#[test]
fn ignore() -> time::Result<()> {
    assert_eq!(Time::MIDNIGHT.format(fd!("[ignore count:2]"))?, "");

    Ok(())
}

#[test]
fn end() -> time::Result<()> {
    assert_eq!(Time::MIDNIGHT.format(fd!("[end]"))?, "");

    Ok(())
}

#[test]
fn unix_timestamp() -> time::Result<()> {
    let dt = datetime!(2009-02-13 23:31:30.123456789 UTC);

    assert_eq!(dt.format(&fd!("[unix_timestamp]"))?, "1234567890");
    assert_eq!(
        dt.format(&fd!("[unix_timestamp sign:mandatory]"))?,
        "+1234567890"
    );
    assert_eq!(
        dt.format(&fd!("[unix_timestamp precision:millisecond]"))?,
        "1234567890123"
    );
    assert_eq!(
        dt.format(&fd!("[unix_timestamp precision:microsecond]"))?,
        "1234567890123456"
    );
    assert_eq!(
        dt.format(&fd!("[unix_timestamp precision:nanosecond]"))?,
        "1234567890123456789"
    );
    assert_eq!(
        datetime!(1969-12-31 23:59:59 UTC).format(&fd!("[unix_timestamp]"))?,
        "-1"
    );

    Ok(())
}
