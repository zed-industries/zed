use criterion::Bencher;
use rand08::rngs::mock::StepRng;
use rand08::Rng;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

macro_rules! bench_rand {
    ($($name:ident : $type:ty),* $(,)?) => {
        setup_benchmark! {
            "Random",
            $(fn $name(ben: &mut Bencher<'_>) {
                iter_batched_ref!(
                    ben,
                    || StepRng::new(0, 1),
                    [|rng| rng.r#gen::<$type>()]
                );
            })*
        }
    }
}

bench_rand![
    time: Time,
    date: Date,
    utc_offset: UtcOffset,
    primitive_date_time: PrimitiveDateTime,
    offset_date_time: OffsetDateTime,
    duration: Duration,
    weekday: Weekday,
    month: Month,
];
