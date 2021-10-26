use crate::{Point, ToOffset};

use super::{Buffer, Content};
use anyhow::Result;
use std::{cmp::Ordering, ops::Range};
use sum_tree::{Bias, SumTree};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Anchor {
    pub full_offset: usize,
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

pub struct AnchorRangeMultimap<T: Clone> {
    entries: SumTree<AnchorRangeMultimapEntry<T>>,
    pub(crate) version: clock::Global,
    pub(crate) start_bias: Bias,
    pub(crate) end_bias: Bias,
}

#[derive(Clone)]
struct AnchorRangeMultimapEntry<T> {
    range: FullOffsetRange,
    value: T,
}

#[derive(Clone, Debug)]
struct FullOffsetRange {
    start: usize,
    end: usize,
}

#[derive(Clone, Debug)]
struct AnchorRangeMultimapSummary {
    start: usize,
    end: usize,
    min_start: usize,
    max_end: usize,
    count: usize,
}

impl Anchor {
    pub fn min() -> Self {
        Self {
            full_offset: 0,
            bias: Bias::Left,
            version: Default::default(),
        }
    }

    pub fn max() -> Self {
        Self {
            full_offset: usize::MAX,
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
            self.full_offset.cmp(&other.full_offset)
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
    pub fn to_points<'a>(
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
    pub fn to_points<'a>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl Iterator<Item = Point> + 'a {
        self.0.to_points(content).map(move |(point, _)| point)
    }
}

impl<T> AnchorRangeMap<T> {
    pub fn to_point_ranges<'a>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl Iterator<Item = (Range<Point>, &'a T)> + 'a {
        let content = content.into();
        content
            .summaries_for_anchor_ranges(self)
            .map(move |(range, value)| ((range.start.lines..range.end.lines), value))
    }

    pub fn version(&self) -> &clock::Global {
        &self.version
    }
}

impl AnchorRangeSet {
    pub fn to_point_ranges<'a>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl Iterator<Item = Range<Point>> + 'a {
        self.0.to_point_ranges(content).map(|(range, _)| range)
    }

    pub fn version(&self) -> &clock::Global {
        self.0.version()
    }
}

impl<T: Clone> AnchorRangeMultimap<T> {
    fn intersecting_point_ranges<'a, O: ToOffset>(
        &'a self,
        range: Range<O>,
        content: impl Into<Content<'a>>,
        inclusive: bool,
    ) -> impl Iterator<Item = (usize, Range<Point>, &T)> + 'a {
        use super::ToPoint as _;

        let content = content.into();
        let start = range.start.to_full_offset(&content, self.start_bias);
        let end = range.end.to_full_offset(&content, self.end_bias);
        let mut cursor = self.entries.filter::<_, usize>(
            move |summary: &AnchorRangeMultimapSummary| {
                if inclusive {
                    start <= summary.max_end && end >= summary.min_start
                } else {
                    start < summary.max_end && end > summary.min_start
                }
            },
            &(),
        );
        let mut anchor = Anchor {
            full_offset: 0,
            bias: Bias::Left,
            version: self.version.clone(),
        };
        std::iter::from_fn(move || {
            if let Some(item) = cursor.item() {
                let ix = *cursor.start();
                anchor.full_offset = item.range.start;
                anchor.bias = self.start_bias;
                let start = anchor.to_point(&content);
                anchor.full_offset = item.range.end;
                anchor.bias = self.end_bias;
                let end = anchor.to_point(&content);
                let value = &item.value;
                cursor.next(&());
                Some((ix, start..end, value))
            } else {
                None
            }
        })
    }
}

impl<T: Clone> sum_tree::Item for AnchorRangeMultimapEntry<T> {
    type Summary = AnchorRangeMultimapSummary;

    fn summary(&self) -> Self::Summary {
        AnchorRangeMultimapSummary {
            start: self.range.start,
            end: self.range.end,
            min_start: self.range.start,
            max_end: self.range.end,
            count: 1,
        }
    }
}

impl Default for AnchorRangeMultimapSummary {
    fn default() -> Self {
        Self {
            start: 0,
            end: usize::MAX,
            min_start: usize::MAX,
            max_end: 0,
            count: 0,
        }
    }
}

impl sum_tree::Summary for AnchorRangeMultimapSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &Self::Context) {
        self.min_start = self.min_start.min(other.min_start);
        self.max_end = self.max_end.max(other.max_end);

        #[cfg(debug_assertions)]
        {
            let start_comparison = self.start.cmp(&other.start);
            assert!(start_comparison <= Ordering::Equal);
            if start_comparison == Ordering::Equal {
                assert!(self.end.cmp(&other.end) >= Ordering::Equal);
            }
        }

        self.start = other.start;
        self.end = other.end;
        self.count += other.count;
    }
}

impl Default for FullOffsetRange {
    fn default() -> Self {
        Self {
            start: 0,
            end: usize::MAX,
        }
    }
}

impl<'a> sum_tree::Dimension<'a, AnchorRangeMultimapSummary> for usize {
    fn add_summary(&mut self, summary: &'a AnchorRangeMultimapSummary, _: &()) {
        *self += summary.count;
    }
}

impl<'a> sum_tree::Dimension<'a, AnchorRangeMultimapSummary> for FullOffsetRange {
    fn add_summary(&mut self, summary: &'a AnchorRangeMultimapSummary, _: &()) {
        self.start = summary.start;
        self.end = summary.end;
    }
}

impl<'a> sum_tree::SeekTarget<'a, AnchorRangeMultimapSummary, FullOffsetRange> for FullOffsetRange {
    fn cmp(&self, cursor_location: &FullOffsetRange, _: &()) -> Ordering {
        Ord::cmp(&self.start, &cursor_location.start)
            .then_with(|| Ord::cmp(&cursor_location.end, &self.end))
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
