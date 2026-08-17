#![cfg(feature = "NSThread")]
use alloc::format;

use crate::NSThread;
use objc2::MainThreadMarker;

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
        assert!(MainThreadMarker::new().is_some());
    } else {
        assert!(!current.isMainThread());
        assert!(MainThreadMarker::new().is_none());
    }
}

#[test]
#[cfg(feature = "std")]
fn test_not_main_thread() {
    let res = std::thread::spawn(|| NSThread::currentThread().isMainThread())
        .join()
        .unwrap();
    assert!(!res);
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
}
