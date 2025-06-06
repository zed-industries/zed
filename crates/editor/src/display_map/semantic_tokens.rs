use gpui::HighlightStyle;
use language::Chunk;
use multi_buffer::MultiBufferSnapshot;
use multi_buffer::ToOffset;
use std::cmp;
use std::collections::BTreeMap;
use std::ops::Range;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct HighlightEndpoint {
    id: usize,
    offset: usize,
    is_start: bool,
    style: HighlightStyle,
}

impl PartialOrd for HighlightEndpoint {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HighlightEndpoint {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.offset
            .cmp(&other.offset)
            .then_with(|| other.is_start.cmp(&self.is_start))
    }
}

pub struct TokenChunks<'a> {
    pub offset: usize,
    buffer_chunks: CustomHighlightsChunks<'a>,
    buffer_chunk: Option<Chunk<'a>>,
    endpoints: std::iter::Peekable<std::vec::IntoIter<HighlightEndpoint>>,
    active_highlights: BTreeMap<usize, HighlightStyle>,
}

impl<'a> TokenChunks<'a> {
    pub fn new(
        buffer: &'a MultiBufferSnapshot,
        buffer_range: Range<usize>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> Self {
        let mut endpoints = vec![];
        for token in &highlights.tokens {
            if !token.range.start.is_valid(buffer) || !token.range.end.is_valid(buffer) {
                continue;
            }
            let start_pos = token.range.start.to_offset(buffer);
            let end_pos = token.range.end.to_offset(buffer);
            endpoints.push(HighlightEndpoint {
                id: token.id,
                offset: start_pos,
                is_start: true,
                style: token.style,
            });
            endpoints.push(HighlightEndpoint {
                id: token.id,
                offset: end_pos,
                is_start: false,
                style: token.style,
            });
        }
        endpoints.sort();
        let buffer_chunks = CustomHighlightsChunks::new(
            buffer_range.clone(),
            language_aware,
            highlights.text_highlights,
            buffer,
        );
        Self {
            endpoints: endpoints.into_iter().peekable(),
            buffer_chunks,
            buffer_chunk: None,
            offset: buffer_range.start,
            active_highlights: Default::default(),
        }
    }

    pub fn seek(&mut self, new_range: Range<usize>) {
        self.offset = new_range.start;
        self.buffer_chunks.seek(new_range);
        self.buffer_chunk = None;
        self.active_highlights.clear();
    }
}

impl<'a> Iterator for TokenChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_highlight_endpoint = usize::MAX;
        while let Some(endpoint) = self.endpoints.peek().copied() {
            if endpoint.offset <= self.offset {
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

        let (prefix, suffix) = chunk
            .text
            .split_at(chunk.text.len().min(next_highlight_endpoint - self.offset));

        chunk.text = suffix;
        self.offset += prefix.len();
        let mut prefix = Chunk {
            text: prefix,
            ..chunk.clone()
        };
        if !self.active_highlights.is_empty() {
            prefix.syntax_highlight_id = None;
            let mut highlight_style = HighlightStyle::default();
            for active_highlight in self.active_highlights.values() {
                let mut new_highlight = active_highlight.clone();
                new_highlight.highlight(highlight_style);
                highlight_style = new_highlight;
            }
            prefix.highlight_style = Some(highlight_style);
        }
        Some(prefix)
    }
}
