// Copyright (c) 2017-2021, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;

use num_traits::{AsPrimitive, FromPrimitive, PrimInt, Signed};

use std::fmt;
use std::fmt::{Debug, Display};
use std::mem::size_of;
use std::ops::AddAssign;

/// Trait for casting between primitive types.
pub trait CastFromPrimitive<T>: Copy + 'static {
    /// Casts the given value into `Self`.
    fn cast_from(v: T) -> Self;
}

macro_rules! impl_cast_from_primitive {
  ( $T:ty => $U:ty ) => {
    impl CastFromPrimitive<$U> for $T {
      #[inline(always)]
      fn cast_from(v: $U) -> Self { v as Self }
    }
  };
  ( $T:ty => { $( $U:ty ),* } ) => {
    $( impl_cast_from_primitive!($T => $U); )*
  };
}

// casts to { u8, u16 } are implemented separately using Pixel, so that the
// compiler understands that CastFromPrimitive<T: Pixel> is always implemented
impl_cast_from_primitive!(u8 => { u32, u64, usize });
impl_cast_from_primitive!(u8 => { i8, i64, isize });
impl_cast_from_primitive!(u16 => { u32, u64, usize });
impl_cast_from_primitive!(u16 => { i8, i64, isize });
impl_cast_from_primitive!(i16 => { u32, u64, usize });
impl_cast_from_primitive!(i16 => { i8, i64, isize });
impl_cast_from_primitive!(i32 => { u32, u64, usize });
impl_cast_from_primitive!(i32 => { i8, i64, isize });

pub trait RegisteredPrimitive:
    PrimInt
    + AsPrimitive<u8>
    + AsPrimitive<i16>
    + AsPrimitive<u16>
    + AsPrimitive<i32>
    + AsPrimitive<u32>
    + AsPrimitive<usize>
    + CastFromPrimitive<u8>
    + CastFromPrimitive<i16>
    + CastFromPrimitive<u16>
    + CastFromPrimitive<i32>
    + CastFromPrimitive<u32>
    + CastFromPrimitive<usize>
{
}

impl RegisteredPrimitive for u8 {}
impl RegisteredPrimitive for u16 {}
impl RegisteredPrimitive for i16 {}
impl RegisteredPrimitive for i32 {}

macro_rules! impl_cast_from_pixel_to_primitive {
    ( $T:ty ) => {
        impl<T: RegisteredPrimitive> CastFromPrimitive<T> for $T {
            #[inline(always)]
            fn cast_from(v: T) -> Self {
                v.as_()
            }
        }
    };
}

impl_cast_from_pixel_to_primitive!(u8);
impl_cast_from_pixel_to_primitive!(i16);
impl_cast_from_pixel_to_primitive!(u16);
impl_cast_from_pixel_to_primitive!(i32);
impl_cast_from_pixel_to_primitive!(u32);

/// Types that can be used as pixel types.
#[derive(PartialEq, Eq)]
pub enum PixelType {
    /// 8 bits per pixel, stored in a `u8`.
    U8,
    /// 10 or 12 bits per pixel, stored in a `u16`.
    U16,
}

/// A type that can be used as a pixel type.
pub trait Pixel:
    RegisteredPrimitive + Into<u32> + Into<i32> + Debug + Display + Send + Sync + 'static
{
    type Coeff: Coefficient;

    /// Returns a [`PixelType`] variant corresponding to this type.
    ///
    /// [`PixelType`]: enum.PixelType.html
    fn type_enum() -> PixelType;

    /// Converts stride in pixels to stride in bytes.
    #[inline]
    fn to_asm_stride(in_stride: usize) -> isize {
        (in_stride * size_of::<Self>()) as isize
    }
}

impl Pixel for u8 {
    type Coeff = i16;

    #[inline]
    fn type_enum() -> PixelType {
        PixelType::U8
    }
}

impl Pixel for u16 {
    type Coeff = i32;

    #[inline]
    fn type_enum() -> PixelType {
        PixelType::U16
    }
}

pub trait Coefficient:
    RegisteredPrimitive + Into<i32> + AddAssign + Signed + Debug + 'static
{
    type Pixel: Pixel;
}

impl Coefficient for i16 {
    type Pixel = u8;
}
impl Coefficient for i32 {
    type Pixel = u16;
}

/// Chroma subsampling format
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen)]
#[repr(C)]
pub enum ChromaSampling {
    /// Both vertically and horizontally subsampled.
    #[default]
    Cs420,
    /// Horizontally subsampled.
    Cs422,
    /// Not subsampled.
    Cs444,
    /// Monochrome.
    Cs400,
}

impl FromPrimitive for ChromaSampling {
    fn from_i64(n: i64) -> Option<Self> {
        use ChromaSampling::*;

        match n {
            n if n == Cs420 as i64 => Some(Cs420),
            n if n == Cs422 as i64 => Some(Cs422),
            n if n == Cs444 as i64 => Some(Cs444),
            n if n == Cs400 as i64 => Some(Cs400),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        ChromaSampling::from_i64(n as i64)
    }
}

impl fmt::Display for ChromaSampling {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                ChromaSampling::Cs420 => "4:2:0",
                ChromaSampling::Cs422 => "4:2:2",
                ChromaSampling::Cs444 => "4:4:4",
                ChromaSampling::Cs400 => "Monochrome",
            }
        )
    }
}

impl ChromaSampling {
    /// Provides the amount to right shift the luma plane dimensions to get the
    ///  chroma plane dimensions.
    /// Only values 0 or 1 are ever returned.
    /// The plane dimensions must also be rounded up to accommodate odd luma plane
    ///  sizes.
    /// Cs400 returns None, as there are no chroma planes.
    pub const fn get_decimation(self) -> Option<(usize, usize)> {
        use self::ChromaSampling::*;
        match self {
            Cs420 => Some((1, 1)),
            Cs422 => Some((1, 0)),
            Cs444 => Some((0, 0)),
            Cs400 => None,
        }
    }

    /// Calculates the size of a chroma plane for this sampling type, given the luma plane dimensions.
    pub const fn get_chroma_dimensions(
        self,
        luma_width: usize,
        luma_height: usize,
    ) -> (usize, usize) {
        if let Some((ss_x, ss_y)) = self.get_decimation() {
            ((luma_width + ss_x) >> ss_x, (luma_height + ss_y) >> ss_y)
        } else {
            (0, 0)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    use wasm_bindgen_test::*;

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    wasm_bindgen_test_configure!(run_in_browser);

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn asm_stride() {
        let tests = [(7, 7, 14), (12, 12, 24), (1234, 1234, 2468)];

        for (in_stride, u8_bytes, u16_bytes) in tests {
            assert_eq!(u8::to_asm_stride(in_stride), u8_bytes);
            assert_eq!(u16::to_asm_stride(in_stride), u16_bytes);
        }
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn type_enum() {
        assert!(u8::type_enum() == PixelType::U8);
        assert!(u16::type_enum() == PixelType::U16);
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn chroma_sampling_from_int() {
        let expected = [
            (-1, None),
            (0, Some(ChromaSampling::Cs420)),
            (1, Some(ChromaSampling::Cs422)),
            (2, Some(ChromaSampling::Cs444)),
            (3, Some(ChromaSampling::Cs400)),
            (4, None),
        ];

        for (int, chroma_sampling) in expected {
            let converted = ChromaSampling::from_i32(int);
            assert_eq!(chroma_sampling, converted);

            let converted_uint = ChromaSampling::from_u32(int as u32);
            assert_eq!(chroma_sampling, converted_uint);
        }
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn display_chroma_sampling() {
        use std::fmt::Write;

        let all_cs = [
            (ChromaSampling::Cs420, "4:2:0"),
            (ChromaSampling::Cs422, "4:2:2"),
            (ChromaSampling::Cs444, "4:4:4"),
            (ChromaSampling::Cs400, "Monochrome"),
        ];

        for (cs, expected) in all_cs {
            let mut s = String::new();
            write!(&mut s, "{cs}").expect("can display");
            assert_eq!(&s, expected);
        }
    }

    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), wasm_bindgen_test)]
    #[test]
    fn chroma_sampling_dimensions() {
        let tests = [
            ((1024, 768), ChromaSampling::Cs444, (1024, 768)),
            ((1024, 768), ChromaSampling::Cs422, (512, 768)),
            ((1024, 768), ChromaSampling::Cs420, (512, 384)),
            ((1024, 768), ChromaSampling::Cs400, (0, 0)),
            // Check odd width/height
            ((1023, 768), ChromaSampling::Cs422, (512, 768)),
            ((1023, 767), ChromaSampling::Cs420, (512, 384)),
        ];

        for (luma, cs, expected_chroma) in tests {
            let chroma = cs.get_chroma_dimensions(luma.0, luma.1);
            assert_eq!(chroma, expected_chroma);
        }
    }
}
