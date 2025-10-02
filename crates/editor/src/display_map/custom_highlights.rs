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
    tag: HighlightKey,
    style: Option<HighlightStyle>,
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

                let start = range.start.to_offset(buffer);
                let end = range.end.to_offset(buffer);
                if start == end {
                    continue;
                }
                highlight_endpoints.push(HighlightEndpoint {
                    offset: start,
                    tag,
                    style: Some(style),
                });
                highlight_endpoints.push(HighlightEndpoint {
                    offset: end,
                    tag,
                    style: None,
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
                if let Some(style) = endpoint.style {
                    self.active_highlights.insert(endpoint.tag, style);
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
            .get_or_insert_with(|| self.buffer_chunks.next().unwrap_or_default());
        if chunk.text.is_empty() {
            *chunk = self.buffer_chunks.next()?;
        }

        let split_idx = chunk.text.len().min(next_highlight_endpoint - self.offset);
        let (prefix, suffix) = chunk.text.split_at(split_idx);

        let (chars, tabs) = if split_idx == 128 {
            let output = (chunk.chars, chunk.tabs);
            chunk.chars = 0;
            chunk.tabs = 0;
            output
        } else {
            let mask = (1 << split_idx) - 1;
            let output = (chunk.chars & mask, chunk.tabs & mask);
            chunk.chars = chunk.chars >> split_idx;
            chunk.tabs = chunk.tabs >> split_idx;
            output
        };

        chunk.text = suffix;
        self.offset += prefix.len();
        let mut prefix = Chunk {
            text: prefix,
            chars,
            tabs,
            ..chunk.clone()
        };
        if !self.active_highlights.is_empty() {
            prefix.highlight_style = self
                .active_highlights
                .values()
                .copied()
                .reduce(|acc, active_highlight| acc.highlight(active_highlight));
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
            .then_with(|| self.style.is_some().cmp(&other.style.is_some()))
    }
}

#[cfg(test)]
mod tests {
    use std::{any::TypeId, sync::Arc};

    use super::*;
    use crate::MultiBuffer;
    use gpui::App;
    use rand::prelude::*;
    use util::RandomCharIter;

    #[gpui::test(iterations = 100)]
    fn test_random_chunk_bitmaps(cx: &mut App, mut rng: StdRng) {
        // Generate random buffer using existing test infrastructure
        let len = rng.random_range(10..10000);
        let buffer = if rng.random() {
            let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };

        let buffer_snapshot = buffer.read(cx).snapshot(cx);

        // Create random highlights
        let mut highlights = sum_tree::TreeMap::default();
        let highlight_count = rng.random_range(1..10);

        for _i in 0..highlight_count {
            let style = HighlightStyle {
                color: Some(gpui::Hsla {
                    h: rng.random::<f32>(),
                    s: rng.random::<f32>(),
                    l: rng.random::<f32>(),
                    a: 1.0,
                }),
                ..Default::default()
            };

            let mut ranges = Vec::new();
            let range_count = rng.random_range(1..10);
            let text = buffer_snapshot.text();
            for _ in 0..range_count {
                if buffer_snapshot.len() == 0 {
                    continue;
                }

                let mut start = rng.random_range(0..=buffer_snapshot.len().saturating_sub(10));

                while !text.is_char_boundary(start) {
                    start = start.saturating_sub(1);
                }

                let end_end = buffer_snapshot.len().min(start + 100);
                let mut end = rng.random_range(start..=end_end);
                while !text.is_char_boundary(end) {
                    end = end.saturating_sub(1);
                }

                if start < end {
                    start = end;
                }
                let start_anchor = buffer_snapshot.anchor_before(start);
                let end_anchor = buffer_snapshot.anchor_after(end);
                ranges.push(start_anchor..end_anchor);
            }

            let type_id = TypeId::of::<()>(); // Simple type ID for testing
            highlights.insert(HighlightKey::Type(type_id), Arc::new((style, ranges)));
        }

        // Get all chunks and verify their bitmaps
        let chunks =
            CustomHighlightsChunks::new(0..buffer_snapshot.len(), false, None, &buffer_snapshot);

        for chunk in chunks {
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
}
