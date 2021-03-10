use super::Buffer;
use crate::time;
use anyhow::Result;
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Anchor {
    Start,
    End,
    Middle {
        insertion_id: time::Local,
        offset: usize,
        bias: AnchorBias,
    },
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AnchorBias {
    Left,
    Right,
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
                    offset: self_offset,
                    bias: self_bias,
                    ..
                },
                Anchor::Middle {
                    offset: other_offset,
                    bias: other_bias,
                    ..
                },
            ) => buffer
                .fragment_id_for_anchor(self)?
                .cmp(buffer.fragment_id_for_anchor(other)?)
                .then_with(|| self_offset.cmp(other_offset))
                .then_with(|| self_bias.cmp(other_bias)),
        })
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
