#![allow(unused)]
// TODO kb

use std::{
    cmp::{self, Reverse},
    ops::{Add, AddAssign, Range, Sub},
    sync::atomic::{self, AtomicUsize},
};

use crate::{Anchor, ExcerptId, InlayHintLocation, MultiBufferSnapshot, ToOffset, ToPoint};

use super::{
    suggestion_map::{
        SuggestionBufferRows, SuggestionChunks, SuggestionEdit, SuggestionOffset, SuggestionPoint,
        SuggestionSnapshot,
    },
    TextHighlights,
};
use collections::{BTreeMap, HashMap, HashSet};
use gpui::fonts::HighlightStyle;
use language::{Chunk, Edit, Point, Rope, TextSummary};
use parking_lot::Mutex;
use project::InlayHint;
use rand::Rng;
use sum_tree::{Bias, Cursor, SumTree};
use util::post_inc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InlayId(usize);

pub struct InlayMap {
    snapshot: Mutex<InlaySnapshot>,
    next_inlay_id: usize,
    pub(super) inlays: HashMap<InlayId, (InlayHintLocation, Inlay)>,
}

#[derive(Clone)]
pub struct InlaySnapshot {
    // TODO kb merge these two together?
    pub suggestion_snapshot: SuggestionSnapshot,
    transforms: SumTree<Transform>,
    pub version: usize,
}

#[derive(Clone, Debug)]
enum Transform {
    Isomorphic(TextSummary),
    Inlay(Inlay),
}

impl Transform {
    fn is_inlay(&self) -> bool {
        matches!(self, Self::Inlay(_))
    }
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self) -> Self::Summary {
        match self {
            Transform::Isomorphic(summary) => TransformSummary {
                input: summary.clone(),
                output: summary.clone(),
            },
            Transform::Inlay(inlay) => TransformSummary {
                input: TextSummary::default(),
                output: inlay.properties.text.summary(),
            },
        }
    }
}

#[derive(Clone, Debug, Default)]
struct TransformSummary {
    input: TextSummary,
    output: TextSummary,
}

impl sum_tree::Summary for TransformSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &()) {
        self.input += &other.input;
        self.output += &other.output;
    }
}

pub type InlayEdit = Edit<InlayOffset>;

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct InlayOffset(pub usize);

impl Add for InlayOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for InlayOffset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign for InlayOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InlayOffset {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.output.len;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for SuggestionOffset {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.input.len;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct InlayPoint(pub Point);

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InlayPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.output.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for SuggestionPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.input.lines;
    }
}

#[derive(Clone)]
pub struct InlayBufferRows<'a> {
    suggestion_rows: SuggestionBufferRows<'a>,
}

pub struct InlayChunks<'a> {
    transforms: Cursor<'a, Transform, (InlayOffset, SuggestionOffset)>,
    suggestion_chunks: SuggestionChunks<'a>,
    suggestion_chunk: Option<Chunk<'a>>,
    inlay_chunks: Option<text::Chunks<'a>>,
    output_offset: InlayOffset,
    max_output_offset: InlayOffset,
}

#[derive(Debug, Clone)]
pub struct Inlay {
    pub(super) id: InlayId,
    pub(super) properties: InlayProperties,
}

#[derive(Debug, Clone)]
pub struct InlayProperties {
    pub(super) position: Anchor,
    pub(super) text: Rope,
}

impl<'a> Iterator for InlayChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_offset == self.max_output_offset {
            return None;
        }

        let chunk = match self.transforms.item()? {
            Transform::Isomorphic(transform) => {
                let chunk = self
                    .suggestion_chunk
                    .get_or_insert_with(|| self.suggestion_chunks.next().unwrap());
                if chunk.text.is_empty() {
                    *chunk = self.suggestion_chunks.next().unwrap();
                }

                let (prefix, suffix) = chunk
                    .text
                    .split_at(cmp::min(transform.len, chunk.text.len()));
                chunk.text = suffix;
                self.output_offset.0 += prefix.len();
                Chunk {
                    text: prefix,
                    ..chunk.clone()
                }
            }
            Transform::Inlay(inlay) => {
                let inlay_chunks = self.inlay_chunks.get_or_insert_with(|| {
                    let start = self.output_offset - self.transforms.start().0;
                    let end = cmp::min(self.max_output_offset, self.transforms.end(&()).0)
                        - self.transforms.start().0;
                    inlay.properties.text.chunks_in_range(start.0..end.0)
                });

                let chunk = inlay_chunks.next().unwrap();
                self.output_offset.0 += chunk.len();
                Chunk {
                    text: chunk,
                    ..Default::default()
                }
            }
        };

        if self.output_offset == self.transforms.end(&()).0 {
            self.transforms.next(&());
        }

        Some(chunk)
    }
}

impl<'a> Iterator for InlayBufferRows<'a> {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.suggestion_rows.next()
    }
}

impl InlayPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(Point::new(row, column))
    }

    pub fn row(self) -> u32 {
        self.0.row
    }

    pub fn column(self) -> u32 {
        self.0.column
    }
}

impl InlayMap {
    pub fn new(suggestion_snapshot: SuggestionSnapshot) -> (Self, InlaySnapshot) {
        let snapshot = InlaySnapshot {
            suggestion_snapshot: suggestion_snapshot.clone(),
            version: 0,
            transforms: SumTree::from_item(
                Transform::Isomorphic(suggestion_snapshot.text_summary()),
                &(),
            ),
        };

        (
            Self {
                snapshot: Mutex::new(snapshot.clone()),
                next_inlay_id: 0,
                inlays: HashMap::default(),
            },
            snapshot,
        )
    }

    pub fn sync(
        &self,
        suggestion_snapshot: SuggestionSnapshot,
        suggestion_edits: Vec<SuggestionEdit>,
    ) -> (InlaySnapshot, Vec<InlayEdit>) {
        let mut snapshot = self.snapshot.lock();

        if snapshot.suggestion_snapshot.version != suggestion_snapshot.version {
            snapshot.version += 1;
        }

        let mut new_transforms = SumTree::new();
        let mut cursor = snapshot.transforms.cursor::<SuggestionOffset>();
        let mut suggestion_edits = suggestion_edits.iter().peekable();

        while let Some(suggestion_edit) = suggestion_edits.next() {
            if suggestion_edit.old.start >= *cursor.start() {
                new_transforms.push_tree(
                    cursor.slice(&suggestion_edit.old.start, Bias::Right, &()),
                    &(),
                );
            }

            if suggestion_edit.old.end > cursor.end(&()) {
                cursor.seek_forward(&suggestion_edit.old.end, Bias::Right, &());
            }

            let transform_start = SuggestionOffset(new_transforms.summary().input.len);
            let mut transform_end = suggestion_edit.new.end;
            if suggestion_edits
                .peek()
                .map_or(true, |edit| edit.old.start > cursor.end(&()))
            {
                transform_end += cursor.end(&()) - suggestion_edit.old.end;
                cursor.next(&());
            }
            push_isomorphic(
                &mut new_transforms,
                suggestion_snapshot.text_summary_for_range(
                    suggestion_snapshot.to_point(transform_start)
                        ..suggestion_snapshot.to_point(transform_end),
                ),
            );
        }

        new_transforms.push_tree(cursor.suffix(&()), &());
        drop(cursor);

        snapshot.suggestion_snapshot = suggestion_snapshot;
        snapshot.transforms = new_transforms;

        (snapshot.clone(), Default::default())
    }

    pub fn splice(
        &mut self,
        to_remove: HashSet<InlayId>,
        to_insert: Vec<(InlayHintLocation, InlayProperties)>,
    ) -> (InlaySnapshot, Vec<InlayEdit>, Vec<InlayId>) {
        let mut snapshot = self.snapshot.lock();

        let mut inlays = BTreeMap::new();
        let mut new_ids = Vec::new();
        for (location, properties) in to_insert {
            let inlay = Inlay {
                id: InlayId(post_inc(&mut self.next_inlay_id)),
                properties,
            };
            self.inlays.insert(inlay.id, (location, inlay.clone()));
            new_ids.push(inlay.id);

            let buffer_point = inlay
                .properties
                .position
                .to_point(snapshot.buffer_snapshot());
            let fold_point = snapshot
                .suggestion_snapshot
                .fold_snapshot
                .to_fold_point(buffer_point, Bias::Left);
            let suggestion_point = snapshot.suggestion_snapshot.to_suggestion_point(fold_point);
            let inlay_point = snapshot.to_inlay_point(suggestion_point);

            inlays.insert((inlay_point, Reverse(inlay.id)), Some(inlay));
        }

        for inlay_id in to_remove {
            if let Some((_, inlay)) = self.inlays.remove(&inlay_id) {
                let buffer_point = inlay
                    .properties
                    .position
                    .to_point(snapshot.buffer_snapshot());
                let fold_point = snapshot
                    .suggestion_snapshot
                    .fold_snapshot
                    .to_fold_point(buffer_point, Bias::Left);
                let suggestion_point = snapshot.suggestion_snapshot.to_suggestion_point(fold_point);
                let inlay_point = snapshot.to_inlay_point(suggestion_point);
                inlays.insert((inlay_point, Reverse(inlay.id)), None);
            }
        }

        let mut new_transforms = SumTree::new();
        let mut cursor = snapshot
            .transforms
            .cursor::<(InlayPoint, SuggestionPoint)>();
        for ((inlay_point, inlay_id), inlay) in inlays {
            new_transforms.push_tree(cursor.slice(&inlay_point, Bias::Right, &()), &());
            while let Some(transform) = cursor.item() {
                match transform {
                    Transform::Isomorphic(_) => break,
                    Transform::Inlay(inlay) => {
                        if inlay.id > inlay_id.0 {
                            new_transforms.push(transform.clone(), &());
                            cursor.next(&());
                        } else {
                            if inlay.id == inlay_id.0 {
                                cursor.next(&());
                            }
                            break;
                        }
                    }
                }
            }

            if let Some(inlay) = inlay {
                if let Some(Transform::Isomorphic(transform)) = cursor.item() {
                    let prefix = inlay_point.0 - cursor.start().0 .0;
                    if !prefix.is_zero() {
                        let prefix_suggestion_start = cursor.start().1;
                        let prefix_suggestion_end = SuggestionPoint(cursor.start().1 .0 + prefix);
                        new_transforms.push(
                            Transform::Isomorphic(
                                snapshot.suggestion_snapshot.text_summary_for_range(
                                    prefix_suggestion_start..prefix_suggestion_end,
                                ),
                            ),
                            &(),
                        );
                    }

                    new_transforms.push(Transform::Inlay(inlay), &());

                    let suffix_suggestion_start = SuggestionPoint(cursor.start().1 .0 + prefix);
                    let suffix_suggestion_end = cursor.end(&()).1;
                    new_transforms.push(
                        Transform::Isomorphic(snapshot.suggestion_snapshot.text_summary_for_range(
                            suffix_suggestion_start..suffix_suggestion_end,
                        )),
                        &(),
                    );

                    cursor.next(&());
                } else {
                    new_transforms.push(Transform::Inlay(inlay), &());
                }
            }
        }

        new_transforms.push_tree(cursor.suffix(&()), &());
        drop(cursor);
        snapshot.transforms = new_transforms;
        snapshot.version += 1;

        (snapshot.clone(), Vec::new(), new_ids)
    }
}

impl InlaySnapshot {
    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        self.suggestion_snapshot.buffer_snapshot()
    }

    pub fn to_point(&self, offset: InlayOffset) -> InlayPoint {
        let mut cursor = self
            .transforms
            .cursor::<(InlayOffset, (InlayPoint, SuggestionOffset))>();
        cursor.seek(&offset, Bias::Right, &());
        let overshoot = offset.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(transform)) => {
                let suggestion_offset_start = cursor.start().1 .1;
                let suggestion_offset_end = SuggestionOffset(suggestion_offset_start.0 + overshoot);
                let suggestion_start = self.suggestion_snapshot.to_point(suggestion_offset_start);
                let suggestion_end = self.suggestion_snapshot.to_point(suggestion_offset_end);
                InlayPoint(cursor.start().1 .0 .0 + (suggestion_end.0 - suggestion_start.0))
            }
            Some(Transform::Inlay(inlay)) => {
                let overshoot = inlay.properties.text.offset_to_point(overshoot);
                InlayPoint(cursor.start().1 .0 .0 + overshoot)
            }
            None => self.max_point(),
        }
    }

    pub fn len(&self) -> InlayOffset {
        InlayOffset(self.transforms.summary().output.len)
    }

    pub fn max_point(&self) -> InlayPoint {
        InlayPoint(self.transforms.summary().output.lines)
    }

    pub fn to_offset(&self, point: InlayPoint) -> InlayOffset {
        let mut cursor = self
            .transforms
            .cursor::<(InlayPoint, (InlayOffset, SuggestionPoint))>();
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(transform)) => {
                let suggestion_point_start = cursor.start().1 .1;
                let suggestion_point_end = SuggestionPoint(suggestion_point_start.0 + overshoot);
                let suggestion_start = self.suggestion_snapshot.to_offset(suggestion_point_start);
                let suggestion_end = self.suggestion_snapshot.to_offset(suggestion_point_end);
                InlayOffset(cursor.start().1 .0 .0 + (suggestion_end.0 - suggestion_start.0))
            }
            Some(Transform::Inlay(inlay)) => {
                let overshoot = inlay.properties.text.point_to_offset(overshoot);
                InlayOffset(cursor.start().1 .0 .0 + overshoot)
            }
            None => self.len(),
        }
    }

    pub fn chars_at(&self, start: InlayPoint) -> impl '_ + Iterator<Item = char> {
        self.chunks(self.to_offset(start)..self.len(), false, None, None)
            .flat_map(|chunk| chunk.text.chars())
    }

    pub fn to_suggestion_point(&self, point: InlayPoint) -> SuggestionPoint {
        let mut cursor = self.transforms.cursor::<(InlayPoint, SuggestionPoint)>();
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(transform)) => {
                SuggestionPoint(cursor.start().1 .0 + overshoot)
            }
            Some(Transform::Inlay(inlay)) => cursor.start().1,
            None => self.suggestion_snapshot.max_point(),
        }
    }

    pub fn to_suggestion_offset(&self, offset: InlayOffset) -> SuggestionOffset {
        let mut cursor = self.transforms.cursor::<(InlayOffset, SuggestionOffset)>();
        cursor.seek(&offset, Bias::Right, &());
        match cursor.item() {
            Some(Transform::Isomorphic(transform)) => {
                let overshoot = offset - cursor.start().0;
                cursor.start().1 + SuggestionOffset(overshoot.0)
            }
            Some(Transform::Inlay(inlay)) => cursor.start().1,
            None => self.suggestion_snapshot.len(),
        }
    }

    pub fn to_inlay_point(&self, point: SuggestionPoint) -> InlayPoint {
        let mut cursor = self.transforms.cursor::<(SuggestionPoint, InlayPoint)>();
        // TODO kb is the bias right? should we have an external one instead?
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(transform)) => InlayPoint(cursor.start().1 .0 + overshoot),
            Some(Transform::Inlay(inlay)) => cursor.start().1,
            None => self.max_point(),
        }
    }

    pub fn clip_point(&self, point: InlayPoint, bias: Bias) -> InlayPoint {
        let mut cursor = self.transforms.cursor::<InlayPoint>();
        cursor.seek(&point, bias, &());
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => return point,
            Some(Transform::Inlay(_)) => {}
            None => cursor.prev(&()),
        }

        while cursor
            .item()
            .map_or(false, |transform| transform.is_inlay())
        {
            match bias {
                Bias::Left => cursor.prev(&()),
                Bias::Right => cursor.next(&()),
            }
        }

        match bias {
            Bias::Left => cursor.end(&()),
            Bias::Right => *cursor.start(),
        }
    }

    pub fn text_summary_for_range(&self, range: Range<InlayPoint>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self.transforms.cursor::<(InlayPoint, SuggestionPoint)>();
        cursor.seek(&range.start, Bias::Right, &());

        let overshoot = range.start.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(transform)) => {
                let suggestion_start = cursor.start().1 .0;
                let suffix_start = SuggestionPoint(suggestion_start + overshoot);
                let suffix_end = SuggestionPoint(
                    suggestion_start
                        + (cmp::min(cursor.end(&()).0, range.end).0 - cursor.start().0 .0),
                );
                summary = self
                    .suggestion_snapshot
                    .text_summary_for_range(suffix_start..suffix_end);
                cursor.next(&());
            }
            Some(Transform::Inlay(inlay)) => {
                let text = &inlay.properties.text;
                let suffix_start = text.point_to_offset(overshoot);
                let suffix_end = text.point_to_offset(
                    cmp::min(cursor.end(&()).0, range.end).0 - cursor.start().0 .0,
                );
                summary = text.cursor(suffix_start).summary(suffix_end);
                cursor.next(&());
            }
            None => {}
        }

        if range.end > cursor.start().0 {
            summary += cursor
                .summary::<_, TransformSummary>(&range.end, Bias::Right, &())
                .output;

            let overshoot = range.end.0 - cursor.start().0 .0;
            match cursor.item() {
                Some(Transform::Isomorphic(transform)) => {
                    let prefix_start = cursor.start().1;
                    let prefix_end = SuggestionPoint(prefix_start.0 + overshoot);
                    summary += self
                        .suggestion_snapshot
                        .text_summary_for_range(prefix_start..prefix_end);
                }
                Some(Transform::Inlay(inlay)) => {
                    let text = &inlay.properties.text;
                    let prefix_end = text.point_to_offset(overshoot);
                    summary += text.cursor(0).summary::<TextSummary>(prefix_end);
                }
                None => {}
            }
        }

        summary
    }

    pub fn buffer_rows<'a>(&'a self, row: u32) -> InlayBufferRows<'a> {
        InlayBufferRows {
            suggestion_rows: self.suggestion_snapshot.buffer_rows(row),
        }
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let line_start = self.to_offset(InlayPoint::new(row, 0)).0;
        let line_end = if row >= self.max_point().row() {
            self.len().0
        } else {
            self.to_offset(InlayPoint::new(row + 1, 0)).0 - 1
        };
        (line_end - line_start) as u32
    }

    pub fn chunks<'a>(
        &'a self,
        range: Range<InlayOffset>,
        language_aware: bool,
        text_highlights: Option<&'a TextHighlights>,
        suggestion_highlight: Option<HighlightStyle>,
    ) -> InlayChunks<'a> {
        let mut cursor = self.transforms.cursor::<(InlayOffset, SuggestionOffset)>();
        cursor.seek(&range.start, Bias::Right, &());

        let suggestion_range =
            self.to_suggestion_offset(range.start)..self.to_suggestion_offset(range.end);
        let suggestion_chunks = self.suggestion_snapshot.chunks(
            suggestion_range,
            language_aware,
            text_highlights,
            suggestion_highlight,
        );

        InlayChunks {
            transforms: cursor,
            suggestion_chunks,
            inlay_chunks: None,
            suggestion_chunk: None,
            output_offset: range.start,
            max_output_offset: range.end,
        }
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(Default::default()..self.len(), false, None, None)
            .map(|chunk| chunk.text)
            .collect()
    }
}

fn push_isomorphic(sum_tree: &mut SumTree<Transform>, summary: TextSummary) {
    let mut summary = Some(summary);
    sum_tree.update_last(
        |transform| {
            if let Transform::Isomorphic(transform) = transform {
                *transform += summary.take().unwrap();
            }
        },
        &(),
    );

    if let Some(summary) = summary {
        sum_tree.push(Transform::Isomorphic(summary), &());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::{fold_map::FoldMap, suggestion_map::SuggestionMap},
        MultiBuffer,
    };
    use gpui::AppContext;

    #[gpui::test]
    fn test_basic_inlays(cx: &mut AppContext) {
        let buffer = MultiBuffer::build_simple("abcdefghi", cx);
        let buffer_edits = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (mut fold_map, fold_snapshot) = FoldMap::new(buffer.read(cx).snapshot(cx));
        let (suggestion_map, suggestion_snapshot) = SuggestionMap::new(fold_snapshot.clone());
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(suggestion_snapshot.clone());
        assert_eq!(inlay_snapshot.text(), "abcdefghi");

        let (inlay_snapshot, _, inlay_ids) = inlay_map.splice(
            HashSet::default(),
            vec![(
                InlayHintLocation {
                    buffer_id: 0,
                    excerpt_id: ExcerptId::default(),
                },
                InlayProperties {
                    position: buffer.read(cx).read(cx).anchor_before(3),
                    text: "|123|".into(),
                },
            )],
        );
        assert_eq!(inlay_snapshot.text(), "abc|123|defghi");

        buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "XYZ")], None, cx));
        let (fold_snapshot, fold_edits) = fold_map.read(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        let (suggestion_snapshot, suggestion_edits) =
            suggestion_map.sync(fold_snapshot.clone(), fold_edits);
        let (inlay_snapshot, _) = inlay_map.sync(suggestion_snapshot.clone(), suggestion_edits);
        assert_eq!(inlay_snapshot.text(), "XYZabc|123|defghi");

        //////// case: folding and unfolding the text should hine and then return the hint back
        let (mut fold_map_writer, _, _) = fold_map.write(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        let (fold_snapshot, fold_edits) = fold_map_writer.fold([4..8]);
        let (suggestion_snapshot, suggestion_edits) =
            suggestion_map.sync(fold_snapshot.clone(), fold_edits);
        let (inlay_snapshot, _) = inlay_map.sync(suggestion_snapshot.clone(), suggestion_edits);
        assert_eq!(inlay_snapshot.text(), "XYZa⋯fghi");

        let (fold_snapshot, fold_edits) = fold_map_writer.unfold([4..8], false);
        let (suggestion_snapshot, suggestion_edits) =
            suggestion_map.sync(fold_snapshot.clone(), fold_edits);
        let (inlay_snapshot, _) = inlay_map.sync(suggestion_snapshot.clone(), suggestion_edits);
        assert_eq!(inlay_snapshot.text(), "XYZabc|123|defghi");

        ////////// case: replacing the anchor that got the hint: it should disappear
        buffer.update(cx, |buffer, cx| buffer.edit([(2..3, "C")], None, cx));
        let (fold_snapshot, fold_edits) = fold_map.read(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        let (suggestion_snapshot, suggestion_edits) =
            suggestion_map.sync(fold_snapshot.clone(), fold_edits);
        let (inlay_snapshot, _) = inlay_map.sync(suggestion_snapshot.clone(), suggestion_edits);
        assert_eq!(inlay_snapshot.text(), "XYZabCdefghi");
    }
}
