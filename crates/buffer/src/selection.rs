use crate::{Anchor, Buffer, Point, ToOffset as _, ToPoint as _};
use std::{cmp::Ordering, mem, ops::Range};

pub type SelectionSetId = clock::Lamport;
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
}
