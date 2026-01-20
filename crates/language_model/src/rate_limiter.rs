use futures::Stream;
use smol::lock::{Semaphore, SemaphoreGuardArc};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use crate::LanguageModelCompletionError;

#[derive(Clone)]
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

pub struct RateLimitGuard<T> {
    inner: T,
    _guard: Option<SemaphoreGuardArc>,
}

impl<T> Stream for RateLimitGuard<T>
where
    T: Stream,
{
    type Item = T::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        unsafe { Pin::map_unchecked_mut(self, |this| &mut this.inner).poll_next(cx) }
    }
}

impl RateLimiter {
    pub fn new(limit: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(limit)),
        }
    }

    pub fn run<'a, Fut, T>(
        &self,
        future: Fut,
    ) -> impl 'a + Future<Output = Result<T, LanguageModelCompletionError>>
    where
        Fut: 'a + Future<Output = Result<T, LanguageModelCompletionError>>,
    {
        let guard = self.semaphore.acquire_arc();
        async move {
            let guard = guard.await;
            let result = future.await?;
            drop(guard);
            Ok(result)
        }
    }

    pub fn stream<'a, Fut, T>(
        &self,
        future: Fut,
    ) -> impl 'a
    + Future<
        Output = Result<impl Stream<Item = T::Item> + use<Fut, T>, LanguageModelCompletionError>,
    >
    where
        Fut: 'a + Future<Output = Result<T, LanguageModelCompletionError>>,
        T: Stream,
    {
        let guard = self.semaphore.acquire_arc();
        async move {
            let guard = guard.await;
            let inner = future.await?;
            Ok(RateLimitGuard {
                inner,
                _guard: Some(guard),
            })
        }
    }

    /// Like `stream`, but conditionally bypasses the rate limiter based on the flag.
    /// Used for nested requests (like edit agent requests) that are already "part of"
    /// a rate-limited request to avoid deadlocks.
    pub fn stream_with_bypass<'a, Fut, T>(
        &self,
        future: Fut,
        bypass: bool,
    ) -> impl 'a
    + Future<
        Output = Result<impl Stream<Item = T::Item> + use<Fut, T>, LanguageModelCompletionError>,
    >
    where
        Fut: 'a + Future<Output = Result<T, LanguageModelCompletionError>>,
        T: Stream,
    {
        let semaphore = self.semaphore.clone();
        async move {
            let guard = if bypass {
                None
            } else {
                Some(semaphore.acquire_arc().await)
            };
            let inner = future.await?;
            Ok(RateLimitGuard {
                inner,
                _guard: guard,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;
    use smol::lock::Barrier;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    /// Tests that nested requests without bypass_rate_limit cause deadlock,
    /// while requests with bypass_rate_limit complete successfully.
    ///
    /// This test simulates the scenario where multiple "parent" requests each
    /// try to spawn a "nested" request (like edit_file tool spawning an edit agent).
    /// With a rate limit of 2 and 2 parent requests, without bypass the nested
    /// requests would block forever waiting for permits that the parents hold.
    #[test]
    fn test_nested_requests_bypass_prevents_deadlock() {
        smol::block_on(async {
            // Use only 2 permits so we can guarantee deadlock conditions
            let rate_limiter = RateLimiter::new(2);
            let completed = Arc::new(AtomicUsize::new(0));
            // Barrier ensures all parents acquire permits before any tries nested request
            let barrier = Arc::new(Barrier::new(2));

            // Spawn 2 "parent" requests that each try to make a "nested" request
            let mut handles = Vec::new();
            for _ in 0..2 {
                let limiter = rate_limiter.clone();
                let completed = completed.clone();
                let barrier = barrier.clone();

                let handle = smol::spawn(async move {
                    // Parent request acquires a permit via stream_with_bypass (bypass=false)
                    let parent_stream = limiter
                        .stream_with_bypass(
                            async {
                                // Wait for all parents to acquire permits
                                barrier.wait().await;

                                // While holding the parent permit, make a nested request
                                // WITH bypass=true (simulating EditAgent behavior)
                                let nested_stream = limiter
                                    .stream_with_bypass(
                                        async { Ok(stream::iter(vec![1, 2, 3])) },
                                        true, // bypass - this is the key!
                                    )
                                    .await?;

                                // Consume the nested stream
                                use futures::StreamExt;
                                let _: Vec<_> = nested_stream.collect().await;

                                Ok(stream::iter(vec!["done"]))
                            },
                            false, // parent does NOT bypass
                        )
                        .await
                        .unwrap();

                    // Consume parent stream
                    use futures::StreamExt;
                    let _: Vec<_> = parent_stream.collect().await;

                    completed.fetch_add(1, Ordering::SeqCst);
                });
                handles.push(handle);
            }

            // With bypass=true for nested requests, this should complete quickly
            let timed_out = Arc::new(AtomicBool::new(false));
            let timed_out_clone = timed_out.clone();

            // Spawn a watchdog that sets timed_out after 2 seconds
            let watchdog = smol::spawn(async move {
                let start = Instant::now();
                while start.elapsed() < Duration::from_secs(2) {
                    smol::future::yield_now().await;
                }
                timed_out_clone.store(true, Ordering::SeqCst);
            });

            // Wait for all handles to complete
            for handle in handles {
                handle.await;
            }

            // Cancel the watchdog
            drop(watchdog);

            if timed_out.load(Ordering::SeqCst) {
                panic!(
                    "Test timed out - deadlock detected! This means bypass_rate_limit is not working."
                );
            }
            assert_eq!(completed.load(Ordering::SeqCst), 2);
        });
    }

    /// Tests that without bypass, nested requests DO cause deadlock.
    /// This test verifies the problem exists when bypass is not used.
    #[test]
    fn test_nested_requests_without_bypass_deadlocks() {
        smol::block_on(async {
            // Use only 2 permits so we can guarantee deadlock conditions
            let rate_limiter = RateLimiter::new(2);
            let completed = Arc::new(AtomicUsize::new(0));
            // Barrier ensures all parents acquire permits before any tries nested request
            let barrier = Arc::new(Barrier::new(2));

            // Spawn 2 "parent" requests that each try to make a "nested" request
            let mut handles = Vec::new();
            for _ in 0..2 {
                let limiter = rate_limiter.clone();
                let completed = completed.clone();
                let barrier = barrier.clone();

                let handle = smol::spawn(async move {
                    // Parent request acquires a permit
                    let parent_stream = limiter
                        .stream_with_bypass(
                            async {
                                // Wait for all parents to acquire permits - this guarantees
                                // that all 2 permits are held before any nested request starts
                                barrier.wait().await;

                                // Nested request WITHOUT bypass - this will deadlock!
                                // Both parents hold permits, so no permits available
                                let nested_stream = limiter
                                    .stream_with_bypass(
                                        async { Ok(stream::iter(vec![1, 2, 3])) },
                                        false, // NO bypass - will try to acquire permit
                                    )
                                    .await?;

                                use futures::StreamExt;
                                let _: Vec<_> = nested_stream.collect().await;

                                Ok(stream::iter(vec!["done"]))
                            },
                            false,
                        )
                        .await
                        .unwrap();

                    use futures::StreamExt;
                    let _: Vec<_> = parent_stream.collect().await;

                    completed.fetch_add(1, Ordering::SeqCst);
                });
                handles.push(handle);
            }

            // This SHOULD timeout because of deadlock (both parents hold permits,
            // both nested requests wait for permits)
            let timed_out = Arc::new(AtomicBool::new(false));
            let timed_out_clone = timed_out.clone();

            // Spawn a watchdog that sets timed_out after 100ms
            let watchdog = smol::spawn(async move {
                let start = Instant::now();
                while start.elapsed() < Duration::from_millis(100) {
                    smol::future::yield_now().await;
                }
                timed_out_clone.store(true, Ordering::SeqCst);
            });

            // Poll briefly to let everything run
            let start = Instant::now();
            while start.elapsed() < Duration::from_millis(100) {
                smol::future::yield_now().await;
            }

            // Cancel the watchdog
            drop(watchdog);

            // Expected - deadlock occurred, which proves the bypass is necessary
            let count = completed.load(Ordering::SeqCst);
            assert_eq!(
                count, 0,
                "Expected complete deadlock (0 completed) but {} requests completed",
                count
            );
        });
    }
}
