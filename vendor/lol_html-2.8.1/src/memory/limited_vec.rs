#![allow(clippy::len_without_is_empty)]

use std::mem::size_of;
use std::ops::{Deref, Index, RangeBounds};
use std::vec::Drain;

use super::{MemoryLimitExceededError, SharedMemoryLimiter};

#[derive(Debug)]
pub(crate) struct LimitedVec<T> {
    limiter: SharedMemoryLimiter,
    vec: Vec<T>,
}

impl<T> LimitedVec<T> {
    pub const fn new(limiter: SharedMemoryLimiter) -> Self {
        Self {
            vec: vec![],
            limiter,
        }
    }

    pub fn push(&mut self, element: T) -> Result<(), MemoryLimitExceededError> {
        #[allow(clippy::branches_sharing_code)]
        if self.vec.capacity() - self.vec.len() >= 1 {
            // the two push calls are optimized into one, but need to be two so each gets its own capacity hint
            self.vec.push(element);
        } else {
            // calculating the new capacity manually to check it before allocation
            let additional = self.vec.capacity().max(Self::min_capacity());
            let new_capacity = self.vec.capacity() + additional;
            let additional_bytes = additional
                .checked_mul(size_of::<T>())
                .ok_or(MemoryLimitExceededError)?;
            self.limiter.increase_usage(additional_bytes)?;

            // exact to reserve what has been accounted for.
            // not bothering with decrease_usage on real OOM, since the library won't recover anyway
            self.vec
                .try_reserve_exact(additional)
                .map_err(|_| MemoryLimitExceededError)?;
            debug_assert_eq!(new_capacity, self.vec.capacity());
            self.vec.push(element);
        }
        Ok(())
    }

    /// Returns the number of elements in the vector, also referred to as its 'length'.
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns the last element of the slice, or None if it is empty.
    #[inline]
    pub fn last(&self) -> Option<&T> {
        self.vec.last()
    }

    /// Returns a mutable pointer to the last item in the slice.
    #[inline]
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.vec.last_mut()
    }

    /// Creates a draining iterator that removes the specified range in the
    /// vector and yields the removed items.
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T>
    where
        R: RangeBounds<usize>,
    {
        self.vec.drain(range)
    }

    const fn min_capacity() -> usize {
        let items = 128 / size_of::<T>();
        if items >= 8 { items } else { 8 }
    }
}

impl<T> Deref for LimitedVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T> Index<usize> for LimitedVec<T> {
    type Output = T;

    #[inline]
    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.vec, index)
    }
}

impl<T> Drop for LimitedVec<T> {
    fn drop(&mut self) {
        self.limiter
            .decrease_usage(size_of::<T>() * self.vec.capacity());
    }
}

#[cfg(test)]
mod tests {
    use super::super::SharedMemoryLimiter;
    use super::*;

    fn test_ty<T: Copy + Default>() -> (LimitedVec<T>, SharedMemoryLimiter) {
        let initial_capacity = LimitedVec::<T>::min_capacity();
        // allow 2x capacity to be able to observe growth
        let limiter = SharedMemoryLimiter::new(2 * initial_capacity * size_of::<T>());
        let vec1 = LimitedVec::<T>::new(limiter.clone());
        let mut vec = LimitedVec::<T>::new(limiter.clone());

        assert_eq!(0, limiter.current_usage());
        assert_eq!(0, vec1.vec.capacity());
        drop(vec1);

        vec.push(Default::default()).unwrap();
        assert_eq!(initial_capacity, vec.vec.capacity());
        assert_eq!(
            limiter.current_usage(),
            vec.vec.capacity() * size_of::<T>(),
            "T={}",
            size_of::<T>()
        );
        drop(vec);

        assert_eq!(0, limiter.current_usage());
        let mut vec = LimitedVec::<T>::new(limiter.clone());
        for _ in 0..3 {
            vec.push(Default::default()).unwrap();
            assert_eq!(initial_capacity, vec.vec.capacity());
            assert_eq!(
                limiter.current_usage(),
                vec.vec.capacity() * size_of::<T>(),
                "T={}",
                size_of::<T>()
            );
        }

        vec.drain(1..);
        assert_eq!(limiter.current_usage(), vec.vec.capacity() * size_of::<T>());
        vec.drain(..);
        assert_eq!(limiter.current_usage(), vec.vec.capacity() * size_of::<T>());

        for _ in 0..initial_capacity {
            assert_eq!(initial_capacity, vec.vec.capacity());
            vec.push(Default::default()).unwrap();
            assert_eq!(limiter.current_usage(), vec.vec.capacity() * size_of::<T>());
        }

        for _ in 0..initial_capacity {
            vec.push(Default::default()).unwrap();
            assert_eq!(initial_capacity * 2, vec.vec.capacity());
            assert_eq!(limiter.current_usage(), vec.vec.capacity() * size_of::<T>());
        }
        (vec, limiter)
    }

    #[test]
    fn test_too_low_limit() {
        let mut vec = LimitedVec::<u8>::new(SharedMemoryLimiter::new(0));
        assert!(vec.push(0).is_err());

        let mut vec = LimitedVec::<u16>::new(SharedMemoryLimiter::new(1));
        assert!(vec.push(0).is_err());

        let mut vec = LimitedVec::<[u8; 257]>::new(SharedMemoryLimiter::new(256));
        assert!(vec.push([0; 257]).is_err());
    }

    #[test]
    fn test_limit() {
        let (mut vec, limiter) = test_ty::<u8>();
        assert!(vec.push(0).is_err());
        assert!(limiter.current_usage() >= vec.vec.capacity() * size_of::<u8>());

        let (mut vec, limiter) = test_ty::<u64>();
        assert!(vec.push(0).is_err());
        assert!(limiter.current_usage() >= vec.vec.capacity() * size_of::<u64>());

        let (mut vec, limiter) = test_ty::<[u8; 7]>();
        assert!(vec.push(Default::default()).is_err());
        assert!(limiter.current_usage() >= vec.vec.capacity() * size_of::<[u8; 7]>());

        let (mut vec, limiter) = test_ty::<[u128; 32]>();
        assert!(vec.push(Default::default()).is_err());
        assert!(limiter.current_usage() >= vec.vec.capacity() * size_of::<[u128; 32]>());
    }

    #[test]
    fn test_drop() {
        let (_, limiter) = test_ty::<u8>();
        assert_eq!(limiter.current_usage(), 0);

        let (_, limiter) = test_ty::<u64>();
        assert_eq!(limiter.current_usage(), 0);

        let (_, limiter) = test_ty::<[u8; 7]>();
        assert_eq!(limiter.current_usage(), 0);

        let (_, limiter) = test_ty::<[u128; 32]>();
        assert_eq!(limiter.current_usage(), 0);
    }
}
