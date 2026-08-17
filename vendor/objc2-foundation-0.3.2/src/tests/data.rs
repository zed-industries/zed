#![cfg(feature = "NSData")]
use alloc::format;

use crate::NSData;

#[test]
fn test_bytes() {
    let bytes = [3, 7, 16, 52, 112, 19];
    let data = NSData::with_bytes(&bytes);
    assert_eq!(data.len(), bytes.len());
    assert_eq!(data.to_vec(), bytes);
}

#[test]
fn test_no_bytes() {
    let data = NSData::new();
    assert!(Some(data.to_vec()).is_some());
}

#[cfg(feature = "block2")]
#[test]
fn test_from_vec() {
    let bytes = alloc::vec![3, 7, 16];
    let bytes_ptr = bytes.as_ptr();

    let data = NSData::from_vec(bytes);
    assert_eq!(unsafe { data.as_bytes_unchecked() }.as_ptr(), bytes_ptr);
}

#[test]
fn test_debug() {
    let bytes = [3, 7, 16, 52, 112, 19];
    let data = NSData::with_bytes(&bytes);
    assert_eq!(format!("{data:?}"), "[3, 7, 16, 52, 112, 19]");
}

#[cfg(feature = "block2")]
#[test]
fn test_collect() {
    let bytes = [3, 7, 16, 52, 112, 19];
    let data: objc2::rc::Retained<NSData> = bytes.into_iter().collect();
    assert_eq!(format!("{data:?}"), "[3, 7, 16, 52, 112, 19]");
}
