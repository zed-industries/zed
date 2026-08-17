/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{OnceCell, RwLock};

/// Expiry-aware cache
///
/// [`ExpiringCache`] implements two important features:
/// 1. Respect expiry of contents
/// 2. Deduplicate load requests to prevent thundering herds when no value is present.
#[derive(Debug)]
pub struct ExpiringCache<T, E> {
    /// Amount of time before the actual expiration time
    /// when the value is considered expired.
    buffer_time: Duration,
    value: Arc<RwLock<OnceCell<(T, SystemTime)>>>,
    _phantom: PhantomData<E>,
}

impl<T, E> Clone for ExpiringCache<T, E> {
    fn clone(&self) -> Self {
        Self {
            buffer_time: self.buffer_time,
            value: self.value.clone(),
            _phantom: Default::default(),
        }
    }
}

impl<T, E> ExpiringCache<T, E>
where
    T: Clone,
{
    /// Creates `ExpiringCache` with the given `buffer_time`.
    pub fn new(buffer_time: Duration) -> Self {
        ExpiringCache {
            buffer_time,
            value: Arc::new(RwLock::new(OnceCell::new())),
            _phantom: Default::default(),
        }
    }

    #[cfg(all(test, feature = "client", feature = "http-auth"))]
    async fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.value
            .read()
            .await
            .get()
            .cloned()
            .map(|(creds, _expiry)| creds)
    }

    /// Attempts to refresh the cached value with the given future.
    /// If multiple threads attempt to refresh at the same time, one of them will win,
    /// and the others will await that thread's result rather than multiple refreshes occurring.
    /// The function given to acquire a value future, `f`, will not be called
    /// if another thread is chosen to load the value.
    pub async fn get_or_load<F, Fut>(&self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(T, SystemTime), E>>,
    {
        let lock = self.value.read().await;
        let future = lock.get_or_try_init(f);
        future.await.map(|(value, _expiry)| value.clone())
    }

    /// If the value is expired, clears the cache. Otherwise, yields the current value.
    pub async fn yield_or_clear_if_expired(&self, now: SystemTime) -> Option<T> {
        // Short-circuit if the value is not expired
        if let Some((value, expiry)) = self.value.read().await.get() {
            if !expired(*expiry, self.buffer_time, now) {
                return Some(value.clone());
            } else {
                tracing::debug!(expiry = ?expiry, delta= ?now.duration_since(*expiry), "An item existed but it expired.")
            }
        }

        // Acquire a write lock to clear the cache, but then once the lock is acquired,
        // check again that the value is not already cleared. If it has been cleared,
        // then another thread is refreshing the cache by the time the write lock was acquired.
        let mut lock = self.value.write().await;
        if let Some((_value, expiration)) = lock.get() {
            // Also check that we're clearing the expired value and not a value
            // that has been refreshed by another thread.
            if expired(*expiration, self.buffer_time, now) {
                *lock = OnceCell::new();
            }
        }
        None
    }
}

fn expired(expiration: SystemTime, buffer_time: Duration, now: SystemTime) -> bool {
    now >= (expiration - buffer_time)
}

#[cfg(all(test, feature = "client", feature = "http-auth"))]
mod tests {
    use super::{expired, ExpiringCache};
    use aws_smithy_runtime_api::box_error::BoxError;
    use aws_smithy_runtime_api::client::identity::http::Token;
    use aws_smithy_runtime_api::client::identity::Identity;
    use std::time::{Duration, SystemTime};
    use tracing_test::traced_test;

    fn identity(expired_secs: u64) -> Result<(Identity, SystemTime), BoxError> {
        let expiration = epoch_secs(expired_secs);
        let identity = Identity::new(Token::new("test", Some(expiration)), Some(expiration));
        Ok((identity, expiration))
    }

    fn epoch_secs(secs: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(secs)
    }

    #[test]
    fn expired_check() {
        let ts = epoch_secs(100);
        assert!(expired(ts, Duration::from_secs(10), epoch_secs(1000)));
        assert!(expired(ts, Duration::from_secs(10), epoch_secs(90)));
        assert!(!expired(ts, Duration::from_secs(10), epoch_secs(10)));
    }

    #[traced_test]
    #[tokio::test]
    async fn cache_clears_if_expired_only() {
        let cache = ExpiringCache::new(Duration::from_secs(10));
        assert!(cache
            .yield_or_clear_if_expired(epoch_secs(100))
            .await
            .is_none());

        cache.get_or_load(|| async { identity(100) }).await.unwrap();
        assert_eq!(
            Some(epoch_secs(100)),
            cache.get().await.unwrap().expiration()
        );

        // It should not clear the credentials if they're not expired
        assert_eq!(
            Some(epoch_secs(100)),
            cache
                .yield_or_clear_if_expired(epoch_secs(10))
                .await
                .unwrap()
                .expiration()
        );

        // It should clear the credentials if they're expired
        assert!(cache
            .yield_or_clear_if_expired(epoch_secs(500))
            .await
            .is_none());
        assert!(cache.get().await.is_none());
    }
}
