#![cfg(feature = "NSException")]
#![cfg(feature = "NSString")]
#![cfg(feature = "NSDictionary")]
#![cfg(feature = "NSObjCRuntime")]
use alloc::format;

use crate::{ns_string, NSException, NSObject};

#[test]
fn create_and_query() {
    let exc = NSException::new(ns_string!("abc"), Some(ns_string!("def")), None).unwrap();

    assert_eq!(&*exc.name(), ns_string!("abc"));
    assert_eq!(&*exc.reason().unwrap(), ns_string!("def"));
    assert!(exc.userInfo().is_none());

    let debug = format!("<NSException: {exc:p}> 'abc' reason: def");
    assert_eq!(format!("{exc:?}"), debug);

    let description = if cfg!(feature = "gnustep-1-7") {
        format!("<NSException: {exc:p}> NAME:abc REASON:def")
    } else {
        "def".into()
    };

    let obj: &NSObject = &exc;
    assert_eq!(format!("{obj:?}"), description);

    let exc = NSException::into_exception(exc);

    // Test `Debug` impl of Exception
    assert_eq!(format!("{exc:?}"), format!("exception {debug}"));
    // Test `Display` impl of Exception
    assert_eq!(format!("{exc}"), "def");
}

#[test]
#[should_panic = "'abc' reason: def"]
fn unwrap() {
    let exc = NSException::new(ns_string!("abc"), Some(ns_string!("def")), None).unwrap();

    panic!("{exc:?}");
}

// Further tests in `tests::exception`
