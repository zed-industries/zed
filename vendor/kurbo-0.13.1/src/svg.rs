// Copyright 2018 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! SVG path representation.

use alloc::vec::Vec;
use core::error::Error;
use core::f64::consts::PI;
use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::io::{self, Write};

use crate::{Arc, BezPath, ParamCurve, PathEl, PathSeg, Point, Vec2};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

// Note: the SVG arc logic is heavily adapted from https://github.com/nical/lyon

/// A single SVG arc segment.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SvgArc {
    /// The arc's start point.
    pub from: Point,
    /// The arc's end point.
    pub to: Point,
    /// The arc's radii, where the vector's x-component is the radius in the
    /// positive x direction after applying `x_rotation`.
    pub radii: Vec2,
    /// How much the arc is rotated, in radians.
    pub x_rotation: f64,
    /// Does this arc sweep through more than π radians?
    pub large_arc: bool,
    /// Determines if the arc should begin moving at positive angles.
    pub sweep: bool,
}

impl BezPath {
    /// Create a `BezPath` with segments corresponding to the sequence of
    /// `PathSeg`s
    pub fn from_path_segments(segments: impl Iterator<Item = PathSeg>) -> BezPath {
        let mut path_elements = Vec::new();
        let mut current_pos = None;

        for segment in segments {
            let start = segment.start();
            if Some(start) != current_pos {
                path_elements.push(PathEl::MoveTo(start));
            }
            path_elements.push(match segment {
                PathSeg::Line(l) => PathEl::LineTo(l.p1),
                PathSeg::Quad(q) => PathEl::QuadTo(q.p1, q.p2),
                PathSeg::Cubic(c) => PathEl::CurveTo(c.p1, c.p2, c.p3),
            });

            current_pos = Some(segment.end());
        }

        BezPath::from_vec(path_elements)
    }

    /// Convert the path to an SVG path string representation.
    ///
    /// The current implementation doesn't take any special care to produce a
    /// short string (reducing precision, using relative movement).
    #[cfg(feature = "std")]
    pub fn to_svg(&self) -> String {
        let mut buffer = Vec::new();
        self.write_to(&mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    /// Write the SVG representation of this path to the provided buffer.
    #[cfg(feature = "std")]
    pub fn write_to<W: Write>(&self, mut writer: W) -> io::Result<()> {
        for (i, el) in self.elements().iter().enumerate() {
            if i > 0 {
                write!(writer, " ")?;
            }
            match *el {
                PathEl::MoveTo(p) => write!(writer, "M{},{}", p.x, p.y)?,
                PathEl::LineTo(p) => write!(writer, "L{},{}", p.x, p.y)?,
                PathEl::QuadTo(p1, p2) => write!(writer, "Q{},{} {},{}", p1.x, p1.y, p2.x, p2.y)?,
                PathEl::CurveTo(p1, p2, p3) => write!(
                    writer,
                    "C{},{} {},{} {},{}",
                    p1.x, p1.y, p2.x, p2.y, p3.x, p3.y
                )?,
                PathEl::ClosePath => write!(writer, "Z")?,
            }
        }

        Ok(())
    }

    /// Try to parse a bezier path from an SVG path element.
    ///
    /// This is implemented on a best-effort basis, intended for cases where the
    /// user controls the source of paths, and is not intended as a replacement
    /// for a general, robust SVG parser.
    pub fn from_svg(data: &str) -> Result<BezPath, SvgParseError> {
        let mut lexer = SvgLexer::new(data);
        let mut path = BezPath::new();
        let mut last_cmd = 0;
        let mut last_ctrl = None;
        let mut first_pt = Point::ORIGIN;
        let mut implicit_moveto = None;
        while let Some(c) = lexer.get_cmd(last_cmd) {
            if c != b'm' && c != b'M' {
                if path.elements().is_empty() {
                    return Err(SvgParseError::UninitializedPath);
                }

                if let Some(pt) = implicit_moveto.take() {
                    path.move_to(pt);
                }
            }
            match c {
                b'm' | b'M' => {
                    implicit_moveto = None;
                    let pt = lexer.get_maybe_relative(c)?;
                    path.move_to(pt);
                    lexer.last_pt = pt;
                    first_pt = pt;
                    last_ctrl = Some(pt);
                    last_cmd = c - (b'M' - b'L');
                }
                b'l' | b'L' => {
                    let pt = lexer.get_maybe_relative(c)?;
                    path.line_to(pt);
                    lexer.last_pt = pt;
                    last_ctrl = Some(pt);
                    last_cmd = c;
                }
                b'h' | b'H' => {
                    let mut x = lexer.get_number()?;
                    lexer.opt_comma();
                    if c == b'h' {
                        x += lexer.last_pt.x;
                    }
                    let pt = Point::new(x, lexer.last_pt.y);
                    path.line_to(pt);
                    lexer.last_pt = pt;
                    last_ctrl = Some(pt);
                    last_cmd = c;
                }
                b'v' | b'V' => {
                    let mut y = lexer.get_number()?;
                    lexer.opt_comma();
                    if c == b'v' {
                        y += lexer.last_pt.y;
                    }
                    let pt = Point::new(lexer.last_pt.x, y);
                    path.line_to(pt);
                    lexer.last_pt = pt;
                    last_ctrl = Some(pt);
                    last_cmd = c;
                }
                b'q' | b'Q' => {
                    let p1 = lexer.get_maybe_relative(c)?;
                    let p2 = lexer.get_maybe_relative(c)?;
                    path.quad_to(p1, p2);
                    last_ctrl = Some(p1);
                    lexer.last_pt = p2;
                    last_cmd = c;
                }
                b't' | b'T' => {
                    let p1 = match last_ctrl {
                        Some(ctrl) => (2.0 * lexer.last_pt.to_vec2() - ctrl.to_vec2()).to_point(),
                        None => lexer.last_pt,
                    };
                    let p2 = lexer.get_maybe_relative(c)?;
                    path.quad_to(p1, p2);
                    last_ctrl = Some(p1);
                    lexer.last_pt = p2;
                    last_cmd = c;
                }
                b'c' | b'C' => {
                    let p1 = lexer.get_maybe_relative(c)?;
                    let p2 = lexer.get_maybe_relative(c)?;
                    let p3 = lexer.get_maybe_relative(c)?;
                    path.curve_to(p1, p2, p3);
                    last_ctrl = Some(p2);
                    lexer.last_pt = p3;
                    last_cmd = c;
                }
                b's' | b'S' => {
                    let p1 = match last_ctrl {
                        Some(ctrl) => (2.0 * lexer.last_pt.to_vec2() - ctrl.to_vec2()).to_point(),
                        None => lexer.last_pt,
                    };
                    let p2 = lexer.get_maybe_relative(c)?;
                    let p3 = lexer.get_maybe_relative(c)?;
                    path.curve_to(p1, p2, p3);
                    last_ctrl = Some(p2);
                    lexer.last_pt = p3;
                    last_cmd = c;
                }
                b'a' | b'A' => {
                    let radii = lexer.get_number_pair()?;
                    let x_rotation = lexer.get_number()?.to_radians();
                    lexer.opt_comma();
                    let large_arc = lexer.get_flag()?;
                    lexer.opt_comma();
                    let sweep = lexer.get_flag()?;
                    lexer.opt_comma();
                    let p = lexer.get_maybe_relative(c)?;
                    let svg_arc = SvgArc {
                        from: lexer.last_pt,
                        to: p,
                        radii: radii.to_vec2(),
                        x_rotation,
                        large_arc,
                        sweep,
                    };

                    match Arc::from_svg_arc(&svg_arc) {
                        Some(arc) => {
                            // TODO: consider making tolerance configurable
                            arc.to_cubic_beziers(0.1, |p1, p2, p3| {
                                path.curve_to(p1, p2, p3);
                            });
                        }
                        None => {
                            path.line_to(p);
                        }
                    }

                    last_ctrl = Some(p);
                    lexer.last_pt = p;
                    last_cmd = c;
                }
                b'z' | b'Z' => {
                    path.close_path();
                    lexer.last_pt = first_pt;
                    implicit_moveto = Some(first_pt);
                }
                _ => return Err(SvgParseError::UnknownCommand(c as char)),
            }
        }
        Ok(path)
    }
}

/// An error which can be returned when parsing an SVG.
#[derive(Debug)]
#[non_exhaustive]
pub enum SvgParseError {
    /// A number was expected.
    Wrong,
    /// The input string ended while still expecting input.
    UnexpectedEof,
    /// Encountered an unknown command letter.
    UnknownCommand(char),
    /// Encountered a command that precedes expected 'moveto' command.
    UninitializedPath,
}

impl Display for SvgParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SvgParseError::Wrong => write!(f, "Unable to parse a number"),
            SvgParseError::UnexpectedEof => write!(f, "Unexpected EOF"),
            SvgParseError::UnknownCommand(letter) => write!(f, "Unknown command, \"{letter}\""),
            SvgParseError::UninitializedPath => {
                write!(f, "Uninitialized path (missing moveto command)")
            }
        }
    }
}

impl Error for SvgParseError {}

struct SvgLexer<'a> {
    data: &'a str,
    ix: usize,
    pub last_pt: Point,
}

impl SvgLexer<'_> {
    fn new(data: &str) -> SvgLexer<'_> {
        SvgLexer {
            data,
            ix: 0,
            last_pt: Point::ORIGIN,
        }
    }

    fn skip_ws(&mut self) {
        while let Some(&c) = self.data.as_bytes().get(self.ix) {
            if !(c == b' ' || c == 9 || c == 10 || c == 12 || c == 13) {
                break;
            }
            self.ix += 1;
        }
    }

    fn get_cmd(&mut self, last_cmd: u8) -> Option<u8> {
        self.skip_ws();
        if let Some(c) = self.get_byte() {
            if c.is_ascii_lowercase() || c.is_ascii_uppercase() {
                return Some(c);
            } else if last_cmd != 0 && (c == b'-' || c == b'.' || c.is_ascii_digit()) {
                // Plausible number start
                self.unget();
                return Some(last_cmd);
            } else {
                self.unget();
            }
        }
        None
    }

    fn get_byte(&mut self) -> Option<u8> {
        self.data.as_bytes().get(self.ix).map(|&c| {
            self.ix += 1;
            c
        })
    }

    fn unget(&mut self) {
        self.ix -= 1;
    }

    fn get_number(&mut self) -> Result<f64, SvgParseError> {
        self.skip_ws();
        let start = self.ix;
        let c = self.get_byte().ok_or(SvgParseError::UnexpectedEof)?;
        if !(c == b'-' || c == b'+') {
            self.unget();
        }
        let mut digit_count = 0;
        let mut seen_period = false;
        while let Some(c) = self.get_byte() {
            if c.is_ascii_digit() {
                digit_count += 1;
            } else if c == b'.' && !seen_period {
                seen_period = true;
            } else {
                self.unget();
                break;
            }
        }
        if let Some(c) = self.get_byte() {
            if c == b'e' || c == b'E' {
                let mut c = self.get_byte().ok_or(SvgParseError::Wrong)?;
                if c == b'-' || c == b'+' {
                    c = self.get_byte().ok_or(SvgParseError::Wrong)?;
                }
                if !c.is_ascii_digit() {
                    return Err(SvgParseError::Wrong);
                }
                while let Some(c) = self.get_byte() {
                    if !c.is_ascii_digit() {
                        self.unget();
                        break;
                    }
                }
            } else {
                self.unget();
            }
        }
        if digit_count > 0 {
            self.data[start..self.ix]
                .parse()
                .map_err(|_| SvgParseError::Wrong)
        } else {
            Err(SvgParseError::Wrong)
        }
    }

    fn get_flag(&mut self) -> Result<bool, SvgParseError> {
        self.skip_ws();
        match self.get_byte().ok_or(SvgParseError::UnexpectedEof)? {
            b'0' => Ok(false),
            b'1' => Ok(true),
            _ => Err(SvgParseError::Wrong),
        }
    }

    fn get_number_pair(&mut self) -> Result<Point, SvgParseError> {
        let x = self.get_number()?;
        self.opt_comma();
        let y = self.get_number()?;
        self.opt_comma();
        Ok(Point::new(x, y))
    }

    fn get_maybe_relative(&mut self, cmd: u8) -> Result<Point, SvgParseError> {
        let pt = self.get_number_pair()?;
        if cmd.is_ascii_lowercase() {
            Ok(self.last_pt + pt.to_vec2())
        } else {
            Ok(pt)
        }
    }

    fn opt_comma(&mut self) {
        self.skip_ws();
        if let Some(c) = self.get_byte() {
            if c != b',' {
                self.unget();
            }
        }
    }
}

impl SvgArc {
    /// Checks that arc is actually a straight line.
    ///
    /// In this case, it can be replaced with a `LineTo`.
    pub fn is_straight_line(&self) -> bool {
        self.radii.x.abs() <= 1e-5 || self.radii.y.abs() <= 1e-5 || self.from == self.to
    }
}

impl Arc {
    /// Creates an `Arc` from a `SvgArc`.
    ///
    /// Returns `None` if `arc` is actually a straight line.
    pub fn from_svg_arc(arc: &SvgArc) -> Option<Arc> {
        // Have to check this first, otherwise `sum_of_sq` will be 0.
        if arc.is_straight_line() {
            return None;
        }

        let mut rx = arc.radii.x.abs();
        let mut ry = arc.radii.y.abs();

        let xr = arc.x_rotation % (2.0 * PI);
        let (sin_phi, cos_phi) = xr.sin_cos();
        let hd_x = (arc.from.x - arc.to.x) * 0.5;
        let hd_y = (arc.from.y - arc.to.y) * 0.5;
        let hs_x = (arc.from.x + arc.to.x) * 0.5;
        let hs_y = (arc.from.y + arc.to.y) * 0.5;

        // F6.5.1
        let p = Vec2::new(
            cos_phi * hd_x + sin_phi * hd_y,
            -sin_phi * hd_x + cos_phi * hd_y,
        );

        // Sanitize the radii.
        // If rf > 1 it means the radii are too small for the arc to
        // possibly connect the end points. In this situation we scale
        // them up according to the formula provided by the SVG spec.

        // F6.6.2
        let rf = p.x * p.x / (rx * rx) + p.y * p.y / (ry * ry);
        if rf > 1.0 {
            let scale = rf.sqrt();
            rx *= scale;
            ry *= scale;
        }

        let rxry = rx * ry;
        let rxpy = rx * p.y;
        let rypx = ry * p.x;
        let sum_of_sq = rxpy * rxpy + rypx * rypx;

        debug_assert!(sum_of_sq != 0.0);

        // F6.5.2
        let sign_coe = if arc.large_arc == arc.sweep {
            -1.0
        } else {
            1.0
        };
        let coe = sign_coe * ((rxry * rxry - sum_of_sq) / sum_of_sq).abs().sqrt();
        let transformed_cx = coe * rxpy / ry;
        let transformed_cy = -coe * rypx / rx;

        // F6.5.3
        let center = Point::new(
            cos_phi * transformed_cx - sin_phi * transformed_cy + hs_x,
            sin_phi * transformed_cx + cos_phi * transformed_cy + hs_y,
        );

        let start_v = Vec2::new((p.x - transformed_cx) / rx, (p.y - transformed_cy) / ry);
        let end_v = Vec2::new((-p.x - transformed_cx) / rx, (-p.y - transformed_cy) / ry);

        let start_angle = start_v.atan2();

        let mut sweep_angle = (end_v.atan2() - start_angle) % (2.0 * PI);

        if arc.sweep && sweep_angle < 0.0 {
            sweep_angle += 2.0 * PI;
        } else if !arc.sweep && sweep_angle > 0.0 {
            sweep_angle -= 2.0 * PI;
        }

        Some(Arc {
            center,
            radii: Vec2::new(rx, ry),
            start_angle,
            sweep_angle,
            x_rotation: arc.x_rotation,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{BezPath, CubicBez, Line, ParamCurve, PathEl, PathSeg, Point, QuadBez, Shape};

    #[test]
    fn test_parse_svg() {
        let path = BezPath::from_svg("m10 10 100 0 0 100 -100 0z").unwrap();
        assert_eq!(path.segments().count(), 4);
    }

    #[test]
    fn test_parse_svg2() {
        let path =
            BezPath::from_svg("M3.5 8a.5.5 0 01.5-.5h8a.5.5 0 010 1H4a.5.5 0 01-.5-.5z").unwrap();
        assert_eq!(path.segments().count(), 6);
    }

    #[test]
    fn test_parse_svg_arc() {
        let path = BezPath::from_svg("M 100 100 A 25 25 0 1 0 -25 25 z").unwrap();
        assert_eq!(path.segments().count(), 3);
    }

    // Regression test for #51
    #[test]
    #[allow(clippy::float_cmp)]
    fn test_parse_svg_arc_pie() {
        let path = BezPath::from_svg("M 100 100 h 25 a 25 25 0 1 0 -25 25 z").unwrap();
        // Approximate figures, but useful for regression testing
        assert_eq!(path.area().round(), -1473.0);
        assert_eq!(path.perimeter(1e-6).round(), 168.0);
    }

    #[test]
    fn test_parse_svg_uninitialized() {
        let path = BezPath::from_svg("L10 10 100 0 0 100");
        assert!(path.is_err());
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_parse_scientific_notation() {
        let path = BezPath::from_svg("M 0 0 L 1e-123 -4E+5").unwrap();
        assert_eq!(
            path.elements(),
            &[
                PathEl::MoveTo(Point { x: 0.0, y: 0.0 }),
                PathEl::LineTo(Point {
                    x: 1e-123,
                    y: -4E+5
                })
            ]
        );
    }

    #[test]
    fn test_write_svg_single() {
        let segments = [CubicBez::new(
            Point::new(10., 10.),
            Point::new(20., 20.),
            Point::new(30., 30.),
            Point::new(40., 40.),
        )
        .into()];
        let path = BezPath::from_path_segments(segments.iter().cloned());

        assert_eq!(path.to_svg(), "M10,10 C20,20 30,30 40,40");
    }

    #[test]
    fn test_write_svg_two_nomove() {
        let segments = [
            CubicBez::new(
                Point::new(10., 10.),
                Point::new(20., 20.),
                Point::new(30., 30.),
                Point::new(40., 40.),
            )
            .into(),
            CubicBez::new(
                Point::new(40., 40.),
                Point::new(30., 30.),
                Point::new(20., 20.),
                Point::new(10., 10.),
            )
            .into(),
        ];
        let path = BezPath::from_path_segments(segments.iter().cloned());

        assert_eq!(
            path.to_svg(),
            "M10,10 C20,20 30,30 40,40 C30,30 20,20 10,10"
        );
    }

    #[test]
    fn test_write_svg_two_move() {
        let segments = [
            CubicBez::new(
                Point::new(10., 10.),
                Point::new(20., 20.),
                Point::new(30., 30.),
                Point::new(40., 40.),
            )
            .into(),
            CubicBez::new(
                Point::new(50., 50.),
                Point::new(30., 30.),
                Point::new(20., 20.),
                Point::new(10., 10.),
            )
            .into(),
        ];
        let path = BezPath::from_path_segments(segments.iter().cloned());

        assert_eq!(
            path.to_svg(),
            "M10,10 C20,20 30,30 40,40 M50,50 C30,30 20,20 10,10"
        );
    }

    use rand::prelude::*;
    // Suppress the unused_crate_dependencies lint for getrandom.
    #[cfg(all(
        target_arch = "wasm32",
        target_vendor = "unknown",
        target_os = "unknown"
    ))]
    use getrandom as _;

    fn gen_random_path_sequence(rng: &mut impl Rng) -> Vec<PathSeg> {
        const MAX_LENGTH: u32 = 10;

        let mut elements = vec![];
        let mut position = None;

        let length = rng.random_range(0..MAX_LENGTH);
        for _ in 0..length {
            let should_follow: bool = rand::random_bool(0.5);
            let kind = rng.random_range(0..3);

            let first = position
                .filter(|_| should_follow)
                .unwrap_or_else(|| Point::new(rng.random(), rng.random()));

            let element: PathSeg = match kind {
                0 => Line::new(first, Point::new(rng.random(), rng.random())).into(),

                1 => QuadBez::new(
                    first,
                    Point::new(rng.random(), rng.random()),
                    Point::new(rng.random(), rng.random()),
                )
                .into(),

                2 => CubicBez::new(
                    first,
                    Point::new(rng.random(), rng.random()),
                    Point::new(rng.random(), rng.random()),
                    Point::new(rng.random(), rng.random()),
                )
                .into(),

                _ => unreachable!(),
            };

            position = Some(element.end());
            elements.push(element);
        }

        elements
    }

    #[test]
    fn test_serialize_deserialize() {
        const N_TESTS: u32 = 100;
        let mut rng = rand::rng();

        for _ in 0..N_TESTS {
            let vec = gen_random_path_sequence(&mut rng);
            let ser = BezPath::from_path_segments(vec.iter().cloned()).to_svg();
            let deser = BezPath::from_svg(&ser).expect("failed deserialization");

            let deser_vec = deser.segments().collect::<Vec<PathSeg>>();

            assert_eq!(vec, deser_vec);
        }
    }
}
