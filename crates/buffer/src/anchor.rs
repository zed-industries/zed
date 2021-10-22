use crate::Point;

use super::{Buffer, Content};
use anyhow::Result;
use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter},
    ops::Range,
};
use sum_tree::Bias;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Anchor {
    pub offset: usize,
    pub bias: Bias,
    pub version: clock::Global,
}

#[derive(Clone)]
pub struct AnchorMap<T> {
    pub(crate) version: clock::Global,
    pub(crate) entries: Vec<((usize, Bias), T)>,
}

#[derive(Clone)]
pub struct AnchorSet(pub(crate) AnchorMap<()>);

#[derive(Clone)]
pub struct AnchorRangeMap<T> {
    pub(crate) version: clock::Global,
    pub(crate) entries: Vec<(Range<(usize, Bias)>, T)>,
}

#[derive(Clone)]
pub struct AnchorRangeSet(pub(crate) AnchorRangeMap<()>);

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

    pub fn cmp<'a>(&self, other: &Anchor, buffer: impl Into<Content<'a>>) -> Result<Ordering> {
        let buffer = buffer.into();

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

impl<T> AnchorMap<T> {
    pub fn points<'a>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl Iterator<Item = (Point, &'a T)> + 'a {
        let content = content.into();
        content
            .summaries_for_anchors(self)
            .map(move |(sum, value)| (sum.lines, value))
    }

    pub fn version(&self) -> &clock::Global {
        &self.version
    }
}

impl AnchorSet {
    pub fn points<'a>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl Iterator<Item = Point> + 'a {
        self.0.points(content).map(move |(point, _)| point)
    }
}

impl<T> AnchorRangeMap<T> {
    pub fn from_raw(version: clock::Global, entries: Vec<(Range<(usize, Bias)>, T)>) -> Self {
        Self { version, entries }
    }

    pub fn raw_entries(&self) -> &[(Range<(usize, Bias)>, T)] {
        &self.entries
    }

    pub fn point_ranges<'a>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl Iterator<Item = (Range<Point>, &'a T)> + 'a {
        let content = content.into();
        content
            .summaries_for_anchor_ranges(self)
            .map(move |(range, value)| ((range.start.lines..range.end.lines), value))
    }

    pub fn offset_ranges<'a>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl Iterator<Item = (Range<usize>, &'a T)> + 'a {
        let content = content.into();
        content
            .summaries_for_anchor_ranges(self)
            .map(move |(range, value)| ((range.start.bytes..range.end.bytes), value))
    }

    pub fn version(&self) -> &clock::Global {
        &self.version
    }
}

impl<T: PartialEq> PartialEq for AnchorRangeMap<T> {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.entries == other.entries
    }
}

impl<T: Eq> Eq for AnchorRangeMap<T> {}

impl<T: Debug> Debug for AnchorRangeMap<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut f = f.debug_map();
        for (range, value) in &self.entries {
            f.key(range);
            f.value(value);
        }
        f.finish()
    }
}

impl AnchorRangeSet {
    pub fn to_point_ranges<'a>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl Iterator<Item = Range<Point>> + 'a {
        self.0.point_ranges(content).map(|(range, _)| range)
    }

    pub fn version(&self) -> &clock::Global {
        self.0.version()
    }
}

pub trait AnchorRangeExt {
    fn cmp<'a>(&self, b: &Range<Anchor>, buffer: impl Into<Content<'a>>) -> Result<Ordering>;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp<'a>(&self, other: &Range<Anchor>, buffer: impl Into<Content<'a>>) -> Result<Ordering> {
        let buffer = buffer.into();
        Ok(match self.start.cmp(&other.start, &buffer)? {
            Ordering::Equal => other.end.cmp(&self.end, buffer)?,
            ord @ _ => ord,
        })
    }
}
