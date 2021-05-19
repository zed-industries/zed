mod buffer;
mod buffer_element;
pub mod buffer_view;
pub mod display_map;
pub mod movement;

pub use buffer::*;
pub use buffer_element::*;
pub use buffer_view::*;
pub use display_map::DisplayPoint;
use display_map::*;
use std::{cmp, ops::Range};

#[derive(Copy, Clone)]
pub enum Bias {
    Left,
    Right,
}

trait RangeExt<T> {
    fn sorted(&self) -> Range<T>;
}

impl<T: Ord + Clone> RangeExt<T> for Range<T> {
    fn sorted(&self) -> Self {
        cmp::min(&self.start, &self.end).clone()..cmp::max(&self.start, &self.end).clone()
    }
}
