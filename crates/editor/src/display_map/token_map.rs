use gpui::HighlightStyle;
use itertools::Itertools;
use language::{Chunk, Edit, Point, TextSummary};
use multi_buffer::{AnchorRangeExt, MultiBufferSnapshot};
use multi_buffer::{MultiBufferRow, MultiBufferRows, RowInfo, ToOffset};
use std::cmp;
use std::ops::{Add, AddAssign, Range, Sub, SubAssign};
use sum_tree::{Bias, Cursor, SumTree};
use text::Patch;

use super::{custom_highlights::CustomHighlightsChunks, Highlights};

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) id: usize,
    pub range: Range<multi_buffer::Anchor>,
    pub style: HighlightStyle,
    pub text: text::Rope,
}

impl Token {
    pub fn new<T: Into<text::Rope>>(
        id: usize,
        range: Range<multi_buffer::Anchor>,
        style: HighlightStyle,
        text: T,
    ) -> Self {
        Self {
            id,
            range,
            style,
            text: text.into(),
        }
    }
}

/// Decides where the [`Token`]s should be displayed.
///
/// See the [`display_map` module documentation](crate::display_map) for more information.
pub struct TokenMap {
    snapshot: TokenSnapshot,
    tokens: Vec<Token>,
}

#[derive(Clone)]
pub struct TokenSnapshot {
    pub buffer: MultiBufferSnapshot,
    transforms: SumTree<Transform>,
    pub version: usize,
}

#[derive(Clone, Debug)]
enum Transform {
    Isomorphic(TextSummary),
    Highlight(Token, TextSummary),
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self, _: &()) -> Self::Summary {
        match self {
            Transform::Isomorphic(summary) => TransformSummary {
                input: *summary,
                output: *summary,
            },
            Transform::Highlight(_, summary) => TransformSummary {
                input: *summary,
                output: *summary,
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

    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, other: &Self, _: &()) {
        self.input += &other.input;
        self.output += &other.output;
    }
}

pub type TokenEdit = Edit<TokenOffset>;

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct TokenOffset(pub usize);

impl Add for TokenOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for TokenOffset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign for TokenOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for TokenOffset {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for TokenOffset {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.output.len;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct TokenPoint(pub Point);

impl<'a> sum_tree::Dimension<'a, TransformSummary> for TokenPoint {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.output.lines;
    }
}

impl Add for TokenPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for TokenPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for usize {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += &summary.input.len;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for Point {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += &summary.input.lines;
    }
}

#[derive(Clone)]
pub struct TokenBufferRows<'a> {
    transforms: Cursor<'a, Transform, (TokenPoint, Point)>,
    buffer_rows: MultiBufferRows<'a>,
    token_row: u32,
    max_buffer_row: MultiBufferRow,
}

pub struct TokenChunks<'a> {
    transforms: Cursor<'a, Transform, (TokenOffset, usize)>,
    buffer_chunks: CustomHighlightsChunks<'a>,
    buffer_chunk: Option<Chunk<'a>>,
    token_chunk: Option<&'a str>,
    token_chunks: Option<text::Chunks<'a>>,
    output_offset: TokenOffset,
    max_output_offset: TokenOffset,
    snapshot: &'a TokenSnapshot,
}

impl TokenChunks<'_> {
    pub fn seek(&mut self, new_range: Range<TokenOffset>) {
        self.transforms.seek(&new_range.start, Bias::Right, &());

        let buffer_range = self.snapshot.to_buffer_offset(new_range.start)
            ..self.snapshot.to_buffer_offset(new_range.end);
        self.buffer_chunks.seek(buffer_range);
        self.token_chunks = None;
        self.buffer_chunk = None;
        self.output_offset = new_range.start;
        self.max_output_offset = new_range.end;
    }

    pub fn offset(&self) -> TokenOffset {
        self.output_offset
    }
}

impl<'a> Iterator for TokenChunks<'a> {
    type Item = Chunk<'a>;

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

                let (prefix, suffix) = chunk.text.split_at(
                    chunk
                        .text
                        .len()
                        .min(self.transforms.end(&()).0 .0 - self.output_offset.0),
                );

                chunk.text = suffix;
                self.output_offset.0 += prefix.len();
                Chunk {
                    text: prefix,
                    ..chunk.clone()
                }
            }
            Transform::Highlight(token, _) => {
                let chunk = self
                    .buffer_chunk
                    .get_or_insert_with(|| self.buffer_chunks.next().unwrap());
                if chunk.text.is_empty() {
                    *chunk = self.buffer_chunks.next().unwrap();
                }

                let (prefix, suffix) = chunk.text.split_at(
                    chunk
                        .text
                        .len()
                        .min(self.transforms.end(&()).0 .0 - self.output_offset.0),
                );

                chunk.text = suffix;
                self.output_offset.0 += prefix.len();
                // let range = token.range.to_offset(&self.snapshot.buffer);
                // let offset_in_token = self.output_offset - self.transforms.start().0;
                // let next_endpoint = if offset_in_token.0 < range.start {
                //     range.start - offset_in_token.0
                // } else if offset_in_token.0 > range.end {
                //     log::error!("fail");
                //     usize::MAX
                // } else {
                //     range.end - offset_in_token.0
                // };
                // let token_chunks = self.token_chunks.get_or_insert_with(|| {
                //     let start = offset_in_token;
                //     let end = cmp::min(self.max_output_offset, self.transforms.end(&()).0)
                //         - self.transforms.start().0;
                //     log::error!("{}", token.text);
                //     token.text.chunks_in_range(start.0..end.0)
                // });
                // let token_chunk = self
                //     .token_chunk
                //     .get_or_insert_with(|| token_chunks.next().unwrap());
                // let (chunk, remainder) = token_chunk.split_at(token_chunk.len().min(next_endpoint));
                // *token_chunk = remainder;
                // if token_chunk.is_empty() {
                //     self.token_chunk = None;
                // }

                // self.output_offset.0 += chunk.len();

                // let (prefix, suffix) = chunk.text.split_at(
                //     chunk
                //         .text
                //         .len()
                //         .min(self.transforms.end(&()).0 .0 - self.output_offset.0),
                // );

                // chunk.text = suffix;
                // self.output_offset.0 += prefix.len();

                Chunk {
                    text: prefix,
                    syntax_highlight_id: None,
                    highlight_style: Some(token.style),
                    ..Default::default()
                }
            }
        };

        if self.output_offset == self.transforms.end(&()).0 {
            self.token_chunks = None;
            self.transforms.next(&());
        }

        Some(chunk)
    }
}

impl TokenBufferRows<'_> {
    pub fn seek(&mut self, row: u32) {
        let token_point = TokenPoint::new(row, 0);
        self.transforms.seek(&token_point, Bias::Left, &());

        let mut buffer_point = self.transforms.start().1;
        let buffer_row = MultiBufferRow(if row == 0 {
            0
        } else {
            match self.transforms.item() {
                Some(Transform::Isomorphic(_)) => {
                    buffer_point += token_point.0 - self.transforms.start().0 .0;
                    buffer_point.row
                }
                Some(Transform::Highlight(_, _)) => {
                    buffer_point += token_point.0 - self.transforms.start().0 .0;
                    buffer_point.row
                }
                _ => cmp::min(buffer_point.row + 1, self.max_buffer_row.0),
            }
        });
        self.token_row = token_point.row();
        self.buffer_rows.seek(buffer_row);
    }
}

impl Iterator for TokenBufferRows<'_> {
    type Item = RowInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let buffer_row = if self.token_row == 0 {
            self.buffer_rows.next().unwrap()
        } else {
            self.transforms.item()?;
            self.buffer_rows.next().unwrap()
        };

        self.token_row += 1;
        self.transforms
            .seek_forward(&TokenPoint::new(self.token_row, 0), Bias::Left, &());

        Some(buffer_row)
    }
}

impl TokenPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(Point::new(row, column))
    }

    pub fn row(self) -> u32 {
        self.0.row
    }
}

impl TokenMap {
    pub fn new(buffer: MultiBufferSnapshot) -> (Self, TokenSnapshot) {
        let version = 0;
        let snapshot = TokenSnapshot {
            buffer: buffer.clone(),
            transforms: SumTree::from_iter(Some(Transform::Isomorphic(buffer.text_summary())), &()),
            version,
        };

        (
            Self {
                snapshot: snapshot.clone(),
                tokens: Vec::new(),
            },
            snapshot,
        )
    }

    pub fn sync(
        &mut self,
        buffer_snapshot: MultiBufferSnapshot,
        mut buffer_edits: Vec<text::Edit<usize>>,
    ) -> (TokenSnapshot, Vec<TokenEdit>) {
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
            let mut token_edits = Patch::default();
            let mut new_transforms = SumTree::default();
            let mut cursor = snapshot.transforms.cursor::<(usize, TokenOffset)>(&());
            let mut buffer_edits_iter = buffer_edits.iter().peekable();
            while let Some(buffer_edit) = buffer_edits_iter.next() {
                new_transforms.append(cursor.slice(&buffer_edit.old.start, Bias::Left, &()), &());
                if let Some(Transform::Isomorphic(transform)) = cursor.item() {
                    if cursor.end(&()).0 == buffer_edit.old.start {
                        push_isomorphic(&mut new_transforms, *transform);
                        cursor.next(&());
                    }
                }

                // Remove all the tokens and transforms contained by the edit.
                let old_start =
                    cursor.start().1 + TokenOffset(buffer_edit.old.start - cursor.start().0);
                // cursor.seek(&buffer_edit.old.end, Bias::Right, &());
                let old_end =
                    cursor.start().1 + TokenOffset(buffer_edit.old.end - cursor.start().0);

                // Push the unchanged prefix with highlights.
                let prefix_start = new_transforms.summary().input.len;
                let prefix_end = buffer_edit.new.start;
                push_semantic_tokens(
                    &self.tokens,
                    &buffer_snapshot,
                    &mut new_transforms,
                    prefix_start..prefix_end,
                );

                // Apply the rest of the edit.
                let new_start = TokenOffset(new_transforms.summary().output.len);
                let transform_start = new_transforms.summary().input.len;
                push_isomorphic(
                    &mut new_transforms,
                    buffer_snapshot.text_summary_for_range(transform_start..buffer_edit.new.end),
                );
                let new_end = TokenOffset(new_transforms.summary().output.len);
                token_edits.push(Edit {
                    old: old_start..old_end,
                    new: new_start..new_end,
                });

                // If the next edit doesn't intersect the current isomorphic transform, then
                // we can push its remainder.
                if buffer_edits_iter
                    .peek()
                    .map_or(true, |edit| edit.old.start >= cursor.end(&()).0)
                {
                    let transform_start = new_transforms.summary().input.len;
                    let transform_end =
                        buffer_edit.new.end + (cursor.end(&()).0 - buffer_edit.old.end);
                    push_isomorphic(
                        &mut new_transforms,
                        buffer_snapshot.text_summary_for_range(transform_start..transform_end),
                    );
                    cursor.next(&());
                }
            }

            new_transforms.append(cursor.suffix(&()), &());
            if new_transforms.is_empty() {
                new_transforms.push(Transform::Isomorphic(Default::default()), &());
            }

            drop(cursor);
            snapshot.transforms = new_transforms;
            snapshot.version += 1;
            snapshot.buffer = buffer_snapshot;
            snapshot.check_invariants();

            (snapshot.clone(), token_edits.into_inner())
        }
    }

    pub fn splice(
        &mut self,
        to_remove: &[usize],
        to_insert: Vec<Token>,
    ) -> (TokenSnapshot, Vec<TokenEdit>) {
        let snapshot = &mut self.snapshot;
        let mut buffer_edits = vec![];

        self.tokens.retain(|token| {
            let retain = !to_remove.contains(&token.id);
            if !retain {
                buffer_edits.push(Edit {
                    old: token.range.to_offset(&snapshot.buffer),
                    new: token.range.to_offset(&snapshot.buffer),
                })
            }
            retain
        });

        for token_to_insert in to_insert {
            buffer_edits.push(Edit {
                old: token_to_insert.range.to_offset(&snapshot.buffer),
                new: token_to_insert.range.to_offset(&snapshot.buffer),
            });
            let (Ok(ix) | Err(ix)) = self.tokens.binary_search_by(|probe| {
                probe
                    .range
                    .start
                    .cmp(&token_to_insert.range.start, &snapshot.buffer)
                    .then(std::cmp::Ordering::Less)
            });
            self.tokens.insert(ix, token_to_insert);
        }

        let buffer_edits = buffer_edits
            .into_iter()
            .sorted_by(|a, b| a.old.start.cmp(&b.old.start).then(cmp::Ordering::Greater))
            .collect_vec();

        let buffer_snapshot = snapshot.buffer.clone();
        let (snapshot, edits) = self.sync(buffer_snapshot, buffer_edits);
        (snapshot, edits)
    }

    pub fn current_tokens(&self) -> impl Iterator<Item = &Token> {
        self.tokens.iter()
    }
}

impl TokenSnapshot {
    pub fn to_point(&self, offset: TokenOffset) -> TokenPoint {
        let mut cursor = self
            .transforms
            .cursor::<(TokenOffset, (TokenPoint, usize))>(&());
        cursor.seek(&offset, Bias::Right, &());
        let overshoot = offset.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let buffer_offset_start = cursor.start().1 .1;
                let buffer_offset_end = buffer_offset_start + overshoot;
                let buffer_start = self.buffer.offset_to_point(buffer_offset_start);
                let buffer_end = self.buffer.offset_to_point(buffer_offset_end);
                TokenPoint(cursor.start().1 .0 .0 + (buffer_end - buffer_start))
            }
            Some(Transform::Highlight(_, _)) => {
                let buffer_offset_start = cursor.start().1 .1;
                let buffer_offset_end = buffer_offset_start + overshoot;
                let buffer_start = self.buffer.offset_to_point(buffer_offset_start);
                let buffer_end = self.buffer.offset_to_point(buffer_offset_end);
                TokenPoint(cursor.start().1 .0 .0 + (buffer_end - buffer_start))
            }
            None => self.max_point(),
        }
    }

    pub fn len(&self) -> TokenOffset {
        TokenOffset(self.transforms.summary().output.len)
    }

    pub fn max_row(&self) -> u32 {
        self.buffer.max_row().0
    }

    pub fn max_point(&self) -> TokenPoint {
        TokenPoint(self.transforms.summary().output.lines)
    }

    pub fn to_offset(&self, point: TokenPoint) -> TokenOffset {
        let mut cursor = self
            .transforms
            .cursor::<(TokenPoint, (TokenOffset, Point))>(&());
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let buffer_point_start = cursor.start().1 .1;
                let buffer_point_end = buffer_point_start + overshoot;
                let buffer_offset_start = self.buffer.point_to_offset(buffer_point_start);
                let buffer_offset_end = self.buffer.point_to_offset(buffer_point_end); // TODO: check here
                TokenOffset(cursor.start().1 .0 .0 + (buffer_offset_end - buffer_offset_start))
            }
            Some(Transform::Highlight(_, _)) => {
                let buffer_point_start = cursor.start().1 .1;
                let buffer_point_end = buffer_point_start + overshoot;
                let buffer_offset_start = self.buffer.point_to_offset(buffer_point_start);
                let buffer_offset_end = self.buffer.point_to_offset(buffer_point_end);
                TokenOffset(cursor.start().1 .0 .0 + (buffer_offset_end - buffer_offset_start))
            }
            None => self.len(),
        }
    }

    pub fn to_buffer_point(&self, point: TokenPoint) -> Point {
        let mut cursor = self.transforms.cursor::<(TokenPoint, Point)>(&());
        cursor.seek(&point, Bias::Right, &());
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let overshoot = point.0 - cursor.start().0 .0;
                cursor.start().1 + overshoot
            }
            Some(Transform::Highlight(_, _)) => {
                let overshoot = point.0 - cursor.start().0 .0;
                cursor.start().1 + overshoot
            }
            None => self.buffer.max_point(),
        }
    }

    pub fn to_buffer_offset(&self, offset: TokenOffset) -> usize {
        let mut cursor = self.transforms.cursor::<(TokenOffset, usize)>(&());
        cursor.seek(&offset, Bias::Right, &());
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let overshoot = offset - cursor.start().0;
                cursor.start().1 + overshoot.0
            }
            Some(Transform::Highlight(_, _)) => {
                let overshoot = offset - cursor.start().0;
                cursor.start().1 + overshoot.0
            }
            None => self.buffer.len(),
        }
    }

    pub fn to_token_offset(&self, offset: usize) -> TokenOffset {
        let mut cursor = self.transforms.cursor::<(usize, TokenOffset)>(&());
        cursor.seek(&offset, Bias::Left, &());
        loop {
            match cursor.item() {
                Some(Transform::Isomorphic(_)) => {
                    if offset == cursor.end(&()).0 {
                        return cursor.end(&()).1;
                    } else {
                        let overshoot = offset - cursor.start().0;
                        return TokenOffset(cursor.start().1 .0 + overshoot);
                    }
                }
                Some(Transform::Highlight(_, _)) => {
                    if offset == cursor.end(&()).0 {
                        return cursor.end(&()).1;
                    } else {
                        let overshoot = offset - cursor.start().0;
                        return TokenOffset(cursor.start().1 .0 + overshoot);
                    }
                }
                None => {
                    return self.len();
                }
            }
        }
    }
    pub fn to_token_point(&self, point: Point) -> TokenPoint {
        let mut cursor = self.transforms.cursor::<(Point, TokenPoint)>(&());
        cursor.seek(&point, Bias::Left, &());
        loop {
            match cursor.item() {
                Some(Transform::Isomorphic(_)) => {
                    if point == cursor.end(&()).0 {
                        return cursor.end(&()).1;
                    } else {
                        let overshoot = point - cursor.start().0;
                        return TokenPoint(cursor.start().1 .0 + overshoot);
                    }
                }
                Some(Transform::Highlight(_, _)) => {
                    if point == cursor.end(&()).0 {
                        return cursor.end(&()).1;
                    } else {
                        let overshoot = point - cursor.start().0;
                        return TokenPoint(cursor.start().1 .0 + overshoot);
                    }
                }
                None => {
                    return self.max_point();
                }
            }
        }
    }

    pub fn clip_point(&self, mut point: TokenPoint, mut bias: Bias) -> TokenPoint {
        let mut cursor = self.transforms.cursor::<(TokenPoint, Point)>(&());
        cursor.seek(&point, Bias::Left, &());
        loop {
            match cursor.item() {
                Some(Transform::Isomorphic(_)) => {
                    if cursor.start().0 == point {
                        return point;
                    } else if cursor.end(&()).0 == point {
                        return point;
                    } else {
                        let overshoot = point.0 - cursor.start().0 .0;
                        let buffer_point = cursor.start().1 + overshoot;
                        let clipped_buffer_point = self.buffer.clip_point(buffer_point, bias);
                        let clipped_overshoot = clipped_buffer_point - cursor.start().1;
                        let clipped_point = TokenPoint(cursor.start().0 .0 + clipped_overshoot);
                        if clipped_point == point {
                            return clipped_point;
                        } else {
                            point = clipped_point;
                        }
                    }
                }
                Some(Transform::Highlight(_, _)) => {
                    if cursor.start().0 == point {
                        return point;
                    } else if cursor.end(&()).0 == point {
                        return point;
                    } else {
                        let overshoot = point.0 - cursor.start().0 .0;
                        let buffer_point = cursor.start().1 + overshoot;
                        let clipped_buffer_point = self.buffer.clip_point(buffer_point, bias);
                        let clipped_overshoot = clipped_buffer_point - cursor.start().1;
                        let clipped_point = TokenPoint(cursor.start().0 .0 + clipped_overshoot);
                        if clipped_point == point {
                            return clipped_point;
                        } else {
                            point = clipped_point;
                        }
                    }
                }
                None => {
                    bias = bias.invert();
                    if bias == Bias::Left {
                        point = cursor.start().0;
                        cursor.prev(&());
                    } else {
                        cursor.next(&());
                        point = cursor.start().0;
                    }
                }
            }
        }
    }

    pub fn text_summary(&self) -> TextSummary {
        self.transforms.summary().output
    }

    pub fn text_summary_for_range(&self, range: Range<TokenOffset>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self.transforms.cursor::<(TokenOffset, usize)>(&());
        cursor.seek(&range.start, Bias::Right, &());

        let overshoot = range.start.0 - cursor.start().0 .0;
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let buffer_start = cursor.start().1;
                let suffix_start = buffer_start + overshoot;
                let suffix_end =
                    buffer_start + (cmp::min(cursor.end(&()).0, range.end).0 - cursor.start().0 .0);
                summary = self.buffer.text_summary_for_range(suffix_start..suffix_end);
                cursor.next(&());
            }
            Some(Transform::Highlight(_, _)) => {
                let buffer_start = cursor.start().1;
                let suffix_start = buffer_start + overshoot;
                let suffix_end =
                    buffer_start + (cmp::min(cursor.end(&()).0, range.end).0 - cursor.start().0 .0);
                summary = self.buffer.text_summary_for_range(suffix_start..suffix_end);
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
                    let prefix_end = prefix_start + overshoot;
                    summary += self
                        .buffer
                        .text_summary_for_range::<TextSummary, _>(prefix_start..prefix_end);
                }
                Some(Transform::Highlight(_, _)) => {
                    let prefix_start = cursor.start().1;
                    let prefix_end = prefix_start + overshoot;
                    summary += self
                        .buffer
                        .text_summary_for_range::<TextSummary, _>(prefix_start..prefix_end);
                }
                None => {}
            }
        }

        summary
    }

    pub fn row_infos(&self, row: u32) -> TokenBufferRows<'_> {
        let mut cursor = self.transforms.cursor::<(TokenPoint, Point)>(&());
        let token_point = TokenPoint::new(row, 0);
        cursor.seek(&token_point, Bias::Left, &());

        let max_buffer_row = self.buffer.max_row();
        let mut buffer_point = cursor.start().1;
        let buffer_row = if row == 0 {
            MultiBufferRow(0)
        } else {
            match cursor.item() {
                Some(Transform::Isomorphic(_)) => {
                    buffer_point += token_point.0 - cursor.start().0 .0;
                    MultiBufferRow(buffer_point.row)
                }
                Some(Transform::Highlight(_, _)) => {
                    buffer_point += token_point.0 - cursor.start().0 .0;
                    MultiBufferRow(buffer_point.row)
                }
                _ => cmp::min(MultiBufferRow(buffer_point.row + 1), max_buffer_row),
            }
        };

        TokenBufferRows {
            transforms: cursor,
            token_row: token_point.row(),
            buffer_rows: self.buffer.row_infos(buffer_row),
            max_buffer_row,
        }
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let line_start = self.to_offset(TokenPoint::new(row, 0)).0;
        let line_end = if row >= self.max_point().row() {
            self.len().0
        } else {
            self.to_offset(TokenPoint::new(row + 1, 0)).0 - 1
        };
        (line_end - line_start) as u32
    }

    pub(crate) fn chunks<'a>(
        &'a self,
        range: Range<TokenOffset>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> TokenChunks<'a> {
        let mut cursor = self.transforms.cursor::<(TokenOffset, usize)>(&());
        cursor.seek(&range.start, Bias::Right, &());

        let buffer_range = self.to_buffer_offset(range.start)..self.to_buffer_offset(range.end);
        let buffer_chunks = CustomHighlightsChunks::new(
            buffer_range,
            language_aware,
            highlights.text_highlights,
            &self.buffer,
        );

        TokenChunks {
            transforms: cursor,
            buffer_chunks,
            token_chunks: None,
            buffer_chunk: None,
            token_chunk: None,
            output_offset: range.start,
            max_output_offset: range.end,
            snapshot: self,
        }
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(Default::default()..self.len(), false, Highlights::default())
            .map(|chunk| chunk.text)
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

fn push_semantic_tokens(
    tokens: &[Token],
    buffer_snapshot: &MultiBufferSnapshot,
    sum_tree: &mut SumTree<Transform>,
    range: Range<usize>,
) {
    let (Ok(ix) | Err(ix)) = tokens.binary_search_by(|probe| {
        probe
            .range
            .start
            .to_offset(&buffer_snapshot)
            .cmp(&range.start)
            .then(std::cmp::Ordering::Greater)
    });

    if ix > tokens.len() {
        push_isomorphic(
            sum_tree,
            buffer_snapshot.text_summary_for_range(range.start..range.end),
        );
        return;
    }

    let mut acc = range.start;
    for token in &tokens[ix..] {
        let buffer_offset = token.range.end.to_offset(buffer_snapshot);
        if buffer_offset > range.end {
            break;
        }

        if acc != token.range.start.to_offset(buffer_snapshot) {
            push_isomorphic(
                sum_tree,
                buffer_snapshot
                    .text_summary_for_range(acc..token.range.start.to_offset(buffer_snapshot)),
            );
        }

        let prefix_start = sum_tree.summary().input.len;
        let prefix_end = buffer_offset;
        sum_tree.push(
            Transform::Highlight(
                token.clone(),
                buffer_snapshot.text_summary_for_range(prefix_start..prefix_end),
            ),
            &(),
        );
        acc = prefix_end;
    }
    if acc != range.end {
        push_isomorphic(
            sum_tree,
            buffer_snapshot.text_summary_for_range(acc..range.end),
        );
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
