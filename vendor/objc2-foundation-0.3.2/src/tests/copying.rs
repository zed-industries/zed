#![cfg(feature = "NSString")]
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSCopying, NSMutableCopying, NSString};

#[test]
fn copy() {
    let obj = NSString::new();
    let protocol_object: &ProtocolObject<dyn NSCopying> = ProtocolObject::from_ref(&*obj);
    let _: Retained<ProtocolObject<dyn NSCopying>> = protocol_object.copy();
}

#[test]
fn copy_mutable() {
    let obj = NSString::new();
    let protocol_object: &ProtocolObject<dyn NSMutableCopying> = ProtocolObject::from_ref(&*obj);
    let _: Retained<ProtocolObject<dyn NSMutableCopying>> = protocol_object.mutableCopy();
}
