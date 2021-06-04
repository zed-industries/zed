use super::Buffer;
use crate::{time, util::Bias};
use anyhow::Result;
use std::{cmp::Ordering, ops::Range};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Anchor {
    Start,
    End,
    Middle {
        offset: usize,
        bias: Bias,
        version: time::Global,
    },
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
                    version: self_version,
                },
                Anchor::Middle {
                    offset: other_offset,
                    bias: other_bias,
                    version: other_version,
                },
            ) => {
                let offset_comparison = if self_version == other_version {
                    self_offset.cmp(other_offset)
                } else {
                    buffer
                        .full_offset_for_anchor(self)
                        .cmp(&buffer.full_offset_for_anchor(other))
                };

                offset_comparison.then_with(|| self_bias.cmp(&other_bias))
            }
        })
    }

    pub fn bias_left(&self, buffer: &Buffer) -> Anchor {
        match self {
            Anchor::Start
            | Anchor::Middle {
                bias: Bias::Left, ..
            } => self.clone(),
            _ => buffer.anchor_before(self),
        }
    }

    pub fn bias_right(&self, buffer: &Buffer) -> Anchor {
        match self {
            Anchor::End
            | Anchor::Middle {
                bias: Bias::Right, ..
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
