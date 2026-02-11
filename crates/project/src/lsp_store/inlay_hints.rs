use std::{collections::hash_map, ops::Range, sync::Arc};

use anyhow::{Context as _, Result};
use collections::HashMap;
use futures::future::Shared;
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, Task};
use language::{
    Buffer,
    row_chunk::{RowChunk, RowChunks},
};
use lsp::LanguageServerId;
use rpc::{TypedEnvelope, proto};
use settings::Settings as _;
use text::{BufferId, Point};

use crate::{
    InlayHint, InlayId, LspStore, LspStoreEvent, ResolveState, lsp_command::InlayHints,
    project_settings::ProjectSettings,
};

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
    chunks: RowChunks,
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

impl std::fmt::Debug for BufferInlayHints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferInlayHints")
            .field("buffer_chunks", &self.chunks)
            .field("hints_by_chunks", &self.hints_by_chunks)
            .field("fetches_by_chunks", &self.fetches_by_chunks)
            .field("hints_by_id", &self.hints_by_id)
            .finish_non_exhaustive()
    }
}

const MAX_ROWS_IN_A_CHUNK: u32 = 50;

impl BufferInlayHints {
    pub fn new(buffer: &Entity<Buffer>, cx: &mut App) -> Self {
        let chunks = RowChunks::new(buffer.read(cx).as_text_snapshot(), MAX_ROWS_IN_A_CHUNK);

        Self {
            hints_by_chunks: vec![None; chunks.len()],
            fetches_by_chunks: vec![None; chunks.len()],
            latest_invalidation_requests: HashMap::default(),
            hints_by_id: HashMap::default(),
            hint_resolves: HashMap::default(),
            chunks,
        }
    }

    pub fn applicable_chunks(&self, ranges: &[Range<Point>]) -> impl Iterator<Item = RowChunk> {
        self.chunks.applicable_chunks(ranges)
    }

    pub fn cached_hints(&mut self, chunk: &RowChunk) -> Option<&CacheInlayHints> {
        self.hints_by_chunks[chunk.id].as_ref()
    }

    pub fn fetched_hints(&mut self, chunk: &RowChunk) -> &mut Option<CacheInlayHintsTask> {
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
        self.hints_by_chunks = vec![None; self.chunks.len()];
        self.fetches_by_chunks = vec![None; self.chunks.len()];
        self.hints_by_id.clear();
        self.hint_resolves.clear();
        self.latest_invalidation_requests.clear();
    }

    pub fn insert_new_hints(
        &mut self,
        chunk: RowChunk,
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

    pub(crate) fn invalidate_for_chunk(&mut self, chunk: RowChunk) {
        self.fetches_by_chunks[chunk.id] = None;
        if let Some(hints_by_server) = self.hints_by_chunks[chunk.id].take() {
            for (hint_id, _) in hints_by_server.into_values().flatten() {
                self.hints_by_id.remove(&hint_id);
                self.hint_resolves.remove(&hint_id);
            }
        }
    }
}

impl LspStore {
    pub(super) fn resolve_inlay_hint(
        &self,
        mut hint: InlayHint,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<InlayHint>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            if !self.check_if_capable_for_proto_request(&buffer, InlayHints::can_resolve_inlays, cx)
            {
                hint.resolve_state = ResolveState::Resolved;
                return Task::ready(Ok(hint));
            }
            let request = proto::ResolveInlayHint {
                project_id,
                buffer_id: buffer.read(cx).remote_id().into(),
                language_server_id: server_id.0 as u64,
                hint: Some(InlayHints::project_to_proto_hint(hint.clone())),
            };
            cx.background_spawn(async move {
                let response = upstream_client
                    .request(request)
                    .await
                    .context("inlay hints proto request")?;
                match response.hint {
                    Some(resolved_hint) => InlayHints::proto_to_project_hint(resolved_hint)
                        .context("inlay hints proto resolve response conversion"),
                    None => Ok(hint),
                }
            })
        } else {
            let Some(lang_server) = buffer.update(cx, |buffer, cx| {
                self.language_server_for_local_buffer(buffer, server_id, cx)
                    .map(|(_, server)| server.clone())
            }) else {
                return Task::ready(Ok(hint));
            };
            if !InlayHints::can_resolve_inlays(&lang_server.capabilities()) {
                return Task::ready(Ok(hint));
            }
            let buffer_snapshot = buffer.read(cx).snapshot();
            let request_timeout = ProjectSettings::get_global(cx)
                .global_lsp_settings
                .get_request_timeout();
            cx.spawn(async move |_, cx| {
                let resolve_task = lang_server.request::<lsp::request::InlayHintResolveRequest>(
                    InlayHints::project_to_lsp_hint(hint, &buffer_snapshot),
                    request_timeout,
                );
                let resolved_hint = resolve_task
                    .await
                    .into_response()
                    .context("inlay hint resolve LSP request")?;
                let resolved_hint = InlayHints::lsp_to_project_hint(
                    resolved_hint,
                    &buffer,
                    server_id,
                    ResolveState::Resolved,
                    false,
                    cx,
                )
                .await?;
                Ok(resolved_hint)
            })
        }
    }

    pub(super) async fn handle_refresh_inlay_hints(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::RefreshInlayHints>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        lsp_store.update(&mut cx, |_, cx| {
            cx.emit(LspStoreEvent::RefreshInlayHints {
                server_id: LanguageServerId::from_proto(envelope.payload.server_id),
                request_id: envelope.payload.request_id.map(|id| id as usize),
            });
        });
        Ok(proto::Ack {})
    }

    pub(super) async fn handle_resolve_inlay_hint(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::ResolveInlayHint>,
        mut cx: AsyncApp,
    ) -> Result<proto::ResolveInlayHintResponse> {
        let proto_hint = envelope
            .payload
            .hint
            .expect("incorrect protobuf resolve inlay hint message: missing the inlay hint");
        let hint = InlayHints::proto_to_project_hint(proto_hint)
            .context("resolved proto inlay hint conversion")?;
        let buffer = lsp_store.update(&mut cx, |lsp_store, cx| {
            let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
            lsp_store.buffer_store.read(cx).get_existing(buffer_id)
        })?;
        let response_hint = lsp_store
            .update(&mut cx, |lsp_store, cx| {
                lsp_store.resolve_inlay_hint(
                    hint,
                    buffer,
                    LanguageServerId(envelope.payload.language_server_id as usize),
                    cx,
                )
            })
            .await
            .context("inlay hints fetch")?;
        Ok(proto::ResolveInlayHintResponse {
            hint: Some(InlayHints::project_to_proto_hint(response_hint)),
        })
    }
}
