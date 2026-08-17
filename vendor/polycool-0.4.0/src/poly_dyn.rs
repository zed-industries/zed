// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Polynomials of dynamic (run-time) degree.

use crate::{Cubic, Quadratic, different_signs, yuksel};

/// A polynomial of dynamic degree.
///
/// It would be nice to have polynomials of type-level degree,
/// but that's a bit awkward without const generic expressions
/// (e.g. to express the type of the derivative). It could be
/// done with `typenum` and `generic_array`...
#[derive(Clone, Debug)]
pub struct PolyDyn {
    /// Coefficients in increasing order of degree.
    ///
    /// For example, `coeffs[0]` is the constant term.
    coeffs: Vec<f64>,
}

impl<'a> std::ops::Mul<&'a PolyDyn> for &'a PolyDyn {
    type Output = PolyDyn;

    fn mul(self, rhs: &PolyDyn) -> PolyDyn {
        let mut coeffs = vec![0.0; (self.coeffs.len() + rhs.coeffs.len()).saturating_sub(1)];

        for (i, c) in self.coeffs.iter().enumerate() {
            for (j, d) in rhs.coeffs.iter().enumerate() {
                coeffs[i + j] += c * d;
            }
        }
        PolyDyn { coeffs }
    }
}

impl std::ops::Mul<&PolyDyn> for PolyDyn {
    type Output = PolyDyn;

    fn mul(self, rhs: &PolyDyn) -> PolyDyn {
        (&self) * rhs
    }
}

impl PolyDyn {
    /// Constructs a new polynomial from coefficients.
    ///
    /// The first coefficient provided will be the constant term, the second will
    /// be the linear term, and so on.
    pub fn new(coeffs: impl IntoIterator<Item = f64>) -> Self {
        PolyDyn {
            coeffs: coeffs.into_iter().collect(),
        }
    }

    /// The coefficients of this polynomial.
    ///
    /// In the returned slice, the coefficient of `x^i` is at index `i`.
    pub fn coeffs(&self) -> &[f64] {
        &self.coeffs
    }

    fn is_finite(&self) -> bool {
        self.coeffs.iter().all(|c| c.is_finite())
    }

    /// Returns the polynomial that's the derivative of this polynomial.
    pub fn deriv(&self) -> PolyDyn {
        let mut coeffs = Vec::with_capacity(self.coeffs.len().saturating_sub(1));
        // If we're empty (meaning that we're the constant zero polynomial),
        // this will just return the zero polynomial again: no need for a
        // special case.
        for (i, c) in self.coeffs.iter().enumerate().skip(1) {
            coeffs.push(c * (i as f64));
        }
        PolyDyn { coeffs }
    }

    /// Evaluates this polynomial at a point.
    pub fn eval(&self, x: f64) -> f64 {
        let mut ret = 0.0;
        let mut x_pow = 1.0;
        for &c in &self.coeffs {
            ret += c * x_pow;
            x_pow *= x;
        }
        ret
    }

    /// The degree of this polynomial.
    ///
    /// This function only looks at the *presence* of coefficients, not their
    /// value. If you construct a polynomial with three coefficients, this
    /// method will say that it has degree 2 even if all of those coefficients
    /// are zero.
    ///
    /// A polynomial with no coefficients will give zero as its degree, as will
    /// a polynomial with one coefficient.
    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }

    /// If this polynomial has degree 3 or less, converts it to a [cubic](crate::Cubic).
    fn to_cubic(&self) -> Option<Cubic> {
        if self.degree() <= 3 {
            Some(Cubic::new([
                self.coeffs.first().copied().unwrap_or(0.0),
                self.coeffs.get(1).copied().unwrap_or(0.0),
                self.coeffs.get(2).copied().unwrap_or(0.0),
                self.coeffs.get(3).copied().unwrap_or(0.0),
            ]))
        } else {
            None
        }
    }

    /// If this polynomial has degree 2 or less, converts it to a [cubic](crate::Quadratic).
    fn to_quadratic(&self) -> Option<Quadratic> {
        if self.degree() <= 2 {
            Some(Quadratic::new([
                self.coeffs.first().copied().unwrap_or(0.0),
                self.coeffs.get(1).copied().unwrap_or(0.0),
                self.coeffs.get(2).copied().unwrap_or(0.0),
            ]))
        } else {
            None
        }
    }

    /// Finds all the roots in an interval, using Yuksel's algorithm.
    ///
    /// This is a numerical, iterative method. It first constructs critical
    /// points to find bracketing intervals (intervals `[x0, x1]` where
    /// `self.eval(x0)` and `self.eval(x1)` have different signs). Then it uses
    /// a kind of modified Newton method to find a root on each bracketing
    /// interval. It has a few limitations:
    ///
    /// - if there is only a small interval where the polynomial changes sign,
    ///   it can miss roots. For example, when two roots are very close together
    ///   it can miss them both.
    /// - run time is quadratic in the degree. However, it is often very fast
    ///   in practice for polynomials of low degree, especially if the interval
    ///   `[lower, upper]` contains few roots.
    pub fn roots_between(&self, lower: f64, upper: f64, x_error: f64) -> Vec<f64> {
        let mut ret = Vec::new();
        let mut scratch = Vec::new();
        self.roots_between_with_buffer(lower, upper, x_error, &mut ret, &mut scratch);
        ret
    }

    /// Finds all the roots in an interval, using Yuksel's algorithm.
    ///
    /// See [`PolyDyn::roots_between`] for more details. This method differs from that
    /// one in that it performs fewer allocations: you provide an `out` buffer
    /// for the result and a `scratch` buffer for intermediate computations.
    pub fn roots_between_with_buffer(
        &self,
        lower: f64,
        upper: f64,
        x_error: f64,
        out: &mut Vec<f64>,
        scratch: &mut Vec<f64>,
    ) {
        out.clear();
        scratch.clear();

        if let Some(q) = self.to_quadratic() {
            out.extend(q.roots().into_iter().filter(|&r| lower <= r && r <= upper));
            return;
        }
        if let Some(c) = self.to_cubic() {
            out.extend(c.roots_between(lower, upper, x_error));
            return;
        }

        let deriv = self.deriv();
        if !deriv.is_finite() {
            return;
        }
        deriv.roots_between_with_buffer(lower, upper, x_error, scratch, out);
        scratch.push(upper);
        out.clear();
        let mut last = lower;
        let mut last_val = self.eval(last);

        // `scratch` now contains all the critical points (in increasing order)
        // and the upper endpoint of the interval. If we throw away all the
        // critical points that are outside of (lower, upper), the things
        // remaining in `scratch` are the endpoints of the potential bracketing
        // intervals of our polynomial. So by filtering out uninteresting
        // critical points, this loop is iterating over potential bracketing
        // intervals.
        for &mut x in scratch {
            if x > last && x <= upper {
                let val = self.eval(x);
                if different_signs(last_val, val) {
                    out.push(yuksel::find_root(
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let x_minus_1 = PolyDyn::new([-1.0, 1.0]);
        let x_minus_2 = PolyDyn::new([-2.0, 1.0]);
        let x_minus_3 = PolyDyn::new([-3.0, 1.0]);
        let x_minus_4 = PolyDyn::new([-4.0, 1.0]);

        let p = &x_minus_1 * &x_minus_2 * &x_minus_3 * &x_minus_4;

        let roots = p.roots_between(0.0, 5.0, 1e-6);
        assert_eq!(roots.len(), 4);
        assert!((roots[0] - 1.0).abs() <= 1e-6);
        assert!((roots[1] - 2.0).abs() <= 1e-6);
        assert!((roots[2] - 3.0).abs() <= 1e-6);
        assert!((roots[3] - 4.0).abs() <= 1e-6);
    }
}
