// Determine whether a path has the shape of an axisa-aligned rectangle.

use crate::math::{point, vector, Box2D, Point, Vector};
use crate::path::PathEvent;

#[cfg(not(feature = "std"))]
use num_traits::Float;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ToRectangleOptions {
    pub tolerance: f32,
    pub auto_close: bool,
    /// If true don't consider open sub-paths with no segment.
    pub ignore_open_empty_sub_paths: bool,
    /// If true don't consider closed sub-paths with no segment.
    pub ignore_closed_empty_sub_paths: bool,
}

impl ToRectangleOptions {
    /// Default parameters relevant for filling paths.
    pub fn fill(tolerance: f32) -> Self {
        ToRectangleOptions {
            tolerance,
            auto_close: true,
            ignore_open_empty_sub_paths: true,
            ignore_closed_empty_sub_paths: true,
        }
    }

    /// Default parameters relevant for stroking paths.
    ///
    /// Accepts a subset of the `fill` configuration.
    pub fn stroke(tolerance: f32) -> Self {
        ToRectangleOptions {
            tolerance,
            auto_close: false,
            ignore_open_empty_sub_paths: true,
            ignore_closed_empty_sub_paths: false,
        }
    }
}

/// If the input path represents an axis-aligned rectangle, return it.
pub fn to_axis_aligned_rectangle<P: IntoIterator<Item = PathEvent>>(
    path: P,
    options: &ToRectangleOptions,
) -> Option<Box2D> {
    let tolerance = options.tolerance;
    let mut ctx = ToRectangle {
        min: point(0.0, 0.0),
        max: point(0.0, 0.0),
        current_dir: Dir::None,
        idx: 0,
        dirs: [Dir::None; 4],
        tolerance,
    };

    for event in path.into_iter() {
        match event {
            PathEvent::Begin { at } => {
                if ctx.idx == 0 {
                    ctx.min = at;
                    ctx.max = at;
                }
            }
            PathEvent::End { first, last, close } => {
                if ctx.idx == 0 {
                    if !close && options.ignore_open_empty_sub_paths {
                        continue;
                    }
                    if close && options.ignore_closed_empty_sub_paths {
                        continue;
                    }
                }

                if close || options.auto_close {
                    ctx.edge(last, first)?;
                }

                ctx.end_sub_path()?;
                break;
            }
            PathEvent::Line { from, to } => {
                ctx.edge(from, to)?;
            }
            PathEvent::Quadratic { from, ctrl, to } => {
                if ctrl != from {
                    let tol = vector(tolerance, tolerance);
                    let to_axis = (to - from).abs().greater_than(tol);
                    let ctrl_axis = (ctrl - from).abs().greater_than(tol);

                    if ctrl_axis != to_axis {
                        return None;
                    }

                    if to_axis.x && !is_between(ctrl.x, from.x, to.x) {
                        return None;
                    }

                    if to_axis.y && !is_between(ctrl.y, from.y, to.y) {
                        return None;
                    }
                }

                ctx.edge(from, to);
            }
            PathEvent::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => {
                let tol = vector(tolerance, tolerance);
                let to_axis = (to - from).abs().greater_than(tol);
                let mut ctrl1_axis = (ctrl1 - from).abs().greater_than(tol);
                let mut ctrl2_axis = (ctrl2 - from).abs().greater_than(tol);

                if ctrl1 == from {
                    ctrl1_axis = to_axis;
                }

                if ctrl2 == from {
                    ctrl2_axis = to_axis;
                }

                if ctrl1_axis != ctrl2_axis || ctrl1_axis != to_axis {
                    return None;
                }

                if to_axis.x
                    && !(is_between(ctrl1.x, from.x, to.x) && is_between(ctrl2.x, from.x, to.x))
                {
                    return None;
                }

                if to_axis.y
                    && !(is_between(ctrl1.y, from.y, to.y) && is_between(ctrl2.y, from.y, to.y))
                {
                    return None;
                }

                ctx.edge(from, to);
            }
        }
    }

    Some(Box2D {
        min: ctx.min,
        max: ctx.max,
    })
}

struct ToRectangle {
    min: Point,
    max: Point,
    current_dir: Dir,
    idx: usize,
    dirs: [Dir; 4],
    tolerance: f32,
}

impl ToRectangle {
    fn edge(&mut self, from: Point, to: Point) -> Option<()> {
        let edge = to - from;
        let dir = direction(edge, self.tolerance)?;
        if dir == Dir::None {
            return Some(());
        }

        if dir != self.current_dir {
            if self.idx >= 4 {
                return None;
            }

            if dir == self.current_dir.opposite() {
                return None;
            }

            self.dirs[self.idx] = dir;
            self.idx += 1;
            self.current_dir = dir;
        }

        self.min.x = self.min.x.min(to.x);
        self.min.y = self.min.y.min(to.y);
        self.max.x = self.max.x.max(to.x);
        self.max.y = self.max.y.max(to.y);

        Some(())
    }

    fn end_sub_path(&self) -> Option<()> {
        if self.idx == 0 {
            return Some(());
        }

        if self.idx != 4 {
            return None;
        }

        if self.dirs[0].opposite() != self.dirs[2] || self.dirs[1].opposite() != self.dirs[3] {
            return None;
        }

        Some(())
    }
}

impl Default for ToRectangleOptions {
    fn default() -> Self {
        ToRectangleOptions {
            tolerance: 0.0,
            auto_close: true,
            ignore_open_empty_sub_paths: true,
            ignore_closed_empty_sub_paths: true,
        }
    }
}

fn is_between(x: f32, from: f32, to: f32) -> bool {
    (from <= x && x <= to) || (to <= x && x <= from)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
    None,
}

impl Dir {
    fn opposite(self) -> Self {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::None => Dir::None,
        }
    }
}

fn direction(v: Vector, tolerance: f32) -> Option<Dir> {
    if !v.is_finite() {
        return None;
    }

    let x = v.x.abs() > tolerance;
    let y = v.y.abs() > tolerance;

    if x && y {
        return None;
    }

    if !x && !y {
        return Some(Dir::None);
    }

    let dir = if x {
        if v.x > 0.0 {
            Dir::Right
        } else {
            Dir::Left
        }
    } else if v.y > 0.0 {
        Dir::Down
    } else {
        Dir::Up
    };

    Some(dir)
}

#[test]
fn test_to_axis_aligned_rectangle() {
    use crate::geom::euclid::approxeq::ApproxEq;
    fn approx_eq(a: Box2D, b: Box2D) -> bool {
        a.min.approx_eq(&b.min) && a.max.approx_eq(&b.max)
    }

    let fill = ToRectangleOptions::fill(0.00001);
    let stroke = ToRectangleOptions::stroke(0.00001);

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(10.0, 0.0));
    builder.line_to(point(10.0, 5.0));
    builder.line_to(point(0.0, 5.0));
    builder.end(true);
    let path = builder.build();

    let r = to_axis_aligned_rectangle(&path, &fill).unwrap();
    assert!(approx_eq(
        r,
        Box2D {
            min: point(0.0, 0.0),
            max: point(10.0, 5.0)
        }
    ));

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(0.0, 5.0));
    builder.line_to(point(10.0, 5.0));
    builder.line_to(point(10.0, 0.0));
    builder.end(false);
    let path = builder.build();

    let r = to_axis_aligned_rectangle(&path, &fill).unwrap();
    assert!(approx_eq(
        r,
        Box2D {
            min: point(0.0, 0.0),
            max: point(10.0, 5.0)
        }
    ));
    assert!(to_axis_aligned_rectangle(&path, &stroke).is_none());

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(0.0, 2.0));
    builder.line_to(point(0.0, 5.0));
    builder.line_to(point(1.0, 5.0));
    builder.line_to(point(9.0, 5.0));
    builder.line_to(point(10.0, 5.0));
    builder.line_to(point(10.0, 0.0));
    builder.line_to(point(0.0, 0.0));
    builder.end(true);
    let path = builder.build();

    let r = to_axis_aligned_rectangle(&path, &fill).unwrap();
    assert!(approx_eq(
        r,
        Box2D {
            min: point(0.0, 0.0),
            max: point(10.0, 5.0)
        }
    ));

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(0.0, 5.0));
    builder.line_to(point(10.0, 5.0));
    builder.end(false);
    let path = builder.build();

    assert!(to_axis_aligned_rectangle(&path, &fill).is_none());

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(0.0, 5.0));
    builder.end(false);
    let path = builder.build();

    assert!(to_axis_aligned_rectangle(&path, &fill).is_none());

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(0.0, 5.0));
    builder.line_to(point(10.0, 5.0));
    builder.line_to(point(10.0, 1.0));
    builder.end(false);
    let path = builder.build();

    assert!(to_axis_aligned_rectangle(&path, &fill).is_none());

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(0.0, 5.0));
    builder.line_to(point(5.0, 5.0));
    builder.line_to(point(10.0, 5.0));
    builder.line_to(point(10.0, 10.0));
    builder.line_to(point(0.0, 10.0));
    builder.end(false);
    let path = builder.build();

    assert!(to_axis_aligned_rectangle(&path, &fill).is_none());

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.quadratic_bezier_to(point(5.0, 0.0), point(10.0, 0.0));
    builder.line_to(point(10.0, 5.0));
    builder.line_to(point(0.0, 5.0));
    builder.end(true);
    let path = builder.build();

    let r = to_axis_aligned_rectangle(&path, &fill).unwrap();
    assert!(approx_eq(
        r,
        Box2D {
            min: point(0.0, 0.0),
            max: point(10.0, 5.0)
        }
    ));

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.quadratic_bezier_to(point(11.0, 0.0), point(10.0, 0.0));
    builder.line_to(point(10.0, 5.0));
    builder.line_to(point(0.0, 5.0));
    builder.end(true);
    let path = builder.build();

    assert!(to_axis_aligned_rectangle(&path, &fill).is_none());

    let mut builder = crate::path::Path::builder();
    builder.begin(point(-1.0, 0.0));
    builder.end(true);

    builder.begin(point(0.0, -1.0));
    builder.end(false);

    builder.begin(point(0.0, 0.0));
    builder.line_to(point(10.0, 0.0));
    builder.line_to(point(10.0, 5.0));
    builder.line_to(point(0.0, 5.0));
    builder.end(true);

    builder.begin(point(-1.0, -1.0));
    builder.end(false);

    builder.begin(point(-1.0, -1.0));
    builder.end(true);

    let path = builder.build();

    let r = to_axis_aligned_rectangle(&path, &fill).unwrap();
    assert!(approx_eq(
        r,
        Box2D {
            min: point(0.0, 0.0),
            max: point(10.0, 5.0)
        }
    ));

    let mut builder = crate::path::Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(10.0, 0.0));
    builder.line_to(point(10.0, 5.0));
    builder.line_to(point(1.0, 5.0));
    builder.end(false);
    let path = builder.build();

    assert!(to_axis_aligned_rectangle(&path, &stroke).is_none());
}
