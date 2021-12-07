mod anchor;
mod buffer;
mod operation_queue;
mod patch;
mod point;
mod point_utf16;
#[cfg(any(test, feature = "test-support"))]
pub mod random_char_iter;
pub mod rope;
mod selection;
pub mod subscription;

pub use anchor::*;
pub use buffer::*;
pub use patch::Patch;
pub use point::*;
pub use point_utf16::*;
#[cfg(any(test, feature = "test-support"))]
pub use random_char_iter::*;
pub use rope::{Chunks, Rope, TextSummary};
pub use selection::*;
pub use subscription::*;
pub use sum_tree::Bias;

pub trait ToOffset {
    fn to_offset<'a>(&self, content: &BufferSnapshot) -> usize;
}

impl ToOffset for Point {
    fn to_offset<'a>(&self, content: &BufferSnapshot) -> usize {
        content.point_to_offset(*self)
    }
}

impl ToOffset for PointUtf16 {
    fn to_offset<'a>(&self, content: &BufferSnapshot) -> usize {
        content.point_utf16_to_offset(*self)
    }
}

impl ToOffset for usize {
    fn to_offset<'a>(&self, content: &BufferSnapshot) -> usize {
        assert!(*self <= content.len(), "offset is out of range");
        *self
    }
}

impl ToOffset for Anchor {
    fn to_offset<'a>(&self, content: &BufferSnapshot) -> usize {
        content.summary_for_anchor(self)
    }
}

impl<'a> ToOffset for &'a Anchor {
    fn to_offset(&self, content: &BufferSnapshot) -> usize {
        content.summary_for_anchor(self)
    }
}

pub trait ToPoint {
    fn to_point<'a>(&self, content: &BufferSnapshot) -> Point;
}

impl ToPoint for Anchor {
    fn to_point<'a>(&self, content: &BufferSnapshot) -> Point {
        content.summary_for_anchor(self)
    }
}

impl ToPoint for usize {
    fn to_point<'a>(&self, content: &BufferSnapshot) -> Point {
        content.offset_to_point(*self)
    }
}

impl ToPoint for Point {
    fn to_point<'a>(&self, _: &BufferSnapshot) -> Point {
        *self
    }
}

pub trait FromAnchor {
    fn from_anchor(anchor: &Anchor, content: &BufferSnapshot) -> Self;
}

impl FromAnchor for Point {
    fn from_anchor(anchor: &Anchor, content: &BufferSnapshot) -> Self {
        anchor.to_point(content)
    }
}

impl FromAnchor for usize {
    fn from_anchor(anchor: &Anchor, content: &BufferSnapshot) -> Self {
        anchor.to_offset(content)
    }
}
