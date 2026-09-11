//! Conversions between this crate's geometry and [`lyon`] path geometry.
//!
//! These conversions are gated behind the `lyon` feature so that consumers of
//! the core primitives do not pull in a rendering dependency. They live here,
//! rather than next to the path builder that uses them, because Rust's orphan
//! rules only allow `From` impls bridging two foreign types to be defined in the
//! crate that owns one of them.

use etagere::euclid::Point2D;
use lyon::math::{Vector, vector};

use crate::{Pixels, Point, point, px};

impl From<lyon::math::Point> for Point<Pixels> {
    fn from(p: lyon::math::Point) -> Self {
        point(px(p.x), px(p.y))
    }
}

impl From<Point<Pixels>> for lyon::math::Point {
    fn from(p: Point<Pixels>) -> Self {
        lyon::math::point(p.x.0, p.y.0)
    }
}

impl From<Point<Pixels>> for Vector {
    fn from(p: Point<Pixels>) -> Self {
        vector(p.x.0, p.y.0)
    }
}

impl From<Point<Pixels>> for Point2D<f32, Pixels> {
    fn from(p: Point<Pixels>) -> Self {
        Point2D::new(p.x.0, p.y.0)
    }
}
