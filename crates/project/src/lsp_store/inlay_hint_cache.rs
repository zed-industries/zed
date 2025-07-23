use std::{collections::BTreeMap, ops::Range};

use clock::Global;
use collections::HashMap;
use futures::future::Shared;
use gpui::{Context, Entity, Task};
use language::BufferRow;
use lsp::LanguageServerId;
use text::BufferId;

use crate::{InlayHint, buffer_store::BufferStore};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InlayHintId(usize);

#[derive(Debug, Default)]
pub struct BufferInlayHints {
    all_hints: HashMap<InlayHintId, InlayHint>,
    hints: HashMap<LanguageServerId, HintChunks>,
    chunks_for_version: Global,
    cache_version: usize,
}

#[derive(Debug, Default)]
struct HintChunks {
    hints_by_chunks: BTreeMap<Range<BufferRow>, Option<Vec<InlayHintId>>>,
    chunk_updates: HashMap<Range<BufferRow>, Shared<Task<InlayHints>>>,
}

pub struct InlayHints {
    pub cache_version: usize,
    pub hints: Vec<InlayHint>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HintFetchStrategy {
    IgnoreCache,
    UseCache { known_cache_version: Option<usize> },
}

impl BufferInlayHints {
    pub fn remove_server_data(&mut self, for_server: LanguageServerId) -> bool {
        let removed = self.hints.remove(&for_server).is_some();
        if removed {
            self.cache_version += 1;
        }
        removed
    }

    pub fn hints(
        &self,
        buffer_store: Entity<BufferStore>,
        buffer: BufferId,
        strategy: HintFetchStrategy,
        range: impl text::ToOffset,
        cx: &mut Context<Self>,
    ) -> Option<(Range<BufferRow>, Shared<Task<InlayHints>>)> {
        todo!("TODO kb")
    }
    // we want to store the cache version outbound, so they can query with it: we can return nothing (`Option`) if the version matches
    // we can get a new server up/down, so we want to re-query for them things, ignoring the cache version
}
