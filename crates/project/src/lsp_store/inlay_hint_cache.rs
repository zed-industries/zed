use std::{collections::hash_map, ops::Range, sync::Arc};

use collections::HashMap;
use futures::future::Shared;
use gpui::{App, Entity, Task};
use language::{Buffer, BufferRow, BufferSnapshot};
use lsp::LanguageServerId;
use text::OffsetRangeExt;

use crate::{InlayHint, InlayId};

pub type CacheInlayHints = HashMap<LanguageServerId, Vec<(InlayId, InlayHint)>>;
pub type CacheInlayHintsTask = Shared<Task<Result<CacheInlayHints, Arc<anyhow::Error>>>>;

pub struct RowChunkCachedHints {
    pub hints: CacheInlayHints,
    pub cached: bool,
}

pub struct BufferInlayHints {
    snapshot: BufferSnapshot,
    buffer_chunks: Vec<BufferChunk>,
    hints_by_chunks: Vec<Option<CacheInlayHints>>,
    fetches_by_chunks: Vec<Option<CacheInlayHintsTask>>,
    hints_by_id: HashMap<InlayId, HintForId>,
    pub(super) hint_resolves: HashMap<InlayId, Shared<Task<()>>>,
}

#[derive(Debug, Clone, Copy)]
struct HintForId {
    chunk_id: usize,
    server_id: LanguageServerId,
    position: usize,
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
            .field("buffer_chunks", &self.buffer_chunks)
            .field("hints_by_chunks", &self.hints_by_chunks)
            .field("fetches_by_chunks", &self.fetches_by_chunks)
            .field("hints_by_id", &self.hints_by_id)
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
            hints_by_chunks: vec![None; buffer_chunks.len()],
            fetches_by_chunks: vec![None; buffer_chunks.len()],
            hints_by_id: HashMap::default(),
            hint_resolves: HashMap::default(),
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

    pub fn cached_hints(&mut self, chunk: &BufferChunk) -> Option<&CacheInlayHints> {
        self.hints_by_chunks[chunk.id].as_ref()
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
            .map(|(_, hint)| hint)
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

    pub fn clear(&mut self) {
        self.hints_by_chunks = vec![None; self.buffer_chunks.len()];
        self.fetches_by_chunks = vec![None; self.buffer_chunks.len()];
        self.hints_by_id.clear();
        self.hint_resolves.clear();
    }

    pub fn insert_new_hints(
        &mut self,
        chunk: BufferChunk,
        server_id: LanguageServerId,
        new_hints: Vec<(InlayId, InlayHint)>,
    ) {
        let existing_hints = self.hints_by_chunks[chunk.id]
            .get_or_insert_default()
            .entry(server_id)
            .or_insert_with(Vec::new);
        let existing_count = existing_hints.len();
        existing_hints.extend(new_hints.into_iter().enumerate().filter_map(
            |(i, (id, new_hint))| {
                let new_hint_for_id = HintForId {
                    chunk_id: chunk.id,
                    server_id,
                    position: existing_count + i,
                };
                if let hash_map::Entry::Vacant(vacant_entry) = self.hints_by_id.entry(id) {
                    vacant_entry.insert(new_hint_for_id);
                    Some((id, new_hint))
                } else {
                    None
                }
            },
        ));
    }

    pub fn hint_for_id(&mut self, id: InlayId) -> Option<&mut InlayHint> {
        let hint_for_id = self.hints_by_id.get(&id)?;
        let (hint_id, hint) = self
            .hints_by_chunks
            .get_mut(hint_for_id.chunk_id)?
            .as_mut()?
            .get_mut(&hint_for_id.server_id)?
            .get_mut(hint_for_id.position)?;
        debug_assert_eq!(*hint_id, id, "Invalid pointer {hint_for_id:?}");
        Some(hint)
    }
}
