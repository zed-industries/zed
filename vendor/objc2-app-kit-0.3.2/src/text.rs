use crate::{NSTextAlignment, TARGET_ABI_USES_IOS_VALUES};

#[allow(non_upper_case_globals)]
#[allow(clippy::bool_to_int_with_if)]
impl NSTextAlignment {
    #[doc(alias = "NSTextAlignmentRight")]
    pub const Right: Self = Self(if TARGET_ABI_USES_IOS_VALUES { 2 } else { 1 });
    #[doc(alias = "NSTextAlignmentCenter")]
    pub const Center: Self = Self(if TARGET_ABI_USES_IOS_VALUES { 1 } else { 2 });
}
