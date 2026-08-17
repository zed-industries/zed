// Copyright 2021 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Quadratic Bézier splines.
use crate::Point;

use crate::QuadBez;
use alloc::vec::Vec;

/// A quadratic Bézier spline in [B-spline](https://en.wikipedia.org/wiki/B-spline) format.
#[derive(Clone, Debug, PartialEq)]
pub struct QuadSpline(Vec<Point>);

impl QuadSpline {
    /// Construct a new `QuadSpline` from an array of [`Point`]s.
    #[inline]
    pub fn new(points: Vec<Point>) -> Self {
        Self(points)
    }

    /// Return the spline's control [`Point`]s.
    #[inline]
    pub fn points(&self) -> &[Point] {
        &self.0
    }

    /// Return an iterator over the implied [`QuadBez`] sequence.
    ///
    /// The returned quads are guaranteed to be G1 continuous.
    #[inline]
    pub fn to_quads(&self) -> impl Iterator<Item = QuadBez> + '_ {
        ToQuadBez {
            idx: 0,
            points: &self.0,
        }
    }
}

struct ToQuadBez<'a> {
    idx: usize,
    points: &'a Vec<Point>,
}

impl<'a> Iterator for ToQuadBez<'a> {
    type Item = QuadBez;

    fn next(&mut self) -> Option<Self::Item> {
        let [mut p0, p1, mut p2]: [Point; 3] =
            self.points.get(self.idx..=self.idx + 2)?.try_into().ok()?;

        if self.idx != 0 {
            p0 = p0.midpoint(p1);
        }
        if self.idx + 2 < self.points.len() - 1 {
            p2 = p1.midpoint(p2);
        }

        self.idx += 1;

        Some(QuadBez { p0, p1, p2 })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, QuadBez, QuadSpline};

    #[test]
    pub fn no_points_no_quads() {
        assert!(QuadSpline::new(Vec::new()).to_quads().next().is_none());
    }

    #[test]
    pub fn one_point_no_quads() {
        assert!(QuadSpline::new(vec![Point::new(1.0, 1.0)])
            .to_quads()
            .next()
            .is_none());
    }

    #[test]
    pub fn two_points_no_quads() {
        assert!(
            QuadSpline::new(vec![Point::new(1.0, 1.0), Point::new(1.0, 1.0)])
                .to_quads()
                .next()
                .is_none()
        );
    }

    #[test]
    pub fn three_points_same_quad() {
        let p0 = Point::new(1.0, 1.0);
        let p1 = Point::new(2.0, 2.0);
        let p2 = Point::new(3.0, 3.0);
        assert_eq!(
            vec![QuadBez { p0, p1, p2 }],
            QuadSpline::new(vec![p0, p1, p2])
                .to_quads()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    pub fn four_points_implicit_on_curve() {
        let p0 = Point::new(1.0, 1.0);
        let p1 = Point::new(3.0, 3.0);
        let p2 = Point::new(5.0, 5.0);
        let p3 = Point::new(8.0, 8.0);
        assert_eq!(
            vec![
                QuadBez {
                    p0,
                    p1,
                    p2: p1.midpoint(p2)
                },
                QuadBez {
                    p0: p1.midpoint(p2),
                    p1: p2,
                    p2: p3
                }
            ],
            QuadSpline::new(vec![p0, p1, p2, p3])
                .to_quads()
                .collect::<Vec<_>>()
        );
    }
}
