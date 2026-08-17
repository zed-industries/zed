use rand08::Rng as _;
use rand09::Rng as _;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

#[test]
fn support08() {
    // Work around rust-random/rand#1020.
    let mut rng = rand08::rngs::mock::StepRng::new(0, 656_175_560);

    for _ in 0..7 {
        let _ = rng.r#gen::<Weekday>();
    }
    for _ in 0..12 {
        let _ = rng.r#gen::<Month>();
    }
    let _ = rng.r#gen::<Time>();
    let _ = rng.r#gen::<Date>();
    let _ = rng.r#gen::<UtcOffset>();
    let _ = rng.r#gen::<PrimitiveDateTime>();
    let _ = rng.r#gen::<OffsetDateTime>();
    let _ = rng.r#gen::<Duration>();
}

#[test]
fn support09() {
    // Work around rust-random/rand#1020.
    let mut rng = StepRng::new(0, 656_175_560);

    for _ in 0..7 {
        let _ = rng.random::<Weekday>();
    }
    for _ in 0..12 {
        let _ = rng.random::<Month>();
    }
    let _ = rng.random::<Time>();
    let _ = rng.random::<Date>();
    let _ = rng.random::<UtcOffset>();
    let _ = rng.random::<PrimitiveDateTime>();
    let _ = rng.random::<OffsetDateTime>();
    let _ = rng.random::<Duration>();
}

// copy of `StepRng` from rand 0.8 to avoid deprecation warnings
#[derive(Debug, Clone)]
struct StepRng {
    v: u64,
    a: u64,
}

impl StepRng {
    fn new(initial: u64, increment: u64) -> Self {
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
