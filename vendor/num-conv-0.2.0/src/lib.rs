//! `num_conv` is a crate to convert between integer types without using `as` casts. This provides
//! better certainty when refactoring, makes the exact behavior of code more explicit, and allows
//! using turbofish syntax.

#![no_std]

/// Anonymously import all extension traits.
///
/// This allows you to use the methods without worrying about polluting the namespace or importing
/// them individually.
///
/// ```rust
/// use num_conv::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{Extend as _, Truncate as _};
}

mod sealed {
    pub trait Integer {}

    macro_rules! impl_integer {
        ($($t:ty)*) => {$(
            impl Integer for $t {}
        )*};
    }

    impl_integer! {
        u8 u16 u32 u64 u128 usize
        i8 i16 i32 i64 i128 isize
    }

    pub trait ExtendTargetSealed<T> {
        fn extend(self) -> T;
    }

    pub trait TruncateTargetSealed<T> {
        fn truncate(self) -> T;
    }
}

/// A type that can be used with turbofish syntax in [`Extend::extend`].
///
/// It is unlikely that you will want to use this trait directly. You are probably looking for the
/// [`Extend`] trait.
pub trait ExtendTarget<T>: sealed::ExtendTargetSealed<T> {}

/// A type that can be used with turbofish syntax in [`Truncate::truncate`].
///
/// It is unlikely that you will want to use this trait directly. You are probably looking for the
/// [`Truncate`] trait.
pub trait TruncateTarget<T>: sealed::TruncateTargetSealed<T> {}

/// Extend to an integer of the same size or larger, preserving its value.
///
/// ```rust
/// # use num_conv::Extend;
/// assert_eq!(0_u8.extend::<u16>(), 0_u16);
/// assert_eq!(0_u16.extend::<u32>(), 0_u32);
/// assert_eq!(0_u32.extend::<u64>(), 0_u64);
/// assert_eq!(0_u64.extend::<u128>(), 0_u128);
/// ```
///
/// ```rust
/// # use num_conv::Extend;
/// assert_eq!((-1_i8).extend::<i16>(), -1_i16);
/// assert_eq!((-1_i16).extend::<i32>(), -1_i32);
/// assert_eq!((-1_i32).extend::<i64>(), -1_i64);
/// assert_eq!((-1_i64).extend::<i128>(), -1_i128);
/// ```
pub trait Extend: sealed::Integer {
    /// Extend an integer to an integer of the same size or larger, preserving its value.
    fn extend<T>(self) -> T
    where
        Self: ExtendTarget<T>;
}

impl<T: sealed::Integer> Extend for T {
    fn extend<U>(self) -> U
    where
        T: ExtendTarget<U>,
    {
        sealed::ExtendTargetSealed::extend(self)
    }
}

/// Truncate to an integer of the same size or smaller, preserving the least significant bits.
///
/// ```rust
/// # use num_conv::Truncate;
/// assert_eq!(u16::MAX.truncate::<u8>(), u8::MAX);
/// assert_eq!(u32::MAX.truncate::<u16>(), u16::MAX);
/// assert_eq!(u64::MAX.truncate::<u32>(), u32::MAX);
/// assert_eq!(u128::MAX.truncate::<u64>(), u64::MAX);
/// ```
///
/// ```rust
/// # use num_conv::Truncate;
/// assert_eq!((-1_i16).truncate::<i8>(), -1_i8);
/// assert_eq!((-1_i32).truncate::<i16>(), -1_i16);
/// assert_eq!((-1_i64).truncate::<i32>(), -1_i32);
/// assert_eq!((-1_i128).truncate::<i64>(), -1_i64);
/// ```
pub trait Truncate: sealed::Integer {
    /// Truncate an integer to an integer of the same size or smaller, preserving the least
    /// significant bits.
    fn truncate<T>(self) -> T
    where
        Self: TruncateTarget<T>;
}

impl<T: sealed::Integer> Truncate for T {
    fn truncate<U>(self) -> U
    where
        T: TruncateTarget<U>,
    {
        sealed::TruncateTargetSealed::truncate(self)
    }
}

macro_rules! impl_extend {
    ($($from:ty => $($to:ty),+;)*) => {$($(
        const _: () = assert!(
            core::mem::size_of::<$from>() <= core::mem::size_of::<$to>(),
            concat!(
                "cannot extend ",
                stringify!($from),
                " to ",
                stringify!($to),
                " because ",
                stringify!($from),
                " is larger than ",
                stringify!($to)
            )
        );

        impl sealed::ExtendTargetSealed<$to> for $from {
            fn extend(self) -> $to {
                self as _
            }
        }

        impl ExtendTarget<$to> for $from {}
    )+)*};
}

macro_rules! impl_truncate {
    ($($($from:ty),+ => $to:ty;)*) => {$($(
        const _: () = assert!(
            core::mem::size_of::<$from>() >= core::mem::size_of::<$to>(),
            concat!(
                "cannot truncate ",
                stringify!($from),
                " to ",
                stringify!($to),
                " because ",
                stringify!($from),
                " is smaller than ",
                stringify!($to)
            )
        );

        impl sealed::TruncateTargetSealed<$to> for $from {
            fn truncate(self) -> $to {
                self as _
            }
        }

        impl TruncateTarget<$to> for $from {}
    )+)*};
}

impl_extend! {
    u8 => u8, u16, u32, u64, u128, usize;
    u16 => u16, u32, u64, u128, usize;
    u32 => u32, u64, u128;
    u64 => u64, u128;
    u128 => u128;
    usize => usize;

    i8 => i8, i16, i32, i64, i128, isize;
    i16 => i16, i32, i64, i128, isize;
    i32 => i32, i64, i128;
    i64 => i64, i128;
    i128 => i128;
    isize => isize;
}

impl_truncate! {
    u8, u16, u32, u64, u128, usize => u8;
    u16, u32, u64, u128, usize => u16;
    u32, u64, u128 => u32;
    u64, u128 => u64;
    u128 => u128;
    usize => usize;

    i8, i16, i32, i64, i128, isize => i8;
    i16, i32, i64, i128, isize => i16;
    i32, i64, i128 => i32;
    i64, i128 => i64;
    i128 => i128;
    isize => isize;
}
