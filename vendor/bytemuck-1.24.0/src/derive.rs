//! This module contains some helpers for the derive macros.

/// A trait that can be used to convert the type of a byte array to an integer
/// type of the same size.
pub trait EnumTagIntegerBytes {
  type Integer;
}

macro_rules! enum_tag_integer_impls {
  ($($ty:ty),*) => {
    $(
      impl EnumTagIntegerBytes for [u8; core::mem::size_of::<$ty>()] {
        type Integer = $ty;
      }
    )*
  };
}

enum_tag_integer_impls!(u8, u16, u32, u64, u128);
