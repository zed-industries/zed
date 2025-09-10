use std::{ops::Range, sync::Arc};

use clock::Global;
use collections::HashMap;
use futures::future::Shared;
use gpui::{App, Entity, Task};
use language::{Buffer, BufferRow, BufferSnapshot};
use lsp::LanguageServerId;
use text::OffsetRangeExt;

use crate::InlayHint;

pub type CacheInlayHints = HashMap<LanguageServerId, Vec<InlayHint>>;
pub type CacheInlayHintsTask = Shared<Task<Result<CacheInlayHints, Arc<anyhow::Error>>>>;

pub struct RowChunkCachedHints {
    pub hints: CacheInlayHints,
    pub cached: bool,
}

pub struct BufferInlayHints {
    pub chunks_for_version: Global,
    snapshot: BufferSnapshot,
    pub(super) buffer_chunks: Vec<BufferChunk>,
    pub(super) hints_by_chunks: Vec<Option<CacheInlayHints>>,
    pub(super) fetches_by_chunks: Vec<Option<CacheInlayHintsTask>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferChunk {
    id: usize,
    pub start: BufferRow,
    pub end: BufferRow,
}

impl std::fmt::Debug for BufferInlayHints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferInlayHints")
            .field("chunks_for_version", &self.chunks_for_version)
            .field("buffer_chunks", &self.buffer_chunks)
            .field("hints_by_chunks", &self.hints_by_chunks)
            .field("fetches_by_chunks", &self.fetches_by_chunks)
            .finish_non_exhaustive()
    }
}

const MAX_ROWS_IN_A_CHUNK: u32 = 50;

impl BufferInlayHints {
    pub fn new(buffer: &Entity<Buffer>, cx: &mut App) -> Self {
        let buffer = buffer.read(cx);
        let snapshot = buffer.snapshot();
        let buffer_point_range = (0..buffer.len()).to_point(&snapshot);
        let buffer_row_range = buffer_point_range.start.row..=buffer_point_range.end.row;
        let buffer_chunks = buffer_row_range
            .clone()
            .step_by(MAX_ROWS_IN_A_CHUNK as usize)
            .enumerate()
            .map(|(id, chunk_start)| {
                let chunk_end =
                    std::cmp::min(chunk_start + MAX_ROWS_IN_A_CHUNK, *buffer_row_range.end());
                BufferChunk {
                    id,
                    start: chunk_start,
                    end: chunk_end,
                }
            })
            .collect::<Vec<_>>();

        Self {
            chunks_for_version: buffer.version(),
            hints_by_chunks: vec![None; buffer_chunks.len()],
            fetches_by_chunks: vec![None; buffer_chunks.len()],
            snapshot,
            buffer_chunks,
        }
    }

    pub fn applicable_chunks(
        &self,
        range: &Range<text::Anchor>,
    ) -> impl Iterator<Item = BufferChunk> {
        let point_range = range.to_point(&self.snapshot);
        let row_range = point_range.start.row..=point_range.end.row;
        self.buffer_chunks
            .iter()
            .filter(move |chunk_range| {
                row_range.contains(&chunk_range.start) || row_range.contains(&chunk_range.end)
            })
            .copied()
    }

    pub fn cached_hints(&mut self, chunk: &BufferChunk) -> &mut Option<CacheInlayHints> {
        &mut self.hints_by_chunks[chunk.id]
    }

    pub fn fetched_hints(&mut self, chunk: &BufferChunk) -> &mut Option<CacheInlayHintsTask> {
        &mut self.fetches_by_chunks[chunk.id]
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn all_cached_hints(&self) -> Vec<InlayHint> {
        self.hints_by_chunks
            .iter()
            .filter_map(|hints| hints.as_ref())
            .flat_map(|hints| hints.values().cloned())
            .flatten()
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn all_fetched_hints(&self) -> Vec<CacheInlayHintsTask> {
        self.fetches_by_chunks
            .iter()
            .filter_map(|fetches| fetches.clone())
            .collect()
    }

    pub fn remove_server_data(&mut self, for_server: LanguageServerId) {
        for (chunk_index, hints) in self.hints_by_chunks.iter_mut().enumerate() {
            if let Some(hints) = hints {
                if hints.remove(&for_server).is_some() {
                    self.fetches_by_chunks[chunk_index] = None;
                }
            }
        }
    }
}
