#![cfg(feature = "NSString")]
use alloc::format;
use alloc::string::ToString;

use objc2::rc::autoreleasepool;

use crate::Foundation::{ns_string, NSString};

#[test]
fn test_equality() {
    let s1 = NSString::from_str("abc");
    let s2 = NSString::from_str("abc");
    assert_eq!(s1, s1);
    assert_eq!(s1, s2);

    let s3 = NSString::from_str("def");
    assert_ne!(s1, s3);
}

#[test]
fn display_debug() {
    let s = NSString::from_str("xyz\"123");
    assert_eq!(format!("{s}"), "xyz\"123");
    assert_eq!(format!("{s:?}"), r#""xyz\"123""#);
}

#[test]
fn test_empty() {
    let s1 = NSString::from_str("");
    let s2 = NSString::new();

    assert_eq!(s1.len(), 0);
    assert_eq!(s2.len(), 0);

    assert_eq!(s1, s2);

    autoreleasepool(|pool| {
        assert_eq!(s1.as_str(pool), "");
        assert_eq!(s2.as_str(pool), "");
    });
}

#[test]
fn test_utf8() {
    let expected = "ประเทศไทย中华Việt Nam";
    let s = NSString::from_str(expected);
    assert_eq!(s.len(), expected.len());
    autoreleasepool(|pool| {
        assert_eq!(s.as_str(pool), expected);
    });
}

#[test]
fn test_nul() {
    let expected = "\0";
    let s = NSString::from_str(expected);
    assert_eq!(s.len(), expected.len());
    autoreleasepool(|pool| {
        assert_eq!(s.as_str(pool), expected);
    });
}

#[test]
fn test_interior_nul() {
    let expected = "Hello\0World";
    let s = NSString::from_str(expected);
    assert_eq!(s.len(), expected.len());
    autoreleasepool(|pool| {
        assert_eq!(s.as_str(pool), expected);
    });
}

#[test]
#[cfg(feature = "NSObject")]
fn test_copy() {
    use crate::Foundation::{NSCopying, NSMutableCopying, NSMutableString, NSObjectProtocol};
    use objc2::rc::Retained;

    let s1 = NSString::from_str("abc");
    let s2 = s1.copy();
    // An optimization that NSString makes, since it is immutable
    assert_eq!(Retained::as_ptr(&s1), Retained::as_ptr(&s2));
    assert!(s2.is_kind_of::<NSString>());

    let s3 = s1.mutableCopy();
    assert_ne!(Retained::as_ptr(&s1), Retained::as_ptr(&s3).cast());
    assert!(s3.is_kind_of::<NSMutableString>());
}

#[test]
#[cfg(feature = "NSObject")]
fn test_copy_nsstring_is_same() {
    use crate::Foundation::NSCopying;

    let string1 = NSString::from_str("Hello, world!");
    let string2 = string1.copy();
    assert!(
        core::ptr::eq(&*string1, &*string2),
        "Cloned NSString didn't have the same address"
    );
}

#[test]
/// Apparently NSString does this for some reason?
fn test_strips_first_leading_zero_width_no_break_space() {
    let ns_string = NSString::from_str("\u{feff}");
    let expected = "";
    autoreleasepool(|pool| {
        assert_eq!(ns_string.as_str(pool), expected);
    });
    assert_eq!(ns_string.len(), 0);

    let s = "\u{feff}\u{feff}a\u{feff}";

    // Huh, this difference might be a GNUStep bug?
    let expected = if cfg!(feature = "gnustep-1-7") {
        "a\u{feff}"
    } else {
        "\u{feff}a\u{feff}"
    };

    let ns_string = NSString::from_str(s);
    autoreleasepool(|pool| {
        assert_eq!(ns_string.as_str(pool), expected);
    });
    assert_eq!(ns_string.len(), expected.len());
}

#[test]
fn test_hash() {
    use core::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    let s1 = NSString::from_str("example string goes here");
    let s2 = NSString::from_str("example string goes here");

    let mut hashstate = DefaultHasher::new();
    let mut hashstate2 = DefaultHasher::new();

    s1.hash(&mut hashstate);
    s2.hash(&mut hashstate2);

    assert_eq!(hashstate.finish(), hashstate2.finish());
}

#[test]
fn test_prefix_suffix() {
    let s = NSString::from_str("abcdef");
    let prefix = NSString::from_str("abc");
    let suffix = NSString::from_str("def");
    assert!(s.hasPrefix(&prefix));
    assert!(s.hasSuffix(&suffix));
    assert!(!s.hasPrefix(&suffix));
    assert!(!s.hasSuffix(&prefix));
}

#[test]
#[allow(clippy::nonminimal_bool)]
#[cfg(feature = "NSObjCRuntime")]
fn test_cmp() {
    let s1 = NSString::from_str("aa");
    assert!(s1 <= s1);
    assert!(s1 >= s1);
    let s2 = NSString::from_str("ab");
    assert!(s1 < s2);
    assert!(!(s1 > s2));
    assert!(s1 <= s2);
    assert!(!(s1 >= s2));
    let s3 = NSString::from_str("ba");
    assert!(s1 < s3);
    assert!(!(s1 > s3));
    assert!(s1 <= s3);
    assert!(!(s1 >= s3));
    assert!(s2 < s3);
    assert!(!(s2 > s3));
    assert!(s2 <= s3);
    assert!(!(s2 >= s3));

    let s = NSString::from_str("abc");
    let shorter = NSString::from_str("a");
    let longer = NSString::from_str("abcdef");
    assert!(s > shorter);
    assert!(s < longer);
}

#[test]
#[cfg(feature = "NSPathUtilities")]
fn test_append() {
    let error_tag = NSString::from_str("Error: ");
    let error_string = NSString::from_str("premature end of file.");
    let error_message = error_tag.stringByAppendingString(&error_string);
    assert_eq!(
        error_message,
        NSString::from_str("Error: premature end of file.")
    );

    let extension = NSString::from_str("scratch.tiff");
    assert_eq!(
        NSString::from_str("/tmp").stringByAppendingPathComponent(&extension),
        NSString::from_str("/tmp/scratch.tiff")
    );
    assert_eq!(
        NSString::from_str("/tmp/").stringByAppendingPathComponent(&extension),
        NSString::from_str("/tmp/scratch.tiff")
    );
    assert_eq!(
        NSString::from_str("/").stringByAppendingPathComponent(&extension),
        NSString::from_str("/scratch.tiff")
    );
    assert_eq!(
        NSString::from_str("").stringByAppendingPathComponent(&extension),
        NSString::from_str("scratch.tiff")
    );
}

#[test]
fn test_macro() {
    macro_rules! test {
        ($($s:expr,)+) => {$({
            let s1 = ns_string!($s);
            let s2 = NSString::from_str($s);

            assert_eq!(s1, s1);
            assert_eq!(s1, &*s2);

            assert_eq!(s1.to_string(), $s);
            assert_eq!(s2.to_string(), $s);
        })+};
    }

    test! {
        "",
        "asdf",
        "🦀",
        "🏳️‍🌈",
        "𝄞music",
        "abcd【e】fg",
        "abcd⒠fg",
        "ääääh",
        "lööps, bröther?",
        "\u{fffd} \u{fffd} \u{fffd}",
        "讓每個人都能打造出。",
        "\0",
        "\0\x01\x02\x03\x04\x05\x06\x07\x08\x09",
        // "\u{feff}", // TODO
        include_str!("string.rs"),
    }
}

#[test]
fn test_macro_in_unsafe() {
    // Test that the `unused_unsafe` lint doesn't trigger
    let s = unsafe {
        let s: *const NSString = ns_string!("abc");
        &*s
    };
    assert_eq!(s.to_string(), "abc");
}
