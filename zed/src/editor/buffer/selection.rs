use crate::{
    editor::{
        buffer::{Anchor, Buffer, Point, ToOffset as _, ToPoint as _},
        Bias, DisplayMapSnapshot, DisplayPoint,
    },
    time,
};
use std::{cmp::Ordering, mem, ops::Range};

pub type SelectionSetId = time::Lamport;
pub type SelectionsVersion = usize;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SelectionGoal {
    None,
    Column(u32),
    ColumnRange { start: u32, end: u32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selection {
    pub id: usize,
    pub start: Anchor,
    pub end: Anchor,
    pub reversed: bool,
    pub goal: SelectionGoal,
}

impl Selection {
    pub fn head(&self) -> &Anchor {
        if self.reversed {
            &self.start
        } else {
            &self.end
        }
    }

    pub fn set_head(&mut self, buffer: &Buffer, cursor: Anchor) {
        if cursor.cmp(self.tail(), buffer).unwrap() < Ordering::Equal {
            if !self.reversed {
                mem::swap(&mut self.start, &mut self.end);
                self.reversed = true;
            }
            self.start = cursor;
        } else {
            if self.reversed {
                mem::swap(&mut self.start, &mut self.end);
                self.reversed = false;
            }
            self.end = cursor;
        }
    }

    pub fn tail(&self) -> &Anchor {
        if self.reversed {
            &self.end
        } else {
            &self.start
        }
    }

    pub fn point_range(&self, buffer: &Buffer) -> Range<Point> {
        let start = self.start.to_point(buffer);
        let end = self.end.to_point(buffer);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    pub fn offset_range(&self, buffer: &Buffer) -> Range<usize> {
        let start = self.start.to_offset(buffer);
        let end = self.end.to_offset(buffer);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    pub fn display_range(&self, map: &DisplayMapSnapshot) -> Range<DisplayPoint> {
        let start = self.start.to_display_point(map, Bias::Left);
        let end = self.end.to_display_point(map, Bias::Left);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    pub fn spanned_rows(
        &self,
        include_end_if_at_line_start: bool,
        map: &DisplayMapSnapshot,
    ) -> (Range<u32>, Range<u32>) {
        let display_start = self.start.to_display_point(map, Bias::Left);
        let mut display_end = self.end.to_display_point(map, Bias::Right);
        if !include_end_if_at_line_start
            && display_end.row() != map.max_point().row()
            && display_start.row() != display_end.row()
            && display_end.column() == 0
        {
            *display_end.row_mut() -= 1;
        }

        let (display_start, buffer_start) = map.prev_row_boundary(display_start);
        let (display_end, buffer_end) = map.next_row_boundary(display_end);

        (
            buffer_start.row..buffer_end.row + 1,
            display_start.row()..display_end.row() + 1,
        )
    }
}
