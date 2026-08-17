#![cfg(feature = "NSUUID")]
use alloc::format;

use crate::Foundation::NSUUID;
use objc2::rc::Retained;

#[test]
fn default_is_random() {
    let uuid1 = <Retained<NSUUID>>::default();
    let uuid2 = NSUUID::UUID();
    assert_ne!(uuid1, uuid2, "Statistically very unlikely");
}

#[test]
fn test_new() {
    let uuid1 = NSUUID::UUID();
    let uuid2 = NSUUID::UUID();
    assert_ne!(uuid1, uuid2, "Statistically very unlikely");
}

#[test]
#[cfg_attr(
    not(target_arch = "aarch64"),
    ignore = "encoding depends on Foundation version"
)]
fn test_bytes() {
    let uuid = NSUUID::from_bytes([10; 16]);
    assert_eq!(uuid.as_bytes(), [10; 16]);
}

#[test]
#[cfg(feature = "NSString")]
#[cfg_attr(
    not(target_arch = "aarch64"),
    ignore = "encoding depends on Foundation version"
)]
fn display_debug() {
    let uuid = NSUUID::from_bytes([10; 16]);
    let expected = "0A0A0A0A-0A0A-0A0A-0A0A-0A0A0A0A0A0A";
    assert_eq!(format!("{uuid}"), expected);
    assert_eq!(format!("{uuid:?}"), expected);
}

// #[test]
// fn test_compare() {
//     let uuid1 = NSUUID::from_bytes([10; 16]);
//     let uuid2 = NSUUID::from_bytes([9; 16]);
//     assert!(uuid1 > uuid2);
// }
