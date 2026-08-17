//! A retry "budget" for allowing only a certain amount of retries over time.

use std::{
    fmt,
    sync::{
        atomic::{AtomicIsize, Ordering},
        Mutex,
    },
    time::Duration,
};
use tokio::time::Instant;

/// Represents a "budget" for retrying requests.
///
/// This is useful for limiting the amount of retries a service can perform
/// over a period of time, or per a certain number of requests attempted.
pub struct Budget {
    bucket: Bucket,
    deposit_amount: isize,
    withdraw_amount: isize,
}

/// Indicates that it is not currently allowed to "withdraw" another retry
/// from the [`Budget`].
#[derive(Debug)]
pub struct Overdrawn {
    _inner: (),
}

#[derive(Debug)]
struct Bucket {
    generation: Mutex<Generation>,
    /// Initial budget allowed for every second.
    reserve: isize,
    /// Slots of a the TTL divided evenly.
    slots: Box<[AtomicIsize]>,
    /// The amount of time represented by each slot.
    window: Duration,
    /// The changers for the current slot to be commited
    /// after the slot expires.
    writer: AtomicIsize,
}

#[derive(Debug)]
struct Generation {
    /// Slot index of the last generation.
    index: usize,
    /// The timestamp since the last generation expired.
    time: Instant,
}

// ===== impl Budget =====

impl Budget {
    /// Create a [`Budget`] that allows for a certain percent of the total
    /// requests to be retried.
    ///
    /// - The `ttl` is the duration of how long a single `deposit` should be
    ///   considered. Must be between 1 and 60 seconds.
    /// - The `min_per_sec` is the minimum rate of retries allowed to accomodate
    ///   clients that have just started issuing requests, or clients that do
    ///   not issue many requests per window.
    /// - The `retry_percent` is the percentage of calls to `deposit` that can
    ///   be retried. This is in addition to any retries allowed for via
    ///   `min_per_sec`. Must be between 0 and 1000.
    ///
    ///   As an example, if `0.1` is used, then for every 10 calls to `deposit`,
    ///   1 retry will be allowed. If `2.0` is used, then every `deposit`
    ///   allows for 2 retries.
    pub fn new(ttl: Duration, min_per_sec: u32, retry_percent: f32) -> Self {
        // assertions taken from finagle
        assert!(ttl >= Duration::from_secs(1));
        assert!(ttl <= Duration::from_secs(60));
        assert!(retry_percent >= 0.0);
        assert!(retry_percent <= 1000.0);
        assert!(min_per_sec < ::std::i32::MAX as u32);

        let (deposit_amount, withdraw_amount) = if retry_percent == 0.0 {
            // If there is no percent, then you gain nothing from deposits.
            // Withdrawals can only be made against the reserve, over time.
            (0, 1)
        } else if retry_percent <= 1.0 {
            (1, (1.0 / retry_percent) as isize)
        } else {
            // Support for when retry_percent is between 1.0 and 1000.0,
            // meaning for every deposit D, D*retry_percent withdrawals
            // can be made.
            (1000, (1000.0 / retry_percent) as isize)
        };
        let reserve = (min_per_sec as isize)
            .saturating_mul(ttl.as_secs() as isize) // ttl is between 1 and 60 seconds
            .saturating_mul(withdraw_amount);

        // AtomicIsize isn't clone, so the slots need to be built in a loop...
        let windows = 10u32;
        let mut slots = Vec::with_capacity(windows as usize);
        for _ in 0..windows {
            slots.push(AtomicIsize::new(0));
        }

        Budget {
            bucket: Bucket {
                generation: Mutex::new(Generation {
                    index: 0,
                    time: Instant::now(),
                }),
                reserve,
                slots: slots.into_boxed_slice(),
                window: ttl / windows,
                writer: AtomicIsize::new(0),
            },
            deposit_amount,
            withdraw_amount,
        }
    }

    /// Store a "deposit" in the budget, which will be used to permit future
    /// withdrawals.
    pub fn deposit(&self) {
        self.bucket.put(self.deposit_amount);
    }

    /// Check whether there is enough "balance" in the budget to issue a new
    /// retry.
    ///
    /// If there is not enough, an `Err(Overdrawn)` is returned.
    pub fn withdraw(&self) -> Result<(), Overdrawn> {
        if self.bucket.try_get(self.withdraw_amount) {
            Ok(())
        } else {
            Err(Overdrawn { _inner: () })
        }
    }
}

impl Default for Budget {
    fn default() -> Budget {
        Budget::new(Duration::from_secs(10), 10, 0.2)
    }
}

impl fmt::Debug for Budget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Budget")
            .field("deposit", &self.deposit_amount)
            .field("withdraw", &self.withdraw_amount)
            .field("balance", &self.bucket.sum())
            .finish()
    }
}

// ===== impl Bucket =====

impl Bucket {
    fn put(&self, amt: isize) {
        self.expire();
        self.writer.fetch_add(amt, Ordering::SeqCst);
    }

    fn try_get(&self, amt: isize) -> bool {
        debug_assert!(amt >= 0);

        self.expire();

        let sum = self.sum();
        if sum >= amt {
            self.writer.fetch_add(-amt, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    fn expire(&self) {
        let mut gen = self.generation.lock().expect("generation lock");

        let now = Instant::now();
        let diff = now.saturating_duration_since(gen.time);
        if diff < self.window {
            // not expired yet
            return;
        }

        let to_commit = self.writer.swap(0, Ordering::SeqCst);
        self.slots[gen.index].store(to_commit, Ordering::SeqCst);

        let mut diff = diff;
        let mut idx = (gen.index + 1) % self.slots.len();
        while diff > self.window {
            self.slots[idx].store(0, Ordering::SeqCst);
            diff -= self.window;
            idx = (idx + 1) % self.slots.len();
        }

        gen.index = idx;
        gen.time = now;
    }

    fn sum(&self) -> isize {
        let current = self.writer.load(Ordering::SeqCst);
        let windowed_sum: isize = self
            .slots
            .iter()
            .map(|slot| slot.load(Ordering::SeqCst))
            // fold() is used instead of sum() to determine overflow behavior
            .fold(0, isize::saturating_add);

        current
            .saturating_add(windowed_sum)
            .saturating_add(self.reserve)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time;

    #[test]
    fn empty() {
        let bgt = Budget::new(Duration::from_secs(1), 0, 1.0);
        bgt.withdraw().unwrap_err();
    }

    #[tokio::test]
    async fn leaky() {
        time::pause();

        let bgt = Budget::new(Duration::from_secs(1), 0, 1.0);
        bgt.deposit();

        time::advance(Duration::from_secs(3)).await;

        bgt.withdraw().unwrap_err();
    }

    #[tokio::test]
    async fn slots() {
        time::pause();

        let bgt = Budget::new(Duration::from_secs(1), 0, 0.5);
        bgt.deposit();
        bgt.deposit();
        time::advance(Duration::from_millis(901)).await;
        // 900ms later, the deposit should still be valid
        bgt.withdraw().unwrap();

        // blank slate
        time::advance(Duration::from_millis(2001)).await;

        bgt.deposit();
        time::advance(Duration::from_millis(301)).await;
        bgt.deposit();
        time::advance(Duration::from_millis(801)).await;
        bgt.deposit();

        // the first deposit is expired, but the 2nd should still be valid,
        // combining with the 3rd
        bgt.withdraw().unwrap();
    }

    #[tokio::test]
    async fn reserve() {
        let bgt = Budget::new(Duration::from_secs(1), 5, 1.0);
        bgt.withdraw().unwrap();
        bgt.withdraw().unwrap();
        bgt.withdraw().unwrap();
        bgt.withdraw().unwrap();
        bgt.withdraw().unwrap();

        bgt.withdraw().unwrap_err();
    }
}
