use super::{location::*, ExcerptSummary, MultiBufferSnapshot, ToOffset, ToPoint};
use anyhow::{anyhow, Result};
use smallvec::SmallVec;
use std::{
    cmp::Ordering,
    ops::{Range, Sub},
};
use sum_tree::Bias;
use text::{rope::TextDimension, AnchorRangeExt as _, Point};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Anchor {
    excerpt_id: ExcerptId,
    text_anchor: text::Anchor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorRangeMap<T> {
    entries: SmallVec<[(ExcerptId, text::AnchorRangeMap<T>); 1]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorRangeSet(AnchorRangeMap<()>);

impl Anchor {
    pub fn min() -> Self {
        Self {
            excerpt_id: ExcerptId::min(),
            text_anchor: text::Anchor::min(),
        }
    }

    pub fn max() -> Self {
        Self {
            excerpt_id: ExcerptId::max(),
            text_anchor: text::Anchor::max(),
        }
    }

    pub fn cmp<'a>(&self, other: &Anchor, snapshot: &MultiBufferSnapshot) -> Result<Ordering> {
        let excerpt_id_cmp = self.excerpt_id.cmp(&other.excerpt_id);
        if excerpt_id_cmp.is_eq() {
            self.text_anchor.cmp(
                &other.text_anchor,
                snapshot
                    .buffer_snapshot_for_excerpt(&self.excerpt_id)
                    .ok_or_else(|| anyhow!("excerpt {:?} not found", self.excerpt_id))?,
            )
        } else {
            return Ok(excerpt_id_cmp);
        }
    }

    pub fn bias_left(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        if self.text_anchor.bias != Bias::Left {
            if let Some(buffer_snapshot) = snapshot.buffer_snapshot_for_excerpt(&self.excerpt_id) {
                return Self {
                    excerpt_id: self.excerpt_id.clone(),
                    text_anchor: self.text_anchor.bias_left(buffer_snapshot),
                };
            }
        }
        self.clone()
    }

    pub fn bias_right(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        if self.text_anchor.bias != Bias::Right {
            if let Some(buffer_snapshot) = snapshot.buffer_snapshot_for_excerpt(&self.excerpt_id) {
                return Self {
                    excerpt_id: self.excerpt_id.clone(),
                    text_anchor: self.text_anchor.bias_right(buffer_snapshot),
                };
            }
        }
        self.clone()
    }

    pub fn summary<'a, D>(&self, snapshot: &'a MultiBufferSnapshot) -> D
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        let mut cursor = snapshot.excerpts.cursor::<ExcerptSummary>();
        cursor.seek(&self.excerpt_id, Bias::Left, &());
        if let Some(excerpt) = cursor.item() {
            if excerpt.id == self.excerpt_id {
                let mut excerpt_start = D::from_text_summary(&cursor.start().text);
                excerpt_start.add_summary(&excerpt.header_summary(), &());
                let excerpt_buffer_start = excerpt.range.start.summary::<D>(&excerpt.buffer);
                let buffer_point = self.text_anchor.summary::<D>(&excerpt.buffer);
                if buffer_point > excerpt_buffer_start {
                    excerpt_start.add_assign(&(buffer_point - excerpt_buffer_start));
                }
                return excerpt_start;
            }
        }
        D::from_text_summary(&cursor.start().text)
    }
}

impl<T> AnchorRangeMap<T> {
    pub fn len(&self) -> usize {
        self.entries
            .iter()
            .map(|(_, text_map)| text_map.len())
            .sum()
    }

    pub fn ranges<'a, D>(
        &'a self,
        snapshot: &'a MultiBufferSnapshot,
    ) -> impl Iterator<Item = (Range<D>, &'a T)> + 'a
    where
        D: TextDimension + Clone,
    {
        let mut cursor = snapshot.excerpts.cursor::<ExcerptSummary>();
        self.entries
            .iter()
            .filter_map(move |(excerpt_id, text_map)| {
                cursor.seek_forward(excerpt_id, Bias::Left, &());
                if let Some(excerpt) = cursor.item() {
                    if excerpt.id == *excerpt_id {
                        let mut excerpt_start = D::from_text_summary(&cursor.start().text);
                        excerpt_start.add_summary(&excerpt.header_summary(), &());
                        return Some(text_map.ranges::<D>(&excerpt.buffer).map(
                            move |(range, value)| {
                                let mut full_range = excerpt_start.clone()..excerpt_start.clone();
                                full_range.start.add_assign(&range.start);
                                full_range.end.add_assign(&range.end);
                                (full_range, value)
                            },
                        ));
                    }
                }
                None
            })
            .flatten()
    }

    pub fn intersecting_ranges<'a, D, I>(
        &'a self,
        range: Range<(I, Bias)>,
        snapshot: &'a MultiBufferSnapshot,
    ) -> impl Iterator<Item = (Range<D>, &'a T)> + 'a
    where
        D: TextDimension,
        I: ToOffset,
    {
        let start_bias = range.start.1;
        let end_bias = range.end.1;
        let start_offset = range.start.0.to_offset(snapshot);
        let end_offset = range.end.0.to_offset(snapshot);

        let mut cursor = snapshot.excerpts.cursor::<ExcerptSummary>();
        cursor.seek(&start_offset, start_bias, &());
        let start_excerpt_id = &cursor.start().excerpt_id;
        let start_ix = match self
            .entries
            .binary_search_by_key(&start_excerpt_id, |e| &e.0)
        {
            Ok(ix) | Err(ix) => ix,
        };

        let mut entry_ranges = None;
        let mut entries = self.entries[start_ix..].iter();
        std::iter::from_fn(move || loop {
            match &mut entry_ranges {
                None => {
                    let (excerpt_id, text_map) = entries.next()?;
                    cursor.seek(excerpt_id, Bias::Left, &());
                    if cursor.start().text.bytes >= end_offset {
                        return None;
                    }

                    if let Some(excerpt) = cursor.item() {
                        if excerpt.id == *excerpt_id {
                            let mut excerpt_start = D::from_text_summary(&cursor.start().text);
                            excerpt_start.add_summary(&excerpt.header_summary(), &());

                            let excerpt_start_offset = cursor.start().text.bytes;
                            let excerpt_end_offset = cursor.end(&()).text.bytes;
                            let excerpt_buffer_range = excerpt.range.to_offset(&excerpt.buffer);

                            let start;
                            if start_offset >= excerpt_start_offset {
                                start = (
                                    excerpt_buffer_range.start + start_offset
                                        - excerpt_start_offset,
                                    start_bias,
                                );
                            } else {
                                start = (excerpt_buffer_range.start, Bias::Left);
                            }

                            let end;
                            if end_offset <= excerpt_end_offset {
                                end = (
                                    excerpt_buffer_range.start + end_offset - excerpt_start_offset,
                                    end_bias,
                                );
                            } else {
                                end = (excerpt_buffer_range.end, Bias::Right);
                            }

                            entry_ranges = Some(
                                text_map
                                    .intersecting_ranges(start..end, &excerpt.buffer)
                                    .map(move |(range, value)| {
                                        let mut full_range =
                                            excerpt_start.clone()..excerpt_start.clone();
                                        full_range.start.add_assign(&range.start);
                                        full_range.end.add_assign(&range.end);
                                        (full_range, value)
                                    }),
                            );
                        }
                    }
                }
                Some(ranges) => {
                    if let Some(item) = ranges.next() {
                        return Some(item);
                    } else {
                        entry_ranges.take();
                    }
                }
            }
        })
    }

    pub fn min_by_key<'a, D, F, K>(
        &self,
        snapshot: &'a MultiBufferSnapshot,
        extract_key: F,
    ) -> Option<(Range<D>, &T)>
    where
        D: TextDimension,
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.min_or_max_by_key(snapshot, Ordering::Less, extract_key)
    }

    pub fn max_by_key<'a, D, F, K>(
        &self,
        snapshot: &'a MultiBufferSnapshot,
        extract_key: F,
    ) -> Option<(Range<D>, &T)>
    where
        D: TextDimension,
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.min_or_max_by_key(snapshot, Ordering::Greater, extract_key)
    }

    fn min_or_max_by_key<'a, D, F, K>(
        &self,
        snapshot: &'a MultiBufferSnapshot,
        target_ordering: Ordering,
        mut extract_key: F,
    ) -> Option<(Range<D>, &T)>
    where
        D: TextDimension,
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut cursor = snapshot.excerpts.cursor::<ExcerptSummary>();
        let mut max = None;
        for (excerpt_id, text_map) in &self.entries {
            cursor.seek(excerpt_id, Bias::Left, &());
            if let Some(excerpt) = cursor.item() {
                if excerpt.id == *excerpt_id {
                    if let Some((range, value)) =
                        text_map.max_by_key(&excerpt.buffer, &mut extract_key)
                    {
                        if max.as_ref().map_or(true, |(_, max_value)| {
                            extract_key(value).cmp(&extract_key(*max_value)) == target_ordering
                        }) {
                            let mut excerpt_start = D::from_text_summary(&cursor.start().text);
                            excerpt_start.add_summary(&excerpt.header_summary(), &());
                            let mut full_range = excerpt_start.clone()..excerpt_start.clone();
                            full_range.start.add_assign(&range.start);
                            full_range.end.add_assign(&range.end);
                            max = Some((full_range, value));
                        }
                    }
                }
            }
        }
        max
    }
}

impl AnchorRangeSet {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn ranges<'a, D>(
        &'a self,
        content: &'a MultiBufferSnapshot,
    ) -> impl 'a + Iterator<Item = Range<Point>>
    where
        D: TextDimension,
    {
        self.0.ranges(content).map(|(range, _)| range)
    }
}

impl ToOffset for Anchor {
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize {
        self.summary(snapshot)
    }
}

impl ToPoint for Anchor {
    fn to_point<'a>(&self, snapshot: &MultiBufferSnapshot) -> Point {
        self.summary(snapshot)
    }
}

pub trait AnchorRangeExt {
    fn cmp(&self, b: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> Result<Ordering>;
    fn to_offset(&self, content: &MultiBufferSnapshot) -> Range<usize>;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> Result<Ordering> {
        Ok(match self.start.cmp(&other.start, buffer)? {
            Ordering::Equal => other.end.cmp(&self.end, buffer)?,
            ord @ _ => ord,
        })
    }

    fn to_offset(&self, content: &MultiBufferSnapshot) -> Range<usize> {
        self.start.to_offset(&content)..self.end.to_offset(&content)
    }
}
