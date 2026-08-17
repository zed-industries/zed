#![cfg(feature = "NSData")]
use alloc::{format, vec};

use crate::Foundation::NSData;

#[test]
fn test_bytes() {
    let bytes = [3, 7, 16, 52, 112, 19];
    let data = NSData::with_bytes(&bytes);
    assert_eq!(data.len(), bytes.len());
    assert_eq!(data.bytes(), bytes);
}

#[test]
fn test_no_bytes() {
    let data = NSData::new();
    assert!(Some(data.bytes()).is_some());
}

#[cfg(feature = "block2")]
#[test]
fn test_from_vec() {
    let bytes = vec![3, 7, 16];
    let bytes_ptr = bytes.as_ptr();

    let data = NSData::from_vec(bytes);
    assert_eq!(data.bytes().as_ptr(), bytes_ptr);
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
