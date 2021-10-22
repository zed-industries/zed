use crate::{AnchorRangeMap, Buffer, Content, Point, ToOffset, ToPoint};
use rpc::proto;
use std::{cmp::Ordering, ops::Range, sync::Arc};
use sum_tree::Bias;

pub type SelectionSetId = clock::Lamport;
pub type SelectionsVersion = usize;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSet {
    pub id: SelectionSetId,
    pub active: bool,
    pub selections: Arc<AnchorRangeMap<SelectionState>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SelectionState {
    pub id: usize,
    pub reversed: bool,
    pub goal: SelectionGoal,
}

impl<T: ToOffset + ToPoint + Copy + Ord> Selection<T> {
    pub fn head(&self) -> T {
        if self.reversed {
            self.start
        } else {
            self.end
        }
    }

    pub fn set_head(&mut self, head: T) {
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
    }

    pub fn tail(&self) -> T {
        if self.reversed {
            self.end
        } else {
            self.start
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

impl SelectionSet {
    pub fn offset_selections<'a>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl 'a + Iterator<Item = Selection<usize>> {
        self.selections
            .offset_ranges(content)
            .map(|(range, state)| Selection {
                id: state.id,
                start: range.start,
                end: range.end,
                reversed: state.reversed,
                goal: state.goal,
            })
    }

    pub fn point_selections<'a>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl 'a + Iterator<Item = Selection<Point>> {
        self.selections
            .point_ranges(content)
            .map(|(range, state)| Selection {
                id: state.id,
                start: range.start,
                end: range.end,
                reversed: state.reversed,
                goal: state.goal,
            })
    }
}

impl<'a> Into<proto::SelectionSet> for &'a SelectionSet {
    fn into(self) -> proto::SelectionSet {
        let version = self.selections.version();
        let entries = self.selections.raw_entries();
        proto::SelectionSet {
            replica_id: self.id.replica_id as u32,
            lamport_timestamp: self.id.value as u32,
            is_active: self.active,
            version: version.into(),
            selections: entries
                .iter()
                .map(|(range, state)| proto::Selection {
                    id: state.id as u64,
                    start: range.start.0 as u64,
                    end: range.end.0 as u64,
                    reversed: state.reversed,
                })
                .collect(),
        }
    }
}

impl From<proto::SelectionSet> for SelectionSet {
    fn from(set: proto::SelectionSet) -> Self {
        Self {
            id: clock::Lamport {
                replica_id: set.replica_id as u16,
                value: set.lamport_timestamp,
            },
            active: set.is_active,
            selections: Arc::new(AnchorRangeMap::from_raw(
                set.version.into(),
                set.selections
                    .into_iter()
                    .map(|selection| {
                        let range = (selection.start as usize, Bias::Left)
                            ..(selection.end as usize, Bias::Right);
                        let state = SelectionState {
                            id: selection.id as usize,
                            reversed: selection.reversed,
                            goal: SelectionGoal::None,
                        };
                        (range, state)
                    })
                    .collect(),
            )),
        }
    }
}
