use std::time::Instant;

use parking_lot::Mutex;

pub trait Clock {
    fn now(&self) -> Instant;
}

pub struct TestClock {
    now: Mutex<Instant>,
}

impl TestClock {
    pub fn new(now: Instant) -> Self {
        Self {
            now: Mutex::new(now),
        }
    }

    pub fn set_now(&self, now: Instant) {
        *self.now.lock() = now;
    }
}

impl Clock for TestClock {
    fn now(&self) -> Instant {
        *self.now.lock()
    }
}
