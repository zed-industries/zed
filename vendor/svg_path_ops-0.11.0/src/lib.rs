// This crate is entirely safe
#![forbid(unsafe_code)]

//!
//! This crate includes utility functions to work with svg paths. Works on types from [svgtypes](https://github.com/RazrFalcon/svgtypes)
//! crate.
//!
//! This package exposes functions to manipulate svg paths with simplification purposes. Also a path transformer fully compatible with
//! [svgpath](https://github.com/fontello/svgpath) is provided.
//!
//!
//! ## 📦 Cargo.toml
//!
//! ```toml
//! [dependencies]
//! svg_path_ops = "0.6"
//! ```
//!
//! ## 🔧 Example
//!
//! ### Translate
//!
//! ``` rust,ignore
//! let translated_path = PathTransformer::new(cat_svg_path)
//!     .translate(230.0, 0.0)
//!     .to_string();
//! ```
//!
//! [full example](https://github.com/orhanbalci/rough-rs/blob/main/rough_piet/examples/translate.rs)
//!
//! ### 🖨️ Output Translate
//! ![translate](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/svg_path_ops/assets/translated_cat.png)
//!
//! ### Rotate
//!
//! ``` rust,ignore
//! let translated_path = PathTransformer::new(cat_svg_path)
//!     .rotate(90.0, 126.0, 140.0)
//!     .translate(220.0, 0.0)
//!     .to_string();
//! ```
//!
//! [full example](https://github.com/orhanbalci/rough-rs/blob/main/rough_piet/examples/rotate.rs)
//!
//! ### 🖨️ Output Rotate
//! ![translate](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/svg_path_ops/assets/rotated_cat.png)
//!
//! ### Skew
//! ``` rust,ignore
//! let translated_path = PathTransformer::new(cat_svg_path)
//!     .skew_x(20.0)
//!     .translate(180.0, 0.0)
//!     .to_string();
//! ```
//!
//! [full example](https://github.com/orhanbalci/rough-rs/blob/main/rough_piet/examples/skew.rs)
//!
//! ### 🖨️ Output Skew
//! ![translate](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/svg_path_ops/assets/skewed_cat.png)
//!
//! ### Scale
//! ``` rust,ignore
//! let translated_path = PathTransformer::new(cat_svg_path)
//!     .scale(0.5, 0.5)
//!     .translate(220.0, 60.0)
//!     .to_string();
//! ```
//!
//! [full example](https://github.com/orhanbalci/rough-rs/blob/main/rough_piet/examples/scale.rs)
//!
//! ### 🖨️ Output Scale
//! ![translate](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/svg_path_ops/assets/scaled_cat.png)

pub(crate) mod a2c;
pub mod bbox;
pub(crate) mod ellipse;
pub mod pt;

use std::borrow::Borrow;
use std::f64::consts::PI;

pub use svgtypes::PathSegment;

/// Translates relative commands to absolute commands. All commands that use relative positions (lower-case ones),
/// turns into absolute position commands (upper-case ones).
pub fn absolutize(
    path_segments: impl Iterator<Item = impl Borrow<PathSegment>>,
) -> impl Iterator<Item = PathSegment> {
    let mut result = vec![];
    let (mut cx, mut cy, mut subx, mut suby) = (0.0, 0.0, 0.0, 0.0);
    for segment in path_segments {
        match *segment.borrow() {
            PathSegment::MoveTo { abs: true, x, y } => {
                result.push(*segment.borrow());
                cx = x;
                cy = y;
                subx = x;
                suby = y;
            }
            PathSegment::MoveTo { abs: false, x, y } => {
                cx += x;
                cy += y;
                result.push(PathSegment::MoveTo { abs: true, x: cx, y: cy });
                subx = cx;
                suby = cy;
            }
            PathSegment::LineTo { abs: true, x, y } => {
                result.push(*segment.borrow());
                cx = x;
                cy = y;
            }
            PathSegment::LineTo { abs: false, x, y } => {
                cx += x;
                cy += y;
                result.push(PathSegment::LineTo { abs: true, x: cx, y: cy });
            }
            PathSegment::CurveTo { abs: true, x1: _, y1: _, x2: _, y2: _, x, y } => {
                result.push(*segment.borrow());
                cx = x;
                cy = y;
            }
            PathSegment::CurveTo { abs: false, x1, y1, x2, y2, x, y } => {
                result.push(PathSegment::CurveTo {
                    abs: true,
                    x1: x1 + cx,
                    y1: y1 + cy,
                    x2: x2 + cx,
                    y2: y2 + cy,
                    x: x + cx,
                    y: y + cy,
                });
                cx += x;
                cy += y;
            }
            PathSegment::Quadratic { abs: true, x1: _, y1: _, x, y } => {
                result.push(*segment.borrow());
                cx = x;
                cy = y;
            }
            PathSegment::Quadratic { abs: false, x1, y1, x, y } => {
                result.push(PathSegment::Quadratic {
                    abs: true,
                    x1: x1 + cx,
                    y1: y1 + cy,
                    x: x + cx,
                    y: y + cy,
                });
                cx += x;
                cy += y;
            }
            PathSegment::EllipticalArc {
                abs: true,
                rx: _,
                ry: _,
                x_axis_rotation: _,
                large_arc: _,
                sweep: _,
                x,
                y,
            } => {
                result.push(*segment.borrow());
                cx = x;
                cy = y;
            }
            PathSegment::EllipticalArc {
                abs: false,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
            } => {
                cx += x;
                cy += y;
                result.push(PathSegment::EllipticalArc {
                    abs: true,
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc,
                    sweep,
                    x: cx,
                    y: cy,
                });
            }
            PathSegment::HorizontalLineTo { abs: true, x } => {
                result.push(*segment.borrow());
                cx = x;
            }
            PathSegment::HorizontalLineTo { abs: false, x } => {
                cx += x;
                result.push(PathSegment::HorizontalLineTo { abs: true, x: cx });
            }
            PathSegment::VerticalLineTo { abs: true, y } => {
                result.push(*segment.borrow());
                cy = y;
            }
            PathSegment::VerticalLineTo { abs: false, y } => {
                cy += y;
                result.push(PathSegment::VerticalLineTo { abs: true, y: cy });
            }
            PathSegment::SmoothCurveTo { abs: true, x2: _, y2: _, x, y } => {
                result.push(*segment.borrow());
                cx = x;
                cy = y;
            }
            PathSegment::SmoothCurveTo { abs: false, x2, y2, x, y } => {
                result.push(PathSegment::SmoothCurveTo {
                    abs: true,
                    x2: x2 + cx,
                    y2: y2 + cy,
                    x: x + cx,
                    y: y + cy,
                });
                cx += x;
                cy += y;
            }
            PathSegment::SmoothQuadratic { abs: true, x, y } => {
                result.push(*segment.borrow());
                cx = x;
                cy = y;
            }
            PathSegment::SmoothQuadratic { abs: false, x, y } => {
                cx += x;
                cy += y;
                result.push(PathSegment::SmoothQuadratic { abs: true, x: cx, y: cy });
            }
            PathSegment::ClosePath { .. } => {
                result.push(PathSegment::ClosePath { abs: true });
                cx = subx;
                cy = suby;
            }
        }
    }

    result.into_iter()
}

pub fn print_line_segment(segment: &PathSegment) {
    match segment {
        PathSegment::MoveTo { abs, x, y } => {
            if *abs {
                println!("M {} {}", x, y);
            } else {
                println!("m {} {}", x, y);
            }
        }
        PathSegment::LineTo { abs, x, y } => {
            if *abs {
                println!("L {} {}", x, y);
            } else {
                println!("l {} {}", x, y);
            }
        }
        PathSegment::HorizontalLineTo { abs, x } => {
            if *abs {
                println!("H {}", x);
            } else {
                println!("h {}", x);
            }
        }
        PathSegment::VerticalLineTo { abs, y } => {
            if *abs {
                println!("V {}", y);
            } else {
                println!("v {}", y);
            }
        }
        PathSegment::CurveTo { abs, x1, y1, x2, y2, x, y } => {
            if *abs {
                println!("C {} {} {} {} {} {}", x1, y1, x2, y2, x, y);
            } else {
                println!("c {} {} {} {} {} {}", x1, y1, x2, y2, x, y);
            }
        }
        PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
            if *abs {
                println!("S {} {} {} {}", x2, y2, x, y);
            } else {
                println!("s {} {} {} {}", x2, y2, x, y);
            }
        }
        PathSegment::Quadratic { abs, x1, y1, x, y } => {
            if *abs {
                println!("Q {} {} {} {}", x1, y1, x, y);
            } else {
                println!("q {} {} {} {}", x1, y1, x, y);
            }
        }
        PathSegment::SmoothQuadratic { abs, x, y } => {
            if *abs {
                println!("T {} {}", x, y);
            } else {
                println!("t {} {}", x, y);
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
                println!(
                    "A {} {} {} {} {} {} {}",
                    rx, ry, x_axis_rotation, large_arc, sweep, x, y
                );
            } else {
                println!(
                    "a {} {} {} {} {} {} {}",
                    rx, ry, x_axis_rotation, large_arc, sweep, x, y
                );
            }
        }
        PathSegment::ClosePath { abs } => {
            if *abs {
                println!("Z");
            } else {
                println!("z");
            }
        }
    }
}

/// Normalize takes a list of absolute segments and outputs a list of segments with only four commands: M, L, C, Z. So every segment is described as move, line, or a bezier curve (cubic).
/// This is useful when translating SVG paths to non SVG mediums - Canvas, or some other graphics platform. Most such platforms will support lines and bezier curves.
/// It also simplifies the cases to consider when modifying these segments.
pub fn normalize(
    path_segments: impl Iterator<Item = impl Borrow<PathSegment>>,
) -> impl Iterator<Item = PathSegment> {
    let mut out = vec![];

    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut subx = 0.0;
    let mut suby = 0.0;
    let mut lcx = 0.0;
    let mut lcy = 0.0;
    let mut last_type: Option<PathSegment> = None;

    for segment in path_segments {
        match *segment.borrow() {
            PathSegment::MoveTo { abs: true, x, y } => {
                out.push(*segment.borrow());
                cx = x;
                cy = y;
                subx = x;
                suby = y;
            }
            PathSegment::CurveTo { abs: true, x1: _, y1: _, x2, y2, x, y } => {
                out.push(*segment.borrow());
                cx = x;
                cy = y;
                lcx = x2;
                lcy = y2;
            }
            PathSegment::LineTo { abs: true, x, y } => {
                out.push(*segment.borrow());
                cx = x;
                cy = y;
            }
            PathSegment::HorizontalLineTo { abs: true, x } => {
                cx = x;
                out.push(PathSegment::LineTo { abs: true, x: cx, y: cy });
            }
            PathSegment::VerticalLineTo { abs: true, y } => {
                cy = y;
                out.push(PathSegment::LineTo { abs: true, x: cx, y: cy });
            }
            PathSegment::SmoothCurveTo { abs: true, x2, y2, x, y } => {
                let cx1;
                let cy1;
                if let Some(lt) = last_type {
                    if matches!(lt, PathSegment::CurveTo { .. })
                        || matches!(lt, PathSegment::SmoothCurveTo { .. })
                    {
                        cx1 = cx + (cx - lcx);
                        cy1 = cy + (cy - lcy);
                    } else {
                        cx1 = cx;
                        cy1 = cy;
                    }
                } else {
                    cx1 = cx;
                    cy1 = cy;
                }
                out.push(PathSegment::CurveTo { abs: true, x1: cx1, y1: cy1, x2, y2, x, y });
                lcx = x2;
                lcy = y2;
                cx = x;
                cy = y;
            }
            PathSegment::SmoothQuadratic { abs: true, x, y } => {
                let x1;
                let y1;
                if let Some(lt) = last_type {
                    if matches!(lt, PathSegment::Quadratic { .. })
                        || matches!(lt, PathSegment::SmoothQuadratic { .. })
                    {
                        x1 = cx + (cx - lcx);
                        y1 = cy + (cy - lcy);
                    } else {
                        x1 = cx;
                        y1 = cy;
                    }
                } else {
                    x1 = cx;
                    y1 = cy;
                }
                let cx1 = cx + 2.0 * (x1 - cx) / 3.0;
                let cy1 = cy + 2.0 * (y1 - cy) / 3.0;
                let cx2 = x + 2.0 * (x1 - x) / 3.0;
                let cy2 = y + 2.0 * (y1 - y) / 3.0;
                out.push(PathSegment::CurveTo {
                    abs: true,
                    x1: cx1,
                    y1: cy1,
                    x2: cx2,
                    y2: cy2,
                    x,
                    y,
                });
                lcx = x1;
                lcy = y1;
                cx = x;
                cy = y;
            }
            PathSegment::Quadratic { abs: true, x1, y1, x, y } => {
                let cx1 = cx + 2.0 * (x1 - cx) / 3.0;
                let cy1 = cy + 2.0 * (y1 - cy) / 3.0;
                let cx2 = x + 2.0 * (x1 - x) / 3.0;
                let cy2 = y + 2.0 * (y1 - y) / 3.0;
                out.push(PathSegment::CurveTo {
                    abs: true,
                    x1: cx1,
                    y1: cy1,
                    x2: cx2,
                    y2: cy2,
                    x,
                    y,
                });
                lcx = x1;
                lcy = y1;
                cx = x;
                cy = y;
            }
            PathSegment::EllipticalArc {
                abs: true,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
            } => {
                let r1 = rx.abs();
                let r2 = ry.abs();
                let angle = x_axis_rotation;
                let large_arc_flag = large_arc;
                let sweep_flag = sweep;
                if r1 == 0.0 || r2 == 0.0 {
                    out.push(PathSegment::CurveTo {
                        abs: true,
                        x1: cx,
                        y1: cy,
                        x2: x,
                        y2: y,
                        x,
                        y,
                    });
                    cx = x;
                    cy = y;
                } else if cx != x || cy != y {
                    let x_param = x;
                    let y_param = y;
                    let r1_param = r1;
                    let r2_param = r2;
                    let curves = arc_to_cubic_curves(
                        cx,
                        cy,
                        x_param,
                        y_param,
                        r1_param,
                        r2_param,
                        angle,
                        large_arc_flag,
                        sweep_flag,
                        None,
                    );
                    for curve in curves.iter() {
                        out.push(PathSegment::CurveTo {
                            abs: true,
                            x1: curve[0],
                            y1: curve[1],
                            x2: curve[2],
                            y2: curve[3],
                            x: curve[4],
                            y: curve[5],
                        })
                    }
                    cx = x;
                    cy = y;
                }
            }
            PathSegment::ClosePath { abs: true } => {
                out.push(*segment.borrow());
                cx = subx;
                cy = suby;
            }
            _ => panic!("Not expecting none absolute path!"),
        }
        last_type = Some(*segment.borrow());
    }

    out.into_iter()
}

fn rotate(x: f64, y: f64, angle_rad: f64) -> (f64, f64) {
    let rotated_x = x * angle_rad.cos() - y * angle_rad.sin();
    let rotated_y = x * angle_rad.sin() + y * angle_rad.cos();
    (rotated_x, rotated_y)
}

fn arc_to_cubic_curves(
    mut x1: f64,
    mut y1: f64,
    mut x2: f64,
    mut y2: f64,
    mut r1: f64,
    mut r2: f64,
    angle: f64,
    large_arc_flag: bool,
    sweep_flag: bool,
    recursive: Option<[f64; 4]>,
) -> Vec<Vec<f64>> {
    let angle_rad = angle.to_radians();
    let mut params: Vec<Vec<f64>> = vec![];
    let (mut f1, mut f2, cx, cy);
    if let Some(rec) = recursive {
        f1 = rec[0];
        f2 = rec[1];
        cx = rec[2];
        cy = rec[3];
    } else {
        (x1, y1) = rotate(x1, y1, -angle_rad);
        (x2, y2) = rotate(x2, y2, -angle_rad);
        let x = (x1 - x2) / 2.0;
        let y = (y1 - y2) / 2.0;
        let mut h = (x * x) / (r1 * r1) + (y * y) / (r2 * r2);
        if h > 1.0 {
            h = h.sqrt();
            r1 *= h;
            r2 *= h;
        }
        let sign = if large_arc_flag == sweep_flag {
            -1.0
        } else {
            1.0
        };
        let r1_pow = r1.powi(2);
        let r2_pow = r2.powi(2);
        let left = r1_pow * r2_pow - r1_pow * y * y - r2_pow * x * x;
        let right = r1_pow * y * y + r2_pow * x * x;

        let k = sign * (left / right).abs().sqrt();

        cx = k * r1 * y / r2 + (x1 + x2) / 2.0;
        cy = k * -r2 * x / r1 + (y1 + y2) / 2.0;

        f1 = ((y1 - cy) / r2).asin();
        f2 = ((y2 - cy) / r2).asin();

        if x1 < cx {
            f1 = PI - f1;
        }
        if x2 < cx {
            f2 = PI - f2;
        }

        if f1 < 0.0 {
            f1 += PI * 2.0;
        }
        if f2 < 0.0 {
            f2 += PI * 2.0;
        }

        if sweep_flag && f1 > f2 {
            f1 -= PI * 2.0;
        }
        if !sweep_flag && f2 > f1 {
            f2 -= PI * 2.0;
        }
    }

    let mut df = f2 - f1;
    if df.abs() > (PI * 120.0 / 180.0) {
        let f2old = f2;
        let x2old = x2;
        let y2old = y2;

        if sweep_flag && f2 > f1 {
            f2 = f1 + (PI * 120.0 / 180.0) * (1.0);
        } else {
            f2 = f1 + (PI * 120.0 / 180.0) * (-1.0);
        }

        x2 = cx + r1 * f2.cos();
        y2 = cy + r2 * f2.sin();
        let x2old_param = x2old;
        let y2old_param = y2old;
        let r1_param = r1;
        let r2_param = r2;
        params = arc_to_cubic_curves(
            x2,
            y2,
            x2old_param,
            y2old_param,
            r1_param,
            r2_param,
            angle,
            false,
            sweep_flag,
            Some([f2, f2old, cx, cy]),
        );
    }

    // let idf = f2 - f1;
    df = f2 - f1;

    let c1 = f1.cos();
    let s1 = f1.sin();
    let c2 = f2.cos();
    let s2 = f2.sin();
    let t = (df / 4.0).tan();
    let hx = 4.0 / 3.0 * r1 * t;
    let hy = 4.0 / 3.0 * r2 * t;

    let m1 = vec![x1, y1];
    let mut m2 = vec![x1 + hx * s1, y1 - hy * c1];
    let m3 = vec![x2 + hx * s2, y2 - hy * c2];
    let m4 = vec![x2, y2];

    m2[0] = 2.0 * m1[0] - m2[0];
    m2[1] = 2.0 * m1[1] - m2[1];

    if recursive.is_some() {
        let mut ret_val = vec![m2, m3, m4];
        ret_val.append(&mut params);
        ret_val
    } else {
        let mut ret_val = vec![m2, m3, m4];
        ret_val.append(&mut params);
        let mut curves = vec![];
        for i in (0..ret_val.len()).step_by(3) {
            let r1 = rotate(ret_val[i][0], ret_val[i][1], angle_rad);
            let r2 = rotate(ret_val[i + 1][0], ret_val[i + 1][1], angle_rad);
            let r3 = rotate(ret_val[i + 2][0], ret_val[i + 2][1], angle_rad);
            curves.push(vec![r1.0, r1.1, r2.0, r2.1, r3.0, r3.1]);
        }
        curves
    }
}

#[cfg(test)]
mod test {

    use svgtypes::{PathParser, PathSegment};

    use super::absolutize;

    #[test]
    pub fn absolutize_happy_path() {
        let path: String = "m 0 0 c 3 -0.6667 6 -1.3333 9 -2 a 1 1 0 0 0 -8 -1 a 1 1 0 0 0 -2 0 l 0 4 v 2 h 8 q 4 -10 9 -5 t -6 8 z".into();
        let path_parser = PathParser::from(path.as_ref());
        let path_segments: Vec<PathSegment> = path_parser.flatten().collect();
        let mut absolute = absolutize(path_segments.iter());
        assert_eq!(
            absolute.next().unwrap(),
            PathSegment::MoveTo { abs: true, x: 0.0, y: 0.0 }
        );
        assert_eq!(
            absolute.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 3.0,
                y1: -0.6667,
                x2: 6.0,
                y2: -1.3333,
                x: 9.0,
                y: -2.0
            }
        );
        assert_eq!(
            absolute.next().unwrap(),
            PathSegment::EllipticalArc {
                abs: true,
                rx: 1.0,
                ry: 1.0,
                x_axis_rotation: 0.0,
                large_arc: false,
                sweep: false,
                x: 1.0,
                y: -3.0
            }
        );

        assert_eq!(
            absolute.next().unwrap(),
            PathSegment::EllipticalArc {
                abs: true,
                rx: 1.0,
                ry: 1.0,
                x_axis_rotation: 0.0,
                large_arc: false,
                sweep: false,
                x: -1.0,
                y: -3.0
            }
        );
        assert_eq!(
            absolute.next().unwrap(),
            PathSegment::LineTo { abs: true, x: -1.0, y: 1.0 }
        );
        assert_eq!(
            absolute.next().unwrap(),
            PathSegment::VerticalLineTo { abs: true, y: 3.0 }
        );
        assert_eq!(
            absolute.next().unwrap(),
            PathSegment::HorizontalLineTo { abs: true, x: 7.0 }
        );
        assert_eq!(
            absolute.next().unwrap(),
            PathSegment::Quadratic { abs: true, x1: 11.0, y1: -7.0, x: 16.0, y: -2.0 }
        );
        assert_eq!(
            absolute.next().unwrap(),
            PathSegment::SmoothQuadratic { abs: true, x: 10.0, y: 6.0 }
        );
        assert_eq!(
            absolute.next().unwrap(),
            PathSegment::ClosePath { abs: true }
        );
    }

    #[test]
    pub fn normalize_happy_path() {
        //M 5.5 0.5
        // C 5.5 0.22385762508460338 5.723857625084603 5.072653133236333e-17 6.0 0.0
        // L 10.0 0.0
        // C 10.38490017945975  2.356833863833126e-17 10.625462791622095 0.4166666666666666 10.433012701892219 0.7499999999999999
        // C 10.343696304415177 0.9047005383792515 10.178632794954082 1.0 10.0 1.0
        // L 9.0 1.0
        // L 9.0 2.0700000000000003
        // C 14.33449985803545, 2.8424915951774232 16.832315587265594 9.10006433409229 13.496068312614259 13.333630930046759
        // C 13.210041054045906 13.696588273349185 12.888785098427809 14.030338740866595 12.536999999999999 14.33
        // L 13.354 15.146
        // C 13.626509327057502 15.418509327057505 13.501827656468443 15.883827656468444 13.129572992939691 15.983572992939692
        // C 12.956809002353436 16.029864964698454 12.77247201882749 15.98047201882749 12.645999999999999 15.854000000000001
        // L 11.722 14.929
        // C 10.607493810968412 15.630605321208161 9.316954914290129 16.00195598169794 8.0 16.0
        // C 6.683143407910542 16.00225541468121 5.392611921472294 15.63125361712913 4.2780000000000005 14.93
        // L 3.3540000000000005 15.854
        // C 3.081490668892032 16.126509323007035 2.6161723413343103 16.00182764550168 2.5164270103961 15.629572980490355
        // C 2.470135042581356 15.45680899435182 2.5195279868119926 15.272472016947662 2.646000000000001 15.145999999999999
        // L 3.463000000000001 14.329999999999998
        // C -0.6402525423176311 10.834724535324305 0.5789613007758652 4.208276195164306 5.6575849175682915 2.4023929877120027
        // C 6.092991729721756 2.247568785367413 6.5426560909258145 2.1362282004438002 7.0 2.07
        // L 7.0 1.0
        // L 5.999 1.0
        // L 5.722857625084603 1.0 5.499 0.7761423749153966 5.499 0.5
        // Z
        // M 0.86 5.387
        // C -0.5920365149431468 4.123950649034067 -0.13226830657018196 1.7626691682261253 1.6875827750713361 1.1366933345457033
        // C 2.6507190706407835 0.8054024659416974 3.7185449350320225 1.0915255439623954 4.387 1.86
        // C 2.866949304852904 2.6317462542645806 1.6317462542645806 3.866949304852904 0.86 5.387
        // Z
        // M 13.5 1.0
        // C 12.747 1.0 12.071 1.333 11.613 1.8599999999999999
        // C 13.133050695147096 2.63174625426458 14.36825374573542 3.866949304852904 15.14 5.3870000000000005
        // C 16.59261327922989 4.124614019125415 16.133923667725163 1.7631227789691712 14.31435869929149 1.1363157677187599
        // C 14.052337629032444 1.0460542784161344 13.777132057971071 0.9999875283075207 13.5 1.0
        // Z
        // M 8.5 5.0
        // C 8.5 4.61509982054025 8.083333333333334 4.374537208377905 7.75 4.56698729810778
        // C 7.5952994616207485 4.656303695584821 7.5 4.821367205045918 7.5 5.0
        // L 7.5 8.882
        // L 6.053 11.776
        // C 5.877899246607476 12.118765042426322 6.139515318575206 12.522545319767424 6.523908929541914 12.502804499213983
        // C 6.70507326307185 12.493500669761916 6.8669876555325775 12.386803875011148 6.947 12.224
        // L 8.447 9.224
        // C 8.48184726536152 9.154471430221115 8.49999479206472 9.077772449437774 8.5 9.0
        // L 8.5 5.0
        // Z
        let path: String = "M5.5.5A.5.5 0 016 0h4a.5.5 0 010 1H9v1.07a7.002 7.002 0 013.537 12.26l.817.816a.5.5 0 01-.708.708l-.924-.925A6.967 6.967 0 018 16a6.967 6.967 0 01-3.722-1.07l-.924.924a.5.5 0 01-.708-.708l.817-.816A7.002 7.002 0 017 2.07V1H5.999a.5.5 0 01-.5-.5zM.86 5.387A2.5 2.5 0 114.387 1.86 8.035 8.035 0 00.86 5.387zM13.5 1c-.753 0-1.429.333-1.887.86a8.035 8.035 0 013.527 3.527A2.5 2.5 0 0013.5 1zm-5 4a.5.5 0 00-1 0v3.882l-1.447 2.894a.5.5 0 10.894.448l1.5-3A.5.5 0 008.5 9V5z".into();
        let path_parser = PathParser::from(path.as_ref());
        let path_segments: Vec<PathSegment> = path_parser.flatten().collect();
        let absolute = absolutize(path_segments.iter());
        let mut normalized = super::normalize(absolute);
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::MoveTo { abs: true, x: 5.5, y: 0.5 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 5.5,
                y1: 0.22385762508460338,
                x2: 5.723857625084603,
                y2: 5.072653133236333e-17,
                x: 6.0,
                y: 0.0,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 10.0, y: 0.0 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 10.38490017945975,
                y1: 2.356833863833126e-17,
                x2: 10.625462791622095,
                y2: 0.4166666666666666,
                x: 10.433012701892219,
                y: 0.7499999999999999,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 10.343696304415177,
                y1: 0.9047005383792515,
                x2: 10.178632794954082,
                y2: 1.0,
                x: 10.0,
                y: 1.0
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 9.0, y: 1.0 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 9.0, y: 2.0700000000000003 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 14.33449985803545,
                y1: 2.8424915951774232,
                x2: 16.832315587265594,
                y2: 9.10006433409229,
                x: 13.496068312614259,
                y: 13.333630930046759,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 13.210041054045906,
                y1: 13.696588273349185,
                x2: 12.888785098427809,
                y2: 14.030338740866595,
                x: 12.536999999999999,
                y: 14.33,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 13.354, y: 15.146 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 13.626509327057502,
                y1: 15.418509327057505,
                x2: 13.501827656468443,
                y2: 15.883827656468444,
                x: 13.129572992939691,
                y: 15.983572992939692,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 12.956809002353436,
                y1: 16.029864964698454,
                x2: 12.77247201882749,
                y2: 15.98047201882749,
                x: 12.645999999999999,
                y: 15.854000000000001,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 11.722, y: 14.929 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 10.607493810968412,
                y1: 15.630605321208161,
                x2: 9.316954914290129,
                y2: 16.00195598169794,
                x: 8.0,
                y: 16.0,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 6.683143407910542,
                y1: 16.00225541468121,
                x2: 5.392611921472294,
                y2: 15.63125361712913,
                x: 4.2780000000000005,
                y: 14.93,
            }
        );

        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 3.3540000000000005, y: 15.854 }
        );

        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 3.081490668892032,
                y1: 16.126509323007035,
                x2: 2.6161723413343103,
                y2: 16.00182764550168,
                x: 2.5164270103961,
                y: 15.629572980490355,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 2.470135042581356,
                y1: 15.45680899435182,
                x2: 2.5195279868119926,
                y2: 15.272472016947662,
                x: 2.646000000000001,
                y: 15.145999999999999,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo {
                abs: true,
                x: 3.463000000000001,
                y: 14.329999999999998,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: -0.6402525423176311,
                y1: 10.834724535324305,
                x2: 0.5789613007758652,
                y2: 4.208276195164306,
                x: 5.6575849175682915,
                y: 2.4023929877120027,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 6.092991729721756,
                y1: 2.247568785367413,
                x2: 6.5426560909258145,
                y2: 2.1362282004438002,
                x: 7.0,
                y: 2.07,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 7.0, y: 1.0 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 5.999, y: 1.0 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 5.722857625084603,
                y1: 1.0,
                x2: 5.499,
                y2: 0.7761423749153966,
                x: 5.499,
                y: 0.5,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::ClosePath { abs: true }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::MoveTo { abs: true, x: 0.86, y: 5.387 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: -0.5920365149431468,
                y1: 4.123950649034067,
                x2: -0.13226830657018196,
                y2: 1.7626691682261253,
                x: 1.6875827750713361,
                y: 1.1366933345457033,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 2.6507190706407835,
                y1: 0.8054024659416974,
                x2: 3.7185449350320225,
                y2: 1.0915255439623954,
                x: 4.387,
                y: 1.86,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 2.866949304852904,
                y1: 2.6317462542645806,
                x2: 1.6317462542645806,
                y2: 3.866949304852904,
                x: 0.86,
                y: 5.387
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::ClosePath { abs: true }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::MoveTo { abs: true, x: 13.5, y: 1.0 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 12.747,
                y1: 1.0,
                x2: 12.071,
                y2: 1.333,
                x: 11.613,
                y: 1.8599999999999999,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 13.133050695147096,
                y1: 2.63174625426458,
                x2: 14.36825374573542,
                y2: 3.866949304852904,
                x: 15.14,
                y: 5.3870000000000005,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 16.59261327922989,
                y1: 4.124614019125415,
                x2: 16.133923667725163,
                y2: 1.7631227789691712,
                x: 14.31435869929149,
                y: 1.1363157677187599,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 14.052337629032444,
                y1: 1.0460542784161344,
                x2: 13.777132057971071,
                y2: 0.9999875283075207,
                x: 13.5,
                y: 1.0,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::ClosePath { abs: true }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::MoveTo { abs: true, x: 8.5, y: 5.0 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 8.5,
                y1: 4.61509982054025,
                x2: 8.083333333333334,
                y2: 4.374537208377905,
                x: 7.75,
                y: 4.56698729810778,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 7.5952994616207485,
                y1: 4.656303695584821,
                x2: 7.5,
                y2: 4.821367205045918,
                x: 7.5,
                y: 5.0,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 7.5, y: 8.882 }
        );

        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 6.053, y: 11.776 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 5.877899246607476,
                y1: 12.118765042426322,
                x2: 6.139515318575206,
                y2: 12.522545319767424,
                x: 6.523908929541914,
                y: 12.502804499213983,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 6.70507326307185,
                y1: 12.493500669761916,
                x2: 6.8669876555325775,
                y2: 12.386803875011148,
                x: 6.947,
                y: 12.224,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 8.447, y: 9.224 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::CurveTo {
                abs: true,
                x1: 8.48184726536152,
                y1: 9.154471430221115,
                x2: 8.49999479206472,
                y2: 9.077772449437774,
                x: 8.5,
                y: 9.0,
            }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::LineTo { abs: true, x: 8.5, y: 5.0 }
        );
        assert_eq!(
            normalized.next().unwrap(),
            PathSegment::ClosePath { abs: true }
        );
    }

    #[test]
    pub fn arc_to_cubic_curves() {
        let result = super::arc_to_cubic_curves(
            79.5, 257.83, 84.25, 249.60, 9.50, 9.50, 90.0, false, true, None,
        );
        assert_eq!(
            result[0],
            vec![
                79.49901422066253,
                254.4349913614547,
                81.30983606638188,
                251.29750424771456,
                84.25,
                249.6
            ]
        );
    }
}
