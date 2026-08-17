//! This example demonstrates how to implement a jittered expiry policy for a cache.
//!
//! The `JitteredExpiry` struct is a custom expiry policy that adds jitter to the
//! expiry duration. It implements the `moka::Expiry` trait and calculates the expiry
//! duration after a write or read operation. The jitter is randomly generated and
//! added to or subtracted from the base expiry duration.
//!
//! This example uses the `moka::sync::Cache` type, which is a synchronous cache. The
//! same expiry policy can be used with the asynchronous cache, `moka::future::Cache`.

use std::time::{Duration, Instant};

use moka::{sync::Cache, Expiry};
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

/// A `moka::Expiry` implementation that adds jitter to the expiry duration.
pub struct JitteredExpiry<J> {
    /// Optional time-to-live duration.
    time_to_live: Option<Duration>,
    /// Optional time-to-idle duration.
    time_to_idle: Option<Duration>,
    /// The distribution to randomly generate the jitter. The jitter is added to
    /// or subtracted from the expiry duration.
    jitter_gen: J,
}

impl<J> JitteredExpiry<J>
where
    J: Distribution<Duration>,
{
    pub fn new(
        time_to_live: Option<Duration>,
        time_to_idle: Option<Duration>,
        jitter_gen: J,
    ) -> Self {
        Self {
            time_to_live,
            time_to_idle,
            jitter_gen,
        }
    }

    /// Calculates the expiry duration after a write operation.
    pub fn calc_expiry_for_write(&self) -> Option<Duration> {
        if matches!((self.time_to_live, self.time_to_idle), (None, None)) {
            return None;
        }

        let expiry = match (self.time_to_live, self.time_to_idle) {
            (Some(ttl), None) => ttl,
            (None, Some(tti)) => tti,
            (Some(ttl), Some(tti)) => ttl.min(tti),
            (None, None) => unreachable!(),
        };

        Some(self.add_jitter(expiry))
    }

    /// Calculates the expiry duration after a read operation.
    pub fn calc_expiry_for_read(&self, read_at: Instant, modified_at: Instant) -> Option<Duration> {
        if matches!((self.time_to_live, self.time_to_idle), (None, None)) {
            return None;
        }

        let expiry = match (self.time_to_live, self.time_to_idle) {
            (Some(ttl), None) => {
                let elapsed = Self::elapsed_since_write(read_at, modified_at);
                Self::remaining_to_ttl(ttl, elapsed)
            }
            (None, Some(tti)) => tti,
            (Some(ttl), Some(tti)) => {
                // Ensure that the expiry duration does not exceed the
                // time-to-live since last write.
                let elapsed = Self::elapsed_since_write(read_at, modified_at);
                let remaining = Self::remaining_to_ttl(ttl, elapsed);
                tti.min(remaining)
            }
            (None, None) => unreachable!(),
        };

        Some(self.add_jitter(expiry))
    }

    /// Calculates the elapsed time between `modified_at` and `read_at`.
    fn elapsed_since_write(read_at: Instant, modified_at: Instant) -> Duration {
        // NOTE: `duration_since` panics if `read_at` is earlier than `modified_at`.
        if read_at >= modified_at {
            read_at.duration_since(modified_at)
        } else {
            Duration::default() // zero duration
        }
    }

    /// Calculates the remaining time to live based on the `ttl` and `elapsed` time.
    fn remaining_to_ttl(ttl: Duration, elapsed: Duration) -> Duration {
        ttl.saturating_sub(elapsed)
    }

    /// Adds jitter to the given duration.
    fn add_jitter(&self, duration: Duration) -> Duration {
        let mut rng = rand::thread_rng();
        let jitter = self.jitter_gen.sample(&mut rng);

        // Add or subtract the jitter to/from the duration.
        if rng.gen() {
            duration.saturating_add(jitter)
        } else {
            duration.saturating_sub(jitter)
        }
    }
}

/// The implementation of the `moka::Expiry` trait for `JitteredExpiry`.
/// https://docs.rs/moka/latest/moka/policy/trait.Expiry.html
impl<K, V, J> Expiry<K, V> for JitteredExpiry<J>
where
    J: Distribution<Duration>,
{
    /// Specifies that the entry should be automatically removed from the cache
    /// once the duration has elapsed after the entry’s creation. This method is
    /// called for cache write methods such as `insert` and `get_with` but only
    /// when the key was not present in the cache.
    fn expire_after_create(&self, _key: &K, _value: &V, _created_at: Instant) -> Option<Duration> {
        dbg!(self.calc_expiry_for_write())
    }

    /// Specifies that the entry should be automatically removed from the cache
    /// once the duration has elapsed after the replacement of its value. This
    /// method is called for cache write methods such as `insert` but only when
    /// the key is already present in the cache.
    fn expire_after_update(
        &self,
        _key: &K,
        _value: &V,
        _updated_at: Instant,
        duration_until_expiry: Option<Duration>,
    ) -> Option<Duration> {
        dbg!(self.calc_expiry_for_write().or(duration_until_expiry))
    }

    /// Specifies that the entry should be automatically removed from the cache
    /// once the duration has elapsed after its last read. This method is called
    /// for cache read methods such as `get` and `get_with` but only when the
    /// key is present in the cache.
    fn expire_after_read(
        &self,
        _key: &K,
        _value: &V,
        read_at: Instant,
        duration_until_expiry: Option<Duration>,
        last_modified_at: Instant,
    ) -> Option<Duration> {
        dbg!(self
            .calc_expiry_for_read(read_at, last_modified_at)
            .or(duration_until_expiry))
    }
}

fn main() {
    let expiry = JitteredExpiry::new(
        // TTL 10 minutes
        Some(Duration::from_secs(10 * 60)),
        // TTI 3 minutes
        Some(Duration::from_secs(3 * 60)),
        // Jitter +/- 30 seconds, 1 second resolution, uniformly distributed
        Uniform::from(0..30).map(Duration::from_secs),
    );

    let cache = Cache::builder().expire_after(expiry).build();

    const NUM_KEYS: usize = 10;

    // Insert some key-value pairs.
    for key in 0..NUM_KEYS {
        cache.insert(key, format!("value-{key}"));
    }

    // Get all entries.
    for key in 0..NUM_KEYS {
        assert_eq!(cache.get(&key), Some(format!("value-{key}")));
    }

    // Update all entries.
    for key in 0..NUM_KEYS {
        cache.insert(key, format!("new-value-{key}"));
    }
}
