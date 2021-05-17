use crate::{
    editor::{
        buffer::{Anchor, Buffer, Point, ToPoint},
        display_map::{Bias, DisplayMap},
        DisplayPoint,
    },
    time,
};
use gpui::AppContext;
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

    pub fn range(&self, buffer: &Buffer) -> Range<Point> {
        let start = self.start.to_point(buffer);
        let end = self.end.to_point(buffer);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    pub fn display_range(&self, map: &DisplayMap, app: &AppContext) -> Range<DisplayPoint> {
        let start = self.start.to_display_point(map, app).unwrap();
        let end = self.end.to_display_point(map, app).unwrap();
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    pub fn buffer_rows_for_display_rows(
        &self,
        include_end_if_at_line_start: bool,
        map: &DisplayMap,
        ctx: &AppContext,
    ) -> (Range<u32>, Range<u32>) {
        let display_start = self.start.to_display_point(map, ctx).unwrap();
        let buffer_start = DisplayPoint::new(display_start.row(), 0)
            .to_buffer_point(map, Bias::Left, ctx)
            .unwrap();

        let mut display_end = self.end.to_display_point(map, ctx).unwrap();
        if !include_end_if_at_line_start
            && display_end.row() != map.max_point(ctx).row()
            && display_start.row() != display_end.row()
            && display_end.column() == 0
        {
            *display_end.row_mut() -= 1;
        }
        let buffer_end = DisplayPoint::new(
            display_end.row(),
            map.line_len(display_end.row(), ctx).unwrap(),
        )
        .to_buffer_point(map, Bias::Left, ctx)
        .unwrap();

        (
            buffer_start.row..buffer_end.row + 1,
            display_start.row()..display_end.row() + 1,
        )
    }
}
