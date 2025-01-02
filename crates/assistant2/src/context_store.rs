use gpui::SharedString;
use project::ProjectEntryId;

use crate::context::{Context, ContextId, ContextKind};

pub struct ContextStore {
    context: Vec<Context>,
    next_context_id: ContextId,
}

impl ContextStore {
    pub fn new() -> Self {
        Self {
            context: Vec::new(),
            next_context_id: ContextId(0),
        }
    }

    pub fn context(&self) -> &Vec<Context> {
        &self.context
    }

    pub fn drain(&mut self) -> Vec<Context> {
        self.context.drain(..).collect()
    }

    pub fn clear(&mut self) {
        self.context.clear();
    }

    pub fn insert_context(
        &mut self,
        kind: ContextKind,
        name: impl Into<SharedString>,
        text: impl Into<SharedString>,
    ) {
        self.context.push(Context {
            id: self.next_context_id.post_inc(),
            name: name.into(),
            kind,
            text: text.into(),
        });
    }

    pub fn remove_context(&mut self, id: &ContextId) {
        self.context.retain(|context| context.id != *id);
    }

    pub fn contains_project_entry(&self, entry_id: ProjectEntryId) -> bool {
        self.context.iter().any(|probe| match probe.kind {
            ContextKind::File(probe_entry_id) => probe_entry_id == entry_id,
            ContextKind::Directory => false,
            ContextKind::FetchedUrl => false,
            ContextKind::Thread => false,
        })
    }
}
