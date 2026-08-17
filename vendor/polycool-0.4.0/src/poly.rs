// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use arrayvec::ArrayVec;

/// A polynomial whose degree is known at compile-time.
///
/// Although this supports polynomials of arbitrary degree, it is intended
/// for low-degree polynomials. For example, the coefficients are stored
/// in an array, and so they will be stack-allocated (unless you `Box`
/// the `Poly`, of course) tend to be copied around.
///
/// Polynomial multiplication is not yet implemented, because doing it "nicely"
/// would require const generic expressions: ideally we'd do something like
///
/// ```ignore
/// impl<N, M> Mul<Poly<M>> for Poly<N> {
///     type Output = Poly<{M + N - 1}>;
/// }
/// ```
///
/// It's possible to work around this with macros, but there are lots of
/// possibilities and I didn't feel like it was worth the trouble (and the hit
/// to compilation time).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Poly<const N: usize> {
    pub(crate) coeffs: [f64; N],
}

/// A polynomial of degree 2.
pub type Quadratic = Poly<3>;

/// A polynomial of degree 3.
pub type Cubic = Poly<4>;

/// A polynomial of degree 4.
pub type Quartic = Poly<5>;

/// A polynomial of degree 5.
pub type Quintic = Poly<6>;

impl<const N: usize> Poly<N> {
    /// Creates a new polynomial with the provided coefficients.
    ///
    /// The constant coefficient comes first, then the linear coefficient, and
    /// so on. So if you pass `[c, b, a]` you'll get the polynomial
    /// `a x^2 + b x + c`.
    pub const fn new(coeffs: [f64; N]) -> Poly<N> {
        Poly { coeffs }
    }

    /// The coefficients of this polynomial.
    ///
    /// In the returned array, the coefficient of `x^i` is at index `i`.
    pub fn coeffs(&self) -> &[f64; N] {
        &self.coeffs
    }

    /// Evaluates this polynomial at a point.
    pub fn eval(&self, x: f64) -> f64 {
        let mut acc = 0.0;
        for c in self.coeffs.iter().rev() {
            // It would be nice to use `f64::mul_add` here, but it's slow on
            // architectures that don't have a dedicated instruction.
            acc = acc * x + c;
        }
        acc
    }

    /// Returns the largest absolute value of any coefficient.
    ///
    /// Always returns a non-negative number, or NaN if some coefficient is NaN.
    pub fn max_abs_coefficient(&self) -> f64 {
        let mut max = 0.0f64;
        for c in &self.coeffs {
            max = max.max(c.abs());
        }
        max
    }

    /// Are all the coefficients finite?
    pub fn is_finite(&self) -> bool {
        self.coeffs.iter().all(|c| c.is_finite())
    }
}

macro_rules! impl_deriv_and_deflate {
    ($N:literal, $N_MINUS_ONE:literal) => {
        impl Poly<$N> {
            /// Compute the derivative of this polynomial, as a polynomial with
            /// one less coefficient.
            pub fn deriv(&self) -> Poly<$N_MINUS_ONE> {
                let mut coeffs = [0.0; $N_MINUS_ONE];
                for (i, (d, c)) in coeffs.iter_mut().zip(&self.coeffs[1..]).enumerate() {
                    *d = (i + 1) as f64 * c;
                }
                Poly::new(coeffs)
            }

            /// Divide this polynomial by the polynomial `x - root`, returning the
            /// quotient (as a polynomial with one less coefficient) and ignoring
            /// the remainder.
            ///
            /// If `root` is actually a root of `self` (as the name suggests
            /// it should be, but this is not actually required), the
            /// remainder will be zero. In general, the remainder will be
            /// `self.eval(root)`.
            pub fn deflate(&self, root: f64) -> Poly<$N_MINUS_ONE> {
                let mut acc = 0.0;
                let mut coeffs = [0.0; $N_MINUS_ONE];
                for (d, c) in coeffs.iter_mut().zip(&self.coeffs[1..]).rev() {
                    acc = acc * root + c;
                    *d = acc;
                }
                Poly::new(coeffs)
            }
        }
    };
}

macro_rules! impl_roots_between_recursive {
    ($N:literal, $N_MINUS_ONE:literal) => {
        impl Poly<$N> {
            /// Computes all roots between `lower` and `upper`, to the desired accuracy.
            ///
            /// We make no guarantees about multiplicity. For example, if there's a
            /// double-root that isn't a triple-root (and therefore has no sign change
            /// nearby) then there's a good chance we miss it altogether. This is
            /// fine if you're using this root-finding to find critical points for
            /// optimizing a polynomial, because roots that don't come with a sign
            /// change aren't local extrema.
            pub fn roots_between(
                self,
                lower: f64,
                upper: f64,
                x_error: f64,
            ) -> ArrayVec<f64, $N_MINUS_ONE> {
                let mut ret = ArrayVec::new();
                let mut scratch = ArrayVec::new();
                self.roots_between_with_buffer(lower, upper, x_error, &mut ret, &mut scratch);
                ret
            }

            // This would ideally have a `where M >= N - 1` bound on it,
            // but it's private so it shouldn't matter too much.
            // We assume that `scratch` and `out` are both empty.
            fn roots_between_with_buffer<const M: usize>(
                self,
                lower: f64,
                upper: f64,
                x_error: f64,
                out: &mut ArrayVec<f64, M>,
                scratch: &mut ArrayVec<f64, M>,
            ) {
                let deriv = self.deriv();
                if !deriv.is_finite() {
                    return;
                }
                deriv.roots_between_with_buffer(lower, upper, x_error, scratch, out);
                scratch.push(upper);
                out.clear();
                let mut last = lower;
                let mut last_val = self.eval(last);

                // `endpoint` now contains all the critical points (in increasing order)
                // and the upper endpoint of the interval. These are the endpoints
                // of the potential bracketing intervals of our polynomial.
                for &mut x in scratch {
                    let val = self.eval(x);
                    if $crate::different_signs(last_val, val) {
                        out.push($crate::yuksel::find_root(
                            |x| self.eval(x),
                            |x| deriv.eval(x),
                            last,
                            x,
                            last_val,
                            val,
                            x_error,
                        ));
                    }

                    last = x;
                    last_val = val;
                }
            }
        }
    };
}

impl_deriv_and_deflate!(3, 2);
impl_deriv_and_deflate!(4, 3);
impl_deriv_and_deflate!(5, 4);
impl_deriv_and_deflate!(6, 5);
impl_deriv_and_deflate!(7, 6);
impl_deriv_and_deflate!(8, 7);
impl_deriv_and_deflate!(9, 8);
impl_deriv_and_deflate!(10, 9);

impl_roots_between_recursive!(5, 4);
impl_roots_between_recursive!(6, 5);
impl_roots_between_recursive!(7, 6);
impl_roots_between_recursive!(8, 7);
impl_roots_between_recursive!(9, 8);
impl_roots_between_recursive!(10, 9);

impl<const N: usize> core::ops::Mul<f64> for Poly<N> {
    type Output = Poly<N>;

    fn mul(mut self, scale: f64) -> Poly<N> {
        self *= scale;
        self
    }
}

impl<const N: usize> core::ops::MulAssign<f64> for Poly<N> {
    fn mul_assign(&mut self, scale: f64) {
        for c in &mut self.coeffs {
            *c *= scale;
        }
    }
}

impl<const N: usize> core::ops::Mul<f64> for &Poly<N> {
    type Output = Poly<N>;

    fn mul(self, scale: f64) -> Poly<N> {
        (*self) * scale
    }
}

impl<const N: usize> core::ops::Div<f64> for Poly<N> {
    type Output = Poly<N>;

    fn div(mut self, scale: f64) -> Poly<N> {
        self /= scale;
        self
    }
}

impl<const N: usize> core::ops::DivAssign<f64> for Poly<N> {
    fn div_assign(&mut self, scale: f64) {
        for c in &mut self.coeffs {
            *c /= scale;
        }
    }
}

impl<const N: usize> core::ops::Div<f64> for &Poly<N> {
    type Output = Poly<N>;

    fn div(self, scale: f64) -> Poly<N> {
        (*self) / scale
    }
}

impl<const N: usize> core::ops::AddAssign<&Poly<N>> for Poly<N> {
    fn add_assign(&mut self, rhs: &Poly<N>) {
        for (c, d) in self.coeffs.iter_mut().zip(rhs.coeffs) {
            *c += d;
        }
    }
}

impl<const N: usize> core::ops::AddAssign<Poly<N>> for Poly<N> {
    fn add_assign(&mut self, rhs: Poly<N>) {
        *self += &rhs;
    }
}

impl<const N: usize> core::ops::Add<Poly<N>> for Poly<N> {
    type Output = Poly<N>;

    fn add(mut self, rhs: Poly<N>) -> Poly<N> {
        self += rhs;
        self
    }
}

impl<const N: usize> core::ops::Add<&Poly<N>> for Poly<N> {
    type Output = Poly<N>;

    fn add(mut self, rhs: &Poly<N>) -> Poly<N> {
        self += rhs;
        self
    }
}

impl<const N: usize> core::ops::Add<Poly<N>> for &Poly<N> {
    type Output = Poly<N>;

    fn add(self, mut rhs: Poly<N>) -> Poly<N> {
        rhs += self;
        rhs
    }
}

impl<const N: usize> core::ops::SubAssign<&Poly<N>> for Poly<N> {
    fn sub_assign(&mut self, rhs: &Poly<N>) {
        for (c, d) in self.coeffs.iter_mut().zip(rhs.coeffs) {
            *c -= d;
        }
    }
}

impl<const N: usize> core::ops::SubAssign<Poly<N>> for Poly<N> {
    fn sub_assign(&mut self, rhs: Poly<N>) {
        *self -= &rhs;
    }
}

impl<const N: usize> core::ops::Sub<Poly<N>> for Poly<N> {
    type Output = Poly<N>;

    fn sub(mut self, rhs: Poly<N>) -> Poly<N> {
        self -= rhs;
        self
    }
}

impl<const N: usize> core::ops::Sub<&Poly<N>> for Poly<N> {
    type Output = Poly<N>;

    fn sub(mut self, rhs: &Poly<N>) -> Poly<N> {
        self -= rhs;
        self
    }
}

impl<const N: usize> core::ops::Sub<Poly<N>> for &Poly<N> {
    type Output = Poly<N>;

    fn sub(self, mut rhs: Poly<N>) -> Poly<N> {
        rhs -= self;
        rhs
    }
}

// We do property-testing with two strategies:
//
// - for the "value-testing" strategy, we test that the polynomial
//   approximately evaluates to zero on all the claimed roots.
// - for the "planted root" strategy, we generate a polynomial with
//   a known root and check that we find it
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let p = Poly::new([-6.0, 11.0, -6.0, 1.0]);

        let roots = p.roots_between(0.0, 5.0, 1e-6);
        assert_eq!(roots.len(), 3);
        assert!((roots[0] - 1.0).abs() <= 1e-6);
        assert!((roots[1] - 2.0).abs() <= 1e-6);
        assert!((roots[2] - 3.0).abs() <= 1e-6);

        let p = Poly::new([24.0, -50.0, 35.0, -10.0, 1.0]);

        let roots = p.roots_between(0.0, 5.0, 1e-6);
        assert_eq!(roots.len(), 4);
        assert!((roots[0] - 1.0).abs() <= 1e-6);
        assert!((roots[1] - 2.0).abs() <= 1e-6);
        assert!((roots[2] - 3.0).abs() <= 1e-6);
        assert!((roots[3] - 4.0).abs() <= 1e-6);
    }

    // Asserts that the supplied "roots" are close to being roots of the
    // cubic, in the sense that the cubic evaluates to approximately zero
    // on each of the roots.
    fn check_root_values<const N: usize>(p: &Poly<N>, roots: &[f64]) {
        // Arbitrary cubics can have coefficients with wild magnitudes,
        // so we need to adjust our error expectations accordingly.
        let magnitude = p.max_abs_coefficient().max(1.0);
        let accuracy = magnitude * 1e-12;

        for r in roots {
            // We can't expect great accuracy for very large roots,
            // because the polynomial evaluation will involve very
            // large terms.
            let accuracy = accuracy * r.abs().powi(N as i32 - 1).max(1.0);
            let y = p.eval(*r);
            assert!(
                y.abs() <= accuracy,
                "poly {p:?} had root {r} evaluate to {y:?}, but expected {accuracy:?}"
            );
        }
    }

    #[test]
    fn root_evaluation_deg3() {
        arbtest::arbtest(|u| {
            let poly: Poly<4> = crate::arbitrary::poly(u)?;
            // Ignore very large polynomials, because they'll just overflow everything.
            if (poly.max_abs_coefficient() * 10.0f64.powi(4)).is_infinite() {
                return Err(arbitrary::Error::IncorrectFormat);
            }
            let roots = poly.roots_between(-10.0, 10.0, 1e-13);

            check_root_values(&poly, &roots);
            Ok(())
        })
        .budget_ms(5_000);
    }

    #[test]
    fn root_evaluation_deg4() {
        arbtest::arbtest(|u| {
            let poly: Poly<5> = crate::arbitrary::poly(u)?;
            if (poly.max_abs_coefficient() * 10.0f64.powi(5)).is_infinite() {
                return Err(arbitrary::Error::IncorrectFormat);
            }
            let roots = poly.roots_between(-10.0, 10.0, 1e-13);
            check_root_values(&poly, &roots);
            Ok(())
        })
        .budget_ms(5_000);
    }

    #[test]
    fn root_evaluation_deg9() {
        arbtest::arbtest(|u| {
            let poly: Poly<10> = crate::arbitrary::poly(u)?;
            if (poly.max_abs_coefficient() * 10.0f64.powi(11)).is_infinite() {
                return Err(arbitrary::Error::IncorrectFormat);
            }
            let roots = poly.roots_between(-10.0, 10.0, 1e-13);
            check_root_values(&poly, &roots);
            Ok(())
        })
        .budget_ms(5_000);
    }

    #[test]
    fn planted_root_deg5() {
        arbtest::arbtest(|u| {
            let planted_root = crate::arbitrary::float_in_unit_interval(u)?;
            let poly: Poly<6> = crate::arbitrary::poly_with_planted_root(u, planted_root, 1e-6)?;

            // Bear in mind that Yuksel's algorithm needs iterated derivatives to be
            // finite (and that we aren't doing any preconditioning or normalization yet),
            // ensure that the polynomial isn't too big.
            if (poly.max_abs_coefficient() * 1024.0).is_infinite() {
                return Err(arbitrary::Error::IncorrectFormat);
            }
            let roots = poly.roots_between(-2.0, 2.0, 1e-13);

            assert!(roots.iter().all(|r| r.is_finite()));

            // Check that the roots are sorted.
            assert!(roots.is_sorted());
            assert!(roots.iter().all(|r| (-2.0..=2.0).contains(r)));

            // We can't expect great accuracy for huge coefficients, because the
            // evaluations during Newton iteration are subject to error.
            let error = poly.max_abs_coefficient().max(1.0) * 1e-12;
            assert!(roots.iter().any(|r| (r - planted_root).abs() <= error));
            Ok(())
        })
        .budget_ms(5_000);
    }
}
