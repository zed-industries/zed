use core::ffi::c_char;
use core::fmt;

use crate::vk;

/// Holds 24 bits in the least significant bits of memory,
/// and 8 bytes in the most significant bits of that memory,
/// occupying a single [`u32`] in total. This is commonly used in
/// [acceleration structure instances] such as
/// [`vk::AccelerationStructureInstanceKHR`],
/// [`vk::AccelerationStructureSRTMotionInstanceNV`] and
/// [`vk::AccelerationStructureMatrixMotionInstanceNV`].
///
/// [acceleration structure instances]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInstanceKHR.html#_description
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct Packed24_8(u32);

impl Packed24_8 {
    pub fn new(low_24: u32, high_8: u8) -> Self {
        Self((low_24 & 0x00ff_ffff) | (u32::from(high_8) << 24))
    }

    /// Extracts the least-significant 24 bits (3 bytes) of this integer
    pub fn low_24(&self) -> u32 {
        self.0 & 0xffffff
    }

    /// Extracts the most significant 8 bits (single byte) of this integer
    pub fn high_8(&self) -> u8 {
        (self.0 >> 24) as u8
    }
}

impl vk::ColorComponentFlags {
    /// Contraction of [`R`][Self::R] | [`G`][Self::G] | [`B`][Self::B] | [`A`][Self::A]
    pub const RGBA: Self = Self(Self::R.0 | Self::G.0 | Self::B.0 | Self::A.0);
}

impl From<vk::Extent2D> for vk::Extent3D {
    fn from(value: vk::Extent2D) -> Self {
        Self {
            width: value.width,
            height: value.height,
            depth: 1,
        }
    }
}

impl From<vk::Extent2D> for vk::Rect2D {
    fn from(extent: vk::Extent2D) -> Self {
        Self {
            offset: Default::default(),
            extent,
        }
    }
}

/// Structures implementing this trait are layout-compatible with [`vk::BaseInStructure`] and
/// [`vk::BaseOutStructure`]. Such structures have an `s_type` field indicating its type, which
/// must always match the value of [`TaggedStructure::STRUCTURE_TYPE`].
pub unsafe trait TaggedStructure {
    const STRUCTURE_TYPE: vk::StructureType;
}

#[inline]
pub(crate) fn wrap_c_str_slice_until_nul(
    str: &[c_char],
) -> Result<&core::ffi::CStr, core::ffi::FromBytesUntilNulError> {
    // SAFETY: The cast from c_char to u8 is ok because a c_char is always one byte.
    let bytes = unsafe { core::slice::from_raw_parts(str.as_ptr().cast(), str.len()) };
    core::ffi::CStr::from_bytes_until_nul(bytes)
}

#[derive(Debug)]
pub struct CStrTooLargeForStaticArray {
    pub static_array_size: usize,
    pub c_str_size: usize,
}
#[cfg(feature = "std")]
impl std::error::Error for CStrTooLargeForStaticArray {}
impl fmt::Display for CStrTooLargeForStaticArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "static `c_char` target array of length `{}` is too small to write a `CStr` (with `NUL`-terminator) of length `{}`",
            self.static_array_size, self.c_str_size
        )
    }
}

#[inline]
pub(crate) fn write_c_str_slice_with_nul(
    target: &mut [c_char],
    str: &core::ffi::CStr,
) -> Result<(), CStrTooLargeForStaticArray> {
    let bytes = str.to_bytes_with_nul();
    // SAFETY: The cast from c_char to u8 is ok because a c_char is always one byte.
    let bytes = unsafe { core::slice::from_raw_parts(bytes.as_ptr().cast(), bytes.len()) };
    let static_array_size = target.len();
    target
        .get_mut(..bytes.len())
        .ok_or(CStrTooLargeForStaticArray {
            static_array_size,
            c_str_size: bytes.len(),
        })?
        .copy_from_slice(bytes);
    Ok(())
}
