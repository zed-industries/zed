#![cfg(feature = "NSProxy")]
use crate::Foundation::NSProxy;
use objc2::ClassType;

#[test]
fn dummy() {
    let _cls = NSProxy::class();
}
