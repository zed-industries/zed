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

#[derive(Debug)]
pub struct InlayHintCache {
    buffer_store: Entity<BufferStore>,
    hints_for_version: Global,
    hints: HashMap<LanguageServerId, Hints>,
    cache_version: usize,
}

#[derive(Debug, Default)]
struct Hints {
    hints: HashMap<InlayHintId, InlayHint>,
    hints_by_chunks: BTreeMap<Range<BufferRow>, Option<Vec<InlayHintId>>>,
    hint_updates: HashMap<Range<BufferRow>, Shared<Task<InlayHints>>>,
}

pub struct InlayHints {
    pub cache_version: usize,
    pub hints: Vec<InlayHint>,
}

impl InlayHintCache {
    pub fn remove_server_data(&mut self, for_server: LanguageServerId) -> bool {
        let removed = self.hints.remove(&for_server).is_some();
        if removed {
            self.cache_version += 1;
        }
        removed
    }

    pub fn hints(
        &self,
        buffer: BufferId,
        range: Range<usize>,
        known_cache_version: Option<usize>,
        cx: &mut Context<Self>,
    ) -> Option<(Range<BufferRow>, Shared<Task<InlayHints>>)> {
        todo!("TODO kb")
    }
    // we want to store the cache version outbound, so they can query with it: we can return nothing (`Option`) if the version matches
    // we can get a new server up/down, so we want to re-query for them things, ignoring the cache version
}
