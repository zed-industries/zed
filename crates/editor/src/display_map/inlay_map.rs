use super::{
    fold_map::{FoldBufferRows, FoldChunks, FoldEdit, FoldOffset, FoldPoint, FoldSnapshot},
    TextHighlights,
};
use crate::{
    inlay_cache::{Inlay, InlayId, InlayProperties},
    MultiBufferSnapshot, ToPoint,
};
use collections::{BTreeSet, HashMap};
use gpui::fonts::HighlightStyle;
use language::{Chunk, Edit, Point, Rope, TextSummary};
use parking_lot::Mutex;
use std::{
    cmp,
    ops::{Add, AddAssign, Range, Sub},
};
use sum_tree::{Bias, Cursor, SumTree};
use text::Patch;

pub struct InlayMap {
    snapshot: Mutex<InlaySnapshot>,
    inlays_by_id: HashMap<InlayId, Inlay>,
    inlays: Vec<Inlay>,
}

#[derive(Clone)]
pub struct InlaySnapshot {
    // TODO kb merge these two together
    pub fold_snapshot: FoldSnapshot,
    transforms: SumTree<Transform>,
    pub version: usize,
}

#[derive(Clone, Debug)]
enum Transform {
    Isomorphic(TextSummary),
    Inlay(Inlay),
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
                output: inlay.text.summary(),
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

impl<'a> sum_tree::Dimension<'a, TransformSummary> for FoldOffset {
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

impl<'a> sum_tree::Dimension<'a, TransformSummary> for FoldPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.input.lines;
    }
}

#[derive(Clone)]
pub struct InlayBufferRows<'a> {
    transforms: Cursor<'a, Transform, (InlayPoint, FoldPoint)>,
    fold_rows: FoldBufferRows<'a>,
    inlay_row: u32,
}

pub struct InlayChunks<'a> {
    transforms: Cursor<'a, Transform, (InlayOffset, FoldOffset)>,
    fold_chunks: FoldChunks<'a>,
    fold_chunk: Option<Chunk<'a>>,
    inlay_chunks: Option<text::Chunks<'a>>,
    output_offset: InlayOffset,
    max_output_offset: InlayOffset,
    highlight_style: Option<HighlightStyle>,
}

impl<'a> Iterator for InlayChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_offset == self.max_output_offset {
            return None;
        }

        let chunk = match self.transforms.item()? {
            Transform::Isomorphic(_) => {
                let chunk = self
                    .fold_chunk
                    .get_or_insert_with(|| self.fold_chunks.next().unwrap());
                if chunk.text.is_empty() {
                    *chunk = self.fold_chunks.next().unwrap();
                }

                let (prefix, suffix) = chunk.text.split_at(cmp::min(
                    self.transforms.end(&()).0 .0 - self.output_offset.0,
                    chunk.text.len(),
                ));

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
                    inlay.text.chunks_in_range(start.0..end.0)
                });

                let chunk = inlay_chunks.next().unwrap();
                self.output_offset.0 += chunk.len();
                Chunk {
                    text: chunk,
                    highlight_style: self.highlight_style,
                    ..Default::default()
                }
            }
        };

        if self.output_offset == self.transforms.end(&()).0 {
            self.inlay_chunks = None;
            self.transforms.next(&());
        }

        Some(chunk)
    }
}

impl<'a> Iterator for InlayBufferRows<'a> {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let buffer_row = if self.inlay_row == 0 {
            self.fold_rows.next().unwrap()
        } else {
            match self.transforms.item()? {
                Transform::Inlay(_) => None,
                Transform::Isomorphic(_) => self.fold_rows.next().unwrap(),
            }
        };

        self.inlay_row += 1;
        self.transforms
            .seek_forward(&InlayPoint::new(self.inlay_row, 0), Bias::Left, &());

        Some(buffer_row)
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
    pub fn new(fold_snapshot: FoldSnapshot) -> (Self, InlaySnapshot) {
        let snapshot = InlaySnapshot {
            fold_snapshot: fold_snapshot.clone(),
            version: 0,
            transforms: SumTree::from_item(
                Transform::Isomorphic(fold_snapshot.text_summary()),
                &(),
            ),
        };

        (
            Self {
                snapshot: Mutex::new(snapshot.clone()),
                inlays_by_id: Default::default(),
                inlays: Default::default(),
            },
            snapshot,
        )
    }

    pub fn sync(
        &mut self,
        fold_snapshot: FoldSnapshot,
        mut fold_edits: Vec<FoldEdit>,
    ) -> (InlaySnapshot, Vec<InlayEdit>) {
        let mut snapshot = self.snapshot.lock();

        let mut new_snapshot = snapshot.clone();
        if new_snapshot.fold_snapshot.version != fold_snapshot.version {
            new_snapshot.version += 1;
        }

        if fold_snapshot
            .buffer_snapshot()
            .trailing_excerpt_update_count()
            != snapshot
                .fold_snapshot
                .buffer_snapshot()
                .trailing_excerpt_update_count()
        {
            if fold_edits.is_empty() {
                fold_edits.push(Edit {
                    old: snapshot.fold_snapshot.len()..snapshot.fold_snapshot.len(),
                    new: fold_snapshot.len()..fold_snapshot.len(),
                });
            }
        }

        let mut inlay_edits = Patch::default();
        let mut new_transforms = SumTree::new();
        let mut cursor = snapshot.transforms.cursor::<(FoldOffset, InlayOffset)>();
        let mut fold_edits_iter = fold_edits.iter().peekable();
        while let Some(fold_edit) = fold_edits_iter.next() {
            new_transforms.push_tree(cursor.slice(&fold_edit.old.start, Bias::Left, &()), &());
            if let Some(Transform::Isomorphic(transform)) = cursor.item() {
                if cursor.end(&()).0 == fold_edit.old.start {
                    new_transforms.push(Transform::Isomorphic(transform.clone()), &());
                    cursor.next(&());
                }
            }

            // Remove all the inlays and transforms contained by the edit.
            let old_start =
                cursor.start().1 + InlayOffset(fold_edit.old.start.0 - cursor.start().0 .0);
            cursor.seek(&fold_edit.old.end, Bias::Right, &());
            let old_end = cursor.start().1 + InlayOffset(fold_edit.old.end.0 - cursor.start().0 .0);

            // Push the unchanged prefix.
            let prefix_start = FoldOffset(new_transforms.summary().input.len);
            let prefix_end = fold_edit.new.start;
            push_isomorphic(
                &mut new_transforms,
                fold_snapshot.text_summary_for_range(
                    prefix_start.to_point(&fold_snapshot)..prefix_end.to_point(&fold_snapshot),
                ),
            );
            let new_start = InlayOffset(new_transforms.summary().output.len);

            let start_point = fold_edit
                .new
                .start
                .to_point(&fold_snapshot)
                .to_buffer_point(&fold_snapshot);
            let start_ix = match self.inlays.binary_search_by(|probe| {
                probe
                    .position
                    .to_point(&fold_snapshot.buffer_snapshot())
                    .cmp(&start_point)
                    .then(std::cmp::Ordering::Greater)
            }) {
                Ok(ix) | Err(ix) => ix,
            };

            for inlay in &self.inlays[start_ix..] {
                let buffer_point = inlay.position.to_point(fold_snapshot.buffer_snapshot());
                let fold_point = fold_snapshot.to_fold_point(buffer_point, Bias::Left);
                let fold_offset = fold_point.to_offset(&fold_snapshot);
                if fold_offset > fold_edit.new.end {
                    break;
                }

                let prefix_start = FoldOffset(new_transforms.summary().input.len);
                let prefix_end = fold_offset;
                push_isomorphic(
                    &mut new_transforms,
                    fold_snapshot.text_summary_for_range(
                        prefix_start.to_point(&fold_snapshot)..prefix_end.to_point(&fold_snapshot),
                    ),
                );

                if inlay.position.is_valid(fold_snapshot.buffer_snapshot()) {
                    new_transforms.push(Transform::Inlay(inlay.clone()), &());
                }
            }

            // Apply the rest of the edit.
            let transform_start = FoldOffset(new_transforms.summary().input.len);
            push_isomorphic(
                &mut new_transforms,
                fold_snapshot.text_summary_for_range(
                    transform_start.to_point(&fold_snapshot)
                        ..fold_edit.new.end.to_point(&fold_snapshot),
                ),
            );
            let new_end = InlayOffset(new_transforms.summary().output.len);
            inlay_edits.push(Edit {
                old: old_start..old_end,
                new: new_start..new_end,
            });

            // If the next edit doesn't intersect the current isomorphic transform, then
            // we can push its remainder.
            if fold_edits_iter
                .peek()
                .map_or(true, |edit| edit.old.start >= cursor.end(&()).0)
            {
                let transform_start = FoldOffset(new_transforms.summary().input.len);
                let transform_end = fold_edit.new.end + (cursor.end(&()).0 - fold_edit.old.end);
                push_isomorphic(
                    &mut new_transforms,
                    fold_snapshot.text_summary_for_range(
                        transform_start.to_point(&fold_snapshot)
                            ..transform_end.to_point(&fold_snapshot),
                    ),
                );
                cursor.next(&());
            }
        }

        new_transforms.push_tree(cursor.suffix(&()), &());
        if new_transforms.first().is_none() {
            new_transforms.push(Transform::Isomorphic(Default::default()), &());
        }
        new_snapshot.transforms = new_transforms;
        new_snapshot.fold_snapshot = fold_snapshot;
        new_snapshot.check_invariants();
        drop(cursor);

        *snapshot = new_snapshot.clone();
        (new_snapshot, inlay_edits.into_inner())
    }

    pub fn splice<T: Into<Rope>>(
        &mut self,
        to_remove: Vec<InlayId>,
        to_insert: Vec<(InlayId, InlayProperties<T>)>,
    ) -> (InlaySnapshot, Vec<InlayEdit>) {
        let mut snapshot = self.snapshot.lock();
        snapshot.version += 1;

        let mut edits = BTreeSet::new();

        self.inlays.retain(|inlay| !to_remove.contains(&inlay.id));
        for inlay_id in to_remove {
            if let Some(inlay) = self.inlays_by_id.remove(&inlay_id) {
                let buffer_point = inlay.position.to_point(snapshot.buffer_snapshot());
                let fold_point = snapshot
                    .fold_snapshot
                    .to_fold_point(buffer_point, Bias::Left);
                let fold_offset = fold_point.to_offset(&snapshot.fold_snapshot);
                edits.insert(fold_offset);
            }
        }

        for (id, properties) in to_insert {
            let inlay = Inlay {
                id,
                position: properties.position,
                text: properties.text.into(),
            };
            self.inlays_by_id.insert(inlay.id, inlay.clone());
            match self.inlays.binary_search_by(|probe| {
                probe
                    .position
                    .cmp(&inlay.position, snapshot.buffer_snapshot())
            }) {
                Ok(ix) | Err(ix) => {
                    self.inlays.insert(ix, inlay.clone());
                }
            }

            let buffer_point = inlay.position.to_point(snapshot.buffer_snapshot());
            let fold_point = snapshot
                .fold_snapshot
                .to_fold_point(buffer_point, Bias::Left);
            let fold_offset = fold_point.to_offset(&snapshot.fold_snapshot);
            edits.insert(fold_offset);
        }

        let fold_snapshot = snapshot.fold_snapshot.clone();
        let fold_edits = edits
            .into_iter()
            .map(|offset| Edit {
                old: offset..offset,
                new: offset..offset,
            })
            .collect();
        drop(snapshot);
        self.sync(fold_snapshot, fold_edits)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn randomly_mutate(
        &mut self,
        next_inlay_id: &mut usize,
        rng: &mut rand::rngs::StdRng,
    ) -> (InlaySnapshot, Vec<InlayEdit>) {
        use rand::prelude::*;
        use util::post_inc;

        let mut to_remove = Vec::new();
        let mut to_insert = Vec::new();
        let snapshot = self.snapshot.lock();
        for _ in 0..rng.gen_range(1..=5) {
            if self.inlays.is_empty() || rng.gen() {
                let buffer_snapshot = snapshot.buffer_snapshot();
                let position = buffer_snapshot.random_byte_range(0, rng).start;
                let bias = if rng.gen() { Bias::Left } else { Bias::Right };
                let len = rng.gen_range(1..=5);
                let text = util::RandomCharIter::new(&mut *rng)
                    .filter(|ch| *ch != '\r')
                    .take(len)
                    .collect::<String>();
                log::info!(
                    "creating inlay at buffer offset {} with bias {:?} and text {:?}",
                    position,
                    bias,
                    text
                );
                to_insert.push((
                    InlayId(post_inc(next_inlay_id)),
                    InlayProperties {
                        position: buffer_snapshot.anchor_at(position, bias),
                        text,
                    },
                ));
            } else {
                to_remove.push(*self.inlays_by_id.keys().choose(rng).unwrap());
            }
        }
        log::info!("removing inlays: {:?}", to_remove);

        drop(snapshot);
        self.splice(to_remove, to_insert)
    }
}

impl InlaySnapshot {
    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        self.fold_snapshot.buffer_snapshot()
    }

    pub fn to_point(&self, offset: InlayOffset) -> InlayPoint {
        let mut cursor = self
            .transforms
            .cursor::<(InlayOffset, (InlayPoint, FoldOffset))>();
        cursor.seek(&offset, Bias::Right, &());
        let overshoot = offset.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let fold_offset_start = cursor.start().1 .1;
                let fold_offset_end = FoldOffset(fold_offset_start.0 + overshoot);
                let fold_start = fold_offset_start.to_point(&self.fold_snapshot);
                let fold_end = fold_offset_end.to_point(&self.fold_snapshot);
                InlayPoint(cursor.start().1 .0 .0 + (fold_end.0 - fold_start.0))
            }
            Some(Transform::Inlay(inlay)) => {
                let overshoot = inlay.text.offset_to_point(overshoot);
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
            .cursor::<(InlayPoint, (InlayOffset, FoldPoint))>();
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let fold_point_start = cursor.start().1 .1;
                let fold_point_end = FoldPoint(fold_point_start.0 + overshoot);
                let fold_start = fold_point_start.to_offset(&self.fold_snapshot);
                let fold_end = fold_point_end.to_offset(&self.fold_snapshot);
                InlayOffset(cursor.start().1 .0 .0 + (fold_end.0 - fold_start.0))
            }
            Some(Transform::Inlay(inlay)) => {
                let overshoot = inlay.text.point_to_offset(overshoot);
                InlayOffset(cursor.start().1 .0 .0 + overshoot)
            }
            None => self.len(),
        }
    }

    pub fn chars_at(&self, start: InlayPoint) -> impl '_ + Iterator<Item = char> {
        self.chunks(self.to_offset(start)..self.len(), false, None, None)
            .flat_map(|chunk| chunk.text.chars())
    }

    pub fn to_fold_point(&self, point: InlayPoint) -> FoldPoint {
        let mut cursor = self.transforms.cursor::<(InlayPoint, FoldPoint)>();
        cursor.seek(&point, Bias::Right, &());
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let overshoot = point.0 - cursor.start().0 .0;
                FoldPoint(cursor.start().1 .0 + overshoot)
            }
            Some(Transform::Inlay(_)) => cursor.start().1,
            None => self.fold_snapshot.max_point(),
        }
    }

    pub fn to_fold_offset(&self, offset: InlayOffset) -> FoldOffset {
        let mut cursor = self.transforms.cursor::<(InlayOffset, FoldOffset)>();
        cursor.seek(&offset, Bias::Right, &());
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let overshoot = offset - cursor.start().0;
                cursor.start().1 + FoldOffset(overshoot.0)
            }
            Some(Transform::Inlay(_)) => cursor.start().1,
            None => self.fold_snapshot.len(),
        }
    }

    pub fn to_inlay_point(&self, point: FoldPoint) -> InlayPoint {
        let mut cursor = self.transforms.cursor::<(FoldPoint, InlayPoint)>();
        cursor.seek(&point, Bias::Left, &());
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let overshoot = point.0 - cursor.start().0 .0;
                InlayPoint(cursor.start().1 .0 + overshoot)
            }
            Some(Transform::Inlay(_)) => cursor.start().1,
            None => self.max_point(),
        }
    }

    pub fn clip_point(&self, point: InlayPoint, bias: Bias) -> InlayPoint {
        let mut cursor = self.transforms.cursor::<(InlayPoint, FoldPoint)>();
        cursor.seek(&point, Bias::Left, &());

        let mut bias = bias;
        let mut skipped_inlay = false;
        loop {
            match cursor.item() {
                Some(Transform::Isomorphic(transform)) => {
                    let overshoot = if skipped_inlay {
                        match bias {
                            Bias::Left => transform.lines,
                            Bias::Right => {
                                if transform.first_line_chars == 0 {
                                    Point::new(1, 0)
                                } else {
                                    Point::new(0, 1)
                                }
                            }
                        }
                    } else {
                        point.0 - cursor.start().0 .0
                    };
                    let fold_point = FoldPoint(cursor.start().1 .0 + overshoot);
                    let clipped_fold_point = self.fold_snapshot.clip_point(fold_point, bias);
                    let clipped_overshoot = clipped_fold_point.0 - cursor.start().1 .0;
                    return InlayPoint(cursor.start().0 .0 + clipped_overshoot);
                }
                Some(Transform::Inlay(_)) => skipped_inlay = true,
                None => match bias {
                    Bias::Left => return Default::default(),
                    Bias::Right => bias = Bias::Left,
                },
            }

            if bias == Bias::Left {
                cursor.prev(&());
            } else {
                cursor.next(&());
            }
        }
    }

    pub fn text_summary_for_range(&self, range: Range<InlayPoint>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self.transforms.cursor::<(InlayPoint, FoldPoint)>();
        cursor.seek(&range.start, Bias::Right, &());

        let overshoot = range.start.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let fold_start = cursor.start().1 .0;
                let suffix_start = FoldPoint(fold_start + overshoot);
                let suffix_end = FoldPoint(
                    fold_start + (cmp::min(cursor.end(&()).0, range.end).0 - cursor.start().0 .0),
                );
                summary = self
                    .fold_snapshot
                    .text_summary_for_range(suffix_start..suffix_end);
                cursor.next(&());
            }
            Some(Transform::Inlay(inlay)) => {
                let suffix_start = inlay.text.point_to_offset(overshoot);
                let suffix_end = inlay.text.point_to_offset(
                    cmp::min(cursor.end(&()).0, range.end).0 - cursor.start().0 .0,
                );
                summary = inlay.text.cursor(suffix_start).summary(suffix_end);
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
                Some(Transform::Isomorphic(_)) => {
                    let prefix_start = cursor.start().1;
                    let prefix_end = FoldPoint(prefix_start.0 + overshoot);
                    summary += self
                        .fold_snapshot
                        .text_summary_for_range(prefix_start..prefix_end);
                }
                Some(Transform::Inlay(inlay)) => {
                    let prefix_end = inlay.text.point_to_offset(overshoot);
                    summary += inlay.text.cursor(0).summary::<TextSummary>(prefix_end);
                }
                None => {}
            }
        }

        summary
    }

    pub fn buffer_rows<'a>(&'a self, row: u32) -> InlayBufferRows<'a> {
        let mut cursor = self.transforms.cursor::<(InlayPoint, FoldPoint)>();
        let inlay_point = InlayPoint::new(row, 0);
        cursor.seek(&inlay_point, Bias::Left, &());

        let mut fold_point = cursor.start().1;
        let fold_row = if row == 0 {
            0
        } else {
            match cursor.item() {
                Some(Transform::Isomorphic(_)) => {
                    fold_point.0 += inlay_point.0 - cursor.start().0 .0;
                    fold_point.row()
                }
                _ => cmp::min(fold_point.row() + 1, self.fold_snapshot.max_point().row()),
            }
        };

        InlayBufferRows {
            transforms: cursor,
            inlay_row: inlay_point.row(),
            fold_rows: self.fold_snapshot.buffer_rows(fold_row),
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
        inlay_highlight_style: Option<HighlightStyle>,
    ) -> InlayChunks<'a> {
        let mut cursor = self.transforms.cursor::<(InlayOffset, FoldOffset)>();
        cursor.seek(&range.start, Bias::Right, &());

        let fold_range = self.to_fold_offset(range.start)..self.to_fold_offset(range.end);
        let fold_chunks = self
            .fold_snapshot
            .chunks(fold_range, language_aware, text_highlights);

        InlayChunks {
            transforms: cursor,
            fold_chunks,
            inlay_chunks: None,
            fold_chunk: None,
            output_offset: range.start,
            max_output_offset: range.end,
            highlight_style: inlay_highlight_style,
        }
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(Default::default()..self.len(), false, None, None)
            .map(|chunk| chunk.text)
            .collect()
    }

    fn check_invariants(&self) {
        #[cfg(any(debug_assertions, feature = "test-support"))]
        {
            assert_eq!(
                self.transforms.summary().input,
                self.fold_snapshot.text_summary()
            );
        }
    }
}

fn push_isomorphic(sum_tree: &mut SumTree<Transform>, summary: TextSummary) {
    if summary.len == 0 {
        return;
    }

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
    use crate::{display_map::fold_map::FoldMap, MultiBuffer};
    use gpui::AppContext;
    use rand::prelude::*;
    use settings::SettingsStore;
    use std::env;
    use text::Patch;
    use util::post_inc;

    #[gpui::test]
    fn test_basic_inlays(cx: &mut AppContext) {
        let buffer = MultiBuffer::build_simple("abcdefghi", cx);
        let buffer_edits = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (fold_map, fold_snapshot) = FoldMap::new(buffer.read(cx).snapshot(cx));
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(fold_snapshot.clone());
        assert_eq!(inlay_snapshot.text(), "abcdefghi");
        let mut next_inlay_id = 0;

        let (inlay_snapshot, _) = inlay_map.splice(
            Vec::new(),
            vec![(
                InlayId(post_inc(&mut next_inlay_id)),
                InlayProperties {
                    position: buffer.read(cx).snapshot(cx).anchor_after(3),
                    text: "|123|",
                },
            )],
        );
        assert_eq!(inlay_snapshot.text(), "abc|123|defghi");
        assert_eq!(
            inlay_snapshot.to_inlay_point(FoldPoint::new(0, 0)),
            InlayPoint::new(0, 0)
        );
        assert_eq!(
            inlay_snapshot.to_inlay_point(FoldPoint::new(0, 1)),
            InlayPoint::new(0, 1)
        );
        assert_eq!(
            inlay_snapshot.to_inlay_point(FoldPoint::new(0, 2)),
            InlayPoint::new(0, 2)
        );
        assert_eq!(
            inlay_snapshot.to_inlay_point(FoldPoint::new(0, 3)),
            InlayPoint::new(0, 3)
        );
        assert_eq!(
            inlay_snapshot.to_inlay_point(FoldPoint::new(0, 4)),
            InlayPoint::new(0, 9)
        );
        assert_eq!(
            inlay_snapshot.to_inlay_point(FoldPoint::new(0, 5)),
            InlayPoint::new(0, 10)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 0), Bias::Left),
            InlayPoint::new(0, 0)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 0), Bias::Right),
            InlayPoint::new(0, 0)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 3), Bias::Left),
            InlayPoint::new(0, 3)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 3), Bias::Right),
            InlayPoint::new(0, 3)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 4), Bias::Left),
            InlayPoint::new(0, 3)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 4), Bias::Right),
            InlayPoint::new(0, 9)
        );

        // Edits before or after the inlay should not affect it.
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(2..3, "x"), (3..3, "y"), (4..4, "z")], None, cx)
        });
        let (fold_snapshot, fold_edits) = fold_map.read(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        let (inlay_snapshot, _) = inlay_map.sync(fold_snapshot.clone(), fold_edits);
        assert_eq!(inlay_snapshot.text(), "abxy|123|dzefghi");

        // An edit surrounding the inlay should invalidate it.
        buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "D")], None, cx));
        let (fold_snapshot, fold_edits) = fold_map.read(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        let (inlay_snapshot, _) = inlay_map.sync(fold_snapshot.clone(), fold_edits);
        assert_eq!(inlay_snapshot.text(), "abxyDzefghi");

        let (inlay_snapshot, _) = inlay_map.splice(
            Vec::new(),
            vec![
                (
                    InlayId(post_inc(&mut next_inlay_id)),
                    InlayProperties {
                        position: buffer.read(cx).snapshot(cx).anchor_before(3),
                        text: "|123|",
                    },
                ),
                (
                    InlayId(post_inc(&mut next_inlay_id)),
                    InlayProperties {
                        position: buffer.read(cx).snapshot(cx).anchor_after(3),
                        text: "|456|",
                    },
                ),
            ],
        );
        assert_eq!(inlay_snapshot.text(), "abx|123||456|yDzefghi");

        // Edits ending where the inlay starts should not move it if it has a left bias.
        buffer.update(cx, |buffer, cx| buffer.edit([(3..3, "JKL")], None, cx));
        let (fold_snapshot, fold_edits) = fold_map.read(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        let (inlay_snapshot, _) = inlay_map.sync(fold_snapshot.clone(), fold_edits);
        assert_eq!(inlay_snapshot.text(), "abx|123|JKL|456|yDzefghi");

        // The inlays can be manually removed.
        let (inlay_snapshot, _) = inlay_map
            .splice::<String>(inlay_map.inlays_by_id.keys().copied().collect(), Vec::new());
        assert_eq!(inlay_snapshot.text(), "abxJKLyDzefghi");
    }

    #[gpui::test]
    fn test_buffer_rows(cx: &mut AppContext) {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi", cx);
        let (_, fold_snapshot) = FoldMap::new(buffer.read(cx).snapshot(cx));
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(fold_snapshot.clone());
        assert_eq!(inlay_snapshot.text(), "abc\ndef\nghi");

        let (inlay_snapshot, _) = inlay_map.splice(
            Vec::new(),
            vec![
                (
                    InlayId(0),
                    InlayProperties {
                        position: buffer.read(cx).snapshot(cx).anchor_before(0),
                        text: "|123|\n",
                    },
                ),
                (
                    InlayId(1),
                    InlayProperties {
                        position: buffer.read(cx).snapshot(cx).anchor_before(4),
                        text: "|456|",
                    },
                ),
                (
                    InlayId(1),
                    InlayProperties {
                        position: buffer.read(cx).snapshot(cx).anchor_before(7),
                        text: "\n|567|\n",
                    },
                ),
            ],
        );
        assert_eq!(inlay_snapshot.text(), "|123|\nabc\n|456|def\n|567|\n\nghi");
        assert_eq!(
            inlay_snapshot.buffer_rows(0).collect::<Vec<_>>(),
            vec![Some(0), None, Some(1), None, None, Some(2)]
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random_inlays(cx: &mut AppContext, mut rng: StdRng) {
        init_test(cx);

        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let len = rng.gen_range(0..30);
        let buffer = if rng.gen() {
            let text = util::RandomCharIter::new(&mut rng)
                .take(len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };
        let mut buffer_snapshot = buffer.read(cx).snapshot(cx);
        log::info!("buffer text: {:?}", buffer_snapshot.text());

        let (mut fold_map, mut fold_snapshot) = FoldMap::new(buffer_snapshot.clone());
        let (mut inlay_map, mut inlay_snapshot) = InlayMap::new(fold_snapshot.clone());
        let mut next_inlay_id = 0;

        for _ in 0..operations {
            let mut fold_edits = Patch::default();
            let mut inlay_edits = Patch::default();

            let mut prev_inlay_text = inlay_snapshot.text();
            let mut buffer_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=29 => {
                    let (snapshot, edits) = inlay_map.randomly_mutate(&mut next_inlay_id, &mut rng);
                    log::info!("mutated text: {:?}", snapshot.text());
                    inlay_edits = Patch::new(edits);
                }
                30..=59 => {
                    for (_, edits) in fold_map.randomly_mutate(&mut rng) {
                        fold_edits = fold_edits.compose(edits);
                    }
                }
                _ => buffer.update(cx, |buffer, cx| {
                    let subscription = buffer.subscribe();
                    let edit_count = rng.gen_range(1..=5);
                    buffer.randomly_mutate(&mut rng, edit_count, cx);
                    buffer_snapshot = buffer.snapshot(cx);
                    let edits = subscription.consume().into_inner();
                    log::info!("editing {:?}", edits);
                    buffer_edits.extend(edits);
                }),
            };

            let (new_fold_snapshot, new_fold_edits) =
                fold_map.read(buffer_snapshot.clone(), buffer_edits);
            fold_snapshot = new_fold_snapshot;
            fold_edits = fold_edits.compose(new_fold_edits);
            let (new_inlay_snapshot, new_inlay_edits) =
                inlay_map.sync(fold_snapshot.clone(), fold_edits.into_inner());
            inlay_snapshot = new_inlay_snapshot;
            inlay_edits = inlay_edits.compose(new_inlay_edits);

            log::info!("buffer text: {:?}", buffer_snapshot.text());
            log::info!("folds text: {:?}", fold_snapshot.text());
            log::info!("inlay text: {:?}", inlay_snapshot.text());

            let inlays = inlay_map
                .inlays
                .iter()
                .filter(|inlay| inlay.position.is_valid(&buffer_snapshot))
                .map(|inlay| {
                    let buffer_point = inlay.position.to_point(&buffer_snapshot);
                    let fold_point = fold_snapshot.to_fold_point(buffer_point, Bias::Left);
                    let fold_offset = fold_point.to_offset(&fold_snapshot);
                    (fold_offset, inlay.clone())
                })
                .collect::<Vec<_>>();
            let mut expected_text = Rope::from(fold_snapshot.text().as_str());
            for (offset, inlay) in inlays.into_iter().rev() {
                expected_text.replace(offset.0..offset.0, &inlay.text.to_string());
            }
            assert_eq!(inlay_snapshot.text(), expected_text.to_string());

            let expected_buffer_rows = inlay_snapshot.buffer_rows(0).collect::<Vec<_>>();
            assert_eq!(
                expected_buffer_rows.len() as u32,
                expected_text.max_point().row + 1
            );
            for row_start in 0..expected_buffer_rows.len() {
                assert_eq!(
                    inlay_snapshot
                        .buffer_rows(row_start as u32)
                        .collect::<Vec<_>>(),
                    &expected_buffer_rows[row_start..],
                    "incorrect buffer rows starting at {}",
                    row_start
                );
            }

            for _ in 0..5 {
                let mut end = rng.gen_range(0..=inlay_snapshot.len().0);
                end = expected_text.clip_offset(end, Bias::Right);
                let mut start = rng.gen_range(0..=end);
                start = expected_text.clip_offset(start, Bias::Right);

                let actual_text = inlay_snapshot
                    .chunks(InlayOffset(start)..InlayOffset(end), false, None, None)
                    .map(|chunk| chunk.text)
                    .collect::<String>();
                assert_eq!(
                    actual_text,
                    expected_text.slice(start..end).to_string(),
                    "incorrect text in range {:?}",
                    start..end
                );

                let start_point = InlayPoint(expected_text.offset_to_point(start));
                let end_point = InlayPoint(expected_text.offset_to_point(end));
                assert_eq!(
                    inlay_snapshot.text_summary_for_range(start_point..end_point),
                    expected_text.slice(start..end).summary()
                );
            }

            for edit in inlay_edits {
                prev_inlay_text.replace_range(
                    edit.new.start.0..edit.new.start.0 + edit.old_len().0,
                    &inlay_snapshot.text()[edit.new.start.0..edit.new.end.0],
                );
            }
            assert_eq!(prev_inlay_text, inlay_snapshot.text());

            assert_eq!(expected_text.max_point(), inlay_snapshot.max_point().0);
            assert_eq!(expected_text.len(), inlay_snapshot.len().0);

            let mut inlay_point = InlayPoint::default();
            let mut inlay_offset = InlayOffset::default();
            for ch in expected_text.chars() {
                assert_eq!(
                    inlay_snapshot.to_offset(inlay_point),
                    inlay_offset,
                    "invalid to_offset({:?})",
                    inlay_point
                );
                assert_eq!(
                    inlay_snapshot.to_point(inlay_offset),
                    inlay_point,
                    "invalid to_point({:?})",
                    inlay_offset
                );
                assert_eq!(
                    inlay_snapshot.to_inlay_point(inlay_snapshot.to_fold_point(inlay_point)),
                    inlay_snapshot.clip_point(inlay_point, Bias::Left),
                    "to_fold_point({:?}) = {:?}",
                    inlay_point,
                    inlay_snapshot.to_fold_point(inlay_point),
                );

                let mut bytes = [0; 4];
                for byte in ch.encode_utf8(&mut bytes).as_bytes() {
                    inlay_offset.0 += 1;
                    if *byte == b'\n' {
                        inlay_point.0 += Point::new(1, 0);
                    } else {
                        inlay_point.0 += Point::new(0, 1);
                    }

                    let clipped_left_point = inlay_snapshot.clip_point(inlay_point, Bias::Left);
                    let clipped_right_point = inlay_snapshot.clip_point(inlay_point, Bias::Right);
                    assert!(
                        clipped_left_point <= clipped_right_point,
                        "clipped left point {:?} is greater than clipped right point {:?}",
                        clipped_left_point,
                        clipped_right_point
                    );

                    // Ensure the clipped points are at valid text locations.
                    assert_eq!(
                        clipped_left_point.0,
                        expected_text.clip_point(clipped_left_point.0, Bias::Left)
                    );
                    assert_eq!(
                        clipped_right_point.0,
                        expected_text.clip_point(clipped_right_point.0, Bias::Right)
                    );

                    // Ensure the clipped points never overshoot the end of the map.
                    assert!(clipped_left_point <= inlay_snapshot.max_point());
                    assert!(clipped_right_point <= inlay_snapshot.max_point());
                }
            }
        }
    }

    fn init_test(cx: &mut AppContext) {
        cx.set_global(SettingsStore::test(cx));
        theme::init((), cx);
    }
}
