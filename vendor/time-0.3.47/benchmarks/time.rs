use std::hint::black_box;

use criterion::Bencher;
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::time;
use time::Time;

setup_benchmark! {
    "Time",

    fn from_hms(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::from_hms(1, 2, 3));
    }

    fn from_hms_milli(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::from_hms_milli(1, 2, 3, 4));
    }

    fn from_hms_micro(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::from_hms_micro(1, 2, 3, 4));
    }

    fn from_hms_nano(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::from_hms_nano(1, 2, 3, 4));
    }

    fn as_hms(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT.as_hms());
    }

    fn as_hms_milli(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT.as_hms_milli());
    }

    fn as_hms_micro(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT.as_hms_micro());
    }

    fn as_hms_nano(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT.as_hms_nano());
    }

    fn hour(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT.hour());
    }

    fn minute(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT.minute());
    }

    fn second(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT.second());
    }

    fn millisecond(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT.millisecond());
    }

    fn microsecond(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT.microsecond());
    }

    fn nanosecond(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT.nanosecond());
    }

    fn add_duration(ben: &mut Bencher<'_>) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();
        ben.iter(|| Time::MIDNIGHT + a);
        ben.iter(|| Time::MIDNIGHT + b);
        ben.iter(|| Time::MIDNIGHT + c);
        ben.iter(|| Time::MIDNIGHT + d);
        ben.iter(|| Time::MIDNIGHT + e);
    }

    fn add_assign_duration(ben: &mut Bencher<'_>) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time += a,
                |time| *time += b,
                |time| *time += c,
                |time| *time += d,
                |time| *time += e,
            ]
        );
    }

    fn sub_duration(ben: &mut Bencher<'_>) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();
        ben.iter(|| Time::MIDNIGHT - a);
        ben.iter(|| Time::MIDNIGHT - b);
        ben.iter(|| Time::MIDNIGHT - c);
        ben.iter(|| Time::MIDNIGHT - d);
        ben.iter(|| Time::MIDNIGHT - e);
    }

    fn sub_assign_duration(ben: &mut Bencher<'_>) {
        let a = 1.milliseconds();
        let b = 1.seconds();
        let c = 1.minutes();
        let d = 1.hours();
        let e = 1.days();
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time -= a,
                |time| *time -= b,
                |time| *time -= c,
                |time| *time -= d,
                |time| *time -= e,
            ]
        );
    }

    fn add_std_duration(ben: &mut Bencher<'_>) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();
        ben.iter(|| Time::MIDNIGHT + a);
        ben.iter(|| Time::MIDNIGHT + b);
        ben.iter(|| Time::MIDNIGHT + c);
        ben.iter(|| Time::MIDNIGHT + d);
        ben.iter(|| Time::MIDNIGHT + e);
    }

    fn add_assign_std_duration(ben: &mut Bencher<'_>) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time += a,
                |time| *time += b,
                |time| *time += c,
                |time| *time += d,
                |time| *time += e,
            ]
        );
    }

    fn sub_std_duration(ben: &mut Bencher<'_>) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();
        ben.iter(|| Time::MIDNIGHT - a);
        ben.iter(|| Time::MIDNIGHT - b);
        ben.iter(|| Time::MIDNIGHT - c);
        ben.iter(|| Time::MIDNIGHT - d);
        ben.iter(|| Time::MIDNIGHT - e);
    }

    fn sub_assign_std_duration(ben: &mut Bencher<'_>) {
        let a = 1.std_milliseconds();
        let b = 1.std_seconds();
        let c = 1.std_minutes();
        let d = 1.std_hours();
        let e = 1.std_days();
        iter_batched_ref!(
            ben,
            || Time::MIDNIGHT,
            [
                |time| *time -= a,
                |time| *time -= b,
                |time| *time -= c,
                |time| *time -= d,
                |time| *time -= e,
            ]
        );
    }

    fn sub_time(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT - time!(0:00:01));
        ben.iter(|| time!(1:00) - Time::MIDNIGHT);
        ben.iter(|| time!(1:00) - time!(0:00:01));
    }

    fn ordering(ben: &mut Bencher<'_>) {
        ben.iter(|| Time::MIDNIGHT < time!(0:00:00.000_000_001));
        ben.iter(|| Time::MIDNIGHT < time!(0:00:01));
        ben.iter(|| time!(12:00) > time!(11:00));
        ben.iter(|| Time::MIDNIGHT == time!(0:00:00.000_000_001));
    }

    fn sort_align_8(ben: &mut Bencher<'_>) {
        ben.iter_batched_ref(
            || {
                #[repr(C,align(8))]
                struct Padder {
                    arr: [Time;4096],
                }
                let mut res = Padder {
                    arr: [Time::MIDNIGHT;4096]
                };
                let mut last = Time::MIDNIGHT;
                let mut last_hour = 0;
                for t in &mut res.arr {
                    *t = last;
                    t.replace_hour(last_hour).expect("failed to replace hour");
                    last += 997.std_milliseconds();
                    last_hour = (last_hour + 5) % 24;
                }
                res.arr.sort_unstable_by_key(|t|
                    (t.nanosecond(),t.second(),t.minute(),t.hour())
                );
                res
            },
            |v| black_box(v).arr.sort_unstable(),
            criterion::BatchSize::SmallInput
        )
    }

    fn sort_align_4(ben: &mut Bencher<'_>) {
        ben.iter_batched_ref(
            || {
                #[repr(C,align(8))]
                struct Padder {
                    pad: u32,
                    arr: [Time;4096],
                }
                let mut res = Padder {
                    pad: 0,
                    arr: [Time::MIDNIGHT;4096]
                };
                let mut last = Time::MIDNIGHT;
                let mut last_hour = 0;
                for t in &mut res.arr {
                    *t = last;
                    t.replace_hour(last_hour).expect("failed to replace hour");
                    last += 997.std_milliseconds();
                    last_hour = (last_hour + 5) % 24;
                }
                res.arr.sort_unstable_by_key(|t|
                    (t.nanosecond(),t.second(),t.minute(),t.hour())
                );
                res
            },
            |v| black_box(v).arr.sort_unstable(),
            criterion::BatchSize::SmallInput
        )
    }

    fn duration_until(ben: &mut Bencher<'_>) {
        let a = black_box(time!(1:02:03.004_005_006));
        let b = black_box(time!(4:05:06.007_008_009));
        ben.iter(|| black_box(a.duration_until(b)));
    }
}
