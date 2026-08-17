use std::borrow::BorrowMut;

use euclid::default::Point2D;
use euclid::{point2, Trig};
use num_traits::{Float, FloatConst, FromPrimitive};
use svg_path_ops::{absolutize, normalize};
use svgtypes::{PathParser, PathSegment};

use super::core::{_c, Options};
use crate::core::{_cc, FillStyle, Op, OpSet, OpSetType, OpType};
use crate::filler::get_filler;
use crate::filler::FillerType::{
    DashedFiller, DotFiller, HatchFiller, ScanLineHachure, ZigZagFiller, ZigZagLineFiller,
};
use crate::geometry::{convert_bezier_quadratic_to_cubic, BezierQuadratic};

#[derive(PartialEq, Eq, Debug)]
pub struct EllipseParams<F: Float> {
    pub rx: F,
    pub ry: F,
    pub increment: F,
}

pub struct EllipseResult<F: Float + FromPrimitive + Trig> {
    pub opset: OpSet<F>,
    pub estimated_points: Vec<Point2D<F>>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ArcParams<F: Float> {
    pub x: F,
    pub y: F,
    pub width: F,
    pub height: F,
    pub start: F,
    pub stop: F,
    pub closed: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ArcRenderParams<F: Float> {
    pub arc: ArcParams<F>,
    pub rough_closure: bool,
}

#[derive(Clone, Copy)]
struct EllipsePointsSpec<F> {
    increment: F,
    cx: F,
    cy: F,
    rx: F,
    ry: F,
    offset: F,
    overlap: F,
}

#[derive(Clone, Copy)]
struct ArcPointsSpec<F> {
    increment: F,
    cx: F,
    cy: F,
    rx: F,
    ry: F,
    strt: F,
    stp: F,
    offset: F,
}

#[derive(Clone, Copy)]
struct BezierToSpec<F> {
    x1: F,
    y1: F,
    x2: F,
    y2: F,
    x: F,
    y: F,
    current: Point2D<F>,
}

/// Constructs a line primitive that can be rendered into relevant context
/// # Arguments
/// * `x1` - Line start point x coordinate
/// * `y1` - Line start point y coordinate
/// * `x2` - Line end point x coordinate
/// * `y2` - Line end point y coordinate
/// * `o`  - Line generation options
///
/// # Example
/// Note that result of this call is highly dependent on your selections of
/// options and random number seed you use
///
/// ```rust
/// use roughr::core::{OpSetType, OptionsBuilder};
/// use roughr::renderer::line;
///
/// // Use a non-zero seed for reproducible output.
/// let mut o = OptionsBuilder::default().seed(1_u64).build().unwrap();
/// let result = line(0.0, 0.0, 1.0, 0.0, &mut o);
/// assert_eq!(result.op_set_type, OpSetType::Path);
/// assert_eq!(result.size, None);
/// assert_eq!(result.path, None);
/// assert_eq!(result.ops.len(), 4);
/// ```
pub fn line<F: Float + Trig + FromPrimitive>(
    x1: F,
    y1: F,
    x2: F,
    y2: F,
    o: &mut Options,
) -> OpSet<F> {
    OpSet {
        op_set_type: OpSetType::Path,
        ops: _double_line(x1, y1, x2, y2, o, false),
        size: None,
        path: None,
    }
}

/// Constructs a linear path with given points by connecting consecutive points
/// with rough line primitives. This function is also used by other high level
/// constructs such as rectangle and polygon. For two element point list
/// it calls line function
///
/// # Arguments
/// * `points` - 2D Points which forms the path. Consecutive points will be connected to each other
/// * `close` - If algorithm should connect last point to first point with a line
/// * `o` - Path generation options.
///
/// # Example
/// Note that result of this call is highly dependent on your selections of
/// options and random number seed you use
///
///```rust
/// use euclid::point2;
/// use roughr::core::{OpSetType, OptionsBuilder};
/// use roughr::renderer::linear_path;
///
/// // Use a non-zero seed for reproducible output.
/// let mut o = OptionsBuilder::default().seed(1_u64).build().unwrap();
/// let result = linear_path(
///     &[point2(0.0f32, 0.0), point2(0.0, 0.1), point2(1.0, 1.0)],
///     false,
///     &mut o,
/// );
/// assert_eq!(result.op_set_type, OpSetType::Path);
/// assert_eq!(result.size, None);
/// assert_eq!(result.path, None);
/// ```
pub fn linear_path<F: Float + Trig + FromPrimitive>(
    points: &[Point2D<F>],
    close: bool,
    o: &mut Options,
) -> OpSet<F> {
    let len = points.len();
    if len > 2 {
        let mut ops: Vec<Op<F>> = Vec::new();
        let mut i = 0;
        while i < (len - 1) {
            ops.append(&mut _double_line(
                points[i].x,
                points[i].y,
                points[i + 1].x,
                points[i + 1].y,
                o,
                false,
            ));
            i += 1;
        }
        if close {
            ops.append(&mut _double_line(
                points[len - 1].x,
                points[len - 1].y,
                points[0].x,
                points[0].y,
                o,
                false,
            ));
        }
        OpSet {
            op_set_type: OpSetType::Path,
            ops,
            path: None,
            size: None,
        }
    } else if len == 2 {
        line(points[0].x, points[0].y, points[1].x, points[1].y, o)
    } else {
        OpSet {
            op_set_type: OpSetType::Path,
            ops: Vec::new(),
            path: None,
            size: None,
        }
    }
}

pub fn polygon<F: Float + Trig + FromPrimitive>(
    points: &[Point2D<F>],
    o: &mut Options,
) -> OpSet<F> {
    linear_path(points, true, o)
}

pub fn rectangle<F: Float + Trig + FromPrimitive>(
    x: F,
    y: F,
    width: F,
    height: F,
    o: &mut Options,
) -> OpSet<F> {
    let points: Vec<Point2D<F>> = vec![
        Point2D::new(x, y),
        Point2D::new(x + width, y),
        Point2D::new(x + width, y + height),
        Point2D::new(x, y + height),
    ];
    polygon(&points, o)
}

pub fn bezier_quadratic<F: Float + Trig + FromPrimitive>(
    start: Point2D<F>,
    cp: Point2D<F>,
    end: Point2D<F>,
    o: &mut Options,
) -> OpSet<F> {
    let ops = _bezier_quadratic_to(cp.x, cp.y, end.x, end.y, &start, o);

    OpSet {
        op_set_type: OpSetType::Path,
        ops,
        path: None,
        size: None,
    }
}

pub fn bezier_cubic<F: Float + Trig + FromPrimitive>(
    start: Point2D<F>,
    cp1: Point2D<F>,
    cp2: Point2D<F>,
    end: Point2D<F>,
    o: &mut Options,
) -> OpSet<F> {
    let ops = _bezier_to(
        BezierToSpec {
            x1: cp1.x,
            y1: cp1.y,
            x2: cp2.x,
            y2: cp2.y,
            x: end.x,
            y: end.y,
            current: start,
        },
        o,
    );

    OpSet {
        op_set_type: OpSetType::Path,
        ops,
        path: None,
        size: None,
    }
}

pub fn curve<F: Float + Trig + FromPrimitive>(points: &[Point2D<F>], o: &mut Options) -> OpSet<F> {
    let mut o1 = _curve_with_offset(
        points,
        _c::<F>(1.0) * _c(1.0 + o.roughness.unwrap_or(0.0) * 0.2),
        o,
    );
    if !o.disable_multi_stroke.unwrap_or(false) {
        let mut o2 = _curve_with_offset(
            points,
            _c::<F>(1.5) * _c(1.0 + o.roughness.unwrap_or(0.0) * 0.22),
            &mut clone_options_alter_seed(o),
        );
        o1.append(&mut o2);
    }
    OpSet {
        op_set_type: OpSetType::Path,
        ops: o1,
        path: None,
        size: None,
    }
}

pub fn ellipse<F: Float + Trig + FromPrimitive>(
    x: F,
    y: F,
    width: F,
    height: F,
    o: &mut Options,
) -> OpSet<F> {
    let params = generate_ellipse_params(width, height, o);
    ellipse_with_params(x, y, o, &params).opset
}

pub fn generate_ellipse_params<F: Float + Trig + FromPrimitive>(
    width: F,
    height: F,
    o: &mut Options,
) -> EllipseParams<F> {
    let psq: F = Float::sqrt(
        _cc::<F>(std::f64::consts::PI * 2.0)
            * Float::sqrt(
                (Float::powi(width / _c(2.0), 2) + Float::powi(height / _c(2.0), 2)) / _c(2.0),
            ),
    );
    let step_count: F = Float::ceil(Float::max(
        _c(o.curve_step_count.unwrap_or(1.0)),
        _c::<F>(o.curve_step_count.unwrap_or(1.0) / Float::sqrt(200.0)) * psq,
    ));
    let increment: F = _cc::<F>(std::f64::consts::PI * 2.0) / step_count;
    let mut rx = Float::abs(width / _c(2.0));
    let mut ry = Float::abs(height / _c(2.0));
    let curve_fit_randomness: F = _c::<F>(1.0) - _c(o.curve_fitting.unwrap_or(0.0));
    rx = rx + _offset_opt(rx * curve_fit_randomness, o, None);
    ry = ry + _offset_opt(ry * curve_fit_randomness, o, None);
    EllipseParams { increment, rx, ry }
}

pub fn ellipse_with_params<F: Float + Trig + FromPrimitive>(
    x: F,
    y: F,
    o: &mut Options,
    ellipse_params: &EllipseParams<F>,
) -> EllipseResult<F> {
    let overlap = ellipse_params.increment
        * _offset(
            _c(0.1),
            _offset(_c::<F>(0.4), _c::<F>(1.0), o, None),
            o,
            None,
        );
    let ellipse_points = _compute_ellipse_points(
        EllipsePointsSpec {
            increment: ellipse_params.increment,
            cx: x,
            cy: y,
            rx: ellipse_params.rx,
            ry: ellipse_params.ry,
            offset: _c(1.0),
            overlap,
        },
        o,
    );
    let ap1 = ellipse_points[0].clone();
    let cp1 = ellipse_points[1].clone();
    let mut o1 = _curve(&ap1, None, o);
    if (!o.disable_multi_stroke.unwrap_or(false)) && (o.roughness.unwrap_or(0.0) != 0.0) {
        let inner_ellipse_points = _compute_ellipse_points(
            EllipsePointsSpec {
                increment: ellipse_params.increment,
                cx: x,
                cy: y,
                rx: ellipse_params.rx,
                ry: ellipse_params.ry,
                offset: _c::<F>(1.5),
                overlap: _c::<F>(0.0),
            },
            o,
        );
        let ap2 = inner_ellipse_points[0].clone();
        let _cp2 = inner_ellipse_points[1].clone();
        let mut o2 = _curve(&ap2, None, o);
        o1.append(&mut o2);
    }
    EllipseResult {
        estimated_points: cp1,
        opset: OpSet {
            op_set_type: OpSetType::Path,
            ops: o1,
            size: None,
            path: None,
        },
    }
}

pub fn arc<F: Float + Trig + FromPrimitive>(
    params: ArcRenderParams<F>,
    o: &mut Options,
) -> OpSet<F> {
    let ArcRenderParams { arc, rough_closure } = params;
    let ArcParams {
        x,
        y,
        width,
        height,
        start,
        stop,
        closed,
    } = arc;
    let cx = x;
    let cy = y;
    let mut rx = Float::abs(width / _c(2.0));
    let mut ry = Float::abs(height / _c(2.0));
    rx = rx + _offset_opt(rx * _c(0.01), o, None);
    ry = ry + _offset_opt(ry * _c(0.01), o, None);
    let mut strt: F = start;
    let mut stp: F = stop;
    while strt < _c(0.0) {
        strt = strt + _c(f32::PI() * 2.0);
        stp = stp + _c(f32::PI() * 2.0);
    }
    if (stp - strt) > _c(f32::PI() * 2.0) {
        strt = _c(0.0);
        stp = _c(f32::PI() * 2.0);
    }
    let ellipse_inc: F = _c::<F>(f32::PI() * 2.0) / _c(o.curve_step_count.unwrap_or(1.0));
    let arc_inc = Float::min(ellipse_inc / _c(2.0), (stp - strt) / _c(2.0));
    let mut ops = _arc(
        ArcPointsSpec {
            increment: arc_inc,
            cx,
            cy,
            rx,
            ry,
            strt,
            stp,
            offset: _c(1.0),
        },
        o,
    );
    if !o.disable_multi_stroke.unwrap_or(false) {
        let mut o2 = _arc(
            ArcPointsSpec {
                increment: arc_inc,
                cx,
                cy,
                rx,
                ry,
                strt,
                stp,
                offset: _c(1.5),
            },
            o,
        );
        ops.append(&mut o2);
    }
    if closed {
        if rough_closure {
            ops.append(&mut _double_line(
                cx,
                cy,
                cx + rx * Float::cos(strt),
                cy + ry * Float::sin(strt),
                o,
                false,
            ));
            ops.append(&mut _double_line(
                cx,
                cy,
                cx + rx * Float::cos(stp),
                cy + ry * Float::sin(stp),
                o,
                false,
            ));
        } else {
            ops.push(Op {
                op: OpType::LineTo,
                data: vec![cx, cy],
            });
            ops.push(Op {
                op: OpType::LineTo,
                data: vec![cx + rx * Float::cos(strt), cy + ry * Float::sin(strt)],
            });
        }
    }
    OpSet {
        op_set_type: OpSetType::Path,
        ops,
        path: None,
        size: None,
    }
}

pub fn solid_fill_polygon<F: Float + Trig + FromPrimitive>(
    polygon_list: &Vec<Vec<Point2D<F>>>,
    options: &mut Options,
) -> OpSet<F> {
    let mut ops = vec![];
    for polygon in polygon_list {
        if polygon.len() > 2 {
            let rand_offset = _c(options.max_randomness_offset.unwrap_or(2.0));
            polygon.iter().enumerate().for_each(|(ind, point)| {
                if ind == 0 {
                    ops.push(Op {
                        op: OpType::Move,
                        data: vec![
                            point.x + _offset_opt(rand_offset, options, None),
                            point.y + _offset_opt(rand_offset, options, None),
                        ],
                    });
                } else {
                    ops.push(Op {
                        op: OpType::LineTo,
                        data: vec![
                            point.x + _offset_opt(rand_offset, options, None),
                            point.y + _offset_opt(rand_offset, options, None),
                        ],
                    });
                }
            })
        }
    }
    OpSet {
        op_set_type: OpSetType::FillPath,
        ops,
        size: None,
        path: None,
    }
}

pub fn rand_offset<F: Float + Trig + FromPrimitive>(x: F, o: &mut Options) -> F {
    _offset_opt(x, o, None)
}

pub fn rand_offset_with_range<F: Float + Trig + FromPrimitive>(
    min: F,
    max: F,
    o: &mut Options,
) -> F {
    _offset(min, max, o, None)
}

pub fn double_line_fill_ops<F: Float + Trig + FromPrimitive>(
    x1: F,
    y1: F,
    x2: F,
    y2: F,
    o: &mut Options,
) -> Vec<Op<F>> {
    _double_line(x1, y1, x2, y2, o, true)
}

fn clone_options_alter_seed(ops: &mut Options) -> Options {
    // Match Rough.js `cloneOptionsAlterSeed` from `bin/renderer.js`:
    // - clones the options object
    // - clears `randomizer` so the clone creates its own PRNG stream
    // - if `seed` is truthy (non-zero), increments it by 1
    let mut result: Options = ops.clone();
    result.randomizer = None;
    if let Some(seed) = ops.seed {
        if seed != 0 {
            result.seed = Some(seed + 1);
        }
    }
    result
}

fn _offset<F: Float + Trig + FromPrimitive>(
    min: F,
    max: F,
    ops: &mut Options,
    roughness_gain: Option<F>,
) -> F {
    let rg: F = roughness_gain.unwrap_or_else(|| _c(1.0));
    _c::<F>(ops.roughness.unwrap_or(1.0)) * rg * ((_cc::<F>(ops.random()) * (max - min)) + min)
}

fn _offset_opt<F: Float + Trig + FromPrimitive>(
    x: F,
    ops: &mut Options,
    roughness_gain: Option<F>,
) -> F {
    _offset(-x, x, ops, roughness_gain)
}

fn _line<F: Float + Trig + FromPrimitive>(
    x1: F,
    y1: F,
    x2: F,
    y2: F,
    o: &mut Options,
    mover: bool,
    overlay: bool,
) -> Vec<Op<F>> {
    let length_sq = (x1 - x2).powi(2) + (y1 - y2).powi(2);
    let length = length_sq.sqrt();
    let roughness_gain;
    if length < _c(200.0_f32) {
        roughness_gain = _c(1.0);
    } else if length > _c(500.0) {
        roughness_gain = _c(0.4);
    } else {
        roughness_gain = _c::<F>(-0.0016668) * length + _c(1.233334);
    }

    let mut offset = _c(o.max_randomness_offset.unwrap_or(2.0));
    if (offset * offset * _c(100.0)) > length_sq {
        offset = length / _c(10.0);
    }
    let half_offset = offset / _c(2.0);
    let diverge_point = _c::<F>(0.2) + _cc::<F>(o.random()) * _c(0.2);
    let mut mid_disp_x =
        _c::<F>(o.bowing.unwrap_or(1.0)) * _c(o.max_randomness_offset.unwrap_or(2.0)) * (y2 - y1)
            / _c(200.0);
    let mut mid_disp_y =
        _c::<F>(o.bowing.unwrap_or(1.0)) * _c(o.max_randomness_offset.unwrap_or(2.0)) * (x1 - x2)
            / _c(200.0);
    mid_disp_x = _offset_opt(mid_disp_x, o, Some(roughness_gain));
    mid_disp_y = _offset_opt(mid_disp_y, o, Some(roughness_gain));
    let mut ops: Vec<Op<F>> = Vec::new();

    let preserve_vertices = o.preserve_vertices.unwrap_or(false);
    if mover {
        if overlay {
            ops.push(Op {
                op: OpType::Move,
                data: vec![
                    x1 + if preserve_vertices {
                        _c(0.0)
                    } else {
                        _offset_opt(half_offset, o, Some(roughness_gain))
                    },
                    y1 + if preserve_vertices {
                        _c(0.0)
                    } else {
                        _offset_opt(half_offset, o, Some(roughness_gain))
                    },
                ],
            });
        } else {
            ops.push(Op {
                op: OpType::Move,
                data: vec![
                    x1 + if preserve_vertices {
                        _c(0.0)
                    } else {
                        _offset_opt(offset, o, Some(roughness_gain))
                    },
                    y1 + if preserve_vertices {
                        _c(0.0)
                    } else {
                        _offset_opt(offset, o, Some(roughness_gain))
                    },
                ],
            });
        }
    }
    if overlay {
        ops.push(Op {
            op: OpType::BCurveTo,
            data: vec![
                mid_disp_x
                    + x1
                    + (x2 - x1) * diverge_point
                    + _offset_opt(half_offset, o, Some(roughness_gain)),
                mid_disp_y
                    + y1
                    + (y2 - y1) * diverge_point
                    + _offset_opt(half_offset, o, Some(roughness_gain)),
                mid_disp_x
                    + x1
                    + _c::<F>(2.0) * (x2 - x1) * diverge_point
                    + _offset_opt(half_offset, o, Some(roughness_gain)),
                mid_disp_y
                    + y1
                    + _c::<F>(2.0) * (y2 - y1) * diverge_point
                    + _offset_opt(half_offset, o, Some(roughness_gain)),
                x2 + if preserve_vertices {
                    _c(0.0)
                } else {
                    _offset_opt(half_offset, o, Some(roughness_gain))
                },
                y2 + if preserve_vertices {
                    _c(0.0)
                } else {
                    _offset_opt(half_offset, o, Some(roughness_gain))
                },
            ],
        });
    } else {
        ops.push(Op {
            op: OpType::BCurveTo,
            data: vec![
                mid_disp_x
                    + x1
                    + (x2 - x1) * diverge_point
                    + _offset_opt(offset, o, Some(roughness_gain)),
                mid_disp_y
                    + y1
                    + (y2 - y1) * diverge_point
                    + _offset_opt(offset, o, Some(roughness_gain)),
                mid_disp_x
                    + x1
                    + _c::<F>(2.0) * (x2 - x1) * diverge_point
                    + _offset_opt(offset, o, Some(roughness_gain)),
                mid_disp_y
                    + y1
                    + _c::<F>(2.0) * (y2 - y1) * diverge_point
                    + _offset_opt(offset, o, Some(roughness_gain)),
                x2 + if preserve_vertices {
                    _c(0.0)
                } else {
                    _offset_opt(offset, o, Some(roughness_gain))
                },
                y2 + if preserve_vertices {
                    _c(0.0)
                } else {
                    _offset_opt(offset, o, Some(roughness_gain))
                },
            ],
        });
    }
    ops
}

pub(crate) fn _double_line<F: Float + Trig + FromPrimitive>(
    x1: F,
    y1: F,
    x2: F,
    y2: F,
    o: &mut Options,
    filling: bool,
) -> Vec<Op<F>> {
    let single_stroke = if filling {
        o.disable_multi_stroke_fill.unwrap_or(false)
    } else {
        o.disable_multi_stroke.unwrap_or(false)
    };
    let mut o1 = _line(x1, y1, x2, y2, o, true, false);
    if single_stroke {
        o1
    } else {
        let mut o2 = _line(x1, y1, x2, y2, o, true, true);
        o1.append(&mut o2);
        o1
    }
}

pub(crate) fn _curve<F: Float + Trig + FromPrimitive>(
    points: &[Point2D<F>],
    close_point: Option<Point2D<F>>,
    o: &mut Options,
) -> Vec<Op<F>> {
    let len = points.len();
    let mut ops: Vec<Op<F>> = vec![];
    if len > 3 {
        let mut b: [[F; 2]; 4] = [[_c(0.0); 2]; 4];
        let s: F = _c::<F>(1.0) - _c(o.curve_tightness.unwrap_or(0.0));
        ops.push(Op {
            op: OpType::Move,
            data: vec![points[1].x, points[1].y],
        });
        let mut i = 1;
        while (i + 2) < len {
            let cached_vert_array = points[i];
            b[0] = [cached_vert_array.x, cached_vert_array.y];
            b[1] = [
                cached_vert_array.x + (s * points[i + 1].x - s * points[i - 1].x) / _c(6.0),
                cached_vert_array.y + (s * points[i + 1].y - s * points[i - 1].y) / _c(6.0),
            ];
            b[2] = [
                points[i + 1].x + (s * points[i].x - s * points[i + 2].x) / _c(6.0),
                points[i + 1].y + (s * points[i].y - s * points[i + 2].y) / _c(6.0),
            ];
            b[3] = [points[i + 1].x, points[i + 1].y];
            ops.push(Op {
                op: OpType::BCurveTo,
                data: vec![b[1][0], b[1][1], b[2][0], b[2][1], b[3][0], b[3][1]],
            });
            i += 1;
        }
        if let Some(cp) = close_point {
            let ro = _c(o.max_randomness_offset.unwrap_or(2.0));
            ops.push(Op {
                op: OpType::LineTo,
                data: vec![
                    cp.x + _offset_opt(ro, o, None),
                    cp.y + _offset_opt(ro, o, None),
                ],
            });
        }
    } else if len == 3 {
        ops.push(Op {
            op: OpType::Move,
            data: vec![points[1].x, points[1].y],
        });
        ops.push(Op {
            op: OpType::BCurveTo,
            data: vec![
                points[1].x,
                points[1].y,
                points[2].x,
                points[2].y,
                points[2].x,
                points[2].y,
            ],
        });
    } else if len == 2 {
        ops.append(&mut _double_line(
            points[0].x,
            points[0].y,
            points[1].x,
            points[1].y,
            o,
            false,
        ));
    }
    ops
}

fn _curve_with_offset<F: Float + Trig + FromPrimitive>(
    points: &[Point2D<F>],
    offset: F,
    o: &mut Options,
) -> Vec<Op<F>> {
    let mut ps: Vec<Point2D<F>> = vec![
        Point2D::new(
            points[0].x + _offset_opt(offset, o, None),
            points[0].y + _offset_opt(offset, o, None),
        ),
        Point2D::new(
            points[0].x + _offset_opt(offset, o, None),
            points[0].y + _offset_opt(offset, o, None),
        ),
    ];
    let mut i = 1;
    while i < points.len() {
        ps.push(Point2D::new(
            points[i].x + _offset_opt(offset, o, None),
            points[i].y + _offset_opt(offset, o, None),
        ));
        if i == (points.len() - 1) {
            ps.push(Point2D::new(
                points[i].x + _offset_opt(offset, o, None),
                points[i].y + _offset_opt(offset, o, None),
            ));
        }
        i += 1;
    }
    _curve(&ps, None, o)
}

fn _compute_ellipse_points<F: Float + Trig + FromPrimitive>(
    spec: EllipsePointsSpec<F>,
    o: &mut Options,
) -> Vec<Vec<Point2D<F>>> {
    let EllipsePointsSpec {
        increment,
        cx,
        cy,
        rx,
        ry,
        offset,
        overlap,
    } = spec;
    let core_only = o.roughness.unwrap_or(0.0) == 0.0;
    let mut core_points: Vec<Point2D<F>> = Vec::new();
    let mut all_points: Vec<Point2D<F>> = Vec::new();

    if core_only {
        let increment_inner = increment / _c(4.0);
        all_points.push(Point2D::new(
            cx + rx * Float::cos(-increment_inner),
            cy + ry * Float::sin(-increment_inner),
        ));

        let mut angle = _c(0.0);
        while angle <= _cc::<F>(std::f64::consts::PI * 2.0) {
            let p = Point2D::new(cx + rx * Float::cos(angle), cy + ry * Float::sin(angle));
            core_points.push(p);
            all_points.push(p);
            angle = angle + increment_inner;
        }
        all_points.push(Point2D::new(
            cx + rx * Float::cos(_c(0.0)),
            cy + ry * Float::sin(_c(0.0)),
        ));
        all_points.push(Point2D::new(
            cx + rx * Float::cos(increment_inner),
            cy + ry * Float::sin(increment_inner),
        ));
    } else {
        let rad_offset: F = _offset_opt::<F>(_c(0.5), o, None) - (_c::<F>(f32::PI()) / _c(2.0));
        all_points.push(Point2D::new(
            _offset_opt(offset, o, None)
                + cx
                + _c::<F>(0.9) * rx * Float::cos(rad_offset - increment),
            _offset_opt(offset, o, None)
                + cy
                + _c::<F>(0.9) * ry * Float::sin(rad_offset - increment),
        ));
        let end_angle = _cc::<F>(std::f64::consts::PI * 2.0) + rad_offset - _c(0.01);
        let mut angle = rad_offset;
        while angle < end_angle {
            let p = Point2D::new(
                _offset_opt(offset, o, None) + cx + rx * Float::cos(angle),
                _offset_opt(offset, o, None) + cy + ry * Float::sin(angle),
            );
            core_points.push(p);
            all_points.push(p);
            angle = angle + increment;
        }

        all_points.push(Point2D::new(
            _offset_opt(offset, o, None)
                + cx
                + rx * Float::cos(
                    rad_offset + _cc::<F>(std::f64::consts::PI * 2.0) + overlap * _c(0.5),
                ),
            _offset_opt(offset, o, None)
                + cy
                + ry * Float::sin(
                    rad_offset + _cc::<F>(std::f64::consts::PI * 2.0) + overlap * _c(0.5),
                ),
        ));
        all_points.push(Point2D::new(
            _offset_opt(offset, o, None)
                + cx
                + _c::<F>(0.98) * rx * Float::cos(rad_offset + overlap),
            _offset_opt(offset, o, None)
                + cy
                + _c::<F>(0.98) * ry * Float::sin(rad_offset + overlap),
        ));
        all_points.push(Point2D::new(
            _offset_opt(offset, o, None)
                + cx
                + _c::<F>(0.9) * rx * Float::cos(rad_offset + overlap * _c(0.5)),
            _offset_opt(offset, o, None)
                + cy
                + _c::<F>(0.9) * ry * Float::sin(rad_offset + overlap * _c(0.5)),
        ));
    }
    vec![all_points, core_points]
}

fn _arc<F: Float + Trig + FromPrimitive>(spec: ArcPointsSpec<F>, o: &mut Options) -> Vec<Op<F>> {
    let ArcPointsSpec {
        increment,
        cx,
        cy,
        rx,
        ry,
        strt,
        stp,
        offset,
    } = spec;
    let rad_offset = strt + _offset_opt(_c(0.1), o, None);
    let mut points: Vec<Point2D<F>> = vec![Point2D::new(
        _offset_opt(offset, o, None) + cx + _c::<F>(0.9) * rx * Float::cos(rad_offset - increment),
        _offset_opt(offset, o, None) + cy + _c::<F>(0.9) * ry * Float::sin(rad_offset - increment),
    )];
    let mut angle = rad_offset;
    while angle <= stp {
        points.push(Point2D::new(
            _offset_opt(offset, o, None) + cx + rx * Float::cos(angle),
            _offset_opt(offset, o, None) + cy + ry * Float::sin(angle),
        ));
        angle = angle + increment;
    }
    points.push(Point2D::new(
        cx + rx * Float::cos(stp),
        cy + ry * Float::sin(stp),
    ));
    points.push(Point2D::new(
        cx + rx * Float::cos(stp),
        cy + ry * Float::sin(stp),
    ));
    _curve(&points, None, o)
}

fn _bezier_quadratic_to<F: Float + Trig + FromPrimitive>(
    x1: F,
    y1: F,
    x: F,
    y: F,
    current: &Point2D<F>,
    o: &mut Options,
) -> Vec<Op<F>> {
    // We simply convert the quadratic to a cubic bezier

    let cubic = convert_bezier_quadratic_to_cubic(BezierQuadratic {
        start: *current,
        cp: Point2D::new(x1, y1),
        end: Point2D::new(x, y),
    });

    _bezier_to(
        BezierToSpec {
            x1: cubic.cp1.x,
            y1: cubic.cp1.y,
            x2: cubic.cp2.x,
            y2: cubic.cp2.y,
            x: cubic.end.x,
            y: cubic.end.y,
            current: cubic.start,
        },
        o,
    )
}

fn _bezier_to<F: Float + Trig + FromPrimitive>(
    spec: BezierToSpec<F>,
    o: &mut Options,
) -> Vec<Op<F>> {
    let BezierToSpec {
        x1,
        y1,
        x2,
        y2,
        x,
        y,
        current,
    } = spec;
    let mut ops: Vec<Op<F>> = Vec::new();
    let ros = [
        _c(o.max_randomness_offset.unwrap_or(2.0)),
        _c(o.max_randomness_offset.unwrap_or(2.0) + 0.3),
    ];
    let mut f: Point2D<F>;
    let iterations = if o.disable_multi_stroke.unwrap_or(false) {
        1
    } else {
        2
    };
    let preserve_vertices = o.preserve_vertices.unwrap_or(false);
    let mut i = 0;
    while i < iterations {
        if i == 0 {
            ops.push(Op {
                op: OpType::Move,
                data: vec![current.x, current.y],
            });
        } else {
            ops.push(Op {
                op: OpType::Move,
                data: vec![
                    current.x
                        + (if preserve_vertices {
                            _c(0.0)
                        } else {
                            _offset_opt(ros[0], o, None)
                        }),
                    current.y
                        + (if preserve_vertices {
                            _c(0.0)
                        } else {
                            _offset_opt(ros[0], o, None)
                        }),
                ],
            });
        }
        f = if preserve_vertices {
            Point2D::new(x, y)
        } else {
            Point2D::new(
                x + _offset_opt(ros[i], o, None),
                y + _offset_opt(ros[i], o, None),
            )
        };
        ops.push(Op {
            op: OpType::BCurveTo,
            data: vec![
                x1 + _offset_opt(ros[i], o, None),
                y1 + _offset_opt(ros[i], o, None),
                x2 + _offset_opt(ros[i], o, None),
                y2 + _offset_opt(ros[i], o, None),
                f.x,
                f.y,
            ],
        });
        i += 1;
    }
    ops
}

pub fn pattern_fill_polygons<F, P>(polygon_list: P, o: &mut Options) -> OpSet<F>
where
    F: Float + Trig + FromPrimitive,
    P: BorrowMut<Vec<Vec<Point2D<F>>>>,
{
    let filler = if let Some(fill_style) = o.fill_style.as_ref() {
        match fill_style {
            FillStyle::Hachure => get_filler(ScanLineHachure),
            FillStyle::Dashed => get_filler(DashedFiller),
            FillStyle::Dots => get_filler(DotFiller),
            FillStyle::CrossHatch => get_filler(HatchFiller),
            FillStyle::ZigZag => get_filler(ZigZagFiller),
            FillStyle::ZigZagLine => get_filler(ZigZagLineFiller),
            _ => get_filler(ScanLineHachure),
        }
    } else {
        get_filler(ScanLineHachure)
    };
    filler.fill_polygons(polygon_list, o)
}

pub fn pattern_fill_arc<F>(
    x: F,
    y: F,
    width: F,
    height: F,
    start: F,
    stop: F,
    o: &mut Options,
) -> OpSet<F>
where
    F: Float + FromPrimitive + Trig,
{
    let cx = x;
    let cy = y;
    let mut rx = F::abs(width / _c(2.0));
    let mut ry = F::abs(height / _c(2.0));

    rx = rx + _offset_opt(rx * _c(0.01), o, None);
    ry = ry + _offset_opt(ry * _c(0.01), o, None);

    let mut strt = start;
    let mut stp = stop;
    let two_pi = _c::<F>(f32::PI()) * _c::<F>(2.0);

    while strt < _c(0.0) {
        strt = strt + two_pi;
        stp = stp + two_pi;
    }

    if (stp - strt) > two_pi {
        strt = F::zero();
        stp = two_pi;
    }

    let increment = (stp / strt) / o.curve_step_count.map(|a| _c(a)).unwrap_or_else(|| _c(1.0));
    let mut points: Vec<Point2D<F>> = vec![];

    let mut angle = strt;

    while angle <= stp {
        points.push(point2(
            cx + rx * Float::cos(angle),
            cy + ry * Float::sin(angle),
        ));
        angle = angle + increment;
    }

    points.push(point2(cx + rx * Float::cos(stp), cy + ry * Float::sin(stp)));
    points.push(point2(cx, cy));
    pattern_fill_polygons(vec![points], o)
}

pub fn svg_path<F>(path: String, o: &mut Options) -> OpSet<F>
where
    F: Float + FromPrimitive + Trig,
{
    let ops = vec![];
    let first = Point2D::new(_c::<F>(0.0), _c::<F>(0.0));
    let current = Point2D::new(_c::<F>(0.0), _c::<F>(0.0));
    let path_parser = PathParser::from(path.as_ref());
    let path_segments: Vec<PathSegment> = path_parser.flatten().collect();
    let normalized_segments = normalize(absolutize(path_segments.iter()));

    opset_from_path(o, ops, first, current, normalized_segments)
}

pub fn svg_segments<F>(path_segments: Vec<PathSegment>, o: &mut Options) -> OpSet<F>
where
    F: Float + FromPrimitive + Trig,
{
    let ops = vec![];
    let first = Point2D::new(_c::<F>(0.0), _c::<F>(0.0));
    let current = Point2D::new(_c::<F>(0.0), _c::<F>(0.0));
    let normalized_segments = normalize(absolutize(path_segments.iter()));

    opset_from_path(o, ops, first, current, normalized_segments)
}

pub fn svg_normalized_segments<F>(normalized_segments: &[PathSegment], o: &mut Options) -> OpSet<F>
where
    F: Float + FromPrimitive + Trig,
{
    let ops = vec![];
    let first = Point2D::new(_c::<F>(0.0), _c::<F>(0.0));
    let current = Point2D::new(_c::<F>(0.0), _c::<F>(0.0));

    opset_from_path(o, ops, first, current, normalized_segments.iter().cloned())
}

fn opset_from_path<F>(
    o: &mut Options,
    mut ops: Vec<Op<F>>,
    mut first: euclid::Point2D<F, euclid::UnknownUnit>,
    mut current: euclid::Point2D<F, euclid::UnknownUnit>,
    normalized_segments: impl Iterator<Item = PathSegment>,
) -> OpSet<F>
where
    F: Float + FromPrimitive + Trig,
{
    for segment in normalized_segments {
        match segment {
            PathSegment::MoveTo { abs: true, x, y } => {
                // Rough.js `svgPath(...)` treats `M` as cursor state and does not emit an output op.
                // This also avoids consuming extra randomness that would shift subsequent
                // `divergePoint` values (critical even when `roughness = 0`).
                current = Point2D::new(_cc::<F>(x), _cc::<F>(y));
                first = Point2D::new(_cc::<F>(x), _cc::<F>(y));
            }
            PathSegment::LineTo { abs: true, x, y } => {
                ops.extend(_double_line(
                    current.x,
                    current.y,
                    _cc::<F>(x),
                    _cc::<F>(y),
                    o,
                    false,
                ));
                current = Point2D::new(_cc::<F>(x), _cc::<F>(y));
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
                ops.extend(_bezier_to(
                    BezierToSpec {
                        x1: _cc::<F>(x1),
                        y1: _cc::<F>(y1),
                        x2: _cc::<F>(x2),
                        y2: _cc::<F>(y2),
                        x: _cc::<F>(x),
                        y: _cc::<F>(y),
                        current,
                    },
                    o,
                ));
                current = Point2D::new(_cc::<F>(x), _cc::<F>(y));
            }
            PathSegment::ClosePath { abs: true } => {
                ops.extend(_double_line(
                    current.x, current.y, first.x, first.y, o, false,
                ));
                current = Point2D::new(first.x, first.y);
            }
            _ => panic!("Unexpected segment type"),
        }
    }
    OpSet {
        op_set_type: OpSetType::Path,
        ops,
        size: None,
        path: None,
    }
}

#[cfg(test)]
mod test {
    use euclid::point2;

    use super::{_compute_ellipse_points, _curve, EllipseParams, EllipsePointsSpec};
    use crate::core::{Op, OpSet, OpSetType, OpType, Options, OptionsBuilder};

    fn get_default_options() -> Options {
        OptionsBuilder::default()
            .seed(345_u64)
            .build()
            .expect("failed to build default options")
    }

    #[test]
    fn linear_path() {
        let result = super::linear_path(
            &[point2(0.0f32, 0.0), point2(0.0, 0.1), point2(1.0, 1.0)],
            false,
            &mut get_default_options(),
        );
        assert_eq!(result.op_set_type, OpSetType::Path);
        assert_eq!(
            result,
            OpSet {
                op_set_type: OpSetType::Path,
                ops: vec![
                    Op {
                        op: OpType::Move,
                        data: vec![-0.0012205532, 0.0027011754]
                    },
                    Op {
                        op: OpType::BCurveTo,
                        data: vec![
                            0.007769434,
                            0.026625521,
                            -0.006868569,
                            0.035775613,
                            -0.008950782,
                            0.09677874
                        ]
                    },
                    Op {
                        op: OpType::Move,
                        data: vec![0.0010519032, -0.0035673995]
                    },
                    Op {
                        op: OpType::BCurveTo,
                        data: vec![
                            -0.0034613428,
                            0.040271826,
                            -0.0024902155,
                            0.074226834,
                            -0.0011456441,
                            0.09860738
                        ]
                    },
                    Op {
                        op: OpType::Move,
                        data: vec![-0.123573944, 0.12996572]
                    },
                    Op {
                        op: OpType::BCurveTo,
                        data: vec![
                            0.19195092, 0.3300887, 0.6037904, 0.64465916, 0.98244137, 1.0048801
                        ]
                    },
                    Op {
                        op: OpType::Move,
                        data: vec![0.047007628, 0.116847344]
                    },
                    Op {
                        op: OpType::BCurveTo,
                        data: vec![
                            0.37384683, 0.40432873, 0.84561193, 0.77220255, 1.0250582, 0.96798605
                        ]
                    }
                ],
                size: None,
                path: None
            }
        );
    }

    #[test]
    #[ignore = "failing due to randomness"]
    fn ellipse_with_params() {
        let expected_estimated_points = vec![
            point2(0.6818724507954145, -0.24215675845215262),
            point2(1.3682071413206485, 0.7930465114686116),
            point2(1.9097816708676238, 0.7671100939304721),
            point2(0.8360414855920169, 1.5122198080140175),
            point2(0.531355187897985, 0.4738367335276372),
            point2(1.111026909625053, 1.3449538537307408),
            point2(1.1040092949849214, 1.801902725957649),
            point2(0.4258957275631308, 1.2442749714336163),
            point2(0.5661545950654607, 0.6328000056262721),
        ];

        let result = super::ellipse_with_params(
            0.1,
            0.1,
            &mut get_default_options(),
            &EllipseParams {
                rx: 0.486848765998615,
                ry: 0.4755334706420514,
                increment: 0.6981317007977318,
            },
        );

        assert_eq!(expected_estimated_points, result.estimated_points);
    }

    #[test]
    #[ignore = "failing due to randommness"]
    fn compute_ellipse_points() {
        let expected = vec![
            vec![
                point2(1.0710641633603797, 0.6343339196221436),
                point2(0.9888360341310736, 0.539884571860436),
                point2(1.0423582717058324, 0.48447611636004245),
                point2(1.1323647757131408, 0.48734422393942145),
                point2(1.097114022520837, 0.5024772415343248),
                point2(1.1983573886194598, 0.6344444071433158),
                point2(1.2951674832143851, 0.641832264291391),
                point2(1.3536023670520665, 0.6251662974163592),
                point2(1.2548224121208582, 0.6352429012560402),
                point2(1.3489034470987185, 0.6012739292011288),
                point2(1.4213037554602923, 0.6261652440563298),
                point2(1.4743145534688815, 0.7882156278963534),
                point2(1.4700412486188879, 0.8875515790754055),
                point2(1.4460278644836544, 0.8456185823210882),
                point2(1.4868741833172523, 0.9079833740096543),
                point2(1.4920518492387598, 0.9095078637143422),
                point2(1.5595453417691338, 0.9901532598343071),
                point2(1.5936742539308373, 1.0213282325299586),
                point2(1.58058656655406, 1.17305000017827),
                point2(1.4480254616492774, 1.0928279018210438),
                point2(1.4539640114348549, 1.144388265648967),
                point2(1.3648317202407696, 1.2212937832283584),
                point2(1.4733929772805416, 1.2083669884937012),
                point2(1.3608398097214693, 1.3207579529041924),
                point2(1.2912648851735424, 1.4205716705529399),
                point2(1.2046625302840053, 1.3826569437709715),
                point2(1.2570442078920254, 1.3410441079145428),
                point2(1.1830529369693072, 1.3820810903226886),
                point2(1.167072937591176, 1.4466053111301487),
                point2(1.0852661499741054, 1.55951044347548),
                point2(1.0494466853794846, 1.5479828315241733),
                point2(1.0033271419673007, 1.468194659125039),
                point2(0.9484890618160645, 1.4530640355956308),
                point2(0.9973592789218273, 1.45324593604413),
                point2(0.97187677594751, 1.5815631933148016),
                point2(0.8144204755613362, 1.3782837410393232),
                point2(0.7950961543969257, 1.444409277208105),
                point2(0.8249520184490917, 1.3374139622566115),
                point2(0.6758412677442227, 1.334436082917169),
                point2(0.64368867956175, 1.3618188433767497),
                point2(0.5445160170270017, 1.2507819758003385),
                point2(0.5261266184295889, 1.290024044761643),
                point2(0.502690056479149, 1.236879918084129),
                point2(0.5280669233268998, 1.1091277406960698),
                point2(0.4827538350322879, 1.1436496314081661),
                point2(0.5883382268183734, 1.175168641400803),
                point2(0.44736030622371087, 1.018503357084688),
                point2(0.5448981202541112, 0.9143727174667883),
                point2(0.4317760080261111, 1.051488996664834),
                point2(0.5085207904485967, 0.9331170328373988),
                point2(0.6001478439304737, 0.8979301783503268),
                point2(0.4373488434812126, 0.723669324069054),
                point2(0.48379460068391017, 0.6896668054813503),
                point2(0.5802149727260961, 0.6326489019654757),
                point2(0.5318481024591232, 0.6672519961193484),
                point2(0.6267954168946062, 0.6264453502200538),
                point2(0.7244414827901777, 0.6742999823788176),
                point2(0.7409838872007461, 0.5515230198623486),
                point2(0.7461775341290393, 0.6232380086449496),
                point2(0.9055915299113261, 0.5326254191949538),
                point2(0.9510466807539406, 0.49366667559390653),
                point2(0.8116223593436764, 0.4695463357704083),
                point2(0.8528118040757474, 0.4635000250267341),
                point2(0.9141212396595003, 0.40460067972212826),
                point2(1.003267583900141, 0.5351889587671019),
                point2(1.0320189898300267, 0.6060923051759772),
                point2(1.0784925820514744, 0.5016457530039365),
            ],
            vec![
                point2(0.9888360341310736, 0.539884571860436),
                point2(1.0423582717058324, 0.48447611636004245),
                point2(1.1323647757131408, 0.48734422393942145),
                point2(1.097114022520837, 0.5024772415343248),
                point2(1.1983573886194598, 0.6344444071433158),
                point2(1.2951674832143851, 0.641832264291391),
                point2(1.3536023670520665, 0.6251662974163592),
                point2(1.2548224121208582, 0.6352429012560402),
                point2(1.3489034470987185, 0.6012739292011288),
                point2(1.4213037554602923, 0.6261652440563298),
                point2(1.4743145534688815, 0.7882156278963534),
                point2(1.4700412486188879, 0.8875515790754055),
                point2(1.4460278644836544, 0.8456185823210882),
                point2(1.4868741833172523, 0.9079833740096543),
                point2(1.4920518492387598, 0.9095078637143422),
                point2(1.5595453417691338, 0.9901532598343071),
                point2(1.5936742539308373, 1.0213282325299586),
                point2(1.58058656655406, 1.17305000017827),
                point2(1.4480254616492774, 1.0928279018210438),
                point2(1.4539640114348549, 1.144388265648967),
                point2(1.3648317202407696, 1.2212937832283584),
                point2(1.4733929772805416, 1.2083669884937012),
                point2(1.3608398097214693, 1.3207579529041924),
                point2(1.2912648851735424, 1.4205716705529399),
                point2(1.2046625302840053, 1.3826569437709715),
                point2(1.2570442078920254, 1.3410441079145428),
                point2(1.1830529369693072, 1.3820810903226886),
                point2(1.167072937591176, 1.4466053111301487),
                point2(1.0852661499741054, 1.55951044347548),
                point2(1.0494466853794846, 1.5479828315241733),
                point2(1.0033271419673007, 1.468194659125039),
                point2(0.9484890618160645, 1.4530640355956308),
                point2(0.9973592789218273, 1.45324593604413),
                point2(0.97187677594751, 1.5815631933148016),
                point2(0.8144204755613362, 1.3782837410393232),
                point2(0.7950961543969257, 1.444409277208105),
                point2(0.8249520184490917, 1.3374139622566115),
                point2(0.6758412677442227, 1.334436082917169),
                point2(0.64368867956175, 1.3618188433767497),
                point2(0.5445160170270017, 1.2507819758003385),
                point2(0.5261266184295889, 1.290024044761643),
                point2(0.502690056479149, 1.236879918084129),
                point2(0.5280669233268998, 1.1091277406960698),
                point2(0.4827538350322879, 1.1436496314081661),
                point2(0.5883382268183734, 1.175168641400803),
                point2(0.44736030622371087, 1.018503357084688),
                point2(0.5448981202541112, 0.9143727174667883),
                point2(0.4317760080261111, 1.051488996664834),
                point2(0.5085207904485967, 0.9331170328373988),
                point2(0.6001478439304737, 0.8979301783503268),
                point2(0.4373488434812126, 0.723669324069054),
                point2(0.48379460068391017, 0.6896668054813503),
                point2(0.5802149727260961, 0.6326489019654757),
                point2(0.5318481024591232, 0.6672519961193484),
                point2(0.6267954168946062, 0.6264453502200538),
                point2(0.7244414827901777, 0.6742999823788176),
                point2(0.7409838872007461, 0.5515230198623486),
                point2(0.7461775341290393, 0.6232380086449496),
                point2(0.9055915299113261, 0.5326254191949538),
                point2(0.9510466807539406, 0.49366667559390653),
                point2(0.8116223593436764, 0.4695463357704083),
                point2(0.8528118040757474, 0.4635000250267341),
                point2(0.9141212396595003, 0.40460067972212826),
            ],
        ];
        let result = _compute_ellipse_points(
            EllipsePointsSpec {
                increment: 0.1,
                cx: 1.0,
                cy: 1.0,
                rx: 0.5,
                ry: 0.5,
                offset: 0.1,
                overlap: 0.1,
            },
            &mut get_default_options(),
        );
        assert_eq!(expected, result);
    }

    #[test]
    fn curve() {
        let result = _curve(
            &[
                point2(0.0, 0.0),
                point2(1.0, 1.0),
                point2(2.0, 0.0),
                point2(-1.0, -1.0),
            ],
            None,
            &mut get_default_options(),
        );
        assert_eq!(result[0].op, OpType::Move);
        assert_eq!(result[0].data, vec![1.0, 1.0]);

        assert_eq!(result[1].op, OpType::BCurveTo);
        assert_eq!(
            result[1].data,
            vec![
                1.3333333333333333,
                1.0,
                2.3333333333333335,
                0.3333333333333333,
                2.0,
                0.0
            ]
        );
    }
}
