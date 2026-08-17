// Copyright 2019 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A 2D size.

use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::common::FloatExt;
use crate::{Axis, Rect, RoundedRect, RoundedRectRadii, Vec2};

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

    /// A size with width and height set to `f64::INFINITY`.
    pub const INFINITY: Size = Size::new(f64::INFINITY, f64::INFINITY);

    /// Create a new `Size` with the provided `width` and `height`.
    #[inline(always)]
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
    pub const fn max_side(self) -> f64 {
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
    pub const fn min_side(self) -> f64 {
        self.width.min(self.height)
    }

    /// The area covered by this size.
    #[inline]
    pub const fn area(self) -> f64 {
        self.width * self.height
    }

    /// Whether this size has zero area.
    #[doc(alias = "is_empty")]
    #[inline]
    pub const fn is_zero_area(self) -> bool {
        self.area() == 0.0
    }

    /// Returns the component-wise minimum of `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Size;
    ///
    /// let this = Size::new(0., 100.);
    /// let other = Size::new(10., 10.);
    ///
    /// assert_eq!(this.min(other), Size::new(0., 10.));
    /// ```
    pub const fn min(self, other: Size) -> Self {
        Size {
            width: self.width.min(other.width),
            height: self.height.min(other.height),
        }
    }

    /// Returns the component-wise maximum of `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kurbo::Size;
    ///
    /// let this = Size::new(0., 100.);
    /// let other = Size::new(10., 10.);
    ///
    /// assert_eq!(this.max(other), Size::new(10., 100.));
    /// ```
    pub const fn max(self, other: Size) -> Self {
        Size {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
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
    pub const fn clamp(self, min: Size, max: Size) -> Self {
        self.max(min).min(max)
    }

    /// Convert this size into a [`Vec2`], with `width` mapped to `x` and `height`
    /// mapped to `y`.
    #[inline(always)]
    pub const fn to_vec2(self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    /// Returns a new `Size`,
    /// with `width` and `height` [rounded] to the nearest integer.
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
    ///
    /// [rounded]: f64::round
    #[inline]
    pub fn round(self) -> Size {
        Size::new(self.width.round(), self.height.round())
    }

    /// Returns a new `Size`,
    /// with `width` and `height` [rounded up] to the nearest integer,
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
    ///
    /// [rounded up]: f64::ceil
    #[inline]
    pub fn ceil(self) -> Size {
        Size::new(self.width.ceil(), self.height.ceil())
    }

    /// Returns a new `Size`,
    /// with `width` and `height` [rounded down] to the nearest integer,
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
    ///
    /// [rounded down]: f64::floor
    #[inline]
    pub fn floor(self) -> Size {
        Size::new(self.width.floor(), self.height.floor())
    }

    /// Returns a new `Size`,
    /// with `width` and `height` [rounded away] from zero to the nearest integer,
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
    ///
    /// [rounded away]: FloatExt::expand
    #[inline]
    pub fn expand(self) -> Size {
        Size::new(self.width.expand(), self.height.expand())
    }

    /// Returns a new `Size`,
    /// with `width` and `height` [rounded towards] zero to the nearest integer,
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
    ///
    /// [rounded towards]: f64::trunc
    #[inline]
    pub fn trunc(self) -> Size {
        Size::new(self.width.trunc(), self.height.trunc())
    }

    /// Returns the aspect ratio of a rectangle with this size.
    ///
    /// The aspect ratio is the ratio of the width to the height.
    ///
    /// If the height is `0`, the output will be `sign(self.width) * infinity`. If the width and
    /// height are both `0`, then the output will be `NaN`.
    #[inline]
    pub const fn aspect_ratio_width(self) -> f64 {
        // ratio is determined by width / height
        // https://en.wikipedia.org/wiki/Aspect_ratio_(image)
        // https://en.wikipedia.org/wiki/Ratio
        self.width / self.height
    }

    /// Returns **the inverse** of the aspect ratio of a rectangle with this size.
    ///
    /// Aspect ratios are usually defined as the ratio of the width to the height, but
    /// this method incorrectly returns the ratio of height to width.
    /// You should generally prefer [`aspect_ratio_width`](Self::aspect_ratio_width).
    ///
    /// If the width is `0`, the output will be `sign(self.height) * infinity`. If the width and
    /// height are both `0`, then the output will be `NaN`.
    #[deprecated(
        note = "You should use `aspect_ratio_width` instead, as this method returns a potentially unexpected value.",
        since = "0.12.0"
    )]
    #[inline]
    // TODO: When we remove this, we should also work out what to do with aspect_ratio_width
    // The tentatitive plan is to rename `aspect_ratio_width` back to this name
    pub const fn aspect_ratio(self) -> f64 {
        self.height / self.width
    }

    /// Convert this `Size` into a [`Rect`] with origin `(0.0, 0.0)`.
    #[inline(always)]
    pub const fn to_rect(self) -> Rect {
        Rect::new(0., 0., self.width, self.height)
    }

    /// Convert this `Size` into a [`RoundedRect`] with origin `(0.0, 0.0)` and
    /// the provided corner radius.
    #[inline]
    pub fn to_rounded_rect(self, radii: impl Into<RoundedRectRadii>) -> RoundedRect {
        self.to_rect().to_rounded_rect(radii)
    }

    /// Is this size [finite]?
    ///
    /// [finite]: f64::is_finite
    #[inline]
    pub const fn is_finite(self) -> bool {
        self.width.is_finite() && self.height.is_finite()
    }

    /// Is this size [NaN]?
    ///
    /// [NaN]: f64::is_nan
    #[inline]
    pub const fn is_nan(self) -> bool {
        self.width.is_nan() || self.height.is_nan()
    }

    /// Get the member matching the given axis.
    #[inline]
    pub const fn get_coord(self, axis: Axis) -> f64 {
        match axis {
            Axis::Horizontal => self.width,
            Axis::Vertical => self.height,
        }
    }

    /// Get a mutable reference to the member matching the given axis.
    #[inline]
    pub const fn get_coord_mut(&mut self, axis: Axis) -> &mut f64 {
        match axis {
            Axis::Horizontal => &mut self.width,
            Axis::Vertical => &mut self.height,
        }
    }

    /// Set the member matching the given axis to the given value.
    #[inline]
    pub const fn set_coord(&mut self, axis: Axis, value: f64) {
        match axis {
            Axis::Horizontal => self.width = value,
            Axis::Vertical => self.height = value,
        }
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
    #[inline(always)]
    fn from(v: (f64, f64)) -> Size {
        Size {
            width: v.0,
            height: v.1,
        }
    }
}

impl From<Size> for (f64, f64) {
    #[inline(always)]
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
    #[expect(deprecated, reason = "Testing deprecated function.")]
    fn aspect_ratio() {
        let s = Size::new(1.0, 1.0);
        assert!((s.aspect_ratio() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn aspect_ratio_width() {
        let s = Size::new(1.0, 1.0);
        assert!((s.aspect_ratio_width() - 1.0).abs() < 1e-6);

        // 3:2 film (mm)
        let s = Size::new(36.0, 24.0);
        assert!((s.aspect_ratio_width() - 1.5).abs() < 1e-6);
        // 4k screen
        let s = Size::new(3840.0, 2160.0);
        assert!((s.aspect_ratio_width() - (16. / 9.)).abs() < 1e-6);
    }
}
