// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::{NonZeroRect, Point};

use crate::scalar::{Scalar, SCALAR_NEARLY_ZERO};

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use crate::NoStdFloat;

/// An affine transformation matrix.
///
/// Unlike other types, doesn't guarantee to be valid. This is Skia quirk.
/// Meaning Transform(0, 0, 0, 0, 0, 0) is ok, while it's technically not.
/// Non-finite values are also not an error.
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Transform {
    pub sx: f32,
    pub kx: f32,
    pub ky: f32,
    pub sy: f32,
    pub tx: f32,
    pub ty: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            sx: 1.0,
            kx: 0.0,
            ky: 0.0,
            sy: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
}

impl Transform {
    /// Creates an identity transform.
    pub fn identity() -> Self {
        Transform::default()
    }

    /// Creates a new `Transform`.
    ///
    /// We are using column-major-column-vector matrix notation, therefore it's ky-kx, not kx-ky.
    pub fn from_row(sx: f32, ky: f32, kx: f32, sy: f32, tx: f32, ty: f32) -> Self {
        Transform {
            sx,
            ky,
            kx,
            sy,
            tx,
            ty,
        }
    }

    /// Creates a new translating `Transform`.
    pub fn from_translate(tx: f32, ty: f32) -> Self {
        Transform::from_row(1.0, 0.0, 0.0, 1.0, tx, ty)
    }

    /// Creates a new scaling `Transform`.
    pub fn from_scale(sx: f32, sy: f32) -> Self {
        Transform::from_row(sx, 0.0, 0.0, sy, 0.0, 0.0)
    }

    /// Creates a new skewing `Transform`.
    pub fn from_skew(kx: f32, ky: f32) -> Self {
        Transform::from_row(1.0, ky, kx, 1.0, 0.0, 0.0)
    }

    /// Creates a new rotating `Transform`.
    ///
    /// `angle` in degrees.
    pub fn from_rotate(angle: f32) -> Self {
        let v = angle.to_radians();
        let a = v.cos();
        let b = v.sin();
        let c = -b;
        let d = a;
        Transform::from_row(a, b, c, d, 0.0, 0.0)
    }

    /// Creates a new rotating `Transform` at the specified position.
    ///
    /// `angle` in degrees.
    pub fn from_rotate_at(angle: f32, tx: f32, ty: f32) -> Self {
        let mut ts = Self::default();
        ts = ts.pre_translate(tx, ty);
        ts = ts.pre_concat(Transform::from_rotate(angle));
        ts = ts.pre_translate(-tx, -ty);
        ts
    }

    /// Converts `Rect` into a bounding box `Transform`.
    #[inline]
    pub fn from_bbox(bbox: NonZeroRect) -> Self {
        Transform::from_row(bbox.width(), 0.0, 0.0, bbox.height(), bbox.x(), bbox.y())
    }

    /// Checks that transform is finite.
    pub fn is_finite(&self) -> bool {
        self.sx.is_finite()
            && self.ky.is_finite()
            && self.kx.is_finite()
            && self.sy.is_finite()
            && self.tx.is_finite()
            && self.ty.is_finite()
    }

    /// Checks that transform is finite and has non-zero scale.
    pub fn is_valid(&self) -> bool {
        if self.is_finite() {
            let (sx, sy) = self.get_scale();
            !(sx.is_nearly_zero_within_tolerance(f32::EPSILON)
                || sy.is_nearly_zero_within_tolerance(f32::EPSILON))
        } else {
            false
        }
    }

    /// Checks that transform is identity.
    pub fn is_identity(&self) -> bool {
        *self == Transform::default()
    }

    /// Checks that transform is scale-only.
    pub fn is_scale(&self) -> bool {
        self.has_scale() && !self.has_skew() && !self.has_translate()
    }

    /// Checks that transform is skew-only.
    pub fn is_skew(&self) -> bool {
        !self.has_scale() && self.has_skew() && !self.has_translate()
    }

    /// Checks that transform is translate-only.
    pub fn is_translate(&self) -> bool {
        !self.has_scale() && !self.has_skew() && self.has_translate()
    }

    /// Checks that transform contains only scale and translate.
    pub fn is_scale_translate(&self) -> bool {
        (self.has_scale() || self.has_translate()) && !self.has_skew()
    }

    /// Checks that transform contains a scale part.
    pub fn has_scale(&self) -> bool {
        self.sx != 1.0 || self.sy != 1.0
    }

    /// Checks that transform contains a skew part.
    pub fn has_skew(&self) -> bool {
        self.kx != 0.0 || self.ky != 0.0
    }

    /// Checks that transform contains a translate part.
    pub fn has_translate(&self) -> bool {
        self.tx != 0.0 || self.ty != 0.0
    }

    /// Returns transform's scale part.
    pub fn get_scale(&self) -> (f32, f32) {
        let x_scale = (self.sx * self.sx + self.kx * self.kx).sqrt();
        let y_scale = (self.ky * self.ky + self.sy * self.sy).sqrt();
        (x_scale, y_scale)
    }

    /// Pre-scales the current transform.
    #[must_use]
    pub fn pre_scale(&self, sx: f32, sy: f32) -> Self {
        self.pre_concat(Transform::from_scale(sx, sy))
    }

    /// Post-scales the current transform.
    #[must_use]
    pub fn post_scale(&self, sx: f32, sy: f32) -> Self {
        self.post_concat(Transform::from_scale(sx, sy))
    }

    /// Pre-translates the current transform.
    #[must_use]
    pub fn pre_translate(&self, tx: f32, ty: f32) -> Self {
        self.pre_concat(Transform::from_translate(tx, ty))
    }

    /// Post-translates the current transform.
    #[must_use]
    pub fn post_translate(&self, tx: f32, ty: f32) -> Self {
        self.post_concat(Transform::from_translate(tx, ty))
    }

    /// Pre-rotates the current transform.
    ///
    /// `angle` in degrees.
    #[must_use]
    pub fn pre_rotate(&self, angle: f32) -> Self {
        self.pre_concat(Transform::from_rotate(angle))
    }

    /// Post-rotates the current transform.
    ///
    /// `angle` in degrees.
    #[must_use]
    pub fn post_rotate(&self, angle: f32) -> Self {
        self.post_concat(Transform::from_rotate(angle))
    }

    /// Pre-rotates the current transform by the specified position.
    ///
    /// `angle` in degrees.
    #[must_use]
    pub fn pre_rotate_at(&self, angle: f32, tx: f32, ty: f32) -> Self {
        self.pre_concat(Transform::from_rotate_at(angle, tx, ty))
    }

    /// Post-rotates the current transform by the specified position.
    ///
    /// `angle` in degrees.
    #[must_use]
    pub fn post_rotate_at(&self, angle: f32, tx: f32, ty: f32) -> Self {
        self.post_concat(Transform::from_rotate_at(angle, tx, ty))
    }

    /// Pre-concats the current transform.
    #[must_use]
    pub fn pre_concat(&self, other: Self) -> Self {
        concat(*self, other)
    }

    /// Post-concats the current transform.
    #[must_use]
    pub fn post_concat(&self, other: Self) -> Self {
        concat(other, *self)
    }

    pub(crate) fn from_sin_cos(sin: f32, cos: f32) -> Self {
        Transform::from_row(cos, sin, -sin, cos, 0.0, 0.0)
    }

    /// Transforms a points using the current transform.
    pub fn map_point(&self, point: &mut Point) {
        if self.is_identity() {
            // Do nothing.
        } else if self.is_translate() {
            point.x += self.tx;
            point.y += self.ty;
        } else if self.is_scale_translate() {
            point.x = point.x * self.sx + self.tx;
            point.y = point.y * self.sy + self.ty;
        } else {
            let x = point.x * self.sx + point.y * self.kx + self.tx;
            let y = point.x * self.ky + point.y * self.sy + self.ty;
            point.x = x;
            point.y = y;
        }
    }

    /// Transforms a slice of points using the current transform.
    pub fn map_points(&self, points: &mut [Point]) {
        if points.is_empty() {
            return;
        }

        // TODO: simd

        if self.is_identity() {
            // Do nothing.
        } else if self.is_translate() {
            for p in points {
                p.x += self.tx;
                p.y += self.ty;
            }
        } else if self.is_scale_translate() {
            for p in points {
                p.x = p.x * self.sx + self.tx;
                p.y = p.y * self.sy + self.ty;
            }
        } else {
            for p in points {
                let x = p.x * self.sx + p.y * self.kx + self.tx;
                let y = p.x * self.ky + p.y * self.sy + self.ty;
                p.x = x;
                p.y = y;
            }
        }
    }

    /// Returns an inverted transform.
    pub fn invert(&self) -> Option<Self> {
        // Allow the trivial case to be inlined.
        if self.is_identity() {
            return Some(*self);
        }

        invert(self)
    }
}

fn invert(ts: &Transform) -> Option<Transform> {
    debug_assert!(!ts.is_identity());

    if ts.is_scale_translate() {
        if ts.has_scale() {
            let inv_x = ts.sx.invert();
            let inv_y = ts.sy.invert();
            Some(Transform::from_row(
                inv_x,
                0.0,
                0.0,
                inv_y,
                -ts.tx * inv_x,
                -ts.ty * inv_y,
            ))
        } else {
            // translate only
            Some(Transform::from_translate(-ts.tx, -ts.ty))
        }
    } else {
        let inv_det = inv_determinant(ts)?;
        let inv_ts = compute_inv(ts, inv_det);

        if inv_ts.is_finite() {
            Some(inv_ts)
        } else {
            None
        }
    }
}

fn inv_determinant(ts: &Transform) -> Option<f64> {
    let det = dcross(ts.sx as f64, ts.sy as f64, ts.kx as f64, ts.ky as f64);

    // Since the determinant is on the order of the cube of the matrix members,
    // compare to the cube of the default nearly-zero constant (although an
    // estimate of the condition number would be better if it wasn't so expensive).
    let tolerance = SCALAR_NEARLY_ZERO * SCALAR_NEARLY_ZERO * SCALAR_NEARLY_ZERO;
    if (det as f32).is_nearly_zero_within_tolerance(tolerance) {
        None
    } else {
        Some(1.0 / det)
    }
}

fn compute_inv(ts: &Transform, inv_det: f64) -> Transform {
    Transform::from_row(
        (ts.sy as f64 * inv_det) as f32,
        (-ts.ky as f64 * inv_det) as f32,
        (-ts.kx as f64 * inv_det) as f32,
        (ts.sx as f64 * inv_det) as f32,
        dcross_dscale(ts.kx, ts.ty, ts.sy, ts.tx, inv_det),
        dcross_dscale(ts.ky, ts.tx, ts.sx, ts.ty, inv_det),
    )
}

fn dcross(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * b - c * d
}

fn dcross_dscale(a: f32, b: f32, c: f32, d: f32, scale: f64) -> f32 {
    (dcross(a as f64, b as f64, c as f64, d as f64) * scale) as f32
}

fn concat(a: Transform, b: Transform) -> Transform {
    if a.is_identity() {
        b
    } else if b.is_identity() {
        a
    } else if !a.has_skew() && !b.has_skew() {
        // just scale and translate
        Transform::from_row(
            a.sx * b.sx,
            0.0,
            0.0,
            a.sy * b.sy,
            a.sx * b.tx + a.tx,
            a.sy * b.ty + a.ty,
        )
    } else {
        Transform::from_row(
            mul_add_mul(a.sx, b.sx, a.kx, b.ky),
            mul_add_mul(a.ky, b.sx, a.sy, b.ky),
            mul_add_mul(a.sx, b.kx, a.kx, b.sy),
            mul_add_mul(a.ky, b.kx, a.sy, b.sy),
            mul_add_mul(a.sx, b.tx, a.kx, b.ty) + a.tx,
            mul_add_mul(a.ky, b.tx, a.sy, b.ty) + a.ty,
        )
    }
}

fn mul_add_mul(a: f32, b: f32, c: f32, d: f32) -> f32 {
    (f64::from(a) * f64::from(b) + f64::from(c) * f64::from(d)) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform() {
        assert_eq!(
            Transform::identity(),
            Transform::from_row(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
        );

        assert_eq!(
            Transform::from_scale(1.0, 2.0),
            Transform::from_row(1.0, 0.0, 0.0, 2.0, 0.0, 0.0)
        );

        assert_eq!(
            Transform::from_skew(2.0, 3.0),
            Transform::from_row(1.0, 3.0, 2.0, 1.0, 0.0, 0.0)
        );

        assert_eq!(
            Transform::from_translate(5.0, 6.0),
            Transform::from_row(1.0, 0.0, 0.0, 1.0, 5.0, 6.0)
        );

        let ts = Transform::identity();
        assert_eq!(ts.is_identity(), true);
        assert_eq!(ts.is_scale(), false);
        assert_eq!(ts.is_skew(), false);
        assert_eq!(ts.is_translate(), false);
        assert_eq!(ts.is_scale_translate(), false);
        assert_eq!(ts.has_scale(), false);
        assert_eq!(ts.has_skew(), false);
        assert_eq!(ts.has_translate(), false);

        let ts = Transform::from_scale(2.0, 3.0);
        assert_eq!(ts.is_identity(), false);
        assert_eq!(ts.is_scale(), true);
        assert_eq!(ts.is_skew(), false);
        assert_eq!(ts.is_translate(), false);
        assert_eq!(ts.is_scale_translate(), true);
        assert_eq!(ts.has_scale(), true);
        assert_eq!(ts.has_skew(), false);
        assert_eq!(ts.has_translate(), false);

        let ts = Transform::from_skew(2.0, 3.0);
        assert_eq!(ts.is_identity(), false);
        assert_eq!(ts.is_scale(), false);
        assert_eq!(ts.is_skew(), true);
        assert_eq!(ts.is_translate(), false);
        assert_eq!(ts.is_scale_translate(), false);
        assert_eq!(ts.has_scale(), false);
        assert_eq!(ts.has_skew(), true);
        assert_eq!(ts.has_translate(), false);

        let ts = Transform::from_translate(2.0, 3.0);
        assert_eq!(ts.is_identity(), false);
        assert_eq!(ts.is_scale(), false);
        assert_eq!(ts.is_skew(), false);
        assert_eq!(ts.is_translate(), true);
        assert_eq!(ts.is_scale_translate(), true);
        assert_eq!(ts.has_scale(), false);
        assert_eq!(ts.has_skew(), false);
        assert_eq!(ts.has_translate(), true);

        let ts = Transform::from_row(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq!(ts.is_identity(), false);
        assert_eq!(ts.is_scale(), false);
        assert_eq!(ts.is_skew(), false);
        assert_eq!(ts.is_translate(), false);
        assert_eq!(ts.is_scale_translate(), false);
        assert_eq!(ts.has_scale(), true);
        assert_eq!(ts.has_skew(), true);
        assert_eq!(ts.has_translate(), true);

        let ts = Transform::from_scale(1.0, 1.0);
        assert_eq!(ts.has_scale(), false);

        let ts = Transform::from_skew(0.0, 0.0);
        assert_eq!(ts.has_skew(), false);

        let ts = Transform::from_translate(0.0, 0.0);
        assert_eq!(ts.has_translate(), false);
    }

    #[test]
    fn concat() {
        let mut ts = Transform::from_row(1.2, 3.4, -5.6, -7.8, 1.2, 3.4);
        ts = ts.pre_scale(2.0, -4.0);
        assert_eq!(ts, Transform::from_row(2.4, 6.8, 22.4, 31.2, 1.2, 3.4));

        let mut ts = Transform::from_row(1.2, 3.4, -5.6, -7.8, 1.2, 3.4);
        ts = ts.post_scale(2.0, -4.0);
        assert_eq!(ts, Transform::from_row(2.4, -13.6, -11.2, 31.2, 2.4, -13.6));
    }
}
