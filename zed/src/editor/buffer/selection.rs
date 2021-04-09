use crate::{
    editor::{
        buffer::{Anchor, Buffer, Point, ToPoint},
        display_map::DisplayMap,
        DisplayPoint,
    },
    time,
};
use gpui::AppContext;
use std::{cmp::Ordering, mem, ops::Range};

pub type SelectionSetId = time::Lamport;
pub type SelectionsVersion = usize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selection {
    pub start: Anchor,
    pub end: Anchor,
    pub reversed: bool,
    pub goal_column: Option<u32>,
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
        let start = self.start.to_point(buffer).unwrap();
        let end = self.end.to_point(buffer).unwrap();
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
}
