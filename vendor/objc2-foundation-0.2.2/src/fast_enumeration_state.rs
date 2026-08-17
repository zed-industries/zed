use std::os::raw::c_ulong;

use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::runtime::AnyObject;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSFastEnumerationState {
    pub state: c_ulong,
    pub itemsPtr: *mut *mut AnyObject,
    pub mutationsPtr: *mut c_ulong,
    pub extra: [c_ulong; 5],
}

unsafe impl Encode for NSFastEnumerationState {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            Encoding::C_ULONG,
            Encoding::Pointer(&Encoding::Object),
            Encoding::Pointer(&Encoding::C_ULONG),
            Encoding::Array(5, &Encoding::C_ULONG),
        ],
    );
}

unsafe impl RefEncode for NSFastEnumerationState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
