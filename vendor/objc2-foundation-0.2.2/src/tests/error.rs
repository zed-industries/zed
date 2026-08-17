#![cfg(feature = "NSError")]
#![cfg(feature = "NSString")]
use alloc::format;

use crate::Foundation::{ns_string, NSCocoaErrorDomain, NSError};

#[test]
fn basic() {
    let error = NSError::new(-999, unsafe { NSCocoaErrorDomain });
    let expected = if cfg!(target_vendor = "apple") {
        "The operation couldn’t be completed. (Cocoa error -999.)"
    } else {
        "NSCocoaErrorDomain -999"
    };
    assert_eq!(format!("{error}"), expected);
}

#[test]
fn custom_domain() {
    let error = NSError::new(42, ns_string!("MyDomain"));
    assert_eq!(error.code(), 42);
    assert_eq!(&*error.domain(), ns_string!("MyDomain"));
    let expected = if cfg!(target_vendor = "apple") {
        "The operation couldn’t be completed. (MyDomain error 42.)"
    } else {
        "MyDomain 42"
    };
    assert_eq!(format!("{error}"), expected);
}
