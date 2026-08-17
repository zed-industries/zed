use std::io;

use criterion::Bencher;
use time::format_description;
use time::format_description::well_known::{Rfc2822, Rfc3339};
use time::macros::{date, datetime, format_description as fd, offset, time};

setup_benchmark! {
    "Formatting",

    fn format_rfc3339(ben: &mut Bencher<'_>) {
        macro_rules! item {
            ($value:expr) => {
                $value.format_into(&mut io::sink(), &Rfc3339)
            }
        }

        ben.iter(|| item!(datetime!(2021-01-02 03:04:05 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.1 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.12 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.123 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.123_4 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.123_45 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.123_456 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.123_456_7 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.123_456_78 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.123_456_789 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.123_456_789 -01:02)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05.123_456_789 +01:02)));
    }

    fn format_rfc2822(ben: &mut Bencher<'_>) {
        macro_rules! item {
            ($value:expr) => {
                $value.format_into(&mut io::sink(), &Rfc2822)
            }
        }

        ben.iter(|| item!(datetime!(2021-01-02 03:04:05 UTC)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05 +06:07)));
        ben.iter(|| item!(datetime!(2021-01-02 03:04:05 -06:07)));
    }


    fn format_time(ben: &mut Bencher<'_>) {
        macro_rules! item {
            ($format:expr) => {
                time!(13:02:03.456_789_012).format_into(
                    &mut io::sink(),
                    &$format,
                )
            }
        }

        ben.iter(|| item!(fd!("[hour]")));
        ben.iter(|| item!(fd!("[hour repr:12]")));
        ben.iter(|| item!(fd!("[hour repr:12 padding:none]")));
        ben.iter(|| item!(fd!("[hour repr:12 padding:space]")));
        ben.iter(|| item!(fd!("[hour repr:24]")));
        ben.iter(|| item!(fd!("[hour repr:24]")));
        ben.iter(|| item!(fd!("[hour repr:24 padding:none]")));
        ben.iter(|| item!(fd!("[hour repr:24 padding:space]")));
        ben.iter(|| item!(fd!("[minute]")));
        ben.iter(|| item!(fd!("[minute padding:none]")));
        ben.iter(|| item!(fd!("[minute padding:space]")));
        ben.iter(|| item!(fd!("[minute padding:zero]")));
        ben.iter(|| item!(fd!("[period]")));
        ben.iter(|| item!(fd!("[period case:upper]")));
        ben.iter(|| item!(fd!("[period case:lower]")));
        ben.iter(|| item!(fd!("[second]")));
        ben.iter(|| item!(fd!("[second padding:none]")));
        ben.iter(|| item!(fd!("[second padding:space]")));
        ben.iter(|| item!(fd!("[second padding:zero]")));
        ben.iter(|| item!(fd!("[subsecond]")));
        ben.iter(|| item!(fd!("[subsecond digits:1]")));
        ben.iter(|| item!(fd!("[subsecond digits:2]")));
        ben.iter(|| item!(fd!("[subsecond digits:3]")));
        ben.iter(|| item!(fd!("[subsecond digits:4]")));
        ben.iter(|| item!(fd!("[subsecond digits:5]")));
        ben.iter(|| item!(fd!("[subsecond digits:6]")));
        ben.iter(|| item!(fd!("[subsecond digits:7]")));
        ben.iter(|| item!(fd!("[subsecond digits:8]")));
        ben.iter(|| item!(fd!("[subsecond digits:9]")));
        ben.iter(|| item!(fd!("[subsecond digits:1+]")));
    }

    fn display_time(ben: &mut Bencher<'_>) {
        ben.iter(|| time!(0:00).to_string());
        ben.iter(|| time!(23:59).to_string());
        ben.iter(|| time!(23:59:59).to_string());
        ben.iter(|| time!(0:00:01).to_string());
        ben.iter(|| time!(0:00:00.001).to_string());
        ben.iter(|| time!(0:00:00.000_001).to_string());
        ben.iter(|| time!(0:00:00.000_000_001).to_string());
    }

    fn format_date(ben: &mut Bencher<'_>) {
        macro_rules! item {
            ($format:expr) => {
                date!(2019-12-31).format_into(&mut io::sink(), &$format)
            }
        }

        ben.iter(|| item!(fd!("[day]")));
        ben.iter(|| item!(fd!("[month]")));
        ben.iter(|| item!(fd!("[month repr:short]")));
        ben.iter(|| item!(fd!("[month repr:long]")));
        ben.iter(|| item!(fd!("[ordinal]")));
        ben.iter(|| item!(fd!("[weekday]")));
        ben.iter(|| item!(fd!("[weekday repr:short]")));
        ben.iter(|| item!(fd!("[weekday repr:sunday]")));
        ben.iter(|| item!(fd!("[weekday repr:sunday one_indexed:false]")));
        ben.iter(|| item!(fd!("[weekday repr:monday]")));
        ben.iter(|| item!(fd!("[weekday repr:monday one_indexed:false]")));
        ben.iter(|| item!(fd!("[week_number]")));
        ben.iter(|| item!(fd!("[week_number padding:none]")));
        ben.iter(|| item!(fd!("[week_number padding:space]")));
        ben.iter(|| item!(fd!("[week_number repr:sunday]")));
        ben.iter(|| item!(fd!("[week_number repr:monday]")));
        ben.iter(|| item!(fd!("[year]")));
        ben.iter(|| item!(fd!("[year base:iso_week]")));
        ben.iter(|| item!(fd!("[year sign:mandatory]")));
        ben.iter(|| item!(fd!("[year base:iso_week sign:mandatory]")));
        ben.iter(|| item!(fd!("[year repr:last_two]")));
        ben.iter(|| item!(fd!("[year base:iso_week repr:last_two]")));
    }

    fn display_date(ben: &mut Bencher<'_>) {
        ben.iter(|| date!(2019-01-01).to_string());
        ben.iter(|| date!(2019-12-31).to_string());
        ben.iter(|| date!(-4713-11-24).to_string());
        ben.iter(|| date!(-0001-01-01).to_string());
    }

    fn format_offset(ben: &mut Bencher<'_>) {
        macro_rules! item {
            ($value:expr, $format:expr) => {
                $value.format_into(&mut io::sink(), &$format)
            }
        }

        ben.iter(|| item!(offset!(+01:02:03), fd!("[offset_hour sign:automatic]")));
        ben.iter(|| item!(offset!(+01:02:03), fd!("[offset_hour sign:mandatory]")));
        ben.iter(|| item!(offset!(-01:02:03), fd!("[offset_hour sign:automatic]")));
        ben.iter(|| item!(offset!(-01:02:03), fd!("[offset_hour sign:mandatory]")));
        ben.iter(|| item!(offset!(+01:02:03), fd!("[offset_minute]")));
        ben.iter(|| item!(offset!(+01:02:03), fd!("[offset_second]")));
    }

    fn display_offset(ben: &mut Bencher<'_>) {
        ben.iter(|| offset!(UTC).to_string());
        ben.iter(|| offset!(+0:00:01).to_string());
        ben.iter(|| offset!(-0:00:01).to_string());
        ben.iter(|| offset!(+1).to_string());
        ben.iter(|| offset!(-1).to_string());
        ben.iter(|| offset!(+23:59).to_string());
        ben.iter(|| offset!(-23:59).to_string());
        ben.iter(|| offset!(+23:59:59).to_string());
        ben.iter(|| offset!(-23:59:59).to_string());
    }

    fn format_pdt(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            datetime!(1970-01-01 0:00).format_into(
                &mut io::sink(),
                fd!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
            )
        });
    }

    fn display_pdt(ben: &mut Bencher<'_>) {
        ben.iter(|| datetime!(1970-01-01 0:00).to_string());
        ben.iter(|| datetime!(1970-01-01 0:00:01).to_string());
    }

    fn format_odt(ben: &mut Bencher<'_>) {
        let format_description = format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond] [offset_hour \
            sign:mandatory]:[offset_minute]:[offset_second]",
        ).expect("invalid format description");

        ben.iter(|| {
            datetime!(1970-01-01 0:00 UTC).format_into(&mut io::sink(), &format_description)
        });
    }

    fn display_odt(ben: &mut Bencher<'_>) {
        ben.iter(|| datetime!(1970-01-01 0:00 UTC).to_string());
    }
}
