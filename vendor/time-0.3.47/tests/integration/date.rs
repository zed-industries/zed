use std::cmp::Ordering;
use std::collections::HashSet;

use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::{date, datetime, time};
use time::{Date, Duration, Month, Weekday, util};

#[test]
fn debug() {
    assert_eq!(format!("{:?}", date!(2020-02-03)), "2020-02-03");
}

#[test]
fn weeks_in_year_exhaustive() {
    let years_with_53 = [
        4, 9, 15, 20, 26, 32, 37, 43, 48, 54, 60, 65, 71, 76, 82, 88, 93, 99, 105, 111, 116, 122,
        128, 133, 139, 144, 150, 156, 161, 167, 172, 178, 184, 189, 195, 201, 207, 212, 218, 224,
        229, 235, 240, 246, 252, 257, 263, 268, 274, 280, 285, 291, 296, 303, 308, 314, 320, 325,
        331, 336, 342, 348, 353, 359, 364, 370, 376, 381, 387, 392, 398,
    ]
    .iter()
    .copied()
    .collect::<HashSet<_>>();

    for year in 0..400 {
        assert_eq!(
            util::weeks_in_year(year),
            if years_with_53.contains(&year) {
                53
            } else {
                52
            }
        );
    }
}

// Test all dominical letters. For leap years, check the dates immediately preceding and after the
// leap day.

#[test]
fn test_monday_based_week_dominical_a() {
    assert_eq!(date!(2023-01-01).monday_based_week(), 0);
    assert_eq!(date!(2023-01-02).monday_based_week(), 1);
    assert_eq!(date!(2023-01-03).monday_based_week(), 1);
    assert_eq!(date!(2023-01-04).monday_based_week(), 1);
    assert_eq!(date!(2023-01-05).monday_based_week(), 1);
    assert_eq!(date!(2023-01-06).monday_based_week(), 1);
    assert_eq!(date!(2023-01-07).monday_based_week(), 1);
}

#[test]
fn test_monday_based_week_dominical_b() {
    assert_eq!(date!(2022-01-01).monday_based_week(), 0);
    assert_eq!(date!(2022-01-02).monday_based_week(), 0);
    assert_eq!(date!(2022-01-03).monday_based_week(), 1);
    assert_eq!(date!(2022-01-04).monday_based_week(), 1);
    assert_eq!(date!(2022-01-05).monday_based_week(), 1);
    assert_eq!(date!(2022-01-06).monday_based_week(), 1);
    assert_eq!(date!(2022-01-07).monday_based_week(), 1);
}

#[test]
fn test_monday_based_week_dominical_c() {
    assert_eq!(date!(2021-01-01).monday_based_week(), 0);
    assert_eq!(date!(2021-01-02).monday_based_week(), 0);
    assert_eq!(date!(2021-01-03).monday_based_week(), 0);
    assert_eq!(date!(2021-01-04).monday_based_week(), 1);
    assert_eq!(date!(2021-01-05).monday_based_week(), 1);
    assert_eq!(date!(2021-01-06).monday_based_week(), 1);
    assert_eq!(date!(2021-01-07).monday_based_week(), 1);
}

#[test]
fn test_monday_based_week_dominical_d() {
    assert_eq!(date!(2026-01-01).monday_based_week(), 0);
    assert_eq!(date!(2026-01-02).monday_based_week(), 0);
    assert_eq!(date!(2026-01-03).monday_based_week(), 0);
    assert_eq!(date!(2026-01-04).monday_based_week(), 0);
    assert_eq!(date!(2026-01-05).monday_based_week(), 1);
    assert_eq!(date!(2026-01-06).monday_based_week(), 1);
    assert_eq!(date!(2026-01-07).monday_based_week(), 1);
}

#[test]
fn test_monday_based_week_dominical_e() {
    assert_eq!(date!(2025-01-01).monday_based_week(), 0);
    assert_eq!(date!(2025-01-02).monday_based_week(), 0);
    assert_eq!(date!(2025-01-03).monday_based_week(), 0);
    assert_eq!(date!(2025-01-04).monday_based_week(), 0);
    assert_eq!(date!(2025-01-05).monday_based_week(), 0);
    assert_eq!(date!(2025-01-06).monday_based_week(), 1);
    assert_eq!(date!(2025-01-07).monday_based_week(), 1);
}

#[test]
fn test_monday_based_week_dominical_f() {
    assert_eq!(date!(2019-01-01).monday_based_week(), 0);
    assert_eq!(date!(2019-01-02).monday_based_week(), 0);
    assert_eq!(date!(2019-01-03).monday_based_week(), 0);
    assert_eq!(date!(2019-01-04).monday_based_week(), 0);
    assert_eq!(date!(2019-01-05).monday_based_week(), 0);
    assert_eq!(date!(2019-01-06).monday_based_week(), 0);
    assert_eq!(date!(2019-01-07).monday_based_week(), 1);
}

#[test]
fn test_monday_based_week_dominical_g() {
    assert_eq!(date!(2018-01-01).monday_based_week(), 1);
    assert_eq!(date!(2018-01-02).monday_based_week(), 1);
    assert_eq!(date!(2018-01-03).monday_based_week(), 1);
    assert_eq!(date!(2018-01-04).monday_based_week(), 1);
    assert_eq!(date!(2018-01-05).monday_based_week(), 1);
    assert_eq!(date!(2018-01-06).monday_based_week(), 1);
    assert_eq!(date!(2018-01-07).monday_based_week(), 1);
}

#[test]
fn test_monday_based_week_dominical_ag() {
    assert_eq!(date!(2012-01-01).monday_based_week(), 0);
    assert_eq!(date!(2012-01-02).monday_based_week(), 1);
    assert_eq!(date!(2012-01-03).monday_based_week(), 1);
    assert_eq!(date!(2012-01-04).monday_based_week(), 1);
    assert_eq!(date!(2012-01-05).monday_based_week(), 1);
    assert_eq!(date!(2012-01-06).monday_based_week(), 1);
    assert_eq!(date!(2012-01-07).monday_based_week(), 1);
    assert_eq!(date!(2012-02-28).monday_based_week(), 9);
    assert_eq!(date!(2012-02-29).monday_based_week(), 9);
    assert_eq!(date!(2012-03-01).monday_based_week(), 9);
    assert_eq!(date!(2012-03-02).monday_based_week(), 9);
    assert_eq!(date!(2012-03-03).monday_based_week(), 9);
    assert_eq!(date!(2012-03-04).monday_based_week(), 9);
    assert_eq!(date!(2012-03-05).monday_based_week(), 10);
    assert_eq!(date!(2012-03-06).monday_based_week(), 10);
    assert_eq!(date!(2012-03-07).monday_based_week(), 10);
}

#[test]
fn test_monday_based_week_dominical_ba() {
    assert_eq!(date!(2028-01-01).monday_based_week(), 0);
    assert_eq!(date!(2028-01-02).monday_based_week(), 0);
    assert_eq!(date!(2028-01-03).monday_based_week(), 1);
    assert_eq!(date!(2028-01-04).monday_based_week(), 1);
    assert_eq!(date!(2028-01-05).monday_based_week(), 1);
    assert_eq!(date!(2028-01-06).monday_based_week(), 1);
    assert_eq!(date!(2028-01-07).monday_based_week(), 1);
    assert_eq!(date!(2028-02-28).monday_based_week(), 9);
    assert_eq!(date!(2028-02-29).monday_based_week(), 9);
    assert_eq!(date!(2028-03-01).monday_based_week(), 9);
    assert_eq!(date!(2028-03-02).monday_based_week(), 9);
    assert_eq!(date!(2028-03-03).monday_based_week(), 9);
    assert_eq!(date!(2028-03-04).monday_based_week(), 9);
    assert_eq!(date!(2028-03-05).monday_based_week(), 9);
    assert_eq!(date!(2028-03-06).monday_based_week(), 10);
    assert_eq!(date!(2028-03-07).monday_based_week(), 10);
}

#[test]
fn test_monday_based_week_dominical_cb() {
    assert_eq!(date!(2016-01-01).monday_based_week(), 0);
    assert_eq!(date!(2016-01-02).monday_based_week(), 0);
    assert_eq!(date!(2016-01-03).monday_based_week(), 0);
    assert_eq!(date!(2016-01-04).monday_based_week(), 1);
    assert_eq!(date!(2016-01-05).monday_based_week(), 1);
    assert_eq!(date!(2016-01-06).monday_based_week(), 1);
    assert_eq!(date!(2016-01-07).monday_based_week(), 1);
    assert_eq!(date!(2016-02-28).monday_based_week(), 8);
    assert_eq!(date!(2016-02-29).monday_based_week(), 9);
    assert_eq!(date!(2016-03-01).monday_based_week(), 9);
    assert_eq!(date!(2016-03-02).monday_based_week(), 9);
    assert_eq!(date!(2016-03-03).monday_based_week(), 9);
    assert_eq!(date!(2016-03-04).monday_based_week(), 9);
    assert_eq!(date!(2016-03-05).monday_based_week(), 9);
    assert_eq!(date!(2016-03-06).monday_based_week(), 9);
    assert_eq!(date!(2016-03-07).monday_based_week(), 10);
}

#[test]
fn test_monday_based_week_dominical_dc() {
    assert_eq!(date!(2032-01-01).monday_based_week(), 0);
    assert_eq!(date!(2032-01-02).monday_based_week(), 0);
    assert_eq!(date!(2032-01-03).monday_based_week(), 0);
    assert_eq!(date!(2032-01-04).monday_based_week(), 0);
    assert_eq!(date!(2032-01-05).monday_based_week(), 1);
    assert_eq!(date!(2032-01-06).monday_based_week(), 1);
    assert_eq!(date!(2032-01-07).monday_based_week(), 1);
    assert_eq!(date!(2032-02-28).monday_based_week(), 8);
    assert_eq!(date!(2032-02-29).monday_based_week(), 8);
    assert_eq!(date!(2032-03-01).monday_based_week(), 9);
    assert_eq!(date!(2032-03-02).monday_based_week(), 9);
    assert_eq!(date!(2032-03-03).monday_based_week(), 9);
    assert_eq!(date!(2032-03-04).monday_based_week(), 9);
    assert_eq!(date!(2032-03-05).monday_based_week(), 9);
    assert_eq!(date!(2032-03-06).monday_based_week(), 9);
    assert_eq!(date!(2032-03-07).monday_based_week(), 9);
}

#[test]
fn test_monday_based_week_dominical_ed() {
    assert_eq!(date!(2020-01-01).monday_based_week(), 0);
    assert_eq!(date!(2020-01-02).monday_based_week(), 0);
    assert_eq!(date!(2020-01-03).monday_based_week(), 0);
    assert_eq!(date!(2020-01-04).monday_based_week(), 0);
    assert_eq!(date!(2020-01-05).monday_based_week(), 0);
    assert_eq!(date!(2020-01-06).monday_based_week(), 1);
    assert_eq!(date!(2020-01-07).monday_based_week(), 1);
    assert_eq!(date!(2020-02-28).monday_based_week(), 8);
    assert_eq!(date!(2020-02-29).monday_based_week(), 8);
    assert_eq!(date!(2020-03-01).monday_based_week(), 8);
    assert_eq!(date!(2020-03-02).monday_based_week(), 9);
    assert_eq!(date!(2020-03-03).monday_based_week(), 9);
    assert_eq!(date!(2020-03-04).monday_based_week(), 9);
    assert_eq!(date!(2020-03-05).monday_based_week(), 9);
    assert_eq!(date!(2020-03-06).monday_based_week(), 9);
    assert_eq!(date!(2020-03-07).monday_based_week(), 9);
}

#[test]
fn test_monday_based_week_dominical_fe() {
    assert_eq!(date!(2036-01-01).monday_based_week(), 0);
    assert_eq!(date!(2036-01-02).monday_based_week(), 0);
    assert_eq!(date!(2036-01-03).monday_based_week(), 0);
    assert_eq!(date!(2036-01-04).monday_based_week(), 0);
    assert_eq!(date!(2036-01-05).monday_based_week(), 0);
    assert_eq!(date!(2036-01-06).monday_based_week(), 0);
    assert_eq!(date!(2036-01-07).monday_based_week(), 1);
    assert_eq!(date!(2036-02-28).monday_based_week(), 8);
    assert_eq!(date!(2036-02-29).monday_based_week(), 8);
    assert_eq!(date!(2036-03-01).monday_based_week(), 8);
    assert_eq!(date!(2036-03-02).monday_based_week(), 8);
    assert_eq!(date!(2036-03-03).monday_based_week(), 9);
    assert_eq!(date!(2036-03-04).monday_based_week(), 9);
    assert_eq!(date!(2036-03-05).monday_based_week(), 9);
    assert_eq!(date!(2036-03-06).monday_based_week(), 9);
    assert_eq!(date!(2036-03-07).monday_based_week(), 9);
}

#[test]
fn test_monday_based_week_dominical_gf() {
    assert_eq!(date!(2024-01-01).monday_based_week(), 1);
    assert_eq!(date!(2024-01-02).monday_based_week(), 1);
    assert_eq!(date!(2024-01-03).monday_based_week(), 1);
    assert_eq!(date!(2024-01-04).monday_based_week(), 1);
    assert_eq!(date!(2024-01-05).monday_based_week(), 1);
    assert_eq!(date!(2024-01-06).monday_based_week(), 1);
    assert_eq!(date!(2024-01-07).monday_based_week(), 1);
    assert_eq!(date!(2024-02-28).monday_based_week(), 9);
    assert_eq!(date!(2024-02-29).monday_based_week(), 9);
    assert_eq!(date!(2024-03-01).monday_based_week(), 9);
    assert_eq!(date!(2024-03-02).monday_based_week(), 9);
    assert_eq!(date!(2024-03-03).monday_based_week(), 9);
    assert_eq!(date!(2024-03-04).monday_based_week(), 10);
    assert_eq!(date!(2024-03-05).monday_based_week(), 10);
    assert_eq!(date!(2024-03-06).monday_based_week(), 10);
    assert_eq!(date!(2024-03-07).monday_based_week(), 10);
}

#[test]
fn test_sunday_based_week_dominical_a() {
    assert_eq!(date!(2023-01-01).sunday_based_week(), 1);
    assert_eq!(date!(2023-01-02).sunday_based_week(), 1);
    assert_eq!(date!(2023-01-03).sunday_based_week(), 1);
    assert_eq!(date!(2023-01-04).sunday_based_week(), 1);
    assert_eq!(date!(2023-01-05).sunday_based_week(), 1);
    assert_eq!(date!(2023-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2023-01-07).sunday_based_week(), 1);
}

#[test]
fn test_sunday_based_week_dominical_b() {
    assert_eq!(date!(2022-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2022-01-02).sunday_based_week(), 1);
    assert_eq!(date!(2022-01-03).sunday_based_week(), 1);
    assert_eq!(date!(2022-01-04).sunday_based_week(), 1);
    assert_eq!(date!(2022-01-05).sunday_based_week(), 1);
    assert_eq!(date!(2022-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2022-01-07).sunday_based_week(), 1);
}

#[test]
fn test_sunday_based_week_dominical_c() {
    assert_eq!(date!(2021-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2021-01-02).sunday_based_week(), 0);
    assert_eq!(date!(2021-01-03).sunday_based_week(), 1);
    assert_eq!(date!(2021-01-04).sunday_based_week(), 1);
    assert_eq!(date!(2021-01-05).sunday_based_week(), 1);
    assert_eq!(date!(2021-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2021-01-07).sunday_based_week(), 1);
}

#[test]
fn test_sunday_based_week_dominical_d() {
    assert_eq!(date!(2026-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2026-01-02).sunday_based_week(), 0);
    assert_eq!(date!(2026-01-03).sunday_based_week(), 0);
    assert_eq!(date!(2026-01-04).sunday_based_week(), 1);
    assert_eq!(date!(2026-01-05).sunday_based_week(), 1);
    assert_eq!(date!(2026-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2026-01-07).sunday_based_week(), 1);
}

#[test]
fn test_sunday_based_week_dominical_e() {
    assert_eq!(date!(2025-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2025-01-02).sunday_based_week(), 0);
    assert_eq!(date!(2025-01-03).sunday_based_week(), 0);
    assert_eq!(date!(2025-01-04).sunday_based_week(), 0);
    assert_eq!(date!(2025-01-05).sunday_based_week(), 1);
    assert_eq!(date!(2025-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2025-01-07).sunday_based_week(), 1);
}

#[test]
fn test_sunday_based_week_dominical_f() {
    assert_eq!(date!(2019-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2019-01-02).sunday_based_week(), 0);
    assert_eq!(date!(2019-01-03).sunday_based_week(), 0);
    assert_eq!(date!(2019-01-04).sunday_based_week(), 0);
    assert_eq!(date!(2019-01-05).sunday_based_week(), 0);
    assert_eq!(date!(2019-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2019-01-07).sunday_based_week(), 1);
}

#[test]
fn test_sunday_based_week_dominical_g() {
    assert_eq!(date!(2018-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2018-01-02).sunday_based_week(), 0);
    assert_eq!(date!(2018-01-03).sunday_based_week(), 0);
    assert_eq!(date!(2018-01-04).sunday_based_week(), 0);
    assert_eq!(date!(2018-01-05).sunday_based_week(), 0);
    assert_eq!(date!(2018-01-06).sunday_based_week(), 0);
    assert_eq!(date!(2018-01-07).sunday_based_week(), 1);
}

#[test]
fn test_sunday_based_week_dominical_ag() {
    assert_eq!(date!(2012-01-01).sunday_based_week(), 1);
    assert_eq!(date!(2012-01-02).sunday_based_week(), 1);
    assert_eq!(date!(2012-01-03).sunday_based_week(), 1);
    assert_eq!(date!(2012-01-04).sunday_based_week(), 1);
    assert_eq!(date!(2012-01-05).sunday_based_week(), 1);
    assert_eq!(date!(2012-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2012-01-07).sunday_based_week(), 1);
    assert_eq!(date!(2012-02-28).sunday_based_week(), 9);
    assert_eq!(date!(2012-02-29).sunday_based_week(), 9);
    assert_eq!(date!(2012-03-01).sunday_based_week(), 9);
    assert_eq!(date!(2012-03-02).sunday_based_week(), 9);
    assert_eq!(date!(2012-03-03).sunday_based_week(), 9);
    assert_eq!(date!(2012-03-04).sunday_based_week(), 10);
    assert_eq!(date!(2012-03-05).sunday_based_week(), 10);
    assert_eq!(date!(2012-03-06).sunday_based_week(), 10);
    assert_eq!(date!(2012-03-07).sunday_based_week(), 10);
}

#[test]
fn test_sunday_based_week_dominical_ba() {
    assert_eq!(date!(2028-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2028-01-02).sunday_based_week(), 1);
    assert_eq!(date!(2028-01-03).sunday_based_week(), 1);
    assert_eq!(date!(2028-01-04).sunday_based_week(), 1);
    assert_eq!(date!(2028-01-05).sunday_based_week(), 1);
    assert_eq!(date!(2028-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2028-01-07).sunday_based_week(), 1);
    assert_eq!(date!(2028-02-28).sunday_based_week(), 9);
    assert_eq!(date!(2028-02-29).sunday_based_week(), 9);
    assert_eq!(date!(2028-03-01).sunday_based_week(), 9);
    assert_eq!(date!(2028-03-02).sunday_based_week(), 9);
    assert_eq!(date!(2028-03-03).sunday_based_week(), 9);
    assert_eq!(date!(2028-03-04).sunday_based_week(), 9);
    assert_eq!(date!(2028-03-05).sunday_based_week(), 10);
    assert_eq!(date!(2028-03-06).sunday_based_week(), 10);
    assert_eq!(date!(2028-03-07).sunday_based_week(), 10);
}

#[test]
fn test_sunday_based_week_dominical_cb() {
    assert_eq!(date!(2016-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2016-01-02).sunday_based_week(), 0);
    assert_eq!(date!(2016-01-03).sunday_based_week(), 1);
    assert_eq!(date!(2016-01-04).sunday_based_week(), 1);
    assert_eq!(date!(2016-01-05).sunday_based_week(), 1);
    assert_eq!(date!(2016-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2016-01-07).sunday_based_week(), 1);
    assert_eq!(date!(2016-02-28).sunday_based_week(), 9);
    assert_eq!(date!(2016-02-29).sunday_based_week(), 9);
    assert_eq!(date!(2016-03-01).sunday_based_week(), 9);
    assert_eq!(date!(2016-03-02).sunday_based_week(), 9);
    assert_eq!(date!(2016-03-03).sunday_based_week(), 9);
    assert_eq!(date!(2016-03-04).sunday_based_week(), 9);
    assert_eq!(date!(2016-03-05).sunday_based_week(), 9);
    assert_eq!(date!(2016-03-06).sunday_based_week(), 10);
    assert_eq!(date!(2016-03-07).sunday_based_week(), 10);
}

#[test]
fn test_sunday_based_week_dominical_dc() {
    assert_eq!(date!(2032-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2032-01-02).sunday_based_week(), 0);
    assert_eq!(date!(2032-01-03).sunday_based_week(), 0);
    assert_eq!(date!(2032-01-04).sunday_based_week(), 1);
    assert_eq!(date!(2032-01-05).sunday_based_week(), 1);
    assert_eq!(date!(2032-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2032-01-07).sunday_based_week(), 1);
    assert_eq!(date!(2032-02-28).sunday_based_week(), 8);
    assert_eq!(date!(2032-02-29).sunday_based_week(), 9);
    assert_eq!(date!(2032-03-01).sunday_based_week(), 9);
    assert_eq!(date!(2032-03-02).sunday_based_week(), 9);
    assert_eq!(date!(2032-03-03).sunday_based_week(), 9);
    assert_eq!(date!(2032-03-04).sunday_based_week(), 9);
    assert_eq!(date!(2032-03-05).sunday_based_week(), 9);
    assert_eq!(date!(2032-03-06).sunday_based_week(), 9);
    assert_eq!(date!(2032-03-07).sunday_based_week(), 10);
}

#[test]
fn test_sunday_based_week_dominical_ed() {
    assert_eq!(date!(2020-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2020-01-02).sunday_based_week(), 0);
    assert_eq!(date!(2020-01-03).sunday_based_week(), 0);
    assert_eq!(date!(2020-01-04).sunday_based_week(), 0);
    assert_eq!(date!(2020-01-05).sunday_based_week(), 1);
    assert_eq!(date!(2020-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2020-01-07).sunday_based_week(), 1);
    assert_eq!(date!(2020-02-28).sunday_based_week(), 8);
    assert_eq!(date!(2020-02-29).sunday_based_week(), 8);
    assert_eq!(date!(2020-03-01).sunday_based_week(), 9);
    assert_eq!(date!(2020-03-02).sunday_based_week(), 9);
    assert_eq!(date!(2020-03-03).sunday_based_week(), 9);
    assert_eq!(date!(2020-03-04).sunday_based_week(), 9);
    assert_eq!(date!(2020-03-05).sunday_based_week(), 9);
    assert_eq!(date!(2020-03-06).sunday_based_week(), 9);
    assert_eq!(date!(2020-03-07).sunday_based_week(), 9);
}

#[test]
fn test_sunday_based_week_dominical_fe() {
    assert_eq!(date!(2036-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2036-01-02).sunday_based_week(), 0);
    assert_eq!(date!(2036-01-03).sunday_based_week(), 0);
    assert_eq!(date!(2036-01-04).sunday_based_week(), 0);
    assert_eq!(date!(2036-01-05).sunday_based_week(), 0);
    assert_eq!(date!(2036-01-06).sunday_based_week(), 1);
    assert_eq!(date!(2036-01-07).sunday_based_week(), 1);
    assert_eq!(date!(2036-02-28).sunday_based_week(), 8);
    assert_eq!(date!(2036-02-29).sunday_based_week(), 8);
    assert_eq!(date!(2036-03-01).sunday_based_week(), 8);
    assert_eq!(date!(2036-03-02).sunday_based_week(), 9);
    assert_eq!(date!(2036-03-03).sunday_based_week(), 9);
    assert_eq!(date!(2036-03-04).sunday_based_week(), 9);
    assert_eq!(date!(2036-03-05).sunday_based_week(), 9);
    assert_eq!(date!(2036-03-06).sunday_based_week(), 9);
    assert_eq!(date!(2036-03-07).sunday_based_week(), 9);
}

#[test]
fn test_sunday_based_week_dominical_gf() {
    assert_eq!(date!(2024-01-01).sunday_based_week(), 0);
    assert_eq!(date!(2024-01-02).sunday_based_week(), 0);
    assert_eq!(date!(2024-01-03).sunday_based_week(), 0);
    assert_eq!(date!(2024-01-04).sunday_based_week(), 0);
    assert_eq!(date!(2024-01-05).sunday_based_week(), 0);
    assert_eq!(date!(2024-01-06).sunday_based_week(), 0);
    assert_eq!(date!(2024-01-07).sunday_based_week(), 1);
    assert_eq!(date!(2024-02-28).sunday_based_week(), 8);
    assert_eq!(date!(2024-02-29).sunday_based_week(), 8);
    assert_eq!(date!(2024-03-01).sunday_based_week(), 8);
    assert_eq!(date!(2024-03-02).sunday_based_week(), 8);
    assert_eq!(date!(2024-03-03).sunday_based_week(), 9);
    assert_eq!(date!(2024-03-04).sunday_based_week(), 9);
    assert_eq!(date!(2024-03-05).sunday_based_week(), 9);
    assert_eq!(date!(2024-03-06).sunday_based_week(), 9);
    assert_eq!(date!(2024-03-07).sunday_based_week(), 9);
}

#[test]
fn from_iso_week_date() {
    use Weekday::*;
    assert!(Date::from_iso_week_date(2019, 1, Monday).is_ok());
    assert!(Date::from_iso_week_date(2019, 1, Tuesday).is_ok());
    assert!(Date::from_iso_week_date(2020, 53, Friday).is_ok());
    assert!(Date::from_iso_week_date(-9999, 1, Monday).is_ok());
    // 2019 doesn't have 53 weeks.
    assert!(Date::from_iso_week_date(2019, 53, Monday).is_err());
    // Regression test. Year zero (1 BCE) has dominical letter BA.
    assert_eq!(
        Date::from_iso_week_date(-1, 52, Saturday),
        Ok(date!(0000-01-01))
    );
    assert_eq!(date!(-0001-W52-6), date!(0000-01-01));
}

#[test]
fn year() {
    assert_eq!(date!(2019-002).year(), 2019);
    assert_eq!(date!(2020-002).year(), 2020);
}

#[test]
fn month() {
    assert_eq!(date!(2019-002).month(), Month::January);
    assert_eq!(date!(2020-002).month(), Month::January);
    assert_eq!(date!(2019-060).month(), Month::March);
    assert_eq!(date!(2020-060).month(), Month::February);
}

#[test]
fn day() {
    assert_eq!(date!(2019-002).day(), 2);
    assert_eq!(date!(2020-002).day(), 2);
    assert_eq!(date!(2019-060).day(), 1);
    assert_eq!(date!(2020-060).day(), 29);
}

#[test]
fn iso_week() {
    assert_eq!(date!(2019-01-01).iso_week(), 1);
    assert_eq!(date!(2019-10-04).iso_week(), 40);
    assert_eq!(date!(2020-01-01).iso_week(), 1);
    assert_eq!(date!(2020-12-31).iso_week(), 53);
    assert_eq!(date!(2021-01-01).iso_week(), 53);
}

#[test]
fn to_calendar_date() {
    assert_eq!(
        date!(2019-01-02).to_calendar_date(),
        (2019, Month::January, 2)
    );
    assert_eq!(
        date!(2019-02-02).to_calendar_date(),
        (2019, Month::February, 2)
    );
    assert_eq!(
        date!(2019-03-02).to_calendar_date(),
        (2019, Month::March, 2)
    );
    assert_eq!(
        date!(2019-04-02).to_calendar_date(),
        (2019, Month::April, 2)
    );
    assert_eq!(date!(2019-05-02).to_calendar_date(), (2019, Month::May, 2));
    assert_eq!(date!(2019-06-02).to_calendar_date(), (2019, Month::June, 2));
    assert_eq!(date!(2019-07-02).to_calendar_date(), (2019, Month::July, 2));
    assert_eq!(
        date!(2019-08-02).to_calendar_date(),
        (2019, Month::August, 2)
    );
    assert_eq!(
        date!(2019-09-02).to_calendar_date(),
        (2019, Month::September, 2)
    );
    assert_eq!(
        date!(2019-10-02).to_calendar_date(),
        (2019, Month::October, 2)
    );
    assert_eq!(
        date!(2019-11-02).to_calendar_date(),
        (2019, Month::November, 2)
    );
    assert_eq!(
        date!(2019-12-02).to_calendar_date(),
        (2019, Month::December, 2)
    );
}

#[test]
fn to_ordinal_date() {
    assert_eq!(date!(2019-01-01).to_ordinal_date(), (2019, 1));
}

#[test]
fn to_iso_week_date() {
    use Weekday::*;
    assert_eq!(date!(2019-01-01).to_iso_week_date(), (2019, 1, Tuesday));
    assert_eq!(date!(2019-10-04).to_iso_week_date(), (2019, 40, Friday));
    assert_eq!(date!(2020-01-01).to_iso_week_date(), (2020, 1, Wednesday));
    assert_eq!(date!(2020-12-31).to_iso_week_date(), (2020, 53, Thursday));
    assert_eq!(date!(2021-01-01).to_iso_week_date(), (2020, 53, Friday));
    assert_eq!(date!(0000-01-01).to_iso_week_date(), (-1, 52, Saturday));
}

#[test]
fn weekday() {
    use Weekday::*;
    assert_eq!(date!(2019-01-01).weekday(), Tuesday);
    assert_eq!(date!(2019-02-01).weekday(), Friday);
    assert_eq!(date!(2019-03-01).weekday(), Friday);
    assert_eq!(date!(2019-04-01).weekday(), Monday);
    assert_eq!(date!(2019-05-01).weekday(), Wednesday);
    assert_eq!(date!(2019-06-01).weekday(), Saturday);
    assert_eq!(date!(2019-07-01).weekday(), Monday);
    assert_eq!(date!(2019-08-01).weekday(), Thursday);
    assert_eq!(date!(2019-09-01).weekday(), Sunday);
    assert_eq!(date!(2019-10-01).weekday(), Tuesday);
    assert_eq!(date!(2019-11-01).weekday(), Friday);
    assert_eq!(date!(2019-12-01).weekday(), Sunday);
}

#[test]
fn next_day() {
    assert_eq!(date!(2019-01-01).next_day(), Some(date!(2019-01-02)));
    assert_eq!(date!(2019-01-31).next_day(), Some(date!(2019-02-01)));
    assert_eq!(date!(2019-12-31).next_day(), Some(date!(2020-01-01)));
    assert_eq!(date!(2020-12-31).next_day(), Some(date!(2021-01-01)));
    assert_eq!(Date::MAX.next_day(), None);
}

#[test]
fn previous_day() {
    assert_eq!(date!(2019-01-02).previous_day(), Some(date!(2019-01-01)));
    assert_eq!(date!(2019-02-01).previous_day(), Some(date!(2019-01-31)));
    assert_eq!(date!(2020-01-01).previous_day(), Some(date!(2019-12-31)));
    assert_eq!(date!(2021-01-01).previous_day(), Some(date!(2020-12-31)));
    assert_eq!(Date::MIN.previous_day(), None);
}

#[test]
fn to_julian_day() {
    assert_eq!(date!(-999_999-01-01).to_julian_day(), -363_521_074);
    assert_eq!(date!(-9999-01-01).to_julian_day(), -1_930_999);
    assert_eq!(date!(-4713-11-24).to_julian_day(), 0);
    assert_eq!(date!(2000-01-01).to_julian_day(), 2_451_545);
    assert_eq!(date!(2019-01-01).to_julian_day(), 2_458_485);
    assert_eq!(date!(2019-12-31).to_julian_day(), 2_458_849);
}

#[test]
fn from_julian_day() {
    assert_eq!(
        Date::from_julian_day(-363_521_074),
        Ok(date!(-999_999-01-01))
    );
    assert_eq!(Date::from_julian_day(-1_930_999), Ok(date!(-9999-01-01)));
    assert_eq!(Date::from_julian_day(0), Ok(date!(-4713-11-24)));
    assert_eq!(Date::from_julian_day(2_451_545), Ok(date!(2000-01-01)));
    assert_eq!(Date::from_julian_day(2_458_485), Ok(date!(2019-01-01)));
    assert_eq!(Date::from_julian_day(2_458_849), Ok(date!(2019-12-31)));
    assert!(Date::from_julian_day(i32::MAX).is_err());
}

#[test]
fn midnight() {
    assert_eq!(date!(1970-01-01).midnight(), datetime!(1970-01-01 0:00));
}

#[test]
fn with_time() {
    assert_eq!(
        date!(1970-01-01).with_time(time!(0:00)),
        datetime!(1970-01-01 0:00),
    );
}

#[test]
fn with_hms() {
    assert_eq!(
        date!(1970-01-01).with_hms(0, 0, 0),
        Ok(datetime!(1970-01-01 0:00)),
    );
    assert!(date!(1970-01-01).with_hms(24, 0, 0).is_err());
}

#[test]
fn with_hms_milli() {
    assert_eq!(
        date!(1970-01-01).with_hms_milli(0, 0, 0, 0),
        Ok(datetime!(1970-01-01 0:00)),
    );
    assert!(date!(1970-01-01).with_hms_milli(24, 0, 0, 0).is_err());
}

#[test]
fn with_hms_micro() {
    assert_eq!(
        date!(1970-01-01).with_hms_micro(0, 0, 0, 0),
        Ok(datetime!(1970-01-01 0:00)),
    );
    assert!(date!(1970-01-01).with_hms_micro(24, 0, 0, 0).is_err());
}

#[test]
fn with_hms_nano() {
    assert_eq!(
        date!(1970-01-01).with_hms_nano(0, 0, 0, 0),
        Ok(datetime!(1970-01-01 0:00)),
    );
    assert!(date!(1970-01-01).with_hms_nano(24, 0, 0, 0).is_err());
}

#[test]
fn add() {
    assert_eq!(date!(2019-01-01) + 5.days(), date!(2019-01-06));
    assert_eq!(date!(2019-12-31) + 1.days(), date!(2020-01-01));
}

#[test]
fn add_std() {
    assert_eq!(date!(2019-01-01) + 5.std_days(), date!(2019-01-06));
    assert_eq!(date!(2019-12-31) + 1.std_days(), date!(2020-01-01));
}

#[test]
fn add_assign() {
    let mut date = date!(2019-12-31);
    date += 1.days();
    assert_eq!(date, date!(2020-01-01));
}

#[test]
fn add_assign_std() {
    let mut date = date!(2019-12-31);
    date += 1.std_days();
    assert_eq!(date, date!(2020-01-01));
}

#[test]
fn sub() {
    assert_eq!(date!(2019-01-06) - 5.days(), date!(2019-01-01));
    assert_eq!(date!(2020-01-01) - 1.days(), date!(2019-12-31));
}

#[test]
fn sub_std() {
    assert_eq!(date!(2019-01-06) - 5.std_days(), date!(2019-01-01));
    assert_eq!(date!(2020-01-01) - 1.std_days(), date!(2019-12-31));
}

#[test]
fn sub_assign() {
    let mut date = date!(2020-01-01);
    date -= 1.days();
    assert_eq!(date, date!(2019-12-31));
}

#[test]
fn sub_assign_std() {
    let mut date = date!(2020-01-01);
    date -= 1.std_days();
    assert_eq!(date, date!(2019-12-31));
}

#[test]
fn sub_self() {
    assert_eq!(date!(2019-01-06) - date!(2019-01-01), 5.days());
    assert_eq!(date!(2020-01-01) - date!(2019-12-31), 1.days());
}

#[test]
fn partial_ord() {
    let first = date!(2019-01-01);
    let second = date!(2019-01-02);

    assert_eq!(first.partial_cmp(&first), Some(Ordering::Equal));
    assert_eq!(first.partial_cmp(&second), Some(Ordering::Less));
    assert_eq!(second.partial_cmp(&first), Some(Ordering::Greater));
}

#[test]
fn ord() {
    let first = date!(2019-01-01);
    let second = date!(2019-01-02);

    assert_eq!(first.cmp(&first), Ordering::Equal);
    assert_eq!(first.cmp(&second), Ordering::Less);
    assert_eq!(second.cmp(&first), Ordering::Greater);
}

#[test]
fn regression_check() {
    let (year, week, weekday) = (date!(0063-365)).to_iso_week_date();
    assert_eq!(year, 64);
    assert_eq!(week, 1);
    assert_eq!(weekday, Weekday::Monday);
}

#[test]
fn checked_add_duration() {
    // Adding subday duration
    assert_eq!(
        Date::MIN.checked_add(Duration::new(86_399, 999_999_999)),
        Some(Date::MIN)
    );
    assert_eq!(
        Date::MIN.checked_add(Duration::new(-86_399, -999_999_999)),
        Some(Date::MIN)
    );

    assert_eq!(
        date!(2021-10-25).checked_add(Duration::new(86_399, 999_999_999)),
        Some(date!(2021-10-25))
    );
    assert_eq!(
        date!(2021-10-25).checked_add(Duration::new(-86_399, -999_999_999)),
        Some(date!(2021-10-25))
    );

    assert_eq!(
        Date::MAX.checked_add(Duration::new(86_399, 999_999_999)),
        Some(Date::MAX)
    );
    assert_eq!(
        Date::MAX.checked_add(Duration::new(-86_399, -999_999_999)),
        Some(Date::MAX)
    );

    // Adding 1 day duration
    assert_eq!(Date::MIN.checked_add(Duration::DAY), Date::MIN.next_day());
    assert_eq!(Date::MIN.checked_add(-Duration::DAY), None);

    assert_eq!(
        date!(2021-10-25).checked_add(Duration::DAY),
        Some(date!(2021-10-26))
    );
    assert_eq!(
        date!(2021-10-25).checked_add(-Duration::DAY),
        Some(date!(2021-10-24))
    );

    assert_eq!(Date::MAX.checked_add(Duration::DAY), None);
    assert_eq!(
        Date::MAX.checked_add(-Duration::DAY),
        Date::MAX.previous_day()
    );

    // Adding MIN/MAX duration
    assert_eq!(Date::MIN.checked_add(Duration::MIN), None);
    assert_eq!(Date::MAX.checked_add(Duration::MAX), None);
}

#[test]
fn checked_sub_duration() {
    // Subtracting subday duration
    assert_eq!(
        Date::MIN.checked_sub(Duration::new(86_399, 999_999_999)),
        Some(Date::MIN)
    );
    assert_eq!(
        Date::MIN.checked_sub(Duration::new(-86_399, -999_999_999)),
        Some(Date::MIN)
    );

    assert_eq!(
        date!(2021-10-25).checked_sub(Duration::new(86_399, 999_999_999)),
        Some(date!(2021-10-25))
    );
    assert_eq!(
        date!(2021-10-25).checked_sub(Duration::new(-86_399, -999_999_999)),
        Some(date!(2021-10-25))
    );

    assert_eq!(
        Date::MAX.checked_sub(Duration::new(86_399, 999_999_999)),
        Some(Date::MAX)
    );
    assert_eq!(
        Date::MAX.checked_sub(Duration::new(-86_399, -999_999_999)),
        Some(Date::MAX)
    );

    // Subtracting 1 day duration
    assert_eq!(Date::MIN.checked_sub(Duration::DAY), None);
    assert_eq!(Date::MIN.checked_sub(-Duration::DAY), Date::MIN.next_day());

    assert_eq!(
        date!(2021-10-25).checked_sub(Duration::DAY),
        Some(date!(2021-10-24))
    );
    assert_eq!(
        date!(2021-10-25).checked_sub(-Duration::DAY),
        Some(date!(2021-10-26))
    );

    assert_eq!(
        Date::MAX.checked_sub(Duration::DAY),
        Date::MAX.previous_day()
    );
    assert_eq!(Date::MAX.checked_sub(-Duration::DAY), None);

    // Subtracting MIN/MAX duration
    assert_eq!(Date::MIN.checked_sub(Duration::MAX), None);
    assert_eq!(Date::MAX.checked_sub(Duration::MIN), None);
}

#[test]
fn saturating_add_duration() {
    assert_eq!(
        date!(2021-11-05).saturating_add(2.days()),
        date!(2021-11-07)
    );
    assert_eq!(
        date!(2021-11-05).saturating_add((-2).days()),
        date!(2021-11-03)
    );

    // Adding with underflow
    assert_eq!(Date::MIN.saturating_add((-10).days()), Date::MIN);

    // Adding with overflow
    assert_eq!(Date::MAX.saturating_add(10.days()), Date::MAX);

    // Adding zero duration at boundaries
    assert_eq!(Date::MIN.saturating_add(Duration::ZERO), Date::MIN);
    assert_eq!(Date::MAX.saturating_add(Duration::ZERO), Date::MAX);
}

#[test]
fn saturating_sub_duration() {
    assert_eq!(
        date!(2021-11-05).saturating_sub(2.days()),
        date!(2021-11-03)
    );
    assert_eq!(
        date!(2021-11-05).saturating_sub((-2).days()),
        date!(2021-11-07)
    );

    // Subtracting with underflow
    assert_eq!(Date::MIN.saturating_sub(10.days()), Date::MIN);

    // Subtracting with overflow
    assert_eq!(Date::MAX.saturating_sub((-10).days()), Date::MAX);

    // Subtracting zero duration at boundaries
    assert_eq!(Date::MIN.saturating_sub(Duration::ZERO), Date::MIN);
    assert_eq!(Date::MAX.saturating_sub(Duration::ZERO), Date::MAX);
}

#[test]
fn replace_year() {
    assert_eq!(date!(2022-02-18).replace_year(2019), Ok(date!(2019-02-18)));
    assert!(date!(2022-02-18).replace_year(-1_000_000_000).is_err()); // -1_000_000_000 isn't a valid year
    assert!(date!(2022-02-18).replace_year(1_000_000_000).is_err()); // 1_000_000_000 isn't a valid year

    // Common to leap year, before leap day.
    assert_eq!(date!(2022-01-01).replace_year(2024), Ok(date!(2024-01-01)));
    // Common to leap year, after leap day.
    assert_eq!(date!(2022-12-01).replace_year(2024), Ok(date!(2024-12-01)));
    // Leap to common year, before leap day.
    assert_eq!(date!(2024-01-01).replace_year(2022), Ok(date!(2022-01-01)));
    // Leap to common year, after leap day.
    assert_eq!(date!(2024-12-01).replace_year(2022), Ok(date!(2022-12-01)));
    // Leap to common year, leap day.
    assert!(date!(2024-02-29).replace_year(2022).is_err());
    // Common to common year.
    assert_eq!(date!(2022-12-01).replace_year(2023), Ok(date!(2023-12-01)));
    // Leap to leap year.
    assert_eq!(date!(2024-12-01).replace_year(2028), Ok(date!(2028-12-01)));
}

#[test]
fn replace_month() {
    assert_eq!(
        date!(2022-02-18).replace_month(Month::January),
        Ok(date!(2022-01-18))
    );
    // 30 isn't a valid day in February
    assert!(date!(2022-01-30).replace_month(Month::February).is_err());
}

#[test]
fn replace_day() {
    assert_eq!(date!(2022-02-18).replace_day(1), Ok(date!(2022-02-01)));
    assert!(date!(2022-02-18).replace_day(0).is_err()); // 0 isn't a valid day
    assert!(date!(2022-02-18).replace_day(30).is_err()); // 30 isn't a valid day in February
}

#[test]
fn replace_ordinal() {
    assert_eq!(date!(2022-02-18).replace_ordinal(1), Ok(date!(2022-001)));
    assert_eq!(date!(2024-02-29).replace_ordinal(366), Ok(date!(2024-366)));
    assert!(date!(2022-049).replace_ordinal(0).is_err()); // 0 isn't a valid day
    assert!(date!(2022-049).replace_ordinal(366).is_err()); // 2022 isn't a leap year
    assert!(date!(2022-049).replace_ordinal(367).is_err()); // 367 isn't a valid day
}

#[test]
fn next_occurrence_test() {
    assert_eq!(
        date!(2023-06-25).next_occurrence(Weekday::Monday),
        date!(2023-06-26)
    );
    assert_eq!(
        date!(2023-06-26).next_occurrence(Weekday::Monday),
        date!(2023-07-03)
    );
    assert_eq!(
        date!(2023-06-27).next_occurrence(Weekday::Monday),
        date!(2023-07-03)
    );
    assert_eq!(
        date!(2023-06-28).next_occurrence(Weekday::Monday),
        date!(2023-07-03)
    );
    assert_eq!(
        date!(2023-06-29).next_occurrence(Weekday::Monday),
        date!(2023-07-03)
    );
    assert_eq!(
        date!(2023-06-30).next_occurrence(Weekday::Monday),
        date!(2023-07-03)
    );
    assert_eq!(
        date!(2023-07-01).next_occurrence(Weekday::Monday),
        date!(2023-07-03)
    );
    assert_eq!(
        date!(2023-07-02).next_occurrence(Weekday::Monday),
        date!(2023-07-03)
    );
    assert_eq!(
        date!(2023-07-03).next_occurrence(Weekday::Monday),
        date!(2023-07-10)
    );
}

#[test]
fn prev_occurrence_test() {
    assert_eq!(
        date!(2023-07-07).prev_occurrence(Weekday::Thursday),
        date!(2023-07-06)
    );
    assert_eq!(
        date!(2023-07-06).prev_occurrence(Weekday::Thursday),
        date!(2023-06-29)
    );
    assert_eq!(
        date!(2023-07-05).prev_occurrence(Weekday::Thursday),
        date!(2023-06-29)
    );
    assert_eq!(
        date!(2023-07-04).prev_occurrence(Weekday::Thursday),
        date!(2023-06-29)
    );
    assert_eq!(
        date!(2023-07-03).prev_occurrence(Weekday::Thursday),
        date!(2023-06-29)
    );
    assert_eq!(
        date!(2023-07-02).prev_occurrence(Weekday::Thursday),
        date!(2023-06-29)
    );
    assert_eq!(
        date!(2023-07-01).prev_occurrence(Weekday::Thursday),
        date!(2023-06-29)
    );
    assert_eq!(
        date!(2023-06-30).prev_occurrence(Weekday::Thursday),
        date!(2023-06-29)
    );
    assert_eq!(
        date!(2023-06-29).prev_occurrence(Weekday::Thursday),
        date!(2023-06-22)
    );
}

#[test]
fn nth_next_occurrence_test() {
    assert_eq!(
        date!(2023-06-25).nth_next_occurrence(Weekday::Monday, 5),
        date!(2023-07-24)
    );
    assert_eq!(
        date!(2023-06-26).nth_next_occurrence(Weekday::Monday, 5),
        date!(2023-07-31)
    );
}

#[test]
fn nth_prev_occurrence_test() {
    assert_eq!(
        date!(2023-06-27).nth_prev_occurrence(Weekday::Monday, 3),
        date!(2023-06-12)
    );
    assert_eq!(
        date!(2023-06-26).nth_prev_occurrence(Weekday::Monday, 3),
        date!(2023-06-05)
    );
}

#[test]
#[should_panic]
fn next_occurrence_overflow_test() {
    date!(+999999-12-25).next_occurrence(Weekday::Saturday);
}

#[test]
#[should_panic]
fn prev_occurrence_overflow_test() {
    date!(-999999-01-07).prev_occurrence(Weekday::Sunday);
}

#[test]
#[should_panic]
fn nth_next_occurrence_overflow_test() {
    date!(+999999-12-25).nth_next_occurrence(Weekday::Saturday, 1);
}

#[test]
#[should_panic]
fn nth_next_occurence_zeroth_occurence_test() {
    date!(2023-06-25).nth_next_occurrence(Weekday::Monday, 0);
}

#[test]
#[should_panic]
fn nth_prev_occurence_zeroth_occurence_test() {
    date!(2023-06-27).nth_prev_occurrence(Weekday::Monday, 0);
}

#[test]
#[should_panic]
fn nth_prev_occurrence_overflow_test() {
    date!(-999999-01-07).nth_prev_occurrence(Weekday::Sunday, 1);
}
