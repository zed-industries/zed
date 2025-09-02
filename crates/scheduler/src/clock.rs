use chrono::{DateTime, Duration, Utc};
use parking_lot::Mutex;

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}

pub struct TestClock {
    now: Mutex<DateTime<Utc>>,
}

impl TestClock {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            now: Mutex::new(now),
        }
    }

    pub fn set_now(&self, now: DateTime<Utc>) {
        *self.now.lock() = now;
    }

    pub fn advance(&self, duration: Duration) {
        *self.now.lock() += duration;
    }
}

impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        *self.now.lock()
    }
}
