use crate::{NSImageResizingMode, TARGET_ABI_USES_IOS_VALUES};

#[allow(non_upper_case_globals)]
#[allow(clippy::bool_to_int_with_if)]
impl NSImageResizingMode {
    #[doc(alias = "NSImageResizingModeStretch")]
    pub const Stretch: Self = Self(if TARGET_ABI_USES_IOS_VALUES { 0 } else { 1 });
    #[doc(alias = "NSImageResizingModeTile")]
    pub const Tile: Self = Self(if TARGET_ABI_USES_IOS_VALUES { 1 } else { 0 });
}

unsafe impl objc2_foundation::NSCoding for crate::NSImage {}
