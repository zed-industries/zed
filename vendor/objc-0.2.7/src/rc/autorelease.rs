use std::os::raw::c_void;
use runtime::{objc_autoreleasePoolPush, objc_autoreleasePoolPop};

// we use a struct to ensure that objc_autoreleasePoolPop during unwinding.
struct AutoReleaseHelper {
    context: *mut c_void,
}

impl AutoReleaseHelper {
    unsafe fn new() -> Self {
        AutoReleaseHelper { context: objc_autoreleasePoolPush() }
    }
}

impl Drop for AutoReleaseHelper {
    fn drop(&mut self) {
        unsafe { objc_autoreleasePoolPop(self.context) }
    }
}

/**
Execute `f` in the context of a new autorelease pool. The pool is drained
after the execution of `f` completes.

This corresponds to `@autoreleasepool` blocks in Objective-C and Swift.
*/
pub fn autoreleasepool<T, F: FnOnce() -> T>(f: F) -> T {
    let _context = unsafe { AutoReleaseHelper::new() };
    f()
}
