use crate::{Anchor, BufferSnapshot, TextDimension};
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SelectionGoal {
    None,
    HorizontalPosition(f64),
    HorizontalRange { start: f64, end: f64 },
    WrappedHorizontalPosition((u32, f32)),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Selection<T> {
    pub id: usize,
    pub start: T,
    pub end: T,
    pub reversed: bool,
    pub goal: SelectionGoal,
}

impl Default for SelectionGoal {
    fn default() -> Self {
        Self::None
    }
}

impl<T: Clone> Selection<T> {
    /// A place where the selection had stopped at.
    pub fn head(&self) -> T {
        if self.reversed {
            self.start.clone()
        } else {
            self.end.clone()
        }
    }

    /// A place where selection was initiated from.
    pub fn tail(&self) -> T {
        if self.reversed {
            self.end.clone()
        } else {
            self.start.clone()
        }
    }

    pub fn map<F, S>(&self, f: F) -> Selection<S>
    where
        F: Fn(T) -> S,
    {
        Selection::<S> {
            id: self.id,
            start: f(self.start.clone()),
            end: f(self.end.clone()),
            reversed: self.reversed,
            goal: self.goal,
        }
    }

    pub fn collapse_to(&mut self, point: T, new_goal: SelectionGoal) {
        self.start = point.clone();
        self.end = point;
        self.goal = new_goal;
        self.reversed = false;
    }
}

impl<T: Copy + Ord> Selection<T> {
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn set_head(&mut self, head: T, new_goal: SelectionGoal) {
        if head.cmp(&self.tail()) < Ordering::Equal {
            if !self.reversed {
                self.end = self.start;
                self.reversed = true;
            }
            self.start = head;
        } else {
            if self.reversed {
                self.start = self.end;
                self.reversed = false;
            }
            self.end = head;
        }
        self.goal = new_goal;
    }

    pub fn set_tail(&mut self, tail: T, new_goal: SelectionGoal) {
        if tail.cmp(&self.head()) <= Ordering::Equal {
            if self.reversed {
                self.end = self.start;
                self.reversed = false;
            }
            self.start = tail;
        } else {
            if !self.reversed {
                self.start = self.end;
                self.reversed = true;
            }
            self.end = tail;
        }
        self.goal = new_goal;
    }

    pub fn set_head_tail(&mut self, head: T, tail: T, new_goal: SelectionGoal) {
        if head < tail {
            self.reversed = true;
            self.start = head;
            self.end = tail;
        } else {
            self.reversed = false;
            self.start = tail;
            self.end = head;
        }
        self.goal = new_goal;
    }

    pub fn swap_head_tail(&mut self) {
        if self.reversed {
            self.reversed = false;
        } else {
            std::mem::swap(&mut self.start, &mut self.end);
        }
    }
}

impl<T: Copy> Selection<T> {
    pub fn range(&self) -> Range<T> {
        self.start..self.end
    }
}

impl Selection<usize> {
    #[cfg(feature = "test-support")]
    pub fn from_offset(offset: usize) -> Self {
        Selection {
            id: 0,
            start: offset,
            end: offset,
            goal: SelectionGoal::None,
            reversed: false,
        }
    }

    pub fn equals(&self, offset_range: &Range<usize>) -> bool {
        self.start == offset_range.start && self.end == offset_range.end
    }
}

impl Selection<Anchor> {
    pub fn resolve<'a, D: 'a + TextDimension>(
        &'a self,
        snapshot: &'a BufferSnapshot,
    ) -> Selection<D> {
        Selection {
            id: self.id,
            start: snapshot.summary_for_anchor(&self.start),
            end: snapshot.summary_for_anchor(&self.end),
            reversed: self.reversed,
            goal: self.goal,
        }
    }
}
