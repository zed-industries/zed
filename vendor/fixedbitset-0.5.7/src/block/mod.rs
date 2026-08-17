#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(dead_code)]
// TODO: Remove once the transmutes are fixed
#![allow(unknown_lints)]
#![allow(clippy::missing_transmute_annotations)]

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};

#[cfg(all(
    not(all(target_family = "wasm", target_feature = "simd128")),
    not(target_feature = "sse2"),
    not(target_feature = "avx"),
    not(target_feature = "avx2"),
))]
mod default;
#[cfg(all(
    not(all(target_family = "wasm", target_feature = "simd128")),
    not(target_feature = "sse2"),
    not(target_feature = "avx"),
    not(target_feature = "avx2"),
))]
pub use self::default::*;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2",
    not(target_feature = "avx"),
    not(target_feature = "avx2"),
))]
mod sse2;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2",
    not(target_feature = "avx"),
    not(target_feature = "avx2"),
))]
pub use self::sse2::*;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx",
    not(target_feature = "avx2")
))]
mod avx;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx",
    not(target_feature = "avx2")
))]
pub use self::avx::*;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
mod avx2;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
pub use self::avx2::*;

#[cfg(all(target_family = "wasm", target_feature = "simd128"))]
mod wasm;
#[cfg(all(target_family = "wasm", target_feature = "simd128"))]
pub use self::wasm::*;

impl Block {
    pub const USIZE_COUNT: usize = core::mem::size_of::<Self>() / core::mem::size_of::<usize>();
    pub const NONE: Self = Self::from_usize_array([0; Self::USIZE_COUNT]);
    pub const ALL: Self = Self::from_usize_array([usize::MAX; Self::USIZE_COUNT]);
    pub const BITS: usize = core::mem::size_of::<Self>() * 8;

    #[inline]
    pub fn into_usize_array(self) -> [usize; Self::USIZE_COUNT] {
        unsafe { core::mem::transmute(self.0) }
    }

    #[inline]
    pub const fn from_usize_array(array: [usize; Self::USIZE_COUNT]) -> Self {
        Self(unsafe { core::mem::transmute(array) })
    }
}

impl Eq for Block {}

impl PartialOrd for Block {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.into_usize_array().cmp(&other.into_usize_array())
    }
}

impl Default for Block {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

impl Hash for Block {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        Hash::hash_slice(&self.into_usize_array(), hasher);
    }
}
