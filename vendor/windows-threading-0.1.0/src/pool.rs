use super::*;

/// A `Pool` object represents a private thread pool with its own thread limits.
///
/// This is in contrast to the default, or shared, thread pool used by the crate's `submit` function
/// as well as other code within the same process.
pub struct Pool(Box<TP_CALLBACK_ENVIRON_V3>);

impl Pool {
    /// Creates a new `Pool` object.
    pub fn new() -> Self {
        let mut e = TP_CALLBACK_ENVIRON_V3 {
            Version: 3,
            CallbackPriority: TP_CALLBACK_PRIORITY_NORMAL,
            Size: core::mem::size_of::<TP_CALLBACK_ENVIRON_V3>() as u32,
            ..Default::default()
        };

        unsafe {
            e.Pool = check(CreateThreadpool(core::ptr::null()));
            e.CleanupGroup = check(CreateThreadpoolCleanupGroup());
        }

        // The `TP_CALLBACK_ENVIRON_V3` is boxed to ensure its memory address remains stable for the life of the `Pool` object.
        Self(Box::new(e))
    }

    /// Sets the thread limits for the `Pool` object.
    pub fn set_thread_limits(&self, min: u32, max: u32) {
        unsafe {
            check(SetThreadpoolThreadMinimum(self.0.Pool, min));
            SetThreadpoolThreadMaximum(self.0.Pool, max);
        }
    }

    /// Submits the closure to run on the `Pool`.
    ///
    /// The closure cannot outlive the `Pool` on which it runs.
    pub fn submit<'a, F: FnOnce() + Send + 'a>(&'a self, f: F) {
        // This is safe because the lifetime of the closure is bounded by the `Pool`.
        unsafe {
            try_submit(&*self.0, f);
        }
    }

    /// Waits for all submissions to finish.
    ///
    /// Dropping the `Pool` will also wait for all submissions to finish.
    pub fn join(&self) {
        unsafe {
            CloseThreadpoolCleanupGroupMembers(self.0.CleanupGroup, 0, core::ptr::null_mut());
        }
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for Pool {}
unsafe impl Send for Pool {}

impl Drop for Pool {
    fn drop(&mut self) {
        // The `Pool` object cannot be dropped without waiting for all closures to complete, as their
        // lifetimes are only guaranteed to be as long as the `Pool` object.
        self.join();

        unsafe {
            CloseThreadpoolCleanupGroup(self.0.CleanupGroup);
            CloseThreadpool(self.0.Pool);
        }
    }
}
