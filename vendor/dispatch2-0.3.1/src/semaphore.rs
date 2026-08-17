use core::mem::ManuallyDrop;

use crate::{DispatchObject, DispatchRetained};

use crate::{DispatchTime, WaitError};

dispatch_object!(
    /// Dispatch semaphore.
    #[doc(alias = "dispatch_semaphore_t")]
    #[doc(alias = "dispatch_semaphore_s")]
    pub struct DispatchSemaphore;
);

dispatch_object_not_data!(unsafe DispatchSemaphore);

impl DispatchSemaphore {
    /// Attempt to acquire the [`DispatchSemaphore`] and return a [`DispatchSemaphoreGuard`].
    ///
    /// # Errors
    ///
    /// Return [WaitError::TimeOverflow] if the passed ``timeout`` is too big.
    ///
    /// Return [WaitError::Timeout] in case of timeout.
    #[inline]
    pub fn try_acquire(&self, timeout: DispatchTime) -> Result<DispatchSemaphoreGuard, WaitError> {
        // Safety: DispatchSemaphore cannot be null.
        let result = Self::wait(self, timeout);

        match result {
            0 => Ok(DispatchSemaphoreGuard(ManuallyDrop::new(self.retain()))),
            _ => Err(WaitError::Timeout),
        }
    }
}

/// Dispatch semaphore guard.
///
/// The semaphore will be signaled when the guard is dropped, or when [`release`] is called.
///
/// [`release`]: DispatchSemaphoreGuard::release
//
// See `release` for discussion of the `ManuallyDrop`.
#[derive(Debug)]
pub struct DispatchSemaphoreGuard(ManuallyDrop<DispatchRetained<DispatchSemaphore>>);

impl DispatchSemaphoreGuard {
    /// Release the [`DispatchSemaphore`].
    #[inline]
    pub fn release(self) -> bool {
        // We suppress `Drop` for the guard because that would signal the sempahore again.
        // The inner `DispatchRetained` is wrapped in `ManuallyDrop` so that it can be
        // separated from the guard and dropped normally.
        let mut this = ManuallyDrop::new(self);
        // SAFETY: The guard is being consumed; the `ManuallyDrop` contents will not be used again.
        let semaphore = unsafe { ManuallyDrop::take(&mut this.0) };
        semaphore.signal() != 0
    }
}

impl Drop for DispatchSemaphoreGuard {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: The guard is being dropped; the `ManuallyDrop` contents will not be used again.
        let semaphore = unsafe { ManuallyDrop::take(&mut self.0) };
        semaphore.signal();
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[cfg(feature = "objc2")]
    #[cfg_attr(
        not(target_vendor = "apple"),
        ignore = "only Apple's libdisptch is interoperable with `objc2`"
    )]
    fn acquire_release() {
        fn retain_count(semaphore: &DispatchSemaphore) -> usize {
            // SAFETY: semaphore is valid and the method signature is correct.
            unsafe { objc2::msg_send![semaphore, retainCount] }
        }

        let semaphore = DispatchSemaphore::new(1);

        assert_eq!(retain_count(&semaphore), 1);
        {
            let _guard = semaphore.try_acquire(DispatchTime::NOW).unwrap();
            assert_eq!(retain_count(&semaphore), 2);
        }
        assert_eq!(retain_count(&semaphore), 1);
        {
            let guard = semaphore.try_acquire(DispatchTime::NOW).unwrap();
            assert_eq!(retain_count(&semaphore), 2);
            guard.release();
            assert_eq!(retain_count(&semaphore), 1);
        }
        assert_eq!(retain_count(&semaphore), 1);
    }
}
