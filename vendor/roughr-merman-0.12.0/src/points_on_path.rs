use std::fmt::Display;
use std::ops::MulAssign;

use euclid::default::Point2D;
use euclid::Trig;
use num_traits::{Float, FromPrimitive};
use points_on_curve::{points_on_bezier_curves, simplify};
use svg_path_ops::{absolutize, normalize};
use svgtypes::{PathParser, PathSegment};

use crate::core::{_c, _cc};

pub fn points_on_path<F>(
    path: String,
    tolerance: Option<F>,
    distance: Option<F>,
) -> Vec<Vec<Point2D<F>>>
where
    F: FromPrimitive + Trig + Float + MulAssign + Display,
{
    let path_parser = PathParser::from(path.as_ref());
    let path_segments: Vec<PathSegment> = path_parser.flatten().collect();
    let normalized_segments = normalize(absolutize(path_segments.iter()));

    generate_points(tolerance, distance, normalized_segments)
}

pub fn points_on_segments<F>(
    path_segments: Vec<PathSegment>,
    tolerance: Option<F>,
    distance: Option<F>,
) -> Vec<Vec<Point2D<F>>>
where
    F: FromPrimitive + Trig + Float + MulAssign + Display,
{
    let normalized_segments = normalize(absolutize(path_segments.iter()));
    generate_points(tolerance, distance, normalized_segments)
}

pub fn normalized_segments(path_segments: &[PathSegment]) -> Vec<PathSegment> {
    normalize(absolutize(path_segments.iter())).collect()
}

pub fn points_on_normalized_segments<F>(
    normalized_segments: &[PathSegment],
    tolerance: Option<F>,
    distance: Option<F>,
) -> Vec<Vec<Point2D<F>>>
where
    F: FromPrimitive + Trig + Float + MulAssign + Display,
{
    generate_points(tolerance, distance, normalized_segments.iter().cloned())
}

fn generate_points<F>(
    tolerance: Option<F>,
    distance: Option<F>,
    normalized_segments: impl Iterator<Item = PathSegment>,
) -> Vec<Vec<euclid::Point2D<F, euclid::UnknownUnit>>>
where
    F: FromPrimitive + Trig + Float + MulAssign + Display,
{
    let mut sets: Vec<Vec<Point2D<F>>> = vec![];
    let mut current_points: Vec<Point2D<F>> = vec![];
    let mut start = Point2D::new(_c::<F>(0.0), _c::<F>(0.0));
    let mut pending_curve: Vec<Point2D<F>> = vec![];

    let append_pending_curve =
        |current_points: &mut Vec<Point2D<F>>, pending_curve: &mut Vec<Point2D<F>>| {
            if pending_curve.len() >= 4 {
                current_points.append(&mut points_on_bezier_curves(
                    &pending_curve[..],
                    tolerance.unwrap_or(_c(0.0)),
                    None,
                ));
            }
            pending_curve.clear();
        };

    let mut append_pending_points =
        |current_points: &mut Vec<Point2D<F>>, pending_curve: &mut Vec<Point2D<F>>| {
            {
                append_pending_curve(current_points, pending_curve);
            }
            if !current_points.is_empty() {
                sets.push(current_points.clone());
                current_points.clear();
            }
        };

    for segment in normalized_segments {
        match segment {
            PathSegment::MoveTo { abs: true, x, y } => {
                append_pending_points(&mut current_points, &mut pending_curve);
                start = Point2D::new(_cc::<F>(x), _cc::<F>(y));
                current_points.push(start);
            }
            PathSegment::LineTo { abs: true, x, y } => {
                append_pending_curve(&mut current_points, &mut pending_curve);
                current_points.push(Point2D::new(_cc::<F>(x), _cc::<F>(y)));
            }
            PathSegment::CurveTo {
                abs: true,
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                if pending_curve.is_empty() {
                    let last_point = if !current_points.is_empty() {
                        current_points.last().unwrap()
                    } else {
                        &start
                    };
                    pending_curve.push(*last_point);
                }
                pending_curve.push(Point2D::new(_cc::<F>(x1), _cc::<F>(y1)));
                pending_curve.push(Point2D::new(_cc::<F>(x2), _cc::<F>(y2)));
                pending_curve.push(Point2D::new(_cc::<F>(x), _cc::<F>(y)));
            }
            PathSegment::ClosePath { abs: true } => {
                append_pending_curve(&mut current_points, &mut pending_curve);
                current_points.push(start);
            }
            _ => panic!("unexpected  path segment"),
        }
    }

    append_pending_points(&mut current_points, &mut pending_curve);

    if let Some(dst) = distance {
        let mut out = vec![];
        for set in sets.iter() {
            let simplified_set = simplify(set, dst);
            if !simplified_set.is_empty() {
                out.push(simplified_set);
            }
        }
        out
    } else {
        sets
    }
}
