use std::path::{Path, PathBuf};

use collections::HashMap;
use gpui::SharedString;

use crate::{
    context::{Context, ContextId, ContextKind},
    thread::ThreadId,
};

pub struct ContextStore {
    context: Vec<Context>,
    next_context_id: ContextId,
    files: HashMap<PathBuf, ContextId>,
    directories: HashMap<PathBuf, ContextId>,
}

impl ContextStore {
    pub fn new() -> Self {
        Self {
            context: Vec::new(),
            next_context_id: ContextId(0),
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    pub fn context(&self) -> &Vec<Context> {
        &self.context
    }

    pub fn drain(&mut self) -> Vec<Context> {
        self.files.clear();
        self.directories.clear();
        self.context.drain(..).collect()
    }

    pub fn clear(&mut self) {
        self.context.clear();
        self.files.clear();
        self.directories.clear();
    }

    pub fn insert_context(
        &mut self,
        kind: ContextKind,
        name: impl Into<SharedString>,
        text: impl Into<SharedString>,
    ) {
        let id = self.next_context_id.post_inc();

        match &kind {
            ContextKind::File(path) => {
                self.files.insert(path.clone(), id);
            }
            ContextKind::Directory(path) => {
                self.directories.insert(path.clone(), id);
            }
            ContextKind::FetchedUrl(_) | ContextKind::Thread(_) => {}
        }

        self.context.push(Context {
            id,
            name: name.into(),
            kind,
            text: text.into(),
        });
    }

    pub fn remove_context(&mut self, id: &ContextId) {
        let Some(ix) = self.context.iter().position(|c| c.id == *id) else {
            return;
        };

        match self.context.remove(ix).kind {
            ContextKind::File(path) => {
                self.files.remove(&path);
            }
            ContextKind::Directory(path) => {
                self.directories.remove(&path);
            }
            ContextKind::FetchedUrl(_) | ContextKind::Thread(_) => {}
        }
    }

    pub fn id_for_file(&self, path: &Path) -> Option<IncludedFile> {
        if let Some(id) = self.files.get(path) {
            return Some(IncludedFile::Direct(*id));
        }

        if self.directories.is_empty() {
            return None;
        }

        let mut buf = path.to_path_buf();

        while buf.pop() {
            if let Some(_) = self.directories.get(&buf) {
                return Some(IncludedFile::InDirectory(buf));
            }
        }

        None
    }

    pub fn id_for_directory(&self, path: &Path) -> Option<ContextId> {
        self.directories.get(path).copied()
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
    InDirectory(PathBuf),
}
