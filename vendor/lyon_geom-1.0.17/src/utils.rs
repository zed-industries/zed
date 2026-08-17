use crate::scalar::{Float, Scalar};
use crate::{vector, Point, Vector};
use arrayvec::ArrayVec;

#[inline]
pub fn min_max<S: Float>(a: S, b: S) -> (S, S) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

#[inline]
pub fn tangent<S: Float>(v: Vector<S>) -> Vector<S> {
    vector(-v.y, v.x)
}

#[inline]
pub fn normalized_tangent<S: Scalar>(v: Vector<S>) -> Vector<S> {
    tangent(v).normalize()
}

/// Angle between vectors v1 and v2 (oriented clockwise assuming y points downwards).
/// The result is a number between `0` and `2 * PI`.
///
/// ex: `directed_angle([0,1], [1,0]) = 3/2 Pi rad`
///
/// ```text
///     x       __
///   0-->     /  \
///  y|       |  x--> v2
///   v        \ |v1
///              v
/// ```
///
/// Or, assuming y points upwards:
/// `directed_angle([0,-1], [1,0]) = 1/2 Pi rad`
///
/// ```text
///   ^           v2
///  y|          x-->
///   0-->    v1 | /
///     x        v-
/// ```
///
#[inline]
pub fn directed_angle<S: Scalar>(v1: Vector<S>, v2: Vector<S>) -> S {
    let angle = S::fast_atan2(v2.y, v2.x) - S::fast_atan2(v1.y, v1.x);

    if angle < S::ZERO {
        angle + S::TWO * S::PI()
    } else {
        angle
    }
}

pub fn directed_angle2<S: Scalar>(center: Point<S>, a: Point<S>, b: Point<S>) -> S {
    directed_angle(a - center, b - center)
}

pub fn cubic_polynomial_roots<S: Scalar>(a: S, b: S, c: S, d: S) -> ArrayVec<S, 3> {
    let mut result = ArrayVec::new();

    let m = a.abs().max(b.abs()).max(c.abs()).max(d.abs());
    let epsilon = S::epsilon_for(m);

    if S::abs(a) < epsilon {
        if S::abs(b) < epsilon {
            if S::abs(c) < epsilon {
                return result;
            }
            // linear equation
            result.push(-d / c);
            return result;
        }
        // quadratic equation
        let delta = c * c - S::FOUR * b * d;
        if delta > S::ZERO {
            let sqrt_delta = S::sqrt(delta);
            result.push((-c - sqrt_delta) / (S::TWO * b));
            result.push((-c + sqrt_delta) / (S::TWO * b));
        } else if S::abs(delta) < epsilon {
            result.push(-c / (S::TWO * b));
        }
        return result;
    }

    let frac_1_3 = S::ONE / S::THREE;

    let bn = b / a;
    let cn = c / a;
    let dn = d / a;

    let delta0 = (S::THREE * cn - bn * bn) / S::NINE;
    let delta1 = (S::NINE * bn * cn - S::value(27.0) * dn - S::TWO * bn * bn * bn) / S::value(54.0);
    let delta_01 = delta0 * delta0 * delta0 + delta1 * delta1;

    if delta_01 >= S::ZERO {
        let delta_p_sqrt = delta1 + S::sqrt(delta_01);
        let delta_m_sqrt = delta1 - S::sqrt(delta_01);

        let s = delta_p_sqrt.signum() * S::abs(delta_p_sqrt).powf(frac_1_3);
        let t = delta_m_sqrt.signum() * S::abs(delta_m_sqrt).powf(frac_1_3);

        result.push(-bn * frac_1_3 + (s + t));

        // Don't add the repeated root when s + t == 0.
        if S::abs(s - t) < epsilon && S::abs(s + t) >= epsilon {
            result.push(-bn * frac_1_3 - (s + t) / S::TWO);
        }
    } else {
        let theta = S::acos(delta1 / S::sqrt(-delta0 * delta0 * delta0));
        let two_sqrt_delta0 = S::TWO * S::sqrt(-delta0);
        result.push(two_sqrt_delta0 * Float::cos(theta * frac_1_3) - bn * frac_1_3);
        result.push(
            two_sqrt_delta0 * Float::cos((theta + S::TWO * S::PI()) * frac_1_3) - bn * frac_1_3,
        );
        result.push(
            two_sqrt_delta0 * Float::cos((theta + S::FOUR * S::PI()) * frac_1_3) - bn * frac_1_3,
        );
    }

    //result.sort();

    result
}

#[test]
fn cubic_polynomial() {
    fn assert_approx_eq(a: ArrayVec<f32, 3>, b: &[f32], epsilon: f32) {
        for i in 0..a.len() {
            if f32::abs(a[i] - b[i]) > epsilon {
                std::println!("{a:?} != {b:?}");
            }
            assert!((a[i] - b[i]).abs() <= epsilon);
        }
        assert_eq!(a.len(), b.len());
    }

    assert_approx_eq(
        cubic_polynomial_roots(2.0, -4.0, 2.0, 0.0),
        &[0.0, 1.0],
        0.0000001,
    );
    assert_approx_eq(
        cubic_polynomial_roots(-1.0, 1.0, -1.0, 1.0),
        &[1.0],
        0.000001,
    );
    assert_approx_eq(
        cubic_polynomial_roots(-2.0, 2.0, -1.0, 10.0),
        &[2.0],
        0.00005,
    );
    // (x - 1)^3, with a triple root, should only return one root.
    assert_approx_eq(
        cubic_polynomial_roots(1.0, -3.0, 3.0, -1.0),
        &[1.0],
        0.00005,
    );

    // Quadratics.
    assert_approx_eq(
        cubic_polynomial_roots(0.0, 1.0, -5.0, -14.0),
        &[-2.0, 7.0],
        0.00005,
    );
    // (x - 3)^2, with a double root, should only return one root.
    assert_approx_eq(cubic_polynomial_roots(0.0, 1.0, -6.0, 9.0), &[3.0], 0.00005);

    // Linear.
    assert_approx_eq(cubic_polynomial_roots(0.0, 0.0, 2.0, 1.0), &[-0.5], 0.00005);

    // Constant.
    assert_approx_eq(cubic_polynomial_roots(0.0, 0.0, 0.0, 0.0), &[], 0.00005);
}
