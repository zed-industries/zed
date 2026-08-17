use core::fmt;
use core::marker::PhantomData;
#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
use core::mem::{self, ManuallyDrop};
#[cfg(feature = "NSThread")]
use core::panic::{RefUnwindSafe, UnwindSafe};

#[cfg(feature = "NSThread")]
use crate::Foundation::NSThread;

use objc2::mutability::IsMainThreadOnly;
use objc2::rc::Allocated;
use objc2::{msg_send_id, ClassType};

#[cfg(feature = "NSThread")]
unsafe impl Send for NSThread {}
#[cfg(feature = "NSThread")]
unsafe impl Sync for NSThread {}

#[cfg(feature = "NSThread")]
impl UnwindSafe for NSThread {}
#[cfg(feature = "NSThread")]
impl RefUnwindSafe for NSThread {}

#[cfg(feature = "NSThread")]
impl fmt::Debug for NSThread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use -[NSThread description] since that includes the thread number
        let obj: &crate::Foundation::NSObject = self;
        fmt::Debug::fmt(obj, f)
    }
}

/// Whether the application is multithreaded according to Cocoa.
#[cfg(feature = "NSThread")]
pub fn is_multi_threaded() -> bool {
    NSThread::isMultiThreaded()
}

/// Whether the current thread is the main thread.
#[cfg(feature = "NSThread")]
#[inline]
pub fn is_main_thread() -> bool {
    #[cfg(not(feature = "gnustep-1-7"))]
    #[inline(always)]
    fn imp() -> bool {
        // Normally you would use NSThread::isMainThread, but that function uses pthread_main_np under
        // the hood. Benchmarks have shown that calling it directly is up to four times faster. So, we
        // just use that instead.

        #[cfg(feature = "libc")]
        use libc::pthread_main_np;

        #[link(name = "c", kind = "dylib")]
        #[cfg(not(feature = "libc"))]
        extern "C" {
            // Avoid a dependency on `libc` if possible
            fn pthread_main_np() -> std::os::raw::c_int;
        }

        // SAFETY: Does not affect thread safety if we're running in an actual macOS environment.
        unsafe { pthread_main_np() != 0 }
    }

    #[cfg(feature = "gnustep-1-7")]
    #[inline(always)]
    fn imp() -> bool {
        // Fall back to isMainThread on GNUStep, as pthread_main_np is not always available.
        NSThread::isMainThread_class()
    }

    imp()
}

#[allow(unused)]
#[cfg(feature = "NSThread")]
fn make_multithreaded() {
    let thread = unsafe { NSThread::new() };
    unsafe { thread.start() };
    // Don't bother waiting for it to complete!
}

/// Submit the given closure to the runloop on the main thread.
///
/// If the current thread is the main thread, this runs the closure.
///
/// The closure is passed a [`MainThreadMarker`] that it can further use
/// to access APIs that are only accessible from the main thread.
///
/// This function should only be used in applications whose main thread is
/// running an event loop with `dispatch_main`, `UIApplicationMain`,
/// `NSApplicationMain`, `CFRunLoop` or similar; it will block
/// indefinitely if that is not the case.
///
///
/// # Example
///
/// ```no_run
/// use objc2_foundation::run_on_main;
/// run_on_main(|mtm| {
///     // Do something on the main thread with the given marker
/// });
/// ```
#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
pub fn run_on_main<F, R>(f: F) -> R
where
    F: Send + FnOnce(MainThreadMarker) -> R,
    R: Send,
{
    if let Some(mtm) = MainThreadMarker::new() {
        f(mtm)
    } else {
        dispatch::Queue::main().exec_sync(|| {
            // SAFETY: The outer closure is submitted to run on the main
            // thread, so now, when the closure actually runs, it's
            // guaranteed to be on the main thread.
            f(unsafe { MainThreadMarker::new_unchecked() })
        })
    }
}

/// A marker type taken by functions that can only be executed on the main
/// thread.
///
/// By design, this is neither [`Send`] nor [`Sync`], and can only be created
/// on the main thread, meaning that if you're holding this, you know you're
/// running on the main thread.
///
/// See the following links for more information on main-thread-only APIs:
/// - [Main Thread Only APIs on OS X](https://www.dribin.org/dave/blog/archives/2009/02/01/main_thread_apis/)
/// - [Thread Safety Summary](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-SW1)
/// - [Are the Cocoa Frameworks Thread Safe?](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/AddingBehaviortoaCocoaProgram/AddingBehaviorCocoa.html#//apple_ref/doc/uid/TP40002974-CH5-SW47)
/// - [Technical Note TN2028 - Threading Architectures](https://developer.apple.com/library/archive/technotes/tn/tn2028.html#//apple_ref/doc/uid/DTS10003065)
/// - [Thread Management](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/CreatingThreads/CreatingThreads.html)
/// - [Mike Ash' article on thread safety](https://www.mikeash.com/pyblog/friday-qa-2009-01-09.html)
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
/// Use when designing APIs that are only safe to use on the main thread:
///
/// ```no_run
/// use objc2_foundation::{MainThreadMarker, NSObject};
/// use objc2::msg_send;
/// # let obj = 0 as *const NSObject;
///
/// // This action requires the main thread, so we take a marker as parameter.
/// // It signals clearly to users "this requires the main thread".
/// unsafe fn do_thing(obj: *const NSObject, _mtm: MainThreadMarker) {
///     msg_send![obj, someActionThatRequiresTheMainThread]
/// }
///
/// // Usage
///
/// // Create a new marker. This requires the `"NSThread"` feature.
/// // If that is not available, create the marker unsafely with
/// // `new_unchecked`, after having checked that the thread is the main one
/// // through other means.
/// #[cfg(feature = "NSThread")]
/// let mtm = MainThreadMarker::new().expect("must be on the main thread");
/// #[cfg(not(feature = "NSThread"))]
/// let mtm = unsafe { MainThreadMarker::new_unchecked() };
/// unsafe { do_thing(obj, mtm) }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// This is valid to Copy because it's still `!Send` and `!Sync`.
pub struct MainThreadMarker {
    // No lifetime information needed; the main thread is static and available
    // throughout the entire program!
    _priv: PhantomData<*mut ()>,
}

impl MainThreadMarker {
    /// Construct a new [`MainThreadMarker`].
    ///
    /// Returns [`None`] if the current thread was not the main thread.
    #[cfg(feature = "NSThread")]
    #[inline]
    pub fn new() -> Option<Self> {
        if is_main_thread() {
            // SAFETY: We just checked that we are running on the main thread.
            Some(unsafe { Self::new_unchecked() })
        } else {
            None
        }
    }

    /// Construct a new [`MainThreadMarker`] without first checking whether
    /// the current thread is the main one.
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
    pub unsafe fn new_unchecked() -> Self {
        // SAFETY: Upheld by caller
        //
        // We can't debug_assert that this actually is the main thread, see
        // the comment above.
        Self { _priv: PhantomData }
    }

    /// Allocate a new instance of the specified class on the main thread.
    ///
    /// This is essentially the same as [`ClassType::alloc`], the difference
    /// being that it is also callable with classes that can only be used on
    /// the main thread.
    ///
    ///
    /// # Example
    ///
    /// Create an object on the main thread.
    ///
    /// ```
    /// use objc2_foundation::MainThreadMarker;
    /// # use objc2_foundation::NSObject as SomeClass;
    /// # #[cfg(for_example)]
    /// use objc2_app_kit::NSView as SomeClass; // An example class
    /// use objc2::rc::Retained;
    /// use objc2::msg_send_id;
    ///
    /// # let mtm = unsafe { MainThreadMarker::new_unchecked() };
    /// # #[cfg(doctests_not_always_run_on_main_thread)]
    /// let mtm = MainThreadMarker::new().expect("must be on the main thread");
    ///
    /// // _All_ objects are safe to allocate on the main thread!
    /// let obj = mtm.alloc::<SomeClass>();
    ///
    /// // Though more knowledge is required for safe initialization
    /// let obj: Retained<SomeClass> = unsafe { msg_send_id![obj, init] };
    /// ```
    #[inline]
    pub fn alloc<T: ClassType>(self) -> Allocated<T> {
        // SAFETY: Same as `ClassType::alloc`, with the addition that since we
        // take `self: MainThreadMarker`, the `IsAllocableAnyThread` bound is
        // not required.
        unsafe { msg_send_id![T::class(), alloc] }
    }

    /// Submit the given closure to the runloop on the main thread.
    ///
    /// Deprecated in favour of the free-standing function [`run_on_main`].
    #[deprecated = "Use the free-standing function `run_on_main` instead"]
    #[cfg(feature = "dispatch")]
    #[cfg(feature = "NSThread")]
    pub fn run_on_main<F, R>(f: F) -> R
    where
        F: Send + FnOnce(MainThreadMarker) -> R,
        R: Send,
    {
        run_on_main(f)
    }
}

/// Get a [`MainThreadMarker`] from a main-thread-only object.
///
/// This function exists purely in the type-system, and will always
/// succeed at runtime.
impl<T: ?Sized + IsMainThreadOnly> From<&T> for MainThreadMarker {
    #[inline]
    fn from(_obj: &T) -> Self {
        // SAFETY: Objects which are `IsMainThreadOnly` are guaranteed
        // `!Send + !Sync` and are only constructible on the main thread.
        //
        // Since we hold a reference to such an object, and we know it cannot
        // now possibly be on another thread than the main, we know that the
        // current thread is the main thread.
        unsafe { Self::new_unchecked() }
    }
}

impl fmt::Debug for MainThreadMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MainThreadMarker").finish()
    }
}

/// Make a type that can only be used on the main thread be `Send` + `Sync`.
///
/// On `Drop`, the inner type is sent to the main thread's runloop and dropped
/// there. This may lead to deadlocks if the main runloop is not running, or
/// if it is waiting on a lock that the dropping thread is holding. See
/// [`run_on_main`] for some of the caveats around that.
///
///
/// # Related
///
/// This type takes inspiration from `threadbound::ThreadBound`.
///
/// The functionality also somewhat resembles Swift's `@MainActor`, which
/// ensures that a type is only usable from the main thread.
#[doc(alias = "@MainActor")]
#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
pub struct MainThreadBound<T>(ManuallyDrop<T>);

// SAFETY: The inner value is guaranteed to originate from the main thread
// because `new` takes [`MainThreadMarker`].
//
// `into_inner` is the only way to get the value out, and that is also
// guaranteed to happen on the main thread.
//
// Finally, the value is dropped on the main thread in `Drop`.
#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
unsafe impl<T> Send for MainThreadBound<T> {}

// SAFETY: We only provide access to the inner value via. `get` and `get_mut`.
//
// Both of these take [`MainThreadMarker`], which guarantees that the access
// is done from the main thread.
#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
unsafe impl<T> Sync for MainThreadBound<T> {}

#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
impl<T> Drop for MainThreadBound<T> {
    fn drop(&mut self) {
        if mem::needs_drop::<T>() {
            // TODO: Figure out whether we should assume the main thread to be
            // dead if we're panicking, and leak instead?
            run_on_main(|_mtm| {
                let this = self;
                // SAFETY: The value is dropped on the main thread, which is
                // the same thread that it originated from (guaranteed by
                // `new` taking `MainThreadMarker`).
                //
                // Additionally, the value is never used again after this
                // point.
                unsafe { ManuallyDrop::drop(&mut this.0) };
            })
        }
    }
}

/// Main functionality.
#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
impl<T> MainThreadBound<T> {
    /// Create a new [`MainThreadBound`] value of type `T`.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use objc2_foundation::{MainThreadMarker, MainThreadBound};
    ///
    /// let foo;
    /// # foo = ();
    /// let mtm = MainThreadMarker::new().expect("must be on the main thread");
    /// let foo = MainThreadBound::new(foo, mtm);
    ///
    /// // `foo` is now `Send + Sync`.
    /// ```
    #[inline]
    pub fn new(inner: T, _mtm: MainThreadMarker) -> Self {
        Self(ManuallyDrop::new(inner))
    }

    /// Returns a reference to the value.
    #[inline]
    pub fn get(&self, _mtm: MainThreadMarker) -> &T {
        &self.0
    }

    /// Returns a mutable reference to the value.
    #[inline]
    pub fn get_mut(&mut self, _mtm: MainThreadMarker) -> &mut T {
        &mut self.0
    }

    /// Extracts the value from the [`MainThreadBound`] container.
    #[inline]
    pub fn into_inner(self, _mtm: MainThreadMarker) -> T {
        // Prevent our `Drop` impl from running.
        //
        // This is a bit confusing, now `this` is:
        // `ManuallyDrop<Self(ManuallyDrop<T>)>`
        let mut this = ManuallyDrop::new(self);

        // SAFETY: `self` is consumed by this function, and wrapped in
        // `ManuallyDrop`, so the item's destructor is never run.
        unsafe { ManuallyDrop::take(&mut this.0) }
    }
}

/// Helper functions for running [`run_on_main`].
#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
impl<T> MainThreadBound<T> {
    /// Access the item on the main thread.
    ///
    /// See [`run_on_main`] for caveats.
    #[inline]
    pub fn get_on_main<F, R>(&self, f: F) -> R
    where
        F: Send + FnOnce(&T) -> R,
        R: Send,
    {
        run_on_main(|mtm| f(self.get(mtm)))
    }

    /// Access the item mutably on the main thread.
    ///
    /// See [`run_on_main`] for caveats.
    #[inline]
    pub fn get_on_main_mut<F, R>(&mut self, f: F) -> R
    where
        F: Send + FnOnce(&mut T) -> R,
        R: Send,
    {
        run_on_main(|mtm| f(self.get_mut(mtm)))
    }
}

#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
impl<T> fmt::Debug for MainThreadBound<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MainThreadBound").finish_non_exhaustive()
    }
}
