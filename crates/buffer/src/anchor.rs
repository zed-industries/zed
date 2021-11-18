use crate::rope::TextDimension;

use super::{Buffer, Content, FromAnchor, FullOffset, Point, ToOffset};
use anyhow::Result;
use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter},
    ops::Range,
};
use sum_tree::{Bias, SumTree};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Anchor {
    pub full_offset: FullOffset,
    pub bias: Bias,
    pub version: clock::Global,
}

#[derive(Clone)]
pub struct AnchorMap<T> {
    pub(crate) version: clock::Global,
    pub(crate) entries: Vec<((FullOffset, Bias), T)>,
}

#[derive(Clone)]
pub struct AnchorSet(pub(crate) AnchorMap<()>);

#[derive(Clone)]
pub struct AnchorRangeMap<T> {
    pub(crate) version: clock::Global,
    pub(crate) entries: Vec<(Range<(FullOffset, Bias)>, T)>,
}

#[derive(Clone)]
pub struct AnchorRangeSet(pub(crate) AnchorRangeMap<()>);

#[derive(Clone)]
pub struct AnchorRangeMultimap<T: Clone> {
    pub(crate) entries: SumTree<AnchorRangeMultimapEntry<T>>,
    pub(crate) version: clock::Global,
    pub(crate) start_bias: Bias,
    pub(crate) end_bias: Bias,
}

#[derive(Clone)]
pub(crate) struct AnchorRangeMultimapEntry<T> {
    pub(crate) range: FullOffsetRange,
    pub(crate) value: T,
}

#[derive(Clone, Debug)]
pub(crate) struct FullOffsetRange {
    pub(crate) start: FullOffset,
    pub(crate) end: FullOffset,
}

#[derive(Clone, Debug)]
pub(crate) struct AnchorRangeMultimapSummary {
    start: FullOffset,
    end: FullOffset,
    min_start: FullOffset,
    max_end: FullOffset,
    count: usize,
}

impl Anchor {
    pub fn min() -> Self {
        Self {
            full_offset: FullOffset(0),
            bias: Bias::Left,
            version: Default::default(),
        }
    }

    pub fn max() -> Self {
        Self {
            full_offset: FullOffset::MAX,
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

    pub fn summary<'a, D, C>(&self, content: C) -> D
    where
        D: TextDimension<'a>,
        C: Into<Content<'a>>,
    {
        content.into().summary_for_anchor(self)
    }
}

impl<T> AnchorMap<T> {
    pub fn version(&self) -> &clock::Global {
        &self.version
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn iter<'a, D, C>(&'a self, content: C) -> impl Iterator<Item = (D, &'a T)> + 'a
    where
        D: 'a + TextDimension<'a>,
        C: 'a + Into<Content<'a>>,
    {
        let content = content.into();
        content
            .summaries_for_anchors(self)
            .map(move |(sum, value)| (sum, value))
    }
}

impl AnchorSet {
    pub fn version(&self) -> &clock::Global {
        &self.0.version
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter<'a, D, C>(&'a self, content: C) -> impl Iterator<Item = D> + 'a
    where
        D: 'a + TextDimension<'a>,
        C: 'a + Into<Content<'a>>,
    {
        self.0.iter(content).map(|(position, _)| position)
    }
}

impl<T> AnchorRangeMap<T> {
    pub fn version(&self) -> &clock::Global {
        &self.version
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn from_full_offset_ranges(
        version: clock::Global,
        entries: Vec<(Range<(FullOffset, Bias)>, T)>,
    ) -> Self {
        Self { version, entries }
    }

    pub fn ranges<'a, D>(
        &'a self,
        content: impl Into<Content<'a>> + 'a,
    ) -> impl Iterator<Item = (Range<D>, &'a T)> + 'a
    where
        D: 'a + TextDimension<'a>,
    {
        let content = content.into();
        content.summaries_for_anchor_ranges(self)
    }

    pub fn full_offset_ranges(&self) -> impl Iterator<Item = (Range<FullOffset>, &T)> {
        self.entries
            .iter()
            .map(|(range, value)| (range.start.0..range.end.0, value))
    }

    pub fn min_by_key<'a, C, D, F, K>(
        &self,
        content: C,
        mut extract_key: F,
    ) -> Option<(Range<D>, &T)>
    where
        C: Into<Content<'a>>,
        D: 'a + TextDimension<'a>,
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let content = content.into();
        self.entries
            .iter()
            .min_by_key(|(_, value)| extract_key(value))
            .map(|(range, value)| (self.resolve_range(range, &content), value))
    }

    pub fn max_by_key<'a, C, D, F, K>(
        &self,
        content: C,
        mut extract_key: F,
    ) -> Option<(Range<D>, &T)>
    where
        C: Into<Content<'a>>,
        D: 'a + TextDimension<'a>,
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let content = content.into();
        self.entries
            .iter()
            .max_by_key(|(_, value)| extract_key(value))
            .map(|(range, value)| (self.resolve_range(range, &content), value))
    }

    fn resolve_range<'a, D>(
        &self,
        range: &Range<(FullOffset, Bias)>,
        content: &Content<'a>,
    ) -> Range<D>
    where
        D: 'a + TextDimension<'a>,
    {
        let (start, start_bias) = range.start;
        let mut anchor = Anchor {
            full_offset: start,
            bias: start_bias,
            version: self.version.clone(),
        };
        let start = content.summary_for_anchor(&anchor);

        let (end, end_bias) = range.end;
        anchor.full_offset = end;
        anchor.bias = end_bias;
        let end = content.summary_for_anchor(&anchor);

        start..end
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

impl Debug for AnchorRangeSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_set();
        for (range, _) in &self.0.entries {
            f.entry(range);
        }
        f.finish()
    }
}

impl AnchorRangeSet {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn version(&self) -> &clock::Global {
        self.0.version()
    }

    pub fn ranges<'a, D, C>(&'a self, content: C) -> impl 'a + Iterator<Item = Range<Point>>
    where
        D: 'a + TextDimension<'a>,
        C: 'a + Into<Content<'a>>,
    {
        self.0.ranges(content).map(|(range, _)| range)
    }
}

impl<T: Clone> Default for AnchorRangeMultimap<T> {
    fn default() -> Self {
        Self {
            entries: Default::default(),
            version: Default::default(),
            start_bias: Bias::Left,
            end_bias: Bias::Left,
        }
    }
}

impl<T: Clone> AnchorRangeMultimap<T> {
    pub fn version(&self) -> &clock::Global {
        &self.version
    }

    pub fn intersecting_ranges<'a, I, O>(
        &'a self,
        range: Range<I>,
        content: Content<'a>,
        inclusive: bool,
    ) -> impl Iterator<Item = (usize, Range<O>, &T)> + 'a
    where
        I: ToOffset,
        O: FromAnchor,
    {
        let end_bias = if inclusive { Bias::Right } else { Bias::Left };
        let range = range.start.to_full_offset(&content, Bias::Left)
            ..range.end.to_full_offset(&content, end_bias);
        let mut cursor = self.entries.filter::<_, usize>(
            {
                let content = content.clone();
                let mut endpoint = Anchor {
                    full_offset: FullOffset(0),
                    bias: Bias::Right,
                    version: self.version.clone(),
                };
                move |summary: &AnchorRangeMultimapSummary| {
                    endpoint.full_offset = summary.max_end;
                    endpoint.bias = self.end_bias;
                    let max_end = endpoint.to_full_offset(&content, self.end_bias);
                    let start_cmp = range.start.cmp(&max_end);

                    endpoint.full_offset = summary.min_start;
                    endpoint.bias = self.start_bias;
                    let min_start = endpoint.to_full_offset(&content, self.start_bias);
                    let end_cmp = range.end.cmp(&min_start);

                    if inclusive {
                        start_cmp <= Ordering::Equal && end_cmp >= Ordering::Equal
                    } else {
                        start_cmp == Ordering::Less && end_cmp == Ordering::Greater
                    }
                }
            },
            &(),
        );

        std::iter::from_fn({
            let mut endpoint = Anchor {
                full_offset: FullOffset(0),
                bias: Bias::Left,
                version: self.version.clone(),
            };
            move || {
                if let Some(item) = cursor.item() {
                    let ix = *cursor.start();
                    endpoint.full_offset = item.range.start;
                    endpoint.bias = self.start_bias;
                    let start = O::from_anchor(&endpoint, &content);
                    endpoint.full_offset = item.range.end;
                    endpoint.bias = self.end_bias;
                    let end = O::from_anchor(&endpoint, &content);
                    let value = &item.value;
                    cursor.next(&());
                    Some((ix, start..end, value))
                } else {
                    None
                }
            }
        })
    }

    pub fn from_full_offset_ranges(
        version: clock::Global,
        start_bias: Bias,
        end_bias: Bias,
        entries: impl Iterator<Item = (Range<FullOffset>, T)>,
    ) -> Self {
        Self {
            version,
            start_bias,
            end_bias,
            entries: SumTree::from_iter(
                entries.map(|(range, value)| AnchorRangeMultimapEntry {
                    range: FullOffsetRange {
                        start: range.start,
                        end: range.end,
                    },
                    value,
                }),
                &(),
            ),
        }
    }

    pub fn full_offset_ranges(&self) -> impl Iterator<Item = (Range<FullOffset>, &T)> {
        self.entries
            .cursor::<()>()
            .map(|entry| (entry.range.start..entry.range.end, &entry.value))
    }

    pub fn filter<'a, O, F>(
        &'a self,
        content: Content<'a>,
        mut f: F,
    ) -> impl 'a + Iterator<Item = (usize, Range<O>, &T)>
    where
        O: FromAnchor,
        F: 'a + FnMut(&'a T) -> bool,
    {
        let mut endpoint = Anchor {
            full_offset: FullOffset(0),
            bias: Bias::Left,
            version: self.version.clone(),
        };
        self.entries
            .cursor::<()>()
            .enumerate()
            .filter_map(move |(ix, entry)| {
                if f(&entry.value) {
                    endpoint.full_offset = entry.range.start;
                    endpoint.bias = self.start_bias;
                    let start = O::from_anchor(&endpoint, &content);
                    endpoint.full_offset = entry.range.end;
                    endpoint.bias = self.end_bias;
                    let end = O::from_anchor(&endpoint, &content);
                    Some((ix, start..end, &entry.value))
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
            start: FullOffset(0),
            end: FullOffset::MAX,
            min_start: FullOffset::MAX,
            max_end: FullOffset(0),
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
            start: FullOffset(0),
            end: FullOffset::MAX,
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
