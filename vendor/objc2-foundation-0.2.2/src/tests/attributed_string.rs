#![cfg(feature = "NSAttributedString")]
#![cfg(feature = "NSString")]
use alloc::string::ToString;
use alloc::{format, vec};

use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::AnyObject;

use crate::Foundation::{self, NSAttributedString, NSObject, NSString};

#[test]
fn test_new() {
    let s = NSAttributedString::new();
    assert_eq!(&s.string().to_string(), "");
}

#[test]
fn test_string_bound_to_attributed() {
    let attr_s = {
        let source = NSString::from_str("Hello world!");
        NSAttributedString::from_nsstring(&source)
    };
    let s = autoreleasepool(|_| attr_s.string());
    assert_eq!(s.len(), 12);
}

#[test]
fn test_from_nsstring() {
    let s = NSAttributedString::from_nsstring(&NSString::from_str("abc"));
    assert_eq!(&s.string().to_string(), "abc");
}

#[test]
fn test_copy() {
    use Foundation::{NSCopying, NSMutableCopying, NSObjectProtocol};

    let s1 = NSAttributedString::from_nsstring(&NSString::from_str("abc"));
    let s2 = s1.copy();
    // NSAttributedString performs this optimization in GNUStep's runtime,
    // but not in Apple's; so we don't test for it!
    // assert_eq!(Retained::as_ptr(&s1), Retained::as_ptr(&s2));
    assert!(s2.is_kind_of::<NSAttributedString>());

    let s3 = s1.mutableCopy();
    assert_ne!(Retained::as_ptr(&s1), Retained::as_ptr(&s3).cast());
    assert!(s3.is_kind_of::<Foundation::NSMutableAttributedString>());
}

#[test]
#[cfg(feature = "NSDictionary")]
fn test_debug() {
    let s = NSAttributedString::from_nsstring(&NSString::from_str("abc"));
    let expected = if cfg!(feature = "gnustep-1-7") {
        "abc{}"
    } else {
        "abc{\n}"
    };
    assert_eq!(format!("{s:?}"), expected);

    let obj = Retained::into_super(NSObject::new());
    let ptr: *const AnyObject = &*obj;
    let s = unsafe {
        NSAttributedString::new_with_attributes(
            &NSString::from_str("abc"),
            &Foundation::NSDictionary::from_vec(&[&*NSString::from_str("test")], vec![obj]),
        )
    };
    let expected = if cfg!(feature = "gnustep-1-7") {
        format!("abc{{test = \"<NSObject: {ptr:?}>\"; }}")
    } else {
        format!("abc{{\n    test = \"<NSObject: {ptr:?}>\";\n}}")
    };
    assert_eq!(format!("{s:?}"), expected);
}

#[test]
fn test_new_mutable() {
    let s = Foundation::NSMutableAttributedString::new();
    assert_eq!(&s.string().to_string(), "");
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn test_copy_mutable() {
    use Foundation::{NSCopying, NSMutableCopying, NSObjectProtocol};

    let s1 = Foundation::NSMutableAttributedString::from_nsstring(&NSString::from_str("abc"));
    let s2 = s1.copy();
    assert_ne!(Retained::as_ptr(&s1).cast(), Retained::as_ptr(&s2));
    assert!(s2.is_kind_of::<NSAttributedString>());

    let s3 = s1.mutableCopy();
    assert_ne!(Retained::as_ptr(&s1), Retained::as_ptr(&s3));
    assert!(s3.is_kind_of::<Foundation::NSMutableAttributedString>());
}
