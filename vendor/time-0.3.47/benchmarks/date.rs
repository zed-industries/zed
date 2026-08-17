#![expect(
    clippy::large_stack_frames,
    reason = "iterating over large array; does not cause stack overflow"
)]

use std::hint::black_box as bb;
use std::sync::LazyLock;

use criterion::Bencher;
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::date;
use time::{Date, Time};

/// Generate a representative sample of all dates.
///
/// The ratio of month sizes, week sizes, year sign, leap years, etc. are all identical to the full
/// range. This ensures that benchmarks accurately reflect random data.
//
// Note that this is a _very_ large array (over 1 MiB), so we silence the warning about large stack
// frames at the top of this file.
fn representative_dates() -> [Date; 292_194] {
    static DATES: LazyLock<[Date; 292_194]> = LazyLock::new(|| {
        let mut dates = [Date::MIN; _];
        let mut current = date!(-0400-01-01);
        let mut i = 0;
        while current < date!(0400-01-01) {
            dates[i] = current;
            current = current.next_day().expect("date is in range");
            i += 1;
        }
        crate::shuffle(dates)
    });

    *DATES
}

setup_benchmark! {
    "Date",

    fn noop(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(date);
            }
        });
    }

    fn noop_windows(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates().windows(2) {
                let first = date[0];
                let second = date[1];
                let _ = bb((bb(first), bb(second)));
            }
        });
    }

    fn from_calendar_date(ben: &mut Bencher<'_>) {
        let dates = representative_dates().map(Date::to_calendar_date);
        ben.iter(|| {
            for (year, month, day) in dates {
                let _ = bb(Date::from_calendar_date(bb(year), bb(month), bb(day)));
            }
        });
    }

    fn from_ordinal_date(ben: &mut Bencher<'_>) {
        let dates = representative_dates().map(Date::to_ordinal_date);
        ben.iter(|| {
            for (year, ordinal) in dates {
                let _ = bb(Date::from_ordinal_date(bb(year), bb(ordinal)));
            }
        });

    }

    fn from_iso_week_date(ben: &mut Bencher<'_>) {
        let dates = representative_dates().map(Date::to_iso_week_date);
        ben.iter(|| {
            for (year, week, weekday) in dates {
                let _ = bb(Date::from_iso_week_date(bb(year), bb(week), bb(weekday)));
            }
        });
    }

    fn from_julian_day(ben: &mut Bencher<'_>) {
        let dates = representative_dates().map(Date::to_julian_day);
        ben.iter(|| {
            for julian_day in dates {
                let _ = bb(Date::from_julian_day(bb(julian_day)));
            }
        });
    }

    fn year(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).year());
            }
        });
    }

    fn month(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).month());
            }
        });
    }

    fn day(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).day());
            }
        });
    }

    fn ordinal(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).ordinal());
            }
        });
    }

    fn iso_week(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).iso_week());
            }
        });
    }

    fn sunday_based_week(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).sunday_based_week());
            }
        });
    }

    fn monday_based_week(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).monday_based_week());
            }
        });
    }

    fn to_calendar_date(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).to_calendar_date());
            }
        });
    }

    fn to_ordinal_date(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).to_ordinal_date());
            }
        });
    }

    fn to_iso_week_date(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).to_iso_week_date());
            }
        });
    }

    fn weekday(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).weekday());
            }
        });
    }

    fn next_day(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).next_day());
            }
        });
    }

    fn previous_day(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).previous_day());
            }
        });
    }

    fn to_julian_day(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).to_julian_day());
            }
        });
    }

    fn midnight(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).midnight());
            }
        });
    }

    fn with_time(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).with_time(bb(Time::MIDNIGHT)));
            }
        });
    }

    fn with_hms(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).with_hms(bb(0), bb(0), bb(0)));
            }
        });
    }

    fn with_hms_milli(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).with_hms_milli(bb(0), bb(0), bb(0), bb(0)));
            }
        });
    }

    fn with_hms_micro(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).with_hms_micro(bb(0), bb(0), bb(0), bb(0)));
            }
        });
    }

    fn with_hms_nano(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date).with_hms_nano(bb(0), bb(0), bb(0), bb(0)));
            }
        });
    }

    fn add(ben: &mut Bencher<'_>) {
        let dt = 5.days();
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date) + bb(dt));
            }
        });
    }

    fn add_std(ben: &mut Bencher<'_>) {
        let dt = 5.std_days();
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date) - bb(dt));
            }
        });
    }

    fn add_assign(ben: &mut Bencher<'_>) {
        let dt = 1.days();
        ben.iter(|| {
            for mut date in representative_dates() {
                date += bb(dt);
                let _ = bb(date);
            }
        });
    }

    fn add_assign_std(ben: &mut Bencher<'_>) {
        let dt = 1.std_days();
        ben.iter(|| {
            for mut date in representative_dates() {
                date += bb(dt);
                let _ = bb(date);
            }
        });
    }

    fn sub(ben: &mut Bencher<'_>) {
        let dt = 5.days();
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date) - bb(dt));
            }
        });
    }

    fn sub_std(ben: &mut Bencher<'_>) {
        let dt = 5.std_days();
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date) - bb(dt));
            }
        });
    }

    fn sub_assign(ben: &mut Bencher<'_>) {
        let dt = 1.days();
        ben.iter(|| {
            for mut date in representative_dates() {
                date -= bb(dt);
                let _ = bb(date);
            }
        });
    }

    fn sub_assign_std(ben: &mut Bencher<'_>) {
        let dt = 1.std_days();
        ben.iter(|| {
            for mut date in representative_dates() {
                date -= bb(dt);
                let _ = bb(date);
            }
        });
    }

    fn sub_self(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates() {
                let _ = bb(bb(date) - bb(date));
            }
        });
    }

    fn partial_ord(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates().windows(2) {
                let first = date[0];
                let second = date[1];
                let _ = bb(bb(first).partial_cmp(&bb(second)));
            }
        });
    }

    fn ord(ben: &mut Bencher<'_>) {
        ben.iter(|| {
            for date in representative_dates().windows(2) {
                let first = date[0];
                let second = date[1];
                let _ = bb(bb(first).cmp(&bb(second)));
            }
        });
    }
}
