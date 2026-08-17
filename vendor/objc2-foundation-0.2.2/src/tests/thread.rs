#![cfg(feature = "NSThread")]
use alloc::format;

use crate::Foundation::{is_main_thread, MainThreadMarker, NSThread};

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "Retrieving main thread is weirdly broken, only works with --test-threads=1"
)]
fn test_main_thread() {
    let current = NSThread::currentThread();
    let main = NSThread::mainThread();

    assert!(main.isMainThread());

    if main == current {
        assert!(current.isMainThread());
        assert!(is_main_thread());
    } else {
        assert!(!current.isMainThread());
        assert!(!is_main_thread());
    }
}

#[test]
fn test_not_main_thread() {
    let res = std::thread::spawn(|| (is_main_thread(), NSThread::currentThread().isMainThread()))
        .join()
        .unwrap();
    assert_eq!(res, (false, false));
}

#[test]
fn test_main_thread_auto_traits() {
    use std::panic::{RefUnwindSafe, UnwindSafe};

    fn assert_traits<T: Unpin + UnwindSafe + RefUnwindSafe + Sized>() {}

    assert_traits::<MainThreadMarker>();
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "Retrieving main thread is weirdly broken, only works with --test-threads=1"
)]
fn test_debug() {
    let thread = NSThread::mainThread();

    let actual = format!("{thread:?}");
    let expected = [
        // macOS 11
        format!("<NSThread: {thread:p}>{{number = 1, name = (null)}}"),
        format!("<NSThread: {thread:p}>{{number = 1, name = main}}"),
        // macOS 12
        format!("<_NSMainThread: {thread:p}>{{number = 1, name = (null)}}"),
        format!("<_NSMainThread: {thread:p}>{{number = 1, name = main}}"),
    ];
    assert!(
        expected.contains(&actual),
        "Expected one of {expected:?}, got {actual:?}",
    );

    // SAFETY: We don't use the marker for anything other than its Debug
    // impl, so this test doesn't actually need to run on the main thread!
    let marker = unsafe { MainThreadMarker::new_unchecked() };
    assert_eq!(format!("{marker:?}"), "MainThreadMarker");
}

#[test]
#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
fn test_main_thread_bound_traits() {
    use crate::Foundation::MainThreadBound;

    struct Foo {
        _inner: *const (),
    }

    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<MainThreadBound<MainThreadMarker>>();
    assert_send_sync::<MainThreadBound<Foo>>();

    fn foo<T>() {
        assert_send_sync::<MainThreadBound<T>>();
    }

    foo::<()>();
}

#[test]
#[cfg(feature = "dispatch")]
#[cfg(feature = "NSThread")]
fn test_main_thread_bound_into_inner() {
    use crate::Foundation::MainThreadBound;
    use core::cell::Cell;

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
