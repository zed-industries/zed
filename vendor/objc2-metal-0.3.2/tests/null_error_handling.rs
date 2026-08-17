//! Regression test for https://github.com/madsmtm/objc2/issues/653.
#![cfg(all(feature = "MTLBinaryArchive", feature = "MTLDevice"))]
use objc2::available;
use objc2_foundation::{ns_string, NSURL};
use objc2_metal::{
    MTLBinaryArchive, MTLBinaryArchiveDescriptor, MTLCreateSystemDefaultDevice, MTLDevice,
};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

#[test]
fn test() {
    let Some(device) = MTLCreateSystemDefaultDevice() else {
        // Ignore, this won't work in CI.
        return;
    };
    // `MTLBinaryArchiveDescriptor` is unavailable before these.
    if !available!(ios = 14.0, macos = 11.0, tvos = 14.0, visionos = 1.0) {
        return;
    }
    let binary_archive = device
        .newBinaryArchiveWithDescriptor_error(&MTLBinaryArchiveDescriptor::new())
        .unwrap();

    let metal_lib = NSURL::URLWithString(ns_string!("file://missing.metallib")).unwrap();
    let err = binary_archive.serializeToURL_error(&metal_lib).unwrap_err();
    if err.domain().to_string() == "__objc2.missingError" {
        assert_eq!(err.code(), 0);
    } else {
        // In macOS 15, the error is no longer NULL
        assert!(err.domain().to_string() == "MTLBinaryArchiveDomain");
    }
}
