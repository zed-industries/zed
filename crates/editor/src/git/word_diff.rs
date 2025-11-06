use std::ops::Range;

use gpui::Hsla;
use multi_buffer::RowInfo;

use crate::DisplayPoint;

pub struct Cache;

impl Cache {
    pub fn update(&mut self) {}
    pub fn color_ranges(
        &self,
        row_info: impl Iterator<Item = RowInfo>,
    ) -> impl Iterator<Item = (Range<DisplayPoint>, Hsla)> {
        todo!()
    }
}
