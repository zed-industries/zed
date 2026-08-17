use crate::scalar::Scalar;
use crate::{LineSegment, Point, Vector};

use core::ops::Range;

/// Common APIs to segment types.
pub trait Segment: Copy + Sized {
    type Scalar: Scalar;

    /// Start of the curve.
    fn from(&self) -> Point<Self::Scalar>;

    /// End of the curve.
    fn to(&self) -> Point<Self::Scalar>;

    /// Sample the curve at t (expecting t between 0 and 1).
    fn sample(&self, t: Self::Scalar) -> Point<Self::Scalar>;

    /// Sample x at t (expecting t between 0 and 1).
    fn x(&self, t: Self::Scalar) -> Self::Scalar {
        self.sample(t).x
    }

    /// Sample y at t (expecting t between 0 and 1).
    fn y(&self, t: Self::Scalar) -> Self::Scalar {
        self.sample(t).y
    }

    /// Sample the derivative at t (expecting t between 0 and 1).
    fn derivative(&self, t: Self::Scalar) -> Vector<Self::Scalar>;

    /// Sample x derivative at t (expecting t between 0 and 1).
    fn dx(&self, t: Self::Scalar) -> Self::Scalar {
        self.derivative(t).x
    }

    /// Sample y derivative at t (expecting t between 0 and 1).
    fn dy(&self, t: Self::Scalar) -> Self::Scalar {
        self.derivative(t).y
    }

    /// Split this curve into two sub-curves.
    fn split(&self, t: Self::Scalar) -> (Self, Self);

    /// Return the curve before the split point.
    fn before_split(&self, t: Self::Scalar) -> Self;

    /// Return the curve after the split point.
    fn after_split(&self, t: Self::Scalar) -> Self;

    /// Return the curve inside a given range of t.
    ///
    /// This is equivalent splitting at the range's end points.
    fn split_range(&self, t_range: Range<Self::Scalar>) -> Self;

    /// Swap the direction of the segment.
    fn flip(&self) -> Self;

    /// Compute the length of the segment using a flattened approximation.
    fn approximate_length(&self, tolerance: Self::Scalar) -> Self::Scalar;

    /// Approximates the curve with sequence of line segments.
    ///
    /// The `tolerance` parameter defines the maximum distance between the curve and
    /// its approximation.
    ///
    /// The parameter `t` at the final segment is guaranteed to be equal to `1.0`.
    #[allow(clippy::type_complexity)]
    fn for_each_flattened_with_t(
        &self,
        tolerance: Self::Scalar,
        callback: &mut dyn FnMut(&LineSegment<Self::Scalar>, Range<Self::Scalar>),
    );
}

macro_rules! impl_segment {
    ($S:ty) => {
        type Scalar = $S;
        fn from(&self) -> Point<$S> {
            self.from()
        }
        fn to(&self) -> Point<$S> {
            self.to()
        }
        fn sample(&self, t: $S) -> Point<$S> {
            self.sample(t)
        }
        fn x(&self, t: $S) -> $S {
            self.x(t)
        }
        fn y(&self, t: $S) -> $S {
            self.y(t)
        }
        fn derivative(&self, t: $S) -> Vector<$S> {
            self.derivative(t)
        }
        fn dx(&self, t: $S) -> $S {
            self.dx(t)
        }
        fn dy(&self, t: $S) -> $S {
            self.dy(t)
        }
        fn split(&self, t: $S) -> (Self, Self) {
            self.split(t)
        }
        fn before_split(&self, t: $S) -> Self {
            self.before_split(t)
        }
        fn after_split(&self, t: $S) -> Self {
            self.after_split(t)
        }
        fn split_range(&self, t_range: Range<$S>) -> Self {
            self.split_range(t_range)
        }
        fn flip(&self) -> Self {
            self.flip()
        }
        fn approximate_length(&self, tolerance: $S) -> $S {
            self.approximate_length(tolerance)
        }
    };
}
