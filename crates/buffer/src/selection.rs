use crate::{Anchor, Buffer, Point, ToOffset as _, ToPoint as _};
use anyhow::anyhow;
use rpc::proto;
use std::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    mem,
    ops::Range,
    sync::Arc,
};

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
    pub selections: Arc<[Selection]>,
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

impl<'a> Into<proto::Selection> for &'a Selection {
    fn into(self) -> proto::Selection {
        proto::Selection {
            id: self.id as u64,
            start: Some((&self.start).into()),
            end: Some((&self.end).into()),
            reversed: self.reversed,
        }
    }
}

impl TryFrom<proto::Selection> for Selection {
    type Error = anyhow::Error;

    fn try_from(selection: proto::Selection) -> Result<Self, Self::Error> {
        Ok(Selection {
            id: selection.id as usize,
            start: selection
                .start
                .ok_or_else(|| anyhow!("missing selection start"))?
                .try_into()?,
            end: selection
                .end
                .ok_or_else(|| anyhow!("missing selection end"))?
                .try_into()?,
            reversed: selection.reversed,
            goal: SelectionGoal::None,
        })
    }
}

impl TryFrom<proto::SelectionSet> for SelectionSet {
    type Error = anyhow::Error;

    fn try_from(set: proto::SelectionSet) -> Result<Self, Self::Error> {
        Ok(Self {
            id: clock::Lamport {
                replica_id: set.replica_id as u16,
                value: set.lamport_timestamp,
            },
            active: set.is_active,
            selections: Arc::from(
                set.selections
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<Selection>, _>>()?,
            ),
        })
    }
}
