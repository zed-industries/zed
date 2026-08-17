//! Test that AnyProtocol objects have a shared retain count.
//!
//! Separate test because this will likely not work if other tests are running
//! at the same time.

use objc2::runtime::{AnyObject, NSObject, NSObjectProtocol};
use objc2::{Message, ProtocolType};

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "Protocols don't implement isKindOfClass: on GNUStep"
)]
#[cfg_attr(
    all(target_os = "macos", target_arch = "x86"),
    ignore = "protocols are not NSObject subclasses in the old runtime"
)]
fn protocol_has_shared_retain_count() {
    let obj: &AnyObject = <dyn NSObjectProtocol>::protocol().unwrap().as_ref();
    let obj = obj.downcast_ref::<NSObject>().unwrap();

    assert_eq!(obj.retainCount(), 1);
    let obj2 = obj.retain();
    assert_eq!(obj.retainCount(), 2);
    drop(obj2);
    assert_eq!(obj.retainCount(), 1);

    let obj2: &AnyObject = <dyn NSObjectProtocol>::protocol().unwrap().as_ref();
    assert_eq!(obj.retainCount(), 1);
    let _obj2 = obj2.retain();
    assert_eq!(obj.retainCount(), 2);
}
