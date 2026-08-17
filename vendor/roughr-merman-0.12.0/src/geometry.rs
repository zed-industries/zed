use euclid::default::Point2D;
use euclid::{Angle, Translation2D, Trig, Vector2D};
use num_traits::{Float, FromPrimitive};

use crate::core::_c;

#[derive(Clone, Debug, PartialEq)]
pub struct Line<F: Float + Trig> {
    pub start_point: Point2D<F>,
    pub end_point: Point2D<F>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BezierQuadratic<F: Float + Trig> {
    pub start: Point2D<F>,
    pub cp: Point2D<F>,
    pub end: Point2D<F>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BezierCubic<F: Float + Trig> {
    pub start: Point2D<F>,
    pub cp1: Point2D<F>,
    pub cp2: Point2D<F>,
    pub end: Point2D<F>,
}

impl<F: Float + Trig> Line<F> {
    pub fn from(points: &[Point2D<F>]) -> Self {
        Line {
            start_point: points[0],
            end_point: points[1],
        }
    }
    pub fn as_points(&self) -> Vec<Point2D<F>> {
        vec![self.start_point, self.end_point]
    }

    pub fn length(&self) -> F {
        (self.end_point - self.start_point).length()
    }

    pub fn rotate(&mut self, center: &Point2D<F>, degrees: F) {
        let angle = Angle::radians(degrees.to_radians());
        let translation = Translation2D::new(-center.x, -center.y);
        let transformation = translation
            .to_transform()
            .then_rotate(angle)
            .then_translate(Vector2D::new(center.x, center.y));
        self.start_point = transformation.transform_point(self.start_point);
        self.end_point = transformation.transform_point(self.end_point);
    }
}

pub fn rotate_points<F: Float + Trig>(
    points: &[Point2D<F>],
    center: &Point2D<F>,
    degrees: F,
) -> Vec<Point2D<F>> {
    let angle = Angle::radians(degrees.to_radians());
    let translation = Translation2D::new(-center.x, -center.y);
    let transformation = translation
        .to_transform()
        .then_rotate(angle)
        .then_translate(Vector2D::new(center.x, center.y));
    points
        .iter()
        .map(|&p| transformation.transform_point(p))
        .collect::<Vec<Point2D<F>>>()
}

pub fn rotate_lines<F: Float + Trig>(
    lines: &[Line<F>],
    center: &Point2D<F>,
    degrees: F,
) -> Vec<Line<F>> {
    lines
        .iter()
        .cloned()
        .map(|mut l| {
            l.rotate(center, degrees);
            l
        })
        .collect::<Vec<Line<F>>>()
}

/// Raises the order from a quadratic bezier to a cubic bezier curve.
pub fn convert_bezier_quadratic_to_cubic<F: Float + FromPrimitive + Trig>(
    bezier_quadratic: BezierQuadratic<F>,
) -> BezierCubic<F> {
    let cubic_x1 = bezier_quadratic.start.x
        + _c::<F>(2.0 / 3.0) * (bezier_quadratic.cp.x - bezier_quadratic.start.x);
    let cubic_y1 = bezier_quadratic.start.y
        + _c::<F>(2.0 / 3.0) * (bezier_quadratic.cp.y - bezier_quadratic.start.y);
    let cubic_x2 = bezier_quadratic.end.x
        + _c::<F>(2.0 / 3.0) * (bezier_quadratic.cp.x - bezier_quadratic.end.x);
    let cubic_y2 = bezier_quadratic.end.y
        + _c::<F>(2.0 / 3.0) * (bezier_quadratic.cp.y - bezier_quadratic.end.y);

    BezierCubic {
        start: bezier_quadratic.start,
        cp1: Point2D::new(cubic_x1, cubic_y1),
        cp2: Point2D::new(cubic_x2, cubic_y2),
        end: bezier_quadratic.end,
    }
}

#[cfg(test)]
mod tests {
    use euclid::default::Point2D;
    #[test]
    fn line_length() {
        let l = super::Line::from(&[Point2D::new(1.0, 1.0), Point2D::new(2.0, 2.0)]);
        assert_eq!(l.length(), f32::sqrt(2.0));
    }
}
