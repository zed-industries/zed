//! Offsets to tables

use crate::{Scalar, Uint24};

/// An offset of a given width for which NULL (zero) is a valid value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct Nullable<T>(T);

// internal implementation detail; lets us implement Default for nullable offsets.
trait NullValue {
    const NULL: Self;
}

impl<T: Scalar> Scalar for Nullable<T> {
    type Raw = T::Raw;

    #[inline]
    fn from_raw(raw: Self::Raw) -> Self {
        Self(T::from_raw(raw))
    }

    #[inline]
    fn to_raw(self) -> Self::Raw {
        self.0.to_raw()
    }
}

impl<T> Nullable<T> {
    /// Return a reference to the inner offset
    #[inline]
    pub fn offset(&self) -> &T {
        &self.0
    }
}

impl<T: PartialEq<u32>> Nullable<T> {
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

impl<T: PartialEq<u32>> PartialEq<u32> for Nullable<T> {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl<T: NullValue> Default for Nullable<T> {
    fn default() -> Self {
        Self(T::NULL)
    }
}

macro_rules! impl_offset {
    ($name:ident, $bits:literal, $rawty:ty) => {
        #[doc = concat!("A", stringify!($bits), "-bit offset to a table.")]
        ///
        /// Specific offset fields may or may not permit NULL values; however we
        /// assume that errors are possible, and expect the caller to handle
        /// the `None` case.
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
        #[repr(transparent)]
        pub struct $name($rawty);

        impl $name {
            /// Create a new offset.
            #[inline]
            pub const fn new(raw: $rawty) -> Self {
                Self(raw)
            }

            /// Return `true` if this offset is null.
            #[inline]
            pub fn is_null(self) -> bool {
                self.to_u32() == 0
            }

            #[inline]
            pub fn to_u32(self) -> u32 {
                self.0.into()
            }
        }

        impl crate::raw::Scalar for $name {
            type Raw = <$rawty as crate::raw::Scalar>::Raw;
            fn from_raw(raw: Self::Raw) -> Self {
                let raw = <$rawty>::from_raw(raw);
                $name::new(raw)
            }

            fn to_raw(self) -> Self::Raw {
                self.0.to_raw()
            }
        }

        // useful for debugging
        impl PartialEq<u32> for $name {
            fn eq(&self, other: &u32) -> bool {
                self.to_u32() == *other
            }
        }

        impl NullValue for $name {
            const NULL: $name = $name(<$rawty>::MIN);
        }
    };
}

impl_offset!(Offset16, 16, u16);
impl_offset!(Offset24, 24, Uint24);
impl_offset!(Offset32, 32, u32);
