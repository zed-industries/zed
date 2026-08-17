use super::{MemoryLimitExceededError, SharedMemoryLimiter};

/// Preallocated region of memory that can grow and never deallocates during the lifetime of
/// the limiter.
#[derive(Debug)]
pub(crate) struct Arena {
    limiter: SharedMemoryLimiter,
    data: Vec<u8>,
}

impl Arena {
    pub fn new(limiter: SharedMemoryLimiter, preallocated_size: usize) -> Self {
        let mut data = Vec::new();

        let preallocated = limiter
            .increase_usage(preallocated_size)
            .ok()
            .and_then(|()| data.try_reserve_exact(preallocated_size).ok())
            .is_some();
        // HtmlRewriter::new() has no way to report this
        debug_assert!(
            preallocated,
            "Total preallocated memory size should be less than `MemorySettings::max_allowed_memory_usage`."
        );

        Self { limiter, data }
    }

    pub fn append(&mut self, slice: &[u8]) -> Result<(), MemoryLimitExceededError> {
        // this specific form of capacity check optimizes out redundant resizing in extend_from_slice
        if self.data.capacity() - self.data.len() < slice.len() {
            let additional = slice.len() + self.data.len() - self.data.capacity();

            // NOTE: approximate usage, as `Vec::(try_)reserve_exact` doesn't
            // give guarantees about exact capacity value :).
            self.limiter.increase_usage(additional)?;

            // NOTE: with wisely chosen preallocated size this branch should be
            // executed quite rarely. We can't afford to use double capacity
            // strategy used by default (see: https://github.com/rust-lang/rust/blob/bdfd698f37184da42254a03ed466ab1f90e6fb6c/src/liballoc/raw_vec.rs#L424)
            // as we'll run out of the space allowance quite quickly.
            self.data
                .try_reserve_exact(slice.len())
                .map_err(|_| MemoryLimitExceededError)?;
        }

        self.data.extend_from_slice(slice);

        Ok(())
    }

    pub fn init_with(&mut self, slice: &[u8]) -> Result<(), MemoryLimitExceededError> {
        self.data.clear();
        self.append(slice)
    }

    pub fn shift(&mut self, byte_count: usize) {
        self.data.copy_within(byte_count.., 0);
        self.data.truncate(self.data.len() - byte_count);
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::super::limiter::SharedMemoryLimiter;
    use super::*;

    #[test]
    fn append() {
        let limiter = SharedMemoryLimiter::new(10);
        let mut arena = Arena::new(limiter.clone(), 2);

        arena.append(&[1, 2]).unwrap();
        assert_eq!(arena.bytes(), &[1, 2]);
        assert_eq!(limiter.current_usage(), 2);

        arena.append(&[3, 4]).unwrap();
        assert_eq!(arena.bytes(), &[1, 2, 3, 4]);
        assert_eq!(limiter.current_usage(), 4);

        arena.append(&[5, 6, 7, 8, 9, 10]).unwrap();
        assert_eq!(arena.bytes(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(limiter.current_usage(), 10);

        let err = arena.append(&[11]).unwrap_err();

        assert_eq!(err, MemoryLimitExceededError);
    }

    #[test]
    fn init_with() {
        let limiter = SharedMemoryLimiter::new(5);
        let mut arena = Arena::new(limiter.clone(), 0);

        arena.init_with(&[1]).unwrap();
        assert_eq!(arena.bytes(), &[1]);
        assert_eq!(limiter.current_usage(), 1);

        arena.append(&[1, 2]).unwrap();
        assert_eq!(arena.bytes(), &[1, 1, 2]);
        assert_eq!(limiter.current_usage(), 3);

        arena.init_with(&[1, 2, 3]).unwrap();
        assert_eq!(arena.bytes(), &[1, 2, 3]);
        assert_eq!(limiter.current_usage(), 3);

        arena.init_with(&[]).unwrap();
        assert!(arena.bytes().is_empty());
        assert_eq!(limiter.current_usage(), 3);

        let err = arena.init_with(&[1, 2, 3, 4, 5, 6, 7]).unwrap_err();

        assert_eq!(err, MemoryLimitExceededError);
    }

    #[test]
    fn shift() {
        let limiter = SharedMemoryLimiter::new(10);
        let mut arena = Arena::new(limiter.clone(), 0);

        arena.append(&[0, 1, 2, 3]).unwrap();
        arena.shift(2);
        assert_eq!(arena.bytes(), &[2, 3]);
        assert_eq!(limiter.current_usage(), 4);

        arena.append(&[0, 1]).unwrap();
        assert_eq!(arena.bytes(), &[2, 3, 0, 1]);
        assert_eq!(limiter.current_usage(), 4);

        arena.shift(3);
        assert_eq!(arena.bytes(), &[1]);
        assert_eq!(limiter.current_usage(), 4);

        arena.append(&[2, 3, 4, 5]).unwrap();
        arena.shift(1);
        assert_eq!(arena.bytes(), &[2, 3, 4, 5]);
        assert_eq!(limiter.current_usage(), 5);
    }
}
