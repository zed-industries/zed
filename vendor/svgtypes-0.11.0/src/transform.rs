use std::f64;

use crate::{Error, Stream};

/// Representation of the [`<transform>`] type.
///
/// [`<transform>`]: https://www.w3.org/TR/SVG2/coords.html#InterfaceSVGTransform
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Transform {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Transform {
    /// Constructs a new transform.
    #[inline]
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Transform { a, b, c, d, e, f }
    }
}

impl Default for Transform {
    #[inline]
    fn default() -> Transform {
        Transform::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    }
}

/// Transform list token.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum TransformListToken {
    Matrix {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    },
    Translate {
        tx: f64,
        ty: f64,
    },
    Scale {
        sx: f64,
        sy: f64,
    },
    Rotate {
        angle: f64,
    },
    SkewX {
        angle: f64,
    },
    SkewY {
        angle: f64,
    },
}

/// A pull-based [`<transform-list>`] parser.
///
/// # Errors
///
/// - Most of the `Error` types can occur.
///
/// # Notes
///
/// - There are no separate `rotate(<rotate-angle> <cx> <cy>)` type.
///   It will be automatically split into three `Transform` tokens:
///   `translate(<cx> <cy>) rotate(<rotate-angle>) translate(-<cx> -<cy>)`.
///   Just like the spec is stated.
///
/// # Examples
///
/// ```
/// use svgtypes::{TransformListParser, TransformListToken};
///
/// let mut p = TransformListParser::from("scale(2) translate(10, -20)");
/// assert_eq!(p.next().unwrap().unwrap(), TransformListToken::Scale { sx: 2.0, sy: 2.0 } );
/// assert_eq!(p.next().unwrap().unwrap(), TransformListToken::Translate { tx: 10.0, ty: -20.0 } );
/// assert_eq!(p.next().is_none(), true);
/// ```
///
/// [`<transform-list>`]: https://www.w3.org/TR/SVG11/shapes.html#PointsBNF
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TransformListParser<'a> {
    stream: Stream<'a>,
    rotate_ts: Option<(f64, f64)>,
    last_angle: Option<f64>,
}

impl<'a> From<&'a str> for TransformListParser<'a> {
    fn from(text: &'a str) -> Self {
        TransformListParser {
            stream: Stream::from(text),
            rotate_ts: None,
            last_angle: None,
        }
    }
}

impl<'a> Iterator for TransformListParser<'a> {
    type Item = Result<TransformListToken, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(a) = self.last_angle {
            self.last_angle = None;
            return Some(Ok(TransformListToken::Rotate { angle: a }));
        }

        if let Some((x, y)) = self.rotate_ts {
            self.rotate_ts = None;
            return Some(Ok(TransformListToken::Translate { tx: -x, ty: -y }));
        }

        self.stream.skip_spaces();

        if self.stream.at_end() {
            // empty attribute is still a valid value
            return None;
        }

        let res = self.parse_next();
        if res.is_err() {
            self.stream.jump_to_end();
        }

        Some(res)
    }
}

impl<'a> TransformListParser<'a> {
    fn parse_next(&mut self) -> Result<TransformListToken, Error> {
        let s = &mut self.stream;

        let start = s.pos();
        let name = s.consume_ident();
        s.skip_spaces();
        s.consume_byte(b'(')?;

        let t = match name.as_bytes() {
            b"matrix" => TransformListToken::Matrix {
                a: s.parse_list_number()?,
                b: s.parse_list_number()?,
                c: s.parse_list_number()?,
                d: s.parse_list_number()?,
                e: s.parse_list_number()?,
                f: s.parse_list_number()?,
            },
            b"translate" => {
                let x = s.parse_list_number()?;
                s.skip_spaces();

                let y = if s.is_curr_byte_eq(b')') {
                    // 'If <ty> is not provided, it is assumed to be zero.'
                    0.0
                } else {
                    s.parse_list_number()?
                };

                TransformListToken::Translate { tx: x, ty: y }
            }
            b"scale" => {
                let x = s.parse_list_number()?;
                s.skip_spaces();

                let y = if s.is_curr_byte_eq(b')') {
                    // 'If <sy> is not provided, it is assumed to be equal to <sx>.'
                    x
                } else {
                    s.parse_list_number()?
                };

                TransformListToken::Scale { sx: x, sy: y }
            }
            b"rotate" => {
                let a = s.parse_list_number()?;
                s.skip_spaces();

                if !s.is_curr_byte_eq(b')') {
                    // 'If optional parameters <cx> and <cy> are supplied, the rotate is about the
                    // point (cx, cy). The operation represents the equivalent of the following
                    // specification:
                    // translate(<cx>, <cy>) rotate(<rotate-angle>) translate(-<cx>, -<cy>).'
                    let cx = s.parse_list_number()?;
                    let cy = s.parse_list_number()?;
                    self.rotate_ts = Some((cx, cy));
                    self.last_angle = Some(a);

                    TransformListToken::Translate { tx: cx, ty: cy }
                } else {
                    TransformListToken::Rotate { angle: a }
                }
            }
            b"skewX" => TransformListToken::SkewX {
                angle: s.parse_list_number()?,
            },
            b"skewY" => TransformListToken::SkewY {
                angle: s.parse_list_number()?,
            },
            _ => {
                return Err(Error::UnexpectedData(s.calc_char_pos_at(start)));
            }
        };

        s.skip_spaces();
        s.consume_byte(b')')?;
        s.skip_spaces();

        if s.is_curr_byte_eq(b',') {
            s.advance(1);
        }

        Ok(t)
    }
}

impl std::str::FromStr for Transform {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Error> {
        let tokens = TransformListParser::from(text);
        let mut ts = Transform::default();

        for token in tokens {
            match token? {
                TransformListToken::Matrix { a, b, c, d, e, f } => {
                    ts = multiply(&ts, &Transform::new(a, b, c, d, e, f))
                }
                TransformListToken::Translate { tx, ty } => {
                    ts = multiply(&ts, &Transform::new(1.0, 0.0, 0.0, 1.0, tx, ty))
                }
                TransformListToken::Scale { sx, sy } => {
                    ts = multiply(&ts, &Transform::new(sx, 0.0, 0.0, sy, 0.0, 0.0))
                }
                TransformListToken::Rotate { angle } => {
                    let v = angle.to_radians();
                    let a = v.cos();
                    let b = v.sin();
                    let c = -b;
                    let d = a;
                    ts = multiply(&ts, &Transform::new(a, b, c, d, 0.0, 0.0))
                }
                TransformListToken::SkewX { angle } => {
                    let c = angle.to_radians().tan();
                    ts = multiply(&ts, &Transform::new(1.0, 0.0, c, 1.0, 0.0, 0.0))
                }
                TransformListToken::SkewY { angle } => {
                    let b = angle.to_radians().tan();
                    ts = multiply(&ts, &Transform::new(1.0, b, 0.0, 1.0, 0.0, 0.0))
                }
            }
        }

        Ok(ts)
    }
}

#[inline(never)]
fn multiply(ts1: &Transform, ts2: &Transform) -> Transform {
    Transform {
        a: ts1.a * ts2.a + ts1.c * ts2.b,
        b: ts1.b * ts2.a + ts1.d * ts2.b,
        c: ts1.a * ts2.c + ts1.c * ts2.d,
        d: ts1.b * ts2.c + ts1.d * ts2.d,
        e: ts1.a * ts2.e + ts1.c * ts2.f + ts1.e,
        f: ts1.b * ts2.e + ts1.d * ts2.f + ts1.f,
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    macro_rules! test {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                let ts = Transform::from_str($text).unwrap();
                let s = format!("matrix({} {} {} {} {} {})", ts.a, ts.b, ts.c, ts.d, ts.e, ts.f);
                assert_eq!(s, $result);
            }
        )
    }

    test!(parse_1,
        "matrix(1 0 0 1 10 20)",
        "matrix(1 0 0 1 10 20)"
    );

    test!(parse_2,
        "translate(10 20)",
        "matrix(1 0 0 1 10 20)"
    );

    test!(parse_3,
        "scale(2 3)",
        "matrix(2 0 0 3 0 0)"
    );

    test!(parse_4,
        "rotate(30)",
        "matrix(0.8660254037844387 0.49999999999999994 -0.49999999999999994 0.8660254037844387 0 0)"
    );

    test!(parse_5,
        "rotate(30 10 20)",
        "matrix(0.8660254037844387 0.49999999999999994 -0.49999999999999994 0.8660254037844387 11.339745962155611 -2.3205080756887746)"
    );

    test!(parse_6,
        "translate(10 15) translate(0 5)",
        "matrix(1 0 0 1 10 20)"
    );

    test!(parse_7,
        "translate(10) scale(2)",
        "matrix(2 0 0 2 10 0)"
    );

    test!(parse_8,
        "translate(25 215) scale(2) skewX(45)",
        "matrix(2 0 1.9999999999999998 2 25 215)"
    );

    test!(parse_9,
        "skewX(45)",
        "matrix(1 0 0.9999999999999999 1 0 0)"
    );

    macro_rules! test_err {
        ($name:ident, $text:expr, $result:expr) => (
            #[test]
            fn $name() {
                let ts = Transform::from_str($text);
                assert_eq!(ts.unwrap_err().to_string(), $result);
            }
        )
    }

    test_err!(parse_err_1, "text", "unexpected end of stream");

    #[test]
    fn parse_err_2() {
        let mut ts = TransformListParser::from("scale(2) text");
        let _ = ts.next().unwrap();
        assert_eq!(ts.next().unwrap().unwrap_err().to_string(),
                   "unexpected end of stream");
    }

    test_err!(parse_err_3, "???G", "expected '(' not '?' at position 1");

    #[test]
    fn parse_err_4() {
        let mut ts = TransformListParser::from(" ");
        assert_eq!(ts.next().is_none(), true);
    }

    #[test]
    fn parse_err_5() {
        let mut ts = TransformListParser::from("\x01");
        assert_eq!(ts.next().unwrap().is_err(), true);
    }

    test_err!(parse_err_6, "rect()", "unexpected data at position 1");

    test_err!(parse_err_7, "scale(2) rect()", "unexpected data at position 10");
}
