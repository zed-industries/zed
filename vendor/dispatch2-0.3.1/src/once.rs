use core::cell::UnsafeCell;
use core::ffi::c_void;
use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::NonNull;
use core::sync::atomic::{AtomicIsize, Ordering};

use crate::generated::dispatch_once_t;

/// A low-level synchronization primitive for one-time global execution.
///
/// This is equivalent to [`std::sync::Once`], except that this uses the
/// underlying system primitives from `libdispatch`, which:
/// - Might result in less code-size overhead.
/// - Aborts on panics in the initialization closure.
///
/// Generally, prefer [`std::sync::Once`] unless you have a specific need for
/// this.
///
///
/// # Example
///
/// Run a closure once for the duration of the entire program, without using
/// [`std::sync::Once`].
///
/// ```
/// use dispatch2::DispatchOnce;
///
/// static INIT: DispatchOnce = DispatchOnce::new();
///
/// INIT.call_once(|| {
///     // run initialization here
/// });
/// ```
///
#[cfg_attr(not(feature = "std"), doc = "[`std::sync::Once`]: #std-not-enabled")]
#[doc(alias = "dispatch_once_t")]
pub struct DispatchOnce {
    predicate: UnsafeCell<dispatch_once_t>,
}

// This is intentionally `extern "C"`, since libdispatch will not propagate an
// internal panic, but will simply abort.
extern "C" fn invoke_closure<F>(context: *mut c_void)
where
    F: FnOnce(),
{
    let context: *mut Option<F> = context.cast();
    // SAFETY: Context was created below in `invoke_dispatch_once` from
    // `&mut Option<F>`.
    let closure: &mut Option<F> = unsafe { &mut *context };

    // SAFETY: libdispatch is implemented correctly, and will only call this
    // once (and we set it to be available before calling dispatch_once).
    let closure = unsafe { closure.take().unwrap_unchecked() };

    (closure)();
}

#[cfg_attr(
    // DISPATCH_ONCE_INLINE_FASTPATH, see DispatchOnce::call_once below.
    any(target_arch = "x86", target_arch = "x86_64", target_vendor = "apple"),
    cold,
    inline(never)
)]
fn invoke_dispatch_once<F>(predicate: NonNull<dispatch_once_t>, closure: F)
where
    F: FnOnce(),
{
    // Convert closure data to context parameter.
    let mut closure = Some(closure);
    let context: *mut Option<F> = &mut closure;
    let context: *mut c_void = context.cast();

    // SAFETY: The function and context are valid, and the predicate pointer
    // is valid.
    //
    // NOTE: The documentation says:
    // > The predicate must point to a variable stored in global or static
    // > scope. The result of using a predicate with automatic or dynamic
    // > storage (including Objective-C instance variables) is undefined.
    //
    // In Rust though, we have stronger guarantees, and can guarantee that the
    // predicate is never moved while in use, because the `DispatchOnce`
    // itself is not cloneable.
    //
    // Even if libdispatch may sometimes use the pointer as a condition
    // variable, or may internally store a self-referential pointer, it can
    // only do that while the DispatchOnce is in use somewhere (i.e. it should
    // not be able to do that while the DispatchOnce is being moved).
    //
    // Outside of being moved, the DispatchOnce can only be in two states:
    // - Initialized.
    // - Done.
    //
    // And those two states are freely movable.
    unsafe { DispatchOnce::once_f(predicate, context, invoke_closure::<F>) };

    // Closure is dropped here, depending on if it was executed (and taken
    // from the `Option`) by `dispatch_once_f` or not.
}

impl DispatchOnce {
    /// Creates a new `DispatchOnce`.
    #[inline]
    #[allow(clippy::new_without_default)] // `std::sync::Once` doesn't have it either
    pub const fn new() -> Self {
        Self {
            predicate: UnsafeCell::new(0),
        }
    }

    /// Executes a closure once for the lifetime of the application.
    ///
    /// If called simultaneously from multiple threads, this function waits
    /// synchronously until the work function has completed.
    ///
    ///
    /// # Aborts
    ///
    /// The process will trap or abort if:
    /// - The given initialization closure unwinds.
    /// - The given closure recursively invokes `call_once` on the same
    ///   `DispatchOnce` instance.
    #[inline]
    #[doc(alias = "dispatch_once")]
    #[doc(alias = "dispatch_once_f")]
    pub fn call_once<F>(&self, work: F)
    where
        F: FnOnce(),
    {
        // Unwrap is fine, the pointer is valid so can never be NULL.
        let predicate = NonNull::new(self.predicate.get()).unwrap();

        // DISPATCH_ONCE_INLINE_FASTPATH
        if cfg!(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_vendor = "apple"
        )) {
            // On certain platforms, the ABI of the predicate is stable enough
            // that we are allowed to read it to check if the condition is
            // done yet.
            //
            // The code in C is inside `_dispatch_once_f` in dispatch/once.h:
            //
            //     if (DISPATCH_EXPECT(*predicate, ~0l) != ~0l) {
            //         dispatch_once_f(predicate, context, function);
            //     } else {
            //         dispatch_compiler_barrier();
            //     }
            //     DISPATCH_COMPILER_CAN_ASSUME(*predicate == ~0l);

            // NOTE: To uphold the rules set by the Rust AM, we use an atomic
            // comparison here to avoid a possible tear, even though the
            // equivalent C code just loads the predicate un-atomically.
            //
            // SAFETY: The predicate is a valid atomic pointer.
            // TODO: Use `AtomicIsize::from_ptr` once in MSRV.
            let atomic_predicate: &AtomicIsize = unsafe { predicate.cast().as_ref() };

            // We use an acquire load, as that's what's done internally in
            // libdispatch, and matches what's done in Rust's std too:
            // <https://github.com/swiftlang/swift-corelibs-libdispatch/blob/swift-6.0.3-RELEASE/src/once.c#L57>
            // <https://github.com/rust-lang/rust/blob/1.83.0/library/std/src/sys/sync/once/queue.rs#L130>
            if atomic_predicate.load(Ordering::Acquire) != !0 {
                invoke_dispatch_once(predicate, work);
            }

            // NOTE: Unlike in C, we cannot use `core::hint::assert_unchecked`,
            // since that would actually be lying from a language perspective;
            // the value seems to only settle on being !0 after some time
            // (something about the _COMM_PAGE_CPU_QUIESCENT_COUNTER?)
            //
            // TODO: Investigate this further!
            // core::hint::assert_unchecked(atomic_predicate.load(Ordering::Acquire) == !0);
        } else {
            invoke_dispatch_once(predicate, work);
        }
    }
}

// SAFETY: Same as `std::sync::Once`
unsafe impl Send for DispatchOnce {}

// SAFETY: Same as `std::sync::Once`
unsafe impl Sync for DispatchOnce {}

impl UnwindSafe for DispatchOnce {}
impl RefUnwindSafe for DispatchOnce {}

impl fmt::Debug for DispatchOnce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DispatchOnce").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use core::cell::Cell;
    use core::mem::ManuallyDrop;

    use super::*;

    #[test]
    fn test_static() {
        static ONCE: DispatchOnce = DispatchOnce::new();
        let mut num = 0;
        ONCE.call_once(|| num += 1);
        ONCE.call_once(|| num += 1);
        assert!(num == 1);
    }

    #[test]
    fn test_in_loop() {
        let once = DispatchOnce::new();

        let mut call_count = 0;
        for _ in 0..10 {
            once.call_once(|| call_count += 1);
        }

        assert_eq!(call_count, 1);
    }

    #[test]
    fn test_move() {
        let once = DispatchOnce::new();

        let mut call_count = 0;
        for _ in 0..10 {
            once.call_once(|| call_count += 1);
        }

        #[allow(clippy::redundant_locals)]
        let once = once;
        for _ in 0..10 {
            once.call_once(|| call_count += 1);
        }

        let once = DispatchOnce {
            predicate: UnsafeCell::new(once.predicate.into_inner()),
        };
        for _ in 0..10 {
            once.call_once(|| call_count += 1);
        }

        assert_eq!(call_count, 1);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_threaded() {
        let once = DispatchOnce::new();

        let num = AtomicIsize::new(0);

        std::thread::scope(|scope| {
            scope.spawn(|| {
                once.call_once(|| {
                    num.fetch_add(1, Ordering::Relaxed);
                });
            });
            scope.spawn(|| {
                once.call_once(|| {
                    num.fetch_add(1, Ordering::Relaxed);
                });
            });
            scope.spawn(|| {
                once.call_once(|| {
                    num.fetch_add(1, Ordering::Relaxed);
                });
            });
        });

        assert!(num.load(Ordering::Relaxed) == 1);
    }

    #[derive(Clone)]
    struct DropTest<'a>(&'a Cell<usize>);

    impl Drop for DropTest<'_> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn test_drop_in_closure() {
        let amount_of_drops = Cell::new(0);
        let once = DispatchOnce::new();

        let tester = DropTest(&amount_of_drops);
        once.call_once(move || {
            let _tester = tester;
        });
        assert_eq!(amount_of_drops.get(), 1);

        let tester = DropTest(&amount_of_drops);
        once.call_once(move || {
            let _tester = tester;
        });
        assert_eq!(amount_of_drops.get(), 2);
    }

    #[test]
    fn test_drop_in_closure_with_leak() {
        let amount_of_drops = Cell::new(0);
        let once = DispatchOnce::new();

        // Not dropped here, since we ManuallyDrop inside the closure (and the
        // closure is executed).
        let tester = DropTest(&amount_of_drops);
        once.call_once(move || {
            let _tester = ManuallyDrop::new(tester);
        });
        assert_eq!(amount_of_drops.get(), 0);

        // Still dropped here, since the once is not executed
        let tester = DropTest(&amount_of_drops);
        once.call_once(move || {
            let _tester = ManuallyDrop::new(tester);
        });
        assert_eq!(amount_of_drops.get(), 1);
    }

    #[test]
    #[ignore = "traps the process (as expected)"]
    fn test_recursive_invocation() {
        let once = DispatchOnce::new();

        once.call_once(|| {
            once.call_once(|| {});
        });
    }
}
