use crate::{Error, Stream};

/// Representation of a path segment.
///
/// If you want to change the segment type (for example MoveTo to LineTo)
/// you should create a new segment.
/// But you still can change points or make segment relative or absolute.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PathSegment {
    MoveTo {
        abs: bool,
        x: f64,
        y: f64,
    },
    LineTo {
        abs: bool,
        x: f64,
        y: f64,
    },
    HorizontalLineTo {
        abs: bool,
        x: f64,
    },
    VerticalLineTo {
        abs: bool,
        y: f64,
    },
    CurveTo {
        abs: bool,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    SmoothCurveTo {
        abs: bool,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    Quadratic {
        abs: bool,
        x1: f64,
        y1: f64,
        x: f64,
        y: f64,
    },
    SmoothQuadratic {
        abs: bool,
        x: f64,
        y: f64,
    },
    EllipticalArc {
        abs: bool,
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc: bool,
        sweep: bool,
        x: f64,
        y: f64,
    },
    ClosePath {
        abs: bool,
    },
}

/// A pull-based [path data] parser.
///
/// # Errors
///
/// - Most of the `Error` types can occur.
///
/// # Notes
///
/// The library does not support implicit commands, so they will be converted to an explicit one.
/// It mostly affects an implicit MoveTo, which will be converted, according to the spec,
/// into explicit LineTo.
///
/// Example: `M 10 20 30 40 50 60` -> `M 10 20 L 30 40 L 50 60`
///
/// # Examples
///
/// ```
/// use svgtypes::{PathParser, PathSegment};
///
/// let mut segments = Vec::new();
/// for segment in PathParser::from("M10-20l30.1.5.1-20z") {
///     segments.push(segment.unwrap());
/// }
///
/// assert_eq!(segments, &[
///     PathSegment::MoveTo { abs: true, x: 10.0, y: -20.0 },
///     PathSegment::LineTo { abs: false, x: 30.1, y: 0.5 },
///     PathSegment::LineTo { abs: false, x: 0.1, y: -20.0 },
///     PathSegment::ClosePath { abs: false },
/// ]);
/// ```
///
/// [path data]: https://www.w3.org/TR/SVG2/paths.html#PathData
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PathParser<'a> {
    stream: Stream<'a>,
    prev_cmd: Option<u8>,
}

impl<'a> From<&'a str> for PathParser<'a> {
    #[inline]
    fn from(v: &'a str) -> Self {
        PathParser {
            stream: Stream::from(v),
            prev_cmd: None,
        }
    }
}

impl<'a> Iterator for PathParser<'a> {
    type Item = Result<PathSegment, Error>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let s = &mut self.stream;

        s.skip_spaces();

        if s.at_end() {
            return None;
        }

        let res = next_impl(s, &mut self.prev_cmd);
        if res.is_err() {
            s.jump_to_end();
        }

        Some(res)
    }
}

fn next_impl(s: &mut Stream, prev_cmd: &mut Option<u8>) -> Result<PathSegment, Error> {
    let start = s.pos();

    let has_prev_cmd = prev_cmd.is_some();
    let first_char = s.curr_byte_unchecked();

    if !has_prev_cmd && !is_cmd(first_char) {
        return Err(Error::UnexpectedData(s.calc_char_pos_at(start)));
    }

    if !has_prev_cmd && !matches!(first_char, b'M' | b'm') {
        // The first segment must be a MoveTo.
        return Err(Error::UnexpectedData(s.calc_char_pos_at(start)));
    }

    // TODO: simplify
    let is_implicit_move_to;
    let cmd: u8;
    if is_cmd(first_char) {
        is_implicit_move_to = false;
        cmd = first_char;
        s.advance(1);
    } else if is_number_start(first_char) && has_prev_cmd {
        // unwrap is safe, because we checked 'has_prev_cmd'
        let p_cmd = prev_cmd.unwrap();

        if p_cmd == b'Z' || p_cmd == b'z' {
            // ClosePath cannot be followed by a number.
            return Err(Error::UnexpectedData(s.calc_char_pos_at(start)));
        }

        if p_cmd == b'M' || p_cmd == b'm' {
            // 'If a moveto is followed by multiple pairs of coordinates,
            // the subsequent pairs are treated as implicit lineto commands.'
            // So we parse them as LineTo.
            is_implicit_move_to = true;
            cmd = if is_absolute(p_cmd) { b'L' } else { b'l' };
        } else {
            is_implicit_move_to = false;
            cmd = p_cmd;
        }
    } else {
        return Err(Error::UnexpectedData(s.calc_char_pos_at(start)));
    }

    let cmdl = to_relative(cmd);
    let absolute = is_absolute(cmd);
    let token = match cmdl {
        b'm' => PathSegment::MoveTo {
            abs: absolute,
            x: s.parse_list_number()?,
            y: s.parse_list_number()?,
        },
        b'l' => PathSegment::LineTo {
            abs: absolute,
            x: s.parse_list_number()?,
            y: s.parse_list_number()?,
        },
        b'h' => PathSegment::HorizontalLineTo {
            abs: absolute,
            x: s.parse_list_number()?,
        },
        b'v' => PathSegment::VerticalLineTo {
            abs: absolute,
            y: s.parse_list_number()?,
        },
        b'c' => PathSegment::CurveTo {
            abs: absolute,
            x1: s.parse_list_number()?,
            y1: s.parse_list_number()?,
            x2: s.parse_list_number()?,
            y2: s.parse_list_number()?,
            x: s.parse_list_number()?,
            y: s.parse_list_number()?,
        },
        b's' => PathSegment::SmoothCurveTo {
            abs: absolute,
            x2: s.parse_list_number()?,
            y2: s.parse_list_number()?,
            x: s.parse_list_number()?,
            y: s.parse_list_number()?,
        },
        b'q' => PathSegment::Quadratic {
            abs: absolute,
            x1: s.parse_list_number()?,
            y1: s.parse_list_number()?,
            x: s.parse_list_number()?,
            y: s.parse_list_number()?,
        },
        b't' => PathSegment::SmoothQuadratic {
            abs: absolute,
            x: s.parse_list_number()?,
            y: s.parse_list_number()?,
        },
        b'a' => {
            // TODO: radius cannot be negative
            PathSegment::EllipticalArc {
                abs: absolute,
                rx: s.parse_list_number()?,
                ry: s.parse_list_number()?,
                x_axis_rotation: s.parse_list_number()?,
                large_arc: parse_flag(s)?,
                sweep: parse_flag(s)?,
                x: s.parse_list_number()?,
                y: s.parse_list_number()?,
            }
        }
        b'z' => PathSegment::ClosePath { abs: absolute },
        _ => unreachable!(),
    };

    *prev_cmd = Some(if is_implicit_move_to {
        if absolute {
            b'M'
        } else {
            b'm'
        }
    } else {
        cmd
    });

    Ok(token)
}

/// Returns `true` if the selected char is the command.
#[rustfmt::skip]
#[inline]
fn is_cmd(c: u8) -> bool {
    matches!(c,
          b'M' | b'm'
        | b'Z' | b'z'
        | b'L' | b'l'
        | b'H' | b'h'
        | b'V' | b'v'
        | b'C' | b'c'
        | b'S' | b's'
        | b'Q' | b'q'
        | b'T' | b't'
        | b'A' | b'a')
}

/// Returns `true` if the selected char is the absolute command.
#[inline]
fn is_absolute(c: u8) -> bool {
    debug_assert!(is_cmd(c));
    matches!(
        c,
        b'M' | b'Z' | b'L' | b'H' | b'V' | b'C' | b'S' | b'Q' | b'T' | b'A'
    )
}

/// Converts the selected command char into the relative command char.
#[inline]
fn to_relative(c: u8) -> u8 {
    debug_assert!(is_cmd(c));
    match c {
        b'M' => b'm',
        b'Z' => b'z',
        b'L' => b'l',
        b'H' => b'h',
        b'V' => b'v',
        b'C' => b'c',
        b'S' => b's',
        b'Q' => b'q',
        b'T' => b't',
        b'A' => b'a',
        _ => c,
    }
}

#[inline]
fn is_number_start(c: u8) -> bool {
    matches!(c, b'0'..=b'9' | b'.' | b'-' | b'+')
}

// By the SVG spec 'large-arc' and 'sweep' must contain only one char
// and can be written without any separators, e.g.: 10 20 30 01 10 20.
fn parse_flag(s: &mut Stream) -> Result<bool, Error> {
    s.skip_spaces();

    let c = s.curr_byte()?;
    match c {
        b'0' | b'1' => {
            s.advance(1);
            if s.is_curr_byte_eq(b',') {
                s.advance(1);
            }
            s.skip_spaces();

            Ok(c == b'1')
        }
        _ => Err(Error::UnexpectedData(s.calc_char_pos_at(s.pos()))),
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $text:expr, $( $seg:expr ),*) => (
            #[test]
            fn $name() {
                let mut s = PathParser::from($text);
                $(
                    assert_eq!(s.next().unwrap().unwrap(), $seg);
                )*

                if let Some(res) = s.next() {
                    assert!(res.is_err());
                }
            }
        )
    }

    test!(null, "", );
    test!(not_a_path, "q", );
    test!(not_a_move_to, "L 20 30", );
    test!(stop_on_err_1, "M 10 20 L 30 40 L 50",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 }
    );

    test!(move_to_1, "M 10 20", PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 });
    test!(move_to_2, "m 10 20", PathSegment::MoveTo { abs: false, x: 10.0, y: 20.0 });
    test!(move_to_3, "M 10 20 30 40 50 60",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::LineTo { abs: true, x: 50.0, y: 60.0 }
    );
    test!(move_to_4, "M 10 20 30 40 50 60 M 70 80 90 100 110 120",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::LineTo { abs: true, x: 50.0, y: 60.0 },
        PathSegment::MoveTo { abs: true, x: 70.0, y: 80.0 },
        PathSegment::LineTo { abs: true, x: 90.0, y: 100.0 },
        PathSegment::LineTo { abs: true, x: 110.0, y: 120.0 }
    );

    test!(arc_to_1, "M 10 20 A 5 5 30 1 1 20 20",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::EllipticalArc {
            abs: true,
            rx: 5.0, ry: 5.0,
            x_axis_rotation: 30.0,
            large_arc: true, sweep: true,
            x: 20.0, y: 20.0
        }
    );

    test!(arc_to_2, "M 10 20 a 5 5 30 0 0 20 20",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::EllipticalArc {
            abs: false,
            rx: 5.0, ry: 5.0,
            x_axis_rotation: 30.0,
            large_arc: false, sweep: false,
            x: 20.0, y: 20.0
        }
    );

    test!(arc_to_10, "M10-20A5.5.3-4 010-.1",
        PathSegment::MoveTo { abs: true, x: 10.0, y: -20.0 },
        PathSegment::EllipticalArc {
            abs: true,
            rx: 5.5, ry: 0.3,
            x_axis_rotation: -4.0,
            large_arc: false, sweep: true,
            x: 0.0, y: -0.1
        }
    );

    test!(separator_1, "M 10 20 L 5 15 C 10 20 30 40 50 60",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 5.0, y: 15.0 },
        PathSegment::CurveTo {
            abs: true,
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    );

    test!(separator_2, "M 10, 20 L 5, 15 C 10, 20 30, 40 50, 60",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 5.0, y: 15.0 },
        PathSegment::CurveTo {
            abs: true,
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    );

    test!(separator_3, "M 10,20 L 5,15 C 10,20 30,40 50,60",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 5.0, y: 15.0 },
        PathSegment::CurveTo {
            abs: true,
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    );

    test!(separator_4, "M10, 20 L5, 15 C10, 20 30 40 50 60",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 5.0, y: 15.0 },
        PathSegment::CurveTo {
            abs: true,
            x1: 10.0, y1: 20.0,
            x2: 30.0, y2: 40.0,
            x:  50.0, y:  60.0,
        }
    );

    test!(separator_5, "M10 20V30H40V50H60Z",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::VerticalLineTo { abs: true, y: 30.0 },
        PathSegment::HorizontalLineTo { abs: true, x: 40.0 },
        PathSegment::VerticalLineTo { abs: true, y: 50.0 },
        PathSegment::HorizontalLineTo { abs: true, x: 60.0 },
        PathSegment::ClosePath { abs: true }
    );

    test!(all_segments_1, "M 10 20 L 30 40 H 50 V 60 C 70 80 90 100 110 120 S 130 140 150 160
        Q 170 180 190 200 T 210 220 A 50 50 30 1 1 230 240 Z",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::HorizontalLineTo { abs: true, x: 50.0 },
        PathSegment::VerticalLineTo { abs: true, y: 60.0 },
        PathSegment::CurveTo {
            abs: true,
            x1:  70.0, y1:  80.0,
            x2:  90.0, y2: 100.0,
            x:  110.0, y:  120.0,
        },
        PathSegment::SmoothCurveTo {
            abs: true,
            x2: 130.0, y2: 140.0,
            x:  150.0, y:  160.0,
        },
        PathSegment::Quadratic {
            abs: true,
            x1: 170.0, y1: 180.0,
            x:  190.0, y:  200.0,
        },
        PathSegment::SmoothQuadratic { abs: true, x: 210.0, y: 220.0 },
        PathSegment::EllipticalArc {
            abs: true,
            rx: 50.0, ry: 50.0,
            x_axis_rotation: 30.0,
            large_arc: true, sweep: true,
            x: 230.0, y: 240.0
        },
        PathSegment::ClosePath { abs: true }
    );

    test!(all_segments_2, "m 10 20 l 30 40 h 50 v 60 c 70 80 90 100 110 120 s 130 140 150 160
        q 170 180 190 200 t 210 220 a 50 50 30 1 1 230 240 z",
        PathSegment::MoveTo { abs: false, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: false, x: 30.0, y: 40.0 },
        PathSegment::HorizontalLineTo { abs: false, x: 50.0 },
        PathSegment::VerticalLineTo { abs: false, y: 60.0 },
        PathSegment::CurveTo {
            abs: false,
            x1:  70.0, y1:  80.0,
            x2:  90.0, y2: 100.0,
            x:  110.0, y:  120.0,
        },
        PathSegment::SmoothCurveTo {
            abs: false,
            x2: 130.0, y2: 140.0,
            x:  150.0, y:  160.0,
        },
        PathSegment::Quadratic {
            abs: false,
            x1: 170.0, y1: 180.0,
            x:  190.0, y:  200.0,
        },
        PathSegment::SmoothQuadratic { abs: false, x: 210.0, y: 220.0 },
        PathSegment::EllipticalArc {
            abs: false,
            rx: 50.0, ry: 50.0,
            x_axis_rotation: 30.0,
            large_arc: true, sweep: true,
            x: 230.0, y: 240.0
        },
        PathSegment::ClosePath { abs: false }
    );

    test!(close_path_1, "M10 20 L 30 40 ZM 100 200 L 300 400",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::ClosePath { abs: true },
        PathSegment::MoveTo { abs: true, x: 100.0, y: 200.0 },
        PathSegment::LineTo { abs: true, x: 300.0, y: 400.0 }
    );

    test!(close_path_2, "M10 20 L 30 40 zM 100 200 L 300 400",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::ClosePath { abs: false },
        PathSegment::MoveTo { abs: true, x: 100.0, y: 200.0 },
        PathSegment::LineTo { abs: true, x: 300.0, y: 400.0 }
    );

    test!(close_path_3, "M10 20 L 30 40 Z Z Z",
        PathSegment::MoveTo { abs: true, x: 10.0, y: 20.0 },
        PathSegment::LineTo { abs: true, x: 30.0, y: 40.0 },
        PathSegment::ClosePath { abs: true },
        PathSegment::ClosePath { abs: true },
        PathSegment::ClosePath { abs: true }
    );

    // first token should be EndOfStream
    test!(invalid_1, "M\t.", );

    // ClosePath can't be followed by a number
    test!(invalid_2, "M 0 0 Z 2",
        PathSegment::MoveTo { abs: true, x: 0.0, y: 0.0 },
        PathSegment::ClosePath { abs: true }
    );

    // ClosePath can be followed by any command
    test!(invalid_3, "M 0 0 Z H 10",
        PathSegment::MoveTo { abs: true, x: 0.0, y: 0.0 },
        PathSegment::ClosePath { abs: true },
        PathSegment::HorizontalLineTo { abs: true, x: 10.0 }
    );
}

/// Representation of a simple path segment.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SimplePathSegment {
    MoveTo {
        x: f64,
        y: f64,
    },
    LineTo {
        x: f64,
        y: f64,
    },
    CurveTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    Quadratic {
        x1: f64,
        y1: f64,
        x: f64,
        y: f64,
    },
    ClosePath,
}

/// A simplifying Path Data parser.
///
/// A more high-level Path Data parser on top of [`PathParser`] that provides:
///
/// - Relative to absolute segment coordinates conversion
/// - ArcTo to CurveTos conversion
/// - SmoothCurveTo and SmoothQuadratic conversion
/// - HorizontalLineTo and VerticalLineTo to LineTo conversion
/// - Implicit MoveTo after ClosePath handling
///
/// In the end, only absolute MoveTo, LineTo, CurveTo, Quadratic and ClosePath segments
/// will be produced.
#[derive(Clone, Debug)]
pub struct SimplifyingPathParser<'a> {
    parser: PathParser<'a>,

    // Previous MoveTo coordinates.
    prev_mx: f64,
    prev_my: f64,

    // Previous SmoothQuadratic coordinates.
    prev_tx: f64,
    prev_ty: f64,

    // Previous coordinates.
    prev_x: f64,
    prev_y: f64,

    prev_seg: PathSegment,
    prev_simple_seg: Option<SimplePathSegment>,

    buffer: Vec<SimplePathSegment>,
}

impl<'a> From<&'a str> for SimplifyingPathParser<'a> {
    #[inline]
    fn from(v: &'a str) -> Self {
        SimplifyingPathParser {
            parser: PathParser::from(v),
            prev_mx: 0.0,
            prev_my: 0.0,
            prev_tx: 0.0,
            prev_ty: 0.0,
            prev_x: 0.0,
            prev_y: 0.0,
            prev_seg: PathSegment::MoveTo {
                abs: true,
                x: 0.0,
                y: 0.0,
            },
            prev_simple_seg: None,
            buffer: Vec::new(),
        }
    }
}

impl<'a> Iterator for SimplifyingPathParser<'a> {
    type Item = Result<SimplePathSegment, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            return Some(Ok(self.buffer.remove(0)));
        }

        let segment = match self.parser.next()? {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };

        // If a ClosePath segment is followed by any command other than MoveTo or ClosePath
        // then MoveTo is implicit.
        if let Some(SimplePathSegment::ClosePath) = self.prev_simple_seg {
            match segment {
                PathSegment::MoveTo { .. } | PathSegment::ClosePath { .. } => {}
                _ => {
                    let new_seg = SimplePathSegment::MoveTo { x: self.prev_mx, y: self.prev_my };
                    self.buffer.push(new_seg);
                    self.prev_simple_seg = Some(new_seg);
                }
            }
        }

        match segment {
            PathSegment::MoveTo { abs, mut x, mut y } => {
                if !abs {
                    // When we get 'm'(relative) segment, which is not first segment - then it's
                    // relative to a previous 'M'(absolute) segment, not to the first segment.
                    if let Some(SimplePathSegment::ClosePath) = self.prev_simple_seg {
                        x += self.prev_mx;
                        y += self.prev_my;
                    } else {
                        x += self.prev_x;
                        y += self.prev_y;
                    }
                }

                self.buffer.push(SimplePathSegment::MoveTo { x, y });
                self.prev_seg = segment;
            }
            PathSegment::LineTo { abs, mut x, mut y } => {
                if !abs {
                    x += self.prev_x;
                    y += self.prev_y;
                }

                self.buffer.push(SimplePathSegment::LineTo { x, y });
                self.prev_seg = segment;
            }
            PathSegment::HorizontalLineTo { abs, mut x } => {
                if !abs {
                    x += self.prev_x;
                }

                self.buffer
                    .push(SimplePathSegment::LineTo { x, y: self.prev_y });
                self.prev_seg = segment;
            }
            PathSegment::VerticalLineTo { abs, mut y } => {
                if !abs {
                    y += self.prev_y;
                }

                self.buffer
                    .push(SimplePathSegment::LineTo { x: self.prev_x, y });
                self.prev_seg = segment;
            }
            PathSegment::CurveTo {
                abs,
                mut x1,
                mut y1,
                mut x2,
                mut y2,
                mut x,
                mut y,
            } => {
                if !abs {
                    x1 += self.prev_x;
                    y1 += self.prev_y;
                    x2 += self.prev_x;
                    y2 += self.prev_y;
                    x += self.prev_x;
                    y += self.prev_y;
                }

                self.buffer.push(SimplePathSegment::CurveTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                });

                // Remember as absolute.
                self.prev_seg = PathSegment::CurveTo {
                    abs: true,
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                };
            }
            PathSegment::SmoothCurveTo {
                abs,
                mut x2,
                mut y2,
                mut x,
                mut y,
            } => {
                // 'The first control point is assumed to be the reflection of the second control
                // point on the previous command relative to the current point.
                // (If there is no previous command or if the previous command
                // was not an C, c, S or s, assume the first control point is
                // coincident with the current point.)'
                let (x1, y1) = match self.prev_seg {
                    PathSegment::CurveTo { x2, y2, x, y, .. }
                    | PathSegment::SmoothCurveTo { x2, y2, x, y, .. } => {
                        (x * 2.0 - x2, y * 2.0 - y2)
                    }
                    _ => (self.prev_x, self.prev_y),
                };

                if !abs {
                    x2 += self.prev_x;
                    y2 += self.prev_y;
                    x += self.prev_x;
                    y += self.prev_y;
                }

                self.buffer.push(SimplePathSegment::CurveTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                });

                // Remember as absolute.
                self.prev_seg = PathSegment::SmoothCurveTo {
                    abs: true,
                    x2,
                    y2,
                    x,
                    y,
                };
            }
            PathSegment::Quadratic {
                abs,
                mut x1,
                mut y1,
                mut x,
                mut y,
            } => {
                if !abs {
                    x1 += self.prev_x;
                    y1 += self.prev_y;
                    x += self.prev_x;
                    y += self.prev_y;
                }

                self.buffer
                    .push(SimplePathSegment::Quadratic { x1, y1, x, y });

                // Remember as absolute.
                self.prev_seg = PathSegment::Quadratic {
                    abs: true,
                    x1,
                    y1,
                    x,
                    y,
                };
            }
            PathSegment::SmoothQuadratic { abs, mut x, mut y } => {
                // 'The control point is assumed to be the reflection of
                // the control point on the previous command relative to
                // the current point. (If there is no previous command or
                // if the previous command was not a Q, q, T or t, assume
                // the control point is coincident with the current point.)'
                let (x1, y1) = match self.prev_seg {
                    PathSegment::Quadratic { x1, y1, x, y, .. } => (x * 2.0 - x1, y * 2.0 - y1),
                    PathSegment::SmoothQuadratic { x, y, .. } => {
                        (x * 2.0 - self.prev_tx, y * 2.0 - self.prev_ty)
                    }
                    _ => (self.prev_x, self.prev_y),
                };

                self.prev_tx = x1;
                self.prev_ty = y1;

                if !abs {
                    x += self.prev_x;
                    y += self.prev_y;
                }

                self.buffer
                    .push(SimplePathSegment::Quadratic { x1, y1, x, y });

                // Remember as absolute.
                self.prev_seg = PathSegment::SmoothQuadratic { abs: true, x, y };
            }
            PathSegment::EllipticalArc {
                abs,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                mut x,
                mut y,
            } => {
                if !abs {
                    x += self.prev_x;
                    y += self.prev_y;
                }

                let svg_arc = kurbo::SvgArc {
                    from: kurbo::Point::new(self.prev_x, self.prev_y),
                    to: kurbo::Point::new(x, y),
                    radii: kurbo::Vec2::new(rx, ry),
                    x_rotation: x_axis_rotation.to_radians(),
                    large_arc,
                    sweep,
                };

                match kurbo::Arc::from_svg_arc(&svg_arc) {
                    Some(arc) => {
                        arc.to_cubic_beziers(0.1, |p1, p2, p| {
                            self.buffer.push(SimplePathSegment::CurveTo {
                                x1: p1.x,
                                y1: p1.y,
                                x2: p2.x,
                                y2: p2.y,
                                x: p.x,
                                y: p.y,
                            });
                        });
                    }
                    None => {
                        self.buffer.push(SimplePathSegment::LineTo { x, y });
                    }
                }

                self.prev_seg = segment;
            }
            PathSegment::ClosePath { .. } => {
                if let Some(SimplePathSegment::ClosePath) = self.prev_simple_seg {
                    // Do not add sequential ClosePath segments.
                    // Otherwise it will break markers rendering.
                } else {
                    self.buffer.push(SimplePathSegment::ClosePath);
                }

                self.prev_seg = segment;
            }
        }

        // Remember last position.
        if let Some(new_segment) = self.buffer.last() {
            self.prev_simple_seg = Some(*new_segment);

            match *new_segment {
                SimplePathSegment::MoveTo { x, y } => {
                    self.prev_x = x;
                    self.prev_y = y;
                    self.prev_mx = self.prev_x;
                    self.prev_my = self.prev_y;
                }
                SimplePathSegment::LineTo { x, y } => {
                    self.prev_x = x;
                    self.prev_y = y;
                }
                SimplePathSegment::CurveTo { x, y, .. } => {
                    self.prev_x = x;
                    self.prev_y = y;
                }
                SimplePathSegment::Quadratic { x, y, .. } => {
                    self.prev_x = x;
                    self.prev_y = y;
                }
                SimplePathSegment::ClosePath => {
                    // ClosePath moves us to the last MoveTo coordinate.
                    self.prev_x = self.prev_mx;
                    self.prev_y = self.prev_my;
                }
            }
        }

        if self.buffer.is_empty() {
            return self.next();
        }

        Some(Ok(self.buffer.remove(0)))
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod simple_tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $text:expr, $( $seg:expr ),*) => (
            #[test]
            fn $name() {
                let mut s = SimplifyingPathParser::from($text);
                $(
                    assert_eq!(s.next().unwrap().unwrap(), $seg);
                )*

                if let Some(res) = s.next() {
                    assert!(res.is_err());
                }
            }
        )
    }

    test!(ignore_duplicated_close_paths, "M 10 20 L 30 40 Z Z Z Z",
        SimplePathSegment::MoveTo { x: 10.0, y: 20.0 },
        SimplePathSegment::LineTo { x: 30.0, y: 40.0 },
        SimplePathSegment::ClosePath
    );

    test!(relative_move_to, "m 30 40 110 120 -20 -130",
        SimplePathSegment::MoveTo { x: 30.0, y: 40.0 },
        SimplePathSegment::LineTo { x: 140.0, y: 160.0 },
        SimplePathSegment::LineTo { x: 120.0, y: 30.0 }
    );

    test!(smooth_curve_to_after_move_to, "M 30 40 S 171 45 180 155",
        SimplePathSegment::MoveTo { x: 30.0, y: 40.0 },
        SimplePathSegment::CurveTo { x1: 30.0, y1: 40.0, x2: 171.0, y2: 45.0, x: 180.0, y: 155.0 }
    );

    test!(smooth_curve_to_after_curve_to, "M 30 40 C 16 137 171 45 100 90 S 171 45 180 155",
        SimplePathSegment::MoveTo { x: 30.0, y: 40.0 },
        SimplePathSegment::CurveTo { x1: 16.0, y1: 137.0, x2: 171.0, y2: 45.0, x: 100.0, y: 90.0 },
        SimplePathSegment::CurveTo { x1: 29.0, y1: 135.0, x2: 171.0, y2: 45.0, x: 180.0, y: 155.0 }
    );

    test!(smooth_quadratic_after_move_to, "M 30 40 T 180 155",
        SimplePathSegment::MoveTo { x: 30.0, y: 40.0 },
        SimplePathSegment::Quadratic { x1: 30.0, y1: 40.0, x: 180.0, y: 155.0 }
    );

    test!(smooth_quadratic_after_quadratic, "M 30 40 Q 171 45 100 90 T 160 180",
        SimplePathSegment::MoveTo { x: 30.0, y: 40.0 },
        SimplePathSegment::Quadratic { x1: 171.0, y1: 45.0, x: 100.0, y: 90.0 },
        SimplePathSegment::Quadratic { x1: 29.0, y1: 135.0, x: 160.0, y: 180.0 }
    );

    test!(relative_smooth_quadratic_after_quadratic, "M 30 40 Q 171 45 100 90 t 60 80",
        SimplePathSegment::MoveTo { x: 30.0, y: 40.0 },
        SimplePathSegment::Quadratic { x1: 171.0, y1: 45.0, x: 100.0, y: 90.0 },
        SimplePathSegment::Quadratic { x1: 29.0, y1: 135.0, x: 160.0, y: 170.0 }
    );

    test!(relative_smooth_quadratic_after_relative_quadratic, "M 30 40 q 171 45 50 40 t 60 80",
        SimplePathSegment::MoveTo { x: 30.0, y: 40.0 },
        SimplePathSegment::Quadratic { x1: 201.0, y1: 85.0, x: 80.0, y: 80.0 },
        SimplePathSegment::Quadratic { x1: -41.0, y1: 75.0, x: 140.0, y: 160.0 }
    );

    test!(smooth_quadratic_after_smooth_quadratic, "M 30 30 T 40 140 T 170 30",
        SimplePathSegment::MoveTo { x: 30.0, y: 30.0 },
        SimplePathSegment::Quadratic { x1: 30.0, y1: 30.0, x: 40.0, y: 140.0 },
        SimplePathSegment::Quadratic { x1: 50.0, y1: 250.0, x: 170.0, y: 30.0 }
    );

    test!(smooth_quadratic_after_relative_smooth_quadratic, "M 30 30 T 40 140 t 100 -30",
        SimplePathSegment::MoveTo { x: 30.0, y: 30.0 },
        SimplePathSegment::Quadratic { x1: 30.0, y1: 30.0, x: 40.0, y: 140.0 },
        SimplePathSegment::Quadratic { x1: 50.0, y1: 250.0, x: 140.0, y: 110.0 }
    );

    test!(smooth_quadratic_after_relative_quadratic, "M 30 30 T 40 140 q 30 100 120 -30",
        SimplePathSegment::MoveTo { x: 30.0, y: 30.0 },
        SimplePathSegment::Quadratic { x1: 30.0, y1: 30.0, x: 40.0, y: 140.0 },
        SimplePathSegment::Quadratic { x1: 70.0, y1: 240.0, x: 160.0, y: 110.0 }
    );

    test!(smooth_quadratic_after_relative_smooth_curve_to, "M 30 30 T 40 170 s 90 -20 90 -90",
        SimplePathSegment::MoveTo { x: 30.0, y: 30.0 },
        SimplePathSegment::Quadratic { x1: 30.0, y1: 30.0, x: 40.0, y: 170.0 },
        SimplePathSegment::CurveTo { x1: 40.0, y1: 170.0, x2: 130.0, y2: 150.0, x: 130.0, y: 80.0 }
    );

    test!(quadratic_after_smooth_quadratic, "M 30 30 T 40 140 Q 80 180 170 30",
        SimplePathSegment::MoveTo { x: 30.0, y: 30.0 },
        SimplePathSegment::Quadratic { x1: 30.0, y1: 30.0, x: 40.0, y: 140.0 },
        SimplePathSegment::Quadratic { x1: 80.0, y1: 180.0, x: 170.0, y: 30.0 }
    );

    test!(implicit_move_to_after_close_path, "M 10 20 L 30 40 Z L 50 60",
        SimplePathSegment::MoveTo { x: 10.0, y: 20.0 },
        SimplePathSegment::LineTo { x: 30.0, y: 40.0 },
        SimplePathSegment::ClosePath,
        SimplePathSegment::MoveTo { x: 10.0, y: 20.0 },
        SimplePathSegment::LineTo { x: 50.0, y: 60.0 }
    );

    #[test]
    fn arc_to() {
        let mut s = SimplifyingPathParser::from("M 30 40 A 40 30 20 1 1 150 100");
        assert_eq!(s.next().unwrap().unwrap(), SimplePathSegment::MoveTo { x: 30.0, y: 40.0 });
        let curve1 = s.next().unwrap().unwrap();
        let curve2 = s.next().unwrap().unwrap();
        if let Some(res) = s.next() {
            assert!(res.is_err());
        }

        if let SimplePathSegment::CurveTo { x1, y1, x2, y2, x, y } = curve1 {
            assert_eq!(x1.round(), 45.0);
            assert_eq!(y1.round(), 16.0);
            assert_eq!(x2.round(), 84.0);
            assert_eq!(y2.round(), 10.0);
            assert_eq!(x.round(), 117.0);
            assert_eq!(y.round(), 27.0);
        } else {
            panic!("invalid type");
        }

        if let SimplePathSegment::CurveTo { x1, y1, x2, y2, x, y } = curve2 {
            assert_eq!(x1.round(), 150.0);
            assert_eq!(y1.round(), 43.0);
            assert_eq!(x2.round(), 165.0);
            assert_eq!(y2.round(), 76.0);
            assert_eq!(x.round(), 150.0);
            assert_eq!(y.round(), 100.0);
        } else {
            panic!("invalid type");
        }
    }
}
