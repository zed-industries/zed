// Copyright 2012 Google Inc.
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use super::Scalar64;

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use tiny_skia_path::NoStdFloat;

pub fn push_valid_ts(s: &[f64], real_roots: usize, t: &mut [f64]) -> usize {
    let mut found_roots = 0;
    'outer: for index in 0..real_roots {
        let mut t_value = s[index];
        if t_value.approximately_zero_or_more() && t_value.approximately_one_or_less() {
            t_value = t_value.bound(0.0, 1.0);

            for idx2 in 0..found_roots {
                if t[idx2].approximately_equal(t_value) {
                    continue 'outer;
                }
            }

            t[found_roots] = t_value;
            found_roots += 1;
        }
    }

    found_roots
}

// note: caller expects multiple results to be sorted smaller first
// note: http://en.wikipedia.org/wiki/Loss_of_significance has an interesting
//  analysis of the quadratic equation, suggesting why the following looks at
//  the sign of B -- and further suggesting that the greatest loss of precision
//  is in b squared less two a c
pub fn roots_valid_t(a: f64, b: f64, c: f64, t: &mut [f64]) -> usize {
    let mut s = [0.0; 3];
    let real_roots = roots_real(a, b, c, &mut s);
    push_valid_ts(&s, real_roots, t)
}

// Numeric Solutions (5.6) suggests to solve the quadratic by computing
//     Q = -1/2(B + sgn(B)Sqrt(B^2 - 4 A C))
// and using the roots
//     t1 = Q / A
//     t2 = C / Q
//
// this does not discard real roots <= 0 or >= 1
pub fn roots_real(a: f64, b: f64, c: f64, s: &mut [f64; 3]) -> usize {
    if a == 0.0 {
        return handle_zero(b, c, s);
    }

    let p = b / (2.0 * a);
    let q = c / a;
    if a.approximately_zero() && (p.approximately_zero_inverse() || q.approximately_zero_inverse())
    {
        return handle_zero(b, c, s);
    }

    // normal form: x^2 + px + q = 0
    let p2 = p * p;
    if !p2.almost_dequal_ulps(q) && p2 < q {
        return 0;
    }

    let mut sqrt_d = 0.0;
    if p2 > q {
        sqrt_d = (p2 - q).sqrt();
    }

    s[0] = sqrt_d - p;
    s[1] = -sqrt_d - p;
    1 + usize::from(!s[0].almost_dequal_ulps(s[1]))
}

fn handle_zero(b: f64, c: f64, s: &mut [f64; 3]) -> usize {
    if b.approximately_zero() {
        s[0] = 0.0;
        (c == 0.0) as usize
    } else {
        s[0] = -c / b;
        1
    }
}
