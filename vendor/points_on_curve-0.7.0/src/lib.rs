#![forbid(unsafe_code)]
#![deny(unreachable_pub)]
#![deny(missing_docs)]

//!
//! This crate is a rustlang port of [points-on-curve](https://github.com/pshihn/bezier-points) npm package written by
//! [@pshihn](https://github.com/pshihn).
//!
//! This package exposes functions to sample points on a bezier curve with certain tolerance.
//! There is also a utility funtion to simplify the shape to use fewer points.
//! This can really be useful when estimating lines/polygons for curves in WebGL or for Hit/Collision detections.
//! Reverse of this operation is also supported meaning given some points generate bezier curve points passing through this points
//!
//!
//! ## 📦 Cargo.toml
//!
//! ```toml
//! [dependencies]
//! points_on_curve = "0.1"
//! ```
//!
//! ## 🔧 Example
//!
//! ```rust
//! use euclid::{default, point2};
//! use points_on_curve::points_on_bezier_curves;
//!
//! let input = vec![
//!     point2(70.0, 240.0),
//!     point2(145.0, 60.0),
//!     point2(275.0, 90.0),
//!     point2(300.0, 230.0),
//! ];
//! let result_015 = points_on_bezier_curves(&input, 0.2, Some(0.15));
//! ```
//!
//!
//! ## 🖨️ Output
//! This picture shows computed points with 4 different distance values 0.15, 0.75, 1.5 and 3.0 with tolerance 2.0.

//! ![tolerance](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/points_on_curve/assets/tolerance.png)
//!
//! ## Details
//!
//! ## 🔭 Examples
//!
//! For more examples have a look at the
//! [examples](https://github.com/orhanbalci/rough-rs/blob/main/points_on_curve/examples) folder.

use std::borrow::Borrow;
use std::cmp::{max_by, min_by};
use std::fmt::Display;
use std::ops::MulAssign;

pub use euclid;
use euclid::default::Point2D;
use euclid::point2;
use num_traits::Float;

/// computes distance squared from a point p to the line segment vw
///
/// # examples
/// ```
/// use euclid::point2;
/// use points_on_curve::distance_to_segment_squared;
/// let expected = 1.0;
/// let result = distance_to_segment_squared(point2(0.0, 1.0), point2(-1.0, 0.0), point2(1.0, 0.0));
/// assert_eq!(expected, result);
/// ```
pub fn distance_to_segment_squared<F, P>(p: P, v: P, w: P) -> F
where
    F: Float + PartialOrd + Display,
    P: Borrow<Point2D<F>>,
{
    let v_ = v.borrow();
    let w_ = w.borrow();
    let p_ = p.borrow();
    let l2 = v_.distance_to(*w_).powi(2);
    if l2 == F::zero() {
        p_.distance_to(*v_).powi(2)
    } else {
        let mut t = ((p_.x - v_.x) * (w_.x - v_.x) + (p_.y - v_.y) * (w_.y - v_.y)) / l2;
        t = max_by(
            F::zero(),
            min_by(F::one(), t, |a, b| {
                a.partial_cmp(b)
                    .unwrap_or_else(|| panic!("can not compare {} and {}", a, b))
            }),
            |a, b| {
                a.partial_cmp(b)
                    .unwrap_or_else(|| panic!("can not compare {} and {}", a, b))
            },
        );
        p_.distance_to(v_.lerp(*w_, t)).powi(2)
    }
}

/// Adapted from https://seant23.wordpress.com/2010/11/12/offset-bezier-curves/
pub fn flatness<F>(points: &[Point2D<F>], offset: usize) -> F
where
    F: Float + MulAssign,
{
    let p1 = points[offset];
    let p2 = points[offset + 1];
    let p3 = points[offset + 2];
    let p4 = points[offset + 3];

    let const_3 = F::from(3).unwrap();
    let const_2 = F::from(2).unwrap();

    let mut ux = const_3 * p2.x - const_2 * p1.x - p4.x;
    ux *= ux;
    let mut uy = const_3 * p2.y - const_2 * p1.y - p4.y;
    uy *= uy;
    let mut vx = const_3 * p3.x - const_2 * p4.x - p1.x;
    vx *= vx;
    let mut vy = const_3 * p3.y - const_2 * p4.y - p1.y;
    vy *= vy;
    if ux < vx {
        ux = vx;
    }
    if uy < vy {
        uy = vy;
    }
    ux + uy
}

fn simplify_points<F>(
    points: &[Point2D<F>],
    start: usize,
    end: usize,
    epsilon: F,
    new_points: &mut Vec<Point2D<F>>,
) -> Vec<Point2D<F>>
where
    F: Float + Display,
{
    let s = points[start];
    let e = points[end - 1];
    let mut max_dist_sq = F::zero();
    let mut max_ndx = 0;
    for p in points.iter().enumerate().take(end - 1).skip(start + 1) {
        let distance_sq = distance_to_segment_squared(*p.1, s, e);
        if distance_sq > max_dist_sq {
            max_dist_sq = distance_sq;
            max_ndx = p.0;
        }
    }

    if max_dist_sq.sqrt() > epsilon {
        simplify_points(points, start, max_ndx + 1, epsilon, new_points);
        simplify_points(points, max_ndx, end, epsilon, new_points);
    } else {
        if new_points.is_empty() {
            new_points.push(s);
        }
        new_points.push(e);
    }

    new_points.to_vec()
}

/// Simplifies given points on curve by reducing number of points using Ramer–Douglas–Peucker algorithm
/// https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm
pub fn simplify<F>(points: &[Point2D<F>], distance: F) -> Vec<Point2D<F>>
where
    F: Float + Display,
{
    simplify_points(points, 0, points.len(), distance, &mut vec![])
}

fn get_points_on_bezier_curve_with_splitting<F>(
    points: &[Point2D<F>],
    offset: usize,
    tolerance: F,
    new_points: &mut Vec<Point2D<F>>,
) -> Vec<Point2D<F>>
where
    F: Float + MulAssign,
{
    if flatness(points, offset) < tolerance {
        let p0 = points[offset];
        if !new_points.is_empty() {
            let d = new_points.last().unwrap().distance_to(p0);
            if d > F::one() {
                new_points.push(p0);
            }
        } else {
            new_points.push(p0);
        }
        new_points.push(points[offset + 3]);
    } else {
        let t = F::from(0.5).unwrap();
        let p1 = points[offset];
        let p2 = points[offset + 1];
        let p3 = points[offset + 2];
        let p4 = points[offset + 3];

        let q1 = p1.lerp(p2, t);
        let q2 = p2.lerp(p3, t);
        let q3 = p3.lerp(p4, t);

        let r1 = q1.lerp(q2, t);
        let r2 = q2.lerp(q3, t);

        let red = r1.lerp(r2, t);

        get_points_on_bezier_curve_with_splitting(&[p1, q1, r1, red], 0, tolerance, new_points);
        get_points_on_bezier_curve_with_splitting(&[red, r2, q3, p4], 0, tolerance, new_points);
    }

    new_points.to_vec()
}

/// Samples points on a Bezier Curve. If distance parameter is given does simplification on sampled points
/// and reduces number of points that represents given Bezier Curve.
pub fn points_on_bezier_curves<F>(
    points: &[Point2D<F>],
    tolerance: F,
    distance: Option<F>,
) -> Vec<Point2D<F>>
where
    F: Float + MulAssign + Display,
{
    let mut new_points = vec![];
    let num_segments = points.len() / 3;
    for i in 0..num_segments {
        let offset = i * 3;
        get_points_on_bezier_curve_with_splitting(points, offset, tolerance, &mut new_points);
    }

    if let Some(dst) = distance {
        if dst > F::zero() {
            return simplify_points(&new_points, 0, new_points.len(), dst, &mut vec![]);
        }
    }
    new_points
}

/// Generates Bezier Curve parameters passing through given points
pub fn curve_to_bezier<F>(points_in: &[Point2D<F>], curve_tightness: F) -> Option<Vec<Point2D<F>>>
where
    F: Float,
{
    if points_in.len() < 3 {
        None
    } else {
        let mut out = vec![];
        if points_in.len() == 3 {
            let mut points_updated = vec![];
            points_updated.extend_from_slice(points_in);
            points_updated.push(*points_in.last().unwrap());
            out = curve_to_bezier(&points_updated, curve_tightness).unwrap();
        } else {
            let mut points = vec![];
            points.push(points_in[0]);
            points.push(points_in[0]);
            for i in 1..points_in.len() {
                points.push(points_in[i]);
                if i == points_in.len() - 1 {
                    points.push(points_in[i]);
                }
            }
            let s = F::one() - curve_tightness;
            out.push(points[0]);
            for i in 1..points.len() - 2 {
                let cached_point = points[i];
                //let b_0  = cached_point.clone();
                let b_1 = point2(
                    cached_point.x
                        + (s * points[i + 1].x - s * points[i - 1].x) / F::from(6).unwrap(),
                    cached_point.y
                        + (s * points[i + 1].y - s * points[i - 1].y) / F::from(6).unwrap(),
                );
                let b_2 = point2(
                    points[i + 1].x + (s * points[i].x - s * points[i + 2].x) / F::from(6).unwrap(),
                    points[i + 1].y + (s * points[i].y - s * points[i + 2].y) / F::from(6).unwrap(),
                );
                let b_3 = point2(points[i + 1].x, points[i + 1].y);
                out.push(b_1);
                out.push(b_2);
                out.push(b_3);
            }
        }
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use euclid::point2;

    #[test]
    fn distance_to_segment_squared() {
        let expected = 1.0;
        let result = super::distance_to_segment_squared(
            point2(0.0, 1.0),
            point2(-1.0, 0.0),
            point2(1.0, 0.0),
        );
        assert_eq!(expected, result);
    }

    #[test]
    fn flatness() {
        let expected = 9.0;
        let result = super::flatness(
            &[
                point2(0.0, 1.0),
                point2(1.0, 3.0),
                point2(2.0, 3.0),
                point2(3.0, 4.0),
            ],
            0,
        );
        assert_eq!(expected, result);
    }

    #[test]
    fn points_on_bezier_curves() {
        let expected = vec![
            point2(70.0, 240.0),
            point2(73.5552978515625, 231.71592712402344),
            point2(77.1875, 223.7371826171875),
            point2(80.8929443359375, 216.0614776611328),
            point2(84.66796875, 208.6865234375),
            point2(88.5089111328125, 201.6100311279297),
            point2(92.412109375, 194.8297119140625),
            point2(96.3739013671875, 188.34327697753906),
            point2(100.390625, 182.1484375),
            point2(104.4586181640625, 176.24290466308594),
            point2(108.57421875, 170.6243896484375),
            point2(112.7337646484375, 165.2906036376953),
            point2(116.93359375, 160.2392578125),
            point2(121.1700439453125, 155.4680633544922),
            point2(125.439453125, 150.9747314453125),
            point2(129.7381591796875, 146.75697326660156),
            point2(134.0625, 142.8125),
            point2(138.4088134765625, 139.13902282714844),
            point2(142.7734375, 135.7342529296875),
            point2(147.1527099609375, 132.5959014892578),
            point2(151.54296875, 129.7216796875),
            point2(155.9405517578125, 127.10929870605469),
            point2(160.341796875, 124.7564697265625),
            point2(164.7430419921875, 122.66090393066406),
            point2(169.140625, 120.8203125),
            point2(173.5308837890625, 119.23240661621094),
            point2(177.91015625, 117.8948974609375),
            point2(182.2747802734375, 116.80549621582031),
            point2(186.62109375, 115.9619140625),
            point2(190.9454345703125, 115.36186218261719),
            point2(195.244140625, 115.0030517578125),
            point2(199.5135498046875, 114.88319396972656),
            point2(203.75, 115.0),
            point2(207.9498291015625, 115.35118103027344),
            point2(212.109375, 115.9344482421875),
            point2(216.2249755859375, 116.74751281738281),
            point2(220.29296875, 117.7880859375),
            point2(224.3096923828125, 119.05387878417969),
            point2(228.271484375, 120.5426025390625),
            point2(232.1746826171875, 122.25196838378906),
            point2(236.015625, 124.1796875),
            point2(239.7906494140625, 126.32347106933594),
            point2(243.49609375, 128.6810302734375),
            point2(247.1282958984375, 131.2500762939453),
            point2(250.68359375, 134.0283203125),
            point2(254.1583251953125, 137.0134735107422),
            point2(257.548828125, 140.2032470703125),
            point2(260.8514404296875, 143.59535217285156),
            point2(264.0625, 147.1875),
            point2(267.1783447265625, 150.97740173339844),
            point2(270.1953125, 154.9627685546875),
            point2(273.1097412109375, 159.1413116455078),
            point2(275.91796875, 163.5107421875),
            point2(278.6163330078125, 168.0687713623047),
            point2(281.201171875, 172.8131103515625),
            point2(283.6688232421875, 177.74147033691406),
            point2(286.015625, 182.8515625),
            point2(288.2379150390625, 188.14109802246094),
            point2(290.33203125, 193.6077880859375),
            point2(292.2943115234375, 199.2493438720703),
            point2(294.12109375, 205.0634765625),
            point2(295.8087158203125, 211.0478973388672),
            point2(297.353515625, 217.2003173828125),
            point2(298.7518310546875, 223.51844787597656),
            point2(300.0, 230.0),
        ];

        let input = vec![
            point2(70.0, 240.0),
            point2(145.0, 60.0),
            point2(275.0, 90.0),
            point2(300.0, 230.0),
        ];
        let result = super::points_on_bezier_curves(&input, 0.15, None);
        assert_eq!(result, expected);
    }

    #[test]
    fn points_on_bezier_curves_with_tolerance() {
        let expected = vec![
            point2(70.0, 240.0),
            point2(77.1875, 223.7371826171875),
            point2(84.66796875, 208.6865234375),
            point2(92.412109375, 194.8297119140625),
            point2(100.390625, 182.1484375),
            point2(108.57421875, 170.6243896484375),
            point2(116.93359375, 160.2392578125),
            point2(125.439453125, 150.9747314453125),
            point2(134.0625, 142.8125),
            point2(142.7734375, 135.7342529296875),
            point2(151.54296875, 129.7216796875),
            point2(160.341796875, 124.7564697265625),
            point2(169.140625, 120.8203125),
            point2(177.91015625, 117.8948974609375),
            point2(186.62109375, 115.9619140625),
            point2(195.244140625, 115.0030517578125),
            point2(203.75, 115.0),
            point2(212.109375, 115.9344482421875),
            point2(220.29296875, 117.7880859375),
            point2(228.271484375, 120.5426025390625),
            point2(236.015625, 124.1796875),
            point2(243.49609375, 128.6810302734375),
            point2(250.68359375, 134.0283203125),
            point2(257.548828125, 140.2032470703125),
            point2(264.0625, 147.1875),
            point2(270.1953125, 154.9627685546875),
            point2(275.91796875, 163.5107421875),
            point2(281.201171875, 172.8131103515625),
            point2(286.015625, 182.8515625),
            point2(290.33203125, 193.6077880859375),
            point2(294.12109375, 205.0634765625),
            point2(297.353515625, 217.2003173828125),
            point2(300.0, 230.0),
        ];
        let input = vec![
            point2(70.0, 240.0),
            point2(145.0, 60.0),
            point2(275.0, 90.0),
            point2(300.0, 230.0),
        ];
        let result = super::points_on_bezier_curves(&input, 0.7, None);
        assert_eq!(result, expected);
    }

    #[test]
    fn points_on_bezier_curves_with_distance() {
        let expected = vec![
            point2(70.0, 240.0),
            point2(73.5552978515625, 231.71592712402344),
            point2(80.8929443359375, 216.0614776611328),
            point2(88.5089111328125, 201.6100311279297),
            point2(96.3739013671875, 188.34327697753906),
            point2(100.390625, 182.1484375),
            point2(108.57421875, 170.6243896484375),
            point2(116.93359375, 160.2392578125),
            point2(125.439453125, 150.9747314453125),
            point2(134.0625, 142.8125),
            point2(142.7734375, 135.7342529296875),
            point2(151.54296875, 129.7216796875),
            point2(160.341796875, 124.7564697265625),
            point2(169.140625, 120.8203125),
            point2(177.91015625, 117.8948974609375),
            point2(186.62109375, 115.9619140625),
            point2(195.244140625, 115.0030517578125),
            point2(203.75, 115.0),
            point2(212.109375, 115.9344482421875),
            point2(220.29296875, 117.7880859375),
            point2(224.3096923828125, 119.05387878417969),
            point2(232.1746826171875, 122.25196838378906),
            point2(236.015625, 124.1796875),
            point2(243.49609375, 128.6810302734375),
            point2(250.68359375, 134.0283203125),
            point2(257.548828125, 140.2032470703125),
            point2(264.0625, 147.1875),
            point2(270.1953125, 154.9627685546875),
            point2(275.91796875, 163.5107421875),
            point2(281.201171875, 172.8131103515625),
            point2(286.015625, 182.8515625),
            point2(290.33203125, 193.6077880859375),
            point2(294.12109375, 205.0634765625),
            point2(297.353515625, 217.2003173828125),
            point2(300.0, 230.0),
        ];

        let input = vec![
            point2(70.0, 240.0),
            point2(145.0, 60.0),
            point2(275.0, 90.0),
            point2(300.0, 230.0),
        ];
        let result = super::points_on_bezier_curves(&input, 0.2, Some(0.15));
        assert_eq!(result, expected);
    }

    #[test]
    fn curve_to_bezier() {
        let expected = vec![
            point2(20.0, 240.0),
            point2(32.5, 211.5),
            point2(60.833333333333336, 94.0),
            point2(95.0, 69.0),
            point2(129.16666666666666, 44.0),
            point2(199.16666666666666, 71.5),
            point2(225.0, 90.0),
            point2(250.83333333333334, 108.5),
            point2(239.16666666666666, 158.33333333333334),
            point2(250.0, 180.0),
            point2(260.8333333333333, 201.66666666666666),
            point2(268.3333333333333, 236.66666666666666),
            point2(290.0, 220.0),
            point2(311.6666666666667, 203.33333333333334),
            point2(365.0, 103.33333333333333),
            point2(380.0, 80.0),
        ];
        let input = vec![
            point2(20.0, 240.0),
            point2(95.0, 69.0),
            point2(225.0, 90.0),
            point2(250.0, 180.0),
            point2(290.0, 220.0),
            point2(380.0, 80.0),
        ];
        let result = super::curve_to_bezier(&input, 0.0).unwrap();
        assert_eq!(expected, result);
    }
}
