use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::ffi::NSInteger;

use super::TARGET_ABI_USES_IOS_VALUES;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextAlignment(pub NSInteger);

unsafe impl Encode for NSTextAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[allow(non_upper_case_globals)]
#[allow(clippy::bool_to_int_with_if)]
impl NSTextAlignment {
    #[doc(alias = "NSTextAlignmentLeft")]
    pub const Left: Self = Self(0);
    #[doc(alias = "NSTextAlignmentRight")]
    pub const Right: Self = Self(if TARGET_ABI_USES_IOS_VALUES { 2 } else { 1 });
    #[doc(alias = "NSTextAlignmentCenter")]
    pub const Center: Self = Self(if TARGET_ABI_USES_IOS_VALUES { 1 } else { 2 });
    #[doc(alias = "NSTextAlignmentJustified")]
    pub const Justified: Self = Self(3);
    #[doc(alias = "NSTextAlignmentNatural")]
    pub const Natural: Self = Self(4);
}
