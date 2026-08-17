#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};

use crate::UniChar;

// Defined in <hfs/hfs_unistr.h>, but we don't expose that anywhere, so let's
// act as-if CoreServices is the one to define this type.
//
// NOTE: This is marked __attribute__((aligned(2), packed)), but that's
// unnecessary, since the alignment of u16/UniChar, and thereby the struct
// itself, is already 2.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HFSUniStr255 {
    length: u16,
    unicode: [UniChar; 255],
}

#[cfg(feature = "objc2")]
unsafe impl Encode for HFSUniStr255 {
    const ENCODING: Encoding = Encoding::Struct(
        "HFSUniStr255",
        &[<u16>::ENCODING, <[UniChar; 255]>::ENCODING],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for HFSUniStr255 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment() {
        assert_eq!(core::mem::align_of::<HFSUniStr255>(), 2);
    }
}
