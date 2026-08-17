use oorandom::Rand64;
use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Rng = Rand64;

thread_local! {
    static SEED_RAND: RefCell<Rand64> = RefCell::new(Rand64::new(
        SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    ));
}

pub fn new_rng() -> Rng {
    SEED_RAND.with(|r| {
        let mut r = r.borrow_mut();
        let seed = ((r.rand_u64() as u128) << 64) | (r.rand_u64() as u128);
        Rand64::new(seed)
    })
}
