// Copyright 2012 Google Inc.
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/*
Find the intersection of a line and cubic by solving for valid t values.

Analogous to line-quadratic intersection, solve line-cubic intersection by
representing the cubic as:
  x = a(1-t)^3 + 2b(1-t)^2t + c(1-t)t^2 + dt^3
  y = e(1-t)^3 + 2f(1-t)^2t + g(1-t)t^2 + ht^3
and the line as:
  y = i*x + j  (if the line is more horizontal)
or:
  x = i*y + j  (if the line is more vertical)

Then using Mathematica, solve for the values of t where the cubic intersects the
line:

  (in) Resultant[
        a*(1 - t)^3 + 3*b*(1 - t)^2*t + 3*c*(1 - t)*t^2 + d*t^3 - x,
        e*(1 - t)^3 + 3*f*(1 - t)^2*t + 3*g*(1 - t)*t^2 + h*t^3 - i*x - j, x]
  (out) -e     +   j     +
       3 e t   - 3 f t   -
       3 e t^2 + 6 f t^2 - 3 g t^2 +
         e t^3 - 3 f t^3 + 3 g t^3 - h t^3 +
     i ( a     -
       3 a t + 3 b t +
       3 a t^2 - 6 b t^2 + 3 c t^2 -
         a t^3 + 3 b t^3 - 3 c t^3 + d t^3 )

if i goes to infinity, we can rewrite the line in terms of x. Mathematica:

  (in) Resultant[
        a*(1 - t)^3 + 3*b*(1 - t)^2*t + 3*c*(1 - t)*t^2 + d*t^3 - i*y - j,
        e*(1 - t)^3 + 3*f*(1 - t)^2*t + 3*g*(1 - t)*t^2 + h*t^3 - y,       y]
  (out)  a     -   j     -
       3 a t   + 3 b t   +
       3 a t^2 - 6 b t^2 + 3 c t^2 -
         a t^3 + 3 b t^3 - 3 c t^3 + d t^3 -
     i ( e     -
       3 e t   + 3 f t   +
       3 e t^2 - 6 f t^2 + 3 g t^2 -
         e t^3 + 3 f t^3 - 3 g t^3 + h t^3 )

Solving this with Mathematica produces an expression with hundreds of terms;
instead, use Numeric Solutions recipe to solve the cubic.

The near-horizontal case, in terms of:  Ax^3 + Bx^2 + Cx + D == 0
    A =   (-(-e + 3*f - 3*g + h) + i*(-a + 3*b - 3*c + d)     )
    B = 3*(-( e - 2*f +   g    ) + i*( a - 2*b +   c    )     )
    C = 3*(-(-e +   f          ) + i*(-a +   b          )     )
    D =   (-( e                ) + i*( a                ) + j )

The near-vertical case, in terms of:  Ax^3 + Bx^2 + Cx + D == 0
    A =   ( (-a + 3*b - 3*c + d) - i*(-e + 3*f - 3*g + h)     )
    B = 3*( ( a - 2*b +   c    ) - i*( e - 2*f +   g    )     )
    C = 3*( (-a +   b          ) - i*(-e +   f          )     )
    D =   ( ( a                ) - i*( e                ) - j )

For horizontal lines:
(in) Resultant[
      a*(1 - t)^3 + 3*b*(1 - t)^2*t + 3*c*(1 - t)*t^2 + d*t^3 - j,
      e*(1 - t)^3 + 3*f*(1 - t)^2*t + 3*g*(1 - t)*t^2 + h*t^3 - y, y]
(out)  e     -   j     -
     3 e t   + 3 f t   +
     3 e t^2 - 6 f t^2 + 3 g t^2 -
       e t^3 + 3 f t^3 - 3 g t^3 + h t^3
*/

use super::cubic64::{self, Cubic64};
use super::point64::SearchAxis;
use super::Scalar64;

pub fn horizontal_intersect(cubic: &Cubic64, axis_intercept: f64, roots: &mut [f64; 3]) -> usize {
    let (a, b, c, mut d) = cubic64::coefficients(&cubic.as_f64_slice()[1..]);
    d -= axis_intercept;
    let mut count = cubic64::roots_valid_t(a, b, c, d, roots);
    let mut index = 0;
    while index < count {
        let calc_pt = cubic.point_at_t(roots[index]);
        if !calc_pt.y.approximately_equal(axis_intercept) {
            let mut extreme_ts = [0.0; 6];
            let extrema = cubic64::find_extrema(&cubic.as_f64_slice()[1..], &mut extreme_ts);
            count = cubic.search_roots(
                extrema,
                axis_intercept,
                SearchAxis::Y,
                &mut extreme_ts,
                roots,
            );
            break;
        }

        index += 1;
    }

    count
}

pub fn vertical_intersect(cubic: &Cubic64, axis_intercept: f64, roots: &mut [f64; 3]) -> usize {
    let (a, b, c, mut d) = cubic64::coefficients(&cubic.as_f64_slice());
    d -= axis_intercept;
    let mut count = cubic64::roots_valid_t(a, b, c, d, roots);
    let mut index = 0;
    while index < count {
        let calc_pt = cubic.point_at_t(roots[index]);
        if !calc_pt.x.approximately_equal(axis_intercept) {
            let mut extreme_ts = [0.0; 6];
            let extrema = cubic64::find_extrema(&cubic.as_f64_slice(), &mut extreme_ts);
            count = cubic.search_roots(
                extrema,
                axis_intercept,
                SearchAxis::X,
                &mut extreme_ts,
                roots,
            );
            break;
        }

        index += 1;
    }

    count
}
