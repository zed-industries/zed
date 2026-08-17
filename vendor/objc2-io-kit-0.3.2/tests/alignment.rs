#![cfg(all(feature = "usb", feature = "AppleUSBDefinitions"))]

#[test]
fn alignment() {
    assert_eq!(std::mem::align_of::<objc2_io_kit::IOUSBHIDReportDesc>(), 1);
    assert_eq!(std::mem::size_of::<objc2_io_kit::IOUSBHIDReportDesc>(), 3);
}
