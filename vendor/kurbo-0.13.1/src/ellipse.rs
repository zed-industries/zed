// Copyright 2020 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementation of ellipse shape.

use core::f64::consts::PI;
use core::{
    iter,
    ops::{Add, Mul, Sub},
};

use crate::{Affine, Arc, ArcAppendIter, Circle, PathEl, Point, Rect, Shape, Size, Vec2};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// An ellipse.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ellipse {
    /// All ellipses can be represented as an affine map of the unit circle,
    /// centred at (0, 0). Therefore we can store the ellipse as an affine map,
    /// with the implication that it be applied to the unit circle to recover the
    /// actual shape.
    inner: Affine,
}

impl Ellipse {
    /// Create A new ellipse with a given center, radii, and rotation.
    ///
    /// The returned ellipse will be the result of taking a circle, stretching
    /// it by the `radii` along the x and y axes, then rotating it from the
    /// x axis by `rotation` radians, before finally translating the center
    /// to `center`.
    ///
    /// Rotation is clockwise in a y-down coordinate system. For more on
    /// rotation, see [`Affine::rotate`].
    #[inline]
    pub fn new(center: impl Into<Point>, radii: impl Into<Vec2>, x_rotation: f64) -> Ellipse {
        let Point { x: cx, y: cy } = center.into();
        let Vec2 { x: rx, y: ry } = radii.into();
        Ellipse::private_new(Vec2 { x: cx, y: cy }, rx, ry, x_rotation)
    }

    /// Returns the largest ellipse that can be bounded by this [`Rect`].
    ///
    /// This uses the absolute width and height of the rectangle.
    ///
    /// This ellipse is always axis-aligned; to apply rotation you can call
    /// [`with_rotation`] with the result.
    ///
    /// [`with_rotation`]: Ellipse::with_rotation
    #[inline]
    pub fn from_rect(rect: Rect) -> Self {
        let center = rect.center().to_vec2();
        let Size { width, height } = rect.size() / 2.0;
        Ellipse::private_new(center, width, height, 0.0)
    }

    /// Create an ellipse from an affine transformation of the unit circle.
    #[inline(always)]
    pub const fn from_affine(affine: Affine) -> Self {
        Ellipse { inner: affine }
    }

    /// Create a new `Ellipse` centered on the provided point.
    #[inline]
    #[must_use]
    pub fn with_center(self, new_center: impl Into<Point>) -> Ellipse {
        let Point { x: cx, y: cy } = new_center.into();
        Ellipse {
            inner: self.inner.with_translation(Vec2 { x: cx, y: cy }),
        }
    }

    /// Create a new `Ellipse` with the provided radii.
    #[inline]
    #[must_use]
    pub fn with_radii(self, new_radii: Vec2) -> Ellipse {
        let rotation = self.inner.svd().1;
        let translation = self.inner.translation();
        Ellipse::private_new(translation, new_radii.x, new_radii.y, rotation)
    }

    /// Create a new `Ellipse`, with the rotation replaced by `rotation`
    /// radians.
    ///
    /// The rotation is clockwise, for a y-down coordinate system. For more
    /// on rotation, See [`Affine::rotate`].
    #[inline]
    #[must_use]
    pub fn with_rotation(self, rotation: f64) -> Ellipse {
        let scale = self.inner.svd().0;
        let translation = self.inner.translation();
        Ellipse::private_new(translation, scale.x, scale.y, rotation)
    }

    /// This gives us an internal method without any type conversions.
    #[inline]
    fn private_new(center: Vec2, scale_x: f64, scale_y: f64, x_rotation: f64) -> Ellipse {
        // Since the circle is symmetric about the x and y axes, using absolute values for the
        // radii results in the same ellipse. For simplicity we make this change here.
        Ellipse {
            inner: Affine::translate(center)
                * Affine::rotate(x_rotation)
                * Affine::scale_non_uniform(scale_x.abs(), scale_y.abs()),
        }
    }

    // Getters and setters.

    /// Returns the center of this ellipse.
    #[inline(always)]
    pub fn center(&self) -> Point {
        self.inner.translation().to_point()
    }

    /// Returns the two radii of this ellipse.
    ///
    /// The first number is the horizontal radius and the second is the vertical
    /// radius, before rotation.
    ///
    /// If you are only interested in the value of the greatest or smallest radius of this ellipse,
    /// consider using [`Ellipse::major_radius`] or [`Ellipse::minor_radius`] instead.
    #[inline]
    pub fn radii(&self) -> Vec2 {
        self.inner.svd().0
    }

    /// Returns the major radius of this ellipse.
    ///
    /// This metric is also known as the semi-major axis.
    #[inline]
    pub fn major_radius(&self) -> f64 {
        self.inner.svd().0.x
    }

    /// Returns the minor radius of this ellipse.
    ///
    /// This metric is also known as the semi-minor axis.
    #[inline]
    pub fn minor_radius(&self) -> f64 {
        self.inner.svd().0.y
    }

    /// The ellipse's rotation, in radians.
    ///
    /// This allows all possible ellipses to be drawn by always starting with
    /// an ellipse with the two radii on the x and y axes.
    #[inline]
    pub fn rotation(&self) -> f64 {
        self.inner.svd().1
    }

    /// Returns the radii and the rotation of this ellipse.
    ///
    /// Equivalent to `(self.radii(), self.rotation())` but more efficient.
    #[inline]
    pub fn radii_and_rotation(&self) -> (Vec2, f64) {
        self.inner.svd()
    }

    /// Is this ellipse [finite]?
    ///
    /// [finite]: f64::is_finite
    #[inline]
    pub const fn is_finite(&self) -> bool {
        self.inner.is_finite()
    }

    /// Is this ellipse [NaN]?
    ///
    /// [NaN]: f64::is_nan
    #[inline]
    pub const fn is_nan(&self) -> bool {
        self.inner.is_nan()
    }
}

impl Add<Vec2> for Ellipse {
    type Output = Ellipse;

    /// In this context adding a `Vec2` applies the corresponding translation to the ellipse.
    #[inline]
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, v: Vec2) -> Ellipse {
        Ellipse {
            inner: Affine::translate(v) * self.inner,
        }
    }
}

impl Sub<Vec2> for Ellipse {
    type Output = Ellipse;

    /// In this context subtracting a `Vec2` applies the corresponding translation to the ellipse.
    #[inline]
    fn sub(self, v: Vec2) -> Ellipse {
        Ellipse {
            inner: Affine::translate(-v) * self.inner,
        }
    }
}

impl Mul<Ellipse> for Affine {
    type Output = Ellipse;
    #[inline]
    fn mul(self, other: Ellipse) -> Self::Output {
        Ellipse {
            inner: self * other.inner,
        }
    }
}

impl From<Circle> for Ellipse {
    #[inline]
    fn from(circle: Circle) -> Self {
        Ellipse::new(circle.center, Vec2::splat(circle.radius), 0.0)
    }
}

impl Shape for Ellipse {
    type PathElementsIter<'iter> = iter::Chain<iter::Once<PathEl>, ArcAppendIter>;

    fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> {
        let (radii, x_rotation) = self.inner.svd();
        Arc {
            center: self.center(),
            radii,
            start_angle: 0.0,
            sweep_angle: 2.0 * PI,
            x_rotation,
        }
        .path_elements(tolerance)
    }

    /// The area of the ellipse.
    ///
    /// This is always positive if the result is finite.
    ///
    /// If non-finite, this will be [`f64::INFINITY`] or [`f64::NAN`] depending on whether the
    /// inner affine's [determinant](Affine::determinant) is [`f64::INFINITY`] (either positive or
    /// negative) or [`f64::NAN`].
    #[inline]
    fn area(&self) -> f64 {
        // `Ellipse` is represented as a unit circle transformed by `Affine`. The transformed area
        // of a region is `area * |det(affine)|`, see
        // <https://en.wikipedia.org/w/index.php?title=Determinant&oldid=1344268205#Geometric_meaning>.
        //
        // A unit circle has area `PI`. Therefore, the area of this ellipse is PI multiplied by the
        // affine's determinant.
        PI * self.inner.determinant().abs()
    }

    /// Approximate the ellipse perimeter.
    ///
    /// This uses a numerical approximation. The absolute error between the calculated perimeter
    /// and the true perimeter is bounded by `accuracy` (modulo floating point rounding errors).
    ///
    /// For circular ellipses (equal horizontal and vertical radii), the calculated perimeter is
    /// exact.
    #[inline]
    fn perimeter(&self, accuracy: f64) -> f64 {
        let radii = self.radii();

        // Note: the radii are calculated from an inner affine map (`self.inner`), and may be NaN.
        // Currently, constructing an ellipse with infinite radii will produce an ellipse whose
        // calculated radii are NaN.
        if !radii.is_finite() {
            return f64::NAN;
        }

        // Check for the trivial case where the ellipse has one of its radii equal to 0, i.e.,
        // where it describes a line, as the numerical method used breaks down with this extreme.
        if radii.x == 0. || radii.y == 0. {
            return 4. * f64::max(radii.x, radii.y);
        }

        // Evaluate an approximation based on a truncated infinite series. If it returns a good
        // enough value, we do not need to iterate.
        if kummer_elliptic_perimeter_range(radii) <= accuracy {
            return kummer_elliptic_perimeter(radii);
        }

        agm_elliptic_perimeter(accuracy, radii)
    }

    #[inline]
    fn winding(&self, pt: Point) -> i32 {
        // Strategy here is to apply the inverse map to the point and see if it is in the unit
        // circle.
        let inv = self.inner.inverse();
        if (inv * pt).to_vec2().hypot2() < 1.0 {
            1
        } else {
            0
        }
    }

    // Compute a tight bounding box of the ellipse.
    //
    // See https://www.iquilezles.org/www/articles/ellipses/ellipses.htm. We can get the two
    // radius vectors by applying the affine map to the two impulses (1, 0) and (0, 1) which gives
    // (a, b) and (c, d) if the affine map is
    //
    //  a | c | e
    // -----------
    //  b | d | f
    //
    //  We can then use the method in the link with the translation to get the bounding box.
    #[inline]
    fn bounding_box(&self) -> Rect {
        let aff = self.inner.as_coeffs();
        let a2 = aff[0] * aff[0];
        let b2 = aff[1] * aff[1];
        let c2 = aff[2] * aff[2];
        let d2 = aff[3] * aff[3];
        let cx = aff[4];
        let cy = aff[5];
        let range_x = (a2 + c2).sqrt();
        let range_y = (b2 + d2).sqrt();
        Rect {
            x0: cx - range_x,
            y0: cy - range_y,
            x1: cx + range_x,
            y1: cy + range_y,
        }
    }
}

/// Calculates circumference C of an ellipse with radii (x, y) as the infinite series
///
/// C = pi (x+y) * ∑ binom(1/2, n)^2 * h^n from n = 0 to inf
/// with h = (x - y)^2 / (x + y)^2
/// and binom(.,.) the binomial coefficient
///
/// as described by Kummer ("Über die Hypergeometrische Reihe", 1837) and rediscovered by
/// Linderholm and Segal ("An Overlooked Series for the Elliptic Perimeter", 1995).
///
/// The series converges very quickly for ellipses with only moderate eccentricity (`h` not close
/// to 1).
///
/// The series is truncated to the sixth power, meaning a lower bound on the true value is
/// returned. Adding the value of [`kummer_elliptic_perimeter_range`] to the value returned by this
/// function calculates an upper bound on the true value.
#[inline]
fn kummer_elliptic_perimeter(radii: Vec2) -> f64 {
    let Vec2 { x, y } = radii;
    let h = ((x - y) / (x + y)).powi(2);
    let h2 = h * h;
    let h3 = h2 * h;
    let h4 = h3 * h;
    let h5 = h4 * h;
    let h6 = h5 * h;

    let lower = PI
        + h * (PI / 4.)
        + h2 * (PI / 64.)
        + h3 * (PI / 256.)
        + h4 * (PI * 25. / 16384.)
        + h5 * (PI * 49. / 65536.)
        + h6 * (PI * 441. / 1048576.);

    (x + y) * lower
}

/// This calculates the error range of [`kummer_elliptic_perimeter`]. That function returns a lower
/// bound on the true value, and though we do not know what the remainder of the infinite series
/// sums to, we can calculate an upper bound:
///
/// ∑ binom(1/2, n)^2 for n = 0 to inf
///   = 1 + (1 / 2!!)^2 + (1!! / 4!!)^2 + (3!! / 6!!)^2 + (5!! / 8!!)^2 + ..
///   = 4 / pi
///   with !! the double factorial
/// (equation 274 in "Summation of Series", L. B. W. Jolley, 1961).
///
/// This means the remainder of the infinite series for C, assuming the series was truncated to the
/// mth term and h = 1, sums to
/// 4 / pi - ∑ binom(1/2, n)^2 for n = 0 to m-1
///
/// As 0 ≤ h ≤ 1, this is an upper bound.
#[inline]
fn kummer_elliptic_perimeter_range(radii: Vec2) -> f64 {
    let Vec2 { x, y } = radii;
    let h = ((x - y) / (x + y)).powi(2);

    const BINOM_SQUARED_REMAINDER: f64 = 0.00101416479131503;
    // = 4. / PI - (1. + 1. / 4. + 1. / 64. + 1. / 256. + 25. / 16384. + 49. / 65536. + 441. / 1048576.)

    PI * BINOM_SQUARED_REMAINDER * h.powi(7) * (x + y)
}

/// Calculates circumference C of an ellipse with radii (x, y) using the arithmetic-geometric mean,
/// as described in equation 19.8.6 of
/// <https://web.archive.org/web/20240926233336/https://dlmf.nist.gov/19.8#i>.
fn agm_elliptic_perimeter(accuracy: f64, radii: Vec2) -> f64 {
    let Vec2 { x, y } = if radii.x >= radii.y {
        radii
    } else {
        Vec2::new(radii.y, radii.x)
    };

    let accuracy = accuracy / (2. * PI * x);

    let mut sum = 1.;
    let mut a = 1.;
    let mut g = y / x;
    let mut c = (1. - g.powi(2)).sqrt();
    let mut mul = 0.5;

    loop {
        let c2 = c.powi(2);
        // term = 2^(n-1) c_n^2
        let term = mul * c2;
        sum -= term;

        // We have c_(n+1) ≤ 1/2 c_n
        // (for a derivation, see e.g. section 2.1 of  "Elliptic integrals, the
        // arithmetic-geometric mean and the Brent-Salamin algorithm for π" by G.J.O. Jameson:
        // <https://web.archive.org/web/20241002140956/https://www.maths.lancs.ac.uk/jameson/ellagm.pdf>)
        //
        // Therefore
        // ∑ 2^(i-1) c_i^2 from i = 1 ≤ ∑ 2^(i-1) ((1/2)^i c_0)^2 from i = 1
        //                            = ∑ 2^-(i+1) c_0^2          from i = 1
        //                            = 1/2 c_0^2
        //
        // or, for arbitrary starting point i = n+1:
        // ∑ 2^(i-1) c_i^2 from i = n+1 ≤ ∑ 2^(i-1) ((1/2)^(i-n) c_n)^2 from i = n+1
        //                              = ∑ 2^(2n - i - 1) c_n^2        from i = n+1
        //                              = 2^(2n) ∑ 2^(-(i+1)) c_n^2     from i = n+1
        //                              = 2^(2n) 2^(-(n+1)) c_n^2
        //                              = 2^(n-1) c_n^2
        //
        // Therefore, the remainder of the series sums to less than or equal to 2^(n-1) c_n^2,
        // which is exactly the value of the nth term.
        //
        // Furthermore, a_m ≥ g_n, and g_n ≤ 1, for all m, n.
        if term <= accuracy * g {
            // `sum` currently overestimates the true value - subtract the upper bound of the
            // remaining series. We will then underestimate the true value, but by no more than
            // `accuracy`.
            sum -= term;
            break;
        }

        mul *= 2.;
        // This is equal to c_next = c^2 / (4 * a_next)
        c = (a - g) / 2.;
        let a_next = (a + g) / 2.;
        g = (a * g).sqrt();
        a = a_next;
    }

    2. * PI * x / a * sum
}

#[cfg(test)]
mod tests {
    use crate::{Circle, Ellipse, Point, Shape};
    use std::f64::consts::PI;

    fn assert_approx_eq(x: f64, y: f64) {
        // Note: we might want to be more rigorous in testing the accuracy
        // of the conversion into Béziers. But this seems good enough.
        assert!((x - y).abs() < 1e-7, "{x} != {y}");
    }

    #[test]
    fn circular_perimeter() {
        for radius in [1.0, 0., 1.5, PI, 10.0, -1.0, 1_234_567_890.1] {
            let circle = Circle::new((0., 0.), radius);
            let ellipse = Ellipse::new((0., 0.), (radius, radius), 0.);

            let circle_p = circle.perimeter(0.);
            let ellipse_p = ellipse.perimeter(0.1);

            // We expect the results to be identical, modulo potential roundoff errors. Compare
            // with a very small relative epsilon.
            let epsilon = f64::EPSILON * 8. * circle_p.max(ellipse_p);
            assert!(
                (circle_p - ellipse_p).abs() <= epsilon,
                "Expected circular ellipse radius {ellipse_p} to be equal to circle radius {circle_p} for radius {radius}"
            );
        }
    }

    #[test]
    fn compare_perimeter_with_bez() {
        const EPSILON: f64 = 0.000_002;

        for radii in [
            (0.5, 1.),
            (2., 1.),
            (0.000_000_1, 1.),
            (0., 1.),
            (1., 0.),
            (-0.5, 1.),
            (-0.000_000_1, -1.),
        ] {
            let ellipse = Ellipse::new((0., 0.), radii, 0.);

            let ellipse_p = ellipse.perimeter(0.000_001);
            let bez_p = ellipse.path_segments(0.000_000_25).perimeter(0.000_000_25);

            assert!(
                (ellipse_p - bez_p).abs() < EPSILON,
                "Numerically approximated ellipse perimeter ({ellipse_p}) does not match bezier segment perimeter length ({bez_p}) for radii {radii:?}"
            );
        }
    }

    #[test]
    fn known_perimeter() {
        const ACCURACY: f64 = 0.000_000_000_001;

        for (radii, perimeter) in [
            ((0.5, 1.), 4.844_224_110_273_838),
            ((0.001, 1.), 4.000_015_588_104_688),
        ] {
            let ellipse = Ellipse::new((0., 0.), radii, 0.);

            let ellipse_p = ellipse.perimeter(ACCURACY);
            assert!(
                (ellipse_p - perimeter).abs() <= ACCURACY,
                "Numerically approximated ellipse perimeter ({ellipse_p}) does not match known perimeter ({perimeter}) radii {radii:?}"
            );
        }
    }

    #[test]
    fn area_sign() {
        let center = Point::new(5.0, 5.0);
        let e = Ellipse::new(center, (5.0, 5.0), 1.0);
        assert_approx_eq(e.area(), 25.0 * PI);
        let e = Ellipse::new(center, (5.0, 10.0), 1.0);
        assert_approx_eq(e.area(), 50.0 * PI);

        assert_eq!(e.winding(center), 1);

        let p = e.to_path(1e-9);
        assert_approx_eq(e.area(), p.area());
        assert_eq!(e.winding(center), p.winding(center));

        let e_neg_radius = Ellipse::new(center, (-5.0, 10.0), 1.0);
        assert_approx_eq(e_neg_radius.area(), 50.0 * PI);

        assert_eq!(e_neg_radius.winding(center), 1);

        let p_neg_radius = e_neg_radius.to_path(1e-9);
        assert_approx_eq(e_neg_radius.area(), p_neg_radius.area());
        assert_eq!(e_neg_radius.winding(center), p_neg_radius.winding(center));
    }
}
