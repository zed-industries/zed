use collections::BTreeSet;
use gpui::HighlightStyle;
use language::{Chunk, Edit, Point, TextSummary};
use multi_buffer::MultiBufferSnapshot;
use multi_buffer::{MultiBufferRow, MultiBufferRows, RowInfo, ToOffset};
use std::cmp;
use std::collections::BTreeMap;
use std::ops::{Add, AddAssign, Range, Sub, SubAssign};
use sum_tree::{Bias, Cursor, SumTree};

use super::{Highlights, custom_highlights::CustomHighlightsChunks};

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
    pub tokens: Vec<Token>,
    snapshot: TokenSnapshot,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct HighlightEndpoint {
    id: usize,
    offset: usize,
    is_start: bool,
    style: HighlightStyle,
}

#[derive(Clone)]
pub struct TokenSnapshot {
    endpoints: Vec<HighlightEndpoint>,
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
            Transform::Highlight(token, summary) => TransformSummary {
                input: *summary,
                output: token.text.summary(),
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
    endpoints: std::iter::Peekable<std::vec::IntoIter<HighlightEndpoint>>,
    offset: TokenOffset,
    active_highlights: BTreeMap<usize, HighlightStyle>,
    snapshot: &'a TokenSnapshot,
}

impl TokenChunks<'_> {
    pub fn seek(&mut self, new_range: Range<TokenOffset>) {
        self.transforms.seek(&new_range.start, Bias::Right, &());

        let buffer_range = self.snapshot.to_buffer_offset(new_range.start)
            ..self.snapshot.to_buffer_offset(new_range.end);
        self.buffer_chunks.seek(buffer_range);
        self.buffer_chunk = None;
        self.offset = new_range.start;
        self.active_highlights.clear();
    }

    pub fn offset(&self) -> TokenOffset {
        self.offset
    }
}

impl<'a> Iterator for TokenChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_highlight_endpoint = usize::MAX;
        while let Some(endpoint) = self.endpoints.peek().copied() {
            if endpoint.offset <= self.offset.0 {
                if endpoint.is_start {
                    self.active_highlights.insert(endpoint.id, endpoint.style);
                } else {
                    self.active_highlights.remove(&endpoint.id);
                }
                self.endpoints.next();
            } else {
                next_highlight_endpoint = endpoint.offset;
                break;
            }
        }

        let chunk = self
            .buffer_chunk
            .get_or_insert_with(|| self.buffer_chunks.next().unwrap());
        if chunk.text.is_empty() {
            *chunk = self.buffer_chunks.next()?;
        }

        let (prefix, suffix) = chunk.text.split_at(
            chunk
                .text
                .len()
                .min(next_highlight_endpoint - self.offset.0),
        );

        chunk.text = suffix;
        self.offset += TokenOffset(prefix.len());
        let mut prefix = Chunk {
            text: prefix,
            ..chunk.clone()
        };
        if !self.active_highlights.is_empty() {
            prefix.syntax_highlight_id = None;
            let mut highlight_style = HighlightStyle::default();
            for active_highlight in self.active_highlights.values() {
                highlight_style.highlight(*active_highlight);
            }
            prefix.highlight_style = Some(highlight_style);
        }
        Some(prefix)
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
                    buffer_point += token_point.0 - self.transforms.start().0.0;
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
            endpoints: vec![],
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
            snapshot.endpoints = vec![];
            let token_edits = buffer_edits
                .into_iter()
                .map(|edit| {
                    let (Ok(ix) | Err(ix)) = self.tokens.binary_search_by(|probe| {
                        probe
                            .range
                            .start
                            .to_offset(&snapshot.buffer)
                            .cmp(&edit.new.start)
                            .then(std::cmp::Ordering::Greater)
                    });

                    for token in &self.tokens[ix..] {
                        if !token.range.start.is_valid(&snapshot.buffer)
                            || !token.range.end.is_valid(&snapshot.buffer)
                        {
                            continue;
                        }
                        let start_pos = token.range.start.to_offset(&snapshot.buffer);
                        let end_pos = token.range.end.to_offset(&snapshot.buffer);
                        if start_pos > edit.new.end {
                            break;
                        }
                        snapshot.endpoints.push(HighlightEndpoint {
                            id: token.id,
                            offset: start_pos,
                            is_start: true,
                            style: token.style,
                        });
                        snapshot.endpoints.push(HighlightEndpoint {
                            id: token.id,
                            offset: end_pos,
                            is_start: false,
                            style: token.style,
                        });
                    }

                    TokenEdit {
                        old: TokenOffset(edit.old.start)..TokenOffset(edit.old.end),
                        new: TokenOffset(edit.new.start)..TokenOffset(edit.new.end),
                    }
                })
                .collect();

            (snapshot.clone(), token_edits)
        }
    }

    pub fn splice(
        &mut self,
        to_remove: &[usize],
        to_insert: Vec<Token>,
    ) -> (TokenSnapshot, Vec<TokenEdit>) {
        let snapshot = &mut self.snapshot;
        let mut edits = BTreeSet::new();

        self.tokens.retain(|token| {
            let retain = !to_remove.contains(&token.id);
            if !retain {
                let offset = token.range.start.to_offset(&snapshot.buffer);
                edits.insert(offset);
            }
            retain
        });

        for token_to_insert in to_insert {
            edits.insert(token_to_insert.range.start.to_offset(&snapshot.buffer));
            let (Ok(ix) | Err(ix)) = self.tokens.binary_search_by(|probe| {
                probe
                    .range
                    .start
                    .cmp(&token_to_insert.range.start, &snapshot.buffer)
                    .then(std::cmp::Ordering::Less)
            });
            self.tokens.insert(ix, token_to_insert);
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

    pub fn current_tokens(&self) -> impl Iterator<Item = &Token> {
        self.tokens.iter()
    }

    #[cfg(test)]
    pub(crate) fn randomly_mutate(
        &mut self,
        next_token_id: &mut usize,
        rng: &mut rand::rngs::StdRng,
    ) -> (TokenSnapshot, Vec<TokenEdit>) {
        use gpui::Hsla;
        use rand::prelude::*;
        use util::post_inc;

        let mut to_remove = Vec::new();
        let mut to_insert = Vec::new();
        let snapshot = &mut self.snapshot;

        for _ in 0..rng.gen_range(1..=5) {
            if self.tokens.is_empty() || rng.r#gen() {
                let position = snapshot.buffer.random_byte_range(0, rng);
                let style = HighlightStyle::color(Hsla {
                    h: rng.gen_range(0.0..1.0), // hue
                    s: rng.gen_range(0.5..1.0), // saturation
                    l: rng.gen_range(0.5..0.9), // lightness
                    a: 1.0,                     // alpha
                });
                let text = util::RandomCharIter::new(&mut *rng)
                    .filter(|ch| *ch != '\r')
                    .take(position.end - position.start)
                    .collect::<String>();

                to_insert.push(Token::new(
                    post_inc(next_token_id),
                    snapshot.buffer.anchor_before(position.start)
                        ..snapshot.buffer.anchor_after(position.end),
                    style,
                    text,
                ));
            } else {
                let token_id = self
                    .tokens
                    .iter()
                    .choose(rng)
                    .map(|token| token.id)
                    .unwrap();
                to_remove.push(token_id);
            }
        }
        log::info!("removing tokens: {:?}", to_remove);

        let (snapshot, edits) = self.splice(&to_remove, to_insert);
        (snapshot, edits)
    }
}

impl TokenSnapshot {
    pub fn to_point(&self, offset: TokenOffset) -> TokenPoint {
        let mut cursor = self
            .transforms
            .cursor::<(TokenOffset, (TokenPoint, usize))>(&());
        cursor.seek(&offset, Bias::Right, &());
        let overshoot = offset.0 - cursor.start().0.0;
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let buffer_offset_start = cursor.start().1.1;
                let buffer_offset_end = buffer_offset_start + overshoot;
                let buffer_start = self.buffer.offset_to_point(buffer_offset_start);
                let buffer_end = self.buffer.offset_to_point(buffer_offset_end);
                TokenPoint(cursor.start().1.0.0 + (buffer_end - buffer_start))
            }
            Some(Transform::Highlight(_, _)) => {
                let buffer_offset_start = cursor.start().1.1;
                let buffer_offset_end = buffer_offset_start + overshoot;
                let buffer_start = self.buffer.offset_to_point(buffer_offset_start);
                let buffer_end = self.buffer.offset_to_point(buffer_offset_end);
                TokenPoint(cursor.start().1.0.0 + (buffer_end - buffer_start))
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
        let overshoot = point.0 - cursor.start().0.0;
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let buffer_point_start = cursor.start().1.1;
                let buffer_point_end = buffer_point_start + overshoot;
                let buffer_offset_start = self.buffer.point_to_offset(buffer_point_start);
                let buffer_offset_end = self.buffer.point_to_offset(buffer_point_end); // TODO: check here
                TokenOffset(cursor.start().1.0.0 + (buffer_offset_end - buffer_offset_start))
            }
            Some(Transform::Highlight(_, _)) => {
                let buffer_point_start = cursor.start().1.1;
                let buffer_point_end = buffer_point_start + overshoot;
                let buffer_offset_start = self.buffer.point_to_offset(buffer_point_start);
                let buffer_offset_end = self.buffer.point_to_offset(buffer_point_end);
                TokenOffset(cursor.start().1.0.0 + (buffer_offset_end - buffer_offset_start))
            }
            None => self.len(),
        }
    }

    pub fn to_buffer_point(&self, point: TokenPoint) -> Point {
        let mut cursor = self.transforms.cursor::<(TokenPoint, Point)>(&());
        cursor.seek(&point, Bias::Right, &());
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let overshoot = point.0 - cursor.start().0.0;
                cursor.start().1 + overshoot
            }
            Some(Transform::Highlight(_, _)) => {
                let overshoot = point.0 - cursor.start().0.0;
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
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                if offset == cursor.end(&()).0 {
                    cursor.end(&()).1
                } else {
                    let overshoot = offset - cursor.start().0;
                    TokenOffset(cursor.start().1.0 + overshoot)
                }
            }
            Some(Transform::Highlight(_, _)) => {
                if offset == cursor.end(&()).0 {
                    cursor.end(&()).1
                } else {
                    let overshoot = offset - cursor.start().0;
                    TokenOffset(cursor.start().1.0 + overshoot)
                }
            }
            None => self.len(),
        }
    }
    pub fn to_token_point(&self, point: Point) -> TokenPoint {
        let mut cursor = self.transforms.cursor::<(Point, TokenPoint)>(&());
        cursor.seek(&point, Bias::Left, &());
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                if point == cursor.end(&()).0 {
                    cursor.end(&()).1
                } else {
                    let overshoot = point - cursor.start().0;
                    TokenPoint(cursor.start().1.0 + overshoot)
                }
            }
            Some(Transform::Highlight(_, _)) => {
                if point == cursor.end(&()).0 {
                    cursor.end(&()).1
                } else {
                    let overshoot = point - cursor.start().0;
                    TokenPoint(cursor.start().1.0 + overshoot)
                }
            }
            None => self.max_point(),
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
                        let overshoot = point.0 - cursor.start().0.0;
                        let buffer_point = cursor.start().1 + overshoot;
                        let clipped_buffer_point = self.buffer.clip_point(buffer_point, bias);
                        let clipped_overshoot = clipped_buffer_point - cursor.start().1;
                        let clipped_point = TokenPoint(cursor.start().0.0 + clipped_overshoot);
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
                        let overshoot = point.0 - cursor.start().0.0;
                        let buffer_point = cursor.start().1 + overshoot;
                        let clipped_buffer_point = self.buffer.clip_point(buffer_point, bias);
                        let clipped_overshoot = clipped_buffer_point - cursor.start().1;
                        let clipped_point = TokenPoint(cursor.start().0.0 + clipped_overshoot);
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

        let overshoot = range.start.0 - cursor.start().0.0;
        match cursor.item() {
            Some(Transform::Isomorphic(_)) => {
                let buffer_start = cursor.start().1;
                let suffix_start = buffer_start + overshoot;
                let suffix_end =
                    buffer_start + (cmp::min(cursor.end(&()).0, range.end).0 - cursor.start().0.0);
                summary = self.buffer.text_summary_for_range(suffix_start..suffix_end);
                cursor.next(&());
            }
            Some(Transform::Highlight(_, _)) => {
                let buffer_start = cursor.start().1;
                let suffix_start = buffer_start + overshoot;
                let suffix_end =
                    buffer_start + (cmp::min(cursor.end(&()).0, range.end).0 - cursor.start().0.0);
                summary = self.buffer.text_summary_for_range(suffix_start..suffix_end);
                cursor.next(&());
            }
            None => {}
        }

        if range.end > cursor.start().0 {
            summary += cursor
                .summary::<_, TransformSummary>(&range.end, Bias::Right, &())
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
                    buffer_point += token_point.0 - cursor.start().0.0;
                    MultiBufferRow(buffer_point.row)
                }
                Some(Transform::Highlight(_, _)) => {
                    buffer_point += token_point.0 - cursor.start().0.0;
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
            buffer_chunk: None,
            endpoints: self.endpoints.clone().into_iter().peekable(),
            active_highlights: Default::default(),
            offset: range.start,
            snapshot: self,
        }
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(Default::default()..self.len(), false, Highlights::default())
            .map(|chunk| chunk.text)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use gpui::Hsla;
    use multi_buffer::MultiBuffer;
    use rand::{Rng as _, rngs::StdRng};
    use ui::App;
    use util::post_inc;

    use super::*;

    #[gpui::test]
    fn test_token_buffer_rows(cx: &mut App) {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi", cx);
        let (mut token_map, token_snapshot) = TokenMap::new(buffer.read(cx).snapshot(cx));
        assert_eq!(token_snapshot.text(), "abc\ndef\nghi");
        let mut next_token_id = 0;

        let (token_snapshot, _) = token_map.splice(
            &[],
            vec![
                Token::new(
                    post_inc(&mut next_token_id),
                    buffer.read(cx).snapshot(cx).anchor_at(0, Bias::Left)
                        ..buffer.read(cx).snapshot(cx).anchor_at(3, Bias::Right),
                    HighlightStyle::color(Hsla::green()),
                    "|1|",
                ),
                Token::new(
                    post_inc(&mut next_token_id),
                    buffer.read(cx).snapshot(cx).anchor_at(4, Bias::Left)
                        ..buffer.read(cx).snapshot(cx).anchor_at(7, Bias::Right),
                    HighlightStyle::color(Hsla::green()),
                    "|2|",
                ),
                Token::new(
                    post_inc(&mut next_token_id),
                    buffer.read(cx).snapshot(cx).anchor_at(8, Bias::Left)
                        ..buffer.read(cx).snapshot(cx).anchor_at(11, Bias::Right),
                    HighlightStyle::color(Hsla::green()),
                    "|3|",
                ),
            ],
        );

        let actual_text = token_snapshot.text();
        assert_eq!(actual_text, "|1|\n|2|\n|3|");
        assert_eq!(
            token_snapshot
                .row_infos(0)
                .map(|info| info.buffer_row)
                .collect::<Vec<_>>(),
            vec![Some(0), Some(1), Some(2)]
        );
    }

    #[gpui::test]
    fn test_token_overlapping(cx: &mut App) {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi", cx);
        let (mut token_map, token_snapshot) = TokenMap::new(buffer.read(cx).snapshot(cx));
        assert_eq!(token_snapshot.text(), "abc\ndef\nghi");
        let mut next_token_id = 0;

        let (_, _) = token_map.splice(
            &[],
            vec![Token::new(
                post_inc(&mut next_token_id),
                buffer.read(cx).snapshot(cx).anchor_at(0, Bias::Left)
                    ..buffer.read(cx).snapshot(cx).anchor_at(11, Bias::Right),
                HighlightStyle::color(Hsla::green()),
                "abc\ndef\nghi",
            )],
        );
        let (token_snapshot, _) = token_map.splice(
            &[],
            vec![Token::new(
                post_inc(&mut next_token_id),
                buffer.read(cx).snapshot(cx).anchor_at(0, Bias::Left)
                    ..buffer.read(cx).snapshot(cx).anchor_at(3, Bias::Right),
                HighlightStyle::color(Hsla::green()),
                "ABC",
            )],
        );

        let actual_text = token_snapshot.text();
        assert_eq!(actual_text, "ABC\ndef\nghi");
        assert_eq!(
            token_snapshot
                .row_infos(0)
                .map(|info| info.buffer_row)
                .collect::<Vec<_>>(),
            vec![Some(0), Some(1), Some(2)]
        );
    }

    #[gpui::test]
    fn test_basic_semantic_tokens(cx: &mut App) {
        let buffer = MultiBuffer::build_simple("abcdefghi", cx);
        let buffer_edits = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (mut token_map, token_snapshot) = TokenMap::new(buffer.read(cx).snapshot(cx));
        assert_eq!(token_snapshot.text(), "abcdefghi");
        let mut next_token_id = 0;

        let (token_snapshot, _) = token_map.splice(
            &[],
            vec![Token::new(
                post_inc(&mut next_token_id),
                buffer.read(cx).snapshot(cx).anchor_at(3, Bias::Left)
                    ..buffer.read(cx).snapshot(cx).anchor_at(6, Bias::Right),
                HighlightStyle::color(Hsla::green()),
                "<->",
            )],
        );
        assert_eq!(token_snapshot.text(), "abc<->ghi");
        assert_eq!(
            token_snapshot.to_buffer_point(TokenPoint::new(0, 0)),
            Point::new(0, 0)
        );
        assert_eq!(
            token_snapshot.to_buffer_point(TokenPoint::new(0, 3)),
            Point::new(0, 3)
        );
        assert_eq!(
            token_snapshot.to_buffer_point(TokenPoint::new(0, 6)),
            Point::new(0, 6)
        );
        assert_eq!(
            token_snapshot.to_token_point(Point::new(0, 0)),
            TokenPoint::new(0, 0)
        );
        assert_eq!(
            token_snapshot.to_token_point(Point::new(0, 3)),
            TokenPoint::new(0, 3)
        );
        assert_eq!(
            token_snapshot.to_token_point(Point::new(0, 6)),
            TokenPoint::new(0, 6)
        );

        // Test clipping points
        assert_eq!(
            token_snapshot.clip_point(TokenPoint::new(0, 0), Bias::Left),
            TokenPoint::new(0, 0)
        );
        assert_eq!(
            token_snapshot.clip_point(TokenPoint::new(0, 9), Bias::Right),
            TokenPoint::new(0, 9)
        );

        // Edits before or after the token should not affect it
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..1, "A"), (8..9, "I")], None, cx)
        });
        let (token_snapshot, _) = token_map.sync(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        assert_eq!(token_snapshot.text(), "Abc<->ghI");

        // Add another token that highlights "bc"
        let (token_snapshot, _) = token_map.splice(
            &[],
            vec![Token::new(
                post_inc(&mut next_token_id),
                buffer.read(cx).snapshot(cx).anchor_at(1, Bias::Left)
                    ..buffer.read(cx).snapshot(cx).anchor_at(3, Bias::Right),
                HighlightStyle::color(Hsla::blue()),
                "xx",
            )],
        );
        assert_eq!(token_snapshot.text(), "Axx<->ghI");

        // An edit within a token should invalidate it
        buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "X")], None, cx));
        let (token_snapshot, _) = token_map.sync(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        assert_eq!(token_snapshot.text(), "AxxdXfghI");

        // TODO: support overlapping tokens
        // Add multiple overlapping tokens
        // let (token_snapshot, _) = token_map.splice(
        //     &[],
        //     vec![
        //         Token::new(
        //             post_inc(&mut next_token_id),
        //             buffer.read(cx).snapshot(cx).anchor_at(2, Bias::Left)
        //                 ..buffer.read(cx).snapshot(cx).anchor_at(5, Bias::Right),
        //             HighlightStyle::color(Hsla::red()),
        //             "cdX",
        //         ),
        //         Token::new(
        //             post_inc(&mut next_token_id),
        //             buffer.read(cx).snapshot(cx).anchor_at(5, Bias::Left)
        //                 ..buffer.read(cx).snapshot(cx).anchor_at(7, Bias::Right),
        //             HighlightStyle::color(Hsla::red()),
        //             "fg",
        //         ),
        //     ],
        // );
        // assert_eq!(token_snapshot.text(), "AbcdXfghI");

        // The tokens can be manually removed
        let (token_snapshot, _) = token_map.splice(
            &token_map
                .current_tokens()
                .map(|token| token.id)
                .collect::<Vec<usize>>(),
            Vec::new(),
        );
        assert_eq!(token_snapshot.text(), "AbcdXfghI");

        // Test with multiline text
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..8, "hello\nworld")], None, cx)
        });
        let (token_snapshot, _) = token_map.sync(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        assert_eq!(token_snapshot.text(), "hello\nworldI");

        // Add tokens spanning multiple lines
        let (token_snapshot, _) = token_map.splice(
            &[],
            vec![
                Token::new(
                    next_token_id,
                    buffer.read(cx).snapshot(cx).anchor_at(2, Bias::Left)
                        ..buffer.read(cx).snapshot(cx).anchor_at(7, Bias::Right),
                    HighlightStyle::color(Hsla::green()),
                    "llo\nw",
                ),
                Token::new(
                    post_inc(&mut next_token_id),
                    buffer.read(cx).snapshot(cx).anchor_at(7, Bias::Left)
                        ..buffer.read(cx).snapshot(cx).anchor_at(10, Bias::Right),
                    HighlightStyle::color(Hsla::blue()),
                    "orl",
                ),
            ],
        );
        assert_eq!(token_snapshot.text(), "hello\nworldI");

        // Test row information
        assert_eq!(
            token_snapshot
                .row_infos(0)
                .map(|info| info.buffer_row)
                .collect::<Vec<_>>(),
            vec![Some(0), Some(1)]
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random_tokens(cx: &mut App, mut rng: StdRng) {
        init_test(cx);

        let operations = std::env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let len = rng.gen_range(0..30);
        let buffer = if rng.r#gen() {
            let text = util::RandomCharIter::new(&mut rng)
                .take(len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };
        let mut buffer_snapshot = buffer.read(cx).snapshot(cx);
        let mut next_token_id = 0;
        log::info!("buffer text: {:?}", buffer_snapshot.text());
        let (mut token_map, _) = TokenMap::new(buffer.read(cx).snapshot(cx));
        for _ in 0..operations {
            let mut buffer_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=50 => {
                    let (snapshot, _) = token_map.randomly_mutate(&mut next_token_id, &mut rng);
                    log::info!("mutated text: {:?}", snapshot.text());
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

            let (new_token_snapshot, _) = token_map.sync(buffer_snapshot.clone(), buffer_edits);

            log::info!("buffer text: {:?}", buffer_snapshot.text());
            log::info!("token text: {:?}", new_token_snapshot.text());

            // Verify the token text matches the buffer text (without highlighting)
            assert_eq!(new_token_snapshot.text(), buffer_snapshot.text());

            let expected_buffer_rows = new_token_snapshot.row_infos(0).collect::<Vec<_>>();
            assert_eq!(
                expected_buffer_rows.len() as u32,
                buffer_snapshot.max_point().row + 1
            );
            for row_start in 0..expected_buffer_rows.len() {
                assert_eq!(
                    new_token_snapshot
                        .row_infos(row_start as u32)
                        .collect::<Vec<_>>(),
                    &expected_buffer_rows[row_start..],
                    "incorrect buffer rows starting at {}",
                    row_start
                );
            }

            for _ in 0..5 {
                let mut end = rng.gen_range(0..=buffer_snapshot.text().len());
                end = buffer_snapshot.clip_offset(end, Bias::Right);
                let mut start = rng.gen_range(0..=end);
                start = buffer_snapshot.clip_offset(start, Bias::Right);

                let range = TokenOffset(start)..TokenOffset(end);
                log::info!("calling token_snapshot.chunks({range:?})");
                let actual_text = new_token_snapshot
                    .chunks(
                        range,
                        false,
                        Highlights {
                            text_highlights: None,
                            ..Highlights::default()
                        },
                    )
                    .map(|chunk| chunk.text)
                    .collect::<String>();
                assert_eq!(
                    actual_text,
                    buffer_snapshot
                        .text_for_range(start..end)
                        .collect::<String>(),
                    "incorrect text in range {:?}",
                    start..end
                );

                assert_eq!(
                    new_token_snapshot.text_summary_for_range(TokenOffset(start)..TokenOffset(end)),
                    buffer_snapshot.text_summary_for_range(start..end)
                );
            }

            let mut buffer_point = Point::default();
            let mut token_point = new_token_snapshot.to_token_point(buffer_point);
            let mut buffer_chars = buffer_snapshot.chars_at(0);
            loop {
                // Ensure conversion from buffer coordinates to token coordinates
                // is consistent.
                let buffer_offset = buffer_snapshot.point_to_offset(buffer_point);
                assert_eq!(
                    new_token_snapshot.to_point(new_token_snapshot.to_token_offset(buffer_offset)),
                    token_point
                );

                // No matter which bias we clip a token point with, it doesn't move
                // because it was constructed from a buffer point.
                assert_eq!(
                    new_token_snapshot.clip_point(token_point, Bias::Left),
                    token_point,
                    "invalid token point for buffer point {:?} when clipped left",
                    buffer_point
                );
                assert_eq!(
                    new_token_snapshot.clip_point(token_point, Bias::Right),
                    token_point,
                    "invalid token point for buffer point {:?} when clipped right",
                    buffer_point
                );

                if let Some(ch) = buffer_chars.next() {
                    if ch == '\n' {
                        buffer_point += Point::new(1, 0);
                    } else {
                        buffer_point += Point::new(0, ch.len_utf8() as u32);
                    }

                    // Ensure that moving forward in the buffer moves the token point forward as well.
                    let new_token_point = new_token_snapshot.to_token_point(buffer_point);
                    assert!(new_token_point >= token_point);
                    token_point = new_token_point;
                } else {
                    break;
                }
            }
        }
    }

    fn init_test(cx: &mut App) {
        let store = settings::SettingsStore::test(cx);
        cx.set_global(store);
        theme::init(theme::LoadThemes::JustBase, cx);
    }
}
