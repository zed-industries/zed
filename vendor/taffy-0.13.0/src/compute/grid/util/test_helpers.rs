//! Helpers for use in unit tests within the grid module
use super::super::OriginZeroLine;
use crate::prelude::*;
use crate::style::{Dimension, GridPlacement, Style};

pub(crate) trait CreateParentTestNode {
    fn into_grid(self) -> Style;
}
impl CreateParentTestNode for (f32, f32, i32, i32) {
    fn into_grid(self) -> Style {
        Style {
            display: Display::Grid,
            size: Size { width: Dimension::from_length(self.0), height: Dimension::from_length(self.1) },
            grid_template_columns: vec![fr(1f32); self.2 as usize],
            grid_template_rows: vec![fr(1f32); self.3 as usize],
            ..Default::default()
        }
    }
}
pub(crate) trait CreateChildTestNode {
    fn into_grid_child(self) -> Style;
}
impl CreateChildTestNode
    for (GridPlacement<String>, GridPlacement<String>, GridPlacement<String>, GridPlacement<String>)
{
    fn into_grid_child(self) -> Style<String> {
        Style {
            display: Display::Grid,
            grid_column: Line { start: self.0, end: self.1 },
            grid_row: Line { start: self.2, end: self.3 },
            ..Default::default()
        }
    }
}

pub(crate) trait CreateExpectedPlacement {
    fn into_oz(self) -> (OriginZeroLine, OriginZeroLine, OriginZeroLine, OriginZeroLine);
}
impl CreateExpectedPlacement for (i16, i16, i16, i16) {
    fn into_oz(self) -> (OriginZeroLine, OriginZeroLine, OriginZeroLine, OriginZeroLine) {
        (OriginZeroLine(self.0), OriginZeroLine(self.1), OriginZeroLine(self.2), OriginZeroLine(self.3))
    }
}
