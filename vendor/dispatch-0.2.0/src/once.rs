use std::cell::UnsafeCell;

use crate::ffi::*;
use crate::context_and_sync_function;

/// A predicate used to execute a closure only once for the lifetime of an
/// application.
#[derive(Debug)]
pub struct Once {
    predicate: UnsafeCell<dispatch_once_t>,
}

impl Once {
    /// Creates a new `Once`.
    pub const fn new() -> Once {
        Once { predicate: UnsafeCell::new(0) }
    }

    /// Executes a closure once, ensuring that no other closure has been or
    /// will be executed by self for the lifetype of the application.
    ///
    /// If called simultaneously from multiple threads, waits synchronously
    // until the work has completed.
    #[inline(always)]
    pub fn call_once<F>(&'static self, work: F) where F: FnOnce() {
        #[cold]
        #[inline(never)]
        fn once<F>(predicate: *mut dispatch_once_t, work: F)
                where F: FnOnce() {
            let mut work = Some(work);
            let (context, work) = context_and_sync_function(&mut work);
            unsafe {
                dispatch_once_f(predicate, context, work);
            }
        }

        unsafe {
            let predicate = self.predicate.get();
            if *predicate != !0 {
                once(predicate, work);
            }
        }
    }
}

unsafe impl Sync for Once { }

#[cfg(test)]
mod tests {
    use super::Once;

    #[test]
    fn test_once() {
        static ONCE: Once = Once::new();
        let mut num = 0;
        ONCE.call_once(|| num += 1);
        ONCE.call_once(|| num += 1);
        assert!(num == 1);
    }
}
