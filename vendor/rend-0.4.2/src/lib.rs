//! # rend
//!
//! rend is a library that provides endian-aware primitives for Rust.
//!
//! It's similar in design to [`simple_endian`](https://crates.io/crates/simple_endian), but has
//! support for more builtin types such as atomics and nonzero integers. It also has support for
//! const functions since it does not rely on traits.
//!
//! rend does not provide endian-aware types for types that are inherently endian-agnostic, such as
//! `bool` and `u8`. It does not provide endian-aware types for types that have an
//! architecture-dependent size, such as `isize` and `usize`. It's also not extensible to custom
//! types.
//!
//! rend is intended to be used to build portable types that can be shared between different
//! architectures, especially with zero-copy deserialization.
//!
//! ## Features
//!
//! - `std`: Enables standard library support (enabled by default)
//! - `validation`: Enables validation support through `bytecheck`
//!
//! ## Example:
//! ```
//! use rend::*;
//!
//! let little_int = i32_le::new(0x12345678);
//! // Internal representation is little-endian
//! assert_eq!(
//!     [0x78, 0x56, 0x34, 0x12],
//!     unsafe { ::core::mem::transmute::<_, [u8; 4]>(little_int) }
//! );
//!
//! // Can also be made with `.into()`
//! let little_int: i32_le = 0x12345678.into();
//! // Still formats correctly
//! assert_eq!("305419896", format!("{}", little_int));
//! assert_eq!("0x12345678", format!("0x{:x}", little_int));
//!
//! let big_int = i32_be::new(0x12345678);
//! // Internal representation is big-endian
//! assert_eq!(
//!     [0x12, 0x34, 0x56, 0x78],
//!     unsafe { ::core::mem::transmute::<_, [u8; 4]>(big_int) }
//! );
//!
//! // Can also be made with `.into()`
//! let big_int: i32_be = 0x12345678.into();
//! // Still formats correctly
//! assert_eq!("305419896", format!("{}", big_int));
//! assert_eq!("0x12345678", format!("0x{:x}", big_int));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused,
    clippy::all
)]

#[macro_use]
mod impl_struct;
#[macro_use]
mod impl_traits;
#[cfg(feature = "validation")]
#[macro_use]
mod impl_validation;
#[cfg(feature = "bytemuck")]
mod impl_bytemuck;

#[cfg(feature = "validation")]
use bytecheck::{CharCheckError, CheckBytes, NonZeroCheckError};
#[cfg(feature = "validation")]
use core::convert::Infallible;
#[cfg(has_atomics)]
use core::sync::atomic::{AtomicI16, AtomicI32, AtomicU16, AtomicU32, Ordering};
#[cfg(has_atomics_64)]
use core::sync::atomic::{AtomicI64, AtomicU64};
use core::{
    hash::{Hash, Hasher},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroU128, NonZeroU16, NonZeroU32,
        NonZeroU64,
    },
};

/// A type that has an associated cross-endian storage type.
pub unsafe trait Primitive {
    /// An endian-agnostic type that can represent the primitve in both little- and big-endian
    /// forms.
    type Storage;
}

/// A wrapper for native-endian types.
///
/// This is mostly useful for `const` conversions to big- and little-endian types in contexts where
/// type inference is required. Because it's native-endian, the inner value is publicly exposed.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[repr(transparent)]
pub struct NativeEndian<T> {
    /// The value of the type
    pub value: T,
}

/// A wrapper for big-endian types.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct LittleEndian<T: Primitive> {
    value: T::Storage,
}

/// A wrapper for little-endian types.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BigEndian<T: Primitive> {
    value: T::Storage,
}

macro_rules! swap_endian {
    (@NativeEndian $expr:expr) => {{
        $expr
    }};
    (@LittleEndian $expr:expr) => {{
        #[cfg(target_endian = "little")]
        {
            $expr
        }
        #[cfg(target_endian = "big")]
        {
            $expr.swap_bytes()
        }
    }};
    (@BigEndian $expr:expr) => {{
        #[cfg(target_endian = "little")]
        {
            $expr.swap_bytes()
        }
        #[cfg(target_endian = "big")]
        {
            $expr
        }
    }};
}

macro_rules! swap_bytes {
    (@signed_int $endian:ident<$ne:ty> $value:expr) => {
        swap_endian!(@$endian $value)
    };
    (@unsigned_int $endian:ident<$ne:ty> $value:expr) => {
        swap_endian!(@$endian $value)
    };
    (@float $endian:ident<$ne:ty> $value:expr) => {
        <$ne>::from_bits(swap_endian!(@$endian $value.to_bits()))
    };
    (@char $endian:ident<$ne:ty> $value:expr) => {
        swap_endian!(@$endian $value)
    };
    (@nonzero $endian:ident<$ne:ty> $value:expr) => {
        unsafe { <$ne>::new_unchecked(swap_endian!(@$endian $value.get())) }
    };
    (@atomic $endian:ident<$ne:ty> $value:expr) => {
        swap_endian!(@$endian $value)
    };
}

macro_rules! from_native {
    (@char NativeEndian<$ne:ty> $value:expr) => {
        $value
    };
    (@char $endian:ident<$ne:ty> $value:expr) => {
        $value as u32
    };
    (@$class:ident $endian:ident<$ne:ty> $value:expr) => {
        $value
    };
}

macro_rules! to_native {
    (@char NativeEndian<$ne:ty> $value:expr) => {
        $value
    };
    (@char $endian:ident<$ne:ty> $value:expr) => {
        unsafe { char::from_u32_unchecked($value) }
    };
    (@$class:ident $endian:ident<$ne:ty> $value:expr) => {
        $value
    };
}

macro_rules! impl_endian {
    (
        @$class:ident $native:ty $(= $prim:ty)?,
        $ne:ident = $ne_doc:literal,
        $le:ident = $le_doc:literal,
        $be:ident = $be_doc:literal
    ) => {
        impl_endian!(@$class $ne_doc NativeEndian<$native> as $ne $(= $prim)?);
        impl_endian!(@$class $le_doc LittleEndian<$native> as $le $(= $prim)?);
        impl_endian!(@$class $be_doc BigEndian<$native> as $be $(= $prim)?);
    };
    (@$class:ident $doc:literal $endian:ident<$ne:ty> as $alias:ident $(= $prim:ty)?) => {
        impl_struct!(@$class $endian<$ne> $(= $prim)?);
        #[cfg(feature = "validation")]
        impl_validation!(@$class $endian<$ne> $(= $prim)?);
        #[doc = "Alias for "]
        #[doc = $doc]
        #[doc = "."]
        #[allow(non_camel_case_types)]
        pub type $alias = $endian<$ne>;
    };
}

macro_rules! impl_primitive {
    ($($ty:ty),+ $(,)?) => {
        $(
            unsafe impl Primitive for $ty {
                type Storage = $ty;
            }
        )+
    };
}

impl_primitive!(
    i16,
    i32,
    i64,
    i128,
    u16,
    u32,
    u64,
    u128,
    f32,
    f64,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroI128,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU128,
);

#[cfg(has_atomics)]
impl_primitive!(AtomicI16, AtomicI32, AtomicU16, AtomicU32);

#[cfg(has_atomics_64)]
impl_primitive!(AtomicI64, AtomicU64);

unsafe impl Primitive for char {
    type Storage = u32;
}

impl_endian!(
    @signed_int i16,
    i16_ne = "`NativeEndian<i16>`",
    i16_le = "`LittleEndian<i16>`",
    i16_be = "`BigEndian<i16>`"
);
impl_endian!(
    @signed_int i32,
    i32_ne = "`NativeEndian<i32>`",
    i32_le = "`LittleEndian<i32>`",
    i32_be = "`BigEndian<i32>`"
);
impl_endian!(
    @signed_int i64,
    i64_ne = "`NativeEndian<i64>`",
    i64_le = "`LittleEndian<i64>`",
    i64_be = "`BigEndian<i64>`"
);
impl_endian!(
    @signed_int i128,
    i128_ne = "`NativeEndian<i128>`",
    i128_le = "`LittleEndian<i128>`",
    i128_be = "`BigEndian<i128>`"
);
impl_endian!(
    @unsigned_int u16,
    u16_ne = "`NativeEndian<u16>`",
    u16_le = "`LittleEndian<u16>`",
    u16_be = "`BigEndian<u16>`"
);
impl_endian!(
    @unsigned_int u32,
    u32_ne = "`NativeEndian<u32>`",
    u32_le = "`LittleEndian<u32>`",
    u32_be = "`BigEndian<u32>`"
);
impl_endian!(
    @unsigned_int u64,
    u64_ne = "`NativeEndian<u64>`",
    u64_le = "`LittleEndian<u64>`",
    u64_be = "`BigEndian<u64>`"
);
impl_endian!(
    @unsigned_int u128,
    u128_ne = "`NativeEndian<u128>`",
    u128_le = "`LittleEndian<u128>`",
    u128_be = "`BigEndian<u128>`"
);

impl_endian!(
    @float f32,
    f32_ne = "`NativeEndian<f32>`",
    f32_le = "`LittleEndian<f32>`",
    f32_be = "`BigEndian<f32>`"
);
impl_endian!(
    @float f64,
    f64_ne = "`NativeEndian<f64>`",
    f64_le = "`LittleEndian<f64>`",
    f64_be = "`BigEndian<f64>`"
);

impl_endian!(
    @char char,
    char_ne = "`NativeEndian<char>`",
    char_le = "`LittleEndian<char>`",
    char_be = "`BigEndian<char>`"
);

impl_endian!(
    @nonzero NonZeroI16 = i16,
    NonZeroI16_ne = "`NativeEndian<NonZeroI16>`",
    NonZeroI16_le = "`LittleEndian<NonZeroI16>`",
    NonZeroI16_be = "`BigEndian<NonZeroI16>`"
);
impl_endian!(
    @nonzero NonZeroI32 = i32,
    NonZeroI32_ne = "`NativeEndian<NonZeroI32>`",
    NonZeroI32_le = "`LittleEndian<NonZeroI32>`",
    NonZeroI32_be = "`BigEndian<NonZeroI32>`"
);
impl_endian!(
    @nonzero NonZeroI64 = i64,
    NonZeroI64_ne = "`NativeEndian<NonZeroI64>`",
    NonZeroI64_le = "`LittleEndian<NonZeroI64>`",
    NonZeroI64_be = "`BigEndian<NonZeroI64>`"
);
impl_endian!(
    @nonzero NonZeroI128 = i128,
    NonZeroI128_ne = "`NativeEndian<NonZeroI128>`",
    NonZeroI128_le = "`LittleEndian<NonZeroI128>`",
    NonZeroI128_be = "`BigEndian<NonZeroI128>`"
);
impl_endian!(
    @nonzero NonZeroU16 = u16,
    NonZeroU16_ne = "`NativeEndian<NonZeroU16>`",
    NonZeroU16_le = "`LittleEndian<NonZeroU16>`",
    NonZeroU16_be = "`BigEndian<NonZeroU16>`"
);
impl_endian!(
    @nonzero NonZeroU32 = u32,
    NonZeroU32_ne = "`NativeEndian<NonZeroU32>`",
    NonZeroU32_le = "`LittleEndian<NonZeroU32>`",
    NonZeroU32_be = "`BigEndian<NonZeroU32>`"
);
impl_endian!(
    @nonzero NonZeroU64 = u64,
    NonZeroU64_ne = "`NativeEndian<NonZeroU64>`",
    NonZeroU64_le = "`LittleEndian<NonZeroU64>`",
    NonZeroU64_be = "`BigEndian<NonZeroU64>`"
);
impl_endian!(
    @nonzero NonZeroU128 = u128,
    NonZeroU128_ne = "`NativeEndian<NonZeroU128>`",
    NonZeroU128_le = "`LittleEndian<NonZeroU128>`",
    NonZeroU128_be = "`BigEndian<NonZeroU128>`"
);

#[cfg(has_atomics)]
impl_endian!(
    @atomic AtomicI16 = i16,
    AtomicI16_ne = "`NativeEndian<AtomicI16>`",
    AtomicI16_le = "`LittleEndian<AtomicI16>`",
    AtomicI16_be = "`BigEndian<AtomicI16>`"
);
#[cfg(has_atomics)]
impl_endian!(
    @atomic AtomicI32 = i32,
    AtomicI32_ne = "`NativeEndian<AtomicI32>`",
    AtomicI32_le = "`LittleEndian<AtomicI32>`",
    AtomicI32_be = "`BigEndian<AtomicI32>`"
);
#[cfg(has_atomics_64)]
impl_endian!(
    @atomic AtomicI64 = i64,
    AtomicI64_ne = "`NativeEndian<AtomicI64>`",
    AtomicI64_le = "`LittleEndian<AtomicI64>`",
    AtomicI64_be = "`BigEndian<AtomicI64>`"
);
#[cfg(has_atomics)]
impl_endian!(
    @atomic AtomicU16 = u16,
    AtomicU16_ne = "`NativeEndian<AtomicU16>`",
    AtomicU16_le = "`LittleEndian<AtomicU16>`",
    AtomicU16_be = "`BigEndian<AtomicU16>`"
);
#[cfg(has_atomics)]
impl_endian!(
    @atomic AtomicU32 = u32,
    AtomicU32_ne = "`NativeEndian<AtomicU32>`",
    AtomicU32_le = "`LittleEndian<AtomicU32>`",
    AtomicU32_be = "`BigEndian<AtomicU32>`"
);
#[cfg(has_atomics_64)]
impl_endian!(
    @atomic AtomicU64 = u64,
    AtomicU64_ne = "`NativeEndian<AtomicU64>`",
    AtomicU64_le = "`LittleEndian<AtomicU64>`",
    AtomicU64_be = "`BigEndian<AtomicU64>`"
);

#[cfg(test)]
mod tests {
    use crate::*;
    use core::mem;

    #[test]
    fn endian_representation() {
        unsafe {
            // i16
            assert_eq!(
                [0x01, 0x02],
                mem::transmute::<_, [u8; 2]>(i16_be::new(0x0102))
            );
            assert_eq!(
                [0x02, 0x01],
                mem::transmute::<_, [u8; 2]>(i16_le::new(0x0102))
            );

            // i32
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                mem::transmute::<_, [u8; 4]>(i32_be::new(0x01020304))
            );
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                mem::transmute::<_, [u8; 4]>(i32_le::new(0x01020304))
            );

            // i64
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                mem::transmute::<_, [u8; 8]>(i64_be::new(0x0102030405060708))
            );
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                mem::transmute::<_, [u8; 8]>(i64_le::new(0x0102030405060708))
            );

            // i128
            assert_eq!(
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                    0x0e, 0x0f, 0x10
                ],
                mem::transmute::<_, [u8; 16]>(i128_be::new(0x0102030405060708090a0b0c0d0e0f10))
            );
            assert_eq!(
                [
                    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04,
                    0x03, 0x02, 0x01
                ],
                mem::transmute::<_, [u8; 16]>(i128_le::new(0x0102030405060708090a0b0c0d0e0f10))
            );

            // u16
            assert_eq!(
                [0x01, 0x02],
                mem::transmute::<_, [u8; 2]>(u16_be::new(0x0102))
            );
            assert_eq!(
                [0x02, 0x01],
                mem::transmute::<_, [u8; 2]>(u16_le::new(0x0102))
            );

            // u32
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                mem::transmute::<_, [u8; 4]>(u32_be::new(0x01020304))
            );
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                mem::transmute::<_, [u8; 4]>(u32_le::new(0x01020304))
            );

            // u64
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                mem::transmute::<_, [u8; 8]>(u64_be::new(0x0102030405060708))
            );
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                mem::transmute::<_, [u8; 8]>(u64_le::new(0x0102030405060708))
            );

            // u128
            assert_eq!(
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                    0x0e, 0x0f, 0x10
                ],
                mem::transmute::<_, [u8; 16]>(u128_be::new(0x0102030405060708090a0b0c0d0e0f10))
            );
            assert_eq!(
                [
                    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04,
                    0x03, 0x02, 0x01
                ],
                mem::transmute::<_, [u8; 16]>(u128_le::new(0x0102030405060708090a0b0c0d0e0f10))
            );

            // f32
            assert_eq!(
                [0x40, 0x49, 0x0f, 0xdb],
                mem::transmute::<_, [u8; 4]>(f32_be::new(core::f32::consts::PI))
            );
            assert_eq!(
                [0xdb, 0x0f, 0x49, 0x40],
                mem::transmute::<_, [u8; 4]>(f32_le::new(core::f32::consts::PI))
            );

            // f64
            assert_eq!(
                [0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18],
                mem::transmute::<_, [u8; 8]>(f64_be::new(core::f64::consts::PI))
            );
            assert_eq!(
                [0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40],
                mem::transmute::<_, [u8; 8]>(f64_le::new(core::f64::consts::PI))
            );

            // char
            assert_eq!(
                [0x00, 0x01, 0xf3, 0x89],
                mem::transmute::<_, [u8; 4]>(char_be::new('🎉'))
            );
            assert_eq!(
                [0x89, 0xf3, 0x01, 0x00],
                mem::transmute::<_, [u8; 4]>(char_le::new('🎉'))
            );

            // AtomicU16
            #[cfg(has_atomics)]
            assert_eq!(
                [0x01, 0x02],
                mem::transmute::<_, [u8; 2]>(AtomicU16_be::new(0x0102))
            );
            #[cfg(has_atomics)]
            assert_eq!(
                [0x02, 0x01],
                mem::transmute::<_, [u8; 2]>(AtomicU16_le::new(0x0102))
            );

            // AtomicU32
            #[cfg(has_atomics)]
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                mem::transmute::<_, [u8; 4]>(AtomicU32_be::new(0x01020304))
            );
            #[cfg(has_atomics)]
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                mem::transmute::<_, [u8; 4]>(AtomicU32_le::new(0x01020304))
            );

            // AtomicU64
            #[cfg(has_atomics_64)]
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                mem::transmute::<_, [u8; 8]>(AtomicU64_be::new(0x0102030405060708))
            );
            #[cfg(has_atomics_64)]
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                mem::transmute::<_, [u8; 8]>(AtomicU64_le::new(0x0102030405060708))
            );
        }
    }
}
