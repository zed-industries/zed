//! Hit testing.

use super::geometry::{Point, Transform};
use super::mask::Mask;
use super::path_data::PathData;
use super::scratch::Scratch;
use super::style::{Fill, Style};

use core::cell::RefCell;

/// Builder for configuring and executing a hit test.
pub struct HitTest<'a, 's, D> {
    data: D,
    style: Style<'a>,
    transform: Option<Transform>,
    threshold: u8,
    scratch: RefCell<Option<&'s mut Scratch>>,
}

impl<'a, 's, D> HitTest<'a, 's, D>
where
    D: PathData,
{
    /// Creates a new hit test builder for the specified path data.
    pub fn new(data: D) -> Self {
        Self {
            data,
            style: Style::Fill(Fill::NonZero),
            transform: None,
            threshold: 0,
            scratch: RefCell::new(None),
        }
    }

    /// Creates a new hit test builder for the specified path data and scratch memory.
    pub fn with_scratch(data: D, scratch: &'s mut Scratch) -> Self {
        Self {
            data,
            style: Style::Fill(Fill::NonZero),
            transform: None,
            threshold: 0,
            scratch: RefCell::new(Some(scratch)),
        }
    }

    /// Sets the style of the path.
    pub fn style(&mut self, style: impl Into<Style<'a>>) -> &mut Self {
        self.style = style.into();
        self
    }

    /// Sets the transformation matrix of the path.
    pub fn transform(&mut self, transform: Option<Transform>) -> &mut Self {
        self.transform = transform;
        self
    }

    /// Sets the threshold value for determining whether a hit test registers.
    pub fn threshold(&mut self, threshold: u8) -> &mut Self {
        self.threshold = threshold;
        self
    }

    /// Returns true if the specified point is painted by the path.
    pub fn test(&self, point: impl Into<Point>) -> bool {
        let mut scratch = self.scratch.borrow_mut();
        let mut buf = [0u8; 1];
        let p = point.into() * -1.;
        if let Some(scratch) = scratch.as_mut() {
            Mask::with_scratch(&self.data, scratch)
                .style(self.style)
                .offset(p)
                .transform(self.transform)
                .size(1, 1)
                .render_into(&mut buf, None);
        } else {
            Mask::new(&self.data)
                .style(self.style)
                .offset(p)
                .transform(self.transform)
                .size(1, 1)
                .render_into(&mut buf, None);
        }
        if self.threshold == 0xFF {
            buf[0] >= self.threshold
        } else {
            buf[0] > self.threshold
        }
    }
}
