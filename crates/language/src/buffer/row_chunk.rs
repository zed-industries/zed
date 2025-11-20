//! A row chunk is an exclusive range of rows, [`BufferRow`] within a buffer of a certain version, [`Global`].
//! All but the last chunk are of a constant, given size.

use std::{ops::Range, sync::Arc};

use clock::Global;
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
    pub(crate) snapshot: text::BufferSnapshot,
    chunks: Arc<[RowChunk]>,
}

impl std::fmt::Debug for RowChunks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RowChunks")
            .field("version", self.snapshot.version())
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
            .enumerate()
            .map(|(id, chunk_start)| RowChunk {
                id,
                start: chunk_start,
                end_exclusive: (chunk_start + max_rows_per_chunk).min(last_row),
            })
            .collect::<Vec<_>>();
        Self {
            snapshot,
            chunks: Arc::from(chunks),
        }
    }

    pub fn version(&self) -> &Global {
        self.snapshot.version()
    }

    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn applicable_chunks(
        &self,
        ranges: &[Range<text::Anchor>],
    ) -> impl Iterator<Item = RowChunk> {
        let row_ranges = ranges
            .iter()
            .map(|range| range.to_point(&self.snapshot))
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

    pub fn chunk_range(&self, chunk: RowChunk) -> Option<Range<Anchor>> {
        if !self.chunks.contains(&chunk) {
            return None;
        }

        let start = Point::new(chunk.start, 0);
        let end = if self.chunks.last() == Some(&chunk) {
            Point::new(
                chunk.end_exclusive,
                self.snapshot.line_len(chunk.end_exclusive),
            )
        } else {
            Point::new(chunk.end_exclusive, 0)
        };
        Some(self.snapshot.anchor_before(start)..self.snapshot.anchor_after(end))
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
}

impl RowChunk {
    pub fn row_range(&self) -> Range<BufferRow> {
        self.start..self.end_exclusive
    }
}
