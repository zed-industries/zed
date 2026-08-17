//! Ergonomic, checked cast functions for primitive types
//!
//! This crate provides one checked cast function for each numeric primitive.
//! Use these functions to perform a cast from any other numeric primitive:
//!
//! ```
//! use cast::{u8, u16, Error};
//!
//! # fn main() {
//! // Infallible operations, like integer promotion, are equivalent to a normal
//! // cast with `as`
//! assert_eq!(u16(0u8), 0u16);
//!
//! // Everything else will return a `Result` depending on the success of the
//! // operation
//! assert_eq!(u8(0u16), Ok(0u8));
//! assert_eq!(u8(256u16), Err(Error::Overflow));
//! assert_eq!(u8(-1i8), Err(Error::Underflow));
//! assert_eq!(u8(1. / 0.), Err(Error::Infinite));
//! assert_eq!(u8(0. / 0.), Err(Error::NaN));
//! # }
//! ```
//!
//! There are no namespace problems between these functions, the "primitive
//! modules" in `core`/`std` and the built-in primitive types, so all them can
//! be in the same scope:
//!
//! ```
//! use std::u8;
//! use cast::{u8, u16};
//!
//! # fn main() {
//! // `u8` as a type
//! let x: u8 = 0;
//! // `u8` as a module
//! let y = u16(u8::MAX);
//! // `u8` as a function
//! let z = u8(y).unwrap();
//! # }
//! ```
//!
//! The checked cast functionality is also usable with type aliases via the
//! `cast` static method:
//!
//! ```
//! use std::os::raw::c_ulonglong;
//! // NOTE avoid shadowing `std::convert::From` - cf. rust-lang/rfcs#1311
//! use cast::From as _0;
//!
//! # fn main() {
//! assert_eq!(c_ulonglong::cast(0u8), 0u64);
//! # }
//! ```
//!
//! This crate also provides a `From` trait that can be used, for example,
//! to create a generic function that accepts any type that can be infallibly
//! casted to `u32`.
//!
//! ```
//! fn to_u32<T>(x: T) -> u32
//!     // reads as: "where u32 can be casted from T with output u32"
//!     where u32: cast::From<T, Output=u32>,
//! {
//!     cast::u32(x)
//! }
//!
//! # fn main() {
//! assert_eq!(to_u32(0u8), 0u32);
//! assert_eq!(to_u32(1u16), 1u32);
//! assert_eq!(to_u32(2u32), 2u32);
//!
//! // to_u32(-1i32);  // Compile error
//! # }
//! ```
//!
//! ## Minimal Supported Rust Version
//!
//! This crate is guaranteed to compile *as a dependency* on stable Rust 1.31 and up.
//! It's not guaranteed that `cargo test`-ing this crate follows the MSRV.
//! It *might* compile on older versions but that may change in any new patch release.
//!
//! ## Building without `std`
//!
//! This crate can be used without Rust's `std` crate by declaring it as
//! follows in your `Cargo.toml`:
//!
//! ``` toml
//! cast = { version = "*", default-features = false }
//! ```

#![allow(const_err)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(warnings)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use core::fmt;
#[cfg(feature = "std")]
use std::error;

#[cfg(test)]
mod test;

/// Cast errors
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Infinite value casted to a type that can only represent finite values
    Infinite,
    /// NaN value casted to a type that can't represent a NaN value
    NaN,
    /// Source value is greater than the maximum value that the destination type
    /// can hold
    Overflow,
    /// Source value is smaller than the minimum value that the destination type
    /// can hold
    Underflow,
}

impl Error {
    /// A private helper function that implements `description`, because
    /// `description` is only available when we have `std` enabled.
    fn description_helper(&self) -> &str {
        match *self {
            Error::Infinite => "Cannot store infinite value in finite type",
            Error::NaN => "Cannot store NaN in type which does not support it",
            Error::Overflow => "Overflow during numeric conversion",
            Error::Underflow => "Underflow during numeric conversion",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description_helper())
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        self.description_helper()
    }
}

/// The "cast from" operation
pub trait From<Src> {
    /// The result of the cast operation: either `Self` or `Result<Self, Error>`
    type Output;

    /// Checked cast from `Src` to `Self`
    fn cast(_: Src) -> Self::Output;
}

macro_rules! fns {
    ($($ty:ident),+) => {
        $(
            /// Checked cast function
            #[inline]
            pub fn $ty<T>(x: T) -> <$ty as From<T>>::Output
                where $ty: From<T>
            {
                <$ty as From<T>>::cast(x)
            }
         )+
    }
}

fns!(f32, f64, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

fns!(i128, u128);

/// `$dst` can hold any value of `$src`
macro_rules! promotion {
    ($($src:ty => $($dst: ty),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = $dst;

                    #[inline]
                    fn cast(src: $src) -> $dst {
                        src as $dst
                    }
                }
            )+
        )+
    }
}

/// `$dst` can hold any positive value of `$src`
macro_rules! half_promotion {
    ($($src:ty => $($dst:ty),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = Result<$dst, Error>;

                    #[inline]
                    fn cast(src: $src) -> Self::Output {
                        if src < 0 {
                            Err(Error::Underflow)
                        } else {
                            Ok(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

/// From an unsigned `$src` to a smaller `$dst`
macro_rules! from_unsigned {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = Result<$dst, Error>;

                    #[inline]
                    fn cast(src: $src) -> Self::Output {
                        use core::$dst;

                        if src > $dst::MAX as $src {
                            Err(Error::Overflow)
                        } else {
                            Ok(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

/// From a signed `$src` to a smaller `$dst`
macro_rules! from_signed {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = Result<$dst, Error>;

                    #[inline]
                    fn cast(src: $src) -> Self::Output {
                        use core::$dst;

                        Err(if src < $dst::MIN as $src {
                            Error::Underflow
                        } else if src > $dst::MAX as $src {
                            Error::Overflow
                        } else {
                            return Ok(src as $dst);
                        })
                    }
                }
            )+
        )+
    }
}

/// From a float `$src` to an integer `$dst`
macro_rules! from_float {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = Result<$dst, Error>;

                    #[inline]
                    fn cast(src: $src) -> Self::Output {
                        use core::{$dst, $src};

                        Err(if src != src {
                            Error::NaN
                        } else if src == $src::INFINITY ||
                            src == $src::NEG_INFINITY {
                            Error::Infinite
                        } else if {
                            // this '$dst::BITS' works on 1.31.0 (MSRV)
                            let dst_bits = core::mem::size_of::<$dst>() as u32 * 8;
                            let lossless = dst_bits < core::$src::MANTISSA_DIGITS;

                            let max = if lossless {
                                $dst::MAX as $src
                            } else {
                                // we subtract 1 ULP (unit of least precision) here because some
                                // lossy conversions like `u64::MAX as f64` round *up* and we want
                                // to avoid the check below evaluating to false in that case
                                $src::from_bits(($dst::MAX as $src).to_bits() - 1)
                            };

                            src > max
                        } {
                            Error::Overflow
                        } else if $dst::MIN == 0 {
                            // when casting to unsigned integer, negative values close to 0 but
                            // larger than 1.0 should be truncated to 0; this behavior matches
                            // casting from a float to a signed integer
                            if src <= -1.0 {
                                Error::Underflow
                            } else {
                                return Ok(src as $dst);
                            }
                        } else if src < $dst::MIN as $src {
                            Error::Underflow
                        } else  {
                            return Ok(src as $dst);
                        })
                    }
                }
            )+
        )+
    }
}

/// From a float `$src` to an integer `$dst`, where $dst is large enough to contain
/// all values of `$src`. We can't ever overflow here
macro_rules! from_float_dst {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                     type Output = Result<$dst, Error>;

                    #[inline]
                    #[allow(unused_comparisons)]
                    fn cast(src: $src) -> Self::Output {
                        use core::{$dst, $src};

                        Err(if src != src {
                            Error::NaN
                        } else if src == $src::INFINITY ||
                            src == $src::NEG_INFINITY {
                            Error::Infinite
                        } else if ($dst::MIN == 0) && src <= -1.0 {
                            Error::Underflow
                        } else {
                            return Ok(src as $dst);
                        })
                    }
                }
            )+
        )+
    }
}

// PLAY TETRIS! ;-)

#[cfg(target_pointer_width = "32")]
mod _32 {
    use crate::{Error, From};

    // Signed
    promotion! {
        i8    => f32, f64, i8, i16, i32, isize, i64;
        i16   => f32, f64,     i16, i32, isize, i64;
        i32   => f32, f64,          i32, isize, i64;
        isize => f32, f64,          i32, isize, i64;
        i64   => f32, f64,                      i64;
    }

    half_promotion! {
        i8    =>                                     u8, u16, u32, usize, u64;
        i16   =>                                         u16, u32, usize, u64;
        i32   =>                                              u32, usize, u64;
        isize =>                                              u32, usize, u64;
        i64   =>                                                          u64;
    }

    from_signed! {

        i16   =>           i8,                       u8;
        i32   =>           i8, i16,                  u8, u16;
        isize =>           i8, i16,                  u8, u16;
        i64   =>           i8, i16, i32, isize,      u8, u16, u32, usize;
    }

    // Unsigned
    promotion! {
        u8    => f32, f64,     i16, i32, isize, i64, u8, u16, u32, usize, u64;
        u16   => f32, f64,          i32, isize, i64,     u16, u32, usize, u64;
        u32   => f32, f64,                      i64,          u32, usize, u64;
        usize => f32, f64,                      i64,          u32, usize, u64;
        u64   => f32, f64,                                                u64;
    }

    from_unsigned! {
        u8    =>           i8;
        u16   =>           i8, i16,                  u8;
        u32   =>           i8, i16, i32, isize,      u8, u16;
        usize =>           i8, i16, i32, isize,      u8, u16;
        u64   =>           i8, i16, i32, isize, i64, u8, u16, u32, usize;
    }

    // Float
    promotion! {
        f32   => f32, f64;
        f64   =>      f64;
    }

    from_float! {
        f32 =>             i8, i16, i32, isize, i64, u8, u16, u32, usize, u64;
        f64 =>             i8, i16, i32, isize, i64, u8, u16, u32, usize, u64;
    }
}

#[cfg(target_pointer_width = "64")]
mod _64 {
    use crate::{Error, From};

    // Signed
    promotion! {
        i8    => f32, f64, i8, i16, i32, i64, isize;
        i16   => f32, f64,     i16, i32, i64, isize;
        i32   => f32, f64,          i32, i64, isize;
        i64   => f32, f64,               i64, isize;
        isize => f32, f64,               i64, isize;
    }

    half_promotion! {
        i8    =>                                     u8, u16, u32, u64, usize;
        i16   =>                                         u16, u32, u64, usize;
        i32   =>                                              u32, u64, usize;
        i64   =>                                                   u64, usize;
        isize =>                                                   u64, usize;
    }

    from_signed! {

        i16   =>           i8,                       u8;
        i32   =>           i8, i16,                  u8, u16;
        i64   =>           i8, i16, i32,             u8, u16, u32;
        isize =>           i8, i16, i32,             u8, u16, u32;
    }

    // Unsigned
    promotion! {
        u8    => f32, f64,     i16, i32, i64, isize, u8, u16, u32, u64, usize;
        u16   => f32, f64,          i32, i64, isize,     u16, u32, u64, usize;
        u32   => f32, f64,               i64, isize,          u32, u64, usize;
        u64   => f32, f64,                                         u64, usize;
        usize => f32, f64,                                         u64, usize;
    }

    from_unsigned! {
        u8    =>           i8;
        u16   =>           i8, i16,                  u8;
        u32   =>           i8, i16, i32,             u8, u16;
        u64   =>           i8, i16, i32, i64, isize, u8, u16, u32;
        usize =>           i8, i16, i32, i64, isize, u8, u16, u32;
    }

    // Float
    promotion! {
        f32  => f32, f64;
        f64  =>      f64;
    }

    from_float! {
        f32 =>             i8, i16, i32, i64, isize, u8, u16, u32, u64, usize;
        f64 =>             i8, i16, i32, i64, isize, u8, u16, u32, u64, usize;
    }
}

mod _x128 {
    use crate::{Error, From};

    // Signed
    promotion! {
        i8    =>                              i128;
        i16   =>                              i128;
        i32   =>                              i128;
        i64   =>                              i128;
        isize =>                              i128;
        i128  => f32, f64,                    i128;
    }

    half_promotion! {
        i8    =>                                                              u128;
        i16   =>                                                              u128;
        i32   =>                                                              u128;
        i64   =>                                                              u128;
        isize =>                                                              u128;
        i128  =>                                                              u128;
    }

    from_signed! {
        i128  =>           i8, i16, i32, i64,       isize, u8, u16, u32, u64,       usize;
    }

    // Unsigned
    promotion! {
        u8    =>                              i128,                           u128;
        u16   =>                              i128,                           u128;
        u32   =>                              i128,                           u128;
        u64   =>                              i128,                           u128;
        usize =>                              i128,                           u128;
        u128  =>      f64,                                                    u128;
    }

    from_unsigned! {
        u128 => f32,       i8, i16, i32, i64, i128, isize, u8, u16, u32, u64,       usize;
    }

    // Float
    from_float_dst! {
        f32 =>                                                                u128;
    }

    from_float! {
        f32 =>                                i128;
        f64 =>                                i128,                           u128;
    }
}

// The missing piece
impl From<f64> for f32 {
    type Output = Result<f32, Error>;

    #[inline]
    fn cast(src: f64) -> Self::Output {
        use core::{f32, f64};

        if src != src || src == f64::INFINITY || src == f64::NEG_INFINITY {
            Ok(src as f32)
        } else if src < f32::MIN as f64 {
            Err(Error::Underflow)
        } else if src > f32::MAX as f64 {
            Err(Error::Overflow)
        } else {
            Ok(src as f32)
        }
    }
}
