//! A row chunk is an exclusive range of rows, [`BufferRow`] within a buffer of a certain version, [`Global`].
//! All but the last chunk are of a constant, given size.

use std::{collections::BTreeSet, ops::Range};

use collections::HashMap;
use parking_lot::Mutex;
use text::{Anchor, Point};
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
///
/// Chunk boundaries are derived arithmetically from the buffer's last row.
/// Each chunk's anchors are computed when the chunk is first queried and memoized:
/// resolving an anchor requires sum tree seeks, which gets expensive if done for
/// all chunks of a large buffer at once.
pub struct RowChunks {
    buffer_snapshot: text::BufferSnapshot,
    last_row: BufferRow,
    max_rows_per_chunk: u32,
    computed_chunks: Mutex<HashMap<usize, RowChunk>>,
}

impl std::fmt::Debug for RowChunks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RowChunks")
            .field("len", &self.len())
            .field("max_rows_per_chunk", &self.max_rows_per_chunk)
            .field("computed_chunks", &self.computed_chunks.lock().len())
            .finish()
    }
}

impl RowChunks {
    pub fn new(snapshot: &text::BufferSnapshot, max_rows_per_chunk: u32) -> Self {
        Self {
            last_row: snapshot.max_point().row,
            buffer_snapshot: snapshot.clone(),
            max_rows_per_chunk,
            computed_chunks: Mutex::new(HashMap::default()),
        }
    }

    pub fn version(&self) -> &clock::Global {
        self.buffer_snapshot.version()
    }

    pub fn len(&self) -> usize {
        (self.last_row / self.max_rows_per_chunk) as usize + 1
    }

    pub fn applicable_chunks(&self, ranges: &[Range<Point>]) -> impl Iterator<Item = RowChunk> {
        let last_chunk_id = self.len() - 1;
        let mut chunk_ids = BTreeSet::new();
        for point_range in ranges {
            // Be lenient and yield multiple chunks if they "touch" the exclusive part of the range.
            // This will result in LSP hints [re-]queried for more ranges, but also more hints already visible when scrolling around.
            let row_range = point_range.start.row..point_range.end.row + 1;
            let first_id = (point_range.start.row.div_ceil(self.max_rows_per_chunk) as usize)
                .saturating_sub(1);
            let last_id =
                ((point_range.end.row / self.max_rows_per_chunk) as usize).min(last_chunk_id);
            for id in first_id..=last_id {
                if self.chunk_row_range(id).to_inclusive().overlaps(&row_range) {
                    chunk_ids.insert(id);
                }
            }
        }
        chunk_ids.into_iter().map(|id| self.chunk(id))
    }

    pub fn previous_chunk(&self, chunk: RowChunk) -> Option<RowChunk> {
        if chunk.id == 0 || chunk.id > self.len() {
            None
        } else {
            Some(self.chunk(chunk.id - 1))
        }
    }

    fn chunk_row_range(&self, id: usize) -> Range<BufferRow> {
        let start = id as u32 * self.max_rows_per_chunk;
        start..(start + self.max_rows_per_chunk).min(self.last_row)
    }

    fn chunk(&self, id: usize) -> RowChunk {
        *self.computed_chunks.lock().entry(id).or_insert_with(|| {
            let row_range = self.chunk_row_range(id);
            let start = Point::new(row_range.start, 0);
            let end = if id == self.len() - 1 {
                Point::new(row_range.end, self.buffer_snapshot.line_len(row_range.end))
            } else {
                Point::new(row_range.end, 0)
            };
            RowChunk {
                id,
                start: row_range.start,
                end_exclusive: row_range.end,
                start_anchor: self.buffer_snapshot.anchor_before(start),
                end_anchor: self.buffer_snapshot.anchor_after(end),
            }
        })
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
