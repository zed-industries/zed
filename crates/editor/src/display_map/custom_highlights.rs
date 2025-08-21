use collections::BTreeMap;
use gpui::HighlightStyle;
use language::Chunk;
use multi_buffer::{MultiBufferChunks, MultiBufferSnapshot, ToOffset as _};
use std::{
    cmp,
    iter::{self, Peekable},
    ops::Range,
    vec,
};

use crate::display_map::{HighlightKey, TextHighlights};

pub struct CustomHighlightsChunks<'a> {
    buffer_chunks: MultiBufferChunks<'a>,
    buffer_chunk: Option<Chunk<'a>>,
    offset: usize,
    multibuffer_snapshot: &'a MultiBufferSnapshot,

    highlight_endpoints: Peekable<vec::IntoIter<HighlightEndpoint>>,
    active_highlights: BTreeMap<HighlightKey, HighlightStyle>,
    text_highlights: Option<&'a TextHighlights>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct HighlightEndpoint {
    offset: usize,
    is_start: bool,
    tag: HighlightKey,
    style: HighlightStyle,
}

impl<'a> CustomHighlightsChunks<'a> {
    pub fn new(
        range: Range<usize>,
        language_aware: bool,
        text_highlights: Option<&'a TextHighlights>,
        multibuffer_snapshot: &'a MultiBufferSnapshot,
    ) -> Self {
        Self {
            buffer_chunks: multibuffer_snapshot.chunks(range.clone(), language_aware),
            buffer_chunk: None,
            offset: range.start,

            text_highlights,
            highlight_endpoints: create_highlight_endpoints(
                &range,
                text_highlights,
                multibuffer_snapshot,
            ),
            active_highlights: Default::default(),
            multibuffer_snapshot,
        }
    }

    pub fn seek(&mut self, new_range: Range<usize>) {
        self.highlight_endpoints =
            create_highlight_endpoints(&new_range, self.text_highlights, self.multibuffer_snapshot);
        self.offset = new_range.start;
        self.buffer_chunks.seek(new_range);
        self.buffer_chunk.take();
        self.active_highlights.clear()
    }
}

fn create_highlight_endpoints(
    range: &Range<usize>,
    text_highlights: Option<&TextHighlights>,
    buffer: &MultiBufferSnapshot,
) -> iter::Peekable<vec::IntoIter<HighlightEndpoint>> {
    let mut highlight_endpoints = Vec::new();
    if let Some(text_highlights) = text_highlights {
        let start = buffer.anchor_after(range.start);
        let end = buffer.anchor_after(range.end);
        for (&tag, text_highlights) in text_highlights.iter() {
            let style = text_highlights.0;
            let ranges = &text_highlights.1;

            let start_ix = match ranges.binary_search_by(|probe| {
                let cmp = probe.end.cmp(&start, buffer);
                if cmp.is_gt() {
                    cmp::Ordering::Greater
                } else {
                    cmp::Ordering::Less
                }
            }) {
                Ok(i) | Err(i) => i,
            };

            for range in &ranges[start_ix..] {
                if range.start.cmp(&end, buffer).is_ge() {
                    break;
                }

                highlight_endpoints.push(HighlightEndpoint {
                    offset: range.start.to_offset(buffer),
                    is_start: true,
                    tag,
                    style,
                });
                highlight_endpoints.push(HighlightEndpoint {
                    offset: range.end.to_offset(buffer),
                    is_start: false,
                    tag,
                    style,
                });
            }
        }
        highlight_endpoints.sort();
    }
    highlight_endpoints.into_iter().peekable()
}

impl<'a> Iterator for CustomHighlightsChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_highlight_endpoint = usize::MAX;
        while let Some(endpoint) = self.highlight_endpoints.peek().copied() {
            if endpoint.offset <= self.offset {
                if endpoint.is_start {
                    self.active_highlights.insert(endpoint.tag, endpoint.style);
                } else {
                    self.active_highlights.remove(&endpoint.tag);
                }
                self.highlight_endpoints.next();
            } else {
                next_highlight_endpoint = endpoint.offset;
                break;
            }
        }

        let chunk = self
            .buffer_chunk
            .get_or_insert_with(|| self.buffer_chunks.next().unwrap());
        if chunk.text.is_empty() {
            *chunk = self.buffer_chunks.next().unwrap();
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
            let mut highlight_style = HighlightStyle::default();
            for active_highlight in self.active_highlights.values() {
                highlight_style.highlight(*active_highlight);
            }
            prefix.highlight_style = Some(highlight_style);
        }
        Some(prefix)
    }
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
