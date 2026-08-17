use criterion::Bencher;
use rand09::Rng;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

macro_rules! bench_rand {
    ($($name:ident : $type:ty),* $(,)?) => {
        setup_benchmark! {
            "Random",
            $(fn $name(ben: &mut Bencher<'_>) {
                iter_batched_ref!(
                    ben,
                    || StepRng::new(0, 1),
                    [|rng| rng.random::<$type>()]
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

// copy of `StepRng` from rand 0.8 to avoid deprecation warnings
#[derive(Debug, Clone)]
struct StepRng {
    v: u64,
    a: u64,
}

impl StepRng {
    const fn new(initial: u64, increment: u64) -> Self {
        Self {
            v: initial,
            a: increment,
        }
    }
}

impl rand09::RngCore for StepRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let res = self.v;
        self.v = self.v.wrapping_add(self.a);
        res
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        rand09::rand_core::impls::fill_bytes_via_next(self, dst)
    }
}
