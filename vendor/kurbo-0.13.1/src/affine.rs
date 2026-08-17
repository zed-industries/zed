// Copyright 2018 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Affine transforms.

use core::ops::{Mul, MulAssign};

use crate::{Point, Rect, Vec2};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// A 2D affine transform.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Affine([f64; 6]);

impl Affine {
    /// The identity transform.
    pub const IDENTITY: Affine = Affine::scale(1.0);

    /// A transform that is flipped on the y-axis. Useful for converting between
    /// y-up and y-down spaces.
    pub const FLIP_Y: Affine = Affine::new([1.0, 0., 0., -1.0, 0., 0.]);

    /// A transform that is flipped on the x-axis.
    pub const FLIP_X: Affine = Affine::new([-1.0, 0., 0., 1.0, 0., 0.]);

    /// Construct an affine transform from coefficients.
    ///
    /// If the coefficients are `(a, b, c, d, e, f)`, then the resulting
    /// transformation represents this augmented matrix:
    ///
    /// ```text
    /// | a c e |
    /// | b d f |
    /// | 0 0 1 |
    /// ```
    ///
    /// Note that this convention is transposed from PostScript and
    /// Direct2D, but is consistent with the
    /// [Wikipedia](https://en.wikipedia.org/wiki/Affine_transformation)
    /// formulation of affine transformation as augmented matrix. The
    /// idea is that `(A * B) * v == A * (B * v)`, where `*` is the
    /// [`Mul`] trait.
    #[inline(always)]
    pub const fn new(c: [f64; 6]) -> Affine {
        Affine(c)
    }

    /// An affine transform representing uniform scaling.
    #[inline(always)]
    pub const fn scale(s: f64) -> Affine {
        Affine([s, 0.0, 0.0, s, 0.0, 0.0])
    }

    /// An affine transform representing non-uniform scaling
    /// with different scale values for x and y
    #[inline(always)]
    pub const fn scale_non_uniform(s_x: f64, s_y: f64) -> Affine {
        Affine([s_x, 0.0, 0.0, s_y, 0.0, 0.0])
    }

    /// An affine transform representing a scale of `scale` about `center`.
    ///
    /// Useful for a view transform that zooms at a specific point,
    /// while keeping that point fixed in the result space.
    ///
    /// See [`Affine::scale()`] for more info.
    #[inline]
    pub fn scale_about(s: f64, center: impl Into<Point>) -> Affine {
        let center = center.into().to_vec2();
        Self::translate(-center)
            .then_scale(s)
            .then_translate(center)
    }

    /// An affine transform representing rotation.
    ///
    /// The convention for rotation is that a positive angle rotates a
    /// positive X direction into positive Y. Thus, in a Y-down coordinate
    /// system (as is common for graphics), it is a clockwise rotation, and
    /// in Y-up (traditional for math), it is anti-clockwise.
    ///
    /// The angle, `th`, is expressed in radians.
    #[inline]
    pub fn rotate(th: f64) -> Affine {
        let (s, c) = th.sin_cos();
        Affine([c, s, -s, c, 0.0, 0.0])
    }

    /// An affine transform representing a rotation of `th` radians about `center`.
    ///
    /// See [`Affine::rotate()`] for more info.
    #[inline]
    pub fn rotate_about(th: f64, center: impl Into<Point>) -> Affine {
        let center = center.into().to_vec2();
        Self::translate(-center)
            .then_rotate(th)
            .then_translate(center)
    }

    /// An affine transform representing translation.
    #[inline(always)]
    pub fn translate<V: Into<Vec2>>(p: V) -> Affine {
        let p = p.into();
        Affine([1.0, 0.0, 0.0, 1.0, p.x, p.y])
    }

    /// An affine transformation representing a skew.
    ///
    /// The `skew_x` and `skew_y` parameters represent skew factors for the
    /// horizontal and vertical directions, respectively.
    ///
    /// This is commonly used to generate a faux oblique transform for
    /// font rendering. In this case, you can slant the glyph 20 degrees
    /// clockwise in the horizontal direction (assuming a Y-up coordinate
    /// system):
    ///
    /// ```
    /// let oblique_transform = kurbo::Affine::skew(20f64.to_radians().tan(), 0.0);
    /// ```
    #[inline(always)]
    pub const fn skew(skew_x: f64, skew_y: f64) -> Affine {
        Affine([1.0, skew_y, skew_x, 1.0, 0.0, 0.0])
    }

    /// Create an affine transform that represents reflection about the line `point + direction * t, t in (-infty, infty)`
    ///
    /// # Examples
    ///
    /// ```
    /// # use kurbo::{Point, Vec2, Affine};
    /// # fn assert_near(p0: Point, p1: Point) {
    /// #     assert!((p1 - p0).hypot() < 1e-9, "{p0:?} != {p1:?}");
    /// # }
    /// let point = Point::new(1., 0.);
    /// let vec = Vec2::new(1., 1.);
    /// let map = Affine::reflect(point, vec);
    /// assert_near(map * Point::new(1., 0.), Point::new(1., 0.));
    /// assert_near(map * Point::new(2., 1.), Point::new(2., 1.));
    /// assert_near(map * Point::new(2., 2.), Point::new(3., 1.));
    /// ```
    #[inline]
    #[must_use]
    pub fn reflect(point: impl Into<Point>, direction: impl Into<Vec2>) -> Self {
        let point = point.into();
        let direction = direction.into();

        let n = Vec2 {
            x: direction.y,
            y: -direction.x,
        }
        .normalize();

        // Compute Householder reflection matrix
        let x2 = n.x * n.x;
        let xy = n.x * n.y;
        let y2 = n.y * n.y;
        // Here we also add in the post translation, because it doesn't require any further calc.
        let aff = Affine::new([
            1. - 2. * x2,
            -2. * xy,
            -2. * xy,
            1. - 2. * y2,
            point.x,
            point.y,
        ]);
        aff.pre_translate(-point.to_vec2())
    }

    /// A [rotation] by `th` followed by `self`.
    ///
    /// Equivalent to `self * Affine::rotate(th)`
    ///
    /// [rotation]: Affine::rotate
    #[inline]
    #[must_use]
    pub fn pre_rotate(self, th: f64) -> Self {
        self * Affine::rotate(th)
    }

    /// A [rotation] by `th` about `center` followed by `self`.
    ///
    /// Equivalent to `self * Affine::rotate_about(th, center)`
    ///
    /// [rotation]: Affine::rotate_about
    #[inline]
    #[must_use]
    pub fn pre_rotate_about(self, th: f64, center: impl Into<Point>) -> Self {
        self * Affine::rotate_about(th, center)
    }

    /// A [scale] by `scale` followed by `self`.
    ///
    /// Equivalent to `self * Affine::scale(scale)`
    ///
    /// [scale]: Affine::scale
    #[inline]
    #[must_use]
    pub fn pre_scale(self, scale: f64) -> Self {
        self * Affine::scale(scale)
    }

    /// A [scale] by `(scale_x, scale_y)` followed by `self`.
    ///
    /// Equivalent to `self * Affine::scale_non_uniform(scale_x, scale_y)`
    ///
    /// [scale]: Affine::scale_non_uniform
    #[inline]
    #[must_use]
    pub fn pre_scale_non_uniform(self, scale_x: f64, scale_y: f64) -> Self {
        self * Affine::scale_non_uniform(scale_x, scale_y)
    }

    /// A [translation] of `trans` followed by `self`.
    ///
    /// Equivalent to `self * Affine::translate(trans)`
    ///
    /// [translation]: Affine::translate
    #[inline]
    #[must_use]
    pub fn pre_translate(self, trans: Vec2) -> Self {
        self * Affine::translate(trans)
    }

    /// A [skew] of `(skew_x, skew_y)` followed by `self`.
    ///
    /// Equivalent to `self * Affine::skew(skew_x, skew_y)`
    ///
    /// [skew]: Affine::skew
    #[inline]
    #[must_use]
    pub fn pre_skew(self, skew_x: f64, skew_y: f64) -> Self {
        self * Affine::skew(skew_x, skew_y)
    }

    /// A [reflection] about the line through `point` in `direction` followed by `self`.
    ///
    /// Equivalent to `self * Affine::reflect(point, direction)`
    ///
    /// [reflection]: Affine::reflect
    #[inline]
    #[must_use]
    pub fn pre_reflect(self, point: impl Into<Point>, direction: impl Into<Vec2>) -> Self {
        self * Affine::reflect(point, direction)
    }

    /// `self` followed by a [rotation] of `th`.
    ///
    /// Equivalent to `Affine::rotate(th) * self`
    ///
    /// [rotation]: Affine::rotate
    #[inline]
    #[must_use]
    pub fn then_rotate(self, th: f64) -> Self {
        Affine::rotate(th) * self
    }

    /// `self` followed by a [rotation] of `th` about `center`.
    ///
    /// Equivalent to `Affine::rotate_about(th, center) * self`
    ///
    /// [rotation]: Affine::rotate_about
    #[inline]
    #[must_use]
    pub fn then_rotate_about(self, th: f64, center: impl Into<Point>) -> Self {
        Affine::rotate_about(th, center) * self
    }

    /// `self` followed by a [scale] of `scale`.
    ///
    /// Equivalent to `Affine::scale(scale) * self`
    ///
    /// [scale]: Affine::scale
    #[inline]
    #[must_use]
    pub fn then_scale(self, scale: f64) -> Self {
        Affine::scale(scale) * self
    }

    /// `self` followed by a [scale] of `(scale_x, scale_y)`.
    ///
    /// Equivalent to `Affine::scale_non_uniform(scale_x, scale_y) * self`
    ///
    /// [scale]: Affine::scale_non_uniform
    #[inline]
    #[must_use]
    pub fn then_scale_non_uniform(self, scale_x: f64, scale_y: f64) -> Self {
        Affine::scale_non_uniform(scale_x, scale_y) * self
    }

    /// `self` followed by a [scale] of `scale` about `center`.
    ///
    /// Equivalent to `Affine::scale_about(scale) * self`
    ///
    /// [scale]: Affine::scale_about
    #[inline]
    #[must_use]
    pub fn then_scale_about(self, scale: f64, center: impl Into<Point>) -> Self {
        Affine::scale_about(scale, center) * self
    }

    /// `self` followed by a [skew] of `(skew_x, skew_y)`.
    ///
    /// Equivalent to `Affine::skew(skew_x, skew_y) * self`
    ///
    /// [skew]: Affine::skew
    #[inline]
    #[must_use]
    pub fn then_skew(self, skew_x: f64, skew_y: f64) -> Self {
        Affine::skew(skew_x, skew_y) * self
    }

    /// `self` followed by a [reflection] about the line through `point` in `direction`.
    ///
    /// Equivalent to `Affine::reflect(point, direction) * self`
    ///
    /// [reflection]: Affine::reflect
    #[inline]
    #[must_use]
    pub fn then_reflect(self, point: impl Into<Point>, direction: impl Into<Vec2>) -> Self {
        Affine::reflect(point, direction) * self
    }

    /// `self` followed by a translation of `trans`.
    ///
    /// Equivalent to `Affine::translate(trans) * self`
    ///
    /// [translation]: Affine::translate
    #[inline]
    #[must_use]
    pub const fn then_translate(mut self, trans: Vec2) -> Self {
        self.0[4] += trans.x;
        self.0[5] += trans.y;
        self
    }

    /// Creates an affine transformation that takes the unit square to the given rectangle.
    ///
    /// Useful when you want to draw into the unit square but have your output fill any rectangle.
    /// In this case push the `Affine` onto the transform stack.
    pub const fn map_unit_square(rect: Rect) -> Affine {
        Affine([rect.width(), 0., 0., rect.height(), rect.x0, rect.y0])
    }

    /// Get the coefficients of the transform.
    #[inline(always)]
    pub const fn as_coeffs(self) -> [f64; 6] {
        self.0
    }

    /// Compute the determinant of this transform.
    ///
    /// # Geometric interpretation
    ///
    /// Consider a region transformed by this affine. The transformed region's area is the area of
    /// the original region scaled by the absolute value of the determinant. A negative determinant
    /// indicates orientation reversal.
    #[inline]
    pub const fn determinant(self) -> f64 {
        self.0[0] * self.0[3] - self.0[1] * self.0[2]
    }

    /// Compute the square of the nuclear norm of this transform.
    ///
    /// This is the square of the [Schatten p-norm][schatten] with `p=1`, also known as the "trace norm."
    ///
    /// Returns the squared norm for efficiency; take the square root as necessary.
    ///
    /// # Geometric interpretation
    ///
    /// Consider a unit circle transformed by this affine. The nuclear norm is the sum of the
    /// resulting ellipse's radii (semi axes). That sum multiplied by π is a first-order
    /// approximation of the ellipse's perimeter.
    ///
    /// [schatten]: <https://en.wikipedia.org/w/index.php?title=Matrix_norm&oldid=1348997593#Schatten_norms>
    #[inline]
    pub const fn nuclear_norm_squared(self) -> f64 {
        self.frobenius_norm_squared() + 2. * self.determinant().abs()
    }

    /// Compute the square of the Frobenius norm of this transform.
    ///
    /// This is the square of the [Schatten p-norm][schatten] with `p=2`.
    ///
    /// Returns the squared norm for efficiency; take the square root as necessary.
    ///
    /// # Geometric interpretation
    ///
    /// Consider a unit circle transformed by this affine. The squared Frobenius norm is twice the
    /// mean squared radius of the resulting ellipse. Alternatively, it is equal to the squared
    /// distance from the ellipse's center to a corner of the rectangle spanned by the ellipse's
    /// axes.
    ///
    /// [schatten]: <https://en.wikipedia.org/w/index.php?title=Matrix_norm&oldid=1348997593#Schatten_norms>
    #[inline]
    pub const fn frobenius_norm_squared(self) -> f64 {
        let [a, b, c, d, _, _] = self.as_coeffs();
        a * a + b * b + c * c + d * d
    }

    /// Compute the spectral norm of this transform.
    ///
    /// This is the [Schatten p-norm][schatten] with `p=∞`.
    ///
    /// # Geometric interpretation
    ///
    /// Consider a unit circle transformed by this affine. The spectral norm is the major radius
    /// (semi-major axis) of the Ellipse.
    ///
    /// [schatten]: <https://en.wikipedia.org/w/index.php?title=Matrix_norm&oldid=1348997593#Schatten_norms>
    #[inline]
    pub fn spectral_norm(self) -> f64 {
        // Note a different calculation, returning the `_squared` form like our nuclear and
        // Frobenius norms, could be `0.5 (frob^2 + sqrt(frob^4 - 4 det^2))`. In terms of operations
        // it's a wash: one fewer sqrt if the user actually wants the squared form, but it uses more
        // muls. More importantly, that form has worse numeric conditioning.
        self.svd().0.x
    }

    /// Compute the inverse transform.
    ///
    /// Produces NaN values when the determinant is zero.
    pub const fn inverse(self) -> Affine {
        let inv_det = self.determinant().recip();
        Affine([
            inv_det * self.0[3],
            -inv_det * self.0[1],
            -inv_det * self.0[2],
            inv_det * self.0[0],
            inv_det * (self.0[2] * self.0[5] - self.0[3] * self.0[4]),
            inv_det * (self.0[1] * self.0[4] - self.0[0] * self.0[5]),
        ])
    }

    /// Compute the bounding box of a transformed rectangle.
    ///
    /// Returns the minimal `Rect` that encloses the given `Rect` after affine transformation.
    /// If the transform is axis-aligned, then this bounding box is "tight", in other words the
    /// returned `Rect` is the transformed rectangle.
    ///
    /// The returned rectangle always has non-negative width and height.
    pub fn transform_rect_bbox(self, rect: Rect) -> Rect {
        let p00 = self * Point::new(rect.x0, rect.y0);
        let p01 = self * Point::new(rect.x0, rect.y1);
        let p10 = self * Point::new(rect.x1, rect.y0);
        let p11 = self * Point::new(rect.x1, rect.y1);
        Rect::from_points(p00, p01).union(Rect::from_points(p10, p11))
    }

    /// Is this map [finite]?
    ///
    /// [finite]: f64::is_finite
    #[inline]
    pub const fn is_finite(&self) -> bool {
        self.0[0].is_finite()
            && self.0[1].is_finite()
            && self.0[2].is_finite()
            && self.0[3].is_finite()
            && self.0[4].is_finite()
            && self.0[5].is_finite()
    }

    /// Is this map [NaN]?
    ///
    /// [NaN]: f64::is_nan
    #[inline]
    pub const fn is_nan(&self) -> bool {
        self.0[0].is_nan()
            || self.0[1].is_nan()
            || self.0[2].is_nan()
            || self.0[3].is_nan()
            || self.0[4].is_nan()
            || self.0[5].is_nan()
    }

    /// Compute the singular value decomposition of the linear transformation (ignoring the
    /// translation).
    ///
    /// All non-degenerate linear transformations can be represented as
    ///
    ///  1. a rotation about the origin.
    ///  2. a scaling along the x and y axes
    ///  3. another rotation about the origin
    ///
    /// composed together. Decomposing a 2x2 matrix in this way is called a "singular value
    /// decomposition" and is written `U Σ V^T`, where U and V^T are orthogonal (rotations) and Σ
    /// is a diagonal matrix (a scaling).
    ///
    /// Since currently this function is used to calculate ellipse radii and rotation from an
    /// affine map on the unit circle, we don't calculate V^T, since a rotation of the unit (or
    /// any) circle about its center always results in the same circle. This is the reason that an
    /// ellipse mapped using an affine map is always an ellipse.
    ///
    /// Will return NaNs if the matrix (or equivalently the linear map) is non-finite.
    ///
    /// The first part of the returned tuple is the scaling, the second part is the angle of
    /// rotation (in radians). The scaling along the x-axis is guaranteed to be greater than or
    /// equal to the scaling along the y-axis.
    //
    // Note: though this does quite some computation, we are often interested only in specific
    // components of the result. Hence this is marked `#[inline(always)]`, to give the compiler a
    // good chance at eliminating dead code.
    #[inline(always)]
    pub(crate) fn svd(self) -> (Vec2, f64) {
        let [a, b, c, d, _, _] = self.0;
        let a2 = a * a;
        let b2 = b * b;
        let c2 = c * c;
        let d2 = d * d;
        let ab = a * b;
        let cd = c * d;
        let angle = 0.5 * (2.0 * (ab + cd)).atan2(a2 - b2 + c2 - d2);

        // Given matrix A = [ a c ]
        //                  [ b d ]
        //
        // The two singular values σ1, σ2 of A are the square roots of the two eigen values λ1, λ2
        // of M = A^T A. The common formula for 2x2 eigenvalues requires evaluating a square root,
        // but we'd like to compute the singular values of the matrix without nested square roots.
        //
        // M = A^T A = [ aa+cc   ab+cd ]
        //             [ ab+cd   bb+dd ]
        //
        // We have
        // λ = 1/2 (tr(M) ± sqrt(tr(M)^2 - 4 det(M))).
        //
        // Note det(M) = det(A^T A) = det(A)^2.
        // => 2λ = tr(M) ± sqrt(tr(M)^2 - 4 det(A)^2)
        // => 2λ = tr(M) ± sqrt[(a^2+b^2+c^2+d^2)^2 - 4 (ad-bc)^2]
        // By factorizing the inner term,
        // => 2λ = tr(M) ± sqrt[((a+d)^2 + (b-c)^2) ((a-d)^2 + (b+c)^2)]
        // => 2λ = tr(M) ± sqrt[(a+d)^2 + (b-c)^2] sqrt[(a-d)^2 + (b+c)^2]
        //
        // Define S1 = sqrt[(a+d)^2 + (b-c)^2]
        //        S2 = sqrt[(a-d)^2 + (b+c)^2].
        //
        // => 2λ = tr(M) ± S1 S2
        // => 2λ = 1/2 (S1^2 + S2^2) ± S1 S2
        // => λ = 1/4 (S1^2 + S2^2 ± 2 S1 S2)
        // => λ = 1/4 (S1 ± S2)^2
        //
        // Note we're interested in
        // σ = sqrt(λ).
        //
        // => σ1 = 1/2 (S1 + S2)
        // and similarly σ2 = 1/2 |S1 - S2|
        let s1 = ((a + d).powi(2) + (b - c).powi(2)).sqrt();
        let s2 = ((a - d).powi(2) + (b + c).powi(2)).sqrt();
        (
            Vec2 {
                x: 0.5 * (s1 + s2),
                y: 0.5 * (s1 - s2).abs(),
            },
            angle,
        )
    }

    /// Returns the translation part of this affine map (`(self.0[4], self.0[5])`).
    #[inline(always)]
    pub const fn translation(self) -> Vec2 {
        Vec2 {
            x: self.0[4],
            y: self.0[5],
        }
    }

    /// Replaces the translation portion of this affine map
    ///
    /// The translation can be seen as being applied after the linear part of the map.
    #[must_use]
    #[inline(always)]
    pub const fn with_translation(mut self, trans: Vec2) -> Affine {
        self.0[4] = trans.x;
        self.0[5] = trans.y;
        self
    }
}

impl Default for Affine {
    #[inline(always)]
    fn default() -> Affine {
        Affine::IDENTITY
    }
}

impl Mul<Point> for Affine {
    type Output = Point;

    #[inline]
    fn mul(self, other: Point) -> Point {
        Point::new(
            self.0[0] * other.x + self.0[2] * other.y + self.0[4],
            self.0[1] * other.x + self.0[3] * other.y + self.0[5],
        )
    }
}

impl Mul for Affine {
    type Output = Affine;

    #[inline]
    fn mul(self, other: Affine) -> Affine {
        Affine([
            self.0[0] * other.0[0] + self.0[2] * other.0[1],
            self.0[1] * other.0[0] + self.0[3] * other.0[1],
            self.0[0] * other.0[2] + self.0[2] * other.0[3],
            self.0[1] * other.0[2] + self.0[3] * other.0[3],
            self.0[0] * other.0[4] + self.0[2] * other.0[5] + self.0[4],
            self.0[1] * other.0[4] + self.0[3] * other.0[5] + self.0[5],
        ])
    }
}

impl MulAssign for Affine {
    #[inline]
    fn mul_assign(&mut self, other: Affine) {
        *self = self.mul(other);
    }
}

impl Mul<Affine> for f64 {
    type Output = Affine;

    #[inline]
    fn mul(self, other: Affine) -> Affine {
        Affine([
            self * other.0[0],
            self * other.0[1],
            self * other.0[2],
            self * other.0[3],
            self * other.0[4],
            self * other.0[5],
        ])
    }
}

// Conversions to and from mint
#[cfg(feature = "mint")]
impl From<Affine> for mint::ColumnMatrix2x3<f64> {
    #[inline(always)]
    fn from(a: Affine) -> mint::ColumnMatrix2x3<f64> {
        mint::ColumnMatrix2x3 {
            x: mint::Vector2 {
                x: a.0[0],
                y: a.0[1],
            },
            y: mint::Vector2 {
                x: a.0[2],
                y: a.0[3],
            },
            z: mint::Vector2 {
                x: a.0[4],
                y: a.0[5],
            },
        }
    }
}

#[cfg(feature = "mint")]
impl From<mint::ColumnMatrix2x3<f64>> for Affine {
    #[inline(always)]
    fn from(m: mint::ColumnMatrix2x3<f64>) -> Affine {
        Affine([m.x.x, m.x.y, m.y.x, m.y.y, m.z.x, m.z.y])
    }
}

#[cfg(test)]
mod tests {
    use crate::{Affine, Point, Vec2};
    use std::f64::consts::PI;

    fn assert_near(p0: Point, p1: Point) {
        assert!((p1 - p0).hypot() < 1e-9, "{p0:?} != {p1:?}");
    }

    fn affine_assert_near(a0: Affine, a1: Affine) {
        for i in 0..6 {
            assert!((a0.0[i] - a1.0[i]).abs() < 1e-9, "{a0:?} != {a1:?}");
        }
    }

    #[test]
    fn affine_basic() {
        let p = Point::new(3.0, 4.0);

        assert_near(Affine::default() * p, p);
        assert_near(Affine::scale(2.0) * p, Point::new(6.0, 8.0));
        assert_near(Affine::rotate(0.0) * p, p);
        assert_near(Affine::rotate(PI / 2.0) * p, Point::new(-4.0, 3.0));
        assert_near(Affine::translate((5.0, 6.0)) * p, Point::new(8.0, 10.0));
        assert_near(Affine::skew(0.0, 0.0) * p, p);
        assert_near(Affine::skew(2.0, 4.0) * p, Point::new(11.0, 16.0));
    }

    #[test]
    fn affine_mul() {
        let a1 = Affine::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let a2 = Affine::new([0.1, 1.2, 2.3, 3.4, 4.5, 5.6]);

        let px = Point::new(1.0, 0.0);
        let py = Point::new(0.0, 1.0);
        let pxy = Point::new(1.0, 1.0);
        assert_near(a1 * (a2 * px), (a1 * a2) * px);
        assert_near(a1 * (a2 * py), (a1 * a2) * py);
        assert_near(a1 * (a2 * pxy), (a1 * a2) * pxy);
    }

    #[test]
    fn affine_inv() {
        let a = Affine::new([0.1, 1.2, 2.3, 3.4, 4.5, 5.6]);
        let a_inv = a.inverse();

        let px = Point::new(1.0, 0.0);
        let py = Point::new(0.0, 1.0);
        let pxy = Point::new(1.0, 1.0);
        assert_near(a * (a_inv * px), px);
        assert_near(a * (a_inv * py), py);
        assert_near(a * (a_inv * pxy), pxy);
        assert_near(a_inv * (a * px), px);
        assert_near(a_inv * (a * py), py);
        assert_near(a_inv * (a * pxy), pxy);
    }

    #[test]
    fn reflection() {
        affine_assert_near(
            Affine::reflect(Point::ZERO, (1., 0.)),
            Affine::new([1., 0., 0., -1., 0., 0.]),
        );
        affine_assert_near(
            Affine::reflect(Point::ZERO, (0., 1.)),
            Affine::new([-1., 0., 0., 1., 0., 0.]),
        );
        // y = x
        affine_assert_near(
            Affine::reflect(Point::ZERO, (1., 1.)),
            Affine::new([0., 1., 1., 0., 0., 0.]),
        );

        // no translate
        let point = Point::new(0., 0.);
        let vec = Vec2::new(1., 1.);
        let map = Affine::reflect(point, vec);
        assert_near(map * Point::new(0., 0.), Point::new(0., 0.));
        assert_near(map * Point::new(1., 1.), Point::new(1., 1.));
        assert_near(map * Point::new(1., 2.), Point::new(2., 1.));

        // with translate
        let point = Point::new(1., 0.);
        let vec = Vec2::new(1., 1.);
        let map = Affine::reflect(point, vec);
        assert_near(map * Point::new(1., 0.), Point::new(1., 0.));
        assert_near(map * Point::new(2., 1.), Point::new(2., 1.));
        assert_near(map * Point::new(2., 2.), Point::new(3., 1.));
    }

    #[test]
    fn svd() {
        let a = Affine::new([1., 2., 3., 4., 5., 6.]);
        let a_no_translate = a.with_translation(Vec2::ZERO);

        // translation should have no effect
        let (scale, rotation) = a.svd();
        let (scale_no_translate, rotation_no_translate) = a_no_translate.svd();
        assert_near(scale.to_point(), scale_no_translate.to_point());
        assert!((rotation - rotation_no_translate).abs() <= 1e-9);

        assert_near(
            scale.to_point(),
            Point::new(5.4649857042190427, 0.36596619062625782),
        );
        assert!((rotation - 0.95691013360780001).abs() <= 1e-9);

        // singular affine
        let a = Affine::new([0., 0., 0., 0., 5., 6.]);
        assert_eq!(a.determinant(), 0.);
        let (scale, rotation) = a.svd();
        assert_eq!(scale, Vec2::new(0., 0.));
        assert_eq!(rotation, 0.);
    }

    #[test]
    fn svd_singular_values() {
        // Test a few known singular values.
        let mat = |a, b, c, d| Affine::new([a, b, c, d, 0., 0.]);

        let s = mat(1., 0., 0., 1.).svd().0;
        assert_near(s.to_point(), Point::new(1., 1.));

        let s = mat(1., 0., 0., -1.).svd().0;
        assert_near(s.to_point(), Point::new(1., 1.));

        let s = mat(1., 1., 1., 1.).svd().0;
        assert_near(s.to_point(), Point::new(2., 0.));

        let s = mat(1., 1., 1., 1.).svd().0;
        assert_near(s.to_point(), Point::new(2., 0.));

        let s = mat(0., 0., 1., 0.).svd().0;
        assert_near(s.to_point(), Point::new(1., 0.));

        // The singular values are the scaling of the affine map. So let's test that.
        let s = Affine::scale_non_uniform(4., 8.)
            .then_rotate_about(42_f64.to_radians(), (-2., 50.))
            .svd()
            .0;
        assert_near(s.to_point(), Point::new(8., 4.));

        // Correctly handles negative scaling (singular values are necessarily non-negative).
        let s = Affine::scale_non_uniform(-20., 3.).svd().0;
        assert_near(s.to_point(), Point::new(20., 3.));
        let s = Affine::scale_non_uniform(-20., -3.).svd().0;
        assert_near(s.to_point(), Point::new(20., 3.));
        let s = Affine::scale_non_uniform(20., -3.).svd().0;
        assert_near(s.to_point(), Point::new(20., 3.));

        // One more property: given a full-rank transform, the product of its singular values
        // should be equal to its absolute determinant.
        let m = mat(10., 9., -2.5, 3.3333);
        let s = m.svd().0;
        let prod = s.x * s.y;
        let det = m.determinant().abs();
        assert!(
            (prod - det) < 1e-9,
            "The product of the singular values {s:?} ({prod}) should be equal to the absolute determinant {det}.",
        );
    }

    #[test]
    fn rotate_about_composition() {
        let theta = core::f64::consts::FRAC_PI_2;
        let center = Point::new(-1., 0.);
        let translation = Vec2::new(0., 1.);
        let probe = Point::ORIGIN;

        let rotate_about = Affine::rotate_about(theta, center);
        let translate = Affine::translate(translation);

        // Establish baselines with raw matrix composition
        // (also a sanity check to ensure the order of ops matters for this contrived test)
        let rotate_then_translate = translate * rotate_about;
        let translate_then_rotate = rotate_about * translate;
        assert_near(rotate_then_translate * probe, Point::new(-1., 2.));
        assert_near(translate_then_rotate * probe, Point::new(-2., 1.));

        // Check .then_* semantics
        affine_assert_near(
            rotate_about.then_translate(translation),
            rotate_then_translate,
        );
        affine_assert_near(
            translate.then_rotate_about(theta, center),
            translate_then_rotate,
        );

        // Check .pre_rotate_about semantics
        affine_assert_near(
            translate.pre_rotate_about(theta, center),
            rotate_then_translate,
        );
    }
}
