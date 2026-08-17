// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{Point, Size, Vec2};

/// An axis in the plane.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Axis {
    /// The x axis.
    Horizontal,
    /// The y axis.
    Vertical,
}

impl Axis {
    /// Get the axis perpendicular to this one.
    #[inline]
    pub const fn cross(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }

    /// Create a new [`Point`] by arranging the given magnitudes.
    ///
    /// The axis value is the one matching the axis (e.g. `y` for [`Self::Vertical`]).
    /// The cross value is the other one.
    #[inline]
    pub const fn pack_point(self, axis_value: f64, cross_value: f64) -> Point {
        match self {
            Self::Horizontal => Point::new(axis_value, cross_value),
            Self::Vertical => Point::new(cross_value, axis_value),
        }
    }

    /// Create a new [`Size`] by arranging the given magnitudes.
    ///
    /// The axis value is the one matching the axis (e.g. `height` for [`Self::Vertical`]).
    /// The cross value is the other one.
    #[inline]
    pub const fn pack_size(self, axis_value: f64, cross_value: f64) -> Size {
        match self {
            Self::Horizontal => Size::new(axis_value, cross_value),
            Self::Vertical => Size::new(cross_value, axis_value),
        }
    }

    /// Create a new [`Vec2`] by arranging the given magnitudes.
    ///
    /// The axis value is the one matching the axis (e.g. `y` for [`Self::Vertical`]).
    /// The cross value is the other one.
    #[inline]
    pub const fn pack_vec2(self, axis_value: f64, cross_value: f64) -> Vec2 {
        match self {
            Self::Horizontal => Vec2::new(axis_value, cross_value),
            Self::Vertical => Vec2::new(cross_value, axis_value),
        }
    }

    /// Create a new `(x, y)` pair by arranging the given magnitudes.
    ///
    /// The axis value is the one matching the axis (e.g. `y` for [`Self::Vertical`]).
    /// The cross value is the other one.
    #[inline]
    pub const fn pack_xy(self, axis_value: f64, cross_value: f64) -> (f64, f64) {
        match self {
            Self::Horizontal => (axis_value, cross_value),
            Self::Vertical => (cross_value, axis_value),
        }
    }
}
