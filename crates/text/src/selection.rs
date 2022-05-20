use crate::Anchor;
use crate::{rope::TextDimension, BufferSnapshot};
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SelectionGoal {
    None,
    Column(u32),
    ColumnRange { start: u32, end: u32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub fn head(&self) -> T {
        if self.reversed {
            self.start.clone()
        } else {
            self.end.clone()
        }
    }

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

    pub fn collapse_to(&mut self, point: T, new_goal: SelectionGoal) {
        self.start = point;
        self.end = point;
        self.goal = new_goal;
        self.reversed = false;
    }

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
