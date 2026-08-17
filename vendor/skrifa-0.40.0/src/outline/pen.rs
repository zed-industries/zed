//! Types for collecting the output when drawing a glyph outline.

use alloc::{string::String, vec::Vec};
use core::fmt::{self, Write};

use crate::metrics::BoundingBox;

/// Interface for accepting a sequence of path commands.
pub trait OutlinePen {
    /// Emit a command to begin a new subpath at (x, y).
    fn move_to(&mut self, x: f32, y: f32);

    /// Emit a line segment from the current point to (x, y).
    fn line_to(&mut self, x: f32, y: f32);

    /// Emit a quadratic bezier segment from the current point with a control
    /// point at (cx0, cy0) and ending at (x, y).
    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32);

    /// Emit a cubic bezier segment from the current point with control
    /// points at (cx0, cy0) and (cx1, cy1) and ending at (x, y).
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32);

    /// Emit a command to close the current subpath.
    fn close(&mut self);
}

/// Single element of a path.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum PathElement {
    /// Begin a new subpath at (x, y).
    MoveTo { x: f32, y: f32 },
    /// Draw a line from the current point to (x, y).
    LineTo { x: f32, y: f32 },
    /// Draw a quadratic bezier from the current point with a control point at
    /// (cx0, cy0) and ending at (x, y).
    QuadTo { cx0: f32, cy0: f32, x: f32, y: f32 },
    /// Draw a cubic bezier from the current point with control points at
    /// (cx0, cy0) and (cx1, cy1) and ending at (x, y).
    CurveTo {
        cx0: f32,
        cy0: f32,
        cx1: f32,
        cy1: f32,
        x: f32,
        y: f32,
    },
    /// Close the current subpath.
    Close,
}

/// Style for path conversion.
///
/// The order to process points in a glyf point stream is ambiguous when the
/// first point is off-curve. Major implementations differ. Which one would
/// you like to match?
///
/// **If you add a new one make sure to update the fuzzer.**
#[derive(Debug, Default, Copy, Clone)]
pub enum PathStyle {
    /// If the first point is off-curve, check if the last is on-curve
    /// If it is, start there. If it isn't, start at the implied midpoint
    /// between first and last.
    #[default]
    FreeType,
    /// If the first point is off-curve, check if the second is on-curve.
    /// If it is, start there. If it isn't, start at the implied midpoint
    /// between first and second.
    ///
    /// Matches hb-draw's interpretation of a point stream.
    HarfBuzz,
}

impl OutlinePen for Vec<PathElement> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.push(PathElement::MoveTo { x, y })
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.push(PathElement::LineTo { x, y })
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.push(PathElement::QuadTo { cx0, cy0, x, y })
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.push(PathElement::CurveTo {
            cx0,
            cy0,
            cx1,
            cy1,
            x,
            y,
        })
    }

    fn close(&mut self) {
        self.push(PathElement::Close)
    }
}

/// Pen that drops all drawing output into the ether.
pub struct NullPen;

impl OutlinePen for NullPen {
    fn move_to(&mut self, _x: f32, _y: f32) {}
    fn line_to(&mut self, _x: f32, _y: f32) {}
    fn quad_to(&mut self, _cx0: f32, _cy0: f32, _x: f32, _y: f32) {}
    fn curve_to(&mut self, _cx0: f32, _cy0: f32, _cx1: f32, _cy1: f32, _x: f32, _y: f32) {}
    fn close(&mut self) {}
}

/// Pen that generates SVG style path data.
#[derive(Clone, Default, Debug)]
pub struct SvgPen(String, Option<usize>);

impl SvgPen {
    /// Creates a new SVG pen that formats floating point values with the
    /// standard behavior.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new SVG pen with the given precision (the number of digits
    /// that will be printed after the decimal).
    pub fn with_precision(precision: usize) -> Self {
        Self(String::default(), Some(precision))
    }

    /// Clears the content of the internal string.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    fn maybe_push_space(&mut self) {
        if !self.0.is_empty() {
            self.0.push(' ');
        }
    }
}

impl core::ops::Deref for SvgPen {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl OutlinePen for SvgPen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.maybe_push_space();
        let _ = if let Some(prec) = self.1 {
            write!(self.0, "M{x:.0$},{y:.0$}", prec)
        } else {
            write!(self.0, "M{x},{y}")
        };
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.maybe_push_space();
        let _ = if let Some(prec) = self.1 {
            write!(self.0, "L{x:.0$},{y:.0$}", prec)
        } else {
            write!(self.0, "L{x},{y}")
        };
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.maybe_push_space();
        let _ = if let Some(prec) = self.1 {
            write!(self.0, "Q{cx0:.0$},{cy0:.0$} {x:.0$},{y:.0$}", prec)
        } else {
            write!(self.0, "Q{cx0},{cy0} {x},{y}")
        };
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.maybe_push_space();
        let _ = if let Some(prec) = self.1 {
            write!(
                self.0,
                "C{cx0:.0$},{cy0:.0$} {cx1:.0$},{cy1:.0$} {x:.0$},{y:.0$}",
                prec
            )
        } else {
            write!(self.0, "C{cx0},{cy0} {cx1},{cy1} {x},{y}")
        };
    }

    fn close(&mut self) {
        self.maybe_push_space();
        self.0.push('Z');
    }
}

impl AsRef<str> for SvgPen {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<String> for SvgPen {
    fn from(value: String) -> Self {
        Self(value, None)
    }
}

impl From<SvgPen> for String {
    fn from(value: SvgPen) -> Self {
        value.0
    }
}

impl fmt::Display for SvgPen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Pen that generates the control bounds of a glyph outline.
#[derive(Clone, Default, Debug)]
pub struct ControlBoundsPen(Option<BoundingBox>);
impl ControlBoundsPen {
    /// Creates a new bounds pen.
    pub fn new() -> Self {
        Self(None)
    }

    /// Returns the bounding box collected by this pen.
    pub fn bounding_box(&self) -> Option<BoundingBox> {
        self.0
    }

    fn update_bounds(&mut self, x: f32, y: f32) {
        if let Some(bb) = &mut self.0 {
            bb.x_min = bb.x_min.min(x);
            bb.y_min = bb.y_min.min(y);
            bb.x_max = bb.x_max.max(x);
            bb.y_max = bb.y_max.max(y);
        } else {
            self.0 = Some(BoundingBox {
                x_min: x,
                y_min: y,
                x_max: x,
                y_max: y,
            });
        }
    }
}

impl OutlinePen for ControlBoundsPen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.update_bounds(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.update_bounds(x, y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.update_bounds(cx0, cy0);
        self.update_bounds(x, y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.update_bounds(cx0, cy0);
        self.update_bounds(cx1, cy1);
        self.update_bounds(x, y);
    }

    fn close(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svg_pen_precision() {
        let svg_data = [None, Some(1), Some(4)].map(|prec| {
            let mut pen = match prec {
                None => SvgPen::new(),
                Some(prec) => SvgPen::with_precision(prec),
            };
            pen.move_to(1.0, 2.45556);
            pen.line_to(1.2, 4.0);
            pen.quad_to(2.0345, 3.56789, -0.157, -425.07);
            pen.curve_to(-37.0010, 4.5, 2.0, 1.0, -0.5, -0.25);
            pen.close();
            pen.to_string()
        });
        let expected = [
            "M1,2.45556 L1.2,4 Q2.0345,3.56789 -0.157,-425.07 C-37.001,4.5 2,1 -0.5,-0.25 Z", 
            "M1.0,2.5 L1.2,4.0 Q2.0,3.6 -0.2,-425.1 C-37.0,4.5 2.0,1.0 -0.5,-0.2 Z", 
            "M1.0000,2.4556 L1.2000,4.0000 Q2.0345,3.5679 -0.1570,-425.0700 C-37.0010,4.5000 2.0000,1.0000 -0.5000,-0.2500 Z"
        ];
        for (result, expected) in svg_data.iter().zip(&expected) {
            assert_eq!(result, expected);
        }
    }
}
