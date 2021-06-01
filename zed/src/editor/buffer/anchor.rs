use super::Buffer;
use crate::{sum_tree, time};
use anyhow::Result;
use std::{cmp::Ordering, ops::Range};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Anchor {
    Start,
    End,
    Middle {
        offset: usize,
        bias: AnchorBias,
        version: time::Global,
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AnchorBias {
    Left,
    Right,
}

impl AnchorBias {
    pub fn to_seek_bias(self) -> sum_tree::SeekBias {
        match self {
            AnchorBias::Left => sum_tree::SeekBias::Left,
            AnchorBias::Right => sum_tree::SeekBias::Right,
        }
    }
}

impl PartialOrd for AnchorBias {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnchorBias {
    fn cmp(&self, other: &Self) -> Ordering {
        use AnchorBias::*;

        if self == other {
            return Ordering::Equal;
        }

        match (self, other) {
            (Left, _) => Ordering::Less,
            (Right, _) => Ordering::Greater,
        }
    }
}

impl Anchor {
    pub fn cmp(&self, other: &Anchor, buffer: &Buffer) -> Result<Ordering> {
        if self == other {
            return Ok(Ordering::Equal);
        }

        Ok(match (self, other) {
            (Anchor::Start, _) | (_, Anchor::End) => Ordering::Less,
            (Anchor::End, _) | (_, Anchor::Start) => Ordering::Greater,
            (
                Anchor::Middle {
                    bias: self_bias, ..
                },
                Anchor::Middle {
                    bias: other_bias, ..
                },
            ) => buffer
                .fragment_ix_for_anchor(self)
                .cmp(&buffer.fragment_ix_for_anchor(other))
                .then_with(|| self_bias.cmp(&other_bias)),
        })
    }

    pub fn bias_left(&self, buffer: &Buffer) -> Anchor {
        match self {
            Anchor::Start
            | Anchor::Middle {
                bias: AnchorBias::Left,
                ..
            } => self.clone(),
            _ => buffer.anchor_before(self),
        }
    }

    pub fn bias_right(&self, buffer: &Buffer) -> Anchor {
        match self {
            Anchor::End
            | Anchor::Middle {
                bias: AnchorBias::Right,
                ..
            } => self.clone(),
            _ => buffer.anchor_after(self),
        }
    }
}

pub trait AnchorRangeExt {
    fn cmp(&self, b: &Range<Anchor>, buffer: &Buffer) -> Result<Ordering>;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &Buffer) -> Result<Ordering> {
        Ok(match self.start.cmp(&other.start, buffer)? {
            Ordering::Equal => other.end.cmp(&self.end, buffer)?,
            ord @ _ => ord,
        })
    }
}
