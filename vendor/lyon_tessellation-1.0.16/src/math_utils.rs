//! Various math tools that are mostly useful for the tessellators.

use crate::math::*;

#[cfg(not(feature = "std"))]
use num_traits::Float;

/// Compute a normal vector at a point P such that ```x ---e1----> P ---e2---> x```
///
/// The resulting vector is not normalized. The length is such that extruding the shape
/// would yield parallel segments exactly 1 unit away from their original. (useful
/// for generating strokes and vertex-aa).
/// The normal points towards the positive side of e1.
///
/// v1 and v2 are expected to be normalized.
pub fn compute_normal(v1: Vector, v2: Vector) -> Vector {
    //debug_assert!((v1.length() - 1.0).abs() < 0.001, "v1 should be normalized ({})", v1.length());
    //debug_assert!((v2.length() - 1.0).abs() < 0.001, "v2 should be normalized ({})", v2.length());

    let epsilon = 1e-4;

    let n1 = vector(-v1.y, v1.x);

    let v12 = v1 + v2;

    if v12.square_length() < epsilon {
        return vector(0.0, 0.0);
    }

    let tangent = v12.normalize();
    let n = vector(-tangent.y, tangent.x);

    let inv_len = n.dot(n1);

    if inv_len.abs() < epsilon {
        return n1;
    }

    n / inv_len
}

#[test]
fn test_compute_normal() {
    fn assert_almost_eq(a: Vector, b: Vector) {
        if (a - b).square_length() > 0.00001 {
            panic!("assert almost equal: {:?} != {:?}", a, b);
        }
    }

    assert_almost_eq(
        compute_normal(vector(1.0, 0.0), vector(0.0, 1.0)),
        vector(-1.0, 1.0),
    );
    assert_almost_eq(
        compute_normal(vector(1.0, 0.0), vector(0.0, -1.0)),
        vector(1.0, 1.0),
    );
    assert_almost_eq(
        compute_normal(vector(1.0, 0.0), vector(1.0, 0.0)),
        vector(0.0, 1.0),
    );
}
