use super::Buffer;
use crate::{time, util::Bias};
use anyhow::Result;
use std::{cmp::Ordering, ops::Range};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Anchor {
    pub offset: usize,
    pub bias: Bias,
    pub version: time::Global,
}

impl Anchor {
    pub fn min() -> Self {
        Self {
            offset: 0,
            bias: Bias::Left,
            version: Default::default(),
        }
    }

    pub fn max() -> Self {
        Self {
            offset: usize::MAX,
            bias: Bias::Right,
            version: Default::default(),
        }
    }

    pub fn cmp(&self, other: &Anchor, buffer: &Buffer) -> Result<Ordering> {
        if self == other {
            return Ok(Ordering::Equal);
        }

        let offset_comparison = if self.version == other.version {
            self.offset.cmp(&other.offset)
        } else {
            buffer
                .full_offset_for_anchor(self)
                .cmp(&buffer.full_offset_for_anchor(other))
        };

        Ok(offset_comparison.then_with(|| self.bias.cmp(&other.bias)))
    }

    pub fn bias_left(&self, buffer: &Buffer) -> Anchor {
        if self.bias == Bias::Left {
            self.clone()
        } else {
            buffer.anchor_before(self)
        }
    }

    pub fn bias_right(&self, buffer: &Buffer) -> Anchor {
        if self.bias == Bias::Right {
            self.clone()
        } else {
            buffer.anchor_after(self)
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
