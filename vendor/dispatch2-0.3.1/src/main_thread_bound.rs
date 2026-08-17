use core::fmt;
use core::mem::{self, ManuallyDrop};

use objc2::MainThreadMarker;

use crate::DispatchQueue;

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
/// use dispatch2::run_on_main;
/// run_on_main(|mtm| {
///     // Do something on the main thread with the given marker
/// });
/// ```
pub fn run_on_main<F, R>(f: F) -> R
where
    F: Send + FnOnce(MainThreadMarker) -> R,
    R: Send,
{
    if let Some(mtm) = MainThreadMarker::new() {
        f(mtm)
    } else {
        let mut ret = None;
        DispatchQueue::main().exec_sync(|| {
            // SAFETY: The outer closure is submitted to run on the main
            // thread, so now, when the closure actually runs, it's
            // guaranteed to be on the main thread.
            ret = Some(f(unsafe { MainThreadMarker::new_unchecked() }))
        });
        ret.unwrap()
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
pub struct MainThreadBound<T>(ManuallyDrop<T>);

// SAFETY: The inner value is guaranteed to originate from the main thread
// because `new` takes [`MainThreadMarker`].
//
// `into_inner` is the only way to get the value out, and that is also
// guaranteed to happen on the main thread.
//
// Finally, the value is dropped on the main thread in `Drop`.
unsafe impl<T> Send for MainThreadBound<T> {}

// SAFETY: We only provide access to the inner value via `get` and `get_mut`.
//
// Both of these take [`MainThreadMarker`], which guarantees that the access
// is done from the main thread.
unsafe impl<T> Sync for MainThreadBound<T> {}

impl<T> Drop for MainThreadBound<T> {
    #[inline]
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
impl<T> MainThreadBound<T> {
    /// Create a new [`MainThreadBound`] value of type `T`.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use dispatch2::MainThreadBound;
    /// use objc2::MainThreadMarker;
    ///
    /// let foo;
    /// # foo = ();
    /// # let mtm = unsafe { MainThreadMarker::new_unchecked() };
    /// # #[cfg(doctests_not_always_run_on_main_thread)]
    /// let mtm = MainThreadMarker::new().expect("must be on the main thread");
    /// let foo = MainThreadBound::new(foo, mtm);
    ///
    /// // `foo` is now `Send + Sync`.
    /// ```
    ///
    /// Create a shared static that is only available from the main thread.
    ///
    /// ```
    /// use core::cell::Cell;
    /// use dispatch2::MainThreadBound;
    /// use objc2::MainThreadMarker;
    ///
    /// // Note: The destructor for this will never be run.
    /// static SHARED: MainThreadBound<Cell<i32>> = MainThreadBound::new(
    ///     Cell::new(42),
    ///     // SAFETY: The MainThreadBound is created at `const`-time and put
    ///     // into a static, there is no thread associated with this, and
    ///     // hence no thread safety to worry about.
    ///     unsafe { MainThreadMarker::new_unchecked() },
    /// );
    ///
    /// # let mtm = unsafe { MainThreadMarker::new_unchecked() };
    /// # #[cfg(doctests_not_always_run_on_main_thread)]
    /// let mtm = MainThreadMarker::new();
    ///
    /// assert_eq!(SHARED.get(mtm).get(), 42);
    /// SHARED.get(mtm).set(43);
    /// assert_eq!(SHARED.get(mtm).get(), 43);
    /// ```
    #[inline]
    pub const fn new(inner: T, _mtm: MainThreadMarker) -> Self {
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

impl<T> fmt::Debug for MainThreadBound<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MainThreadBound").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;

    static_assertions::assert_impl_all!(MainThreadBound<MainThreadMarker>: Send, Sync);
    static_assertions::assert_impl_all!(MainThreadBound<*const ()>: Send, Sync);

    #[test]
    fn always_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        fn foo<T>() {
            assert_send_sync::<MainThreadBound<T>>();
        }

        foo::<()>();
    }

    #[test]
    fn test_main_thread_bound_into_inner() {
        // SAFETY: For testing only
        let mtm = unsafe { MainThreadMarker::new_unchecked() };

        struct Foo<'a> {
            is_dropped: &'a Cell<bool>,
        }

        impl Drop for Foo<'_> {
            fn drop(&mut self) {
                self.is_dropped.set(true);
            }
        }

        let is_dropped = Cell::new(false);
        let foo = Foo {
            is_dropped: &is_dropped,
        };
        let foo = MainThreadBound::new(foo, mtm);
        assert!(!is_dropped.get());

        let foo = foo.into_inner(mtm);
        assert!(!is_dropped.get());

        drop(foo);
        assert!(is_dropped.get());
    }
}
