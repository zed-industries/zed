use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use collections::{BTreeMap, HashMap};
use gpui::{AppContext, Model, SharedString};
use language::Buffer;
use text::BufferId;

use crate::context::{
    ContextKind, ContextSnapshot, DirectoryContext, FetchedUrlContext, ThreadContext,
};
use crate::thread::Thread;
use crate::{
    context::{Context, ContextId, ContextVariant, FileContext},
    thread::ThreadId,
};

pub struct ContextStore {
    context: Vec<Context>,
    // TODO: If an EntityId is used for all context types (like BufferId), can remove ContextId.
    next_context_id: ContextId,
    files: BTreeMap<BufferId, ContextId>,
    directories: HashMap<PathBuf, ContextId>,
    threads: HashMap<ThreadId, ContextId>,
    fetched_urls: HashMap<String, ContextId>,
}

impl ContextStore {
    pub fn new() -> Self {
        Self {
            context: Vec::new(),
            next_context_id: ContextId(0),
            files: BTreeMap::default(),
            directories: HashMap::default(),
            threads: HashMap::default(),
            fetched_urls: HashMap::default(),
        }
    }

    pub fn snapshot<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = ContextSnapshot> + 'a {
        self.context()
            .iter()
            .flat_map(|context| context.snapshot(cx))
    }

    pub fn context(&self) -> &Vec<Context> {
        &self.context
    }

    pub fn clear(&mut self) {
        self.context.clear();
        self.files.clear();
        self.directories.clear();
        self.threads.clear();
        self.fetched_urls.clear();
    }

    pub fn insert_file(&mut self, buffer_model: Model<Buffer>, cx: &AppContext) {
        let buffer = buffer_model.read(cx);
        let Some(file) = buffer.file() else {
            return;
        };

        let mut text = String::new();
        push_fenced_codeblock(file.path(), buffer.text(), &mut text);

        let id = self.next_context_id.post_inc();
        self.files.insert(buffer.remote_id(), id);
        self.context.push(Context {
            id,
            variant: ContextVariant::File(FileContext {
                buffer: buffer_model,
                version: buffer.version.clone(),
                text: text.into(),
            }),
        });
    }

    pub fn insert_directory(
        &mut self,
        path: &Path,
        buffers: BTreeMap<BufferId, (Model<Buffer>, clock::Global)>,
        text: impl Into<SharedString>,
    ) {
        let id = self.next_context_id.post_inc();
        self.directories.insert(path.to_path_buf(), id);

        let full_path: SharedString = path.to_string_lossy().into_owned().into();

        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().into(),
            None => full_path.clone(),
        };

        let parent = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned().into());

        self.context.push(Context {
            id,
            variant: ContextVariant::Directory(DirectoryContext {
                path: path.into(),
                buffers,
                snapshot: ContextSnapshot {
                    id,
                    name,
                    parent,
                    tooltip: Some(full_path),
                    kind: ContextKind::Directory,
                    text: text.into(),
                },
            }),
        });
    }

    pub fn insert_thread(&mut self, thread: Model<Thread>, cx: &AppContext) {
        let context_id = self.next_context_id.post_inc();
        let thread_ref = thread.read(cx);
        let text = thread_ref.text().into();

        self.threads.insert(thread_ref.id().clone(), context_id);
        self.context.push(Context {
            id: context_id,
            variant: ContextVariant::Thread(ThreadContext { thread, text }),
        });
    }

    pub fn insert_fetched_url(&mut self, url: String, text: impl Into<SharedString>) {
        let context_id = self.next_context_id.post_inc();

        self.fetched_urls.insert(url.clone(), context_id);
        self.context.push(Context {
            id: context_id,
            variant: ContextVariant::FetchedUrl(FetchedUrlContext {
                url: url.into(),
                text: text.into(),
            }),
        });
    }

    pub fn remove_context(&mut self, id: ContextId) {
        let Some(ix) = self.context.iter().position(|context| context.id == id) else {
            return;
        };

        match self.context.remove(ix).variant {
            ContextVariant::File(_) => {
                self.files.retain(|_, context_id| *context_id != id);
            }
            ContextVariant::Directory(_) => {
                self.directories.retain(|_, context_id| *context_id != id);
            }
            ContextVariant::FetchedUrl(_) => {
                self.fetched_urls.retain(|_, context_id| *context_id != id);
            }
            ContextVariant::Thread(_) => {
                self.threads.retain(|_, context_id| *context_id != id);
            }
        }
    }

    // todo! The implementation and naming here is assuming that directories will be rescanned.

    pub fn will_include_buffer(&self, buffer_id: BufferId, path: &Path) -> Option<FileInclusion> {
        if let Some(context_id) = self.files.get(&buffer_id) {
            return Some(FileInclusion::Direct(*context_id));
        }

        self.will_include_file_path_via_directory(path)
    }

    pub fn will_include_file_path(&self, path: &Path, cx: &AppContext) -> Option<FileInclusion> {
        if !self.files.is_empty() {
            // todo! This is not very efficient, and is used when rendering file matches.
            let found_file_context = self.context.iter().find(|context| match &context.variant {
                ContextVariant::File(file_context) => {
                    if let Some(file_path) = file_context.path(cx) {
                        *file_path == *path
                    } else {
                        false
                    }
                }
                _ => false,
            });
            if let Some(context) = found_file_context {
                return Some(FileInclusion::Direct(context.id));
            }
        }

        self.will_include_file_path_via_directory(path)
    }

    fn will_include_file_path_via_directory(&self, path: &Path) -> Option<FileInclusion> {
        if self.directories.is_empty() {
            return None;
        }

        let mut buf = path.to_path_buf();

        while buf.pop() {
            if let Some(_) = self.directories.get(&buf) {
                return Some(FileInclusion::InDirectory(buf));
            }
        }

        None
    }

    pub fn includes_directory(&self, path: &Path) -> Option<ContextId> {
        self.directories.get(path).copied()
    }

    pub fn includes_thread(&self, thread_id: &ThreadId) -> Option<ContextId> {
        self.threads.get(thread_id).copied()
    }

    pub fn includes_url(&self, url: &str) -> Option<ContextId> {
        self.fetched_urls.get(url).copied()
    }
}

pub enum FileInclusion {
    Direct(ContextId),
    InDirectory(PathBuf),
}

pub(crate) fn push_fenced_codeblock(path: &Path, content: String, buffer: &mut String) {
    buffer.reserve(content.len() + 64);

    write!(buffer, "```").unwrap();

    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        write!(buffer, "{} ", extension).unwrap();
    }

    write!(buffer, "{}", path.display()).unwrap();

    buffer.push('\n');
    buffer.push_str(&content);

    if !buffer.ends_with('\n') {
        buffer.push('\n');
    }

    buffer.push_str("```\n");
}
