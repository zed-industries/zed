#![cfg(feature = "MTLDevice")]
use objc2_metal::MTLCreateSystemDefaultDevice;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

#[test]
#[ignore = "doesn't work in CI"]
fn test_create_default() {
    let _ = MTLCreateSystemDefaultDevice();
}

#[test]
fn get_all() {
    let _ = objc2_metal::MTLCopyAllDevices();
}
