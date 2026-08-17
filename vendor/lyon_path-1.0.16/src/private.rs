#![allow(clippy::too_many_arguments)]

// This module contains a few helpers that should not be considered as part of the public API,
// but are exposed for use by other lyon crates.
// Changing them doesn't necessarily imply semver breaking bumps.

pub use crate::geom::{CubicBezierSegment, QuadraticBezierSegment};
pub use crate::math::Point;
pub use crate::traits::PathBuilder;
pub use crate::{Attributes, EndpointId};

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct DebugValidator {
    #[cfg(debug_assertions)]
    in_subpath: bool,
}

impl DebugValidator {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn begin(&mut self) {
        #[cfg(debug_assertions)]
        {
            assert!(!self.in_subpath, "multiple begin() calls without end()");
            self.in_subpath = true;
        }
    }

    #[inline(always)]
    pub fn end(&mut self) {
        #[cfg(debug_assertions)]
        {
            assert!(self.in_subpath, "end() called without begin()");
            self.in_subpath = false;
        }
    }

    #[inline(always)]
    pub fn edge(&self) {
        #[cfg(debug_assertions)]
        assert!(self.in_subpath, "edge operation is made before begin()");
    }

    /// TODO: this should consume `self` to ensure it is dropped after this call
    /// TODO: also, DebugValidator probably should not be exposed in the public API.
    #[inline(always)]
    pub fn build(&self) {
        #[cfg(debug_assertions)]
        assert!(!self.in_subpath, "build() called before end()");
    }
}

pub fn flatten_quadratic_bezier(
    tolerance: f32,
    from: Point,
    ctrl: Point,
    to: Point,
    attributes: Attributes,
    prev_attributes: Attributes,
    builder: &mut impl PathBuilder,
    buffer: &mut [f32],
) -> EndpointId {
    let curve = QuadraticBezierSegment { from, ctrl, to };
    let n = attributes.len();
    let mut id = EndpointId::INVALID;
    curve.for_each_flattened_with_t(tolerance, &mut |line, t| {
        let attr = if t.end == 1.0 {
            attributes
        } else {
            for i in 0..n {
                buffer[i] = prev_attributes[i] * (1.0 - t.end) + attributes[i] * t.end;
            }
            // BUG: https://github.com/rust-lang/rust-clippy/issues/10608
            #[allow(clippy::redundant_slicing)]
            &buffer[..]
        };
        id = builder.line_to(line.to, attr);
    });

    id
}

pub fn flatten_cubic_bezier(
    tolerance: f32,
    from: Point,
    ctrl1: Point,
    ctrl2: Point,
    to: Point,
    attributes: Attributes,
    prev_attributes: Attributes,
    builder: &mut impl PathBuilder,
    buffer: &mut [f32],
) -> EndpointId {
    let curve = CubicBezierSegment {
        from,
        ctrl1,
        ctrl2,
        to,
    };
    let n = attributes.len();
    let mut id = EndpointId::INVALID;
    curve.for_each_flattened_with_t(tolerance, &mut |line, t| {
        let attr = if t.end == 1.0 {
            attributes
        } else {
            for i in 0..n {
                buffer[i] = prev_attributes[i] * (1.0 - t.end) + attributes[i] * t.end;
            }
            // BUG: https://github.com/rust-lang/rust-clippy/issues/10608
            #[allow(clippy::redundant_slicing)]
            &buffer[..]
        };
        id = builder.line_to(line.to, attr);
    });

    id
}
