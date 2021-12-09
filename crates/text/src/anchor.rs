use crate::{rope::TextDimension, Snapshot};

use super::{Buffer, ToOffset};
use anyhow::Result;
use std::{cmp::Ordering, fmt::Debug, ops::Range};
use sum_tree::Bias;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Anchor {
    Min,
    Insertion {
        timestamp: clock::Local,
        offset: usize,
        bias: Bias,
    },
    Max,
}

impl Anchor {
    pub fn min() -> Self {
        Self::Min
    }

    pub fn max() -> Self {
        Self::Max
    }

    pub fn cmp<'a>(&self, other: &Anchor, buffer: &Snapshot) -> Result<Ordering> {
        match (self, other) {
            (Self::Min, Self::Min) => Ok(Ordering::Equal),
            (Self::Min, _) => Ok(Ordering::Less),
            (_, Self::Min) => Ok(Ordering::Greater),
            (Self::Max, Self::Max) => Ok(Ordering::Equal),
            (Self::Max, _) => Ok(Ordering::Greater),
            (_, Self::Max) => Ok(Ordering::Less),
            (
                Self::Insertion {
                    timestamp: lhs_id,
                    bias: lhs_bias,
                    offset: lhs_offset,
                },
                Self::Insertion {
                    timestamp: rhs_id,
                    bias: rhs_bias,
                    offset: rhs_offset,
                },
            ) => {
                let offset_comparison = if lhs_id == rhs_id {
                    lhs_offset.cmp(&rhs_offset)
                } else {
                    buffer
                        .full_offset_for_anchor(self)
                        .cmp(&buffer.full_offset_for_anchor(other))
                };

                Ok(offset_comparison.then_with(|| lhs_bias.cmp(&rhs_bias)))
            }
        }
    }

    pub fn bias_left(&self, buffer: &Buffer) -> Anchor {
        match self {
            Anchor::Min => Anchor::Min,
            Anchor::Insertion { bias, .. } => {
                if *bias == Bias::Left {
                    self.clone()
                } else {
                    buffer.anchor_before(self)
                }
            }
            Anchor::Max => buffer.anchor_before(self),
        }
    }

    pub fn bias_right(&self, buffer: &Buffer) -> Anchor {
        match self {
            Anchor::Min => buffer.anchor_after(self),
            Anchor::Insertion { bias, .. } => {
                if *bias == Bias::Right {
                    self.clone()
                } else {
                    buffer.anchor_after(self)
                }
            }
            Anchor::Max => Anchor::Max,
        }
    }

    pub fn summary<'a, D>(&self, content: &'a Snapshot) -> D
    where
        D: TextDimension<'a>,
    {
        content.summary_for_anchor(self)
    }
}

pub trait AnchorRangeExt {
    fn cmp(&self, b: &Range<Anchor>, buffer: &Snapshot) -> Result<Ordering>;
    fn to_offset(&self, content: &Snapshot) -> Range<usize>;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &Snapshot) -> Result<Ordering> {
        Ok(match self.start.cmp(&other.start, buffer)? {
            Ordering::Equal => other.end.cmp(&self.end, buffer)?,
            ord @ _ => ord,
        })
    }

    fn to_offset(&self, content: &Snapshot) -> Range<usize> {
        self.start.to_offset(&content)..self.end.to_offset(&content)
    }
}
