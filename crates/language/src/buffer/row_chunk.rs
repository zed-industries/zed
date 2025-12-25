//! A row chunk is an exclusive range of rows, [`BufferRow`] within a buffer of a certain version, [`Global`].
//! All but the last chunk are of a constant, given size.

use std::{ops::Range, sync::Arc};

use text::{Anchor, OffsetRangeExt as _, Point};
use util::RangeExt;

use crate::BufferRow;

/// An range of rows, exclusive as [`lsp::Range`] and
/// <https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#range>
/// denote.
///
/// Represents an area in a text editor, adjacent to other ones.
/// Together, chunks form entire document at a particular version [`Global`].
/// Each chunk is queried for inlays as `(start_row, 0)..(end_exclusive, 0)` via
/// <https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#inlayHintParams>
#[derive(Clone)]
pub struct RowChunks {
    chunks: Arc<[RowChunk]>,
    version: clock::Global,
}

impl std::fmt::Debug for RowChunks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RowChunks")
            .field("chunks", &self.chunks)
            .finish()
    }
}

impl RowChunks {
    pub fn new(snapshot: text::BufferSnapshot, max_rows_per_chunk: u32) -> Self {
        let buffer_point_range = (0..snapshot.len()).to_point(&snapshot);
        let last_row = buffer_point_range.end.row;
        let chunks = (buffer_point_range.start.row..=last_row)
            .step_by(max_rows_per_chunk as usize)
            .collect::<Vec<_>>();
        let last_chunk_id = chunks.len() - 1;
        let chunks = chunks
            .into_iter()
            .enumerate()
            .map(|(id, chunk_start)| {
                let start = Point::new(chunk_start, 0);
                let end_exclusive = (chunk_start + max_rows_per_chunk).min(last_row);
                let end = if id == last_chunk_id {
                    Point::new(end_exclusive, snapshot.line_len(end_exclusive))
                } else {
                    Point::new(end_exclusive, 0)
                };
                RowChunk {
                    id,
                    start: chunk_start,
                    end_exclusive,
                    start_anchor: snapshot.anchor_before(start),
                    end_anchor: snapshot.anchor_after(end),
                }
            })
            .collect::<Vec<_>>();
        Self {
            chunks: Arc::from(chunks),
            version: snapshot.version().clone(),
        }
    }

    pub fn version(&self) -> &clock::Global {
        &self.version
    }

    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn applicable_chunks(&self, ranges: &[Range<Point>]) -> impl Iterator<Item = RowChunk> {
        let row_ranges = ranges
            .iter()
            // Be lenient and yield multiple chunks if they "touch" the exclusive part of the range.
            // This will result in LSP hints [re-]queried for more ranges, but also more hints already visible when scrolling around.
            .map(|point_range| point_range.start.row..point_range.end.row + 1)
            .collect::<Vec<_>>();
        self.chunks
            .iter()
            .filter(move |chunk| -> bool {
                let chunk_range = chunk.row_range().to_inclusive();
                row_ranges
                    .iter()
                    .any(|row_range| chunk_range.overlaps(&row_range))
            })
            .copied()
    }

    pub fn previous_chunk(&self, chunk: RowChunk) -> Option<RowChunk> {
        if chunk.id == 0 {
            None
        } else {
            self.chunks.get(chunk.id - 1).copied()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowChunk {
    pub id: usize,
    pub start: BufferRow,
    pub end_exclusive: BufferRow,
    pub start_anchor: Anchor,
    pub end_anchor: Anchor,
}

impl RowChunk {
    pub fn row_range(&self) -> Range<BufferRow> {
        self.start..self.end_exclusive
    }

    pub fn anchor_range(&self) -> Range<Anchor> {
        self.start_anchor..self.end_anchor
    }
}
