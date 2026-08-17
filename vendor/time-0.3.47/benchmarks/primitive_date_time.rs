use criterion::Bencher;
use time::ext::{NumericalDuration, NumericalStdDuration};
use time::macros::{datetime, offset};

setup_benchmark! {
    "PrimitiveDateTime",

    // All getters are trivially dispatched to the relevant field, and do not need to be benchmarked
    // a second time.

    fn assume_offset(ben: &mut Bencher<'_>) {
        ben.iter(|| datetime!(2019-01-01 0:00).assume_offset(offset!(UTC)));
        ben.iter(|| datetime!(2019-01-01 0:00).assume_offset(offset!(-1)));
    }

    fn assume_utc(ben: &mut Bencher<'_>) {
        ben.iter(|| datetime!(2019-01-01 0:00).assume_utc());
    }

    fn add_duration(ben: &mut Bencher<'_>) {
        let a = 5.days();
        let b = 1.days();
        let c = 2.seconds();
        let d = (-2).seconds();
        let e = 1.hours();

        ben.iter(|| datetime!(2019-01-01 0:00) + a);
        ben.iter(|| datetime!(2019-12-31 0:00) + b);
        ben.iter(|| datetime!(2019-12-31 23:59:59) + c);
        ben.iter(|| datetime!(2020-01-01 0:00:01) + d);
        ben.iter(|| datetime!(1999-12-31 23:00) + e);
    }

    fn add_std_duration(ben: &mut Bencher<'_>) {
        let a = 5.std_days();
        let b = 1.std_days();
        let c = 2.std_seconds();

        ben.iter(|| datetime!(2019-01-01 0:00) + a);
        ben.iter(|| datetime!(2019-12-31 0:00) + b);
        ben.iter(|| datetime!(2019-12-31 23:59:59) + c);
    }

    fn add_assign_duration(ben: &mut Bencher<'_>) {
        let a = 1.days();
        let b = 1.seconds();
        iter_batched_ref!(
            ben,
            || datetime!(2019-01-01 0:00),
            [
                |datetime| *datetime += a,
                |datetime| *datetime += b,
            ]
        );
    }

    fn add_assign_std_duration(ben: &mut Bencher<'_>) {
        let a = 1.std_days();
        let b = 1.std_seconds();
        iter_batched_ref!(
            ben,
            || datetime!(2019-01-01 0:00),
            [
                |datetime| *datetime += a,
                |datetime| *datetime += b,
            ]
        );
    }

    fn sub_duration(ben: &mut Bencher<'_>) {
        let a = 5.days();
        let b = 1.days();
        let c = 2.seconds();
        let d = (-2).seconds();
        let e = (-1).hours();

        ben.iter(|| datetime!(2019-01-06 0:00) - a);
        ben.iter(|| datetime!(2020-01-01 0:00) - b);
        ben.iter(|| datetime!(2020-01-01 0:00:01) - c);
        ben.iter(|| datetime!(2019-12-31 23:59:59) - d);
        ben.iter(|| datetime!(1999-12-31 23:00) - e);
    }

    fn sub_std_duration(ben: &mut Bencher<'_>) {
        let a = 5.std_days();
        let b = 1.std_days();
        let c = 2.std_seconds();

        ben.iter(|| datetime!(2019-01-06 0:00) - a);
        ben.iter(|| datetime!(2020-01-01 0:00) - b);
        ben.iter(|| datetime!(2020-01-01 0:00:01) - c);
    }

    fn sub_assign_duration(ben: &mut Bencher<'_>) {
        let a = 1.days();
        let b = 1.seconds();
        iter_batched_ref!(
            ben,
            || datetime!(2019-01-01 0:00),
            [
                |datetime| *datetime -= a,
                |datetime| *datetime -= b,
            ]
        );
    }

    fn sub_assign_std_duration(ben: &mut Bencher<'_>) {
        let a = 1.std_days();
        let b = 1.std_seconds();
        iter_batched_ref!(
            ben,
            || datetime!(2019-01-01 0:00),
            [
                |datetime| *datetime -= a,
                |datetime| *datetime -= b,
            ]
        );
    }

    fn sub_datetime(ben: &mut Bencher<'_>) {
        ben.iter(|| datetime!(2019-01-02 0:00) - datetime!(2019-01-01 0:00));
        ben.iter(|| datetime!(2019-01-01 0:00) - datetime!(2019-01-02 0:00));
        ben.iter(|| datetime!(2020-01-01 0:00) - datetime!(2019-12-31 0:00));
        ben.iter(|| datetime!(2019-12-31 0:00) - datetime!(2020-01-01 0:00));
    }

    fn ord(ben: &mut Bencher<'_>) {
        ben.iter(|| datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00)));
        ben.iter(|| datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2020-01-01 0:00)));
        ben.iter(|| datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-02-01 0:00)));
        ben.iter(|| datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-02 0:00)));
        ben.iter(|| datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 1:00)));
        ben.iter(|| datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:01)));
        ben.iter(|| datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00:01)));
        ben.iter(|| datetime!(2019-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00:00.000_000_001)));
        ben.iter(|| datetime!(2020-01-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00)));
        ben.iter(|| datetime!(2019-02-01 0:00).partial_cmp(&datetime!(2019-01-01 0:00)));
        ben.iter(|| datetime!(2019-01-02 0:00).partial_cmp(&datetime!(2019-01-01 0:00)));
        ben.iter(|| datetime!(2019-01-01 1:00).partial_cmp(&datetime!(2019-01-01 0:00)));
        ben.iter(|| datetime!(2019-01-01 0:01).partial_cmp(&datetime!(2019-01-01 0:00)));
        ben.iter(|| datetime!(2019-01-01 0:00:01).partial_cmp(&datetime!(2019-01-01 0:00)));
        ben.iter(|| datetime!(2019-01-01 0:00:00.000_000_001).partial_cmp(&datetime!(2019-01-01 0:00)));
    }
}
