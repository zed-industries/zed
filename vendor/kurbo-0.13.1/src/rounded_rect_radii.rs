// Copyright 2021 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A description of the radii for each corner of a rounded rectangle.

use core::convert::From;

#[allow(unused_imports)] // This is unused in later versions of Rust because of additions to core::f32
#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// Radii for each corner of a rounded rectangle.
///
/// The use of `top` as in `top_left` assumes a y-down coordinate space. Piet
/// (and Druid by extension) uses a y-down coordinate space, but Kurbo also
/// supports a y-up coordinate space, in which case `top_left` would actually
/// refer to the bottom-left corner, and vice versa. Top may not always
/// actually be the top, but `top` corners will always have a smaller y-value
/// than `bottom` corners.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RoundedRectRadii {
    /// The radius of the top-left corner.
    pub top_left: f64,
    /// The radius of the top-right corner.
    pub top_right: f64,
    /// The radius of the bottom-right corner.
    pub bottom_right: f64,
    /// The radius of the bottom-left corner.
    pub bottom_left: f64,
}

impl RoundedRectRadii {
    /// Create a new `RoundedRectRadii`. This function takes radius values for
    /// the four corners. The argument order is `top_left`, `top_right`,
    /// `bottom_right`, `bottom_left`, or clockwise starting from `top_left`.
    #[inline(always)]
    pub const fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        RoundedRectRadii {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }

    /// Create a new `RoundedRectRadii` from a single radius. The `radius`
    /// argument will be set as the radius for all four corners.
    #[inline(always)]
    pub const fn from_single_radius(radius: f64) -> Self {
        RoundedRectRadii {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    /// Takes the absolute value of all corner radii.
    pub const fn abs(&self) -> Self {
        RoundedRectRadii::new(
            self.top_left.abs(),
            self.top_right.abs(),
            self.bottom_right.abs(),
            self.bottom_left.abs(),
        )
    }

    /// For each corner, takes the min of that corner's radius and `max`.
    pub const fn clamp(&self, max: f64) -> Self {
        RoundedRectRadii::new(
            self.top_left.min(max),
            self.top_right.min(max),
            self.bottom_right.min(max),
            self.bottom_left.min(max),
        )
    }

    /// Returns `true` if all radius values are finite.
    pub const fn is_finite(&self) -> bool {
        self.top_left.is_finite()
            && self.top_right.is_finite()
            && self.bottom_right.is_finite()
            && self.bottom_left.is_finite()
    }

    /// Returns `true` if any corner radius value is NaN.
    pub const fn is_nan(&self) -> bool {
        self.top_left.is_nan()
            || self.top_right.is_nan()
            || self.bottom_right.is_nan()
            || self.bottom_left.is_nan()
    }

    /// If all radii are equal, returns the value of the radii. Otherwise,
    /// returns `None`.
    pub const fn as_single_radius(&self) -> Option<f64> {
        let epsilon = 1e-9;

        if (self.top_left - self.top_right).abs() < epsilon
            && (self.top_right - self.bottom_right).abs() < epsilon
            && (self.bottom_right - self.bottom_left).abs() < epsilon
        {
            Some(self.top_left)
        } else {
            None
        }
    }
}

impl From<f64> for RoundedRectRadii {
    #[inline(always)]
    fn from(radius: f64) -> Self {
        RoundedRectRadii::from_single_radius(radius)
    }
}

impl From<(f64, f64, f64, f64)> for RoundedRectRadii {
    #[inline(always)]
    fn from(radii: (f64, f64, f64, f64)) -> Self {
        RoundedRectRadii::new(radii.0, radii.1, radii.2, radii.3)
    }
}
