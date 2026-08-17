#![cfg(feature = "MTLDevice")]
use objc2::rc::Retained;
use objc2_metal::MTLCreateSystemDefaultDevice;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

#[test]
#[ignore = "doesn't work in CI"]
fn test_create_default() {
    let _ = unsafe { Retained::from_raw(MTLCreateSystemDefaultDevice()).unwrap() };
}

#[test]
#[cfg(target_os = "macos")]
fn get_all() {
    let _ = unsafe { Retained::from_raw(objc2_metal::MTLCopyAllDevices().as_ptr()).unwrap() };
}
