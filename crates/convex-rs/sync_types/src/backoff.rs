use std::{
    cmp,
    time::Duration,
};

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Backoff {
    initial_backoff: Duration,
    max_backoff: Duration,
    num_failures: u32,
}

impl Backoff {
    pub const fn new(initial_backoff: Duration, max_backoff: Duration) -> Self {
        Self {
            initial_backoff,
            max_backoff,
            num_failures: 0,
        }
    }

    pub fn reset(&mut self) {
        self.num_failures = 0;
    }

    /// For stateful backoff, set the number of failures to a specific value.
    pub fn set_failures(&mut self, num_failures: u32) {
        self.num_failures = num_failures;
    }

    pub fn fail(&mut self, rng: &mut impl Rng) -> Duration {
        // See https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/
        let p = 2u32.checked_pow(self.num_failures).unwrap_or(u32::MAX);
        self.num_failures += 1;
        let jitter = rng.random::<f32>();
        let backoff = self
            .initial_backoff
            .checked_mul(p)
            .unwrap_or(self.max_backoff);
        cmp::min(backoff, self.max_backoff).mul_f32(jitter)
    }

    pub fn failures(&self) -> u32 {
        self.num_failures
    }
}
