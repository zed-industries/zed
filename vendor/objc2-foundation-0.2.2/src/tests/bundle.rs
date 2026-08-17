#![cfg(feature = "NSBundle")]
use alloc::format;

use crate::Foundation::NSBundle;

#[test]
#[cfg(feature = "NSString")]
#[cfg(feature = "NSDictionary")]
#[cfg_attr(not(target_os = "macos"), ignore = "varies between platforms")]
fn try_running_functions() {
    // This is mostly empty since cargo doesn't bundle the application
    // before executing.
    let bundle = NSBundle::mainBundle();
    std::println!("{bundle:?}");
    assert_eq!(format!("{:?}", bundle.infoDictionary().unwrap()), "{}");
    assert_eq!(bundle.name(), None);
}
