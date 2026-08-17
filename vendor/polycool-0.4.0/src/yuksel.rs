// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An implementation of Yuksel's robust version of Newton's algorithm.

fn different_signs(x: f64, y: f64) -> bool {
    (x < 0.0) != (y < 0.0)
}

pub(crate) fn find_root<F: Fn(f64) -> f64, DF: Fn(f64) -> f64>(
    f: F,
    deriv: DF,
    mut lower: f64,
    mut upper: f64,
    val_lower: f64,
    val_upper: f64,
    x_error: f64,
) -> f64 {
    if !val_lower.is_finite() || !val_upper.is_finite() {
        return f64::NAN;
    }
    debug_assert!(different_signs(val_lower, val_upper));

    let mut x = lower + (upper - lower) / 2.0;
    let mut step = (upper - lower) / 2.0;

    if step.abs() <= x_error {
        return x;
    }

    while step.abs() > x_error && x.is_finite() {
        // There's potentially a performance gain to be had by evaluating the
        // derivative and the polynomial "jointly", so that subexpressions can
        // be shared. At least for the cubic case, the compiler is smart enough
        // to do that for us.
        let deriv_x = deriv(x);
        let val_x = f(x);

        // By returning early on zero, we make it impossible for `step` to
        // be NaN. This seems to benchmark a little better than checking for
        // NaN after the fact.
        if val_x == 0.0 {
            return x;
        }
        let root_in_first_half = different_signs(val_lower, val_x);
        if root_in_first_half {
            upper = x;
        } else {
            lower = x;
        }

        debug_assert!(deriv_x.is_finite());
        debug_assert!(val_x.is_finite());

        step = -val_x / deriv_x;
        let mut new_x = x + step;

        if new_x <= lower || new_x >= upper {
            new_x = lower + (upper - lower) / 2.0;

            if new_x == upper || new_x == lower {
                // This should be rare, but it happens if they ask for more
                // accuracy than is reasonable. For example, suppose (because
                // of large coefficients) the output value jumps from -1.0
                // to 1.0 between adjacent floats and they ask for an output
                // error of smaller than 0.5. Then we'll eventually shrink
                // the search interval to a pair of adjacent floats and hit
                // this case.
                return new_x;
            }
        }
        step = new_x - x;
        x = new_x;
    }
    x
}
