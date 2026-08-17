//-
// Copyright 2024 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature = "handle-panics")]
mod internal {
    //! Implementation of scoped panic hooks
    //!
    //! 1. `with_hook` serves as entry point, it executes body closure with panic hook closure
    //!     installed as scoped panic hook
    //! 2. Upon first execution, current panic hook is replaced with `scoped_hook_dispatcher`
    //!     in a thread-safe manner, and original hook is stored for later use
    //! 3. When panic occurs, `scoped_hook_dispatcher` either delegates execution to scoped
    //!     panic hook, if one is installed, or back to original hook stored earlier.
    //!     This preserves original behavior when scoped hook isn't used
    //! 4. When `with_hook` is used, it replaces stored scoped hook pointer with pointer to
    //!     hook closure passed as parameter. Old hook pointer is set to be restored unconditionally
    //!     via drop guard. Then, normal body closure is executed.
    use std::boxed::Box;
    use std::cell::Cell;
    use std::panic::{set_hook, take_hook, PanicInfo};
    use std::sync::Once;
    use std::{mem, ptr};

    thread_local! {
        /// Pointer to currently installed scoped panic hook, if any
        ///
        /// NB: pointers to arbitrary fn's are fat, and Rust doesn't allow crafting null pointers
        /// to fat objects. So we just store const pointer to tuple with whatever data we need
        static SCOPED_HOOK_PTR: Cell<*const (*mut dyn FnMut(&PanicInfo<'_>),)> = Cell::new(ptr::null());
    }

    static INIT_ONCE: Once = Once::new();
    /// Default panic hook, the one which was present before installing scoped one
    ///
    /// NB: no need for external sync, value is mutated only once, when init is performed
    static mut DEFAULT_HOOK: Option<Box<dyn Fn(&PanicInfo<'_>) + Send + Sync>> =
        None;
    /// Replaces currently installed panic hook with `scoped_hook_dispatcher` once,
    /// in a thread-safe manner
    fn init() {
        INIT_ONCE.call_once(|| {
            let old_handler = take_hook();
            set_hook(Box::new(scoped_hook_dispatcher));
            unsafe {
                DEFAULT_HOOK = Some(old_handler);
            }
        });
    }
    /// Panic hook which delegates execution to scoped hook,
    /// if one installed, or to default hook
    fn scoped_hook_dispatcher(info: &PanicInfo<'_>) {
        let handler = SCOPED_HOOK_PTR.get();
        if !handler.is_null() {
            // It's assumed that if container's ptr is not null, ptr to `FnMut` is non-null too.
            // Correctness **must** be ensured by hook switch code in `with_hook`
            let hook = unsafe { &mut *(*handler).0 };
            (hook)(info);
            return;
        }

        #[allow(static_mut_refs)]
        if let Some(hook) = unsafe { DEFAULT_HOOK.as_ref() } {
            (hook)(info);
        }
    }
    /// Executes stored closure when dropped
    struct Finally<F: FnOnce()>(Option<F>);

    impl<F: FnOnce()> Finally<F> {
        fn new(body: F) -> Self {
            Self(Some(body))
        }
    }

    impl<F: FnOnce()> Drop for Finally<F> {
        fn drop(&mut self) {
            if let Some(body) = self.0.take() {
                body();
            }
        }
    }
    /// Executes main closure `body` while installing `guard` as scoped panic hook,
    /// for execution duration.
    ///
    /// Any panics which happen during execution of `body` are passed to `guard` hook
    /// to collect any info necessary, although unwind process is **NOT** interrupted.
    /// See module documentation for details
    ///
    /// # Parameters
    /// * `panic_hook` - scoped panic hook, functions for the duration of `body` execution
    /// * `body` - actual logic covered by `panic_hook`
    ///
    /// # Returns
    /// `body`'s return value
    pub fn with_hook<R>(
        mut panic_hook: impl FnMut(&PanicInfo<'_>),
        body: impl FnOnce() -> R,
    ) -> R {
        init();
        // Construct scoped hook pointer
        let guard_tuple = (unsafe {
            // `mem::transmute` is needed due to borrow checker restrictions to erase all lifetimes
            mem::transmute(&mut panic_hook as *mut dyn FnMut(&PanicInfo<'_>))
        },);
        let old_tuple = SCOPED_HOOK_PTR.replace(&guard_tuple);
        // Old scoped hook **must** be restored before leaving function scope to keep it sound
        let _undo = Finally::new(|| {
            SCOPED_HOOK_PTR.set(old_tuple);
        });
        body()
    }
}

#[cfg(not(feature = "handle-panics"))]
mod internal {
    use core::panic::PanicInfo;

    /// Simply executes `body` and returns its execution result.
    /// Hook parameter is ignored
    pub fn with_hook<R>(
        _: impl FnMut(&PanicInfo<'_>),
        body: impl FnOnce() -> R,
    ) -> R {
        body()
    }
}

pub use internal::with_hook;
