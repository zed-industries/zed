// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use euclid::UnknownUnit;

impl From<euclid::Vector2D<f64, UnknownUnit>> for crate::Vec2 {
    #[inline(always)]
    fn from(value: euclid::Vector2D<f64, UnknownUnit>) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<crate::Vec2> for euclid::Vector2D<f64, UnknownUnit> {
    #[inline(always)]
    fn from(value: crate::Vec2) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<euclid::Point2D<f64, UnknownUnit>> for crate::Point {
    #[inline(always)]
    fn from(value: euclid::Point2D<f64, UnknownUnit>) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<crate::Point> for euclid::Point2D<f64, UnknownUnit> {
    #[inline(always)]
    fn from(value: crate::Point) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<euclid::Size2D<f64, UnknownUnit>> for crate::Size {
    #[inline(always)]
    fn from(value: euclid::Size2D<f64, UnknownUnit>) -> Self {
        Self::new(value.width, value.height)
    }
}

impl From<crate::Size> for euclid::Size2D<f64, UnknownUnit> {
    #[inline(always)]
    fn from(value: crate::Size) -> Self {
        Self::new(value.width, value.height)
    }
}

impl From<euclid::Rect<f64, UnknownUnit>> for crate::Rect {
    #[inline]
    fn from(value: euclid::Rect<f64, UnknownUnit>) -> Self {
        Self::from_origin_size(value.origin, value.size)
    }
}

impl From<crate::Rect> for euclid::Rect<f64, UnknownUnit> {
    #[inline]
    fn from(value: crate::Rect) -> Self {
        Self::new(value.origin().into(), value.size().into())
    }
}

impl From<euclid::Box2D<f64, UnknownUnit>> for crate::Rect {
    #[inline]
    fn from(value: euclid::Box2D<f64, UnknownUnit>) -> Self {
        Self::from_points(value.min, value.max)
    }
}

impl From<crate::Rect> for euclid::Box2D<f64, UnknownUnit> {
    #[inline]
    fn from(value: crate::Rect) -> Self {
        Self::new(
            euclid::Point2D::new(value.min_x(), value.min_y()),
            euclid::Point2D::new(value.max_x(), value.max_y()),
        )
    }
}

impl From<euclid::Transform2D<f64, UnknownUnit, UnknownUnit>> for crate::Affine {
    #[inline(always)]
    fn from(t: euclid::Transform2D<f64, UnknownUnit, UnknownUnit>) -> Self {
        Self::new(t.to_array())
    }
}

impl From<crate::Affine> for euclid::Transform2D<f64, UnknownUnit, UnknownUnit> {
    #[inline(always)]
    fn from(a: crate::Affine) -> Self {
        Self::from_array(a.as_coeffs())
    }
}
