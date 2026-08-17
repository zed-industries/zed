use path::{
    geom::{ArcFlags, SvgArc},
    math::{point, vector, Angle, Point},
    traits::PathBuilder,
};

extern crate thiserror;

use self::thiserror::Error;

#[non_exhaustive]
#[derive(Error, Clone, Debug, PartialEq)]
pub enum ParseError {
    #[error("Line {line} Column {column}: Expected number, got {src:?}.")]
    Number { src: String, line: i32, column: i32 },
    #[error("Line {line} Column {column}: Expected flag (0/1), got {src:?}.")]
    Flag { src: char, line: i32, column: i32 },
    #[error("Line {line} Column {column}: Invalid command {command:?}.")]
    Command {
        command: char,
        line: i32,
        column: i32,
    },
    #[error("Line {line} Column {column}: Expected move-to command, got {command:?}.")]
    MissingMoveTo {
        command: char,
        line: i32,
        column: i32,
    },
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct ParserOptions {
    /// Number of custom attributes per endpoint.
    pub num_attributes: usize,
    /// Optionally stop parsing when encountering a provided special character.
    pub stop_at: Option<char>,
}

impl ParserOptions {
    pub const DEFAULT: ParserOptions = ParserOptions {
        num_attributes: 0,
        stop_at: None,
    };
}

// A buffered iterator of characters keeping track of line and column.
pub struct Source<Iter> {
    src: Iter,
    current: char,
    line: i32,
    col: i32,
    finished: bool,
}

impl<Iter: Iterator<Item = char>> Source<Iter> {
    pub fn new<IntoIter>(src: IntoIter) -> Self
    where
        IntoIter: IntoIterator<IntoIter = Iter>,
    {
        Self::with_position(0, 0, src)
    }

    pub fn with_position<IntoIter>(line: i32, column: i32, src: IntoIter) -> Self
    where
        IntoIter: IntoIterator<IntoIter = Iter>,
    {
        let mut src = src.into_iter();

        let (current, finished) = match src.next() {
            Some(c) => (c, false),
            None => (' ', true),
        };

        let line = line + if current == '\n' { 1 } else { 0 };

        Source {
            current,
            finished,
            src,
            line,
            col: column,
        }
    }

    /// Consume the source and returns the iterator, line and column.
    pub fn unwrap(self) -> (Iter, i32, i32) {
        (self.src, self.line, self.col)
    }

    fn skip_whitespace(&mut self) {
        while !self.finished && (self.current.is_whitespace() || self.current == ',') {
            self.advance_one();
        }
    }

    fn advance_one(&mut self) {
        if self.finished {
            return;
        }
        match self.src.next() {
            Some('\n') => {
                self.current = '\n';
                self.line += 1;
                self.col = -1;
            }
            Some(c) => {
                self.current = c;
                self.col += 1;
            }
            None => {
                self.current = '~';
                self.finished = true;
            }
        }
    }
}

/// A context object for parsing the extended path syntax.
///
/// # Syntax
///
/// The extended path syntax is a super-set of the SVG path syntax, with support for extra per-endpoint
/// data for custom attributes.
///
/// Wherever an endpoint is specified, the extended path syntax expects a sequence N extra numbers
/// where N is the number of custom attributes.
///
/// For example with 1 custom attribute `M 0 0 10 Q 1 1 2 2 20 L 3 3 30` reads as follows:
///
/// - Begin at endpoint position [0, 0] with custom attribute 10,
/// - quadratic bézier curve ending at endpoint [2, 2] custom attribute 20, with control point [1, 1],
/// - line to endpoint [3, 3] custom attribute 30.
///
/// Note that only endpoints have custom attributes, control points do not.
#[derive(Debug, Default)]
pub struct PathParser {
    attribute_buffer: Vec<f32>,
    float_buffer: String,
    num_attributes: usize,
    stop_at: Option<char>,
    current_position: Point,
    need_end: bool,
}

impl PathParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse<Iter, Builder>(
        &mut self,
        options: &ParserOptions,
        src: &mut Source<Iter>,
        output: &mut Builder,
    ) -> Result<(), ParseError>
    where
        Iter: Iterator<Item = char>,
        Builder: PathBuilder,
    {
        self.num_attributes = options.num_attributes;
        self.stop_at = options.stop_at;
        self.need_end = false;

        let res = self.parse_path(src, output);

        if self.need_end {
            output.end(false);
        }

        res
    }

    fn parse_path(
        &mut self,
        src: &mut Source<impl Iterator<Item = char>>,
        output: &mut impl PathBuilder,
    ) -> Result<(), ParseError> {
        // Per-spec: "If a relative moveto (m) appears as the first element of the path, then it is
        // treated as a pair of absolute coordinates."
        self.current_position = point(0.0, 0.0);
        let mut first_position = point(0.0, 0.0);

        let mut need_start = false;
        let mut prev_cubic_ctrl = None;
        let mut prev_quadratic_ctrl = None;
        let mut implicit_cmd = 'M';

        src.skip_whitespace();

        while !src.finished {
            let mut cmd = src.current;
            let cmd_line = src.line;
            let cmd_col = src.col;

            if self.stop_at == Some(cmd) {
                break;
            }

            if cmd.is_ascii_alphabetic() {
                src.advance_one();
            } else {
                cmd = implicit_cmd;
            }

            if need_start && cmd != 'm' && cmd != 'M' {
                return Err(ParseError::MissingMoveTo {
                    command: cmd,
                    line: cmd_line,
                    column: cmd_col,
                });
            }

            //println!("{:?} at line {:?} column {:?}", cmd, cmd_line, cmd_col);

            let is_relative = cmd.is_lowercase();

            match cmd {
                'l' | 'L' => {
                    let to = self.parse_endpoint(is_relative, src)?;
                    output.line_to(to, &self.attribute_buffer);
                }
                'h' | 'H' => {
                    let mut x = self.parse_number(src)?;
                    if is_relative {
                        x += self.current_position.x;
                    }
                    let to = point(x, self.current_position.y);
                    self.current_position = to;
                    self.parse_attributes(src)?;
                    output.line_to(to, &self.attribute_buffer);
                }
                'v' | 'V' => {
                    let mut y = self.parse_number(src)?;
                    if is_relative {
                        y += self.current_position.y;
                    }
                    let to = point(self.current_position.x, y);
                    self.current_position = to;
                    self.parse_attributes(src)?;
                    output.line_to(to, &self.attribute_buffer);
                }
                'q' | 'Q' => {
                    let ctrl = self.parse_point(is_relative, src)?;
                    let to = self.parse_endpoint(is_relative, src)?;
                    prev_quadratic_ctrl = Some(ctrl);
                    output.quadratic_bezier_to(ctrl, to, &self.attribute_buffer);
                }
                't' | 'T' => {
                    let ctrl = self.get_smooth_ctrl(prev_quadratic_ctrl);
                    let to = self.parse_endpoint(is_relative, src)?;
                    prev_quadratic_ctrl = Some(ctrl);
                    output.quadratic_bezier_to(ctrl, to, &self.attribute_buffer);
                }
                'c' | 'C' => {
                    let ctrl1 = self.parse_point(is_relative, src)?;
                    let ctrl2 = self.parse_point(is_relative, src)?;
                    let to = self.parse_endpoint(is_relative, src)?;
                    prev_cubic_ctrl = Some(ctrl2);
                    output.cubic_bezier_to(ctrl1, ctrl2, to, &self.attribute_buffer);
                }
                's' | 'S' => {
                    let ctrl1 = self.get_smooth_ctrl(prev_cubic_ctrl);
                    let ctrl2 = self.parse_point(is_relative, src)?;
                    let to = self.parse_endpoint(is_relative, src)?;
                    prev_cubic_ctrl = Some(ctrl2);
                    output.cubic_bezier_to(ctrl1, ctrl2, to, &self.attribute_buffer);
                }
                'a' | 'A' => {
                    let prev_attributes = self.attribute_buffer.clone();
                    let mut interpolated_attributes = self.attribute_buffer.clone();

                    let from = self.current_position;
                    let rx = self.parse_number(src)?;
                    let ry = self.parse_number(src)?;
                    let x_rotation = self.parse_number(src)?;
                    let large_arc = self.parse_flag(src)?;
                    let sweep = self.parse_flag(src)?;
                    let to = self.parse_endpoint(is_relative, src)?;
                    let svg_arc = SvgArc {
                        from,
                        to,
                        radii: vector(rx, ry),
                        x_rotation: Angle::degrees(x_rotation),
                        flags: ArcFlags { large_arc, sweep },
                    };

                    if svg_arc.is_straight_line() {
                        output.line_to(to, &self.attribute_buffer[..]);
                    } else {
                        let arc = svg_arc.to_arc();

                        arc.for_each_quadratic_bezier_with_t(&mut |curve, range| {
                            for i in 0..self.num_attributes {
                                interpolated_attributes[i] = prev_attributes[i] * (1.0 - range.end)
                                    + self.attribute_buffer[i] * range.end;
                            }
                            output.quadratic_bezier_to(
                                curve.ctrl,
                                curve.to,
                                &interpolated_attributes,
                            );
                        });
                    }
                }
                'm' | 'M' => {
                    if self.need_end {
                        output.end(false);
                        self.need_end = false;
                    }

                    let to = self.parse_endpoint(is_relative, src)?;
                    first_position = to;
                    output.begin(to, &self.attribute_buffer);
                    self.need_end = true;
                    need_start = false;
                }
                'z' | 'Z' => {
                    output.end(true);
                    self.current_position = first_position;
                    self.need_end = false;
                    need_start = true;
                }
                _ => {
                    return Err(ParseError::Command {
                        command: cmd,
                        line: cmd_line,
                        column: cmd_col,
                    });
                }
            }

            match cmd {
                'c' | 'C' | 's' | 'S' => {
                    prev_quadratic_ctrl = None;
                }
                'q' | 'Q' | 't' | 'T' => {
                    prev_cubic_ctrl = None;
                }
                _ => {
                    prev_cubic_ctrl = None;
                    prev_quadratic_ctrl = None;
                }
            }

            implicit_cmd = match cmd {
                'm' => 'l',
                'M' => 'L',
                'z' => 'm',
                'Z' => 'M',
                c => c,
            };

            src.skip_whitespace();
        }

        Ok(())
    }

    fn get_smooth_ctrl(&self, prev_ctrl: Option<Point>) -> Point {
        if let Some(prev_ctrl) = prev_ctrl {
            self.current_position + (self.current_position - prev_ctrl)
        } else {
            self.current_position
        }
    }

    fn parse_endpoint(
        &mut self,
        is_relative: bool,
        src: &mut Source<impl Iterator<Item = char>>,
    ) -> Result<Point, ParseError> {
        let position = self.parse_point(is_relative, src)?;
        self.current_position = position;

        self.parse_attributes(src)?;

        Ok(position)
    }

    fn parse_attributes(
        &mut self,
        src: &mut Source<impl Iterator<Item = char>>,
    ) -> Result<(), ParseError> {
        self.attribute_buffer.clear();
        for _ in 0..self.num_attributes {
            let value = self.parse_number(src)?;
            self.attribute_buffer.push(value);
        }

        Ok(())
    }

    fn parse_point(
        &mut self,
        is_relative: bool,
        src: &mut Source<impl Iterator<Item = char>>,
    ) -> Result<Point, ParseError> {
        let mut x = self.parse_number(src)?;
        let mut y = self.parse_number(src)?;

        if is_relative {
            x += self.current_position.x;
            y += self.current_position.y;
        }

        Ok(point(x, y))
    }

    fn parse_number(
        &mut self,
        src: &mut Source<impl Iterator<Item = char>>,
    ) -> Result<f32, ParseError> {
        self.float_buffer.clear();

        src.skip_whitespace();

        let line = src.line;
        let column = src.col;

        if src.current == '-' {
            self.float_buffer.push('-');
            src.advance_one();
        }

        while src.current.is_numeric() {
            self.float_buffer.push(src.current);
            src.advance_one();
        }

        if src.current == '.' {
            self.float_buffer.push('.');
            src.advance_one();

            while src.current.is_numeric() {
                self.float_buffer.push(src.current);
                src.advance_one();
            }
        }

        if src.current == 'e' || src.current == 'E' {
            self.float_buffer.push(src.current);
            src.advance_one();

            if src.current == '-' {
                self.float_buffer.push('-');
                src.advance_one();
            }

            while src.current.is_numeric() {
                self.float_buffer.push(src.current);
                src.advance_one();
            }
        }

        match self.float_buffer.parse::<f32>() {
            Ok(val) => Ok(val),
            Err(_) => Err(ParseError::Number {
                src: std::mem::take(&mut self.float_buffer),
                line,
                column,
            }),
        }
    }

    fn parse_flag(
        &mut self,
        src: &mut Source<impl Iterator<Item = char>>,
    ) -> Result<bool, ParseError> {
        src.skip_whitespace();
        match src.current {
            '1' => {
                src.advance_one();
                Ok(true)
            }
            '0' => {
                src.advance_one();
                Ok(false)
            }
            _ => Err(ParseError::Flag {
                src: src.current,
                line: src.line,
                column: src.col,
            }),
        }
    }
}

#[cfg(test)]
use crate::path::Path;

#[test]
fn empty() {
    let options = ParserOptions {
        num_attributes: 0,
        ..ParserOptions::DEFAULT
    };

    let mut parser = PathParser::new();

    let mut builder = Path::builder_with_attributes(options.num_attributes);
    parser
        .parse(&options, &mut Source::new("".chars()), &mut builder)
        .unwrap();

    let mut builder = Path::builder_with_attributes(options.num_attributes);
    parser
        .parse(&options, &mut Source::new(" ".chars()), &mut builder)
        .unwrap();
}

#[test]
fn simple_square() {
    let options = ParserOptions {
        num_attributes: 0,
        ..ParserOptions::DEFAULT
    };
    let mut parser = PathParser::new();
    let mut builder = Path::builder_with_attributes(options.num_attributes);
    let mut src = Source::new("M 0 0 L 1 0 L 1 1 L 0 1 Z".chars());

    parser.parse(&options, &mut src, &mut builder).unwrap();
}

#[test]
fn simple_attr() {
    let options = ParserOptions {
        num_attributes: 1,
        ..ParserOptions::DEFAULT
    };
    let mut parser = PathParser::new();
    let mut builder = Path::builder_with_attributes(options.num_attributes);
    let mut src = Source::new("M 0 0 1.0 L 1 0 2.0 L 1 1 3.0 L 0 1 4.0 Z".chars());

    parser.parse(&options, &mut src, &mut builder).unwrap();
}

#[test]
fn implicit_polyline() {
    let options = ParserOptions {
        num_attributes: 1,
        ..ParserOptions::DEFAULT
    };
    let mut parser = PathParser::new();
    let mut builder = Path::builder_with_attributes(options.num_attributes);
    let mut src = Source::new("0 0 0 1 1 1.0 2 2 2.0 3 3 3".chars());

    parser.parse(&options, &mut src, &mut builder).unwrap();
}

#[test]
fn invalid_cmd() {
    let options = ParserOptions {
        num_attributes: 1,
        ..ParserOptions::DEFAULT
    };
    let mut parser = PathParser::new();
    let mut src = Source::new("x 0 0 0".chars());

    let mut builder = Path::builder_with_attributes(options.num_attributes);
    let result = parser
        .parse(&options, &mut src, &mut builder)
        .err()
        .unwrap();
    assert_eq!(
        result,
        ParseError::Command {
            command: 'x',
            line: 0,
            column: 0
        }
    );

    let mut builder = Path::builder_with_attributes(options.num_attributes);
    let mut src = Source::new("\n M 0 \n0 1 x 1 1 1".chars());

    let result = parser
        .parse(&options, &mut src, &mut builder)
        .err()
        .unwrap();
    assert_eq!(
        result,
        ParseError::Command {
            command: 'x',
            line: 2,
            column: 4
        }
    );
}

#[test]
fn number_01() {
    let options = ParserOptions {
        num_attributes: 0,
        ..ParserOptions::DEFAULT
    };
    let mut parser = PathParser::new();

    // Per SVG spec, this is equivalent to "M 0.6 0.5".
    let mut src = Source::new("M 0.6.5".chars());
    let mut builder = Path::builder_with_attributes(options.num_attributes);

    parser.parse(&options, &mut src, &mut builder).unwrap();
    let path = builder.build();

    let mut iter = path.iter();
    use path::PathEvent;
    assert_eq!(
        iter.next(),
        Some(PathEvent::Begin {
            at: point(0.6, 0.5)
        })
    );
    assert_eq!(
        iter.next(),
        Some(PathEvent::End {
            last: point(0.6, 0.5),
            first: point(0.6, 0.5),
            close: false
        })
    );
    assert_eq!(iter.next(), None);
}

#[test]
fn number_scientific_notation() {
    let options = ParserOptions {
        num_attributes: 0,
        ..ParserOptions::DEFAULT
    };
    let mut parser = PathParser::new();
    let mut builder = Path::builder_with_attributes(options.num_attributes);
    let mut src = Source::new("M 1e-2 -1E3".chars());

    parser.parse(&options, &mut src, &mut builder).unwrap();
}

#[test]
fn bad_numbers() {
    let options = ParserOptions {
        num_attributes: 0,
        ..ParserOptions::DEFAULT
    };
    let mut parser = PathParser::new();

    let bad_number = &mut|src: &str| {
        let r = parser.parse(
            &options,
            &mut Source::new(src.chars()),
            &mut Path::builder_with_attributes(0)
        );
        match r {
            Err(ParseError::Number { .. }) => true,
            _ => {
                println!("{r:?}");
                false
            }
        }
    };

    assert!(bad_number("M 0 --1"));
    assert!(bad_number("M 0 1ee2"));
    assert!(bad_number("M 0 1e--1"));
    assert!(bad_number("M 0 *2"));
    assert!(bad_number("M 0 e"));
    assert!(bad_number("M 0 1e"));
    assert!(bad_number("M 0 +1"));
}

#[test]
fn stop() {
    let options = ParserOptions {
        num_attributes: 0,
        stop_at: Some('|'),
        ..ParserOptions::DEFAULT
    };
    let mut parser = PathParser::new();

    let parse = &mut |src: &str| {
        parser.parse(
            &options,
            &mut Source::new(src.chars()),
            &mut Path::builder_with_attributes(0),
        )
    };

    parse("M 0 0 | xxxxxx").unwrap();
    parse("M 0 0| xxxxxx").unwrap();
    parse("| xxxxxx").unwrap();
    parse("    | xxxxxx").unwrap();
}

#[test]
fn need_start() {
    let options = ParserOptions {
        num_attributes: 0,
        stop_at: Some('|'),
        ..ParserOptions::DEFAULT
    };
    let mut parser = PathParser::new();

    let mut builder = Path::builder_with_attributes(options.num_attributes);
    let res = parser.parse(
        &options,
        &mut Source::new("M 0 0 Z L 1 1 2 2 L 3 3 Z M 4 4".chars()),
        &mut builder,
    );
    let p1 = builder.build();
    match res {
        Err(ParseError::MissingMoveTo { .. }) => {}
        _ => {
            panic!("{:?}", res);
        }
    }

    let mut builder = Path::builder_with_attributes(options.num_attributes);
    parser
        .parse(&options, &mut Source::new("M 0 0 Z".chars()), &mut builder)
        .unwrap();
    let p2 = builder.build();

    let mut p1 = p1.iter();
    let mut p2 = p2.iter();
    loop {
        let e1 = p1.next();
        let e2 = p2.next();

        assert_eq!(e1, e2);

        if e1.is_none() {
            break;
        }
    }
}

#[test]
fn issue_895() {
    let options = ParserOptions::DEFAULT;
    let mut parser = PathParser::new();

    let parse = &mut |src: &str| {
        parser.parse(
            &options,
            &mut Source::new(src.chars()),
            &mut Path::builder_with_attributes(0),
        )
    };

    parse("M 1e-9 0").unwrap();
    parse("M -1e-9 0").unwrap();
    parse("M -1e11 0").unwrap();
    parse("M 1.e-9 1.4e-4z").unwrap();
    parse("M 1.6e-9 1.4e-4 z").unwrap();
    parse("M0 1.6e-9L0 1.4e-4").unwrap();
}

#[test]
fn issue_919() {
    let mut builder = lyon_path::Path::builder();
    let mut parser = PathParser::new();
    let _ = parser.parse(
        &ParserOptions::DEFAULT,
        &mut Source::new("0 0M".chars()),
        &mut builder,
    );
}
