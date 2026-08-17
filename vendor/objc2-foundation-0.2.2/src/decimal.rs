use std::os::raw::c_ushort;

use objc2::encode::{Encode, Encoding, RefEncode};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSDecimal {
    // signed   int _exponent:8;
    // unsigned int _length:4;
    // unsigned int _isNegative:1;
    // unsigned int _isCompact:1;
    // unsigned int _reserved:18;
    _inner: i32,
    _mantissa: [c_ushort; 8],
}

unsafe impl Encode for NSDecimal {
    const ENCODING: Encoding = Encoding::Struct("NSDecimal", &[]);
}

unsafe impl RefEncode for NSDecimal {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
