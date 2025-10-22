use crate::{
    ChunkRenderer, HighlightStyles,
    inlays::{Inlay, InlayContent},
};
use collections::BTreeSet;
use language::{Chunk, Edit, Point, TextSummary};
use multi_buffer::{MultiBufferRow, MultiBufferRows, MultiBufferSnapshot, RowInfo, ToOffset};
use project::InlayId;
use std::{
    cmp,
    ops::{Add, AddAssign, Range, Sub, SubAssign},
    sync::Arc,
};
use sum_tree::{Bias, Cursor, Dimensions, SumTree};
use text::{ChunkBitmaps, Patch};
use ui::{ActiveTheme, IntoElement as _, ParentElement as _, Styled as _, div};

use super::{Highlights, custom_highlights::CustomHighlightsChunks, fold_map::ChunkRendererId};

/// Decides where the [`Inlay`]s should be displayed.
///
/// See the [`display_map` module documentation](crate::display_map) for more information.
pub struct InlayMap {
    snapshot: InlaySnapshot,
    inlays: Vec<Inlay>,
}

#[derive(Clone)]
pub struct InlaySnapshot {
    pub buffer: MultiBufferSnapshot,
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

    fn summary(&self, _: ()) -> Self::Summary {
        match self {
            Transform::Isomorphic(summary) => TransformSummary {
                input: *summary,
                output: *summary,
            },
            Transform::Inlay(inlay) => TransformSummary {
                input: TextSummary::default(),
                output: inlay.text().summary(),
            },
        }
    }
}

#[derive(Clone, Debug, Default)]
struct TransformSummary {
    input: TextSummary,
    output: TextSummary,
}

impl sum_tree::ContextLessSummary for TransformSummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, other: &Self) {
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

impl SubAssign for InlayOffset {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InlayOffset {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: ()) {
        self.0 += &summary.output.len;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct InlayPoint(pub Point);

impl Add for InlayPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for InlayPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InlayPoint {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: ()) {
        self.0 += &summary.output.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for usize {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: ()) {
        *self += &summary.input.len;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for Point {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: ()) {
        *self += &summary.input.lines;
    }
}

#[derive(Clone)]
pub struct InlayBufferRows<'a> {
    transforms: Cursor<'a, 'static, Transform, Dimensions<InlayPoint, Point>>,
    buffer_rows: MultiBufferRows<'a>,
    inlay_row: u32,
    max_buffer_row: MultiBufferRow,
}

pub struct InlayChunks<'a> {
    transforms: Cursor<'a, 'static, Transform, Dimensions<InlayOffset, usize>>,
    buffer_chunks: CustomHighlightsChunks<'a>,
    buffer_chunk: Option<Chunk<'a>>,
    inlay_chunks: Option<text::ChunkWithBitmaps<'a>>,
    /// text, char bitmap, tabs bitmap
    inlay_chunk: Option<ChunkBitmaps<'a>>,
    output_offset: InlayOffset,
    max_output_offset: InlayOffset,
    highlight_styles: HighlightStyles,
    highlights: Highlights<'a>,
    snapshot: &'a InlaySnapshot,
}

#[derive(Clone)]
pub struct InlayChunk<'a> {
    pub chunk: Chunk<'a>,
    /// Whether the inlay should be customly rendered.
    pub renderer: Option<ChunkRenderer>,
}

impl InlayChunks<'_> {
    pub fn seek(&mut self, new_range: Range<InlayOffset>) {
        self.transforms.seek(&new_range.start, Bias::Right);

        let buffer_range = self.snapshot.to_buffer_offset(new_range.start)
            ..self.snapshot.to_buffer_offset(new_range.end);
        self.buffer_chunks.seek(buffer_range);
        self.inlay_chunks = None;
        self.buffer_chunk = None;
        self.output_offset = new_range.start;
        self.max_output_offset = new_range.end;
    }

    pub fn offset(&self) -> InlayOffset {
        self.output_offset
    }
}

impl<'a> Iterator for InlayChunks<'a> {
    type Item = InlayChunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_offset == self.max_output_offset {
            return None;
        }

        let chunk = match self.transforms.item()? {
            Transform::Isomorphic(_) => {
                let chunk = self
                    .buffer_chunk
                    .get_or_insert_with(|| self.buffer_chunks.next().unwrap());
                if chunk.text.is_empty() {
                    *chunk = self.buffer_chunks.next().unwrap();
                }

                let desired_bytes = self.transforms.end().0.0 - self.output_offset.0;

                // If we're already at the transform boundary, skip to the next transform
                if desired_bytes == 0 {
                    self.inlay_chunks = None;
                    self.transforms.next();
                    return self.next();
                }

                // Determine split index handling edge cases
                let split_index = if desired_bytes >= chunk.text.len() {
                    chunk.text.len()
                } else if chunk.text.is_char_boundary(desired_bytes) {
                    desired_bytes
                } else {
                    find_next_utf8_boundary(chunk.text, desired_bytes)
                };

                let (prefix, suffix) = chunk.text.split_at(split_index);
                self.output_offset.0 += prefix.len();

                let mask = 1u128.unbounded_shl(split_index as u32).wrapping_sub(1);
                let chars = chunk.chars & mask;
                let tabs = chunk.tabs & mask;

                chunk.chars = chunk.chars.unbounded_shr(split_index as u32);
                chunk.tabs = chunk.tabs.unbounded_shr(split_index as u32);
                chunk.text = suffix;

                InlayChunk {
                    chunk: Chunk {
                        text: prefix,
                        chars,
                        tabs,
                        ..chunk.clone()
                    },
                    renderer: None,
                }
            }
            Transform::Inlay(inlay) => {
                let mut inlay_style_and_highlight = None;
                if let Some(inlay_highlights) = self.highlights.inlay_highlights {
                    for (_, inlay_id_to_data) in inlay_highlights.iter() {
                        let style_and_highlight = inlay_id_to_data.get(&inlay.id);
                        if style_and_highlight.is_some() {
                            inlay_style_and_highlight = style_and_highlight;
                            break;
                        }
                    }
                }

                let mut renderer = None;
                let mut highlight_style = match inlay.id {
                    InlayId::EditPrediction(_) => self.highlight_styles.edit_prediction.map(|s| {
                        if inlay.text().chars().all(|c| c.is_whitespace()) {
                            s.whitespace
                        } else {
                            s.insertion
                        }
                    }),
                    InlayId::Hint(_) => self.highlight_styles.inlay_hint,
                    InlayId::DebuggerValue(_) => self.highlight_styles.inlay_hint,
                    InlayId::Color(_) => {
                        if let InlayContent::Color(color) = inlay.content {
                            renderer = Some(ChunkRenderer {
                                id: ChunkRendererId::Inlay(inlay.id),
                                render: Arc::new(move |cx| {
                                    div()
                                        .relative()
                                        .size_3p5()
                                        .child(
                                            div()
                                                .absolute()
                                                .right_1()
                                                .size_3()
                                                .border_1()
                                                .border_color(
                                                    if cx.theme().appearance().is_light() {
                                                        gpui::black().opacity(0.5)
                                                    } else {
                                                        gpui::white().opacity(0.5)
                                                    },
                                                )
                                                .bg(color),
                                        )
                                        .into_any_element()
                                }),
                                constrain_width: false,
                                measured_width: None,
                            });
                        }
                        self.highlight_styles.inlay_hint
                    }
                };
                let next_inlay_highlight_endpoint;
                let offset_in_inlay = self.output_offset - self.transforms.start().0;
                if let Some((style, highlight)) = inlay_style_and_highlight {
                    let range = &highlight.range;
                    if offset_in_inlay.0 < range.start {
                        next_inlay_highlight_endpoint = range.start - offset_in_inlay.0;
                    } else if offset_in_inlay.0 >= range.end {
                        next_inlay_highlight_endpoint = usize::MAX;
                    } else {
                        next_inlay_highlight_endpoint = range.end - offset_in_inlay.0;
                        highlight_style = highlight_style
                            .map(|highlight| highlight.highlight(*style))
                            .or_else(|| Some(*style));
                    }
                } else {
                    next_inlay_highlight_endpoint = usize::MAX;
                }

                let inlay_chunks = self.inlay_chunks.get_or_insert_with(|| {
                    let start = offset_in_inlay;
                    let end = cmp::min(self.max_output_offset, self.transforms.end().0)
                        - self.transforms.start().0;
                    let chunks = inlay.text().chunks_in_range(start.0..end.0);
                    text::ChunkWithBitmaps(chunks)
                });
                let ChunkBitmaps {
                    text: inlay_chunk,
                    chars,
                    tabs,
                } = self
                    .inlay_chunk
                    .get_or_insert_with(|| inlay_chunks.next().unwrap());

                // Determine split index handling edge cases
                let split_index = if next_inlay_highlight_endpoint >= inlay_chunk.len() {
                    inlay_chunk.len()
                } else if next_inlay_highlight_endpoint == 0 {
                    // Need to take at least one character to make progress
                    inlay_chunk
                        .chars()
                        .next()
                        .map(|c| c.len_utf8())
                        .unwrap_or(1)
                } else if inlay_chunk.is_char_boundary(next_inlay_highlight_endpoint) {
                    next_inlay_highlight_endpoint
                } else {
                    find_next_utf8_boundary(inlay_chunk, next_inlay_highlight_endpoint)
                };

                let (chunk, remainder) = inlay_chunk.split_at(split_index);
                *inlay_chunk = remainder;

                let mask = 1u128.unbounded_shl(split_index as u32).wrapping_sub(1);
                let new_chars = *chars & mask;
                let new_tabs = *tabs & mask;

                *chars = chars.unbounded_shr(split_index as u32);
                *tabs = tabs.unbounded_shr(split_index as u32);

                if inlay_chunk.is_empty() {
                    self.inlay_chunk = None;
                }

                self.output_offset.0 += chunk.len();

                InlayChunk {
                    chunk: Chunk {
                        text: chunk,
                        chars: new_chars,
                        tabs: new_tabs,
                        highlight_style,
                        is_inlay: true,
                        ..Chunk::default()
                    },
                    renderer,
                }
            }
        };

        if self.output_offset >= self.transforms.end().0 {
            self.inlay_chunks = None;
            self.transforms.next();
        }

        Some(chunk)
    }
}

impl InlayBufferRows<'_> {
    pub fn seek(&mut self, row: u32) {
        let inlay_point = InlayPoint::new(row, 0);
        self.transforms.seek(&inlay_point, Bias::Left);

        let mut buffer_point = self.transforms.start().1;
        let buffer_row = MultiBufferRow(if row == 0 {
            0
        } else {
            match self.transforms.item() {
                Some(Transform::Isomorphic(_)) => {
                    buffer_point += inlay_point.0 - self.transforms.start().0.0;
                    buffer_point.row
                }
                _ => cmp::min(buffer_point.row + 1, self.max_buffer_row.0),
            }
        });
        self.inlay_row = inlay_point.row();
        self.buffer_rows.seek(buffer_row);
    }
}

impl Iterator for InlayBufferRows<'_> {
    type Item = RowInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let buffer_row = if self.inlay_row == 0 {
            self.buffer_rows.next().unwrap()
        } else {
            match self.transforms.item()? {
                Transform::Inlay(_) => Default::default(),
                Transform::Isomorphic(_) => self.buffer_rows.next().unwrap(),
            }
        };

        self.inlay_row += 1;
        self.transforms
            .seek_forward(&InlayPoint::new(self.inlay_row, 0), Bias::Left);

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
}

impl InlayMap {
    pub fn new(buffer: MultiBufferSnapshot) -> (Self, InlaySnapshot) {
        let version = 0;
        let snapshot = InlaySnapshot {
            buffer: buffer.clone(),
            transforms: SumTree::from_iter(Some(Transform::Isomorphic(buffer.text_summary())), ()),
            version,
        };

        (
            Self {
                snapshot: snapshot.clone(),
                inlays: Vec::new(),
            },
            snapshot,
        )
    }

    pub fn sync(
        &mut self,
        buffer_snapshot: MultiBufferSnapshot,
        mut buffer_edits: Vec<text::Edit<usize>>,
    ) -> (InlaySnapshot, Vec<InlayEdit>) {
        let snapshot = &mut self.snapshot;

        if buffer_edits.is_empty()
            && snapshot.buffer.trailing_excerpt_update_count()
                != buffer_snapshot.trailing_excerpt_update_count()
        {
            buffer_edits.push(Edit {
                old: snapshot.buffer.len()..snapshot.buffer.len(),
                new: buffer_snapshot.len()..buffer_snapshot.len(),
            });
        }

        if buffer_edits.is_empty() {
            if snapshot.buffer.edit_count() != buffer_snapshot.edit_count()
                || snapshot.buffer.non_text_state_update_count()
                    != buffer_snapshot.non_text_state_update_count()
                || snapshot.buffer.trailing_excerpt_update_count()
                    != buffer_snapshot.trailing_excerpt_update_count()
            {
                snapshot.version += 1;
            }

            snapshot.buffer = buffer_snapshot;
            (snapshot.clone(), Vec::new())
        } else {
            let mut inlay_edits = Patch::default();
            let mut new_transforms = SumTree::default();
            let mut cursor = snapshot
                .transforms
                .cursor::<Dimensions<usize, InlayOffset>>(());
            let mut buffer_edits_iter = buffer_edits.iter().peekable();
            while let Some(buffer_edit) = buffer_edits_iter.next() {
                new_transforms.append(cursor.slice(&buffer_edit.old.start, Bias::Left), ());
                if let Some(Transform::Isomorphic(transform)) = cursor.item()
                    && cursor.end().0 == buffer_edit.old.start
                {
                    push_isomorphic(&mut new_transforms, *transform);
                    cursor.next();
                }

                // Remove all the inlays and transforms contained by the edit.
                let old_start =
                    cursor.start().1 + InlayOffset(buffer_edit.old.start - cursor.start().0);
                cursor.seek(&buffer_edit.old.end, Bias::Right);
                let old_end =
                    cursor.start().1 + InlayOffset(buffer_edit.old.end - cursor.start().0);

                // Push the unchanged prefix.
                let prefix_start = new_transforms.summary().input.len;
                let prefix_end = buffer_edit.new.start;
                push_isomorphic(
                    &mut new_transforms,
                    buffer_snapshot.text_summary_for_range(prefix_start..prefix_end),
                );
                let new_start = InlayOffset(new_transforms.summary().output.len);

                let start_ix = match self.inlays.binary_search_by(|probe| {
                    probe
                        .position
                        .to_offset(&buffer_snapshot)
                        .cmp(&buffer_edit.new.start)
                        .then(std::cmp::Ordering::Greater)
                }) {
                    Ok(ix) | Err(ix) => ix,
                };

                for inlay in &self.inlays[start_ix..] {
                    if !inlay.position.is_valid(&buffer_snapshot) {
                        continue;
                    }
                    let buffer_offset = inlay.position.to_offset(&buffer_snapshot);
                    if buffer_offset > buffer_edit.new.end {
                        break;
                    }

                    let prefix_start = new_transforms.summary().input.len;
                    let prefix_end = buffer_offset;
                    push_isomorphic(
                        &mut new_transforms,
                        buffer_snapshot.text_summary_for_range(prefix_start..prefix_end),
                    );

                    new_transforms.push(Transform::Inlay(inlay.clone()), ());
                }

                // Apply the rest of the edit.
                let transform_start = new_transforms.summary().input.len;
                push_isomorphic(
                    &mut new_transforms,
                    buffer_snapshot.text_summary_for_range(transform_start..buffer_edit.new.end),
                );
                let new_end = InlayOffset(new_transforms.summary().output.len);
                inlay_edits.push(Edit {
                    old: old_start..old_end,
                    new: new_start..new_end,
                });

                // If the next edit doesn't intersect the current isomorphic transform, then
                // we can push its remainder.
                if buffer_edits_iter
                    .peek()
                    .is_none_or(|edit| edit.old.start >= cursor.end().0)
                {
                    let transform_start = new_transforms.summary().input.len;
                    let transform_end =
                        buffer_edit.new.end + (cursor.end().0 - buffer_edit.old.end);
                    push_isomorphic(
                        &mut new_transforms,
                        buffer_snapshot.text_summary_for_range(transform_start..transform_end),
                    );
                    cursor.next();
                }
            }

            new_transforms.append(cursor.suffix(), ());
            if new_transforms.is_empty() {
                new_transforms.push(Transform::Isomorphic(Default::default()), ());
            }

            drop(cursor);
            snapshot.transforms = new_transforms;
            snapshot.version += 1;
            snapshot.buffer = buffer_snapshot;
            snapshot.check_invariants();

            (snapshot.clone(), inlay_edits.into_inner())
        }
    }

    pub fn splice(
        &mut self,
        to_remove: &[InlayId],
        to_insert: Vec<Inlay>,
    ) -> (InlaySnapshot, Vec<InlayEdit>) {
        let snapshot = &mut self.snapshot;
        let mut edits = BTreeSet::new();

        self.inlays.retain(|inlay| {
            let retain = !to_remove.contains(&inlay.id);
            if !retain {
                let offset = inlay.position.to_offset(&snapshot.buffer);
                edits.insert(offset);
            }
            retain
        });

        for inlay_to_insert in to_insert {
            // Avoid inserting empty inlays.
            if inlay_to_insert.text().is_empty() {
                continue;
            }

            let offset = inlay_to_insert.position.to_offset(&snapshot.buffer);
            match self.inlays.binary_search_by(|probe| {
                probe
                    .position
                    .cmp(&inlay_to_insert.position, &snapshot.buffer)
                    .then(std::cmp::Ordering::Less)
            }) {
                Ok(ix) | Err(ix) => {
                    self.inlays.insert(ix, inlay_to_insert);
                }
            }

            edits.insert(offset);
        }

        let buffer_edits = edits
            .into_iter()
            .map(|offset| Edit {
                old: offset..offset,
                new: offset..offset,
            })
            .collect();
        let buffer_snapshot = snapshot.buffer.clone();
        let (snapshot, edits) = self.sync(buffer_snapshot, buffer_edits);
        (snapshot, edits)
    }

    pub fn current_inlays(&self) -> impl Iterator<Item = &Inlay> {
        self.inlays.iter()
    }

    #[cfg(test)]
    pub(crate) fn randomly_mutate(
        &mut self,
        next_inlay_id: &mut usize,
        rng: &mut rand::rngs::StdRng,
    ) -> (InlaySnapshot, Vec<InlayEdit>) {
        use rand::prelude::*;
        use util::post_inc;

        let mut to_remove = Vec::new();
        let mut to_insert = Vec::new();
        let snapshot = &mut self.snapshot;
        for i in 0..rng.random_range(1..=5) {
            if self.inlays.is_empty() || rng.random() {
                let position = snapshot.buffer.random_byte_range(0, rng).start;
                let bias = if rng.random() {
                    Bias::Left
                } else {
                    Bias::Right
                };
                let len = if rng.random_bool(0.01) {
                    0
                } else {
                    rng.random_range(1..=5)
                };
                let text = util::RandomCharIter::new(&mut *rng)
                    .filter(|ch| *ch != '\r')
                    .take(len)
                    .collect::<String>();

                let next_inlay = if i % 2 == 0 {
                    Inlay::mock_hint(
                        post_inc(next_inlay_id),
                        snapshot.buffer.anchor_at(position, bias),
                        &text,
                    )
                } else {
                    Inlay::edit_prediction(
                        post_inc(next_inlay_id),
                        snapshot.buffer.anchor_at(position, bias),
                        &text,
                    )
                };
                let inlay_id = next_inlay.id;
                log::info!(
                    "creating inlay {inlay_id:?} at buffer offset {position} with bias {bias:?} and text {text:?}"
                );
                to_insert.push(next_inlay);
            } else {
                to_remove.push(
                    self.inlays
                        .iter()
                        .choose(rng)
                        .map(|inlay| inlay.id)
                        .unwrap(),
                );
            }
        }
        log::info!("removing inlays: {:?}", to_remove);

        let (snapshot, edits) = self.splice(&to_remove, to_insert);
        (snapshot, edits)
    }
}

impl InlaySnapshot {
    pub fn to_point(&self, offset: InlayOffset) -> InlayPoint {
        let (start, _, item) = self
            .transforms
            .find::<Dimensions<InlayOffset, InlayPoint, usize>, _>((), &offset, Bias::Right);
        let overshoot = offset.0 - start.0.0;
        match item {
            Some(Transform::Isomorphic(_)) => {
                let buffer_offset_start = start.2;
                let buffer_offset_end = buffer_offset_start + overshoot;
                let buffer_start = self.buffer.offset_to_point(buffer_offset_start);
                let buffer_end = self.buffer.offset_to_point(buffer_offset_end);
                InlayPoint(start.1.0 + (buffer_end - buffer_start))
            }
            Some(Transform::Inlay(inlay)) => {
                let overshoot = inlay.text().offset_to_point(overshoot);
                InlayPoint(start.1.0 + overshoot)
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
        let (start, _, item) = self
            .transforms
            .find::<Dimensions<InlayPoint, InlayOffset, Point>, _>((), &point, Bias::Right);
        let overshoot = point.0 - start.0.0;
        match item {
            Some(Transform::Isomorphic(_)) => {
                let buffer_point_start = start.2;
                let buffer_point_end = buffer_point_start + overshoot;
                let buffer_offset_start = self.buffer.point_to_offset(buffer_point_start);
                let buffer_offset_end = self.buffer.point_to_offset(buffer_point_end);
                InlayOffset(start.1.0 + (buffer_offset_end - buffer_offset_start))
            }
            Some(Transform::Inlay(inlay)) => {
                let overshoot = inlay.text().point_to_offset(overshoot);
                InlayOffset(start.1.0 + overshoot)
            }
            None => self.len(),
        }
    }
    pub fn to_buffer_point(&self, point: InlayPoint) -> Point {
        let (start, _, item) =
            self.transforms
                .find::<Dimensions<InlayPoint, Point>, _>((), &point, Bias::Right);
        match item {
            Some(Transform::Isomorphic(_)) => {
                let overshoot = point.0 - start.0.0;
                start.1 + overshoot
            }
            Some(Transform::Inlay(_)) => start.1,
            None => self.buffer.max_point(),
        }
    }
    pub fn to_buffer_offset(&self, offset: InlayOffset) -> usize {
        let (start, _, item) =
            self.transforms
                .find::<Dimensions<InlayOffset, usize>, _>((), &offset, Bias::Right);
        match item {
            Some(Transform::Isomorphic(_)) => {
                let overshoot = offset - start.0;
                start.1 + overshoot.0
            }
            Some(Transform::Inlay(_)) => start.1,
            None => self.buffer.len(),
        }
    }

    pub fn to_inlay_offset(&self, offset: usize) -> InlayOffset {
        let mut cursor = self.transforms.cursor::<Dimensions<usize, InlayOffset>>(());
        cursor.seek(&offset, Bias::Left);
        loop {
            match cursor.item() {
                Some(Transform::Isomorphic(_)) => {
                    if offset == cursor.end().0 {
                        while let Some(Transform::Inlay(inlay)) = cursor.next_item() {
                            if inlay.position.bias() == Bias::Right {
                                break;
                            } else {
                                cursor.next();
                            }
                        }
                        return cursor.end().1;
                    } else {
                        let overshoot = offset - cursor.start().0;
                        return InlayOffset(cursor.start().1.0 + overshoot);
                    }
                }
                Some(Transform::Inlay(inlay)) => {
                    if inlay.position.bias() == Bias::Left {
                        cursor.next();
                    } else {
                        return cursor.start().1;
                    }
                }
                None => {
                    return self.len();
                }
            }
        }
    }
    pub fn to_inlay_point(&self, point: Point) -> InlayPoint {
        let mut cursor = self.transforms.cursor::<Dimensions<Point, InlayPoint>>(());
        cursor.seek(&point, Bias::Left);
        loop {
            match cursor.item() {
                Some(Transform::Isomorphic(_)) => {
                    if point == cursor.end().0 {
                        while let Some(Transform::Inlay(inlay)) = cursor.next_item() {
                            if inlay.position.bias() == Bias::Right {
                                break;
                            } else {
                                cursor.next();
                            }
                        }
                        return cursor.end().1;
                    } else {
                        let overshoot = point - cursor.start().0;
                        return InlayPoint(cursor.start().1.0 + overshoot);
                    }
                }
                Some(Transform::Inlay(inlay)) => {
                    if inlay.position.bias() == Bias::Left {
                        cursor.next();
                    } else {
                        return cursor.start().1;
                    }
                }
                None => {
                    return self.max_point();
                }
            }
        }
    }

    pub fn clip_point(&self, mut point: InlayPoint, mut bias: Bias) -> InlayPoint {
        let mut cursor = self.transforms.cursor::<Dimensions<InlayPoint, Point>>(());
        cursor.seek(&point, Bias::Left);
        loop {
            match cursor.item() {
                Some(Transform::Isomorphic(transform)) => {
                    if cursor.start().0 == point {
                        if let Some(Transform::Inlay(inlay)) = cursor.prev_item() {
                            if inlay.position.bias() == Bias::Left {
                                return point;
                            } else if bias == Bias::Left {
                                cursor.prev();
                            } else if transform.first_line_chars == 0 {
                                point.0 += Point::new(1, 0);
                            } else {
                                point.0 += Point::new(0, 1);
                            }
                        } else {
                            return point;
                        }
                    } else if cursor.end().0 == point {
                        if let Some(Transform::Inlay(inlay)) = cursor.next_item() {
                            if inlay.position.bias() == Bias::Right {
                                return point;
                            } else if bias == Bias::Right {
                                cursor.next();
                            } else if point.0.column == 0 {
                                point.0.row -= 1;
                                point.0.column = self.line_len(point.0.row);
                            } else {
                                point.0.column -= 1;
                            }
                        } else {
                            return point;
                        }
                    } else {
                        let overshoot = point.0 - cursor.start().0.0;
                        let buffer_point = cursor.start().1 + overshoot;
                        let clipped_buffer_point = self.buffer.clip_point(buffer_point, bias);
                        let clipped_overshoot = clipped_buffer_point - cursor.start().1;
                        let clipped_point = InlayPoint(cursor.start().0.0 + clipped_overshoot);
                        if clipped_point == point {
                            return clipped_point;
                        } else {
                            point = clipped_point;
                        }
                    }
                }
                Some(Transform::Inlay(inlay)) => {
                    if point == cursor.start().0 && inlay.position.bias() == Bias::Right {
                        match cursor.prev_item() {
                            Some(Transform::Inlay(inlay)) => {
                                if inlay.position.bias() == Bias::Left {
                                    return point;
                                }
                            }
                            _ => return point,
                        }
                    } else if point == cursor.end().0 && inlay.position.bias() == Bias::Left {
                        match cursor.next_item() {
                            Some(Transform::Inlay(inlay)) => {
                                if inlay.position.bias() == Bias::Right {
                                    return point;
                                }
                            }
                            _ => return point,
                        }
                    }

                    if bias == Bias::Left {
                        point = cursor.start().0;
                        cursor.prev();
                    } else {
                        cursor.next();
                        point = cursor.start().0;
                    }
                }
                None => {
                    bias = bias.invert();
                    if bias == Bias::Left {
                        point = cursor.start().0;
                        cursor.prev();
                    } else {
                        cursor.next();
                        point = cursor.start().0;
                    }
                }
            }
        }
    }

    pub fn text_summary(&self) -> TextSummary {
        self.transforms.summary().output
    }

    pub fn text_summary_for_range(&self, range: Range<InlayOffset>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self.transforms.cursor::<Dimensions<InlayOffset, usize>>(());
        cursor.seek(&range.start, Bias::Right);

        let overshoot = range.start.0 - cursor.start().0.0;
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let buffer_start = cursor.start().1;
                let suffix_start = buffer_start + overshoot;
                let suffix_end =
                    buffer_start + (cmp::min(cursor.end().0, range.end).0 - cursor.start().0.0);
                summary = self.buffer.text_summary_for_range(suffix_start..suffix_end);
                cursor.next();
            }
            Some(Transform::Inlay(inlay)) => {
                let suffix_start = overshoot;
                let suffix_end = cmp::min(cursor.end().0, range.end).0 - cursor.start().0.0;
                summary = inlay.text().cursor(suffix_start).summary(suffix_end);
                cursor.next();
            }
            None => {}
        }

        if range.end > cursor.start().0 {
            summary += cursor
                .summary::<_, TransformSummary>(&range.end, Bias::Right)
                .output;

            let overshoot = range.end.0 - cursor.start().0.0;
            match cursor.item() {
                Some(Transform::Isomorphic(_)) => {
                    let prefix_start = cursor.start().1;
                    let prefix_end = prefix_start + overshoot;
                    summary += self
                        .buffer
                        .text_summary_for_range::<TextSummary, _>(prefix_start..prefix_end);
                }
                Some(Transform::Inlay(inlay)) => {
                    let prefix_end = overshoot;
                    summary += inlay.text().cursor(0).summary::<TextSummary>(prefix_end);
                }
                None => {}
            }
        }

        summary
    }

    pub fn row_infos(&self, row: u32) -> InlayBufferRows<'_> {
        let mut cursor = self.transforms.cursor::<Dimensions<InlayPoint, Point>>(());
        let inlay_point = InlayPoint::new(row, 0);
        cursor.seek(&inlay_point, Bias::Left);

        let max_buffer_row = self.buffer.max_row();
        let mut buffer_point = cursor.start().1;
        let buffer_row = if row == 0 {
            MultiBufferRow(0)
        } else {
            match cursor.item() {
                Some(Transform::Isomorphic(_)) => {
                    buffer_point += inlay_point.0 - cursor.start().0.0;
                    MultiBufferRow(buffer_point.row)
                }
                _ => cmp::min(MultiBufferRow(buffer_point.row + 1), max_buffer_row),
            }
        };

        InlayBufferRows {
            transforms: cursor,
            inlay_row: inlay_point.row(),
            buffer_rows: self.buffer.row_infos(buffer_row),
            max_buffer_row,
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

    pub(crate) fn chunks<'a>(
        &'a self,
        range: Range<InlayOffset>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> InlayChunks<'a> {
        let mut cursor = self.transforms.cursor::<Dimensions<InlayOffset, usize>>(());
        cursor.seek(&range.start, Bias::Right);

        let buffer_range = self.to_buffer_offset(range.start)..self.to_buffer_offset(range.end);
        let buffer_chunks = CustomHighlightsChunks::new(
            buffer_range,
            language_aware,
            highlights.text_highlights,
            &self.buffer,
        );

        InlayChunks {
            transforms: cursor,
            buffer_chunks,
            inlay_chunks: None,
            inlay_chunk: None,
            buffer_chunk: None,
            output_offset: range.start,
            max_output_offset: range.end,
            highlight_styles: highlights.styles,
            highlights,
            snapshot: self,
        }
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(Default::default()..self.len(), false, Highlights::default())
            .map(|chunk| chunk.chunk.text)
            .collect()
    }

    fn check_invariants(&self) {
        #[cfg(any(debug_assertions, feature = "test-support"))]
        {
            assert_eq!(self.transforms.summary().input, self.buffer.text_summary());
            let mut transforms = self.transforms.iter().peekable();
            while let Some(transform) = transforms.next() {
                let transform_is_isomorphic = matches!(transform, Transform::Isomorphic(_));
                if let Some(next_transform) = transforms.peek() {
                    let next_transform_is_isomorphic =
                        matches!(next_transform, Transform::Isomorphic(_));
                    assert!(
                        !transform_is_isomorphic || !next_transform_is_isomorphic,
                        "two adjacent isomorphic transforms"
                    );
                }
            }
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
        (),
    );

    if let Some(summary) = summary {
        sum_tree.push(Transform::Isomorphic(summary), ());
    }
}

/// Given a byte index that is NOT a UTF-8 boundary, find the next one.
/// Assumes: 0 < byte_index < text.len() and !text.is_char_boundary(byte_index)
#[inline(always)]
fn find_next_utf8_boundary(text: &str, byte_index: usize) -> usize {
    let bytes = text.as_bytes();
    let mut idx = byte_index + 1;

    // Scan forward until we find a boundary
    while idx < text.len() {
        if is_utf8_char_boundary(bytes[idx]) {
            return idx;
        }
        idx += 1;
    }

    // Hit the end, return the full length
    text.len()
}

// Private helper function taken from Rust's core::num module (which is both Apache2 and MIT licensed)
const fn is_utf8_char_boundary(byte: u8) -> bool {
    // This is bit magic equivalent to: b < 128 || b >= 192
    (byte as i8) >= -0x40
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MultiBuffer,
        display_map::{HighlightKey, InlayHighlights, TextHighlights},
        hover_links::InlayHighlight,
    };
    use gpui::{App, HighlightStyle};
    use multi_buffer::Anchor;
    use project::{InlayHint, InlayHintLabel, ResolveState};
    use rand::prelude::*;
    use settings::SettingsStore;
    use std::{any::TypeId, cmp::Reverse, env, sync::Arc};
    use sum_tree::TreeMap;
    use text::{Patch, Rope};
    use util::RandomCharIter;
    use util::post_inc;

    #[test]
    fn test_inlay_properties_label_padding() {
        assert_eq!(
            Inlay::hint(
                InlayId::Hint(0),
                Anchor::min(),
                &InlayHint {
                    label: InlayHintLabel::String("a".to_string()),
                    position: text::Anchor::MIN,
                    padding_left: false,
                    padding_right: false,
                    tooltip: None,
                    kind: None,
                    resolve_state: ResolveState::Resolved,
                },
            )
            .text()
            .to_string(),
            "a",
            "Should not pad label if not requested"
        );

        assert_eq!(
            Inlay::hint(
                InlayId::Hint(0),
                Anchor::min(),
                &InlayHint {
                    label: InlayHintLabel::String("a".to_string()),
                    position: text::Anchor::MIN,
                    padding_left: true,
                    padding_right: true,
                    tooltip: None,
                    kind: None,
                    resolve_state: ResolveState::Resolved,
                },
            )
            .text()
            .to_string(),
            " a ",
            "Should pad label for every side requested"
        );

        assert_eq!(
            Inlay::hint(
                InlayId::Hint(0),
                Anchor::min(),
                &InlayHint {
                    label: InlayHintLabel::String(" a ".to_string()),
                    position: text::Anchor::MIN,
                    padding_left: false,
                    padding_right: false,
                    tooltip: None,
                    kind: None,
                    resolve_state: ResolveState::Resolved,
                },
            )
            .text()
            .to_string(),
            " a ",
            "Should not change already padded label"
        );

        assert_eq!(
            Inlay::hint(
                InlayId::Hint(0),
                Anchor::min(),
                &InlayHint {
                    label: InlayHintLabel::String(" a ".to_string()),
                    position: text::Anchor::MIN,
                    padding_left: true,
                    padding_right: true,
                    tooltip: None,
                    kind: None,
                    resolve_state: ResolveState::Resolved,
                },
            )
            .text()
            .to_string(),
            " a ",
            "Should not change already padded label"
        );
    }

    #[gpui::test]
    fn test_inlay_hint_padding_with_multibyte_chars() {
        assert_eq!(
            Inlay::hint(
                InlayId::Hint(0),
                Anchor::min(),
                &InlayHint {
                    label: InlayHintLabel::String("🎨".to_string()),
                    position: text::Anchor::MIN,
                    padding_left: true,
                    padding_right: true,
                    tooltip: None,
                    kind: None,
                    resolve_state: ResolveState::Resolved,
                },
            )
            .text()
            .to_string(),
            " 🎨 ",
            "Should pad single emoji correctly"
        );
    }

    #[gpui::test]
    fn test_basic_inlays(cx: &mut App) {
        let buffer = MultiBuffer::build_simple("abcdefghi", cx);
        let buffer_edits = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer.read(cx).snapshot(cx));
        assert_eq!(inlay_snapshot.text(), "abcdefghi");
        let mut next_inlay_id = 0;

        let (inlay_snapshot, _) = inlay_map.splice(
            &[],
            vec![Inlay::mock_hint(
                post_inc(&mut next_inlay_id),
                buffer.read(cx).snapshot(cx).anchor_after(3),
                "|123|",
            )],
        );
        assert_eq!(inlay_snapshot.text(), "abc|123|defghi");
        assert_eq!(
            inlay_snapshot.to_inlay_point(Point::new(0, 0)),
            InlayPoint::new(0, 0)
        );
        assert_eq!(
            inlay_snapshot.to_inlay_point(Point::new(0, 1)),
            InlayPoint::new(0, 1)
        );
        assert_eq!(
            inlay_snapshot.to_inlay_point(Point::new(0, 2)),
            InlayPoint::new(0, 2)
        );
        assert_eq!(
            inlay_snapshot.to_inlay_point(Point::new(0, 3)),
            InlayPoint::new(0, 3)
        );
        assert_eq!(
            inlay_snapshot.to_inlay_point(Point::new(0, 4)),
            InlayPoint::new(0, 9)
        );
        assert_eq!(
            inlay_snapshot.to_inlay_point(Point::new(0, 5)),
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
        let (inlay_snapshot, _) = inlay_map.sync(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        assert_eq!(inlay_snapshot.text(), "abxy|123|dzefghi");

        // An edit surrounding the inlay should invalidate it.
        buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "D")], None, cx));
        let (inlay_snapshot, _) = inlay_map.sync(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        assert_eq!(inlay_snapshot.text(), "abxyDzefghi");

        let (inlay_snapshot, _) = inlay_map.splice(
            &[],
            vec![
                Inlay::mock_hint(
                    post_inc(&mut next_inlay_id),
                    buffer.read(cx).snapshot(cx).anchor_before(3),
                    "|123|",
                ),
                Inlay::edit_prediction(
                    post_inc(&mut next_inlay_id),
                    buffer.read(cx).snapshot(cx).anchor_after(3),
                    "|456|",
                ),
            ],
        );
        assert_eq!(inlay_snapshot.text(), "abx|123||456|yDzefghi");

        // Edits ending where the inlay starts should not move it if it has a left bias.
        buffer.update(cx, |buffer, cx| buffer.edit([(3..3, "JKL")], None, cx));
        let (inlay_snapshot, _) = inlay_map.sync(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        assert_eq!(inlay_snapshot.text(), "abx|123|JKL|456|yDzefghi");

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 0), Bias::Left),
            InlayPoint::new(0, 0)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 0), Bias::Right),
            InlayPoint::new(0, 0)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 1), Bias::Left),
            InlayPoint::new(0, 1)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 1), Bias::Right),
            InlayPoint::new(0, 1)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 2), Bias::Left),
            InlayPoint::new(0, 2)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 2), Bias::Right),
            InlayPoint::new(0, 2)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 3), Bias::Left),
            InlayPoint::new(0, 2)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 3), Bias::Right),
            InlayPoint::new(0, 8)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 4), Bias::Left),
            InlayPoint::new(0, 2)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 4), Bias::Right),
            InlayPoint::new(0, 8)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 5), Bias::Left),
            InlayPoint::new(0, 2)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 5), Bias::Right),
            InlayPoint::new(0, 8)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 6), Bias::Left),
            InlayPoint::new(0, 2)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 6), Bias::Right),
            InlayPoint::new(0, 8)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 7), Bias::Left),
            InlayPoint::new(0, 2)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 7), Bias::Right),
            InlayPoint::new(0, 8)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 8), Bias::Left),
            InlayPoint::new(0, 8)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 8), Bias::Right),
            InlayPoint::new(0, 8)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 9), Bias::Left),
            InlayPoint::new(0, 9)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 9), Bias::Right),
            InlayPoint::new(0, 9)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 10), Bias::Left),
            InlayPoint::new(0, 10)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 10), Bias::Right),
            InlayPoint::new(0, 10)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 11), Bias::Left),
            InlayPoint::new(0, 11)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 11), Bias::Right),
            InlayPoint::new(0, 11)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 12), Bias::Left),
            InlayPoint::new(0, 11)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 12), Bias::Right),
            InlayPoint::new(0, 17)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 13), Bias::Left),
            InlayPoint::new(0, 11)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 13), Bias::Right),
            InlayPoint::new(0, 17)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 14), Bias::Left),
            InlayPoint::new(0, 11)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 14), Bias::Right),
            InlayPoint::new(0, 17)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 15), Bias::Left),
            InlayPoint::new(0, 11)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 15), Bias::Right),
            InlayPoint::new(0, 17)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 16), Bias::Left),
            InlayPoint::new(0, 11)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 16), Bias::Right),
            InlayPoint::new(0, 17)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 17), Bias::Left),
            InlayPoint::new(0, 17)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 17), Bias::Right),
            InlayPoint::new(0, 17)
        );

        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 18), Bias::Left),
            InlayPoint::new(0, 18)
        );
        assert_eq!(
            inlay_snapshot.clip_point(InlayPoint::new(0, 18), Bias::Right),
            InlayPoint::new(0, 18)
        );

        // The inlays can be manually removed.
        let (inlay_snapshot, _) = inlay_map.splice(
            &inlay_map
                .inlays
                .iter()
                .map(|inlay| inlay.id)
                .collect::<Vec<InlayId>>(),
            Vec::new(),
        );
        assert_eq!(inlay_snapshot.text(), "abxJKLyDzefghi");
    }

    #[gpui::test]
    fn test_inlay_buffer_rows(cx: &mut App) {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi", cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer.read(cx).snapshot(cx));
        assert_eq!(inlay_snapshot.text(), "abc\ndef\nghi");
        let mut next_inlay_id = 0;

        let (inlay_snapshot, _) = inlay_map.splice(
            &[],
            vec![
                Inlay::mock_hint(
                    post_inc(&mut next_inlay_id),
                    buffer.read(cx).snapshot(cx).anchor_before(0),
                    "|123|\n",
                ),
                Inlay::mock_hint(
                    post_inc(&mut next_inlay_id),
                    buffer.read(cx).snapshot(cx).anchor_before(4),
                    "|456|",
                ),
                Inlay::edit_prediction(
                    post_inc(&mut next_inlay_id),
                    buffer.read(cx).snapshot(cx).anchor_before(7),
                    "\n|567|\n",
                ),
            ],
        );
        assert_eq!(inlay_snapshot.text(), "|123|\nabc\n|456|def\n|567|\n\nghi");
        assert_eq!(
            inlay_snapshot
                .row_infos(0)
                .map(|info| info.buffer_row)
                .collect::<Vec<_>>(),
            vec![Some(0), None, Some(1), None, None, Some(2)]
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random_inlays(cx: &mut App, mut rng: StdRng) {
        init_test(cx);

        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let len = rng.random_range(0..30);
        let buffer = if rng.random() {
            let text = util::RandomCharIter::new(&mut rng)
                .take(len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };
        let mut buffer_snapshot = buffer.read(cx).snapshot(cx);
        let mut next_inlay_id = 0;
        log::info!("buffer text: {:?}", buffer_snapshot.text());
        let (mut inlay_map, mut inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        for _ in 0..operations {
            let mut inlay_edits = Patch::default();

            let mut prev_inlay_text = inlay_snapshot.text();
            let mut buffer_edits = Vec::new();
            match rng.random_range(0..=100) {
                0..=50 => {
                    let (snapshot, edits) = inlay_map.randomly_mutate(&mut next_inlay_id, &mut rng);
                    log::info!("mutated text: {:?}", snapshot.text());
                    inlay_edits = Patch::new(edits);
                }
                _ => buffer.update(cx, |buffer, cx| {
                    let subscription = buffer.subscribe();
                    let edit_count = rng.random_range(1..=5);
                    buffer.randomly_mutate(&mut rng, edit_count, cx);
                    buffer_snapshot = buffer.snapshot(cx);
                    let edits = subscription.consume().into_inner();
                    log::info!("editing {:?}", edits);
                    buffer_edits.extend(edits);
                }),
            };

            let (new_inlay_snapshot, new_inlay_edits) =
                inlay_map.sync(buffer_snapshot.clone(), buffer_edits);
            inlay_snapshot = new_inlay_snapshot;
            inlay_edits = inlay_edits.compose(new_inlay_edits);

            log::info!("buffer text: {:?}", buffer_snapshot.text());
            log::info!("inlay text: {:?}", inlay_snapshot.text());

            let inlays = inlay_map
                .inlays
                .iter()
                .filter(|inlay| inlay.position.is_valid(&buffer_snapshot))
                .map(|inlay| {
                    let offset = inlay.position.to_offset(&buffer_snapshot);
                    (offset, inlay.clone())
                })
                .collect::<Vec<_>>();
            let mut expected_text = Rope::from(&buffer_snapshot.text());
            for (offset, inlay) in inlays.iter().rev() {
                expected_text.replace(*offset..*offset, &inlay.text().to_string());
            }
            assert_eq!(inlay_snapshot.text(), expected_text.to_string());

            let expected_buffer_rows = inlay_snapshot.row_infos(0).collect::<Vec<_>>();
            assert_eq!(
                expected_buffer_rows.len() as u32,
                expected_text.max_point().row + 1
            );
            for row_start in 0..expected_buffer_rows.len() {
                assert_eq!(
                    inlay_snapshot
                        .row_infos(row_start as u32)
                        .collect::<Vec<_>>(),
                    &expected_buffer_rows[row_start..],
                    "incorrect buffer rows starting at {}",
                    row_start
                );
            }

            let mut text_highlights = TextHighlights::default();
            let text_highlight_count = rng.random_range(0_usize..10);
            let mut text_highlight_ranges = (0..text_highlight_count)
                .map(|_| buffer_snapshot.random_byte_range(0, &mut rng))
                .collect::<Vec<_>>();
            text_highlight_ranges.sort_by_key(|range| (range.start, Reverse(range.end)));
            log::info!("highlighting text ranges {text_highlight_ranges:?}");
            text_highlights.insert(
                HighlightKey::Type(TypeId::of::<()>()),
                Arc::new((
                    HighlightStyle::default(),
                    text_highlight_ranges
                        .into_iter()
                        .map(|range| {
                            buffer_snapshot.anchor_before(range.start)
                                ..buffer_snapshot.anchor_after(range.end)
                        })
                        .collect(),
                )),
            );

            let mut inlay_highlights = InlayHighlights::default();
            if !inlays.is_empty() {
                let inlay_highlight_count = rng.random_range(0..inlays.len());
                let mut inlay_indices = BTreeSet::default();
                while inlay_indices.len() < inlay_highlight_count {
                    inlay_indices.insert(rng.random_range(0..inlays.len()));
                }
                let new_highlights = TreeMap::from_ordered_entries(
                    inlay_indices
                        .into_iter()
                        .filter_map(|i| {
                            let (_, inlay) = &inlays[i];
                            let inlay_text_len = inlay.text().len();
                            match inlay_text_len {
                                0 => None,
                                1 => Some(InlayHighlight {
                                    inlay: inlay.id,
                                    inlay_position: inlay.position,
                                    range: 0..1,
                                }),
                                n => {
                                    let inlay_text = inlay.text().to_string();
                                    let mut highlight_end = rng.random_range(1..n);
                                    let mut highlight_start = rng.random_range(0..highlight_end);
                                    while !inlay_text.is_char_boundary(highlight_end) {
                                        highlight_end += 1;
                                    }
                                    while !inlay_text.is_char_boundary(highlight_start) {
                                        highlight_start -= 1;
                                    }
                                    Some(InlayHighlight {
                                        inlay: inlay.id,
                                        inlay_position: inlay.position,
                                        range: highlight_start..highlight_end,
                                    })
                                }
                            }
                        })
                        .map(|highlight| (highlight.inlay, (HighlightStyle::default(), highlight))),
                );
                log::info!("highlighting inlay ranges {new_highlights:?}");
                inlay_highlights.insert(TypeId::of::<()>(), new_highlights);
            }

            for _ in 0..5 {
                let mut end = rng.random_range(0..=inlay_snapshot.len().0);
                end = expected_text.clip_offset(end, Bias::Right);
                let mut start = rng.random_range(0..=end);
                start = expected_text.clip_offset(start, Bias::Right);

                let range = InlayOffset(start)..InlayOffset(end);
                log::info!("calling inlay_snapshot.chunks({range:?})");
                let actual_text = inlay_snapshot
                    .chunks(
                        range,
                        false,
                        Highlights {
                            text_highlights: Some(&text_highlights),
                            inlay_highlights: Some(&inlay_highlights),
                            ..Highlights::default()
                        },
                    )
                    .map(|chunk| chunk.chunk.text)
                    .collect::<String>();
                assert_eq!(
                    actual_text,
                    expected_text.slice(start..end).to_string(),
                    "incorrect text in range {:?}",
                    start..end
                );

                assert_eq!(
                    inlay_snapshot.text_summary_for_range(InlayOffset(start)..InlayOffset(end)),
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

            let mut buffer_point = Point::default();
            let mut inlay_point = inlay_snapshot.to_inlay_point(buffer_point);
            let mut buffer_chars = buffer_snapshot.chars_at(0);
            loop {
                // Ensure conversion from buffer coordinates to inlay coordinates
                // is consistent.
                let buffer_offset = buffer_snapshot.point_to_offset(buffer_point);
                assert_eq!(
                    inlay_snapshot.to_point(inlay_snapshot.to_inlay_offset(buffer_offset)),
                    inlay_point
                );

                // No matter which bias we clip an inlay point with, it doesn't move
                // because it was constructed from a buffer point.
                assert_eq!(
                    inlay_snapshot.clip_point(inlay_point, Bias::Left),
                    inlay_point,
                    "invalid inlay point for buffer point {:?} when clipped left",
                    buffer_point
                );
                assert_eq!(
                    inlay_snapshot.clip_point(inlay_point, Bias::Right),
                    inlay_point,
                    "invalid inlay point for buffer point {:?} when clipped right",
                    buffer_point
                );

                if let Some(ch) = buffer_chars.next() {
                    if ch == '\n' {
                        buffer_point += Point::new(1, 0);
                    } else {
                        buffer_point += Point::new(0, ch.len_utf8() as u32);
                    }

                    // Ensure that moving forward in the buffer always moves the inlay point forward as well.
                    let new_inlay_point = inlay_snapshot.to_inlay_point(buffer_point);
                    assert!(new_inlay_point > inlay_point);
                    inlay_point = new_inlay_point;
                } else {
                    break;
                }
            }

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
                        "inlay point {:?} when clipped left is greater than when clipped right ({:?} > {:?})",
                        inlay_point,
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

                    // Ensure the clipped points are at valid buffer locations.
                    assert_eq!(
                        inlay_snapshot
                            .to_inlay_point(inlay_snapshot.to_buffer_point(clipped_left_point)),
                        clipped_left_point,
                        "to_buffer_point({:?}) = {:?}",
                        clipped_left_point,
                        inlay_snapshot.to_buffer_point(clipped_left_point),
                    );
                    assert_eq!(
                        inlay_snapshot
                            .to_inlay_point(inlay_snapshot.to_buffer_point(clipped_right_point)),
                        clipped_right_point,
                        "to_buffer_point({:?}) = {:?}",
                        clipped_right_point,
                        inlay_snapshot.to_buffer_point(clipped_right_point),
                    );
                }
            }
        }
    }

    #[gpui::test(iterations = 100)]
    fn test_random_chunk_bitmaps(cx: &mut gpui::App, mut rng: StdRng) {
        init_test(cx);

        // Generate random buffer using existing test infrastructure
        let text_len = rng.random_range(0..10000);
        let buffer = if rng.random() {
            let text = RandomCharIter::new(&mut rng)
                .take(text_len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };

        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, _) = InlayMap::new(buffer_snapshot.clone());

        // Perform random mutations to add inlays
        let mut next_inlay_id = 0;
        let mutation_count = rng.random_range(1..10);
        for _ in 0..mutation_count {
            inlay_map.randomly_mutate(&mut next_inlay_id, &mut rng);
        }

        let (snapshot, _) = inlay_map.sync(buffer_snapshot, vec![]);

        // Get all chunks and verify their bitmaps
        let chunks = snapshot.chunks(
            InlayOffset(0)..InlayOffset(snapshot.len().0),
            false,
            Highlights::default(),
        );

        for chunk in chunks.into_iter().map(|inlay_chunk| inlay_chunk.chunk) {
            let chunk_text = chunk.text;
            let chars_bitmap = chunk.chars;
            let tabs_bitmap = chunk.tabs;

            // Check empty chunks have empty bitmaps
            if chunk_text.is_empty() {
                assert_eq!(
                    chars_bitmap, 0,
                    "Empty chunk should have empty chars bitmap"
                );
                assert_eq!(tabs_bitmap, 0, "Empty chunk should have empty tabs bitmap");
                continue;
            }

            // Verify that chunk text doesn't exceed 128 bytes
            assert!(
                chunk_text.len() <= 128,
                "Chunk text length {} exceeds 128 bytes",
                chunk_text.len()
            );

            // Verify chars bitmap
            let char_indices = chunk_text
                .char_indices()
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            for byte_idx in 0..chunk_text.len() {
                let should_have_bit = char_indices.contains(&byte_idx);
                let has_bit = chars_bitmap & (1 << byte_idx) != 0;

                if has_bit != should_have_bit {
                    eprintln!("Chunk text bytes: {:?}", chunk_text.as_bytes());
                    eprintln!("Char indices: {:?}", char_indices);
                    eprintln!("Chars bitmap: {:#b}", chars_bitmap);
                    assert_eq!(
                        has_bit, should_have_bit,
                        "Chars bitmap mismatch at byte index {} in chunk {:?}. Expected bit: {}, Got bit: {}",
                        byte_idx, chunk_text, should_have_bit, has_bit
                    );
                }
            }

            // Verify tabs bitmap
            for (byte_idx, byte) in chunk_text.bytes().enumerate() {
                let is_tab = byte == b'\t';
                let has_bit = tabs_bitmap & (1 << byte_idx) != 0;

                if has_bit != is_tab {
                    eprintln!("Chunk text bytes: {:?}", chunk_text.as_bytes());
                    eprintln!("Tabs bitmap: {:#b}", tabs_bitmap);
                    assert_eq!(
                        has_bit, is_tab,
                        "Tabs bitmap mismatch at byte index {} in chunk {:?}. Byte: {:?}, Expected bit: {}, Got bit: {}",
                        byte_idx, chunk_text, byte as char, is_tab, has_bit
                    );
                }
            }
        }
    }

    fn init_test(cx: &mut App) {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        theme::init(theme::LoadThemes::JustBase, cx);
    }

    /// Helper to create test highlights for an inlay
    fn create_inlay_highlights(
        inlay_id: InlayId,
        highlight_range: Range<usize>,
        position: Anchor,
    ) -> TreeMap<TypeId, TreeMap<InlayId, (HighlightStyle, InlayHighlight)>> {
        let mut inlay_highlights = TreeMap::default();
        let mut type_highlights = TreeMap::default();
        type_highlights.insert(
            inlay_id,
            (
                HighlightStyle::default(),
                InlayHighlight {
                    inlay: inlay_id,
                    range: highlight_range,
                    inlay_position: position,
                },
            ),
        );
        inlay_highlights.insert(TypeId::of::<()>(), type_highlights);
        inlay_highlights
    }

    #[gpui::test]
    fn test_inlay_utf8_boundary_panic_fix(cx: &mut App) {
        init_test(cx);

        // This test verifies that we handle UTF-8 character boundaries correctly
        // when splitting inlay text for highlighting. Previously, this would panic
        // when trying to split at byte 13, which is in the middle of the '…' character.
        //
        // See https://github.com/zed-industries/zed/issues/33641
        let buffer = MultiBuffer::build_simple("fn main() {}\n", cx);
        let (mut inlay_map, _) = InlayMap::new(buffer.read(cx).snapshot(cx));

        // Create an inlay with text that contains a multi-byte character
        // The string "SortingDirec…" contains an ellipsis character '…' which is 3 bytes (E2 80 A6)
        let inlay_text = "SortingDirec…";
        let position = buffer.read(cx).snapshot(cx).anchor_before(Point::new(0, 5));

        let inlay = Inlay {
            id: InlayId::Hint(0),
            position,
            content: InlayContent::Text(text::Rope::from(inlay_text)),
        };

        let (inlay_snapshot, _) = inlay_map.splice(&[], vec![inlay]);

        // Create highlights that request a split at byte 13, which is in the middle
        // of the '…' character (bytes 12..15). We include the full character.
        let inlay_highlights = create_inlay_highlights(InlayId::Hint(0), 0..13, position);

        let highlights = crate::display_map::Highlights {
            text_highlights: None,
            inlay_highlights: Some(&inlay_highlights),
            styles: crate::display_map::HighlightStyles::default(),
        };

        // Collect chunks - this previously would panic
        let chunks: Vec<_> = inlay_snapshot
            .chunks(
                InlayOffset(0)..InlayOffset(inlay_snapshot.len().0),
                false,
                highlights,
            )
            .collect();

        // Verify the chunks are correct
        let full_text: String = chunks.iter().map(|c| c.chunk.text).collect();
        assert_eq!(full_text, "fn maSortingDirec…in() {}\n");

        // Verify the highlighted portion includes the complete ellipsis character
        let highlighted_chunks: Vec<_> = chunks
            .iter()
            .filter(|c| c.chunk.highlight_style.is_some() && c.chunk.is_inlay)
            .collect();

        assert_eq!(highlighted_chunks.len(), 1);
        assert_eq!(highlighted_chunks[0].chunk.text, "SortingDirec…");
    }

    #[gpui::test]
    fn test_inlay_utf8_boundaries(cx: &mut App) {
        init_test(cx);

        struct TestCase {
            inlay_text: &'static str,
            highlight_range: Range<usize>,
            expected_highlighted: &'static str,
            description: &'static str,
        }

        let test_cases = vec![
            TestCase {
                inlay_text: "Hello👋World",
                highlight_range: 0..7,
                expected_highlighted: "Hello👋",
                description: "Emoji boundary - rounds up to include full emoji",
            },
            TestCase {
                inlay_text: "Test→End",
                highlight_range: 0..5,
                expected_highlighted: "Test→",
                description: "Arrow boundary - rounds up to include full arrow",
            },
            TestCase {
                inlay_text: "café",
                highlight_range: 0..4,
                expected_highlighted: "café",
                description: "Accented char boundary - rounds up to include full é",
            },
            TestCase {
                inlay_text: "🎨🎭🎪",
                highlight_range: 0..5,
                expected_highlighted: "🎨🎭",
                description: "Multiple emojis - partial highlight",
            },
            TestCase {
                inlay_text: "普通话",
                highlight_range: 0..4,
                expected_highlighted: "普通",
                description: "Chinese characters - partial highlight",
            },
            TestCase {
                inlay_text: "Hello",
                highlight_range: 0..2,
                expected_highlighted: "He",
                description: "ASCII only - no adjustment needed",
            },
            TestCase {
                inlay_text: "👋",
                highlight_range: 0..1,
                expected_highlighted: "👋",
                description: "Single emoji - partial byte range includes whole char",
            },
            TestCase {
                inlay_text: "Test",
                highlight_range: 0..0,
                expected_highlighted: "",
                description: "Empty range",
            },
            TestCase {
                inlay_text: "🎨ABC",
                highlight_range: 2..5,
                expected_highlighted: "A",
                description: "Range starting mid-emoji skips the emoji",
            },
        ];

        for test_case in test_cases {
            let buffer = MultiBuffer::build_simple("test", cx);
            let (mut inlay_map, _) = InlayMap::new(buffer.read(cx).snapshot(cx));
            let position = buffer.read(cx).snapshot(cx).anchor_before(Point::new(0, 2));

            let inlay = Inlay {
                id: InlayId::Hint(0),
                position,
                content: InlayContent::Text(text::Rope::from(test_case.inlay_text)),
            };

            let (inlay_snapshot, _) = inlay_map.splice(&[], vec![inlay]);
            let inlay_highlights = create_inlay_highlights(
                InlayId::Hint(0),
                test_case.highlight_range.clone(),
                position,
            );

            let highlights = crate::display_map::Highlights {
                text_highlights: None,
                inlay_highlights: Some(&inlay_highlights),
                styles: crate::display_map::HighlightStyles::default(),
            };

            let chunks: Vec<_> = inlay_snapshot
                .chunks(
                    InlayOffset(0)..InlayOffset(inlay_snapshot.len().0),
                    false,
                    highlights,
                )
                .collect();

            // Verify we got chunks and they total to the expected text
            let full_text: String = chunks.iter().map(|c| c.chunk.text).collect();
            assert_eq!(
                full_text,
                format!("te{}st", test_case.inlay_text),
                "Full text mismatch for case: {}",
                test_case.description
            );

            // Verify that the highlighted portion matches expectations
            let highlighted_text: String = chunks
                .iter()
                .filter(|c| c.chunk.highlight_style.is_some() && c.chunk.is_inlay)
                .map(|c| c.chunk.text)
                .collect();
            assert_eq!(
                highlighted_text, test_case.expected_highlighted,
                "Highlighted text mismatch for case: {} (text: '{}', range: {:?})",
                test_case.description, test_case.inlay_text, test_case.highlight_range
            );
        }
    }
}
