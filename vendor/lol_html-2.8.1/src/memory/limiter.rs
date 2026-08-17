use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;

/// An error that occures when rewriter exceedes the memory limit specified in the
/// [`MemorySettings`].
///
/// [`MemorySettings`]: ../struct.MemorySettings.html
#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
#[error("The memory limit has been exceeded.")]
pub struct MemoryLimitExceededError;

// Pub only for integration tests
#[derive(Debug, Clone)]
pub struct SharedMemoryLimiter {
    current_usage: Arc<AtomicUsize>,
    max: usize,
}

impl SharedMemoryLimiter {
    #[must_use]
    pub fn new(max: usize) -> Self {
        Self {
            current_usage: Arc::new(AtomicUsize::new(0)),
            max,
        }
    }

    #[cfg(test)]
    #[must_use]
    pub fn current_usage(&self) -> usize {
        self.current_usage.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn increase_usage(&self, byte_count: usize) -> Result<(), MemoryLimitExceededError> {
        let previous_usage = self.current_usage.fetch_add(byte_count, Ordering::Relaxed);
        let current_usage = previous_usage + byte_count;

        if current_usage > self.max {
            Err(MemoryLimitExceededError)
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn decrease_usage(&self, byte_count: usize) {
        self.current_usage.fetch_sub(byte_count, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_usage() {
        let limiter = SharedMemoryLimiter::new(10);

        assert_eq!(limiter.current_usage(), 0);

        limiter.increase_usage(3).unwrap();
        assert_eq!(limiter.current_usage(), 3);

        limiter.increase_usage(5).unwrap();
        assert_eq!(limiter.current_usage(), 8);

        limiter.decrease_usage(4);
        assert_eq!(limiter.current_usage(), 4);

        let err = limiter.increase_usage(15).unwrap_err();

        assert_eq!(err, MemoryLimitExceededError);
    }
}
