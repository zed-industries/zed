// Copyright 2019 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A 2D size.

use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::common::FloatExt;
use crate::{Rect, RoundedRect, RoundedRectRadii, Vec2};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// A 2D size.
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size {
    /// The width.
    pub width: f64,
    /// The height.
    pub height: f64,
}

impl Size {
    /// A size with zero width or height.
    pub const ZERO: Size = Size::new(0., 0.);

    /// Create a new `Size` with the provided `width` and `height`.
    #[inline]
    pub const fn new(width: f64, height: f64) -> Self {
        Size { width, height }
    }

    /// Returns the max of `width` and `height`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Size;
    /// let size = Size::new(-10.5, 42.0);
    /// assert_eq!(size.max_side(), 42.0);
    /// ```
    pub fn max_side(self) -> f64 {
        self.width.max(self.height)
    }

    /// Returns the min of `width` and `height`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Size;
    /// let size = Size::new(-10.5, 42.0);
    /// assert_eq!(size.min_side(), -10.5);
    /// ```
    pub fn min_side(self) -> f64 {
        self.width.min(self.height)
    }

    /// The area covered by this size.
    #[inline]
    pub fn area(self) -> f64 {
        self.width * self.height
    }

    /// Whether this size has zero area.
    ///
    /// Note: a size with negative area is not considered empty.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.area() == 0.0
    }

    /// Returns a new size bounded by `min` and `max.`
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Size;
    ///
    /// let this = Size::new(0., 100.);
    /// let min = Size::new(10., 10.,);
    /// let max = Size::new(50., 50.);
    /// assert_eq!(this.clamp(min, max), Size::new(10., 50.))
    /// ```
    pub fn clamp(self, min: Size, max: Size) -> Self {
        let width = self.width.max(min.width).min(max.width);
        let height = self.height.max(min.height).min(max.height);
        Size { width, height }
    }

    /// Convert this size into a [`Vec2`], with `width` mapped to `x` and `height`
    /// mapped to `y`.
    #[inline]
    pub const fn to_vec2(self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    /// Returns a new `Size`,
    /// with `width` and `height` rounded to the nearest integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Size;
    /// let size_pos = Size::new(3.3, 3.6).round();
    /// assert_eq!(size_pos.width, 3.0);
    /// assert_eq!(size_pos.height, 4.0);
    /// let size_neg = Size::new(-3.3, -3.6).round();
    /// assert_eq!(size_neg.width, -3.0);
    /// assert_eq!(size_neg.height, -4.0);
    /// ```
    #[inline]
    pub fn round(self) -> Size {
        Size::new(self.width.round(), self.height.round())
    }

    /// Returns a new `Size`,
    /// with `width` and `height` rounded up to the nearest integer,
    /// unless they are already an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Size;
    /// let size_pos = Size::new(3.3, 3.6).ceil();
    /// assert_eq!(size_pos.width, 4.0);
    /// assert_eq!(size_pos.height, 4.0);
    /// let size_neg = Size::new(-3.3, -3.6).ceil();
    /// assert_eq!(size_neg.width, -3.0);
    /// assert_eq!(size_neg.height, -3.0);
    /// ```
    #[inline]
    pub fn ceil(self) -> Size {
        Size::new(self.width.ceil(), self.height.ceil())
    }

    /// Returns a new `Size`,
    /// with `width` and `height` rounded down to the nearest integer,
    /// unless they are already an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Size;
    /// let size_pos = Size::new(3.3, 3.6).floor();
    /// assert_eq!(size_pos.width, 3.0);
    /// assert_eq!(size_pos.height, 3.0);
    /// let size_neg = Size::new(-3.3, -3.6).floor();
    /// assert_eq!(size_neg.width, -4.0);
    /// assert_eq!(size_neg.height, -4.0);
    /// ```
    #[inline]
    pub fn floor(self) -> Size {
        Size::new(self.width.floor(), self.height.floor())
    }

    /// Returns a new `Size`,
    /// with `width` and `height` rounded away from zero to the nearest integer,
    /// unless they are already an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Size;
    /// let size_pos = Size::new(3.3, 3.6).expand();
    /// assert_eq!(size_pos.width, 4.0);
    /// assert_eq!(size_pos.height, 4.0);
    /// let size_neg = Size::new(-3.3, -3.6).expand();
    /// assert_eq!(size_neg.width, -4.0);
    /// assert_eq!(size_neg.height, -4.0);
    /// ```
    #[inline]
    pub fn expand(self) -> Size {
        Size::new(self.width.expand(), self.height.expand())
    }

    /// Returns a new `Size`,
    /// with `width` and `height` rounded down towards zero the nearest integer,
    /// unless they are already an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Size;
    /// let size_pos = Size::new(3.3, 3.6).trunc();
    /// assert_eq!(size_pos.width, 3.0);
    /// assert_eq!(size_pos.height, 3.0);
    /// let size_neg = Size::new(-3.3, -3.6).trunc();
    /// assert_eq!(size_neg.width, -3.0);
    /// assert_eq!(size_neg.height, -3.0);
    /// ```
    #[inline]
    pub fn trunc(self) -> Size {
        Size::new(self.width.trunc(), self.height.trunc())
    }

    /// Returns the aspect ratio of a rectangle with the given size.
    ///
    /// If the width is `0`, the output will be `sign(self.height) * infinity`. If The width and
    /// height are `0`, then the output will be `NaN`.
    pub fn aspect_ratio(self) -> f64 {
        self.height / self.width
    }

    /// Convert this `Size` into a [`Rect`] with origin `(0.0, 0.0)`.
    #[inline]
    pub const fn to_rect(self) -> Rect {
        Rect::new(0., 0., self.width, self.height)
    }

    /// Convert this `Size` into a [`RoundedRect`] with origin `(0.0, 0.0)` and
    /// the provided corner radius.
    #[inline]
    pub fn to_rounded_rect(self, radii: impl Into<RoundedRectRadii>) -> RoundedRect {
        self.to_rect().to_rounded_rect(radii)
    }

    /// Is this size finite?
    #[inline]
    pub fn is_finite(self) -> bool {
        self.width.is_finite() && self.height.is_finite()
    }

    /// Is this size NaN?
    #[inline]
    pub fn is_nan(self) -> bool {
        self.width.is_nan() || self.height.is_nan()
    }
}

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}W×{:?}H", self.width, self.height)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "(")?;
        fmt::Display::fmt(&self.width, formatter)?;
        write!(formatter, "×")?;
        fmt::Display::fmt(&self.height, formatter)?;
        write!(formatter, ")")
    }
}

impl MulAssign<f64> for Size {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        *self = Size {
            width: self.width * other,
            height: self.height * other,
        };
    }
}

impl Mul<Size> for f64 {
    type Output = Size;

    #[inline]
    fn mul(self, other: Size) -> Size {
        other * self
    }
}

impl Mul<f64> for Size {
    type Output = Size;

    #[inline]
    fn mul(self, other: f64) -> Size {
        Size {
            width: self.width * other,
            height: self.height * other,
        }
    }
}

impl DivAssign<f64> for Size {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        *self = Size {
            width: self.width / other,
            height: self.height / other,
        };
    }
}

impl Div<f64> for Size {
    type Output = Size;

    #[inline]
    fn div(self, other: f64) -> Size {
        Size {
            width: self.width / other,
            height: self.height / other,
        }
    }
}

impl Add<Size> for Size {
    type Output = Size;
    #[inline]
    fn add(self, other: Size) -> Size {
        Size {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl AddAssign<Size> for Size {
    #[inline]
    fn add_assign(&mut self, other: Size) {
        *self = *self + other;
    }
}

impl Sub<Size> for Size {
    type Output = Size;
    #[inline]
    fn sub(self, other: Size) -> Size {
        Size {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl SubAssign<Size> for Size {
    #[inline]
    fn sub_assign(&mut self, other: Size) {
        *self = *self - other;
    }
}

impl From<(f64, f64)> for Size {
    #[inline]
    fn from(v: (f64, f64)) -> Size {
        Size {
            width: v.0,
            height: v.1,
        }
    }
}

impl From<Size> for (f64, f64) {
    #[inline]
    fn from(v: Size) -> (f64, f64) {
        (v.width, v.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let s = Size::new(-0.12345, 9.87654);
        assert_eq!(format!("{s}"), "(-0.12345×9.87654)");

        let s = Size::new(-0.12345, 9.87654);
        assert_eq!(format!("{s:+6.2}"), "( -0.12× +9.88)");
    }

    #[test]
    fn aspect_ratio() {
        let s = Size::new(1.0, 1.0);
        assert!((s.aspect_ratio() - 1.0).abs() < 1e-6);
    }
}
