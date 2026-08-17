//! Style types for representing lengths / sizes
use super::CompactLength;
use crate::geometry::Rect;
use crate::style_helpers::{FromLength, FromPercent, TaffyAuto, TaffyZero};
#[cfg(feature = "parse")]
use crate::util::parse::{from_str_from_css, CssParseResult, FromCss, Parser, Token};

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`](crate::geometry::Size).
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LengthPercentage(pub(crate) CompactLength);
impl TaffyZero for LengthPercentage {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl FromLength for LengthPercentage {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::length(value.into() as f32)
    }
}
impl FromPercent for LengthPercentage {
    fn from_percent<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::percent(value.into() as f32)
    }
}

#[cfg(feature = "parse")]
impl FromCss for LengthPercentage {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        match parser.next()?.clone() {
            Token::Percentage { unit_value, .. } => Ok(Self::percent(unit_value)),
            Token::Dimension { unit, value, .. } if unit == "px" => Ok(Self::length(value)),
            token => Err(parser.new_unexpected_token_error(token))?,
        }
    }
}
#[cfg(feature = "parse")]
from_str_from_css!(LengthPercentage);

impl LengthPercentage {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    #[inline(always)]
    pub const fn length(val: f32) -> Self {
        Self(CompactLength::length(val))
    }

    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    #[inline(always)]
    pub const fn percent(val: f32) -> Self {
        Self(CompactLength::percent(val))
    }

    /// A `calc()` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline(always)]
    #[cfg(feature = "calc")]
    pub fn calc(ptr: *const ()) -> Self {
        Self(CompactLength::calc(ptr))
    }

    /// Create a LengthPercentage from a raw `CompactLength`.
    /// # Safety
    /// CompactLength must represent a valid variant for LengthPercentage
    #[allow(unsafe_code)]
    pub const unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }

    /// Get the underlying `CompactLength` representation of the value
    pub const fn into_raw(self) -> CompactLength {
        self.0
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LengthPercentage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = CompactLength::deserialize(deserializer)?;
        // Note: validation intentionally excludes the CALC_TAG as deserializing calc() values is not supported
        if matches!(inner.tag(), CompactLength::LENGTH_TAG | CompactLength::PERCENT_TAG) {
            Ok(Self(inner))
        } else {
            Err(serde::de::Error::custom("Invalid tag"))
        }
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`](crate::geometry::Size).
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LengthPercentageAuto(pub(crate) CompactLength);
impl TaffyZero for LengthPercentageAuto {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl TaffyAuto for LengthPercentageAuto {
    const AUTO: Self = Self(CompactLength::AUTO);
}
impl FromLength for LengthPercentageAuto {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::length(value.into() as f32)
    }
}
impl FromPercent for LengthPercentageAuto {
    fn from_percent<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::percent(value.into() as f32)
    }
}
impl From<LengthPercentage> for LengthPercentageAuto {
    fn from(input: LengthPercentage) -> Self {
        Self(input.0)
    }
}

#[cfg(feature = "parse")]
impl FromCss for LengthPercentageAuto {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        match parser.next()?.clone() {
            Token::Percentage { unit_value, .. } => Ok(Self::percent(unit_value)),
            Token::Dimension { unit, value, .. } if unit == "px" => Ok(Self::length(value)),
            Token::Ident(ident) if ident == "auto" => Ok(Self::auto()),
            token => Err(parser.new_unexpected_token_error(token))?,
        }
    }
}
#[cfg(feature = "parse")]
from_str_from_css!(LengthPercentageAuto);

impl LengthPercentageAuto {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    #[inline(always)]
    pub const fn length(val: f32) -> Self {
        Self(CompactLength::length(val))
    }

    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    #[inline(always)]
    pub const fn percent(val: f32) -> Self {
        Self(CompactLength::percent(val))
    }

    /// The dimension should be automatically computed according to algorithm-specific rules
    /// regarding the default size of boxes.
    #[inline(always)]
    pub const fn auto() -> Self {
        Self(CompactLength::auto())
    }

    /// A `calc()` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline]
    #[cfg(feature = "calc")]
    pub fn calc(ptr: *const ()) -> Self {
        Self(CompactLength::calc(ptr))
    }

    /// Create a LengthPercentageAuto from a raw `CompactLength`.
    /// # Safety
    /// CompactLength must represent a valid variant for LengthPercentageAuto
    #[allow(unsafe_code)]
    pub const unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }

    /// Get the underlying `CompactLength` representation of the value
    pub const fn into_raw(self) -> CompactLength {
        self.0
    }

    /// Returns:
    ///   - Some(length) for Length variants
    ///   - Some(resolved) using the provided context for Percent variants
    ///   - None for Auto variants
    #[inline(always)]
    pub fn resolve_to_option(self, context: f32, calc_resolver: impl Fn(*const (), f32) -> f32) -> Option<f32> {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => Some(self.0.value()),
            CompactLength::PERCENT_TAG => Some(context * self.0.value()),
            CompactLength::AUTO_TAG => None,
            #[cfg(feature = "calc")]
            _ if self.0.is_calc() => Some(calc_resolver(self.0.calc_value(), context)),
            _ => unreachable!("LengthPercentageAuto values cannot be constructed with other tags"),
        }
    }

    /// Returns true if value is LengthPercentageAuto::Auto
    #[inline(always)]
    pub fn is_auto(self) -> bool {
        self.0.is_auto()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LengthPercentageAuto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = CompactLength::deserialize(deserializer)?;
        // Note: validation intentionally excludes the CALC_TAG as deserializing calc() values is not supported
        if matches!(inner.tag(), CompactLength::LENGTH_TAG | CompactLength::PERCENT_TAG | CompactLength::AUTO_TAG) {
            Ok(Self(inner))
        } else {
            Err(serde::de::Error::custom("Invalid tag"))
        }
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`](crate::geometry::Size).
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Dimension(pub(crate) CompactLength);
impl TaffyZero for Dimension {
    const ZERO: Self = Self(CompactLength::ZERO);
}
impl TaffyAuto for Dimension {
    const AUTO: Self = Self(CompactLength::AUTO);
}
impl FromLength for Dimension {
    fn from_length<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::length(value.into() as f32)
    }
}
impl FromPercent for Dimension {
    fn from_percent<Input: Into<f64> + Copy>(value: Input) -> Self {
        Self::percent(value.into() as f32)
    }
}
impl From<LengthPercentage> for Dimension {
    fn from(input: LengthPercentage) -> Self {
        Self(input.0)
    }
}
impl From<LengthPercentageAuto> for Dimension {
    fn from(input: LengthPercentageAuto) -> Self {
        Self(input.0)
    }
}

#[cfg(feature = "parse")]
impl FromCss for Dimension {
    fn from_css<'i>(parser: &mut Parser<'i, '_>) -> CssParseResult<'i, Self> {
        match parser.next()?.clone() {
            Token::Percentage { unit_value, .. } => Ok(Self::percent(unit_value)),
            Token::Dimension { unit, value, .. } if unit == "px" => Ok(Self::length(value)),
            Token::Ident(ident) if ident == "auto" => Ok(Self::auto()),
            token => Err(parser.new_unexpected_token_error(token))?,
        }
    }
}
#[cfg(feature = "parse")]
from_str_from_css!(Dimension);

impl Dimension {
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    #[inline(always)]
    pub const fn length(val: f32) -> Self {
        Self(CompactLength::length(val))
    }

    /// A percentage length relative to the size of the containing block.
    ///
    /// **NOTE: percentages are represented as a f32 value in the range [0.0, 1.0] NOT the range [0.0, 100.0]**
    #[inline(always)]
    pub const fn percent(val: f32) -> Self {
        Self(CompactLength::percent(val))
    }

    /// The dimension should be automatically computed according to algorithm-specific rules
    /// regarding the default size of boxes.
    #[inline(always)]
    pub const fn auto() -> Self {
        Self(CompactLength::auto())
    }

    /// A `calc()` value. The value passed here is treated as an opaque handle to
    /// the actual calc representation and may be a pointer, index, etc.
    ///
    /// The low 3 bits are used as a tag value and will be returned as 0.
    #[inline]
    #[cfg(feature = "calc")]
    pub fn calc(ptr: *const ()) -> Self {
        Self(CompactLength::calc(ptr))
    }

    /// Create a LengthPercentageAuto from a raw `CompactLength`.
    /// # Safety
    /// CompactLength must represent a valid variant for LengthPercentageAuto
    #[allow(unsafe_code)]
    pub const unsafe fn from_raw(val: CompactLength) -> Self {
        Self(val)
    }

    /// Get the underlying `CompactLength` representation of the value
    pub const fn into_raw(self) -> CompactLength {
        self.0
    }

    /// Get Length value if value is Length variant
    #[cfg(feature = "grid")]
    pub fn into_option(self) -> Option<f32> {
        match self.0.tag() {
            CompactLength::LENGTH_TAG => Some(self.0.value()),
            _ => None,
        }
    }
    /// Returns true if value is Auto
    #[inline(always)]
    pub fn is_auto(self) -> bool {
        self.0.is_auto()
    }

    /// Get the raw `CompactLength` tag
    pub fn tag(self) -> usize {
        self.0.tag()
    }

    /// Get the raw `CompactLength` value for non-calc variants that have a numeric parameter
    pub fn value(self) -> f32 {
        self.0.value()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Dimension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = CompactLength::deserialize(deserializer)?;
        // Note: validation intentionally excludes the CALC_TAG as deserializing calc() values is not supported
        if matches!(inner.tag(), CompactLength::LENGTH_TAG | CompactLength::PERCENT_TAG | CompactLength::AUTO_TAG) {
            Ok(Self(inner))
        } else {
            Err(serde::de::Error::custom("Invalid tag"))
        }
    }
}

impl Rect<Dimension> {
    /// Create a new Rect with length values
    #[must_use]
    pub const fn from_length(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension(CompactLength::length(start)),
            right: Dimension(CompactLength::length(end)),
            top: Dimension(CompactLength::length(top)),
            bottom: Dimension(CompactLength::length(bottom)),
        }
    }

    /// Create a new Rect with percentage values
    #[must_use]
    pub const fn from_percent(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension(CompactLength::percent(start)),
            right: Dimension(CompactLength::percent(end)),
            top: Dimension(CompactLength::percent(top)),
            bottom: Dimension(CompactLength::percent(bottom)),
        }
    }
}
