use std::path::Path;

use gpui::SharedString;

use crate::{
    context::{Context, ContextId, ContextKind},
    thread::ThreadId,
};

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

    pub fn id_for_file(&self, path: &Path) -> Option<IncludedFile> {
        self.context.iter().find_map(|probe| match &probe.kind {
            ContextKind::File(probe_path) if probe_path == path => {
                Some(IncludedFile::Direct(probe.id))
            }
            ContextKind::Directory(probe_dir) if path.starts_with(probe_dir) => {
                Some(IncludedFile::InDirectory(probe.name.clone()))
            }
            ContextKind::File(_)
            | ContextKind::Directory(_)
            | ContextKind::FetchedUrl(_)
            | ContextKind::Thread(_) => None,
        })
    }

    pub fn id_for_directory(&self, path: &Path) -> Option<ContextId> {
        self.context
            .iter()
            .find(|probe| match &probe.kind {
                ContextKind::Directory(probe_path) => probe_path == path,
                ContextKind::File(_) | ContextKind::FetchedUrl(_) | ContextKind::Thread(_) => false,
            })
            .map(|context| context.id)
    }

    pub fn id_for_thread(&self, thread_id: &ThreadId) -> Option<ContextId> {
        self.context
            .iter()
            .find(|probe| match probe.kind {
                ContextKind::Thread(ref probe_thread_id) => probe_thread_id == thread_id,
                ContextKind::File(_) | ContextKind::Directory(_) | ContextKind::FetchedUrl(_) => {
                    false
                }
            })
            .map(|context| context.id)
    }

    pub fn id_for_url(&self, url: &str) -> Option<ContextId> {
        self.context
            .iter()
            .find(|probe| match &probe.kind {
                ContextKind::FetchedUrl(probe_url) => probe_url == url,
                ContextKind::File(_) | ContextKind::Directory(_) | ContextKind::Thread(_) => false,
            })
            .map(|context| context.id)
    }
}

pub enum IncludedFile {
    Direct(ContextId),
    InDirectory(SharedString),
}
