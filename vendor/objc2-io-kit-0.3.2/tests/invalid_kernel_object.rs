#![cfg(feature = "libc")]
use objc2_io_kit::{kIOReturnSuccess, IOObjectRelease, IOObjectRetain};

#[test]
fn release() {
    assert_ne!(IOObjectRelease(100), kIOReturnSuccess);
}

#[test]
fn retain() {
    assert_ne!(IOObjectRetain(100), kIOReturnSuccess);
}
