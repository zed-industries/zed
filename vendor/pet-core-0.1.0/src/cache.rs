// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Generic caching abstraction for locators.
//!
//! Provides a thread-safe cache wrapper that consolidates common caching patterns
//! used across multiple locators in the codebase.

use std::{
    collections::HashMap,
    hash::Hash,
    path::PathBuf,
    sync::{Arc, Condvar, Mutex, RwLock},
};

use crate::{manager::EnvManager, python_environment::PythonEnvironment};

/// A thread-safe cache that stores key-value pairs using RwLock for concurrent access.
///
/// This cache uses read-write locks to allow multiple concurrent readers while
/// ensuring exclusive access for writers. Values must implement Clone to be
/// returned from the cache.
pub struct LocatorCache<K, V> {
    cache: RwLock<HashMap<K, V>>,
    in_flight: Mutex<HashMap<K, Arc<InFlightEntry<V>>>>,
}

struct InFlightEntry<V> {
    result: Mutex<Option<InFlightResult<V>>>,
    changed: Condvar,
}

#[derive(Clone)]
enum InFlightResult<V> {
    Value(Option<V>),
    Panicked,
}

struct InFlightOwnerGuard<'a, K: Eq + Hash, V> {
    key: Option<K>,
    entry: Arc<InFlightEntry<V>>,
    in_flight: &'a Mutex<HashMap<K, Arc<InFlightEntry<V>>>>,
}

enum InFlightClaim<'a, K: Eq + Hash, V> {
    Owner(InFlightOwnerGuard<'a, K, V>),
    Waiter(Arc<InFlightEntry<V>>),
}

impl<V> InFlightEntry<V> {
    fn new() -> Self {
        Self {
            result: Mutex::new(None),
            changed: Condvar::new(),
        }
    }
}

impl<K: Eq + Hash, V> InFlightOwnerGuard<'_, K, V> {
    fn complete(mut self, result: Option<V>) {
        self.publish_result(result);
    }

    fn publish_result(&mut self, result: Option<V>) {
        self.publish(InFlightResult::Value(result));
    }

    fn publish_panic(&mut self) {
        self.publish(InFlightResult::Panicked);
    }

    fn publish(&mut self, result: InFlightResult<V>) {
        *self
            .entry
            .result
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(result);

        if let Some(key) = self.key.take() {
            self.in_flight
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .remove(&key);
        }

        self.entry.changed.notify_all();
    }
}

impl<K: Eq + Hash, V> Drop for InFlightOwnerGuard<'_, K, V> {
    fn drop(&mut self) {
        if self.key.is_some() {
            self.publish_panic();
        }
    }
}

impl<K: Eq + Hash, V: Clone> LocatorCache<K, V> {
    /// Creates a new empty cache.
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            in_flight: Mutex::new(HashMap::new()),
        }
    }

    /// Returns a cloned value for the given key if it exists in the cache.
    pub fn get(&self, key: &K) -> Option<V> {
        self.cache
            .read()
            .expect("locator cache lock poisoned")
            .get(key)
            .cloned()
    }

    /// Checks if the cache contains the given key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.cache
            .read()
            .expect("locator cache lock poisoned")
            .contains_key(key)
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// Returns the previous value if the key was already present.
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.cache
            .write()
            .expect("locator cache lock poisoned")
            .insert(key, value)
    }

    /// Inserts multiple key-value pairs into the cache atomically.
    ///
    /// This method acquires a single write lock for all insertions, which is more
    /// efficient than calling `insert` multiple times when inserting many entries.
    pub fn insert_many(&self, entries: impl IntoIterator<Item = (K, V)>) {
        let mut cache = self.cache.write().expect("locator cache lock poisoned");
        for (key, value) in entries {
            cache.insert(key, value);
        }
    }

    /// Returns a cloned value for the given key if it exists, otherwise computes
    /// and inserts the value using the provided closure.
    ///
    /// This method first checks with a read lock. If the key is missing, it
    /// claims a per-key in-flight slot before computing the value so concurrent
    /// callers for the same key wait for the first computation instead of
    /// running duplicate closures with duplicate side effects. `None` results
    /// are shared with current waiters but are not stored in the cache, so later
    /// calls can retry the computation.
    ///
    /// The closure must not call `get_or_insert_with` for the same cache and key,
    /// directly or indirectly, because the owner would wait on its own in-flight
    /// entry. If the owner panics before publishing a result, waiters for the same
    /// key are woken and panic instead of silently receiving an uncached `None`.
    #[must_use]
    pub fn get_or_insert_with<F>(&self, key: K, f: F) -> Option<V>
    where
        F: FnOnce() -> Option<V>,
        K: Clone,
    {
        // First check with read lock.
        {
            let cache = self.cache.read().expect("locator cache lock poisoned");
            if let Some(value) = cache.get(&key) {
                return Some(value.clone());
            }
        }

        let in_flight = match self.claim_in_flight(&key) {
            InFlightClaim::Owner(in_flight) => in_flight,
            InFlightClaim::Waiter(entry) => return Self::wait_for_in_flight(entry),
        };

        // Check again after claiming the in-flight slot. Another thread may have
        // completed the same key while this thread was waiting.
        {
            let cache = self.cache.read().expect("locator cache lock poisoned");
            if let Some(value) = cache.get(&key) {
                let result = Some(value.clone());
                in_flight.complete(result.clone());
                return result;
            }
        }

        // Compute the value (outside of any lock)
        let result = if let Some(value) = f() {
            // Acquire write lock and insert
            let mut cache = self.cache.write().expect("locator cache lock poisoned");
            // Double-check in case another thread inserted while we were computing
            if let Some(existing) = cache.get(&key) {
                Some(existing.clone())
            } else {
                cache.insert(key, value.clone());
                Some(value)
            }
        } else {
            None
        };

        in_flight.complete(result.clone());
        result
    }

    fn claim_in_flight(&self, key: &K) -> InFlightClaim<'_, K, V>
    where
        K: Clone,
    {
        let mut in_flight = self
            .in_flight
            .lock()
            .expect("locator cache in-flight lock poisoned");

        if let Some(entry) = in_flight.get(key) {
            return InFlightClaim::Waiter(entry.clone());
        }

        let entry = Arc::new(InFlightEntry::new());
        in_flight.insert(key.clone(), entry.clone());
        InFlightClaim::Owner(InFlightOwnerGuard {
            key: Some(key.clone()),
            entry,
            in_flight: &self.in_flight,
        })
    }

    fn wait_for_in_flight(entry: Arc<InFlightEntry<V>>) -> Option<V> {
        let mut result = entry
            .result
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        while result.is_none() {
            result = entry
                .changed
                .wait(result)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }

        match result.clone().unwrap() {
            InFlightResult::Value(value) => value,
            InFlightResult::Panicked => {
                panic!("locator cache in-flight owner panicked before publishing a result");
            }
        }
    }

    /// Clears all entries from the cache.
    pub fn clear(&self) {
        self.cache
            .write()
            .expect("locator cache lock poisoned")
            .clear();
    }

    /// Returns all values in the cache as a vector.
    pub fn values(&self) -> Vec<V> {
        self.cache
            .read()
            .expect("locator cache lock poisoned")
            .values()
            .cloned()
            .collect()
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> usize {
        self.cache
            .read()
            .expect("locator cache lock poisoned")
            .len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache
            .read()
            .expect("locator cache lock poisoned")
            .is_empty()
    }

    /// Returns all entries in the cache as a HashMap.
    pub fn clone_map(&self) -> HashMap<K, V>
    where
        K: Clone,
    {
        self.cache
            .read()
            .expect("locator cache lock poisoned")
            .clone()
    }
}

impl<K: Eq + Hash, V: Clone> Default for LocatorCache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for caching Python environments by their path.
pub type EnvironmentCache = LocatorCache<PathBuf, PythonEnvironment>;

/// Type alias for caching environment managers by their path.
pub type ManagerCache = LocatorCache<PathBuf, EnvManager>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc, Arc, Barrier, Mutex,
    };
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_cache_get_and_insert() {
        let cache: LocatorCache<String, i32> = LocatorCache::new();

        assert!(cache.get(&"key1".to_string()).is_none());
        assert!(!cache.contains_key(&"key1".to_string()));

        cache.insert("key1".to_string(), 42);

        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        assert!(cache.contains_key(&"key1".to_string()));
    }

    #[test]
    fn test_cache_get_or_insert_with() {
        let cache: LocatorCache<String, i32> = LocatorCache::new();

        // First call should compute and insert
        let result = cache.get_or_insert_with("key1".to_string(), || Some(42));
        assert_eq!(result, Some(42));

        // Second call should return cached value
        let result = cache.get_or_insert_with("key1".to_string(), || Some(100));
        assert_eq!(result, Some(42));

        // Test with None return
        let result = cache.get_or_insert_with("key2".to_string(), || None);
        assert!(result.is_none());
        assert!(!cache.contains_key(&"key2".to_string()));
    }

    #[test]
    fn test_cache_get_or_insert_with_runs_one_closure_per_key() {
        let cache: Arc<LocatorCache<String, i32>> = Arc::new(LocatorCache::new());
        let barrier = Arc::new(Barrier::new(3));
        let calls = Arc::new(AtomicUsize::new(0));
        let (started_tx, started_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let release_rx = Arc::new(Mutex::new(release_rx));
        let mut handles = vec![];

        for _ in 0..2 {
            let cache = cache.clone();
            let barrier = barrier.clone();
            let calls = calls.clone();
            let started_tx = started_tx.clone();
            let release_rx = release_rx.clone();
            handles.push(thread::spawn(move || {
                barrier.wait();
                cache.get_or_insert_with("key".to_string(), || {
                    calls.fetch_add(1, Ordering::SeqCst);
                    started_tx.send(()).unwrap();
                    release_rx
                        .lock()
                        .unwrap()
                        .recv_timeout(Duration::from_secs(5))
                        .unwrap();
                    Some(42)
                })
            }));
        }

        barrier.wait();
        started_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(started_rx.try_recv().is_err());

        release_tx.send(()).unwrap();
        release_tx.send(()).unwrap();

        let mut results = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>();
        results.sort();

        assert_eq!(results, vec![Some(42), Some(42)]);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_cache_get_or_insert_with_shares_concurrent_none_result() {
        let entry = Arc::new(InFlightEntry::new());
        let waiter_entry = entry.clone();
        let waiter =
            thread::spawn(move || LocatorCache::<String, i32>::wait_for_in_flight(waiter_entry));

        *entry
            .result
            .lock()
            .expect("locator cache in-flight result lock poisoned") =
            Some(InFlightResult::Value(None));
        entry.changed.notify_all();

        assert_eq!(waiter.join().unwrap(), None);

        let cache: LocatorCache<String, i32> = LocatorCache::new();
        assert_eq!(cache.get_or_insert_with("key".to_string(), || None), None);
        assert!(!cache.contains_key(&"key".to_string()));

        assert_eq!(
            cache.get_or_insert_with("key".to_string(), || Some(42)),
            Some(42)
        );
    }

    #[test]
    fn test_cache_get_or_insert_with_panic_releases_in_flight_key() {
        let cache: LocatorCache<String, i32> = LocatorCache::new();

        let result = std::panic::catch_unwind(|| {
            let _ = cache.get_or_insert_with("key".to_string(), || -> Option<i32> {
                panic!("boom");
            });
        });

        assert!(result.is_err());
        assert!(!cache.contains_key(&"key".to_string()));
        assert_eq!(
            cache.get_or_insert_with("key".to_string(), || Some(42)),
            Some(42)
        );
    }

    #[test]
    fn test_cache_panicked_owner_wakes_waiters_with_panic() {
        let key = "key".to_string();
        let entry = Arc::new(InFlightEntry::new());
        let in_flight: Mutex<HashMap<String, Arc<InFlightEntry<i32>>>> = Mutex::new(HashMap::new());

        in_flight.lock().unwrap().insert(key.clone(), entry.clone());
        let owner = InFlightOwnerGuard {
            key: Some(key),
            entry: entry.clone(),
            in_flight: &in_flight,
        };

        drop(owner);

        let waiter_result =
            std::panic::catch_unwind(|| LocatorCache::<String, i32>::wait_for_in_flight(entry));

        assert!(waiter_result.is_err());
        assert!(in_flight.lock().unwrap().is_empty());
    }

    #[test]
    fn test_cache_publish_result_recovers_poisoned_in_flight_locks() {
        let key = "key".to_string();
        let entry = Arc::new(InFlightEntry::new());
        let in_flight: Arc<Mutex<HashMap<String, Arc<InFlightEntry<i32>>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        in_flight.lock().unwrap().insert(key.clone(), entry.clone());

        let poison_entry = entry.clone();
        assert!(thread::spawn(move || {
            let _guard = poison_entry.result.lock().unwrap();
            panic!("poison result lock");
        })
        .join()
        .is_err());

        let poison_in_flight = in_flight.clone();
        assert!(thread::spawn(move || {
            let _guard = poison_in_flight.lock().unwrap();
            panic!("poison in-flight lock");
        })
        .join()
        .is_err());

        let owner = InFlightOwnerGuard {
            key: Some(key),
            entry: entry.clone(),
            in_flight: &in_flight,
        };

        owner.complete(Some(42));

        assert_eq!(
            LocatorCache::<String, i32>::wait_for_in_flight(entry),
            Some(42)
        );
        assert!(in_flight
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .is_empty());
    }

    #[test]
    fn test_cache_clear() {
        let cache: LocatorCache<String, i32> = LocatorCache::new();

        cache.insert("key1".to_string(), 42);
        cache.insert("key2".to_string(), 100);

        assert_eq!(cache.len(), 2);

        cache.clear();

        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_values() {
        let cache: LocatorCache<String, i32> = LocatorCache::new();

        cache.insert("key1".to_string(), 42);
        cache.insert("key2".to_string(), 100);

        let mut values = cache.values();
        values.sort();
        assert_eq!(values, vec![42, 100]);
    }

    #[test]
    fn test_cache_insert_many() {
        let cache: LocatorCache<String, i32> = LocatorCache::new();

        let entries = vec![
            ("key1".to_string(), 42),
            ("key2".to_string(), 100),
            ("key3".to_string(), 200),
        ];

        cache.insert_many(entries);

        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        assert_eq!(cache.get(&"key2".to_string()), Some(100));
        assert_eq!(cache.get(&"key3".to_string()), Some(200));
    }
}
