use objc2::encode::{Encode, Encoding, RefEncode};
use objc2::ffi::NSInteger;

use super::TARGET_ABI_USES_IOS_VALUES;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSImageResizingMode(pub NSInteger);

unsafe impl Encode for NSImageResizingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSImageResizingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[allow(non_upper_case_globals)]
#[allow(clippy::bool_to_int_with_if)]
impl NSImageResizingMode {
    #[doc(alias = "NSImageResizingModeStretch")]
    pub const Stretch: Self = Self(if TARGET_ABI_USES_IOS_VALUES { 0 } else { 1 });
    #[doc(alias = "NSImageResizingModeTile")]
    pub const Tile: Self = Self(if TARGET_ABI_USES_IOS_VALUES { 1 } else { 0 });
}

unsafe impl objc2_foundation::NSCoding for crate::NSImage {}
