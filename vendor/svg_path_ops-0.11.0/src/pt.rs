use std::collections::{HashMap, VecDeque};

use cgmath::num_traits::Pow;
use cgmath::{Angle, Deg, Matrix3, Rad, Vector2, Vector3};
use svgtypes::{PathParser, PathSegment, TransformListParser, TransformListToken};

use super::ellipse::Ellipse;
use crate::a2c::a2c;
use crate::bbox::{BBox, InboxParameters};

#[derive(Clone)]
pub struct PathTransformer {
    path_segments: VecDeque<PathSegment>,
    stack: Vec<Matrix3<f64>>,
}

impl PathTransformer {
    pub fn new(path: String) -> Self {
        let path_parser = PathParser::from(path.as_ref());
        // if path_parser.any(|a| a.is_err()) {
        //     panic!("unexpected path string. can not parse it.")
        // }

        PathTransformer {
            path_segments: path_parser
                .filter(|ps| ps.is_ok())
                .map(|ps| ps.unwrap())
                .collect(),
            stack: Vec::new(),
        }
    }

    pub fn translate(&mut self, tx: f64, ty: f64) -> &mut Self {
        self.stack
            .push(Matrix3::from_translation(Vector2::new(tx, ty)));
        self
    }

    pub fn scale(&mut self, sx: f64, sy: f64) -> &mut Self {
        self.stack.push(Matrix3::from_nonuniform_scale(sx, sy));
        self
    }

    pub fn rotate(&mut self, angle: f64, rx: f64, ry: f64) -> &mut Self {
        if angle != 0.0 {
            self.translate(-rx, -ry);
            let rad = Rad::from(Deg(angle));
            self.matrix([rad.cos(), rad.sin(), -rad.sin(), rad.cos(), 0.0, 0.0]);
            self.translate(rx, ry);
        }
        self
    }

    pub fn skew_x(&mut self, degrees: f64) -> &mut Self {
        let rad = Rad::from(Deg(degrees));
        self.matrix([1.0, 0.0, rad.tan(), 1.0, 0.0, 0.0]);
        self
    }
    pub fn skew_y(&mut self, degrees: f64) -> &mut Self {
        let rad = Rad::from(Deg(degrees));
        self.matrix([1.0, rad.tan(), 0.0, 1.0, 0.0, 0.0]);
        self
    }

    pub fn matrix(&mut self, matrix: [f64; 6]) -> &mut Self {
        let converted = Matrix3::new(
            matrix[0], matrix[1], 0.0, matrix[2], matrix[3], 0.0, matrix[4], matrix[5], 1.0,
        );
        self.stack.push(converted);
        self
    }

    pub fn transform(&mut self, transform: String) -> &mut Self {
        let parser = TransformListParser::from(transform.as_str());
        let transforms = parser.into_iter().collect::<Vec<_>>();
        for path_transform in transforms.iter().rev() {
            match path_transform {
                Ok(pt) => {
                    self.apply_token(*pt);
                }
                Err(_) => {
                    panic!("Can not parse transform string.");
                }
            }
        }
        self
    }

    fn apply_token(&mut self, token: TransformListToken) -> &mut Self {
        match token {
            TransformListToken::Matrix { a, b, c, d, e, f } => self.matrix([a, b, c, d, e, f]),
            TransformListToken::Translate { tx, ty } => self.translate(tx, ty),
            TransformListToken::Scale { sx, sy } => self.scale(sx, sy),
            TransformListToken::Rotate { angle } => self.rotate(angle, 0.0, 0.0),
            TransformListToken::SkewX { angle } => self.skew_x(angle),
            TransformListToken::SkewY { angle } => self.skew_y(angle),
        };

        self
    }

    fn evaluate_stack(&mut self) -> &mut Self {
        if self.stack.len() == 0 {
            return self;
        } else {
            if self.stack.len() == 1 {
                let single_transformation = self.stack.pop().expect("empty transformation stack");
                self.apply_matrix(single_transformation);
                return self;
            } else {
                let mut combined = Matrix3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
                while !self.stack.is_empty() {
                    combined = combined
                        * self
                            .stack
                            .pop()
                            .expect("can not find transformation matrix");
                }
                self.apply_matrix(combined);
                return self;
            }
        }
    }

    fn apply_matrix(&mut self, final_matrix: Matrix3<f64>) -> &mut Self {
        self.iterate(|segment, pos, x, y| {
            let result: PathSegment;
            match *segment {
                PathSegment::MoveTo { abs, x: seg_x, y: seg_y } => {
                    if abs {
                        let p = final_matrix * Vector3::new(seg_x, seg_y, 1.0);
                        result = PathSegment::MoveTo { abs: true, x: p.x, y: p.y }
                    } else {
                        // Edge case. The very first `m` should be processed as absolute, if happens.
                        // Make sense for coord shift transforms.
                        let is_relative = pos > 0;
                        let p = final_matrix
                            * Vector3::new(seg_x, seg_y, if is_relative { 0.0 } else { 1.0 });

                        result = PathSegment::MoveTo { abs: !is_relative, x: p.x, y: p.y };
                    }
                }
                PathSegment::LineTo { abs, x: seg_x, y: seg_y } => {
                    let p = final_matrix * Vector3::new(seg_x, seg_y, if abs { 1.0 } else { 0.0 });
                    result = PathSegment::LineTo { abs, x: p.x, y: p.y }
                }
                PathSegment::HorizontalLineTo { abs, x: seg_x } => {
                    if abs {
                        let p = final_matrix * Vector3::new(seg_x, y, 1.0);
                        result = if p.y == (final_matrix * Vector3::new(x, y, 1.0)).y {
                            PathSegment::HorizontalLineTo { abs: true, x: p.x }
                        } else {
                            PathSegment::LineTo { abs: true, x: p.x, y: p.y }
                        }
                    } else {
                        let p = final_matrix * Vector3::new(seg_x, 0.0, 0.0);

                        result = if p.y == 0.0 {
                            PathSegment::HorizontalLineTo { abs: false, x: p.x }
                        } else {
                            PathSegment::LineTo { abs: false, x: p.x, y: p.y }
                        };
                    }
                }
                PathSegment::VerticalLineTo { abs, y: seg_y } => {
                    if abs {
                        let p = final_matrix * Vector3::new(x, seg_y, 1.0);
                        result = if p.x == (final_matrix * Vector3::new(x, y, 1.0)).x {
                            PathSegment::VerticalLineTo { abs: true, y: p.y }
                        } else {
                            PathSegment::LineTo { abs: true, x: p.x, y: p.y }
                        };
                    } else {
                        let p = final_matrix * Vector3::new(0.0, seg_y, 0.0);
                        result = if p.x == 0.0 {
                            PathSegment::VerticalLineTo { abs: false, y: p.y }
                        } else {
                            PathSegment::LineTo { abs: false, x: p.x, y: p.y }
                        };
                    }
                }
                PathSegment::CurveTo { abs, x1, y1, x2, y2, x: seg_x, y: seg_y } => {
                    let p1 = final_matrix * Vector3::new(x1, y1, if abs { 1.0 } else { 0.0 });
                    let p2 = final_matrix * Vector3::new(x2, y2, if abs { 1.0 } else { 0.0 });
                    let p3 = final_matrix * Vector3::new(seg_x, seg_y, if abs { 1.0 } else { 0.0 });
                    result = PathSegment::CurveTo {
                        abs,
                        x1: p1.x,
                        y1: p1.y,
                        x2: p2.x,
                        y2: p2.y,
                        x: p3.x,
                        y: p3.y,
                    };
                }
                PathSegment::SmoothCurveTo { abs, x2, y2, x: seg_x, y: seg_y } => {
                    let p2 = final_matrix * Vector3::new(x2, y2, if abs { 1.0 } else { 0.0 });
                    let p3 = final_matrix * Vector3::new(seg_x, seg_y, if abs { 1.0 } else { 0.0 });
                    result =
                        PathSegment::SmoothCurveTo { abs, x2: p2.x, y2: p2.y, x: p3.x, y: p3.y };
                }
                PathSegment::Quadratic { abs, x1, y1, x: seg_x, y: seg_y } => {
                    let p1 = final_matrix * Vector3::new(x1, y1, if abs { 1.0 } else { 0.0 });
                    let p2 = final_matrix * Vector3::new(seg_x, seg_y, if abs { 1.0 } else { 0.0 });
                    result = PathSegment::Quadratic { abs, x1: p1.x, y1: p1.y, x: p2.x, y: p2.y };
                }
                PathSegment::SmoothQuadratic { abs, x: seg_x, y: seg_y } => {
                    let p2 = final_matrix * Vector3::new(seg_x, seg_y, if abs { 1.0 } else { 0.0 });
                    result = PathSegment::SmoothQuadratic { abs, x: p2.x, y: p2.y }
                }

                PathSegment::EllipticalArc {
                    abs,
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc,
                    sweep,
                    x: seg_x,
                    y: seg_y,
                } => {
                    let mut sweep = sweep;
                    // Transform rx, ry and the x-axis-rotation
                    // var ma = m.toArray();
                    let mut e = Ellipse::new(rx, ry, x_axis_rotation);
                    e.transform(&[
                        final_matrix[0][0],
                        final_matrix[0][1],
                        final_matrix[1][0],
                        final_matrix[1][1],
                    ]);

                    // flip sweep-flag if matrix is not orientation-preserving
                    if final_matrix[0][0] * final_matrix[1][1]
                        - final_matrix[0][1] * final_matrix[1][0]
                        < 0.0
                    {
                        sweep = if sweep { false } else { true };
                    }

                    // Transform end point as usual (without translation for relative notation)
                    let p = final_matrix * Vector3::new(seg_x, seg_y, if abs { 1.0 } else { 0.0 });

                    // Empty arcs can be ignored by renderer, but should not be dropped
                    // to avoid collisions with `S A S` and so on. Replace with empty line.
                    if (abs && seg_x == x && seg_y == y) || (!abs && seg_x == 0.0 && seg_y == 0.0) {
                        result = PathSegment::LineTo { abs, x: p.x, y: p.y };
                    } else {
                        // if the resulting ellipse is (almost) a segment ...
                        if e.is_degenrate() {
                            // replace the arc by a line
                            result = PathSegment::LineTo { abs, x: p.x, y: p.y };
                        } else {
                            // if it is a real ellipse
                            // s[0], s[4] and s[5] are not modified
                            result = PathSegment::EllipticalArc {
                                abs,
                                rx: e.rx,
                                ry: e.ry,
                                x_axis_rotation: e.ax.0,
                                large_arc,
                                sweep,
                                x: p.x,
                                y: p.y,
                            }
                        }
                    }
                }
                PathSegment::ClosePath { abs } => result = PathSegment::ClosePath { abs },
            };
            vec![result]
        });
        return self;
    }

    fn to_fixed(input: f64, d: u8) -> f64 {
        (input * 10.0f64.pow(d)).round() / 10.0f64.pow(d)
    }

    pub fn round(&mut self, d: u8) -> &mut Self {
        let mut contour_start_delta_x = 0.0;
        let mut contour_start_delta_y = 0.0;
        let mut delta_x = 0.0;
        let mut delta_y = 0.0;

        self.evaluate_stack();

        self.iterate(|segment, _pos, _x, _y| match segment {
            PathSegment::HorizontalLineTo { abs, x: seg_x } => {
                let mut seg_x = *seg_x;
                if !abs {
                    seg_x += delta_x;
                }
                let rounded = Self::to_fixed(seg_x, d);
                delta_x = seg_x - rounded;
                seg_x = rounded;
                vec![PathSegment::HorizontalLineTo { abs: *abs, x: seg_x }]
            }
            PathSegment::VerticalLineTo { abs, y: seg_y } => {
                let mut seg_y = *seg_y;
                if !abs {
                    seg_y += delta_y;
                }
                let rounded = Self::to_fixed(seg_y, d);
                delta_y = seg_y - rounded;
                seg_y = rounded;
                vec![PathSegment::VerticalLineTo { abs: *abs, y: seg_y }]
            }
            PathSegment::ClosePath { abs: _ } => {
                delta_x = contour_start_delta_x;
                delta_y = contour_start_delta_y;
                vec![]
            }

            PathSegment::MoveTo { abs, x: seg_x, y: seg_y } => {
                let mut seg_x = *seg_x;
                let mut seg_y = *seg_y;

                if !abs {
                    seg_x += delta_x;
                    seg_y += delta_y;
                }
                delta_x = seg_x - Self::to_fixed(seg_x, d);
                delta_y = seg_y - Self::to_fixed(seg_y, d);

                contour_start_delta_x = delta_x;
                contour_start_delta_y = delta_y;

                seg_x = Self::to_fixed(seg_x, d);
                seg_y = Self::to_fixed(seg_y, d);

                vec![PathSegment::MoveTo { abs: *abs, x: seg_x, y: seg_y }]
            }
            PathSegment::EllipticalArc {
                abs,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x: seg_x,
                y: seg_y,
            } => {
                // [cmd, rx, ry, x-axis-rotation, large-arc-flag, sweep-flag, x, y]
                let mut seg_x = *seg_x;
                let mut seg_y = *seg_y;
                let mut rx = *rx;
                let mut ry = *ry;
                let mut x_axis_rotation = *x_axis_rotation;

                if !*abs {
                    seg_x += delta_x;
                    seg_y += delta_y;
                }

                delta_x = seg_x - Self::to_fixed(seg_x, d);
                delta_y = seg_y - Self::to_fixed(seg_y, d);

                rx = Self::to_fixed(rx, d);
                ry = Self::to_fixed(ry, d);
                x_axis_rotation = Self::to_fixed(x_axis_rotation, d + 2);
                seg_x = Self::to_fixed(seg_x, d);
                seg_y = Self::to_fixed(seg_y, d);
                vec![PathSegment::EllipticalArc {
                    abs: *abs,
                    rx: rx,
                    ry: ry,
                    x_axis_rotation: x_axis_rotation,
                    large_arc: *large_arc,
                    sweep: *sweep,
                    x: seg_x,
                    y: seg_y,
                }]
            }
            PathSegment::LineTo { abs, x: seg_x, y: seg_y } => {
                let mut seg_x = *seg_x;
                let mut seg_y = *seg_y;

                if !abs {
                    seg_x += delta_x;
                    seg_y += delta_y;
                }

                delta_x = seg_x - Self::to_fixed(seg_x, d);
                delta_y = seg_y - Self::to_fixed(seg_y, d);

                seg_x = Self::to_fixed(seg_x, d);
                seg_y = Self::to_fixed(seg_y, d);
                vec![PathSegment::LineTo { abs: *abs, x: seg_x, y: seg_y }]
            }
            PathSegment::CurveTo { abs, x1, y1, x2, y2, x: seg_x, y: seg_y } => {
                let mut seg_x = *seg_x;
                let mut seg_y = *seg_y;
                let mut x1 = *x1;
                let mut y1 = *y1;
                let mut x2 = *x2;
                let mut y2 = *y2;

                if !abs {
                    seg_x += delta_x;
                    seg_y += delta_y;
                }
                delta_x = seg_x - Self::to_fixed(seg_x, d);
                delta_y = seg_y - Self::to_fixed(seg_y, d);

                seg_x = Self::to_fixed(seg_x, d);
                seg_y = Self::to_fixed(seg_y, d);
                x1 = Self::to_fixed(x1, d);
                y1 = Self::to_fixed(y1, d);
                x2 = Self::to_fixed(x2, d);
                y2 = Self::to_fixed(y2, d);
                Vec::from([PathSegment::CurveTo { abs: *abs, x1, y1, x2, y2, x: seg_x, y: seg_y }])
            }
            PathSegment::SmoothCurveTo { abs, x2, y2, x: seg_x, y: seg_y } => {
                let mut seg_x = *seg_x;
                let mut seg_y = *seg_y;
                let mut x2 = *x2;
                let mut y2 = *y2;

                if !abs {
                    seg_x += delta_x;
                    seg_y += delta_y;
                }
                delta_x = seg_x - Self::to_fixed(seg_x, d);
                delta_y = seg_y - Self::to_fixed(seg_y, d);

                seg_x = Self::to_fixed(seg_x, d);
                seg_y = Self::to_fixed(seg_y, d);
                x2 = Self::to_fixed(x2, d);
                y2 = Self::to_fixed(y2, d);
                Vec::from([PathSegment::SmoothCurveTo { abs: *abs, x2, y2, x: seg_x, y: seg_y }])
            }
            PathSegment::Quadratic { abs, x1, y1, x: seg_x, y: seg_y } => {
                let mut seg_x = *seg_x;
                let mut seg_y = *seg_y;
                let mut x1 = *x1;
                let mut y1 = *y1;

                if !abs {
                    seg_x += delta_x;
                    seg_y += delta_y;
                }
                delta_x = seg_x - Self::to_fixed(seg_x, d);
                delta_y = seg_y - Self::to_fixed(seg_y, d);

                seg_x = Self::to_fixed(seg_x, d);
                seg_y = Self::to_fixed(seg_y, d);
                x1 = Self::to_fixed(x1, d);
                y1 = Self::to_fixed(y1, d);
                Vec::from([PathSegment::Quadratic { abs: *abs, x1, y1, x: seg_x, y: seg_y }])
            }
            PathSegment::SmoothQuadratic { abs, x: seg_x, y: seg_y } => {
                let mut seg_x = *seg_x;
                let mut seg_y = *seg_y;

                if !abs {
                    seg_x += delta_x;
                    seg_y += delta_y;
                }
                delta_x = seg_x - Self::to_fixed(seg_x, d);
                delta_y = seg_y - Self::to_fixed(seg_y, d);

                seg_x = Self::to_fixed(seg_x, d);
                seg_y = Self::to_fixed(seg_y, d);
                Vec::from([PathSegment::SmoothQuadratic { abs: *abs, x: seg_x, y: seg_y }])
            }
        });
        self
    }

    fn iterate<F>(&mut self, mut func: F) -> &mut Self
    where
        F: FnMut(&PathSegment, usize, f64, f64) -> Vec<PathSegment>,
    {
        let mut last_x: f64 = 0.0;
        let mut last_y: f64 = 0.0;
        let mut contour_start_x: f64 = 0.0;
        let mut contour_start_y: f64 = 0.0;
        let mut replacements = HashMap::new();

        for (pos, segment) in self.path_segments.iter().enumerate() {
            let transformation_result = func(segment, pos, last_x, last_y);

            if !transformation_result.is_empty() {
                replacements.insert(pos, transformation_result);
            }

            match segment {
                PathSegment::MoveTo { abs, x, y } => {
                    last_x = x + if *abs { 0.0 } else { last_x };
                    last_y = y + if *abs { 0.0 } else { last_y };
                    contour_start_x = last_x;
                    contour_start_y = last_y;
                }
                PathSegment::LineTo { abs, x, y } => {
                    last_x = x + if *abs { 0.0 } else { last_x };
                    last_y = y + if *abs { 0.0 } else { last_y };
                }
                PathSegment::HorizontalLineTo { abs, x } => {
                    last_x = x + if *abs { 0.0 } else { last_x };
                }
                PathSegment::VerticalLineTo { abs, y } => {
                    last_y = y + if *abs { 0.0 } else { last_y };
                }
                PathSegment::CurveTo { abs, x1: _, y1: _, x2: _, y2: _, x, y } => {
                    last_x = x + if *abs { 0.0 } else { last_x };
                    last_y = y + if *abs { 0.0 } else { last_y };
                }
                PathSegment::SmoothCurveTo { abs, x2: _, y2: _, x, y } => {
                    last_x = x + if *abs { 0.0 } else { last_x };
                    last_y = y + if *abs { 0.0 } else { last_y };
                }
                PathSegment::Quadratic { abs, x1: _, y1: _, x, y } => {
                    last_x = x + if *abs { 0.0 } else { last_x };
                    last_y = y + if *abs { 0.0 } else { last_y };
                }
                PathSegment::SmoothQuadratic { abs, x, y } => {
                    last_x = x + if *abs { 0.0 } else { last_x };
                    last_y = y + if *abs { 0.0 } else { last_y };
                }
                PathSegment::EllipticalArc {
                    abs,
                    rx: _,
                    ry: _,
                    x_axis_rotation: _,
                    large_arc: _,
                    sweep: _,
                    x,
                    y,
                } => {
                    last_x = x + if *abs { 0.0 } else { last_x };
                    last_y = y + if *abs { 0.0 } else { last_y };
                }
                PathSegment::ClosePath { abs: _ } => {
                    last_x = contour_start_x;
                    last_y = contour_start_y;
                }
            }
        }

        if replacements.len() == 0 {
            return self;
        } else {
            let mut updated_segments = VecDeque::new();
            for i in 0..self.path_segments.len() {
                if replacements.contains_key(&i) {
                    let replacing_segments =
                        replacements.get(&i).expect("can not retrieve replacement");
                    replacing_segments
                        .iter()
                        .for_each(|r| updated_segments.push_back(*r));
                } else {
                    updated_segments.push_back(
                        *self
                            .path_segments
                            .get(i)
                            .expect("can not retrieve path segment"),
                    );
                }
            }

            self.path_segments = updated_segments;

            return self;
        }
    }

    pub fn to_string(&mut self) -> String {
        self.evaluate_stack();

        self.path_segments
            .iter()
            .map(|a| PathTransformer::to_string_segment(a))
            .reduce(|segment1, segment2| format!("{} {}", segment1, segment2))
            .expect("can not convert to string")
    }

    fn to_string_segment(segment: &PathSegment) -> String {
        match segment {
            PathSegment::MoveTo { abs, x, y } => {
                if *abs {
                    format!("M {} {}", x, y)
                } else {
                    format!("m {} {}", x, y)
                }
            }
            PathSegment::LineTo { abs, x, y } => {
                if *abs {
                    format!("L {} {}", x, y)
                } else {
                    format!("l {} {}", x, y)
                }
            }
            PathSegment::HorizontalLineTo { abs, x } => {
                if *abs {
                    format!("H {}", x)
                } else {
                    format!("h {}", x)
                }
            }
            PathSegment::VerticalLineTo { abs, y } => {
                if *abs {
                    format!("V {}", y)
                } else {
                    format!("v {}", y)
                }
            }
            PathSegment::CurveTo { abs, x1, y1, x2, y2, x, y } => {
                if *abs {
                    format!("C {} {} {} {} {} {}", x1, y1, x2, y2, x, y)
                } else {
                    format!("c {} {} {} {} {} {}", x1, y1, x2, y2, x, y)
                }
            }
            PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                if *abs {
                    format!("S {} {} {} {}", x2, y2, x, y)
                } else {
                    format!("s {} {} {} {}", x2, y2, x, y)
                }
            }
            PathSegment::Quadratic { abs, x1, y1, x, y } => {
                if *abs {
                    format!("Q {} {} {} {}", x1, y1, x, y)
                } else {
                    format!("q {} {} {} {}", x1, y1, x, y)
                }
            }
            PathSegment::SmoothQuadratic { abs, x, y } => {
                if *abs {
                    format!("T {} {}", x, y)
                } else {
                    format!("t {} {}", x, y)
                }
            }
            PathSegment::EllipticalArc {
                abs,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
            } => {
                if *abs {
                    format!(
                        "A {} {} {} {} {} {} {}",
                        rx, ry, x_axis_rotation, *large_arc as i32, *sweep as i32, x, y
                    )
                } else {
                    format!(
                        "a {} {} {} {} {} {} {}",
                        rx, ry, x_axis_rotation, *large_arc as i32, *sweep as i32, x, y
                    )
                }
            }
            PathSegment::ClosePath { abs } => {
                if *abs {
                    format!("Z")
                } else {
                    format!("z")
                }
            }
        }
    }

    pub fn abs(&mut self) -> &mut Self {
        self.iterate(|s, _, x, y| match s {
            PathSegment::MoveTo { abs, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![]
                } else {
                    vec![PathSegment::MoveTo { abs: true, x: seg_x + x, y: seg_y + y }]
                }
            }
            PathSegment::LineTo { abs, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![]
                } else {
                    vec![PathSegment::LineTo { abs: true, x: seg_x + x, y: seg_y + y }]
                }
            }
            PathSegment::HorizontalLineTo { abs, x: seg_x } => {
                if *abs {
                    vec![]
                } else {
                    vec![PathSegment::HorizontalLineTo { abs: true, x: seg_x + x }]
                }
            }
            PathSegment::VerticalLineTo { abs, y: seg_y } => {
                if *abs {
                    vec![]
                } else {
                    vec![PathSegment::VerticalLineTo { abs: true, y: seg_y + y }]
                }
            }
            PathSegment::CurveTo { abs, x1, y1, x2, y2, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![]
                } else {
                    vec![PathSegment::CurveTo {
                        abs: true,
                        x1: x1 + x,
                        y1: y1 + y,
                        x2: x2 + x,
                        y2: y2 + y,
                        x: seg_x + x,
                        y: seg_y + y,
                    }]
                }
            }
            PathSegment::SmoothCurveTo { abs, x2, y2, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![]
                } else {
                    vec![PathSegment::SmoothCurveTo {
                        abs: true,
                        x2: x2 + x,
                        y2: y2 + y,
                        x: seg_x + x,
                        y: seg_y + y,
                    }]
                }
            }
            PathSegment::Quadratic { abs, x1, y1, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![]
                } else {
                    vec![PathSegment::Quadratic {
                        abs: true,
                        x1: x1 + x,
                        y1: y1 + y,
                        x: seg_x + x,
                        y: seg_y + y,
                    }]
                }
            }
            PathSegment::SmoothQuadratic { abs, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![]
                } else {
                    vec![PathSegment::SmoothQuadratic { abs: true, x: seg_x + x, y: seg_y + y }]
                }
            }
            PathSegment::EllipticalArc {
                abs,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x: seg_x,
                y: seg_y,
            } => {
                if *abs {
                    vec![]
                } else {
                    vec![PathSegment::EllipticalArc {
                        abs: true,
                        rx: *rx,
                        ry: *ry,
                        x_axis_rotation: *x_axis_rotation,
                        large_arc: *large_arc,
                        sweep: *sweep,
                        x: seg_x + x,
                        y: seg_y + y,
                    }]
                }
            }
            PathSegment::ClosePath { abs } => {
                if *abs {
                    vec![]
                } else {
                    vec![PathSegment::ClosePath { abs: true }]
                }
            }
        });

        self
    }

    pub fn rel(&mut self) -> &mut Self {
        self.iterate(|s, pos, x, y| match s {
            PathSegment::MoveTo { abs, x: seg_x, y: seg_y } => {
                if *abs {
                    if pos == 0 {
                        vec![]
                    } else {
                        vec![PathSegment::MoveTo { abs: false, x: seg_x - x, y: seg_y - y }]
                    }
                } else {
                    vec![]
                }
            }
            PathSegment::LineTo { abs, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![PathSegment::LineTo { abs: false, x: seg_x - x, y: seg_y - y }]
                } else {
                    vec![]
                }
            }
            PathSegment::HorizontalLineTo { abs, x: seg_x } => {
                if *abs {
                    vec![PathSegment::HorizontalLineTo { abs: false, x: seg_x - x }]
                } else {
                    vec![]
                }
            }
            PathSegment::VerticalLineTo { abs, y: seg_y } => {
                if *abs {
                    vec![PathSegment::VerticalLineTo { abs: false, y: seg_y - y }]
                } else {
                    vec![]
                }
            }
            PathSegment::CurveTo { abs, x1, y1, x2, y2, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![PathSegment::CurveTo {
                        abs: false,
                        x1: x1 - x,
                        y1: y1 - y,
                        x2: x2 - x,
                        y2: y2 - y,
                        x: seg_x - x,
                        y: seg_y - y,
                    }]
                } else {
                    vec![]
                }
            }
            PathSegment::SmoothCurveTo { abs, x2, y2, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![PathSegment::SmoothCurveTo {
                        abs: false,
                        x2: x2 - x,
                        y2: y2 - y,
                        x: seg_x - x,
                        y: seg_y - y,
                    }]
                } else {
                    vec![]
                }
            }
            PathSegment::Quadratic { abs, x1, y1, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![PathSegment::Quadratic {
                        abs: false,
                        x1: x1 - x,
                        y1: y1 - y,
                        x: seg_x - x,
                        y: seg_y - y,
                    }]
                } else {
                    vec![]
                }
            }
            PathSegment::SmoothQuadratic { abs, x: seg_x, y: seg_y } => {
                if *abs {
                    vec![PathSegment::SmoothQuadratic { abs: false, x: seg_x - x, y: seg_y - y }]
                } else {
                    vec![]
                }
            }
            PathSegment::EllipticalArc {
                abs,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x: seg_x,
                y: seg_y,
            } => {
                if *abs {
                    vec![PathSegment::EllipticalArc {
                        abs: false,
                        rx: *rx,
                        ry: *ry,
                        x_axis_rotation: *x_axis_rotation,
                        large_arc: *large_arc,
                        sweep: *sweep,
                        x: seg_x - x,
                        y: seg_y - y,
                    }]
                } else {
                    vec![]
                }
            }
            PathSegment::ClosePath { abs } => {
                if *abs {
                    vec![PathSegment::ClosePath { abs: false }]
                } else {
                    vec![]
                }
            }
        })
    }

    pub fn unarc(&mut self) -> &mut Self {
        self.iterate(|segment, _, x, y| {
            let next_x: f64;
            let next_y: f64;

            match *segment {
                PathSegment::EllipticalArc {
                    abs,
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc,
                    sweep,
                    x: seg_x,
                    y: seg_y,
                } => {
                    if !abs {
                        next_x = seg_x + x;
                        next_y = seg_y + y;
                    } else {
                        next_x = seg_x;
                        next_y = seg_y;
                    }
                    let new_segments = a2c(
                        x,
                        y,
                        next_x,
                        next_y,
                        large_arc,
                        sweep,
                        rx,
                        ry,
                        x_axis_rotation,
                    );

                    if new_segments.len() == 0 {
                        vec![PathSegment::LineTo { abs: abs, x: seg_x, y: seg_y }]
                    } else {
                        new_segments
                            .iter()
                            .map(|s| PathSegment::CurveTo {
                                abs: true,
                                x1: s[2],
                                y1: s[3],
                                x2: s[4],
                                y2: s[5],
                                x: s[6],
                                y: s[7],
                            })
                            .collect()
                    }
                }
                _ => {
                    vec![]
                }
            }
        });
        self
    }

    pub fn unshort(&mut self) -> &mut Self {
        // var segments = this.segments;
        let mut prev_control_x = 0.0;
        let mut prev_control_y = 0.0;
        let mut prev_segment: PathSegment = PathSegment::MoveTo { abs: false, x: 0.0, y: 0.0 };
        let mut cur_control_x = 0.0;
        let mut cur_control_y = 0.0;
        let segments_cache = self.path_segments.clone();

        // TODO: add lazy evaluation flag when relative commands supported

        self.iterate(|s, idx, x, y| {
            // var name = s[0], nameUC = name.toUpperCase(), isRelative;

            // First command MUST be M|m, it's safe to skip.
            // Protect from access to [-1] for sure.
            if idx == 0 {
                return vec![];
            }

            match *s {
                PathSegment::SmoothQuadratic { abs, x: seg_outer_x, y: seg_outer_y } => {
                    prev_segment = segments_cache[idx - 1];
                    match prev_segment {
                        PathSegment::Quadratic { abs, x1, y1, x: seg_x, y: seg_y } => {
                            if abs {
                                prev_control_x = x1 - x;
                                prev_control_y = y1 - y;
                            } else {
                                prev_control_x = x1 - seg_x;
                                prev_control_y = y1 - seg_y;
                            }
                        }
                        _ => {
                            prev_control_x = 0.0;
                            prev_control_y = 0.0;
                        }
                    };
                    cur_control_x = -prev_control_x;
                    cur_control_y = -prev_control_y;
                    if abs {
                        cur_control_x += x;
                        cur_control_y += y;
                    }
                    vec![PathSegment::Quadratic {
                        abs: abs,
                        x1: cur_control_x,
                        y1: cur_control_y,
                        x: seg_outer_x,
                        y: seg_outer_y,
                    }]
                }
                PathSegment::SmoothCurveTo { abs, x2, y2, x: seg_outer_x, y: seg_outer_y } => {
                    prev_segment = segments_cache[idx - 1];
                    match prev_segment {
                        PathSegment::CurveTo {
                            abs,
                            x1: _,
                            y1: _,
                            x2: seg_x2,
                            y2: seg_y2,
                            x: seg_x,
                            y: seg_y,
                        } => {
                            if abs {
                                prev_control_x = seg_x2 - x;
                                prev_control_y = seg_y2 - y;
                            } else {
                                prev_control_x = seg_x2 - seg_x;
                                prev_control_y = seg_y2 - seg_y;
                            }
                        }
                        _ => {
                            prev_control_x = 0.0;
                            prev_control_y = 0.0;
                        }
                    };

                    cur_control_x = -prev_control_x;
                    cur_control_y = -prev_control_y;

                    if abs {
                        cur_control_x += x;
                        cur_control_y += y;
                    }

                    vec![PathSegment::CurveTo {
                        abs: abs,
                        x1: cur_control_x,
                        y1: cur_control_y,
                        x2: x2,
                        y2: y2,
                        x: seg_outer_x,
                        y: seg_outer_y,
                    }]
                }
                _ => {
                    vec![]
                }
            }
        });
        self
    }

    // Return the bounding box (Box object) of the path path.
    // The path is converted before with .abs().unarc().unshort()
    //
    pub fn to_box(&self, precision: Option<u8>) -> BBox {
        let mut bb = BBox::new();

        if self.path_segments.is_empty() {
            return bb;
        }

        // Create a copy of the path and transform it
        let mut p = self.clone();
        p.abs().unarc().unshort().evaluate_stack();

        // Iterate through segments and update bounding box
        p.iterate(|segment, _pos, x, y| {
            match segment {
                PathSegment::HorizontalLineTo { abs: _, x: seg_x } => {
                    bb.add_x(*seg_x);
                }
                PathSegment::VerticalLineTo { abs: _, y: seg_y } => {
                    bb.add_y(*seg_y);
                }
                PathSegment::MoveTo { abs: _, x: seg_x, y: seg_y }
                | PathSegment::LineTo { abs: _, x: seg_x, y: seg_y } => {
                    bb.add_x(*seg_x);
                    bb.add_y(*seg_y);
                }
                PathSegment::Quadratic { abs: _, x1, y1, x: seg_x, y: seg_y } => {
                    bb.add_x_q(&[x, *x1, *seg_x]);
                    bb.add_y_q(&[y, *y1, *seg_y]);
                }
                PathSegment::CurveTo { abs: _, x1, y1, x2, y2, x: seg_x, y: seg_y } => {
                    bb.add_x_c(&[x, *x1, *x2, *seg_x]);
                    bb.add_y_c(&[y, *y1, *y2, *seg_y]);
                }
                _ => {}
            };
            vec![*segment]
        });

        // Round with the current precision
        if let Some(p) = precision {
            bb.min_x = bb.min_x.map(|x| roundp(x, p));
            bb.min_y = bb.min_y.map(|y| roundp(y, p));
            bb.max_x = bb.max_x.map(|x| roundp(x, p));
            bb.max_y = bb.max_y.map(|y| roundp(y, p));
        }

        bb
    }

    /// translate and scale the path to fit in a box
    /// controlled by the following parameters :
    /// - type:
    ///  - fit(=none) : scale the path (aspect ratio is not preserved) to fit the box
    ///  - meet (the default) : scale the path (aspect ratio is preserved) as much as possible
    ///                         to fit the entire path in the box
    ///  - slice : scale the path (aspect ratio is preserved) as less as possible to cover the box
    ///  - move : translate only (no scale) the path according to x???y??? parameter
    /// - position x(Min|Mid|Max)Y(Min|Mid|Max).
    ///  example : .inbox('-10 10 300 400 meet xMidYMin')
    ///            .inbox(-10, 10, 300, 400, 'meet xMidYMin')
    ///
    pub fn inbox(&mut self, params: InboxParameters) -> &mut Self {
        // Get the transformation matrix from the current bounding box
        let matrix = self.to_box(Some(2)).inbox_matrix(&params);

        // Apply the transformation matrix to the path
        self.matrix(matrix);

        self
    }
}

fn roundp(value: f64, precision: u8) -> f64 {
    let factor = 10.0f64.powi(precision as i32);
    (value * factor).round() / factor
}

#[cfg(test)]
mod test {
    use super::PathTransformer;

    #[test]
    fn basic_translate() {
        let actual = PathTransformer::new("M0 0 L 10 10".into())
            .scale(2.0, 2.0)
            .to_string();

        assert_eq!(actual, "M 0 0 L 20 20")
    }

    #[test]
    fn not_collapse_multiple_abs_m() {
        let actual =
            PathTransformer::new("M 10 10 M 10 100 M 100 100 M 100 10 Z".into()).to_string();
        assert_eq!(actual, "M 10 10 M 10 100 M 100 100 M 100 10 Z");
    }

    #[test]
    #[ignore = "First M fails"]
    fn not_collapse_multiple_rel_m() {
        let actual =
            PathTransformer::new("m 10 10 m 10 100 m 100 100 m 100 10 z".into()).to_string();
        assert_eq!(actual, "M 10 10 m 10 100 m 100 100 m 100 10 z");
    }

    #[test]
    fn scale_abs_curve() {
        let actual = PathTransformer::new("M10 10 C 20 40 40 40 50 10".into())
            .scale(2.0, 1.5)
            .to_string();
        assert_eq!(actual, "M 20 15 C 40 60 80 60 100 15");
    }

    #[test]
    fn scale_rel_curve() {
        let actual = PathTransformer::new("M10 10 c 10 30 30 30 40 0".into())
            .scale(2.0, 1.5)
            .to_string();
        assert_eq!(actual, "M 20 15 c 20 45 60 45 80 0");
    }

    #[test]
    fn scale_horizontal_lines() {
        let actual = PathTransformer::new("M10 10H40h50".into())
            .scale(2.0, 1.5)
            .to_string();
        assert_eq!(actual, "M 20 15 H 80 h 100");
    }

    #[test]
    fn scale_vertical_lines() {
        let actual = PathTransformer::new("M10 10V40v50".into())
            .scale(2.0, 1.5)
            .to_string();
        assert_eq!(actual, "M 20 15 V 60 v 75");
    }

    #[test]
    fn scale_arc_rel() {
        let actual = PathTransformer::new("M40 30a20 40 -45 0 1 20 50".into())
            .scale(2.0, 1.5)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 80 45 a 72 34 32.04 0 1 40 75");
    }

    #[test]
    fn scale_arc_abs() {
        let actual = PathTransformer::new("M40 30A20 40 -45 0 1 20 50".into())
            .scale(2.0, 1.5)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 80 45 A 72 34 32.04 0 1 40 75");
    }

    #[test]
    fn rotate_by_90_degrees_about_point_10_10() {
        let actual = PathTransformer::new("M10 10L15 10".into())
            .rotate(90.0, 10.0, 10.0)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 10 10 L 10 15");
    }

    #[test]
    fn rotate_by_negative_90_degrees_about_point_10_10() {
        let actual = PathTransformer::new("M0 10L0 20".into())
            .rotate(-90.0, 0.0, 0.0)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 10 0 L 20 0");
    }

    #[test]
    fn skew_x() {
        let actual = PathTransformer::new("M5 5L15 20".into())
            .skew_x(75.96)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 25 5 L 95 20");
    }

    #[test]
    fn skew_y() {
        let actual = PathTransformer::new("M5 5L15 20".into())
            .skew_y(75.96)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 5 25 L 15 80");
    }

    #[test]
    fn transform_path_with_absolute_segments() {
        let actual = PathTransformer::new("M5 5 C20 30 10 15 30 15".into())
            .matrix([1.5, 0.5, 0.5, 1.5, 10.0, 15.0])
            .to_string();
        assert_eq!(actual, "M 20 25 C 55 70 32.5 42.5 62.5 52.5");
    }

    #[test]
    pub fn transform_path_with_relative_segments() {
        let actual = PathTransformer::new("M5 5 c10 12 10 15 20 30".into())
            .matrix([1.5, 0.5, 0.5, 1.5, 10.0, 15.0])
            .to_string();
        assert_eq!(actual, "M 20 25 c 21 23 22.5 27.5 45 55");
    }

    #[test]
    pub fn unit_transform() {
        let actual = PathTransformer::new("M5 5 C20 30 10 15 30 15".into())
            .matrix([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
            .to_string();
        assert_eq!(actual, "M 5 5 C 20 30 10 15 30 15");
    }

    #[test]
    pub fn scale_and_translate() {
        let actual = PathTransformer::new("M0 0 L 10 10 20 10".into())
            .scale(2.0, 3.0)
            .translate(100.0, 100.0)
            .to_string();
        assert_eq!(actual, "M 100 100 L 120 130 L 140 130");
    }

    #[test]
    pub fn scale_and_rotate() {
        let actual = PathTransformer::new("M0 0 L 10 10 20 10".into())
            .scale(2.0, 3.0)
            .rotate(90.0, 0.0, 0.0)
            .round(0)
            .to_string();

        assert_eq!(actual, "M 0 0 L -30 20 L -30 40");
    }

    #[test]
    pub fn handle_unit_transforms() {
        let actual = PathTransformer::new("M0 0 L 10 10 20 10".into())
            .translate(0.0, 0.0)
            .scale(1.0, 1.0)
            .rotate(0.0, 10.0, 10.0)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 0 0 L 10 10 L 20 10")
    }

    #[test]
    pub fn translate_abs_curve() {
        let actual = PathTransformer::new("M10 10 C 20 40 40 40 50 10".into())
            .translate(5.0, 15.0)
            .to_string();
        assert_eq!(actual, "M 15 25 C 25 55 45 55 55 25")
    }

    #[test]
    pub fn translate_rel_curve() {
        let actual = PathTransformer::new("M10 10 c 10 30 30 30 40 0".into())
            .translate(5.0, 15.0)
            .to_string();
        assert_eq!(actual, "M 15 25 c 10 30 30 30 40 0")
    }

    #[test]
    pub fn translate_horizontal_lines() {
        let actual = PathTransformer::new("M10 10H40h50".into())
            .translate(10.0, 15.0)
            .to_string();
        assert_eq!(actual, "M 20 25 H 50 h 50");
    }

    #[test]
    pub fn translate_vertical_lines() {
        let actual = PathTransformer::new("M10 10V40v50".into())
            .translate(10.0, 15.0)
            .to_string();
        assert_eq!(actual, "M 20 25 V 55 v 50");
    }

    #[test]
    pub fn translate_rel_arcs() {
        let actual = PathTransformer::new("M40 30a20 40 -45 0 1 20 50".into())
            .translate(10.0, 15.0)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 50 45 a 40 20 45 0 1 20 50");
    }

    #[test]
    pub fn translate_abs_arcs() {
        let actual = PathTransformer::new("M40 30A20 40 -45 0 1 20 50".into())
            .translate(10.0, 15.0)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 50 45 A 40 20 45 0 1 30 65");
    }

    #[test]
    pub fn round_arcs() {
        let actual = PathTransformer::new("M10 10A12.5 17.5 45.5 0 0 15.5 19.5".into())
            .round(0)
            .to_string();
        assert_eq!(actual, "M 10 10 A 13 18 45.5 0 0 16 20");
    }

    #[test]
    pub fn round_curves() {
        let actual = PathTransformer::new("M10 10 c 10.12 30.34 30.56 30 40.00 0.12".into())
            .round(0)
            .to_string();
        assert_eq!(actual, "M 10 10 c 10 30 31 30 40 0");
    }

    #[test]
    pub fn round_precision() {
        let actual = PathTransformer::new("M10.123 10.456L20.4351 30.0000".into())
            .round(2)
            .to_string();
        assert_eq!(actual, "M 10.12 10.46 L 20.44 30");
    }

    #[test]
    pub fn track_errors() {
        let actual = PathTransformer::new("M1.2 1.4l1.2 1.4 l1.2 1.4".into())
            .round(0)
            .to_string();
        assert_eq!(actual, "M 1 1 l 1 2 l 2 1");
    }

    #[test]
    pub fn track_errors_2() {
        let actual = PathTransformer::new("M1.2 1.4 H2.4 h1.2 v2.4 h-2.4 V2.4 v-1.2".into())
            .round(0)
            .to_string();
        assert_eq!(actual, "M 1 1 H 2 h 2 v 3 h -3 V 2 v -1");
    }

    #[test]
    pub fn track_errors_for_contour_start() {
        let actual = PathTransformer::new("m0.4 0.2zm0.4 0.2m0.4 0.2m0.4 0.2zm0.4 0.2".into())
            .round(0)
            .abs()
            .to_string();
        assert_eq!(actual, "M 0 0 Z M 1 0 M 1 1 M 2 1 Z M 2 1");
    }

    #[test]
    pub fn reset_delta_error_on_contour_end() {
        let actual = PathTransformer::new("m.1 .1l.3 .3zm.1 .1l.3 .3zm0 0z".into())
            .round(0)
            .abs()
            .to_string();
        assert_eq!(actual, "M 0 0 L 0 0 Z M 0 0 L 1 1 Z M 0 0 Z");
    }

    #[test]
    pub fn replace_arcs_with_lines() {
        let actual =
            PathTransformer::new("M40 30a0 40 -45 0 1 20 50Z M40 30A20 0 -45 0 1 20 50Z".into())
                .scale(2.0, 2.0)
                .to_string();
        assert_eq!(actual, "M 80 60 l 40 100 Z M 80 60 L 40 100 Z");
    }

    #[test]
    pub fn drop_arcs_with_same_start_and_end_points() {
        let actual = PathTransformer::new("M40 30a20 40 -45 0 1 0 0".into())
            .scale(2.0, 2.0)
            .to_string();
        assert_eq!(actual, "M 80 60 l 0 0");
    }

    #[test]
    pub fn drop_arcs_with_same_start_and_end_points_2() {
        let actual = PathTransformer::new("M40 30A20 40 -45 0 1 40 30".into())
            .scale(2.0, 2.0)
            .to_string();

        assert_eq!(actual, "M 80 60 L 80 60");
    }

    #[test]
    pub fn arc_to_line_at_scale() {
        let actual = PathTransformer::new("M40 30a20 40 -45 0 1 20 50".into())
            .scale(0.0, 1.0)
            .to_string();
        assert_eq!(actual, "M 0 30 l 0 50");
    }

    #[test]
    pub fn arc_to_line_at_scale_2() {
        let actual = PathTransformer::new("M40 30A20 40 -45 0 1 20 50".into())
            .scale(1.0, 0.0)
            .to_string();
        assert_eq!(actual, "M 40 0 L 20 0");
    }

    #[test]
    pub fn arc_rotate_90() {
        let actual = PathTransformer::new("M40 30a20 40 -45 0 1 20 50".into())
            .rotate(90.0, 0.0, 0.0)
            .round(0)
            .to_string();

        assert_eq!(actual, "M -30 40 a 20 40 45 0 1 -50 20");
    }

    #[test]
    pub fn apply_arc_matrix() {
        let actual = PathTransformer::new("M40 30a20 40 -45 0 1 20 50".into())
            .matrix([0.0, 1.0, -1.0, 0.0, 0.0, 0.0])
            .round(0)
            .to_string();
        assert_eq!(actual, "M -30 40 a 20 40 45 0 1 -50 20");
    }

    #[test]
    pub fn arc_rotate_negative_90() {
        let actual = PathTransformer::new("M40 30a20 40 -45 0 1 20 50".into())
            .rotate(-90.0, 0.0, 0.0)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 30 -40 a 20 40 45 0 1 50 -20");
    }

    #[test]
    pub fn apply_arc_matrix_2() {
        let actual = PathTransformer::new("M40 30a20 40 -45 0 1 20 50".into())
            .matrix([0.0, -1.0, 1.0, 0.0, 0.0, 0.0])
            .round(0)
            .to_string();
        assert_eq!(actual, "M 30 -40 a 20 40 45 0 1 50 -20");
    }

    #[test]
    pub fn process_circle_like_segments() {
        let actual = PathTransformer::new("M50 50A30 30 -45 0 1 100 100".into())
            .scale(0.5, 0.5)
            .round(0)
            .to_string();
        assert_eq!(actual, "M 25 25 A 15 15 0 0 1 50 50");
    }

    #[test]
    pub fn almost_zero_eigen_values() {
        let actual = PathTransformer::new("M148.7 277.9A228.7 113.2 90 1 0 159.3 734.8".into())
            .translate(10.0, 10.0)
            .round(1)
            .to_string();
        assert_eq!(actual, "M 158.7 287.9 A 228.7 113.2 90 1 0 169.3 744.8");
    }

    #[test]
    pub fn should_flip_sweep_flag_if_image_is_flipped() {
        let actual = PathTransformer::new("M10 10A20 15 90 0 1 30 10".into())
            .scale(1.0, -1.0)
            .translate(0.0, 40.0)
            .to_string();
        assert_eq!(actual, "M 10 30 A 20 15 90 0 0 30 30");
    }

    #[test]
    pub fn should_flip_sweep_flag_if_image_is_flipped_2() {
        let actual = PathTransformer::new("M10 10A20 15 90 0 1 30 10".into())
            .scale(-1.0, -1.0)
            .translate(40.0, 40.0)
            .to_string();
        assert_eq!(actual, "M 30 30 A 20 15 90 0 1 10 30");
    }

    #[test]
    pub fn rel_convert_line() {
        let actual = PathTransformer::new("M10 10 L30 30".into())
            .rel()
            .to_string();
        assert_eq!(actual, "M 10 10 l 20 20");
    }

    #[test]
    pub fn dont_touch_rel_line() {
        let actual = PathTransformer::new("m10 10 l30 30".into())
            .rel()
            .to_string();

        assert_eq!(actual, "m 10 10 l 30 30");
    }

    #[test]
    pub fn rel_convert_multi_segment_curve() {
        let actual = PathTransformer::new("M10 10 C 20 40 40 40 50 10 60 -20 70 -20 90 10".into())
            .rel()
            .to_string();
        assert_eq!(actual, "M 10 10 c 10 30 30 30 40 0 c 10 -30 20 -30 40 0");
    }

    #[test]
    pub fn rel_horizontal_line() {
        let actual = PathTransformer::new("M10 10H40h50".into())
            .rel()
            .to_string();

        assert_eq!(actual, "M 10 10 h 30 h 50");
    }

    #[test]
    pub fn rel_vertical_line() {
        let actual = PathTransformer::new("M10 10V40v50".into())
            .rel()
            .to_string();

        assert_eq!(actual, "M 10 10 v 30 v 50");
    }

    #[test]
    pub fn rel_arcs() {
        let actual = PathTransformer::new("M40 30A20 40 -45 0 1 60 80".into())
            .rel()
            .to_string();

        assert_eq!(actual, "M 40 30 a 20 40 -45 0 1 20 50");
    }

    #[test]
    pub fn rel_track_position_after_z() {
        let actual = PathTransformer::new("M10 10 L20 10 L20 20 Z L10 20 L20 20 z L9 9".into())
            .rel()
            .to_string();

        assert_eq!(actual, "M 10 10 l 10 0 l 0 10 z l 0 10 l 10 0 z l -1 -1");
    }

    pub mod transform_string {

        pub mod translate {
            use crate::pt::PathTransformer;

            #[test]
            pub fn x_only() {
                let actual = PathTransformer::new("M10 10 L15 15".into())
                    .transform("translate(20)".into())
                    .to_string();
                assert_eq!(actual, "M 30 10 L 35 15");
            }

            #[test]
            pub fn x_and_y() {
                let actual = PathTransformer::new("M10 10 L15 15".into())
                    .transform("translate(20, 10)".into())
                    .to_string();
                assert_eq!(actual, "M 30 20 L 35 25");
            }

            #[test]
            pub fn x_and_y_with_relative_curves() {
                let actual = PathTransformer::new("M10 10 c15 15, 20 10, 15 15".into())
                    .transform("translate(20, 10)".into())
                    .to_string();
                assert_eq!(actual, "M 30 20 c 15 15 20 10 15 15");
            }

            #[test]
            pub fn x_and_y_with_absolute_curves() {
                let actual = PathTransformer::new("M10 10 C15 15, 20 10, 15 15".into())
                    .transform("translate(20, 10)".into())
                    .to_string();

                assert_eq!(actual, "M 30 20 C 35 25 40 20 35 25");
            }

            #[test]
            pub fn rel_after_translate_should_not_break() {
                let actual = PathTransformer::new("m70 70 l20 20 l-20 0 l0 -20".into())
                    .transform("translate(100, 100)".into())
                    .to_string();
                assert_eq!(actual, "M 170 170 l 20 20 l -20 0 l 0 -20");
            }

            #[test]
            pub fn rel_after_translate_should_not_break_2() {
                let actual = PathTransformer::new("m70 70 l20 20 l-20 0 l0 -20".into())
                    .transform("translate(100, 100)".into())
                    .rel()
                    .to_string();
                assert_eq!(actual, "M 170 170 l 20 20 l -20 0 l 0 -20");
            }
        }

        pub mod rotate {
            use crate::pt::PathTransformer;

            #[test]
            fn rotate_by_90_degrees() {
                let actual = PathTransformer::new("M10 10L15 10".into())
                    .transform("rotate(90, 10, 10)".into())
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 10 10 L 10 15")
            }

            #[test]
            pub fn rotate_by_negative_90_degrees() {
                let actual = PathTransformer::new("M0 10L0 20".into())
                    .transform("rotate(-90)".into())
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 10 0 L 20 0")
            }
        }

        pub mod scale {
            use crate::pt::PathTransformer;

            #[test]
            pub fn scale_by_2() {
                let actual = PathTransformer::new("M5 5L15 20".into())
                    .transform("scale(2)".into())
                    .to_string();

                assert_eq!(actual, "M 10 10 L 30 40");
            }

            #[test]
            pub fn scale_by_x_and_y() {
                let actual = PathTransformer::new("M5 5L30 20".into())
                    .transform("scale(0.5, 1.5)".into())
                    .to_string();
                assert_eq!(actual, "M 2.5 7.5 L 15 30")
            }

            #[test]
            pub fn scale_by_x_and_y_with_relative_curves() {
                let actual = PathTransformer::new("M5 5c15 15, 20 10, 15 15".into())
                    .transform("scale(.5, 1.5)".to_string())
                    .to_string();
                assert_eq!(actual, "M 2.5 7.5 c 7.5 22.5 10 15 7.5 22.5");
            }
        }

        pub mod skew {
            use crate::pt::PathTransformer;

            #[test]
            pub fn skew_x() {
                let actual = PathTransformer::new("M5 5L15 20".into())
                    .transform("skewX(75.96)".into())
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 25 5 L 95 20");
            }

            #[test]
            pub fn skew_y() {
                let actual = PathTransformer::new("M5 5L15 20".into())
                    .transform("skewY(75.96)".into())
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 5 25 L 15 80");
            }
        }

        pub mod matrix {
            use crate::pt::PathTransformer;

            #[test]
            pub fn path_with_absolute_segments() {
                let actual = PathTransformer::new("M5 5 C20 30 10 15 30 15".into())
                    .transform("matrix(1.5, 0.5, 0.5, 1.5 10, 15)".into())
                    .to_string();
                assert_eq!(actual, "M 20 25 C 55 70 32.5 42.5 62.5 52.5");
            }

            #[test]
            pub fn path_with_relative_segments() {
                let actual = PathTransformer::new("M5 5 c10 12 10 15 20 30".into())
                    .transform("matrix(1.5, 0.5, 0.5, 1.5 10, 15)".into())
                    .to_string();

                assert_eq!(actual, "M 20 25 c 21 23 22.5 27.5 45 55");
            }
        }

        pub mod combinations {
            use crate::pt::PathTransformer;

            #[test]
            pub fn scale_and_translate() {
                let actual = PathTransformer::new("M0 0 L 10 10 20 10".into())
                    .transform("translate(100,100) scale(2,3)".into())
                    .to_string();
                assert_eq!(actual, "M 100 100 L 120 130 L 140 130");
            }

            #[test]
            pub fn scale_and_rotate() {
                let actual = PathTransformer::new("M0 0 L 10 10 20 10".into())
                    .transform("rotate(90) scale(2,3)".into())
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 0 0 L -30 20 L -30 40");
            }

            #[test]
            pub fn skew_and_scale() {
                let actual = PathTransformer::new("M0 0 L 10 10 20 10".into())
                    .transform("skewX(75.96) scale(2,3)".into())
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 0 0 L 140 30 L 160 30")
            }
        }

        pub mod misc {
            use crate::pt::PathTransformer;

            #[test]
            pub fn empty_transforms() {
                let actual = PathTransformer::new("M0 0 L 10 10 20 10".into())
                    .transform("rotate(0) scale(1,1) translate(0,0) skewX(0) skewY(0)".into())
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 0 0 L 10 10 L 20 10");
            }

            #[test]
            #[should_panic]
            pub fn wrong_param_count() {
                PathTransformer::new("M0 0 L 10 10 20 10".into())
                    .transform("rotate(10,0) scale(10,10,1)".into())
                    .round(0)
                    .to_string();
            }

            #[test]
            pub fn segment_replacement() {
                let actual = PathTransformer::new("M0 0 H 10 V 10 Z M 100 100 h 15 v -10".into())
                    .transform("rotate(45)".into())
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 0 0 L 7 7 L 0 14 Z M 0 141 l 11 11 l 7 -7");
            }

            #[test]
            pub fn nothing_to_transform() {
                let actual = PathTransformer::new("M10 10 L15 15".into())
                    .transform("   ".into())
                    .to_string();
                assert_eq!(actual, "M 10 10 L 15 15");
            }

            #[test]
            pub fn first_m_should_be_absolute() {
                let actual = PathTransformer::new("m70 70 70 70".into())
                    .transform("translate(100,100)".into())
                    .to_string();
                assert_eq!(actual, "M 170 170 l 70 70");
            }
        }

        pub mod unarc {
            use crate::pt::PathTransformer;

            #[test]
            pub fn complete_arc() {
                let actual = PathTransformer::new("M100 100 A30 50 0 1 1 110 110".into())
                    .unarc()
                    .round(0)
                    .to_string();

                assert_eq!(actual, "M 100 100 C 89 83 87 54 96 33 C 105 12 122 7 136 20 C 149 33 154 61 147 84 C 141 108 125 119 110 110");
            }

            #[test]
            pub fn small_arc() {
                let actual = PathTransformer::new("M100 100 a30 50 0 0 1 30 30".into())
                    .unarc()
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 100 100 C 113 98 125 110 130 130")
            }

            #[test]
            pub fn circle() {
                let actual = PathTransformer::new(
                    "M100 100 m -75 0 a 75 75 0 1 0 150 0 a 75 75 0 1 0 -150 0".into(),
                )
                .unarc()
                .round(0)
                .to_string();
                assert_eq!(actual, "M 100 100 m -75 0 C 25 141 59 175 100 175 C 141 175 175 141 175 100 C 175 59 141 25 100 25 C 59 25 25 59 25 100");
            }

            #[test]
            pub fn rounding_errors() {
                let actual = PathTransformer::new(
                    "M-0.5 0 A 0.09188163040671497 0.011583783896639943 0 0 1 0 0.5".into(),
                )
                .unarc()
                .round(5)
                .to_string();
                assert_eq!(actual, "M -0.5 0 C 0.59517 -0.01741 1.59491 0.08041 1.73298 0.21848 C 1.87105 0.35655 1.09517 0.48259 0 0.5");
            }

            #[test]
            pub fn rounding_errors_2() {
                let actual = PathTransformer::new(
                    "M-0.07467194809578359 -0.3862391309812665 A1.2618792965076864 0.2013618852943182 90 0 1 -0.7558937461581081 -0.8010219619609416".into(),
                ).unarc().round(5).to_string();
                assert_eq!(actual, "M -0.07467 -0.38624 C -0.09295 0.79262 -0.26026 1.65542 -0.44838 1.54088 C -0.63649 1.42634 -0.77417 0.37784 -0.75589 -0.80102");
            }

            #[test]
            pub fn curve_between_same_point() {
                let actual = PathTransformer::new("M100 100A123 456 90 0 1 100 100".into())
                    .unarc()
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 100 100 L 100 100");
            }

            #[test]
            pub fn curve_between_same_point_2() {
                let actual = PathTransformer::new("M100 100a123 456 90 0 1 0 0".into())
                    .unarc()
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 100 100 l 0 0");
            }

            #[test]
            pub fn rx_ry_zero() {
                let actual = PathTransformer::new("M100 100A0 0 0 0 1 110 110".into())
                    .unarc()
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 100 100 L 110 110");
            }

            #[test]
            pub fn rx_zero() {
                let actual = PathTransformer::new("M100 100A0 100 0 0 1 110 110".into())
                    .unarc()
                    .round(0)
                    .to_string();
                assert_eq!(actual, "M 100 100 L 110 110");
            }
        }

        pub mod unshort {
            use crate::pt::PathTransformer;

            #[test]
            pub fn shouldnt_change_full_arc() {
                let actual = PathTransformer::new("M10 10 C 20 20, 40 20, 50 10".into())
                    .unshort()
                    .to_string();
                assert_eq!(actual, "M 10 10 C 20 20 40 20 50 10");
            }

            #[test]
            pub fn should_reflect_control_point() {
                let actual =
                    PathTransformer::new("M10 10 C 20 20, 40 20, 50 10 S 80 0, 90 10".into())
                        .unshort()
                        .to_string();
                assert_eq!(actual, "M 10 10 C 20 20 40 20 50 10 C 60 0 80 0 90 10");
            }

            #[test]
            pub fn should_copy_starting_point() {
                let actual = PathTransformer::new("M10 10 S 50 50, 90 10".into())
                    .unshort()
                    .to_string();
                assert_eq!(actual, "M 10 10 C 10 10 50 50 90 10");
            }

            #[test]
            pub fn relative_paths() {
                let actual =
                    PathTransformer::new("M30 50 c 10 30, 30 30, 40 0 s 30 -30, 40 0".into())
                        .unshort()
                        .to_string();

                assert_eq!(actual, "M 30 50 c 10 30 30 30 40 0 c 10 -30 30 -30 40 0");
            }

            #[test]
            pub fn quadratic_shouldnt_change_full_arc() {
                let actual = PathTransformer::new("M10 10 Q 50 50, 90 10".into())
                    .unshort()
                    .to_string();
                assert_eq!(actual, "M 10 10 Q 50 50 90 10");
            }

            #[test]
            pub fn quadratic_should_reflect_control_point() {
                let actual = PathTransformer::new("M30 50 Q 50 90, 90 50 T 150 50".into())
                    .unshort()
                    .to_string();
                assert_eq!(actual, "M 30 50 Q 50 90 90 50 Q 130 10 150 50");
            }

            #[test]
            pub fn quadratic_copy_starting_point() {
                let actual = PathTransformer::new("M10 30 T150 50".into())
                    .unshort()
                    .to_string();
                assert_eq!(actual, "M 10 30 Q 10 30 150 50");
            }

            #[test]
            pub fn quadratic_handle_relative_paths() {
                let actual = PathTransformer::new("M30 50 q 20 20, 40 0 t 40 0".into())
                    .unshort()
                    .to_string();
                assert_eq!(actual, "M 30 50 q 20 20 40 0 q 20 -20 40 0");
            }
        }

        pub mod bbox {
            use crate::{
                bbox::{Alignment, BBox, BoxAlignment, InboxParameters, ScaleType},
                pt::PathTransformer,
            };

            #[test]
            fn get_bounding_box() {
                // Test complex path with curves and arc
                let mut path = PathTransformer::new(
                       "M10,10 c 10,0 10,10 0,10 s -10,0 0,10 q 10,10 15 20 t 10,0 a25,25 -30 0,1 50,-25z"
                           .to_string(),
                   );

                let bbox = path.round(3).to_box(None);
                let bbox_array = bbox.to_array().unwrap();

                assert_eq!(
                    format!(
                        "{} {} {} {}",
                        bbox_array[0], bbox_array[1], bbox_array[2], bbox_array[3]
                    ),
                    "2.5 9.543027464337513 84.99999999999997 55"
                );

                // // Test with different rounding precision
                let mut path = PathTransformer::new(
                    "M10,10 c 10,0 10,10 0,10 s -10,0 0,10 q 10,10 15 20 t 10,0 a25,25 -30 0,1 50,-25z"
                        .to_string(),
                );
                let bbox = path.round(2).to_box(None);
                assert_eq!(bbox.to_string(Some(2)), "2.50 9.54 82.50 45.46");

                // // Test with string format precision
                let path = PathTransformer::new(
                    "M10,10 c 10,0 10,10 0,10 s -10,0 0,10 q 10,10 15 20 t 10,0 a25,25 -30 0,1 50,-25z"
                        .to_string(),
                );
                let bbox = path.to_box(Some(2));
                assert_eq!(bbox.to_string(Some(2)), "2.50 9.54 82.50 45.46");
            }

            #[test]
            fn test_matrix_to_fit_in_box() {
                // Test case 1: default meet xMidYMid
                let mut path = PathTransformer::new("M10,10 h10 v20".to_string());
                let params = InboxParameters {
                    destination: BBox::from("0 0 100 100"),
                    scale_type: ScaleType::Meet,
                    alignment: BoxAlignment { x: Alignment::Mid, y: Alignment::Mid },
                };
                path.inbox(params);
                assert_eq!(path.to_box(None).to_string(None), "25 0 50 100");

                // Test case 2: slice xMinYMax
                let mut path = PathTransformer::new("M10,10 h10 v20".to_string());
                let params = InboxParameters {
                    destination: BBox::from("0 0 100 100"),
                    scale_type: ScaleType::Slice,
                    alignment: BoxAlignment { x: Alignment::Min, y: Alignment::Max },
                };
                path.inbox(params);
                assert_eq!(path.to_box(None).to_string(None), "0 -100 100 200");

                // Test case 3: fit
                let mut path = PathTransformer::new("M10,10 h10 v20".to_string());
                let params = InboxParameters {
                    destination: BBox::from("0 0 100 100"),
                    scale_type: ScaleType::Fit,
                    alignment: BoxAlignment { x: Alignment::Mid, y: Alignment::Mid },
                };
                path.inbox(params);
                assert_eq!(path.to_box(None).to_string(None), "0 0 100 100");

                // Test case 4: move xMaxYMid
                let mut path = PathTransformer::new("M10,10 h10 v20".to_string());
                let params = InboxParameters {
                    destination: BBox::from("0 0 100 100"),
                    scale_type: ScaleType::Move,
                    alignment: BoxAlignment { x: Alignment::Max, y: Alignment::Mid },
                };
                path.inbox(params);
                assert_eq!(path.to_box(None).to_string(None), "90 40 10 20");
            }
        }
    }
}
