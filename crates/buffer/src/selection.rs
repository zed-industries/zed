use crate::{Anchor, AnchorRangeMap, Buffer, Point, ToOffset as _, ToPoint as _};
use rpc::proto;
use std::{cmp::Ordering, mem, ops::Range, sync::Arc};
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
pub struct Selection {
    pub id: usize,
    pub start: Anchor,
    pub end: Anchor,
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
