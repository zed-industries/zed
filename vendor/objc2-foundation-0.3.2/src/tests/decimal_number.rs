#![cfg(all(
    feature = "NSDecimal",
    feature = "NSDecimalNumber",
    feature = "NSValue"
))]
use objc2::AnyThread;

use crate::{NSDecimal, NSDecimalNumber};

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "has different encoding, yet unsupported"
)]
fn test_decimal_encoding() {
    let decimal = NSDecimal {
        _inner: 0,
        _mantissa: [0; 8],
    };

    let obj = NSDecimalNumber::initWithDecimal(NSDecimalNumber::alloc(), decimal);
    assert_eq!(decimal, obj.decimalValue());
}
