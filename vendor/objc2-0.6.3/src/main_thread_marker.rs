//! This belongs in `std` IMO, but wasn't accepted there:
//! <https://github.com/rust-lang/rust/pull/136616>
use core::fmt;
use core::marker::PhantomData;

use crate::rc::Allocated;
use crate::{ClassType, MainThreadOnly};

/// Whether the current thread is the main thread.
#[inline]
fn is_main_thread() -> bool {
    #[cfg(target_vendor = "apple")]
    {
        // Normally you would use `+[NSThread isMainThread]`, but benchmarks
        // have shown that calling the underlying `pthread_main_np` directly
        // is up to four times faster, so we use that instead.

        // SAFETY: The signature in here is the exact same as in `libc`.
        //
        // `pthread_main_np` is included via `libSystem` when `libstd` is
        // linked. All of this is done to avoid a dependency on the `libc`
        // crate.
        //
        // `extern "C"` is safe because this will never unwind.
        #[cfg_attr(not(feature = "std"), link(name = "c", kind = "dylib"))]
        extern "C" {
            fn pthread_main_np() -> core::ffi::c_int;
        }

        // SAFETY: Can be called from any thread.
        //
        // Apple's man page says:
        // > The pthread_main_np() function returns 1 if the calling thread is the initial thread, 0 if
        // > the calling thread is not the initial thread, and -1 if the thread's initialization has not
        // > yet completed.
        //
        // However, Apple's header says:
        // > Returns non-zero if the current thread is the main thread.
        //
        // So unclear if we should be doing a comparison against 1, or a negative comparison against 0?
        // To be safe, we compare against 1, though in reality, the current implementation can only ever
        // return 0 or 1:
        // https://github.com/apple-oss-distributions/libpthread/blob/libpthread-535/src/pthread.c#L1084-L1089
        unsafe { pthread_main_np() == 1 }
    }

    #[cfg(not(target_vendor = "apple"))]
    {
        // Fall back to isMainThread on non-Apple platforms, as
        // `pthread_main_np` is not always available there.
        unsafe { crate::msg_send![crate::class!(NSThread), isMainThread] }
    }
}

/// A marker type for functionality only available on the main thread.
///
/// The main thread is a system-level property on Apple/Darwin platforms, and
/// has extra capabilities not available on other threads. This is usually
/// relevant when using native GUI frameworks, where most operations must be
/// done on the main thread.
///
/// This type enables you to manage that capability. By design, it is neither
/// [`Send`] nor [`Sync`], and can only be created on the main thread, meaning
/// that if you have an instance of this, you are guaranteed to be on the main
/// thread / have the "main-thread capability".
///
/// [The `main` function][main-functions] will run on the main thread. This
/// type can also be used with `#![no_main]` or other such cases where Rust is
/// not defining the binary entry point.
///
/// See the following links for more information on main-thread-only APIs:
/// - [Are the Cocoa Frameworks Thread Safe?](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/AddingBehaviortoaCocoaProgram/AddingBehaviorCocoa.html#//apple_ref/doc/uid/TP40002974-CH5-SW47)
/// - [About Threaded Programming](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/AboutThreads/AboutThreads.html)
/// - [Thread Safety Summary](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-SW1)
/// - [Technical Note TN2028 - Threading Architectures](https://developer.apple.com/library/archive/technotes/tn/tn2028.html#//apple_ref/doc/uid/DTS10003065)
/// - [Thread Management](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/CreatingThreads/CreatingThreads.html)
/// - [Swift's `@MainActor`](https://developer.apple.com/documentation/swift/mainactor)
/// - [Main Thread Only APIs on OS X](https://www.dribin.org/dave/blog/archives/2009/02/01/main_thread_apis/)
/// - [Mike Ash' article on thread safety](https://www.mikeash.com/pyblog/friday-qa-2009-01-09.html)
///
/// [main-functions]: https://doc.rust-lang.org/reference/crates-and-source-files.html#main-functions
///
///
/// # Main Thread Checker
///
/// Xcode provides a tool called the ["Main Thread Checker"][mtc] which
/// verifies that UI APIs are being used from the correct thread. This is not
/// as principled as `MainThreadMarker`, but is helpful for catching mistakes.
///
/// You can use this tool on macOS by loading `libMainThreadChecker.dylib`
/// into your process using `DYLD_INSERT_LIBRARIES`:
///
/// ```console
/// DYLD_INSERT_LIBRARIES=/Applications/Xcode.app/Contents/Developer/usr/lib/libMainThreadChecker.dylib MTC_RESET_INSERT_LIBRARIES=0 cargo run
/// ```
///
/// If you're not running your binary through Cargo, you can omit
/// [`MTC_RESET_INSERT_LIBRARIES`][mtc-reset].
///
/// ```console
/// DYLD_INSERT_LIBRARIES=/Applications/Xcode.app/Contents/Developer/usr/lib/libMainThreadChecker.dylib target/debug/myapp
/// ```
///
/// If you're developing for iOS, you probably better off enabling the tool in
/// Xcode's own UI.
///
/// See [this excellent blog post][mtc-cfg] for details on further
/// configuration options.
///
/// [mtc]: https://developer.apple.com/documentation/xcode/diagnosing-memory-thread-and-crash-issues-early#Detect-improper-UI-updates-on-background-threads
/// [mtc-reset]: https://bryce.co/main-thread-checker-configuration/#mtc_reset_insert_libraries
/// [mtc-cfg]: https://bryce.co/main-thread-checker-configuration/
///
///
/// # Examples
///
/// Retrieve the main thread marker in different situations.
///
/// ```
/// use objc2::MainThreadMarker;
///
/// # // explicitly uses `fn main`
/// fn main() {
///     // The thread that `fn main` runs on is the main thread.
///     assert!(MainThreadMarker::new().is_some());
///
///     // Subsequently spawned threads are not the main thread.
///     std::thread::spawn(|| {
///         assert!(MainThreadMarker::new().is_none());
///     }).join().unwrap();
/// }
/// ```
///
/// Use when accessing APIs that are only safe to use on the main thread.
///
/// ```no_run
/// use objc2::MainThreadMarker;
/// # #[cfg(needs_app_kit)]
/// use objc2_app_kit::NSApplication;
/// #
/// # use objc2::runtime::NSObject as NSApplication;
/// # trait Foo {
/// #     fn sharedApplication(_mtm: MainThreadMarker) {}
/// # }
/// # impl Foo for NSApplication {}
///
/// # // explicitly uses `fn main`
/// fn main() {
///     // Create a new MainThreadMarker.
///     let mtm = MainThreadMarker::new().expect("must be on the main thread");
///
///     // NSApplication is only usable on the main thread,
///     // so we need to pass the marker as an argument.
///     let app = NSApplication::sharedApplication(mtm);
///
///     // Do something with the application
///     // app.run();
/// }
/// ```
///
/// Create a static that is only usable on the main thread. This is similar to
/// a thread-local, but can be more efficient because it doesn't handle
/// multiple threads.
///
/// See also `dispatch2::MainThreadBound`.
///
/// ```
/// use objc2::MainThreadMarker;
/// use std::cell::UnsafeCell;
///
/// struct SyncUnsafeCell<T>(UnsafeCell<T>);
///
/// unsafe impl<T> Sync for SyncUnsafeCell<T> {}
///
/// static MAIN_THREAD_ONLY_VALUE: SyncUnsafeCell<i32> = SyncUnsafeCell(UnsafeCell::new(0));
///
/// fn set(value: i32, _mtm: MainThreadMarker) {
///     // SAFETY: We have an instance of `MainThreadMarker`, so we know that
///     // we're running on the main thread (and thus do not need any
///     // synchronization, since the only accesses to this value is from the
///     // main thread).
///     unsafe { *MAIN_THREAD_ONLY_VALUE.0.get() = value };
/// }
///
/// fn get(_mtm: MainThreadMarker) -> i32 {
///     // SAFETY: Same as above.
///     unsafe { *MAIN_THREAD_ONLY_VALUE.0.get() }
/// }
///
/// # // explicitly uses `fn main`
/// fn main() {
///     let mtm = MainThreadMarker::new().expect("must be on the main thread");
///     set(42, mtm);
///     assert_eq!(get(mtm), 42);
/// }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
//              ^^^^ this is valid because it's still `!Send` and `!Sync`.
pub struct MainThreadMarker {
    // No lifetime information needed; the main thread is static and available
    // throughout the entire program!

    // Ensure `!Send` and `!Sync`.
    _priv: PhantomData<*mut ()>,
}

impl MainThreadMarker {
    /// Construct a new `MainThreadMarker`.
    ///
    /// Returns [`None`] if the current thread was not the main thread.
    ///
    ///
    /// # Example
    ///
    /// Check whether the current thread is the main thread.
    ///
    /// ```
    /// use objc2::MainThreadMarker;
    ///
    /// if MainThreadMarker::new().is_some() {
    ///     // Is the main thread
    /// } else {
    ///     // Not the main thread
    /// }
    /// ```
    #[inline]
    #[doc(alias = "is_main_thread")]
    #[doc(alias = "pthread_main_np")]
    #[doc(alias = "isMainThread")]
    pub fn new() -> Option<Self> {
        if is_main_thread() {
            // SAFETY: We just checked that we are running on the main thread.
            Some(unsafe { Self::new_unchecked() })
        } else {
            None
        }
    }

    /// Construct a new `MainThreadMarker` without first checking whether the
    /// current thread is the main one.
    ///
    ///
    /// # Safety
    ///
    /// The current thread must be the main thread.
    ///
    /// Alternatively, you may create this briefly if you know that a an API
    /// is safe in a specific case, but is not marked so. If you do that, you
    /// must ensure that any use of the marker is actually safe to do from
    /// another thread than the main one.
    #[inline]
    pub const unsafe fn new_unchecked() -> Self {
        // SAFETY: Upheld by caller.
        //
        // We can't debug_assert that this actually is the main thread, both
        // because this is `const` (to allow usage in `static`s), and because
        // users may sometimes want to create this briefly, e.g. to access an
        // API that in most cases requires the marker, but is safe to use
        // without in specific cases.
        Self { _priv: PhantomData }
    }

    /// Allocate a new instance of the specified class on the main thread.
    ///
    /// This can be useful in certain situations, such as generic contexts
    /// where you don't know whether the class is main thread or not, but
    /// usually you should prefer [`MainThreadOnly::alloc`].
    #[inline]
    pub fn alloc<T: ClassType>(self) -> Allocated<T> {
        // SAFETY: We hold `MainThreadMarker`, and classes are either only
        // safe to allocate on the main thread, or safe to allocate
        // everywhere.
        unsafe { Allocated::alloc(T::class()) }
    }
}

/// Get a [`MainThreadMarker`] from a main-thread-only object.
///
/// This is a shorthand for [`MainThreadOnly::mtm`].
impl<T: ?Sized + MainThreadOnly> From<&T> for MainThreadMarker {
    #[inline]
    fn from(obj: &T) -> Self {
        obj.mtm()
    }
}

impl fmt::Debug for MainThreadMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MainThreadMarker").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{RefUnwindSafe, UnwindSafe};

    static_assertions::assert_impl_all!(MainThreadMarker: Unpin, UnwindSafe, RefUnwindSafe, Sized);
    static_assertions::assert_not_impl_any!(MainThreadMarker: Send, Sync);

    #[test]
    fn debug() {
        // SAFETY: We don't use the marker for anything other than its Debug
        // impl, so this test doesn't actually need to run on the main thread!
        let marker = unsafe { MainThreadMarker::new_unchecked() };
        assert_eq!(std::format!("{marker:?}"), "MainThreadMarker");
    }

    #[test]
    fn test_not_main_thread() {
        let res = std::thread::spawn(|| MainThreadMarker::new().is_none())
            .join()
            .unwrap();
        assert!(res);
    }
}
