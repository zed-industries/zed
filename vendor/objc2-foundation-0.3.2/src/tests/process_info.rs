#![cfg(feature = "NSProcessInfo")]
use alloc::format;

use crate::NSProcessInfo;

#[test]
fn debug() {
    let info = NSProcessInfo::processInfo();

    #[cfg(feature = "NSString")]
    let expected = format!(
        "NSProcessInfo {{ processName: {:?}, .. }}",
        info.processName()
    );

    #[cfg(not(feature = "NSString"))]
    let expected = "NSProcessInfo { .. }";

    assert_eq!(format!("{info:?}"), expected);
}

#[test]
#[cfg(not(feature = "gnustep-1-7"))]
fn encoding_operating_system_version() {
    let info = NSProcessInfo::processInfo();
    let _version = info.operatingSystemVersion();
}
