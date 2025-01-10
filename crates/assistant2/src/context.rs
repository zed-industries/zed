use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use file_icons::FileIcons;
use gpui::{AppContext, Model, SharedString};
use language::Buffer;
use language_model::{LanguageModelRequestMessage, MessageContent};
use serde::{Deserialize, Serialize};
use text::BufferId;
use ui::IconName;
use util::post_inc;

use crate::thread::Thread;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct ContextId(pub(crate) usize);

impl ContextId {
    pub fn post_inc(&mut self) -> Self {
        Self(post_inc(&mut self.0))
    }
}

/// Some context attached to a message in a thread.
#[derive(Debug, Clone)]
pub struct ContextSnapshot {
    pub id: ContextId,
    pub name: SharedString,
    pub parent: Option<SharedString>,
    pub tooltip: Option<SharedString>,
    pub icon_path: Option<SharedString>,
    pub kind: ContextKind,
    /// Concatenating these strings yields text to send to the model. Not refreshed by `snapshot`.
    pub text: Box<[SharedString]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextKind {
    File,
    Directory,
    FetchedUrl,
    Thread,
}

impl ContextKind {
    pub fn icon(&self) -> IconName {
        match self {
            ContextKind::File => IconName::File,
            ContextKind::Directory => IconName::Folder,
            ContextKind::FetchedUrl => IconName::Globe,
            ContextKind::Thread => IconName::MessageCircle,
        }
    }
}

#[derive(Debug)]
pub enum Context {
    File(FileContext),
    Directory(DirectoryContext),
    FetchedUrl(FetchedUrlContext),
    Thread(ThreadContext),
}

impl Context {
    pub fn id(&self) -> ContextId {
        match self {
            Self::File(file) => file.id,
            Self::Directory(directory) => directory.snapshot.id,
            Self::FetchedUrl(url) => url.id,
            Self::Thread(thread) => thread.id,
        }
    }
}

#[derive(Debug)]
pub struct FileContext {
    pub id: ContextId,
    pub buffer: ContextBuffer,
}

#[derive(Debug)]
pub struct DirectoryContext {
    #[allow(unused)]
    pub path: Rc<Path>,
    #[allow(unused)]
    pub buffers: Vec<ContextBuffer>,
    pub snapshot: ContextSnapshot,
}

#[derive(Debug)]
pub struct FetchedUrlContext {
    pub id: ContextId,
    pub url: SharedString,
    pub text: SharedString,
}

// TODO: Model<Thread> holds onto the thread even if the thread is deleted. Can either handle this
// explicitly or have a WeakModel<Thread> and remove during snapshot.

#[derive(Debug)]
pub struct ThreadContext {
    pub id: ContextId,
    pub thread: Model<Thread>,
    pub text: SharedString,
}

// TODO: Model<Buffer> holds onto the buffer even if the file is deleted and closed. Should remove
// the context from the message editor in this case.

#[derive(Debug)]
pub struct ContextBuffer {
    #[allow(unused)]
    pub id: BufferId,
    pub buffer: Model<Buffer>,
    #[allow(unused)]
    pub version: clock::Global,
    pub text: SharedString,
}

impl Context {
    pub fn snapshot(&self, cx: &AppContext) -> Option<ContextSnapshot> {
        match &self {
            Self::File(file_context) => file_context.snapshot(cx),
            Self::Directory(directory_context) => Some(directory_context.snapshot()),
            Self::FetchedUrl(fetched_url_context) => Some(fetched_url_context.snapshot()),
            Self::Thread(thread_context) => Some(thread_context.snapshot(cx)),
        }
    }
}

impl FileContext {
    pub fn path(&self, cx: &AppContext) -> Option<Arc<Path>> {
        let buffer = self.buffer.buffer.read(cx);
        if let Some(file) = buffer.file() {
            Some(file.path().clone())
        } else {
            log::error!("Buffer that had a path unexpectedly no longer has a path.");
            None
        }
    }

    pub fn snapshot(&self, cx: &AppContext) -> Option<ContextSnapshot> {
        let path = self.path(cx)?;
        let full_path: SharedString = path.to_string_lossy().into_owned().into();
        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().into(),
            None => full_path.clone(),
        };
        let parent = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned().into());

        let icon_path = FileIcons::get_icon(&path, cx);

        Some(ContextSnapshot {
            id: self.id,
            name,
            parent,
            tooltip: Some(full_path),
            icon_path,
            kind: ContextKind::File,
            text: Box::new([self.buffer.text.clone()]),
        })
    }
}

impl DirectoryContext {
    pub fn snapshot(&self) -> ContextSnapshot {
        self.snapshot.clone()
    }
}

impl FetchedUrlContext {
    pub fn snapshot(&self) -> ContextSnapshot {
        ContextSnapshot {
            id: self.id,
            name: self.url.clone(),
            parent: None,
            tooltip: None,
            icon_path: None,
            kind: ContextKind::FetchedUrl,
            text: Box::new([self.text.clone()]),
        }
    }
}

impl ThreadContext {
    pub fn snapshot(&self, cx: &AppContext) -> ContextSnapshot {
        let thread = self.thread.read(cx);
        ContextSnapshot {
            id: self.id,
            name: thread.summary().unwrap_or("New thread".into()),
            parent: None,
            tooltip: None,
            icon_path: None,
            kind: ContextKind::Thread,
            text: Box::new([self.text.clone()]),
        }
    }
}

pub fn attach_context_to_message(
    message: &mut LanguageModelRequestMessage,
    contexts: impl Iterator<Item = ContextSnapshot>,
) {
    let mut file_context = Vec::new();
    let mut directory_context = Vec::new();
    let mut fetch_context = Vec::new();
    let mut thread_context = Vec::new();

    for context in contexts {
        match context.kind {
            ContextKind::File => file_context.push(context),
            ContextKind::Directory => directory_context.push(context),
            ContextKind::FetchedUrl => fetch_context.push(context),
            ContextKind::Thread => thread_context.push(context),
        }
    }

    let mut context_text = String::new();

    if !file_context.is_empty() {
        context_text.push_str("The following files are available:\n");
        for context in file_context {
            for chunk in context.text {
                context_text.push_str(&chunk);
            }
            context_text.push('\n');
        }
    }

    if !directory_context.is_empty() {
        context_text.push_str("The following directories are available:\n");
        for context in directory_context {
            for chunk in context.text {
                context_text.push_str(&chunk);
            }
            context_text.push('\n');
        }
    }

    if !fetch_context.is_empty() {
        context_text.push_str("The following fetched results are available\n");
        for context in fetch_context {
            context_text.push_str(&context.name);
            context_text.push('\n');
            for chunk in context.text {
                context_text.push_str(&chunk);
            }
            context_text.push('\n');
        }
    }

    if !thread_context.is_empty() {
        context_text.push_str("The following previous conversation threads are available\n");
        for context in thread_context {
            context_text.push_str(&context.name);
            context_text.push('\n');
            for chunk in context.text {
                context_text.push_str(&chunk);
            }
            context_text.push('\n');
        }
    }

    if !context_text.is_empty() {
        message.content.push(MessageContent::Text(context_text));
    }
}
