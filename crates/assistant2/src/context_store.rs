use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use collections::{HashMap, HashSet};
use gpui::SharedString;
use language::Buffer;

use crate::thread::Thread;
use crate::{
    context::{Context, ContextId, ContextKind},
    thread::ThreadId,
};

pub struct ContextStore {
    context: Vec<Context>,
    next_context_id: ContextId,
    files: HashMap<PathBuf, ContextId>,
    directories: HashMap<PathBuf, ContextId>,
    threads: HashMap<ThreadId, ContextId>,
    fetched_urls: HashMap<String, ContextId>,
}

impl ContextStore {
    pub fn new() -> Self {
        Self {
            context: Vec::new(),
            next_context_id: ContextId(0),
            files: HashMap::default(),
            directories: HashMap::default(),
            threads: HashMap::default(),
            fetched_urls: HashMap::default(),
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

    pub fn insert_file(&mut self, buffer: &Buffer) {
        let Some(file) = buffer.file() else {
            return;
        };

        let path = file.path();

        let id = self.next_context_id.post_inc();
        self.files.insert(path.to_path_buf(), id);

        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().into(),
            None => path.to_string_lossy().into_owned().into(),
        };

        let mut text = String::new();
        push_fenced_codeblock(path, buffer.text(), &mut text);

        self.context.push(Context {
            id,
            name,
            path: Some(path.to_path_buf()),
            kind: ContextKind::File,
            text: text.into(),
        });
    }

    pub fn insert_directory(&mut self, path: &Path, text: impl Into<SharedString>) {
        let id = self.next_context_id.post_inc();
        self.directories.insert(path.to_path_buf(), id);

        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().into(),
            None => path.to_string_lossy().into_owned().into(),
        };

        self.context.push(Context {
            id,
            name,
            path: Some(path.to_path_buf()),
            kind: ContextKind::Directory,
            text: text.into(),
        });
    }

    pub fn insert_thread(&mut self, thread: &Thread) {
        let context_id = self.next_context_id.post_inc();
        self.threads.insert(thread.id().clone(), context_id);

        self.context.push(Context {
            id: context_id,
            name: thread.summary().unwrap_or("New thread".into()),
            path: None,
            kind: ContextKind::Thread,
            text: thread.text().into(),
        });
    }

    pub fn insert_fetched_url(&mut self, url: String, text: impl Into<SharedString>) {
        let context_id = self.next_context_id.post_inc();
        self.fetched_urls.insert(url.clone(), context_id);

        self.context.push(Context {
            id: context_id,
            name: url.into(),
            path: None,
            kind: ContextKind::FetchedUrl,
            text: text.into(),
        });
    }

    pub fn remove_context(&mut self, id: &ContextId) {
        let Some(ix) = self.context.iter().position(|c| c.id == *id) else {
            return;
        };

        match self.context.remove(ix).kind {
            ContextKind::File => {
                self.files.retain(|_, p_id| p_id != id);
            }
            ContextKind::Directory => {
                self.directories.retain(|_, p_id| p_id != id);
            }
            ContextKind::FetchedUrl => {
                self.fetched_urls.retain(|_, p_id| p_id != id);
            }
            ContextKind::Thread => {
                self.threads.retain(|_, p_id| p_id != id);
            }
        }
    }

    pub fn included_file(&self, path: &Path) -> Option<IncludedFile> {
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

    pub fn included_directory(&self, path: &Path) -> Option<ContextId> {
        self.directories.get(path).copied()
    }

    pub fn included_thread(&self, thread_id: &ThreadId) -> Option<ContextId> {
        self.threads.get(thread_id).copied()
    }

    pub fn included_url(&self, url: &str) -> Option<ContextId> {
        self.fetched_urls.get(url).copied()
    }

    pub fn duplicated_basenames(&self) -> HashSet<SharedString> {
        let mut seen = HashSet::default();
        let mut dupes = HashSet::default();

        for path in self.files.keys().chain(self.directories.keys()) {
            if let Some(basename) = path.file_name() {
                if seen.contains(&basename) {
                    dupes.insert(basename.to_string_lossy().into_owned().into());
                } else {
                    seen.insert(basename);
                }
            }
        }

        dupes
    }
}

pub enum IncludedFile {
    Direct(ContextId),
    InDirectory(PathBuf),
}

pub(crate) fn push_fenced_codeblock(path: &Path, content: String, buf: &mut String) {
    buf.reserve(content.len() + 64);

    write!(buf, "```").unwrap();

    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        write!(buf, "{} ", extension).unwrap();
    }

    write!(buf, "{}", path.display()).unwrap();

    buf.push('\n');
    buf.push_str(&content);

    if !buf.ends_with('\n') {
        buf.push('\n');
    }

    buf.push_str("```\n");
}
