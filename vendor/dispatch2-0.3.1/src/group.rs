use alloc::boxed::Box;
use core::ffi::c_void;

use crate::generated::{dispatch_group_enter, dispatch_group_wait};
use crate::{DispatchObject, DispatchQueue, DispatchRetained, DispatchTime};

use super::utils::function_wrapper;
use super::WaitError;

dispatch_object!(
    /// Dispatch group.
    #[doc(alias = "dispatch_group_t")]
    #[doc(alias = "dispatch_group_s")]
    pub struct DispatchGroup;
);

dispatch_object_not_data!(unsafe DispatchGroup);

impl DispatchGroup {
    /// Submit a function to a [`DispatchQueue`] and associates it with the [`DispatchGroup`].
    #[inline]
    pub fn exec_async<F>(&self, queue: &DispatchQueue, work: F)
    where
        // We need `'static` to make sure any referenced values are borrowed for
        // long enough since `work` will be performed asynchronously.
        F: Send + FnOnce() + 'static,
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast::<c_void>();

        // Safety: All parameters cannot be null.
        unsafe { Self::exec_async_f(self, queue, work_boxed, function_wrapper::<F>) };
    }

    /// Wait synchronously for the previously submitted functions to finish.
    ///
    /// # Errors
    ///
    /// Return [WaitError::Timeout] in case of timeout.
    #[inline]
    pub fn wait(&self, timeout: DispatchTime) -> Result<(), WaitError> {
        let result = dispatch_group_wait(self, timeout);

        match result {
            0 => Ok(()),
            _ => Err(WaitError::Timeout),
        }
    }

    /// Schedule a function to be submitted to a [`DispatchQueue`] when a group of previously submitted functions have completed.
    #[inline]
    pub fn notify<F>(&self, queue: &DispatchQueue, work: F)
    where
        F: Send + FnOnce(),
    {
        let work_boxed = Box::into_raw(Box::new(work)).cast::<c_void>();

        // Safety: All parameters cannot be null.
        unsafe {
            Self::notify_f(self, queue, work_boxed, function_wrapper::<F>);
        }
    }

    /// Explicitly indicates that the function has entered the [`DispatchGroup`].
    #[inline]
    pub fn enter(&self) -> DispatchGroupGuard {
        // SAFETY: TODO: Is it a soundness requirement that this is paired with leave?
        unsafe { dispatch_group_enter(self) };

        DispatchGroupGuard(self.retain())
    }
}

/// Dispatch group guard.
#[derive(Debug)]
pub struct DispatchGroupGuard(DispatchRetained<DispatchGroup>);

impl DispatchGroupGuard {
    /// Explicitly indicate that the function in the [`DispatchGroup`] finished executing.
    #[inline]
    pub fn leave(self) {
        // Drop.
        let _ = self;
    }
}

impl Drop for DispatchGroupGuard {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: TODO: Is it a soundness requirement that this is paired with enter?
        unsafe { DispatchGroup::leave(&self.0) };
    }
}
