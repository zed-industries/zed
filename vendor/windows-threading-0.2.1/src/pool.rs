use super::*;
use core::{marker::PhantomData, ops::Deref};

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

    /// Convenience function for creating a new pool and calling [`scope`][Self::scope].
    pub fn with_scope<'env, F>(f: F)
    where
        F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>),
    {
        let pool = Pool::new();
        pool.scope(f);
    }

    /// Sets the thread limits for the `Pool` object.
    pub fn set_thread_limits(&self, min: u32, max: u32) {
        unsafe {
            check(SetThreadpoolThreadMinimum(self.0.Pool, min));
            SetThreadpoolThreadMaximum(self.0.Pool, max);
        }
    }

    /// Submit the closure to the thread pool.
    ///
    /// * The closure must have `'static` lifetime as the thread may outlive the lifetime in which `submit` is called.
    /// * The closure must be `Send` as it will be sent to another thread for execution.
    pub fn submit<F: FnOnce() + Send + 'static>(&self, f: F) {
        // This is safe because the closure has a `'static` lifetime.
        unsafe {
            try_submit(&*self.0, f);
        }
    }

    /// Create a scope for submitting closures.
    ///
    /// Within this scope local variables can be sent to the pool thread for execution.
    /// This is possible because `scope` will wait for all submitted closures to finish before returning,
    /// Note however that it will also wait for closures that were submitted from other threads.
    pub fn scope<'env, F>(&self, f: F)
    where
        F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>),
    {
        struct DropGuard<'a>(&'a Pool);
        impl Drop for DropGuard<'_> {
            fn drop(&mut self) {
                self.0.join();
            }
        }
        // Ensure that we always join the pool before returning.
        let _guard = DropGuard(self);
        let scope = Scope {
            pool: self,
            env: PhantomData,
            scope: PhantomData,
        };
        f(&scope);
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

/// A scope to submit closures in.
///
/// See [`scope`][Pool::scope] for details.
pub struct Scope<'scope, 'env: 'scope> {
    pool: &'scope Pool,
    scope: PhantomData<&'scope mut &'scope ()>,
    env: PhantomData<&'env mut &'env ()>,
}

impl<'scope, 'env> Scope<'scope, 'env> {
    /// Submits the closure to run on the `Pool`.
    ///
    /// The closure cannot outlive the `Scope` it's run in.
    pub fn submit<F: FnOnce() + Send + 'scope>(&'scope self, f: F) {
        unsafe {
            try_submit(&*self.pool.0, f);
        }
    }
}

impl Deref for Scope<'_, '_> {
    type Target = Pool;
    fn deref(&self) -> &Self::Target {
        self.pool
    }
}
