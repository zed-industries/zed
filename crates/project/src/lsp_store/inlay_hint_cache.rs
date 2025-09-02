use std::{ops::Range, sync::Arc};

use clock::Global;
use collections::HashMap;
use futures::future::Shared;
use gpui::{App, Entity, Task};
use language::{Buffer, BufferRow, BufferSnapshot};
use lsp::LanguageServerId;
use text::OffsetRangeExt;

use crate::InlayHint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InlayHintId(usize);

pub struct BufferInlayHints {
    pub all_hints: HashMap<InlayHintId, InlayHint>,
    pub hints: HashMap<
        Range<BufferRow>,
        HashMap<LanguageServerId, Shared<Task<Result<Vec<InlayHint>, Arc<anyhow::Error>>>>>,
    >,
    pub chunks_for_version: Global,
    pub buffer_chunks: Vec<Range<BufferRow>>,
    snapshot: BufferSnapshot,
}

impl std::fmt::Debug for BufferInlayHints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferInlayHints")
            .field("all_hints", &self.all_hints)
            .field("hints", &self.hints)
            .field("chunks_for_version", &self.chunks_for_version)
            .field("buffer_chunks", &self.buffer_chunks)
            .finish_non_exhaustive()
    }
}

const MAX_ROWS_IN_A_CHUNK: u32 = 50;

impl BufferInlayHints {
    pub fn new(buffer: &Entity<Buffer>, cx: &mut App) -> Self {
        let buffer = buffer.read(cx);
        let snapshot = buffer.snapshot();
        let buffer_point_range = (0..buffer.len()).to_point(&snapshot);
        let buffer_row_range = buffer_point_range.start.row..buffer_point_range.end.row;
        // TODO kb recheck
        let buffer_chunks: Vec<Range<BufferRow>> = buffer_row_range
            .clone()
            .step_by(MAX_ROWS_IN_A_CHUNK as usize)
            .map(|chunk_start| {
                let chunk_end =
                    std::cmp::min(chunk_start + MAX_ROWS_IN_A_CHUNK, buffer_row_range.end);
                chunk_start..chunk_end
            })
            .collect();

        Self {
            all_hints: HashMap::default(),
            hints: HashMap::default(),
            chunks_for_version: buffer.version(),
            snapshot,
            buffer_chunks,
        }
    }

    pub fn applicable_chunks(&self, range: &Range<text::Anchor>) -> Vec<Range<BufferRow>> {
        let point_range = range.to_point(&self.snapshot);
        let row_range = point_range.start.row..point_range.end.row;
        self.buffer_chunks
            .iter()
            .filter(|chunk_range| {
                // TODO kb recheck
                row_range.contains(&chunk_range.start) || row_range.contains(&chunk_range.end)
            })
            .cloned()
            .collect()
    }

    pub fn remove_server_data(&mut self, for_server: LanguageServerId) {
        for hints in self.hints.values_mut() {
            hints.remove(&for_server);
        }
    }

    pub fn cached_inlay_hints(
        &self,
        buffer: Entity<Buffer>,
        range: Range<text::Anchor>,
        cx: &mut App,
    ) -> Option<(
        Range<BufferRow>,
        Shared<Task<Result<Vec<InlayHint>, Arc<anyhow::Error>>>>,
    )> {
        todo!("TODO kb")
    }

    // we want to store the cache version outbound, so they can query with it: we can return nothing (`Option`) if the version matches
    // we can get a new server up/down, so we want to re-query for them things, ignoring the cache version
}
