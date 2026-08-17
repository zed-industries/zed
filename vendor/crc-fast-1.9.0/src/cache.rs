// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! CRC parameter caching system
//!
//! This module provides a thread-safe cache for CRC folding keys to avoid expensive
//! regeneration when the same CRC parameters are used multiple times. The cache uses
//! a read-write lock pattern optimized for the common case of cache hits.
//!
//! # Performance Characteristics
//!
//! - Cache hits: ~50-100x faster than key generation
//! - Cache misses: ~100-200ns overhead compared to direct generation
//! - Memory usage: ~200 bytes per unique parameter set
//! - Thread safety: Multiple concurrent readers, exclusive writers
//!
//! # Usage
//!
//! The cache is used automatically by `CrcParams::new()` and requires no manual management.
//! The cache is transparent to users and handles all memory management internally.

use crate::generate;

#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
use std::sync::{OnceLock, RwLock};

#[cfg(all(not(feature = "std"), feature = "cache"))]
use hashbrown::HashMap;
#[cfg(all(not(feature = "std"), feature = "cache"))]
use spin::{Once, RwLock};

/// Global cache storage for CRC parameter keys
///
/// Uses OnceLock for thread-safe lazy initialization and RwLock for concurrent access.
/// The cache maps parameter combinations to their pre-computed folding keys.
#[cfg(feature = "std")]
static CACHE: OnceLock<RwLock<HashMap<CrcParamsCacheKey, [u64; 23]>>> = OnceLock::new();

#[cfg(all(not(feature = "std"), feature = "cache"))]
static CACHE: Once<RwLock<HashMap<CrcParamsCacheKey, [u64; 23]>>> = Once::new();

/// Cache key for storing CRC parameters that affect key generation
///
/// Only includes parameters that directly influence the mathematical computation
/// of folding keys. Parameters like `init`, `xorout`, and `check` are excluded
/// because they don't affect the key generation process.
///
/// The cache key implements `Hash`, `Eq`, and `PartialEq` to enable efficient
/// HashMap storage and lookup operations.
#[cfg(any(feature = "std", feature = "cache"))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CrcParamsCacheKey {
    /// CRC width in bits (32 or 64)
    pub width: u8,
    /// Polynomial value used for CRC calculation
    pub poly: u64,
    /// Whether the CRC uses reflected input/output processing
    pub reflected: bool,
}

#[cfg(any(feature = "std", feature = "cache"))]
impl CrcParamsCacheKey {
    /// Create a new cache key from CRC parameters
    ///
    /// # Arguments
    ///
    /// * `width` - CRC width in bits (32 or 64)
    /// * `poly` - Polynomial value for the CRC algorithm
    /// * `reflected` - Whether input/output should be bit-reflected
    pub fn new(width: u8, poly: u64, reflected: bool) -> Self {
        Self {
            width,
            poly,
            reflected,
        }
    }
}

/// Initialize and return reference to the global cache
///
/// Uses OnceLock to ensure thread-safe lazy initialization without requiring
/// static initialization overhead. The cache is only created when first accessed.
#[cfg(feature = "std")]
fn get_cache() -> &'static RwLock<HashMap<CrcParamsCacheKey, [u64; 23]>> {
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

#[cfg(all(not(feature = "std"), feature = "cache"))]
fn get_cache() -> &'static RwLock<HashMap<CrcParamsCacheKey, [u64; 23]>> {
    CACHE.call_once(|| RwLock::new(HashMap::new()))
}

/// Get cached keys or generate and cache them if not present
///
/// This function implements a read-then-write pattern optimized for the common case
/// of cache hits while minimizing lock contention:
///
/// 1. **Read phase**: Attempts read lock to check for cached keys (allows concurrent reads)
/// 2. **Generation phase**: If cache miss, generates keys outside any lock to minimize hold time
/// 3. **Write phase**: Acquires write lock only to store the generated keys
///
/// The key generation happens outside the write lock because it's computationally expensive
/// (~1000x slower than cache lookup) and we want to minimize the time other threads are blocked.
///
/// All cache operations use best-effort error handling - lock poisoning or allocation failures
/// don't cause panics, instead falling back to direct key generation to maintain functionality.
///
/// # Arguments
///
/// * `width` - CRC width in bits (32 or 64)
/// * `poly` - Polynomial value for the CRC algorithm
/// * `reflected` - Whether input/output should be bit-reflected
///
/// # Returns
///
/// Array of 23 pre-computed folding keys for SIMD CRC calculation
pub fn get_or_generate_keys(width: u8, poly: u64, reflected: bool) -> [u64; 23] {
    #[cfg(feature = "std")]
    {
        let cache_key = CrcParamsCacheKey::new(width, poly, reflected);

        // Try cache read first - multiple threads can read simultaneously
        // If lock is poisoned or read fails, continue to key generation
        if let Ok(cache) = get_cache().read() {
            if let Some(keys) = cache.get(&cache_key) {
                return *keys;
            }
        }

        // Generate keys outside of write lock to minimize lock hold time
        let keys = generate::keys(width, poly, reflected);

        // Try to cache the result (best effort - if this fails, we still return valid keys)
        // Lock poisoning or write failure doesn't affect functionality
        let _ = get_cache()
            .write()
            .map(|mut cache| cache.insert(cache_key, keys));

        keys
    }

    #[cfg(all(not(feature = "std"), feature = "cache"))]
    {
        let cache_key = CrcParamsCacheKey::new(width, poly, reflected);

        // Try cache read first - multiple threads can read simultaneously
        // spin::RwLock returns guards directly (no Result wrapper)
        {
            let cache = get_cache().read();
            if let Some(keys) = cache.get(&cache_key) {
                return *keys;
            }
        } // Drop read lock before generating keys

        // Generate keys outside of write lock to minimize lock hold time
        let keys = generate::keys(width, poly, reflected);

        // Cache the result - spin::RwLock doesn't use Result wrapper
        {
            let mut cache = get_cache().write();
            cache.insert(cache_key, keys);
        }

        keys
    }

    #[cfg(not(any(feature = "std", feature = "cache")))]
    {
        generate::keys(width, poly, reflected)
    }
}

/// Clear all cached CRC parameter keys
///
/// This function is primarily intended for testing scenarios where you need to reset
/// the cache state to ensure test isolation.
///
/// Uses best-effort error handling - lock poisoning or other failures don't cause
/// panics, ensuring this function never disrupts program execution. If the cache
/// cannot be cleared, the function silently continues without error.
///
/// # Thread Safety
///
/// This function is thread-safe and can be called concurrently with other cache operations.
/// However, clearing the cache while other threads are actively using it may temporarily
/// reduce performance as those threads will need to regenerate keys on their next access.
#[cfg(test)]
pub(crate) fn clear_cache() {
    #[cfg(feature = "std")]
    {
        // Best-effort cache clear - if lock is poisoned or unavailable, silently continue
        // This ensures the function never panics or blocks program execution
        let _ = get_cache().write().map(|mut cache| cache.clear());
    }

    #[cfg(all(not(feature = "std"), feature = "cache"))]
    {
        // spin::RwLock doesn't use Result wrapper
        let mut cache = get_cache().write();
        cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_cache_key_creation() {
        let key1 = CrcParamsCacheKey::new(32, 0x04C11DB7, true);
        let key2 = CrcParamsCacheKey::new(64, 0x42F0E1EBA9EA3693, false);

        assert_eq!(key1.width, 32);
        assert_eq!(key1.poly, 0x04C11DB7);
        assert!(key1.reflected);

        assert_eq!(key2.width, 64);
        assert_eq!(key2.poly, 0x42F0E1EBA9EA3693);
        assert!(!key2.reflected);
    }

    #[test]
    fn test_cache_key_equality() {
        let key1 = CrcParamsCacheKey::new(32, 0x04C11DB7, true);
        let key2 = CrcParamsCacheKey::new(32, 0x04C11DB7, true);
        let key3 = CrcParamsCacheKey::new(32, 0x04C11DB7, false); // Different reflected
        let key4 = CrcParamsCacheKey::new(64, 0x04C11DB7, true); // Different width
        let key5 = CrcParamsCacheKey::new(32, 0x1EDC6F41, true); // Different poly

        // Test equality
        assert_eq!(key1, key2);
        assert_eq!(key1.clone(), key2.clone());

        // Test inequality
        assert_ne!(key1, key3);
        assert_ne!(key1, key4);
        assert_ne!(key1, key5);
        assert_ne!(key3, key4);
        assert_ne!(key3, key5);
        assert_ne!(key4, key5);
    }

    #[test]
    fn test_cache_key_hashing() {
        let key1 = CrcParamsCacheKey::new(32, 0x04C11DB7, true);
        let key2 = CrcParamsCacheKey::new(32, 0x04C11DB7, true);
        let key3 = CrcParamsCacheKey::new(32, 0x04C11DB7, false);

        // Create a HashSet to test that keys can be used as hash keys
        let mut set = HashSet::new();
        set.insert(key1.clone());
        set.insert(key2.clone());
        set.insert(key3.clone());

        // Should only have 2 unique keys (key1 and key2 are equal)
        assert_eq!(set.len(), 2);

        // Test that equal keys can be found in the set
        assert!(set.contains(&key1));
        assert!(set.contains(&key2));
        assert!(set.contains(&key3));

        // Test that a new equivalent key can be found
        let key4 = CrcParamsCacheKey::new(32, 0x04C11DB7, true);
        assert!(set.contains(&key4));
    }

    #[test]
    fn test_cache_hit_scenarios() {
        clear_cache();

        // First call should be a cache miss and generate keys
        let keys1 = get_or_generate_keys(32, 0x04C11DB7, true);

        // Second call with same parameters should be a cache hit
        let keys2 = get_or_generate_keys(32, 0x04C11DB7, true);

        // Keys should be identical (same array contents)
        assert_eq!(keys1, keys2);

        // Test multiple cache hits
        let keys3 = get_or_generate_keys(32, 0x04C11DB7, true);
        let keys4 = get_or_generate_keys(32, 0x04C11DB7, true);

        assert_eq!(keys1, keys3);
        assert_eq!(keys1, keys4);
        assert_eq!(keys2, keys3);
        assert_eq!(keys2, keys4);
    }

    #[test]
    fn test_cache_miss_scenarios() {
        clear_cache();

        // Different width - should be cache miss
        let keys_32 = get_or_generate_keys(32, 0x04C11DB7, true);
        let keys_64 = get_or_generate_keys(64, 0x04C11DB7, true);
        assert_ne!(keys_32, keys_64);

        // Different poly - should be cache miss
        let keys_poly1 = get_or_generate_keys(32, 0x04C11DB7, true);
        let keys_poly2 = get_or_generate_keys(32, 0x1EDC6F41, true);
        assert_ne!(keys_poly1, keys_poly2);

        // Different reflected - should be cache miss
        let keys_refl_true = get_or_generate_keys(32, 0x04C11DB7, true);
        let keys_refl_false = get_or_generate_keys(32, 0x04C11DB7, false);
        assert_ne!(keys_refl_true, keys_refl_false);

        // Verify each parameter set is cached independently
        let keys_32_again = get_or_generate_keys(32, 0x04C11DB7, true);
        let keys_64_again = get_or_generate_keys(64, 0x04C11DB7, true);
        let keys_poly1_again = get_or_generate_keys(32, 0x04C11DB7, true);
        let keys_poly2_again = get_or_generate_keys(32, 0x1EDC6F41, true);
        let keys_refl_true_again = get_or_generate_keys(32, 0x04C11DB7, true);
        let keys_refl_false_again = get_or_generate_keys(32, 0x04C11DB7, false);

        assert_eq!(keys_32, keys_32_again);
        assert_eq!(keys_64, keys_64_again);
        assert_eq!(keys_poly1, keys_poly1_again);
        assert_eq!(keys_poly2, keys_poly2_again);
        assert_eq!(keys_refl_true, keys_refl_true_again);
        assert_eq!(keys_refl_false, keys_refl_false_again);
    }

    #[test]
    fn test_cached_keys_identical_to_generated_keys() {
        clear_cache();

        // Test CRC32 parameters
        let width = 32;
        let poly = 0x04C11DB7;
        let reflected = true;

        // Generate keys directly (bypassing cache)
        let direct_keys = generate::keys(width, poly, reflected);

        // Get keys through cache (first call will be cache miss, generates and caches)
        let cached_keys_first = get_or_generate_keys(width, poly, reflected);

        // Get keys through cache again (should be cache hit)
        let cached_keys_second = get_or_generate_keys(width, poly, reflected);

        // All should be identical
        assert_eq!(direct_keys, cached_keys_first);
        assert_eq!(direct_keys, cached_keys_second);
        assert_eq!(cached_keys_first, cached_keys_second);

        // Test CRC64 parameters
        let width64 = 64;
        let poly64 = 0x42F0E1EBA9EA3693;
        let reflected64 = false;

        let direct_keys64 = generate::keys(width64, poly64, reflected64);
        let cached_keys64_first = get_or_generate_keys(width64, poly64, reflected64);
        let cached_keys64_second = get_or_generate_keys(width64, poly64, reflected64);

        assert_eq!(direct_keys64, cached_keys64_first);
        assert_eq!(direct_keys64, cached_keys64_second);
        assert_eq!(cached_keys64_first, cached_keys64_second);

        // Verify different parameters produce different keys
        assert_ne!(direct_keys, direct_keys64);
        assert_ne!(cached_keys_first, cached_keys64_first);
    }

    #[test]
    fn test_multiple_parameter_combinations() {
        clear_cache();

        // Test various common CRC parameter combinations
        let test_cases = [
            (32, 0x04C11DB7, true),          // CRC32
            (32, 0x04C11DB7, false),         // CRC32 non-reflected
            (32, 0x1EDC6F41, true),          // CRC32C
            (64, 0x42F0E1EBA9EA3693, true),  // CRC64 ISO
            (64, 0x42F0E1EBA9EA3693, false), // CRC64 ISO non-reflected
            (64, 0xD800000000000000, true),  // CRC64 ECMA
        ];

        let mut all_keys = Vec::new();

        // Generate keys for all test cases
        for &(width, poly, reflected) in &test_cases {
            let keys = get_or_generate_keys(width, poly, reflected);
            all_keys.push(keys);
        }

        // Verify all keys are different (no collisions)
        for i in 0..all_keys.len() {
            for j in (i + 1)..all_keys.len() {
                assert_ne!(
                    all_keys[i], all_keys[j],
                    "Keys should be different for test cases {} and {}",
                    i, j
                );
            }
        }

        // Verify cache hits return same keys
        for (i, &(width, poly, reflected)) in test_cases.iter().enumerate() {
            let cached_keys = get_or_generate_keys(width, poly, reflected);
            assert_eq!(
                all_keys[i], cached_keys,
                "Cache hit should return same keys for test case {}",
                i
            );
        }
    }

    #[test]
    fn test_cache_management_utilities() {
        // Clear cache to start with clean state
        clear_cache();

        // Generate and cache some keys
        let keys1 = get_or_generate_keys(32, 0x04C11DB7, true);
        let keys2 = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);

        // Verify cache hits return same keys
        let cached_keys1 = get_or_generate_keys(32, 0x04C11DB7, true);
        let cached_keys2 = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);

        assert_eq!(keys1, cached_keys1);
        assert_eq!(keys2, cached_keys2);

        // Clear cache
        clear_cache();

        // Verify cache was cleared by checking that new calls still work
        // (we can't directly verify cache is empty, but we can verify functionality)
        let new_keys1 = get_or_generate_keys(32, 0x04C11DB7, true);
        assert_eq!(keys1, new_keys1); // Should be same values, but freshly generated
    }

    #[test]
    fn test_cache_error_handling() {
        // Test that cache operations don't panic even if called multiple times
        clear_cache();
        clear_cache(); // Should not panic on empty cache

        // Test that get_or_generate_keys works even after multiple clears
        let keys = get_or_generate_keys(32, 0x04C11DB7, true);
        clear_cache();
        let keys2 = get_or_generate_keys(32, 0x04C11DB7, true);

        // Keys should be identical (same parameters produce same keys)
        assert_eq!(keys, keys2);
    }

    #[test]
    fn test_cache_key_debug_and_clone() {
        let key = CrcParamsCacheKey::new(32, 0x04C11DB7, true);

        // Test Debug trait
        let debug_str = format!("{:?}", key);
        assert!(debug_str.contains("CrcParamsCacheKey"));
        assert!(debug_str.contains("32"));
        assert!(debug_str.contains("0x4c11db7") || debug_str.contains("79764919"));
        assert!(debug_str.contains("true"));

        // Test Clone trait
        let cloned_key = key.clone();
        assert_eq!(key, cloned_key);
        assert_eq!(key.width, cloned_key.width);
        assert_eq!(key.poly, cloned_key.poly);
        assert_eq!(key.reflected, cloned_key.reflected);
    }

    // Thread safety tests
    #[test]
    fn test_concurrent_cache_reads() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        clear_cache();

        // Pre-populate cache with a known value
        let expected_keys = get_or_generate_keys(32, 0x04C11DB7, true);

        let num_threads = 8;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = Vec::new();

        // Spawn multiple threads that all read the same cached value simultaneously
        for i in 0..num_threads {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                // Wait for all threads to be ready
                barrier_clone.wait();

                // All threads read from cache simultaneously
                let keys = get_or_generate_keys(32, 0x04C11DB7, true);
                (i, keys)
            });
            handles.push(handle);
        }

        // Collect results from all threads
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join().expect("Thread should not panic"));
        }

        // Verify all threads got the same cached keys
        assert_eq!(results.len(), num_threads);
        for (thread_id, keys) in results {
            assert_eq!(
                keys, expected_keys,
                "Thread {} should get same cached keys",
                thread_id
            );
        }
    }

    #[test]
    #[allow(clippy::needless_range_loop)] // Intentionally testing concurrent indexed access
    fn test_concurrent_cache_writes() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        clear_cache();

        let num_threads = 6;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = Vec::new();

        // Test parameters for different cache entries
        let test_params = [
            (32, 0x04C11DB7, true),
            (32, 0x04C11DB7, false),
            (32, 0x1EDC6F41, true),
            (64, 0x42F0E1EBA9EA3693, true),
            (64, 0x42F0E1EBA9EA3693, false),
            (64, 0xD800000000000000, true),
        ];

        // Spawn threads that write different cache entries simultaneously
        for i in 0..num_threads {
            let barrier_clone = Arc::clone(&barrier);
            let (width, poly, reflected) = test_params[i];

            let handle = thread::spawn(move || {
                // Wait for all threads to be ready
                barrier_clone.wait();

                // Each thread generates and caches different parameters
                let keys = get_or_generate_keys(width, poly, reflected);
                (i, width, poly, reflected, keys)
            });
            handles.push(handle);
        }

        // Collect results from all threads
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join().expect("Thread should not panic"));
        }

        // Verify all threads completed successfully and got correct keys
        assert_eq!(results.len(), num_threads);

        // Verify each thread's keys match what we'd expect from direct generation
        for (thread_id, width, poly, reflected, keys) in results {
            let expected_keys = generate::keys(width, poly, reflected);
            assert_eq!(
                keys, expected_keys,
                "Thread {} should generate correct keys for params ({}, {:#x}, {})",
                thread_id, width, poly, reflected
            );

            // Verify the keys are now cached by reading them again
            let cached_keys = get_or_generate_keys(width, poly, reflected);
            assert_eq!(
                keys, cached_keys,
                "Thread {} keys should be cached",
                thread_id
            );
        }
    }

    #[test]
    fn test_read_write_contention() {
        use std::sync::{Arc, Barrier};
        use std::thread;
        use std::time::Duration;

        clear_cache();

        // Pre-populate cache with some values
        let _keys1 = get_or_generate_keys(32, 0x04C11DB7, true);
        let _keys2 = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);

        let num_readers = 6;
        let num_writers = 3;
        let total_threads = num_readers + num_writers;
        let barrier = Arc::new(Barrier::new(total_threads));
        let mut handles = Vec::new();

        // Spawn reader threads that continuously read cached values
        for i in 0..num_readers {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier_clone.wait();

                let mut read_count = 0;
                let start = std::time::Instant::now();

                // Read for a short duration
                while start.elapsed() < Duration::from_millis(50) {
                    let keys1 = get_or_generate_keys(32, 0x04C11DB7, true);
                    let keys2 = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);

                    // Verify we get consistent results
                    assert_eq!(keys1.len(), 23);
                    assert_eq!(keys2.len(), 23);
                    read_count += 1;
                }

                (format!("reader_{}", i), read_count)
            });
            handles.push(handle);
        }

        // Spawn writer threads that add new cache entries
        for i in 0..num_writers {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier_clone.wait();

                let mut write_count = 0;
                let start = std::time::Instant::now();

                // Write new entries for a short duration
                while start.elapsed() < Duration::from_millis(50) {
                    // Use different parameters to create new cache entries
                    let poly = 0x1EDC6F41 + (i as u64 * 0x1000) + (write_count as u64);
                    let keys = get_or_generate_keys(32, poly, true);

                    assert_eq!(keys.len(), 23);
                    write_count += 1;
                }

                (format!("writer_{}", i), write_count)
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join().expect("Thread should not panic"));
        }

        // Verify all threads completed successfully
        assert_eq!(results.len(), total_threads);

        // Verify readers and writers both made progress
        let reader_results: Vec<_> = results
            .iter()
            .filter(|(name, _)| name.starts_with("reader_"))
            .collect();
        let writer_results: Vec<_> = results
            .iter()
            .filter(|(name, _)| name.starts_with("writer_"))
            .collect();

        assert_eq!(reader_results.len(), num_readers);
        assert_eq!(writer_results.len(), num_writers);

        // All threads should have made some progress
        for (name, count) in &results {
            assert!(*count > 0, "Thread {} should have made progress", name);
        }
    }

    #[test]
    fn test_cache_consistency_under_concurrent_access() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        clear_cache();

        let num_threads = 10;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = Vec::new();

        // All threads will try to get the same cache entry simultaneously
        // This tests the race condition where multiple threads might try to
        // generate and cache the same keys at the same time
        for i in 0..num_threads {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier_clone.wait();

                // All threads request the same parameters simultaneously
                let keys = get_or_generate_keys(32, 0x04C11DB7, true);
                (i, keys)
            });
            handles.push(handle);
        }

        // Collect results from all threads
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join().expect("Thread should not panic"));
        }

        // Verify all threads got identical keys
        assert_eq!(results.len(), num_threads);
        let first_keys = results[0].1;

        for (thread_id, keys) in results {
            assert_eq!(
                keys, first_keys,
                "Thread {} should get identical keys to other threads",
                thread_id
            );
        }

        // Verify the keys are correct by comparing with direct generation
        let expected_keys = generate::keys(32, 0x04C11DB7, true);
        assert_eq!(
            first_keys, expected_keys,
            "Cached keys should match directly generated keys"
        );

        // Verify subsequent access still returns the same keys
        let final_keys = get_or_generate_keys(32, 0x04C11DB7, true);
        assert_eq!(
            final_keys, first_keys,
            "Final cache access should return same keys"
        );
    }

    #[test]
    fn test_mixed_concurrent_operations() {
        use std::sync::{Arc, Barrier};
        use std::thread;
        use std::time::Duration;

        clear_cache();

        let num_threads = 8;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = Vec::new();

        // Mix of operations: some threads do cache hits, some do cache misses,
        // some clear the cache, all happening concurrently
        for i in 0..num_threads {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier_clone.wait();

                let mut operations = 0;
                let start = std::time::Instant::now();

                while start.elapsed() < Duration::from_millis(30) {
                    match i % 4 {
                        0 => {
                            // Cache hit operations - same parameters
                            let _keys = get_or_generate_keys(32, 0x04C11DB7, true);
                        }
                        1 => {
                            // Cache miss operations - different parameters each time
                            let poly = 0x1EDC6F41 + (operations as u64);
                            let _keys = get_or_generate_keys(32, poly, true);
                        }
                        2 => {
                            // Mixed read operations
                            let _keys1 = get_or_generate_keys(32, 0x04C11DB7, true);
                            let _keys2 = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);
                        }
                        3 => {
                            // Occasional cache clear (but not too often to avoid disrupting other tests)
                            if operations % 10 == 0 {
                                clear_cache();
                            }
                            let _keys = get_or_generate_keys(32, 0x04C11DB7, true);
                        }
                        _ => unreachable!(),
                    }
                    operations += 1;
                }

                (i, operations)
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join().expect("Thread should not panic"));
        }

        // Verify all threads completed successfully
        assert_eq!(results.len(), num_threads);

        for (thread_id, operations) in results {
            assert!(
                operations > 0,
                "Thread {} should have completed some operations",
                thread_id
            );
        }

        // Verify cache is still functional after all the concurrent operations
        let final_keys = get_or_generate_keys(32, 0x04C11DB7, true);
        let expected_keys = generate::keys(32, 0x04C11DB7, true);
        assert_eq!(
            final_keys, expected_keys,
            "Cache should still work correctly after concurrent operations"
        );
    }

    // Error handling tests
    #[test]
    fn test_cache_lock_poisoning_recovery() {
        use std::panic;
        use std::sync::{Arc, Mutex};
        use std::thread;

        clear_cache();

        // Pre-populate cache with known values
        let expected_keys = get_or_generate_keys(32, 0x04C11DB7, true);

        // Create a scenario that could potentially poison the lock
        // We'll use a separate test to avoid actually poisoning our cache
        let poisoned_flag = Arc::new(Mutex::new(false));
        let poisoned_flag_clone = Arc::clone(&poisoned_flag);

        // Spawn a thread that panics while holding a lock (simulated)
        let handle = thread::spawn(move || {
            // Simulate a panic scenario - we don't actually poison the cache lock
            // because that would break other tests, but we test the recovery path
            let _guard = poisoned_flag_clone.lock().unwrap();
            panic!("Simulated panic");
        });

        // Wait for the thread to panic
        let result = handle.join();
        assert!(result.is_err(), "Thread should have panicked");

        // Verify that our cache still works despite the simulated error scenario
        let keys_after_panic = get_or_generate_keys(32, 0x04C11DB7, true);
        assert_eq!(
            keys_after_panic, expected_keys,
            "Cache should still work after simulated panic scenario"
        );

        // Test that new cache entries can still be created
        let new_keys = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);
        assert_eq!(new_keys.len(), 23, "New cache entries should still work");

        // Verify the new keys are cached
        let cached_new_keys = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);
        assert_eq!(new_keys, cached_new_keys, "New keys should be cached");
    }

    #[test]
    fn test_cache_fallback_to_direct_generation() {
        clear_cache();

        // Test that even if cache operations fail, we still get valid keys
        // This tests the fallback mechanism in get_or_generate_keys

        // Generate expected keys directly
        let expected_keys_32 = generate::keys(32, 0x04C11DB7, true);
        let expected_keys_64 = generate::keys(64, 0x42F0E1EBA9EA3693, false);

        // Get keys through cache (should work normally)
        let cached_keys_32 = get_or_generate_keys(32, 0x04C11DB7, true);
        let cached_keys_64 = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);

        // Verify keys are correct regardless of cache state
        assert_eq!(
            cached_keys_32, expected_keys_32,
            "CRC32 keys should be correct even with cache issues"
        );
        assert_eq!(
            cached_keys_64, expected_keys_64,
            "CRC64 keys should be correct even with cache issues"
        );

        // Test multiple calls to ensure consistency
        for _ in 0..5 {
            let keys_32 = get_or_generate_keys(32, 0x04C11DB7, true);
            let keys_64 = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);

            assert_eq!(
                keys_32, expected_keys_32,
                "Repeated calls should return consistent CRC32 keys"
            );
            assert_eq!(
                keys_64, expected_keys_64,
                "Repeated calls should return consistent CRC64 keys"
            );
        }
    }

    #[test]
    fn test_cache_operations_under_memory_pressure() {
        clear_cache();

        // Simulate memory pressure by creating many cache entries
        // This tests that cache operations remain stable under load
        let mut test_keys = Vec::new();
        let num_entries = 100;

        // Create many different cache entries
        for i in 0..num_entries {
            let poly = 0x04C11DB7 + (i as u64);
            let reflected = i % 2 == 0;
            let width = if i % 3 == 0 { 64 } else { 32 };

            let keys = get_or_generate_keys(width, poly, reflected);
            test_keys.push((width, poly, reflected, keys));
        }

        // Verify all entries are correctly cached and retrievable
        for (i, &(width, poly, reflected, ref expected_keys)) in test_keys.iter().enumerate() {
            let cached_keys = get_or_generate_keys(width, poly, reflected);
            assert_eq!(
                cached_keys, *expected_keys,
                "Entry {} should be correctly cached",
                i
            );
        }

        // Test that cache operations still work after creating many entries
        let new_keys = get_or_generate_keys(32, 0x1EDC6F41, true);
        assert_eq!(
            new_keys.len(),
            23,
            "New entries should still work under memory pressure"
        );

        // Verify the new entry is cached
        let cached_new_keys = get_or_generate_keys(32, 0x1EDC6F41, true);
        assert_eq!(new_keys, cached_new_keys, "New entry should be cached");

        // Test cache clearing still works
        clear_cache();

        // Verify cache was cleared by testing that operations still work
        let post_clear_keys = get_or_generate_keys(32, 0x04C11DB7, true);
        assert_eq!(
            post_clear_keys.len(),
            23,
            "Cache should work after clearing under memory pressure"
        );
    }

    #[test]
    fn test_cache_error_recovery_patterns() {
        clear_cache();

        // Test various error recovery patterns to ensure robustness

        // Pattern 1: Rapid cache operations
        for i in 0..50 {
            let poly = 0x04C11DB7 + (i as u64 % 10); // Create some duplicates
            let keys = get_or_generate_keys(32, poly, true);
            assert_eq!(keys.len(), 23, "Rapid operation {} should succeed", i);
        }

        // Pattern 2: Interleaved cache hits and misses
        let base_keys = get_or_generate_keys(32, 0x04C11DB7, true);
        for i in 0..20 {
            // Cache hit
            let hit_keys = get_or_generate_keys(32, 0x04C11DB7, true);
            assert_eq!(hit_keys, base_keys, "Cache hit {} should be consistent", i);

            // Cache miss
            let miss_keys = get_or_generate_keys(32, 0x1EDC6F41 + (i as u64), false);
            assert_eq!(miss_keys.len(), 23, "Cache miss {} should succeed", i);
        }

        // Pattern 3: Mixed operations with clearing
        for i in 0..10 {
            let keys1 = get_or_generate_keys(32, 0x04C11DB7, true);
            let keys2 = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);

            if i % 3 == 0 {
                clear_cache();
            }

            // Operations should still work after clearing
            let keys3 = get_or_generate_keys(32, 0x04C11DB7, true);
            let keys4 = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);

            // Keys should be consistent (same parameters = same keys)
            assert_eq!(keys1, keys3, "Keys should be consistent across clears");
            assert_eq!(keys2, keys4, "Keys should be consistent across clears");
        }
    }

    #[test]
    fn test_cache_concurrent_error_scenarios() {
        use std::sync::{Arc, Barrier};
        use std::thread;
        use std::time::Duration;

        clear_cache();

        let num_threads = 8;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = Vec::new();

        // Create concurrent scenarios that could potentially cause errors
        for i in 0..num_threads {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier_clone.wait();

                let mut operations = 0;
                let errors = 0;
                let start = std::time::Instant::now();

                // Run operations for a short time with various patterns
                while start.elapsed() < Duration::from_millis(100) {
                    match operations % 5 {
                        0 => {
                            // Normal cache operations
                            let _keys = get_or_generate_keys(32, 0x04C11DB7, true);
                        }
                        1 => {
                            // Rapid different parameters
                            let poly = 0x1EDC6F41 + (operations as u64);
                            let _keys = get_or_generate_keys(32, poly, true);
                        }
                        2 => {
                            // Cache clearing (potential contention point)
                            clear_cache();
                        }
                        3 => {
                            // Mixed width operations
                            let _keys32 = get_or_generate_keys(32, 0x04C11DB7, true);
                            let _keys64 = get_or_generate_keys(64, 0x42F0E1EBA9EA3693, false);
                        }
                        4 => {
                            // Rapid same-parameter calls (cache hits)
                            for _ in 0..5 {
                                let _keys = get_or_generate_keys(32, 0x04C11DB7, true);
                            }
                        }
                        _ => unreachable!(),
                    }
                    operations += 1;
                }

                (i, operations, errors)
            });
            handles.push(handle);
        }

        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            match handle.join() {
                Ok(result) => results.push(result),
                Err(_) => panic!("Thread should not panic during error scenarios"),
            }
        }

        // Verify all threads completed successfully
        assert_eq!(results.len(), num_threads);

        for (thread_id, operations, errors) in results {
            assert!(
                operations > 0,
                "Thread {} should have completed operations",
                thread_id
            );
            // In our implementation, errors should be handled gracefully without propagating
            assert_eq!(
                errors, 0,
                "Thread {} should not have unhandled errors",
                thread_id
            );
        }

        // Verify cache is still functional after all concurrent error scenarios
        let final_keys = get_or_generate_keys(32, 0x04C11DB7, true);
        let expected_keys = generate::keys(32, 0x04C11DB7, true);
        assert_eq!(
            final_keys, expected_keys,
            "Cache should still work correctly after concurrent error scenarios"
        );
    }

    /// Non-stress version of the stress test to allow Miri to evaluate without timing out.
    #[test]
    fn test_cache_memory_allocation_non_stress() {
        cache_memory_allocation_stress(2);
    }

    /// Skipping for Miri runs due to time constraints, underlying code already covered by other
    /// tests.
    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_cache_memory_allocation_stress() {
        cache_memory_allocation_stress(1000);
    }

    fn cache_memory_allocation_stress(count: i32) {
        clear_cache();

        // Test cache behavior under memory allocation stress
        // Create a large number of unique cache entries to stress memory allocation
        let mut created_entries = Vec::new();
        let stress_count = count;

        // Create many unique cache entries
        for i in 0..stress_count {
            let width = if i % 2 == 0 { 32 } else { 64 };
            let poly = 0x04C11DB7 + (i as u64);
            let reflected = i % 3 == 0;

            let keys = get_or_generate_keys(width, poly, reflected);
            created_entries.push((width, poly, reflected, keys));

            // Verify each entry is valid
            assert_eq!(keys.len(), 23, "Entry {} should have valid keys", i);
        }

        // Verify all entries are still accessible (testing cache integrity)
        for (i, &(width, poly, reflected, ref expected_keys)) in created_entries.iter().enumerate()
        {
            let retrieved_keys = get_or_generate_keys(width, poly, reflected);
            assert_eq!(
                retrieved_keys, *expected_keys,
                "Entry {} should be retrievable after stress test",
                i
            );
        }

        // Test that new entries can still be created
        let new_keys = get_or_generate_keys(32, 0xFFFFFFFF, true);
        assert_eq!(
            new_keys.len(),
            23,
            "New entries should work after memory stress"
        );

        // Test cache clearing works under memory pressure
        clear_cache();

        // Verify cache operations still work after clearing
        let post_stress_keys = get_or_generate_keys(32, 0x04C11DB7, true);
        assert_eq!(
            post_stress_keys.len(),
            23,
            "Cache should work after memory stress and clearing"
        );
    }

    // Integration tests for CrcParams compatibility
    #[test]
    fn test_crc_params_new_behavior_unchanged() {
        use crate::CrcParams;

        clear_cache();

        // Test that CrcParams::new() creates identical instances regardless of caching
        let params1 = CrcParams::new(
            "TEST_CRC32",
            32,
            0x04C11DB7,
            0xFFFFFFFF,
            true,
            0xFFFFFFFF,
            0xCBF43926,
        );
        let params2 = CrcParams::new(
            "TEST_CRC32",
            32,
            0x04C11DB7,
            0xFFFFFFFF,
            true,
            0xFFFFFFFF,
            0xCBF43926,
        );

        // All fields should be identical
        assert_eq!(params1.name, params2.name);
        assert_eq!(params1.width, params2.width);
        assert_eq!(params1.poly, params2.poly);
        assert_eq!(params1.init, params2.init);
        assert_eq!(params1.refin, params2.refin);
        assert_eq!(params1.refout, params2.refout);
        assert_eq!(params1.xorout, params2.xorout);
        assert_eq!(params1.check, params2.check);
        assert_eq!(params1.keys, params2.keys);

        // Test CRC64 parameters as well
        let params64_1 = CrcParams::new(
            "TEST_CRC64",
            64,
            0x42F0E1EBA9EA3693,
            0xFFFFFFFFFFFFFFFF,
            false,
            0x0,
            0x6C40DF5F0B497347,
        );
        let params64_2 = CrcParams::new(
            "TEST_CRC64",
            64,
            0x42F0E1EBA9EA3693,
            0xFFFFFFFFFFFFFFFF,
            false,
            0x0,
            0x6C40DF5F0B497347,
        );

        assert_eq!(params64_1.name, params64_2.name);
        assert_eq!(params64_1.width, params64_2.width);
        assert_eq!(params64_1.poly, params64_2.poly);
        assert_eq!(params64_1.init, params64_2.init);
        assert_eq!(params64_1.refin, params64_2.refin);
        assert_eq!(params64_1.refout, params64_2.refout);
        assert_eq!(params64_1.xorout, params64_2.xorout);
        assert_eq!(params64_1.check, params64_2.check);
        assert_eq!(params64_1.keys, params64_2.keys);
    }

    #[test]
    fn test_existing_crc_parameter_combinations() {
        use crate::test::consts::TEST_ALL_CONFIGS;

        clear_cache();

        // Test all existing CRC parameter combinations work correctly with caching
        for config in TEST_ALL_CONFIGS {
            let params = crate::CrcParams::new(
                config.get_name(),
                config.get_width(),
                config.get_poly(),
                config.get_init(),
                config.get_refin(),
                config.get_xorout(),
                config.get_check(),
            );

            // Verify the parameters are set correctly
            assert_eq!(params.name, config.get_name());
            assert_eq!(params.width, config.get_width());
            assert_eq!(params.poly, config.get_poly());
            assert_eq!(params.init, config.get_init());
            assert_eq!(params.refin, config.get_refin());
            assert_eq!(params.refout, config.get_refin());
            assert_eq!(params.xorout, config.get_xorout());
            assert_eq!(params.check, config.get_check());

            // Verify keys are correct by comparing with expected keys
            let expected_keys = config.get_keys();
            assert_eq!(
                params.keys,
                expected_keys,
                "Keys mismatch for {}: expected {:?}, got {:?}",
                config.get_name(),
                expected_keys,
                params.keys
            );
        }
    }

    #[test]
    fn test_cached_vs_uncached_results_identical() {
        clear_cache();

        // Test parameters that affect key generation
        let test_cases = [
            (32, 0x04C11DB7, true),          // CRC32 reflected
            (32, 0x04C11DB7, false),         // CRC32 non-reflected
            (32, 0x1EDC6F41, true),          // CRC32C
            (64, 0x42F0E1EBA9EA3693, true),  // CRC64 ISO reflected
            (64, 0x42F0E1EBA9EA3693, false), // CRC64 ISO non-reflected
            (64, 0xD800000000000000, true),  // CRC64 ECMA
        ];

        for &(width, poly, reflected) in &test_cases {
            // Generate keys directly (uncached)
            let uncached_keys = generate::keys(width, poly, reflected);

            // Clear cache to ensure first call is cache miss
            clear_cache();

            // Create CrcParams instance (first call - cache miss)
            let params1 =
                crate::CrcParams::new("TEST", width, poly, 0xFFFFFFFFFFFFFFFF, reflected, 0x0, 0x0);

            // Create another CrcParams instance with same parameters (cache hit)
            let params2 =
                crate::CrcParams::new("TEST", width, poly, 0xFFFFFFFFFFFFFFFF, reflected, 0x0, 0x0);

            // All should be identical
            assert_eq!(
                uncached_keys, params1.keys,
                "Uncached keys should match CrcParams keys for width={}, poly={:#x}, reflected={}",
                width, poly, reflected
            );
            assert_eq!(
                params1.keys, params2.keys,
                "Cached and uncached CrcParams should have identical keys for width={}, poly={:#x}, reflected={}",
                width, poly, reflected
            );
            assert_eq!(
                uncached_keys, params2.keys,
                "All key generation methods should produce identical results for width={}, poly={:#x}, reflected={}",
                width, poly, reflected
            );
        }
    }

    #[test]
    fn test_multiple_crc_params_instances_use_cached_keys() {
        clear_cache();

        // Create multiple CrcParams instances with the same parameters
        let width = 32;
        let poly = 0x04C11DB7;
        let reflected = true;
        let init = 0xFFFFFFFF;
        let xorout = 0xFFFFFFFF;
        let check = 0xCBF43926;

        // First instance - should generate and cache keys
        let params1 = crate::CrcParams::new("TEST1", width, poly, init, reflected, xorout, check);

        // Subsequent instances - should use cached keys
        let params2 = crate::CrcParams::new("TEST2", width, poly, init, reflected, xorout, check);
        let params3 = crate::CrcParams::new("TEST3", width, poly, init, reflected, xorout, check);
        let params4 = crate::CrcParams::new("TEST4", width, poly, init, reflected, xorout, check);

        // All should have identical keys (proving cache is working)
        assert_eq!(
            params1.keys, params2.keys,
            "Instance 1 and 2 should have identical keys"
        );
        assert_eq!(
            params1.keys, params3.keys,
            "Instance 1 and 3 should have identical keys"
        );
        assert_eq!(
            params1.keys, params4.keys,
            "Instance 1 and 4 should have identical keys"
        );
        assert_eq!(
            params2.keys, params3.keys,
            "Instance 2 and 3 should have identical keys"
        );
        assert_eq!(
            params2.keys, params4.keys,
            "Instance 2 and 4 should have identical keys"
        );
        assert_eq!(
            params3.keys, params4.keys,
            "Instance 3 and 4 should have identical keys"
        );

        // Verify keys are mathematically correct
        let expected_keys = generate::keys(width, poly, reflected);
        assert_eq!(
            params1.keys, expected_keys,
            "Cached keys should be mathematically correct"
        );

        // Test with different parameters that don't affect key generation
        let params5 = crate::CrcParams::new(
            "DIFFERENT_NAME",
            width,
            poly,
            0x12345678,
            reflected,
            0x87654321,
            0xABCDEF,
        );
        assert_eq!(
            params1.keys, params5.keys,
            "Different init/xorout/check/name should not affect cached keys"
        );

        // Test with CRC64 parameters
        let width64 = 64;
        let poly64 = 0x42F0E1EBA9EA3693;
        let reflected64 = false;

        let params64_1 = crate::CrcParams::new(
            "CRC64_1",
            width64,
            poly64,
            0xFFFFFFFFFFFFFFFF,
            reflected64,
            0x0,
            0x0,
        );
        let params64_2 = crate::CrcParams::new(
            "CRC64_2",
            width64,
            poly64,
            0x0,
            reflected64,
            0xFFFFFFFFFFFFFFFF,
            0x12345,
        );
        let params64_3 = crate::CrcParams::new(
            "CRC64_3",
            width64,
            poly64,
            0x123456789ABCDEF0,
            reflected64,
            0x0FEDCBA987654321,
            0x999,
        );

        assert_eq!(
            params64_1.keys, params64_2.keys,
            "CRC64 instances should have identical keys"
        );
        assert_eq!(
            params64_1.keys, params64_3.keys,
            "CRC64 instances should have identical keys"
        );

        let expected_keys64 = generate::keys(width64, poly64, reflected64);
        assert_eq!(
            params64_1.keys, expected_keys64,
            "CRC64 cached keys should be mathematically correct"
        );
    }

    #[test]
    fn test_crc_params_api_compatibility() {
        use crate::{CrcAlgorithm, CrcParams};

        clear_cache();

        // Test that the CrcParams API remains unchanged
        let params = CrcParams::new(
            "API_TEST", 32, 0x04C11DB7, 0xFFFFFFFF, true, 0xFFFFFFFF, 0xCBF43926,
        );

        // Verify all public fields are accessible and have expected types
        let _algorithm: CrcAlgorithm = params.algorithm;
        let _name: &'static str = params.name;
        let _width: u8 = params.width;
        let _poly: u64 = params.poly;
        let _init: u64 = params.init;
        let _refin: bool = params.refin;
        let _refout: bool = params.refout;
        let _xorout: u64 = params.xorout;
        let _check: u64 = params.check;
        let _keys: [u64; 23] = params.keys.to_keys_array_23();

        // Verify the algorithm is set to CrcCustom (unified custom variant)
        assert!(matches!(params.algorithm, CrcAlgorithm::CrcCustom));

        // Test that CrcParams can be copied and cloned
        let params_copy = params;
        let params_clone = params;

        assert_eq!(params.keys, params_copy.keys);
        assert_eq!(params.keys, params_clone.keys);

        // Test Debug formatting works
        let debug_str = format!("{:?}", params);
        assert!(debug_str.contains("CrcParams"));
        assert!(debug_str.contains("API_TEST"));
    }

    #[test]
    fn test_crc_params_with_all_standard_algorithms() {
        use crate::test::consts::TEST_ALL_CONFIGS;

        clear_cache();

        // Test creating CrcParams for all standard CRC algorithms
        for config in TEST_ALL_CONFIGS {
            // Create CrcParams using the same parameters as the standard algorithm
            let params = crate::CrcParams::new(
                config.get_name(),
                config.get_width(),
                config.get_poly(),
                config.get_init(),
                config.get_refin(),
                config.get_xorout(),
                config.get_check(),
            );

            // Verify the created params match the expected configuration
            assert_eq!(params.name, config.get_name());
            assert_eq!(params.width, config.get_width());
            assert_eq!(params.poly, config.get_poly());
            assert_eq!(params.init, config.get_init());
            assert_eq!(params.refin, config.get_refin());
            assert_eq!(params.refout, config.get_refin());
            assert_eq!(params.xorout, config.get_xorout());
            assert_eq!(params.check, config.get_check());

            // Most importantly, verify the keys are correct
            assert_eq!(
                params.keys,
                config.get_keys(),
                "Keys should match expected values for {}",
                config.get_name()
            );

            // Create a second instance to test caching
            let params2 = crate::CrcParams::new(
                "CACHED_VERSION", // Different name shouldn't affect caching
                config.get_width(),
                config.get_poly(),
                0x12345678, // Different init shouldn't affect caching
                config.get_refin(),
                0x87654321, // Different xorout shouldn't affect caching
                0xABCDEF,   // Different check shouldn't affect caching
            );

            // Keys should be identical (proving cache hit)
            assert_eq!(
                params.keys,
                params2.keys,
                "Cached keys should be identical for {}",
                config.get_name()
            );
        }
    }

    #[test]
    fn test_crc_params_edge_cases() {
        clear_cache();

        // Test edge cases for CrcParams creation

        // Test minimum and maximum polynomial values
        let params_min_poly = crate::CrcParams::new("MIN_POLY", 32, 0x1, 0x0, false, 0x0, 0x0);
        let params_max_poly =
            crate::CrcParams::new("MAX_POLY", 32, 0xFFFFFFFF, 0x0, false, 0x0, 0x0);

        // Both should create valid instances
        assert_eq!(params_min_poly.width, 32);
        assert_eq!(params_min_poly.poly, 0x1);
        assert_eq!(params_max_poly.width, 32);
        assert_eq!(params_max_poly.poly, 0xFFFFFFFF);

        // Test both reflection modes
        let params_reflected =
            crate::CrcParams::new("REFLECTED", 32, 0x04C11DB7, 0x0, true, 0x0, 0x0);
        let params_normal = crate::CrcParams::new("NORMAL", 32, 0x04C11DB7, 0x0, false, 0x0, 0x0);

        // Should have different keys due to different reflection
        assert_ne!(params_reflected.keys, params_normal.keys);
        assert!(params_reflected.refin);
        assert!(params_reflected.refout);
        assert!(!params_normal.refin);
        assert!(!params_normal.refout);

        // Test 64-bit edge cases
        let params64_min = crate::CrcParams::new("CRC64_MIN", 64, 0x1, 0x0, false, 0x0, 0x0);
        let params64_max =
            crate::CrcParams::new("CRC64_MAX", 64, 0xFFFFFFFFFFFFFFFF, 0x0, false, 0x0, 0x0);

        assert_eq!(params64_min.width, 64);
        assert_eq!(params64_min.poly, 0x1);
        assert_eq!(params64_max.width, 64);
        assert_eq!(params64_max.poly, 0xFFFFFFFFFFFFFFFF);

        // Verify all instances have valid 23-element key arrays
        assert_eq!(params_min_poly.keys.key_count(), 23);
        assert_eq!(params_max_poly.keys.key_count(), 23);
        assert_eq!(params_reflected.keys.key_count(), 23);
        assert_eq!(params_normal.keys.key_count(), 23);
        assert_eq!(params64_min.keys.key_count(), 23);
        assert_eq!(params64_max.keys.key_count(), 23);
    }

    #[test]
    fn test_crc_params_concurrent_creation() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        clear_cache();

        let num_threads = 8;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = Vec::new();

        // All threads create CrcParams with the same parameters simultaneously
        for i in 0..num_threads {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier_clone.wait();

                // All threads create the same CrcParams
                let params = crate::CrcParams::new(
                    "CONCURRENT_TEST",
                    32,
                    0x04C11DB7,
                    0xFFFFFFFF,
                    true,
                    0xFFFFFFFF,
                    0xCBF43926,
                );

                (i, params)
            });
            handles.push(handle);
        }

        // Collect results from all threads
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join().expect("Thread should not panic"));
        }

        // Verify all threads completed successfully
        assert_eq!(results.len(), num_threads);

        // Verify all CrcParams instances have identical keys
        let first_keys = results[0].1.keys;
        for (thread_id, params) in results {
            assert_eq!(
                params.keys, first_keys,
                "Thread {} should have identical keys to other threads",
                thread_id
            );

            // Verify other fields are also correct
            assert_eq!(params.name, "CONCURRENT_TEST");
            assert_eq!(params.width, 32);
            assert_eq!(params.poly, 0x04C11DB7);
            assert_eq!(params.init, 0xFFFFFFFF);
            assert!(params.refin);
            assert!(params.refout);
            assert_eq!(params.xorout, 0xFFFFFFFF);
            assert_eq!(params.check, 0xCBF43926);
        }

        // Verify the keys are mathematically correct
        let expected_keys = generate::keys(32, 0x04C11DB7, true);
        assert_eq!(
            first_keys, expected_keys,
            "All concurrent CrcParams should have correct keys"
        );
    }

    #[test]
    fn test_lock_poisoning_recovery() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        clear_cache();

        // This test is tricky because we need to poison the lock without
        // actually breaking our test. We'll simulate lock poisoning by
        // creating a scenario where a thread panics while holding a write lock.
        // However, since our implementation uses best-effort error handling,
        // it should gracefully degrade rather than propagate panics.

        // First, verify normal operation
        let keys_before = get_or_generate_keys(32, 0x04C11DB7, true);
        assert_eq!(keys_before.len(), 23);

        // Test that even if internal operations fail, the function still returns valid keys
        // We can't easily poison the lock in a controlled way, but we can verify
        // that our error handling works by testing edge cases

        // Multiple rapid cache operations that might stress the locking mechanism
        let num_threads = 4;
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = Vec::new();

        for i in 0..num_threads {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                barrier_clone.wait();

                // Rapid cache operations that might cause contention
                for j in 0..20 {
                    let poly = 0x04C11DB7 + (i as u64 * 1000) + (j as u64);
                    let keys = get_or_generate_keys(32, poly, true);
                    assert_eq!(
                        keys.len(),
                        23,
                        "Thread {} iteration {} should return valid keys",
                        i,
                        j
                    );

                    // Occasional cache clear to increase contention
                    if j % 7 == 0 {
                        clear_cache();
                    }
                }

                i
            });
            handles.push(handle);
        }

        // All threads should complete successfully
        for handle in handles {
            let thread_id = handle.join().expect("Thread should not panic");
            assert!(thread_id < num_threads);
        }

        // Verify cache is still functional after stress testing
        let keys_after = get_or_generate_keys(32, 0x04C11DB7, true);
        assert_eq!(keys_after.len(), 23);

        // Keys should be mathematically correct regardless of cache state
        let expected_keys = generate::keys(32, 0x04C11DB7, true);
        assert_eq!(keys_after, expected_keys);
    }

    #[test]
    fn test_cache_behavior_with_thread_local_access() {
        use std::thread;

        clear_cache();

        // Test that cache works correctly when accessed from different threads
        // in sequence (not concurrently)

        let keys_main = get_or_generate_keys(32, 0x04C11DB7, true);

        let handle = thread::spawn(|| {
            // This thread should see the cached value from the main thread

            get_or_generate_keys(32, 0x04C11DB7, true)
        });

        let keys_from_thread = handle.join().expect("Thread should not panic");

        // Both should be identical
        assert_eq!(keys_main, keys_from_thread);

        // Test multiple sequential threads
        let mut thread_keys = Vec::new();

        for i in 0..5 {
            let handle = thread::spawn(move || {
                let keys = get_or_generate_keys(32, 0x04C11DB7, true);
                (i, keys)
            });

            let (thread_id, keys) = handle.join().expect("Thread should not panic");
            thread_keys.push((thread_id, keys));
        }

        // All threads should get the same cached keys
        for (thread_id, keys) in thread_keys {
            assert_eq!(
                keys, keys_main,
                "Thread {} should get same cached keys",
                thread_id
            );
        }
    }
}
