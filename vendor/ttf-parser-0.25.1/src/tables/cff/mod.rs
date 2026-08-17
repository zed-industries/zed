mod argstack;
pub mod cff1;
#[cfg(feature = "variable-fonts")]
pub mod cff2;
mod charset;
mod charstring;
mod dict;
mod encoding;
mod index;
#[cfg(feature = "glyph-names")]
mod std_names;

use core::convert::TryFrom;

use crate::parser::{FromData, TryNumFrom};
use crate::{OutlineBuilder, RectF};

/// A list of errors that can occur during a CFF glyph outlining.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CFFError {
    NoGlyph,
    ReadOutOfBounds,
    ZeroBBox,
    InvalidOperator,
    UnsupportedOperator,
    MissingEndChar,
    DataAfterEndChar,
    NestingLimitReached,
    ArgumentsStackLimitReached,
    InvalidArgumentsStackLength,
    BboxOverflow,
    MissingMoveTo,
    InvalidSubroutineIndex,
    NoLocalSubroutines,
    InvalidSeacCode,
    #[cfg(feature = "variable-fonts")]
    InvalidItemVariationDataIndex,
    #[cfg(feature = "variable-fonts")]
    InvalidNumberOfBlendOperands,
    #[cfg(feature = "variable-fonts")]
    BlendRegionsLimitReached,
}

pub(crate) struct Builder<'a> {
    builder: &'a mut dyn OutlineBuilder,
    bbox: RectF,
}

impl<'a> Builder<'a> {
    #[inline]
    fn move_to(&mut self, x: f32, y: f32) {
        self.bbox.extend_by(x, y);
        self.builder.move_to(x, y);
    }

    #[inline]
    fn line_to(&mut self, x: f32, y: f32) {
        self.bbox.extend_by(x, y);
        self.builder.line_to(x, y);
    }

    #[inline]
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.bbox.extend_by(x1, y1);
        self.bbox.extend_by(x2, y2);
        self.bbox.extend_by(x, y);
        self.builder.curve_to(x1, y1, x2, y2, x, y);
    }

    #[inline]
    fn close(&mut self) {
        self.builder.close();
    }
}

/// A type-safe wrapper for string ID.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug)]
pub struct StringId(u16);

impl FromData for StringId {
    const SIZE: usize = 2;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        u16::parse(data).map(StringId)
    }
}

pub trait IsEven {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}

impl IsEven for usize {
    #[inline]
    fn is_even(&self) -> bool {
        (*self) & 1 == 0
    }

    #[inline]
    fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

#[cfg(feature = "std")]
#[inline]
pub fn f32_abs(n: f32) -> f32 {
    n.abs()
}

#[cfg(not(feature = "std"))]
#[inline]
pub fn f32_abs(n: f32) -> f32 {
    if n.is_sign_negative() {
        -n
    } else {
        n
    }
}

#[inline]
pub fn conv_subroutine_index(index: f32, bias: u16) -> Result<u32, CFFError> {
    conv_subroutine_index_impl(index, bias).ok_or(CFFError::InvalidSubroutineIndex)
}

#[inline]
fn conv_subroutine_index_impl(index: f32, bias: u16) -> Option<u32> {
    let index = i32::try_num_from(index)?;
    let bias = i32::from(bias);

    let index = index.checked_add(bias)?;
    u32::try_from(index).ok()
}

// Adobe Technical Note #5176, Chapter 16 "Local / Global Subrs INDEXes"
#[inline]
pub fn calc_subroutine_bias(len: u32) -> u16 {
    if len < 1240 {
        107
    } else if len < 33900 {
        1131
    } else {
        32768
    }
}
