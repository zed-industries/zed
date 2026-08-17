// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use arrayvec::ArrayVec;

use crate::{Cubic, Quadratic, different_signs};

#[cfg(feature = "libm")]
#[allow(unused_imports, reason = "unused if libm and std are both around")]
use crate::libm_polyfill::FloatFuncs as _;

// We assume that there is at least one element, at most 3 elements, and buf[1]
// and buf[2] are already in the right order (if present).
//
// This might be faster than the stdlib sort for our special case,
// but the real reason it's here is for no_std support.
fn partial_sort<const M: usize>(buf: &mut ArrayVec<f64, M>) {
    if buf.len() > 1 && buf[0] > buf[1] {
        buf.swap(0, 1);
        if buf.len() > 2 && buf[1] > buf[2] {
            buf.swap(1, 2);
        }
    }
}

impl Cubic {
    /// This is like [`Cubic::eval`] but faster.
    ///
    /// It would be nice if we could just make `eval` like this, but I couldn't
    /// figure out how, given the lack of specialization.
    ///
    /// Marked `pub` for benchmarking.
    #[doc(hidden)]
    pub fn eval_opt(&self, x: f64) -> f64 {
        let [c0, c1, c2, c3] = self.coeffs;
        let xx = x * x;
        let xxx = xx * x;
        c0 + c1 * x + c2 * xx + c3 * xxx
    }

    /// Evaluate this cubic and its gradient at the same time, reusing some
    /// of the intermediate computations.
    ///
    /// In micro-benchmarks, this is faster than evaluating separately. But
    /// it doesn't help with the performance of Yuksel's algorithm, presumably
    /// because the compiler is inlining and optimizing reuse of the
    /// intermediate computations already.
    ///
    /// Marked `pub` for benchmarking.
    #[doc(hidden)]
    pub fn eval_with_deriv_opt(&self, deriv: &Quadratic, x: f64) -> (f64, f64) {
        let [c0, c1, c2, c3] = self.coeffs;
        let [d0, d1, d2] = deriv.coeffs;
        let xx = x * x;
        let xxx = xx * x;
        (c0 + c1 * x + c2 * xx + c3 * xxx, d0 + d1 * x + d2 * xx)
    }

    /// Computes the critical points of this cubic, as long
    /// as the discriminant of the derivative is positive.
    /// The return values are in increasing order.
    ///
    /// Some corner cases worth noting:
    ///   - If the discriminant is zero, returns nothing. That is, we don't find
    ///     double-roots of the derivative. These aren't needed anyway, as these
    ///     critical points are used to construct bracketing intervals for the
    ///     roots. Double-roots of the derivative are inflection points of the
    ///     cubic, and they aren't needed to bracket roots.
    ///   - If the derivative is linear or close to it, we might
    ///     return +/- infinity as one of the roots.
    ///   - Unless some input is NaN, we don't return NaN.
    fn critical_points(&self) -> Option<(f64, f64)> {
        let a = 3.0 * self.coeffs[3];
        let b_2 = self.coeffs[2];
        let c = self.coeffs[1];
        let disc_4 = b_2 * b_2 - a * c;

        if !disc_4.is_finite() {
            return self.rescaled_critical_points();
        }

        if disc_4 > 0.0 {
            let q = -(b_2 + disc_4.sqrt().copysign(b_2));
            let r0 = q / a;
            let r1 = c / q;
            Some((r0.min(r1), r0.max(r1)))
        } else {
            None
        }
    }

    #[cold]
    fn rescaled_critical_points(&self) -> Option<(f64, f64)> {
        let scale = 2.0f64.powi(-515);
        (*self * scale).critical_points()
    }

    fn one_root(
        &self,
        lower: f64,
        upper: f64,
        lower_val: f64,
        upper_val: f64,
        x_error: f64,
    ) -> f64 {
        let deriv = self.deriv();
        if !deriv.is_finite() {
            return f64::NAN;
        }
        crate::yuksel::find_root(
            |x| self.eval_opt(x),
            |x| deriv.eval_opt(x),
            lower,
            upper,
            lower_val,
            upper_val,
            x_error,
        )
    }

    fn first_root(self, lower: f64, upper: f64, x_error: f64) -> Option<f64> {
        if let Some((x0, x1)) = self.critical_points() {
            let possible_endpoints: [f64; 3] = [x0, x1, upper];
            let mut last = lower;
            let mut last_val = self.eval(last);
            for x in possible_endpoints {
                if x > last && x <= upper {
                    let val = self.eval(x);
                    if different_signs(last_val, val) {
                        return Some(self.one_root(last, x, last_val, val, x_error));
                    }

                    last = x;
                    last_val = val;
                }
            }
            None
        } else {
            let lower_val = self.eval(lower);
            let upper_val = self.eval(upper);
            if different_signs(lower_val, upper_val) {
                Some(self.one_root(lower, upper, lower_val, upper_val, x_error))
            } else {
                None
            }
        }
    }

    /// Computes all roots between `lower` and `upper`, to the desired accuracy.
    ///
    /// We make no guarantees about multiplicity. In fact, if there's a
    /// double-root that isn't a triple-root (and therefore has no sign change
    /// nearby) then there's a good chance we miss it altogether. This is
    /// fine if you're using this root-finding to optimize a quartic, because
    /// double-roots of the derivative aren't local extrema.
    pub fn roots_between(self, lower: f64, upper: f64, x_error: f64) -> ArrayVec<f64, 3> {
        let mut ret = ArrayVec::new();
        let mut scratch = ArrayVec::new();
        self.roots_between_with_buffer(lower, upper, x_error, &mut ret, &mut scratch);
        ret
    }

    pub(crate) fn roots_between_with_buffer<const M: usize>(
        self,
        lower: f64,
        upper: f64,
        x_error: f64,
        out: &mut ArrayVec<f64, M>,
        _scratch: &mut ArrayVec<f64, M>,
    ) {
        if let Some(r) = self.first_root(lower, upper, x_error) {
            out.push(r);
            let quad = self.deflate(r);
            if let Some((x0, x1)) = quad.positive_discriminant_roots() {
                if lower <= x0 && x0 <= upper {
                    out.push(x0);
                }
                if lower <= x1 && x1 <= upper {
                    out.push(x1);
                }

                // `self.first_root` is supposed to return the smallest root in
                // our interval, but it's possible it doesn't because it misses
                // a double-root (or near-double-root).
                if lower <= x0 && x0 < r {
                    partial_sort(out);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Embarrassingly, an early version gave NaN on x^3, because
    // the Newton iterations ran into 0.0/0.0.
    #[test]
    fn x_cubed() {
        let p = Cubic::new([0.0, 0.0, 0.0, 1.0]);
        let roots = p.roots_between(-1.0, 1.0, 1e-6);
        assert_eq!(roots.len(), 1);
        assert!(roots[0].abs() <= 1e-6);
    }
}
