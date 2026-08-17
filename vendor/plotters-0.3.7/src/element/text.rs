use std::borrow::Borrow;

use super::{Drawable, PointCollection};
use crate::style::{FontDesc, FontResult, LayoutBox, TextStyle};
use plotters_backend::{BackendCoord, DrawingBackend, DrawingErrorKind};

/// A single line text element. This can be owned or borrowed string, dependents on
/// `String` or `str` moved into.
pub struct Text<'a, Coord, T: Borrow<str>> {
    text: T,
    coord: Coord,
    style: TextStyle<'a>,
}

impl<'a, Coord, T: Borrow<str>> Text<'a, Coord, T> {
    /// Create a new text element
    /// - `text`: The text for the element
    /// - `points`: The upper left conner for the text element
    /// - `style`: The text style
    /// - Return the newly created text element
    pub fn new<S: Into<TextStyle<'a>>>(text: T, points: Coord, style: S) -> Self {
        Self {
            text,
            coord: points,
            style: style.into(),
        }
    }
}

impl<'b, 'a, Coord: 'a, T: Borrow<str> + 'a> PointCollection<'a, Coord> for &'a Text<'b, Coord, T> {
    type Point = &'a Coord;
    type IntoIter = std::iter::Once<&'a Coord>;
    fn point_iter(self) -> Self::IntoIter {
        std::iter::once(&self.coord)
    }
}

impl<'a, Coord: 'a, DB: DrawingBackend, T: Borrow<str>> Drawable<DB> for Text<'a, Coord, T> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        if let Some(a) = points.next() {
            return backend.draw_text(self.text.borrow(), &self.style, a);
        }
        Ok(())
    }
}

/// An multi-line text element. The `Text` element allows only single line text
/// and the `MultiLineText` supports drawing multiple lines
pub struct MultiLineText<'a, Coord, T: Borrow<str>> {
    lines: Vec<T>,
    coord: Coord,
    style: TextStyle<'a>,
    line_height: f64,
}

impl<'a, Coord, T: Borrow<str>> MultiLineText<'a, Coord, T> {
    /// Create an empty multi-line text element.
    /// Lines can be append to the empty multi-line by calling `push_line` method
    ///
    /// `pos`: The upper left corner
    /// `style`: The style of the text
    pub fn new<S: Into<TextStyle<'a>>>(pos: Coord, style: S) -> Self {
        MultiLineText {
            lines: vec![],
            coord: pos,
            style: style.into(),
            line_height: 1.25,
        }
    }

    /// Set the line height of the multi-line text element
    pub fn set_line_height(&mut self, value: f64) -> &mut Self {
        self.line_height = value;
        self
    }

    /// Push a new line into the given multi-line text
    /// `line`: The line to be pushed
    pub fn push_line<L: Into<T>>(&mut self, line: L) {
        self.lines.push(line.into());
    }

    /// Estimate the multi-line text element's dimension
    pub fn estimate_dimension(&self) -> FontResult<(i32, i32)> {
        let (mut mx, mut my) = (0, 0);

        for ((x, y), t) in self.layout_lines((0, 0)).zip(self.lines.iter()) {
            let (dx, dy) = self.style.font.box_size(t.borrow())?;
            mx = mx.max(x + dx as i32);
            my = my.max(y + dy as i32);
        }

        Ok((mx, my))
    }

    /// Move the location to the specified location
    pub fn relocate(&mut self, coord: Coord) {
        self.coord = coord
    }

    fn layout_lines(&self, (x0, y0): BackendCoord) -> impl Iterator<Item = BackendCoord> {
        let font_height = self.style.font.get_size();
        let actual_line_height = font_height * self.line_height;
        (0..self.lines.len() as u32).map(move |idx| {
            let y = f64::from(y0) + f64::from(idx) * actual_line_height;
            // TODO: Support text alignment as well, currently everything is left aligned
            let x = f64::from(x0);
            (x.round() as i32, y.round() as i32)
        })
    }
}

// Rewrite of the layout function for multiline-text. It crashes when UTF-8 is used
// instead of ASCII. Solution taken from:
// https://stackoverflow.com/questions/68122526/splitting-a-utf-8-string-into-chunks
// and modified for our purposes.
fn layout_multiline_text<'a, F: FnMut(&'a str)>(
    text: &'a str,
    max_width: u32,
    font: FontDesc<'a>,
    mut func: F,
) {
    for line in text.lines() {
        if max_width == 0 || line.is_empty() {
            func(line);
        } else {
            let mut indices = line.char_indices().map(|(idx, _)| idx).peekable();

            let it = std::iter::from_fn(|| {
                let start_idx = match indices.next() {
                    Some(idx) => idx,
                    None => return None,
                };

                // iterate over indices
                for idx in indices.by_ref() {
                    let substring = &line[start_idx..idx];
                    let width = font.box_size(substring).unwrap_or((0, 0)).0 as i32;
                    if width > max_width as i32 {
                        break;
                    }
                }

                let end_idx = match indices.peek() {
                    Some(idx) => *idx,
                    None => line.bytes().len(),
                };

                Some(&line[start_idx..end_idx])
            });

            for chunk in it {
                func(chunk);
            }
        }
    }
}

// Only run the test on Linux because the default font is different
// on other platforms, causing different multiline splits.
#[cfg(all(feature = "ttf", target_os = "linux"))]
#[test]
fn test_multi_layout() {
    use plotters_backend::{FontFamily, FontStyle};

    let font = FontDesc::new(FontFamily::SansSerif, 20 as f64, FontStyle::Bold);

    layout_multiline_text("öäabcde", 40, font, |txt| {
        println!("Got: {}", txt);
        assert!(txt == "öäabc" || txt == "de");
    });

    let font = FontDesc::new(FontFamily::SansSerif, 20 as f64, FontStyle::Bold);
    layout_multiline_text("öä", 100, font, |txt| {
        // This does not divide the line, but still crashed in the previous implementation
        // of layout_multiline_text. So this test should be reliable
        println!("Got: {}", txt);
        assert_eq!(txt, "öä")
    });
}

impl<'a, T: Borrow<str>> MultiLineText<'a, BackendCoord, T> {
    /// Compute the line layout
    pub fn compute_line_layout(&self) -> FontResult<Vec<LayoutBox>> {
        let mut ret = vec![];
        for ((x, y), t) in self.layout_lines(self.coord).zip(self.lines.iter()) {
            let (dx, dy) = self.style.font.box_size(t.borrow())?;
            ret.push(((x, y), (x + dx as i32, y + dy as i32)));
        }
        Ok(ret)
    }
}

impl<'a, Coord> MultiLineText<'a, Coord, &'a str> {
    /// Parse a multi-line text into an multi-line element.
    ///
    /// `text`: The text that is parsed
    /// `pos`: The position of the text
    /// `style`: The style for this text
    /// `max_width`: The width of the multi-line text element, the line will break
    /// into two lines if the line is wider than the max_width. If 0 is given, do not
    /// do any line wrapping
    pub fn from_str<ST: Into<&'a str>, S: Into<TextStyle<'a>>>(
        text: ST,
        pos: Coord,
        style: S,
        max_width: u32,
    ) -> Self {
        let text = text.into();
        let mut ret = MultiLineText::new(pos, style);

        layout_multiline_text(text, max_width, ret.style.font.clone(), |l| {
            ret.push_line(l)
        });
        ret
    }
}

impl<'a, Coord> MultiLineText<'a, Coord, String> {
    /// Parse a multi-line text into an multi-line element.
    ///
    /// `text`: The text that is parsed
    /// `pos`: The position of the text
    /// `style`: The style for this text
    /// `max_width`: The width of the multi-line text element, the line will break
    /// into two lines if the line is wider than the max_width. If 0 is given, do not
    /// do any line wrapping
    pub fn from_string<S: Into<TextStyle<'a>>>(
        text: String,
        pos: Coord,
        style: S,
        max_width: u32,
    ) -> Self {
        let mut ret = MultiLineText::new(pos, style);

        layout_multiline_text(text.as_str(), max_width, ret.style.font.clone(), |l| {
            ret.push_line(l.to_string())
        });
        ret
    }
}

impl<'b, 'a, Coord: 'a, T: Borrow<str> + 'a> PointCollection<'a, Coord>
    for &'a MultiLineText<'b, Coord, T>
{
    type Point = &'a Coord;
    type IntoIter = std::iter::Once<&'a Coord>;
    fn point_iter(self) -> Self::IntoIter {
        std::iter::once(&self.coord)
    }
}

impl<'a, Coord: 'a, DB: DrawingBackend, T: Borrow<str>> Drawable<DB>
    for MultiLineText<'a, Coord, T>
{
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        if let Some(a) = points.next() {
            for (point, text) in self.layout_lines(a).zip(self.lines.iter()) {
                backend.draw_text(text.borrow(), &self.style, point)?;
            }
        }
        Ok(())
    }
}
