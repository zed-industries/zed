use core::ffi::c_ushort;

use objc2::encode::{Encode, Encoding, RefEncode};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSDecimal {
    // signed   int _exponent:8;
    // unsigned int _length:4;
    // unsigned int _isNegative:1;
    // unsigned int _isCompact:1;
    // unsigned int _reserved:18;
    pub(crate) _inner: i32,
    pub(crate) _mantissa: [c_ushort; 8],
}

unsafe impl Encode for NSDecimal {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            Encoding::BitField(8, None),
            Encoding::BitField(4, None),
            Encoding::BitField(1, None),
            Encoding::BitField(1, None),
            Encoding::BitField(18, None),
            Encoding::Array(8, &Encoding::UShort),
        ],
    );
}

unsafe impl RefEncode for NSDecimal {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
