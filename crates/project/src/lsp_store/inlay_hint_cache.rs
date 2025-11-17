use std::{collections::hash_map, ops::Range, sync::Arc};

use collections::HashMap;
use futures::future::Shared;
use gpui::{App, Entity, Task};
use language::{Buffer, BufferRow, BufferSnapshot};
use lsp::LanguageServerId;
use text::OffsetRangeExt;
use util::RangeExt as _;

use crate::{InlayHint, InlayId};

pub type CacheInlayHints = HashMap<LanguageServerId, Vec<(InlayId, InlayHint)>>;
pub type CacheInlayHintsTask = Shared<Task<Result<CacheInlayHints, Arc<anyhow::Error>>>>;

/// A logic to apply when querying for new inlay hints and deciding what to do with the old entries in the cache in case of conflicts.
#[derive(Debug, Clone, Copy)]
pub enum InvalidationStrategy {
    /// Language servers reset hints via <a href="https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_inlayHint_refresh">request</a>.
    /// Demands to re-query all inlay hints needed and invalidate all cached entries, but does not require instant update with invalidation.
    ///
    /// Despite nothing forbids language server from sending this request on every edit, it is expected to be sent only when certain internal server state update, invisible for the editor otherwise.
    RefreshRequested {
        server_id: LanguageServerId,
        request_id: Option<usize>,
    },
    /// Multibuffer excerpt(s) and/or singleton buffer(s) were edited at least on one place.
    /// Neither editor nor LSP is able to tell which open file hints' are not affected, so all of them have to be invalidated, re-queried and do that fast enough to avoid being slow, but also debounce to avoid loading hints on every fast keystroke sequence.
    BufferEdited,
    /// A new file got opened/new excerpt was added to a multibuffer/a [multi]buffer was scrolled to a new position.
    /// No invalidation should be done at all, all new hints are added to the cache.
    ///
    /// A special case is the editor toggles and settings change:
    /// in addition to LSP capabilities, Zed allows omitting certain hint kinds (defined by the corresponding LSP part: type/parameter/other) and toggling hints.
    /// This does not lead to cache invalidation, but would require cache usage for determining which hints are not displayed and issuing an update to inlays on the screen.
    None,
}

impl InvalidationStrategy {
    pub fn should_invalidate(&self) -> bool {
        matches!(
            self,
            InvalidationStrategy::RefreshRequested { .. } | InvalidationStrategy::BufferEdited
        )
    }
}

pub struct BufferInlayHints {
    snapshot: BufferSnapshot,
    buffer_chunks: Vec<BufferChunk>,
    hints_by_chunks: Vec<Option<CacheInlayHints>>,
    fetches_by_chunks: Vec<Option<CacheInlayHintsTask>>,
    hints_by_id: HashMap<InlayId, HintForId>,
    latest_invalidation_requests: HashMap<LanguageServerId, Option<usize>>,
    pub(super) hint_resolves: HashMap<InlayId, Shared<Task<()>>>,
}

#[derive(Debug, Clone, Copy)]
struct HintForId {
    chunk_id: usize,
    server_id: LanguageServerId,
    position: usize,
}

/// An range of rows, exclusive as [`lsp::Range`] and
/// <https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#range>
/// denote.
///
/// Represents an area in a text editor, adjacent to other ones.
/// Together, chunks form entire document at a particular version [`clock::Global`].
/// Each chunk is queried for inlays as `(start_row, 0)..(end_exclusive, 0)` via
/// <https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#inlayHintParams>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferChunk {
    pub id: usize,
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
        let last_row = buffer_point_range.end.row;
        let buffer_chunks = (buffer_point_range.start.row..=last_row)
            .step_by(MAX_ROWS_IN_A_CHUNK as usize)
            .enumerate()
            .map(|(id, chunk_start)| BufferChunk {
                id,
                start: chunk_start,
                end: (chunk_start + MAX_ROWS_IN_A_CHUNK).min(last_row),
            })
            .collect::<Vec<_>>();

        Self {
            hints_by_chunks: vec![None; buffer_chunks.len()],
            fetches_by_chunks: vec![None; buffer_chunks.len()],
            latest_invalidation_requests: HashMap::default(),
            hints_by_id: HashMap::default(),
            hint_resolves: HashMap::default(),
            snapshot,
            buffer_chunks,
        }
    }

    pub fn applicable_chunks(
        &self,
        ranges: &[Range<text::Anchor>],
    ) -> impl Iterator<Item = BufferChunk> {
        let row_ranges = ranges
            .iter()
            .map(|range| range.to_point(&self.snapshot))
            // Be lenient and yield multiple chunks if they "touch" the exclusive part of the range.
            // This will result in LSP hints [re-]queried for more ranges, but also more hints already visible when scrolling around.
            .map(|point_range| point_range.start.row..point_range.end.row + 1)
            .collect::<Vec<_>>();
        self.buffer_chunks
            .iter()
            .filter(move |chunk| {
                let chunk_range = chunk.start..=chunk.end;
                row_ranges
                    .iter()
                    .any(|row_range| chunk_range.overlaps(&row_range))
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
        self.latest_invalidation_requests.clear();
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
        *self.fetched_hints(&chunk) = None;
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

    pub fn buffer_chunks_len(&self) -> usize {
        self.buffer_chunks.len()
    }

    pub(crate) fn invalidate_for_server_refresh(
        &mut self,
        for_server: LanguageServerId,
        request_id: Option<usize>,
    ) -> bool {
        match self.latest_invalidation_requests.entry(for_server) {
            hash_map::Entry::Occupied(mut o) => {
                if request_id > *o.get() {
                    o.insert(request_id);
                } else {
                    return false;
                }
            }
            hash_map::Entry::Vacant(v) => {
                v.insert(request_id);
            }
        }

        for (chunk_id, chunk_data) in self.hints_by_chunks.iter_mut().enumerate() {
            if let Some(removed_hints) = chunk_data
                .as_mut()
                .and_then(|chunk_data| chunk_data.remove(&for_server))
            {
                for (id, _) in removed_hints {
                    self.hints_by_id.remove(&id);
                    self.hint_resolves.remove(&id);
                }
                self.fetches_by_chunks[chunk_id] = None;
            }
        }

        true
    }

    pub(crate) fn invalidate_for_chunk(&mut self, chunk: BufferChunk) {
        self.fetches_by_chunks[chunk.id] = None;
        if let Some(hints_by_server) = self.hints_by_chunks[chunk.id].take() {
            for (hint_id, _) in hints_by_server.into_values().flatten() {
                self.hints_by_id.remove(&hint_id);
                self.hint_resolves.remove(&hint_id);
            }
        }
    }
}
