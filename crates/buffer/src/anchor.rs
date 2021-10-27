use super::{Buffer, Content, FromAnchor, Point, ToOffset};
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
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct AnchorRangeMultimapSummary {
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
                    full_offset: 0,
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
                full_offset: 0,
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
