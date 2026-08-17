/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{fmt, time::Duration};

#[derive(Debug)]
pub(super) struct Backoff {
    min_delay: Duration,
    max_delay: Duration,
    max_wait: Duration,
    attempt_ceiling: u32,
    random: RandomImpl,
}

impl Backoff {
    pub(super) fn new(
        min_delay: Duration,
        max_delay: Duration,
        max_wait: Duration,
        random: RandomImpl,
    ) -> Self {
        Self {
            min_delay,
            max_delay,
            max_wait,
            // Attempt ceiling calculation taken from the Smithy spec: https://smithy.io/2.0/additional-specs/waiters.html#waiter-retries
            attempt_ceiling: (((max_delay.as_secs_f64() / min_delay.as_secs_f64()).ln()
                / 2f64.ln())
                + 1.0) as u32,
            random,
        }
    }

    // Calculates backoff delay time according to the Smithy spec: https://smithy.io/2.0/additional-specs/waiters.html#waiter-retries
    pub(super) fn delay(&self, attempt: u32, elapsed: Duration) -> Duration {
        let delay = if attempt > self.attempt_ceiling {
            self.max_delay.as_secs()
        } else {
            self.min_delay.as_secs() * 2u64.pow(attempt - 1)
        };
        let mut delay = Duration::from_secs(self.random.random(self.min_delay.as_secs(), delay));

        let remaining_time = self.max_wait.saturating_sub(elapsed);
        if remaining_time.saturating_sub(delay) <= self.min_delay {
            // Note: deviating from the spec here. Subtracting `min_delay` doesn't fulfill the original intent.
            delay = remaining_time;
        }
        delay
    }

    #[inline]
    pub(super) fn max_wait(&self) -> Duration {
        self.max_wait
    }
}

#[derive(Default)]
pub(super) enum RandomImpl {
    #[default]
    Default,
    #[cfg(test)]
    Override(Box<dyn Fn(u64, u64) -> u64 + Send + Sync>),
}

impl fmt::Debug for RandomImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "Default"),
            #[cfg(test)]
            Self::Override(_) => f.debug_tuple("Override").field(&"** function **").finish(),
        }
    }
}

impl RandomImpl {
    fn random(&self, min_inclusive: u64, max_inclusive: u64) -> u64 {
        match self {
            Self::Default => fastrand::u64(min_inclusive..=max_inclusive),
            #[cfg(test)]
            Self::Override(overrid) => (overrid)(min_inclusive, max_inclusive),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    fn test_backoff(
        min_delay: u64,
        max_delay: u64,
        test_random: impl Fn(u64, u64) -> u64 + Send + Sync + 'static,
        attempt_delays: &[(u64, u64)],
    ) {
        let backoff = dbg!(Backoff::new(
            Duration::from_secs(min_delay),
            Duration::from_secs(max_delay),
            Duration::from_secs(300),
            RandomImpl::Override(Box::new(test_random)),
        ));

        for (index, (delay, time)) in attempt_delays.iter().enumerate() {
            let attempt = index + 1;
            println!("attempt: {attempt}, delay: {delay}, time: {time}");
            assert_eq!(
                Duration::from_secs(*delay),
                backoff.delay(attempt as _, Duration::from_secs(*time))
            );
        }
    }

    #[test]
    fn backoff_jitter_as_average() {
        let test_random = |min: u64, max: u64| (min + max) / 2;
        let attempt_delays = &[
            // delay, time
            (2, 2),
            (3, 4),
            (5, 7),
            (9, 12),
            (17, 21),
            (33, 38),
            (61, 71),
            (61, 132),
            (61, 193),
            (46, 254),
            (0, 300),
        ];
        test_backoff(2, 120, test_random, attempt_delays);
    }

    #[test]
    fn backoff_with_seeded_jitter() {
        let random = Arc::new(Mutex::new(fastrand::Rng::with_seed(1)));
        let test_random = move |min: u64, max: u64| random.lock().unwrap().u64(min..=max);
        let attempt_delays = &[
            // delay, time
            (2, 2),
            (3, 4),
            (3, 7),
            (13, 12),
            (2, 14),
            (51, 16),
            (93, 73),
            (102, 164),
            (73, 170),
            (21, 227),
            (9, 256),
            (17, 283),
            (0, 300),
        ];
        test_backoff(2, 120, test_random, attempt_delays);
    }

    #[test]
    fn backoff_with_large_min_delay() {
        let test_random = |min: u64, max: u64| (min + max) / 2;
        let attempt_delays = &[
            // delay, time
            (15, 1),
            (22, 16),
            (37, 38),
            (67, 75),
            (67, 142),
            (24, 276),
            (0, 300),
        ];
        test_backoff(15, 120, test_random, attempt_delays);
    }
}
