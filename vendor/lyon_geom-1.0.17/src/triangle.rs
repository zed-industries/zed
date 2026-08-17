use crate::scalar::Scalar;
use crate::traits::Transformation;
use crate::LineSegment;
use crate::{point, Box2D, Point};

/// A 2D triangle defined by three points `a`, `b` and `c`.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Triangle<S> {
    pub a: Point<S>,
    pub b: Point<S>,
    pub c: Point<S>,
}

impl<S: Scalar> Triangle<S> {
    #[inline]
    fn get_barycentric_coords_for_point(&self, point: Point<S>) -> (S, S, S) {
        let v0 = self.b - self.a;
        let v1 = self.c - self.a;
        let v2 = point - self.a;
        let inv = S::ONE / v0.cross(v1);
        let a = v0.cross(v2) * inv;
        let b = v2.cross(v1) * inv;
        let c = S::ONE - a - b;

        (a, b, c)
    }

    pub fn contains_point(&self, point: Point<S>) -> bool {
        let coords = self.get_barycentric_coords_for_point(point);

        coords.0 > S::ZERO && coords.1 > S::ZERO && coords.2 > S::ZERO
    }

    /// Returns a conservative range of x that contains this triangle.
    #[inline]
    pub fn bounding_range_x(&self) -> (S, S) {
        let min_x = self.a.x.min(self.b.x).min(self.c.x);
        let max_x = self.a.x.max(self.b.x).max(self.c.x);

        (min_x, max_x)
    }

    /// Returns a conservative range of y that contains this triangle.
    #[inline]
    pub fn bounding_range_y(&self) -> (S, S) {
        let min_y = self.a.y.min(self.b.y).min(self.c.y);
        let max_y = self.a.y.max(self.b.y).max(self.c.y);

        (min_y, max_y)
    }

    /// Returns the smallest rectangle that contains this triangle.
    #[inline]
    pub fn bounding_box(&self) -> Box2D<S> {
        let (min_x, max_x) = self.bounding_range_x();
        let (min_y, max_y) = self.bounding_range_y();

        Box2D {
            min: point(min_x, min_y),
            max: point(max_x, max_y),
        }
    }

    #[inline]
    pub fn ab(&self) -> LineSegment<S> {
        LineSegment {
            from: self.a,
            to: self.b,
        }
    }

    #[inline]
    pub fn ba(&self) -> LineSegment<S> {
        LineSegment {
            from: self.b,
            to: self.a,
        }
    }

    #[inline]
    pub fn bc(&self) -> LineSegment<S> {
        LineSegment {
            from: self.b,
            to: self.c,
        }
    }

    #[inline]
    pub fn cb(&self) -> LineSegment<S> {
        LineSegment {
            from: self.c,
            to: self.b,
        }
    }

    #[inline]
    pub fn ca(&self) -> LineSegment<S> {
        LineSegment {
            from: self.c,
            to: self.a,
        }
    }

    #[inline]
    pub fn ac(&self) -> LineSegment<S> {
        LineSegment {
            from: self.a,
            to: self.c,
        }
    }

    /// [Not implemented] Applies the transform to this triangle and returns the results.
    #[inline]
    pub fn transform<T: Transformation<S>>(&self, transform: &T) -> Self {
        Triangle {
            a: transform.transform_point(self.a),
            b: transform.transform_point(self.b),
            c: transform.transform_point(self.c),
        }
    }

    /// Test for triangle-triangle intersection.
    pub fn intersects(&self, other: &Self) -> bool {
        // TODO: This should be optimized.
        // A bounding rect check should speed this up dramatically.
        // Inlining and reusing intermediate computation of the intersections
        // functions below and using SIMD would help too.
        self.ab().intersects(&other.ab())
            || self.ab().intersects(&other.bc())
            || self.ab().intersects(&other.ac())
            || self.bc().intersects(&other.ab())
            || self.bc().intersects(&other.bc())
            || self.bc().intersects(&other.ac())
            || self.ac().intersects(&other.ab())
            || self.ac().intersects(&other.bc())
            || self.ac().intersects(&other.ac())
            || self.contains_point(other.a)
            || other.contains_point(self.a)
            || *self == *other
    }

    /// Test for triangle-segment intersection.
    #[inline]
    pub fn intersects_line_segment(&self, segment: &LineSegment<S>) -> bool {
        self.ab().intersects(segment)
            || self.bc().intersects(segment)
            || self.ac().intersects(segment)
            || self.contains_point(segment.from)
    }
}

#[test]
fn test_triangle_contains() {
    assert!(Triangle {
        a: point(0.0, 0.0),
        b: point(1.0, 0.0),
        c: point(0.0, 1.0),
    }
    .contains_point(point(0.2, 0.2)));
    assert!(!Triangle {
        a: point(0.0, 0.0),
        b: point(1.0, 0.0),
        c: point(0.0, 1.0),
    }
    .contains_point(point(1.2, 0.2)));

    // Triangle vertex winding should not matter
    assert!(Triangle {
        a: point(1.0, 0.0),
        b: point(0.0, 0.0),
        c: point(0.0, 1.0),
    }
    .contains_point(point(0.2, 0.2)));

    // Point exactly on the edge counts as outside the triangle.
    assert!(!Triangle {
        a: point(0.0, 0.0),
        b: point(1.0, 0.0),
        c: point(0.0, 1.0),
    }
    .contains_point(point(0.0, 0.0)));
}

#[test]
fn test_segments() {
    let t = Triangle {
        a: point(1.0, 2.0),
        b: point(3.0, 4.0),
        c: point(5.0, 6.0),
    };

    assert_eq!(t.ab(), t.ba().flip());
    assert_eq!(t.ac(), t.ca().flip());
    assert_eq!(t.bc(), t.cb().flip());
}

#[test]
fn test_triangle_intersections() {
    let t1 = Triangle {
        a: point(1.0, 1.0),
        b: point(6.0, 1.0),
        c: point(3.0, 6.0),
    };

    let t2 = Triangle {
        a: point(2.0, 2.0),
        b: point(0.0, 3.0),
        c: point(1.0, 6.0),
    };

    assert!(t1.intersects(&t2));
    assert!(t2.intersects(&t1));

    // t3 and t1 have an overlapping edge, they are "touching" but not intersecting.
    let t3 = Triangle {
        a: point(6.0, 5.0),
        b: point(6.0, 1.0),
        c: point(3.0, 6.0),
    };

    assert!(!t1.intersects(&t3));
    assert!(!t3.intersects(&t1));

    // t4 is entirely inside t1.
    let t4 = Triangle {
        a: point(2.0, 2.0),
        b: point(5.0, 2.0),
        c: point(3.0, 4.0),
    };

    assert!(t1.intersects(&t4));
    assert!(t4.intersects(&t1));

    // Triangles intersect themselves.
    assert!(t1.intersects(&t1));
    assert!(t2.intersects(&t2));
    assert!(t3.intersects(&t3));
    assert!(t4.intersects(&t4));
}

#[test]
fn test_segment_intersection() {
    let tri = Triangle {
        a: point(1.0, 1.0),
        b: point(6.0, 1.0),
        c: point(3.0, 6.0),
    };

    let l1 = LineSegment {
        from: point(2.0, 0.0),
        to: point(3.0, 4.0),
    };

    assert!(tri.intersects_line_segment(&l1));

    let l2 = LineSegment {
        from: point(1.0, 3.0),
        to: point(0.0, 4.0),
    };

    assert!(!tri.intersects_line_segment(&l2));

    // The segment is entirely inside the triangle.
    let inside = LineSegment {
        from: point(2.0, 2.0),
        to: point(5.0, 2.0),
    };

    assert!(tri.intersects_line_segment(&inside));

    // A triangle does not intersect its own segments.
    assert!(!tri.intersects_line_segment(&tri.ab()));
    assert!(!tri.intersects_line_segment(&tri.bc()));
    assert!(!tri.intersects_line_segment(&tri.ac()));
}

#[test]
fn test_bounding_box() {
    let t1 = Triangle {
        a: point(10.0, 20.0),
        b: point(35.0, 40.0),
        c: point(50.0, 10.0),
    };
    let r1 = Box2D {
        min: point(10.0, 10.0),
        max: point(50.0, 40.0),
    };

    let t2 = Triangle {
        a: point(5.0, 30.0),
        b: point(25.0, 10.0),
        c: point(35.0, 40.0),
    };
    let r2 = Box2D {
        min: point(5.0, 10.0),
        max: point(35.0, 40.0),
    };

    let t3 = Triangle {
        a: point(1.0, 1.0),
        b: point(2.0, 5.0),
        c: point(0.0, 4.0),
    };
    let r3 = Box2D {
        min: point(0.0, 1.0),
        max: point(2.0, 5.0),
    };

    let cases = std::vec![(t1, r1), (t2, r2), (t3, r3)];
    for &(tri, r) in &cases {
        assert_eq!(tri.bounding_box(), r);
    }
}
