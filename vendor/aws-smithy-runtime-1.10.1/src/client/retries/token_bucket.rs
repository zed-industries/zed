/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_async::time::TimeSource;
use aws_smithy_types::config_bag::{Storable, StoreReplace};
use aws_smithy_types::retry::ErrorKind;
use std::fmt;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

const DEFAULT_CAPACITY: usize = 500;
// On a 32 bit architecture, the value of Semaphore::MAX_PERMITS is 536,870,911.
// Therefore, we will enforce a value lower than that to ensure behavior is
// identical across platforms.
// This also allows room for slight bucket overfill in the case where a bucket
// is at maximum capacity and another thread drops a permit it was holding.
/// The maximum number of permits a token bucket can have.
pub const MAXIMUM_CAPACITY: usize = 500_000_000;
const DEFAULT_RETRY_COST: u32 = 5;
const DEFAULT_RETRY_TIMEOUT_COST: u32 = DEFAULT_RETRY_COST * 2;
const PERMIT_REGENERATION_AMOUNT: usize = 1;
const DEFAULT_SUCCESS_REWARD: f32 = 0.0;

/// Token bucket used for standard and adaptive retry.
#[derive(Clone, Debug)]
pub struct TokenBucket {
    semaphore: Arc<Semaphore>,
    max_permits: usize,
    timeout_retry_cost: u32,
    retry_cost: u32,
    success_reward: f32,
    fractional_tokens: Arc<AtomicF32>,
    refill_rate: f32,
    // Note this value is only an AtomicU32 so it works on 32bit powerpc architectures.
    // If we ever remove the need for that compatibility it should become an AtomicU64
    last_refill_time_secs: Arc<AtomicU32>,
}

impl std::panic::UnwindSafe for AtomicF32 {}
impl std::panic::RefUnwindSafe for AtomicF32 {}
struct AtomicF32 {
    storage: AtomicU32,
}
impl AtomicF32 {
    fn new(value: f32) -> Self {
        let as_u32 = value.to_bits();
        Self {
            storage: AtomicU32::new(as_u32),
        }
    }
    fn store(&self, value: f32) {
        let as_u32 = value.to_bits();
        self.storage.store(as_u32, Ordering::Relaxed)
    }
    fn load(&self) -> f32 {
        let as_u32 = self.storage.load(Ordering::Relaxed);
        f32::from_bits(as_u32)
    }
}

impl fmt::Debug for AtomicF32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use debug_struct, debug_tuple, or write! for formatting
        f.debug_struct("AtomicF32")
            .field("value", &self.load())
            .finish()
    }
}

impl Clone for AtomicF32 {
    fn clone(&self) -> Self {
        // Manually clone each field
        AtomicF32 {
            storage: AtomicU32::new(self.storage.load(Ordering::Relaxed)),
        }
    }
}

impl Storable for TokenBucket {
    type Storer = StoreReplace<Self>;
}

impl Default for TokenBucket {
    fn default() -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(DEFAULT_CAPACITY)),
            max_permits: DEFAULT_CAPACITY,
            timeout_retry_cost: DEFAULT_RETRY_TIMEOUT_COST,
            retry_cost: DEFAULT_RETRY_COST,
            success_reward: DEFAULT_SUCCESS_REWARD,
            fractional_tokens: Arc::new(AtomicF32::new(0.0)),
            refill_rate: 0.0,
            last_refill_time_secs: Arc::new(AtomicU32::new(0)),
        }
    }
}

impl TokenBucket {
    /// Creates a new `TokenBucket` with the given initial quota.
    pub fn new(initial_quota: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(initial_quota)),
            max_permits: initial_quota,
            ..Default::default()
        }
    }

    /// A token bucket with unlimited capacity that allows retries at no cost.
    pub fn unlimited() -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(MAXIMUM_CAPACITY)),
            max_permits: MAXIMUM_CAPACITY,
            timeout_retry_cost: 0,
            retry_cost: 0,
            success_reward: 0.0,
            fractional_tokens: Arc::new(AtomicF32::new(0.0)),
            refill_rate: 0.0,
            last_refill_time_secs: Arc::new(AtomicU32::new(0)),
        }
    }

    /// Creates a builder for constructing a `TokenBucket`.
    pub fn builder() -> TokenBucketBuilder {
        TokenBucketBuilder::default()
    }

    pub(crate) fn acquire(
        &self,
        err: &ErrorKind,
        time_source: &impl TimeSource,
    ) -> Option<OwnedSemaphorePermit> {
        // Add time-based tokens to fractional accumulator
        self.refill_tokens_based_on_time(time_source);
        // Convert accumulated fractional tokens to whole tokens
        self.convert_fractional_tokens();

        let retry_cost = if err == &ErrorKind::TransientError {
            self.timeout_retry_cost
        } else {
            self.retry_cost
        };

        self.semaphore
            .clone()
            .try_acquire_many_owned(retry_cost)
            .ok()
    }

    pub(crate) fn success_reward(&self) -> f32 {
        self.success_reward
    }

    pub(crate) fn regenerate_a_token(&self) {
        self.add_permits(PERMIT_REGENERATION_AMOUNT);
    }

    /// Converts accumulated fractional tokens to whole tokens and adds them as permits.
    /// Stores the remaining fractional amount back.
    /// This is shared by both time-based refill and success rewards.
    #[inline]
    fn convert_fractional_tokens(&self) {
        let mut calc_fractional_tokens = self.fractional_tokens.load();
        // Verify that fractional tokens have not become corrupted - if they have, reset to zero
        if !calc_fractional_tokens.is_finite() {
            tracing::error!(
                "Fractional tokens corrupted to: {}, resetting to 0.0",
                calc_fractional_tokens
            );
            self.fractional_tokens.store(0.0);
            return;
        }

        let full_tokens_accumulated = calc_fractional_tokens.floor();
        if full_tokens_accumulated >= 1.0 {
            self.add_permits(full_tokens_accumulated as usize);
            calc_fractional_tokens -= full_tokens_accumulated;
        }
        // Always store the updated fractional tokens back, even if no conversion happened
        self.fractional_tokens.store(calc_fractional_tokens);
    }

    /// Refills tokens based on elapsed time since last refill.
    /// This method implements lazy evaluation - tokens are only calculated when accessed.
    /// Uses a single compare-and-swap to ensure only one thread processes each time window.
    #[inline]
    fn refill_tokens_based_on_time(&self, time_source: &impl TimeSource) {
        if self.refill_rate > 0.0 {
            // The cast to u32 here is safe until 2106, and I will be long dead then so ¯\_(ツ)_/¯
            let current_time_secs = time_source
                .now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::ZERO)
                .as_secs() as u32;

            let last_refill_secs = self.last_refill_time_secs.load(Ordering::Relaxed);

            // Early exit if no time elapsed - most threads take this path
            if current_time_secs == last_refill_secs {
                return;
            }

            // Try to atomically claim this time window with a single CAS
            // If we lose, another thread is handling the refill, so we can exit
            if self
                .last_refill_time_secs
                .compare_exchange(
                    last_refill_secs,
                    current_time_secs,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_err()
            {
                // Another thread claimed this time window, we're done
                return;
            }

            // We won the CAS - we're responsible for adding tokens for this time window
            let current_fractional = self.fractional_tokens.load();
            let max_fractional = self.max_permits as f32;

            // Skip token addition if already at cap
            if current_fractional >= max_fractional {
                return;
            }

            let elapsed_secs = current_time_secs.saturating_sub(last_refill_secs);
            let tokens_to_add = elapsed_secs as f32 * self.refill_rate;

            // Add tokens to fractional accumulator, capping at max_permits to prevent unbounded growth
            let new_fractional = (current_fractional + tokens_to_add).min(max_fractional);
            self.fractional_tokens.store(new_fractional);
        }
    }

    #[inline]
    pub(crate) fn reward_success(&self) {
        if self.success_reward > 0.0 {
            let current = self.fractional_tokens.load();
            let max_fractional = self.max_permits as f32;
            // Early exit if already at cap - no point calculating
            if current >= max_fractional {
                return;
            }
            // Cap fractional tokens at max_permits to prevent unbounded growth
            let new_fractional = (current + self.success_reward).min(max_fractional);
            self.fractional_tokens.store(new_fractional);
        }
    }

    pub(crate) fn add_permits(&self, amount: usize) {
        let available = self.semaphore.available_permits();
        if available >= self.max_permits {
            return;
        }
        self.semaphore
            .add_permits(amount.min(self.max_permits - available));
    }

    /// Returns true if the token bucket is full, false otherwise
    pub fn is_full(&self) -> bool {
        self.semaphore.available_permits() >= self.max_permits
    }

    /// Returns true if the token bucket is empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.semaphore.available_permits() == 0
    }

    #[allow(dead_code)] // only used in tests
    #[cfg(any(test, feature = "test-util", feature = "legacy-test-util"))]
    pub(crate) fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    /// Only used in tests
    #[allow(dead_code)]
    #[doc(hidden)]
    #[cfg(any(test, feature = "test-util", feature = "legacy-test-util"))]
    pub fn last_refill_time_secs(&self) -> Arc<AtomicU32> {
        self.last_refill_time_secs.clone()
    }
}

/// Builder for constructing a `TokenBucket`.
#[derive(Clone, Debug, Default)]
pub struct TokenBucketBuilder {
    capacity: Option<usize>,
    retry_cost: Option<u32>,
    timeout_retry_cost: Option<u32>,
    success_reward: Option<f32>,
    refill_rate: Option<f32>,
}

impl TokenBucketBuilder {
    /// Creates a new `TokenBucketBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum bucket capacity for the builder.
    pub fn capacity(mut self, mut capacity: usize) -> Self {
        if capacity > MAXIMUM_CAPACITY {
            capacity = MAXIMUM_CAPACITY;
        }
        self.capacity = Some(capacity);
        self
    }

    /// Sets the specified retry cost for the builder.
    pub fn retry_cost(mut self, retry_cost: u32) -> Self {
        self.retry_cost = Some(retry_cost);
        self
    }

    /// Sets the specified timeout retry cost for the builder.
    pub fn timeout_retry_cost(mut self, timeout_retry_cost: u32) -> Self {
        self.timeout_retry_cost = Some(timeout_retry_cost);
        self
    }

    /// Sets the reward for any successful request for the builder.
    pub fn success_reward(mut self, reward: f32) -> Self {
        self.success_reward = Some(reward);
        self
    }

    /// Sets the refill rate (tokens per second) for time-based token regeneration.
    ///
    /// Negative values are clamped to 0.0. A refill rate of 0.0 disables time-based regeneration.
    /// Non-finite values (NaN, infinity) are treated as 0.0.
    pub fn refill_rate(mut self, rate: f32) -> Self {
        let validated_rate = if rate.is_finite() { rate.max(0.0) } else { 0.0 };
        self.refill_rate = Some(validated_rate);
        self
    }

    /// Builds a `TokenBucket`.
    pub fn build(self) -> TokenBucket {
        TokenBucket {
            semaphore: Arc::new(Semaphore::new(self.capacity.unwrap_or(DEFAULT_CAPACITY))),
            max_permits: self.capacity.unwrap_or(DEFAULT_CAPACITY),
            retry_cost: self.retry_cost.unwrap_or(DEFAULT_RETRY_COST),
            timeout_retry_cost: self
                .timeout_retry_cost
                .unwrap_or(DEFAULT_RETRY_TIMEOUT_COST),
            success_reward: self.success_reward.unwrap_or(DEFAULT_SUCCESS_REWARD),
            fractional_tokens: Arc::new(AtomicF32::new(0.0)),
            refill_rate: self.refill_rate.unwrap_or(0.0),
            last_refill_time_secs: Arc::new(AtomicU32::new(0)),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use aws_smithy_async::test_util::ManualTimeSource;
    use std::{sync::LazyLock, time::UNIX_EPOCH};

    static TIME_SOURCE: LazyLock<ManualTimeSource> =
        LazyLock::new(|| ManualTimeSource::new(UNIX_EPOCH + Duration::from_secs(12344321)));

    #[test]
    fn test_unlimited_token_bucket() {
        let bucket = TokenBucket::unlimited();

        // Should always acquire permits regardless of error type
        assert!(bucket
            .acquire(&ErrorKind::ThrottlingError, &*TIME_SOURCE)
            .is_some());
        assert!(bucket
            .acquire(&ErrorKind::TransientError, &*TIME_SOURCE)
            .is_some());

        // Should have maximum capacity
        assert_eq!(bucket.max_permits, MAXIMUM_CAPACITY);

        // Should have zero retry costs
        assert_eq!(bucket.retry_cost, 0);
        assert_eq!(bucket.timeout_retry_cost, 0);

        // The loop count is arbitrary; should obtain permits without limit
        let mut permits = Vec::new();
        for _ in 0..100 {
            let permit = bucket.acquire(&ErrorKind::ThrottlingError, &*TIME_SOURCE);
            assert!(permit.is_some());
            permits.push(permit);
            // Available permits should stay constant
            assert_eq!(MAXIMUM_CAPACITY, bucket.semaphore.available_permits());
        }
    }

    #[test]
    fn test_bounded_permits_exhaustion() {
        let bucket = TokenBucket::new(10);
        let mut permits = Vec::new();

        for _ in 0..100 {
            let permit = bucket.acquire(&ErrorKind::ThrottlingError, &*TIME_SOURCE);
            if let Some(p) = permit {
                permits.push(p);
            } else {
                break;
            }
        }

        assert_eq!(permits.len(), 2); // 10 capacity / 5 retry cost = 2 permits

        // Verify next acquisition fails
        assert!(bucket
            .acquire(&ErrorKind::ThrottlingError, &*TIME_SOURCE)
            .is_none());
    }

    #[test]
    fn test_fractional_tokens_accumulate_and_convert() {
        let bucket = TokenBucket::builder()
            .capacity(10)
            .success_reward(0.4)
            .build();

        // acquire 10 tokens to bring capacity below max so we can test accumulation
        let _hold_permit = bucket.acquire(&ErrorKind::TransientError, &*TIME_SOURCE);
        assert_eq!(bucket.semaphore.available_permits(), 0);

        // First success: 0.4 fractional tokens
        bucket.reward_success();
        bucket.convert_fractional_tokens();
        assert_eq!(bucket.semaphore.available_permits(), 0);

        // Second success: 0.8 fractional tokens
        bucket.reward_success();
        bucket.convert_fractional_tokens();
        assert_eq!(bucket.semaphore.available_permits(), 0);

        // Third success: 1.2 fractional tokens -> 1 full token added
        bucket.reward_success();
        bucket.convert_fractional_tokens();
        assert_eq!(bucket.semaphore.available_permits(), 1);
    }

    #[test]
    fn test_fractional_tokens_respect_max_capacity() {
        let bucket = TokenBucket::builder()
            .capacity(10)
            .success_reward(2.0)
            .build();

        for _ in 0..20 {
            bucket.reward_success();
        }

        assert!(bucket.semaphore.available_permits() == 10);
    }

    #[test]
    fn test_convert_fractional_tokens() {
        // (input, expected_permits_added, expected_remaining)
        let test_cases = [
            (0.7, 0, 0.7),
            (1.0, 1, 0.0),
            (2.3, 2, 0.3),
            (5.8, 5, 0.8),
            (10.0, 10, 0.0),
            // verify that if fractional permits are corrupted, we reset to 0 gracefully
            (f32::NAN, 0, 0.0),
            (f32::INFINITY, 0, 0.0),
        ];

        for (input, expected_permits, expected_remaining) in test_cases {
            let bucket = TokenBucket::builder().capacity(10).build();
            let _hold_permit = bucket.acquire(&ErrorKind::TransientError, &*TIME_SOURCE);
            let initial = bucket.semaphore.available_permits();

            bucket.fractional_tokens.store(input);
            bucket.convert_fractional_tokens();

            assert_eq!(
                bucket.semaphore.available_permits() - initial,
                expected_permits
            );
            assert!((bucket.fractional_tokens.load() - expected_remaining).abs() < 0.0001);
        }
    }

    #[cfg(any(feature = "test-util", feature = "legacy-test-util"))]
    #[test]
    fn test_builder_with_custom_values() {
        let bucket = TokenBucket::builder()
            .capacity(100)
            .retry_cost(10)
            .timeout_retry_cost(20)
            .success_reward(0.5)
            .refill_rate(2.5)
            .build();

        assert_eq!(bucket.max_permits, 100);
        assert_eq!(bucket.retry_cost, 10);
        assert_eq!(bucket.timeout_retry_cost, 20);
        assert_eq!(bucket.success_reward, 0.5);
        assert_eq!(bucket.refill_rate, 2.5);
    }

    #[test]
    fn test_builder_refill_rate_validation() {
        // Test negative values are clamped to 0.0
        let bucket = TokenBucket::builder().refill_rate(-5.0).build();
        assert_eq!(bucket.refill_rate, 0.0);

        // Test valid positive value
        let bucket = TokenBucket::builder().refill_rate(1.5).build();
        assert_eq!(bucket.refill_rate, 1.5);

        // Test zero is valid
        let bucket = TokenBucket::builder().refill_rate(0.0).build();
        assert_eq!(bucket.refill_rate, 0.0);
    }

    #[cfg(any(feature = "test-util", feature = "legacy-test-util"))]
    #[test]
    fn test_builder_custom_time_source() {
        use aws_smithy_async::test_util::ManualTimeSource;
        use std::time::UNIX_EPOCH;

        // Test that TokenBucket uses provided TimeSource when specified via builder
        let manual_time = ManualTimeSource::new(UNIX_EPOCH);
        let bucket = TokenBucket::builder()
            .capacity(100)
            .refill_rate(1.0)
            .build();

        // Consume all tokens to test refill from empty state
        let _permits = bucket.semaphore.try_acquire_many(100).unwrap();
        assert_eq!(bucket.available_permits(), 0);

        // Advance time and verify tokens are added based on manual time
        manual_time.advance(Duration::from_secs(5));

        bucket.refill_tokens_based_on_time(&manual_time);
        bucket.convert_fractional_tokens();

        // Should have 5 tokens (5 seconds * 1 token/sec)
        assert_eq!(bucket.available_permits(), 5);
    }

    #[test]
    fn test_atomicf32_f32_to_bits_conversion_correctness() {
        // This is the core functionality
        let test_values = vec![
            0.0,
            -0.0,
            1.0,
            -1.0,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
            f32::MIN,
            f32::MAX,
            f32::MIN_POSITIVE,
            f32::EPSILON,
            std::f32::consts::PI,
            std::f32::consts::E,
            // Test values that could expose bit manipulation bugs
            1.23456789e-38, // Very small normal number
            1.23456789e38,  // Very large number (within f32 range)
            1.1754944e-38,  // Near MIN_POSITIVE for f32
        ];

        for &expected in &test_values {
            let atomic = AtomicF32::new(expected);
            let actual = atomic.load();

            // For NaN, we can't use == but must check bit patterns
            if expected.is_nan() {
                assert!(actual.is_nan(), "Expected NaN, got {}", actual);
                // Different NaN bit patterns should be preserved exactly
                assert_eq!(expected.to_bits(), actual.to_bits());
            } else {
                assert_eq!(expected.to_bits(), actual.to_bits());
            }
        }
    }

    #[cfg(any(feature = "test-util", feature = "legacy-test-util"))]
    #[test]
    fn test_atomicf32_store_load_preserves_exact_bits() {
        let atomic = AtomicF32::new(0.0);

        // Test that store/load cycle preserves EXACT bit patterns
        // This would catch bugs in the to_bits/from_bits conversion
        let critical_bit_patterns = vec![
            0x00000000u32, // +0.0
            0x80000000u32, // -0.0
            0x7F800000u32, // +infinity
            0xFF800000u32, // -infinity
            0x7FC00000u32, // Quiet NaN
            0x7FA00000u32, // Signaling NaN
            0x00000001u32, // Smallest positive subnormal
            0x007FFFFFu32, // Largest subnormal
            0x00800000u32, // Smallest positive normal (MIN_POSITIVE)
        ];

        for &expected_bits in &critical_bit_patterns {
            let expected_f32 = f32::from_bits(expected_bits);
            atomic.store(expected_f32);
            let loaded_f32 = atomic.load();
            let actual_bits = loaded_f32.to_bits();

            assert_eq!(expected_bits, actual_bits);
        }
    }

    #[cfg(any(feature = "test-util", feature = "legacy-test-util"))]
    #[test]
    fn test_atomicf32_concurrent_store_load_safety() {
        use std::sync::Arc;
        use std::thread;

        let atomic = Arc::new(AtomicF32::new(0.0));
        let test_values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mut handles = Vec::new();

        // Start multiple threads that continuously write different values
        for &value in &test_values {
            let atomic_clone = Arc::clone(&atomic);
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    atomic_clone.store(value);
                }
            });
            handles.push(handle);
        }

        // Start a reader thread that continuously reads
        let atomic_reader = Arc::clone(&atomic);
        let reader_handle = thread::spawn(move || {
            let mut readings = Vec::new();
            for _ in 0..5000 {
                let value = atomic_reader.load();
                readings.push(value);
            }
            readings
        });

        // Wait for all writers to complete
        for handle in handles {
            handle.join().expect("Writer thread panicked");
        }

        let readings = reader_handle.join().expect("Reader thread panicked");

        // Verify that all read values are valid (one of the written values)
        // This tests that there's no data corruption from concurrent access
        for &reading in &readings {
            assert!(test_values.contains(&reading) || reading == 0.0);

            // More importantly, verify the reading is a valid f32
            // (not corrupted bits that happen to parse as valid)
            assert!(
                reading.is_finite() || reading == 0.0,
                "Corrupted reading detected"
            );
        }
    }

    #[cfg(any(feature = "test-util", feature = "legacy-test-util"))]
    #[test]
    fn test_atomicf32_stress_concurrent_access() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let expected_values = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let atomic = Arc::new(AtomicF32::new(0.0));
        let barrier = Arc::new(Barrier::new(10)); // Synchronize all threads
        let mut handles = Vec::new();

        // Launch threads that all start simultaneously
        for i in 0..10 {
            let atomic_clone = Arc::clone(&atomic);
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier_clone.wait(); // All threads start at same time

                // Tight loop increases chance of race conditions
                for _ in 0..10000 {
                    let value = i as f32;
                    atomic_clone.store(value);
                    let loaded = atomic_clone.load();
                    // Verify no corruption occurred
                    assert!(loaded >= 0.0 && loaded <= 9.0);
                    assert!(
                        expected_values.contains(&loaded),
                        "Got unexpected value: {}, expected one of {:?}",
                        loaded,
                        expected_values
                    );
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_atomicf32_integration_with_token_bucket_usage() {
        let atomic = AtomicF32::new(0.0);
        let success_reward = 0.3;
        let iterations = 5;

        // Accumulate fractional tokens
        for _ in 1..=iterations {
            let current = atomic.load();
            atomic.store(current + success_reward);
        }

        let accumulated = atomic.load();
        let expected_total = iterations as f32 * success_reward; // 1.5

        // Test the floor() operation pattern
        let full_tokens = accumulated.floor();
        atomic.store(accumulated - full_tokens);
        let remaining = atomic.load();

        // These assertions should be general:
        assert_eq!(full_tokens, expected_total.floor()); // Could be 1.0, 2.0, 3.0, etc.
        assert!(remaining >= 0.0 && remaining < 1.0);
        assert_eq!(remaining, expected_total - expected_total.floor());
    }

    #[cfg(any(feature = "test-util", feature = "legacy-test-util"))]
    #[test]
    fn test_atomicf32_clone_creates_independent_copy() {
        let original = AtomicF32::new(123.456);
        let cloned = original.clone();

        // Verify they start with the same value
        assert_eq!(original.load(), cloned.load());

        // Verify they're independent - modifying one doesn't affect the other
        original.store(999.0);
        assert_eq!(
            cloned.load(),
            123.456,
            "Clone should be unaffected by original changes"
        );
        assert_eq!(original.load(), 999.0, "Original should have new value");
    }

    #[test]
    fn test_combined_time_and_success_rewards() {
        use aws_smithy_async::test_util::ManualTimeSource;
        use std::time::UNIX_EPOCH;

        let time_source = ManualTimeSource::new(UNIX_EPOCH);
        let current_time_secs = UNIX_EPOCH
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        let bucket = TokenBucket {
            refill_rate: 1.0,
            success_reward: 0.5,
            last_refill_time_secs: Arc::new(AtomicU32::new(current_time_secs)),
            semaphore: Arc::new(Semaphore::new(0)),
            max_permits: 100,
            ..Default::default()
        };

        // Add success rewards: 2 * 0.5 = 1.0 token
        bucket.reward_success();
        bucket.reward_success();

        // Advance time by 2 seconds
        time_source.advance(Duration::from_secs(2));

        // Trigger time-based refill: 2 sec * 1.0 = 2.0 tokens
        // Total: 1.0 + 2.0 = 3.0 tokens
        bucket.refill_tokens_based_on_time(&time_source);
        bucket.convert_fractional_tokens();

        assert_eq!(bucket.available_permits(), 3);
        assert!(bucket.fractional_tokens.load().abs() < 0.0001);
    }

    #[test]
    fn test_refill_rates() {
        use aws_smithy_async::test_util::ManualTimeSource;
        use std::time::UNIX_EPOCH;
        // (refill_rate, elapsed_secs, expected_permits, expected_fractional)
        let test_cases = [
            (10.0, 2, 20, 0.0),      // Basic: 2 sec * 10 tokens/sec = 20 tokens
            (0.001, 1100, 1, 0.1),   // Small: 1100 * 0.001 = 1.1 tokens
            (0.0001, 11000, 1, 0.1), // Tiny: 11000 * 0.0001 = 1.1 tokens
            (0.001, 1200, 1, 0.2),   // 1200 * 0.001 = 1.2 tokens
            (0.0001, 10000, 1, 0.0), // 10000 * 0.0001 = 1.0 tokens
            (0.001, 500, 0, 0.5),    // Fractional only: 500 * 0.001 = 0.5 tokens
        ];

        for (refill_rate, elapsed_secs, expected_permits, expected_fractional) in test_cases {
            let time_source = ManualTimeSource::new(UNIX_EPOCH);
            let current_time_secs = UNIX_EPOCH
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;

            let bucket = TokenBucket {
                refill_rate,
                last_refill_time_secs: Arc::new(AtomicU32::new(current_time_secs)),
                semaphore: Arc::new(Semaphore::new(0)),
                max_permits: 100,
                ..Default::default()
            };

            // Advance time by the specified duration
            time_source.advance(Duration::from_secs(elapsed_secs));

            bucket.refill_tokens_based_on_time(&time_source);
            bucket.convert_fractional_tokens();

            assert_eq!(
                bucket.available_permits(),
                expected_permits,
                "Rate {}: After {}s expected {} permits",
                refill_rate,
                elapsed_secs,
                expected_permits
            );
            assert!(
                (bucket.fractional_tokens.load() - expected_fractional).abs() < 0.0001,
                "Rate {}: After {}s expected {} fractional, got {}",
                refill_rate,
                elapsed_secs,
                expected_fractional,
                bucket.fractional_tokens.load()
            );
        }
    }

    #[cfg(any(feature = "test-util", feature = "legacy-test-util"))]
    #[test]
    fn test_rewards_capped_at_max_capacity() {
        use aws_smithy_async::test_util::ManualTimeSource;
        use std::time::UNIX_EPOCH;

        let time_source = ManualTimeSource::new(UNIX_EPOCH);
        let current_time_secs = UNIX_EPOCH
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        let bucket = TokenBucket {
            refill_rate: 50.0,
            success_reward: 2.0,
            last_refill_time_secs: Arc::new(AtomicU32::new(current_time_secs)),
            semaphore: Arc::new(Semaphore::new(5)),
            max_permits: 10,
            ..Default::default()
        };

        // Add success rewards: 50 * 2.0 = 100 tokens (without cap)
        for _ in 0..50 {
            bucket.reward_success();
        }

        // Fractional tokens capped at 10 from success rewards
        assert_eq!(bucket.fractional_tokens.load(), 10.0);

        // Advance time by 100 seconds
        time_source.advance(Duration::from_secs(100));

        // Time-based refill: 100 * 50 = 5000 tokens (without cap)
        // But fractional is already at 10, so it stays at 10
        bucket.refill_tokens_based_on_time(&time_source);

        // Fractional tokens should be capped at max_permits (10)
        assert_eq!(
            bucket.fractional_tokens.load(),
            10.0,
            "Fractional tokens should be capped at max_permits"
        );
        // Convert should add 5 tokens (bucket at 5, can add 5 more to reach max 10)
        bucket.convert_fractional_tokens();
        assert_eq!(bucket.available_permits(), 10);
    }

    #[cfg(any(feature = "test-util", feature = "legacy-test-util"))]
    #[test]
    fn test_concurrent_time_based_refill_no_over_generation() {
        use aws_smithy_async::test_util::ManualTimeSource;
        use std::sync::{Arc, Barrier};
        use std::thread;
        use std::time::UNIX_EPOCH;

        let time_source = ManualTimeSource::new(UNIX_EPOCH);
        let current_time_secs = UNIX_EPOCH
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        // Create bucket with 1 token/sec refill
        let bucket = Arc::new(TokenBucket {
            refill_rate: 1.0,
            last_refill_time_secs: Arc::new(AtomicU32::new(current_time_secs)),
            semaphore: Arc::new(Semaphore::new(0)),
            max_permits: 100,
            ..Default::default()
        });

        // Advance time by 10 seconds
        time_source.advance(Duration::from_secs(10));
        let shared_time_source = aws_smithy_async::time::SharedTimeSource::new(time_source);

        // Launch 100 threads that all try to refill simultaneously
        let barrier = Arc::new(Barrier::new(100));
        let mut handles = Vec::new();

        for _ in 0..100 {
            let bucket_clone1 = Arc::clone(&bucket);
            let barrier_clone1 = Arc::clone(&barrier);
            let time_source_clone1 = shared_time_source.clone();
            let bucket_clone2 = Arc::clone(&bucket);
            let barrier_clone2 = Arc::clone(&barrier);
            let time_source_clone2 = shared_time_source.clone();

            let handle1 = thread::spawn(move || {
                // Wait for all threads to be ready
                barrier_clone1.wait();

                // All threads call refill at the same time
                bucket_clone1.refill_tokens_based_on_time(&time_source_clone1);
            });

            let handle2 = thread::spawn(move || {
                // Wait for all threads to be ready
                barrier_clone2.wait();

                // All threads call refill at the same time
                bucket_clone2.refill_tokens_based_on_time(&time_source_clone2);
            });
            handles.push(handle1);
            handles.push(handle2);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Convert fractional tokens to whole tokens
        bucket.convert_fractional_tokens();

        // Should have exactly 10 tokens (10 seconds * 1 token/sec)
        // Not 1000 tokens (100 threads * 10 tokens each)
        assert_eq!(
            bucket.available_permits(),
            10,
            "Only one thread should have added tokens, not all 100"
        );

        // Fractional should be 0 after conversion
        assert!(bucket.fractional_tokens.load().abs() < 0.0001);
    }
}
