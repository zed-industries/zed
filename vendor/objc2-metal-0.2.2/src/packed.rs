use std::os::raw::c_float;

use objc2::encode::{Encode, Encoding, RefEncode};

// `MTLPackedFloat3` is actually a union internally, but replacing it with
// this simpler struct same semantics in Rust, and is generally the nicer
// pattern.
//
// <https://users.rust-lang.org/t/mapping-nested-packed-union-from-c-to-rust/87334>
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLPackedFloat3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

unsafe impl Encode for MTLPackedFloat3 {
    const ENCODING: Encoding = Encoding::Struct(
        "_MTLPackedFloat3",
        &[Encoding::Union(
            "?",
            &[
                Encoding::Struct(
                    "?",
                    &[c_float::ENCODING, c_float::ENCODING, c_float::ENCODING],
                ),
                Encoding::Array(3, &c_float::ENCODING),
            ],
        )],
    );
}

unsafe impl RefEncode for MTLPackedFloat3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use objc2::encode::Encode;

    use crate::MTLPackedFloat4x3;

    #[test]
    fn test_packed_float() {
        assert_eq!(
            MTLPackedFloat4x3::ENCODING.to_string(),
            "{_MTLPackedFloat4x3=[4{_MTLPackedFloat3=(?={?=fff}[3f])}]}",
        );
    }
}
