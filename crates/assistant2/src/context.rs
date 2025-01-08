use std::sync::Arc;
use std::{path::Path, rc::Rc};

use crate::thread::Thread;
use collections::BTreeMap;
use gpui::{AppContext, Model, SharedString};
use language::Buffer;
use language_model::{LanguageModelRequestMessage, MessageContent};
use project::ProjectPath;
use serde::{Deserialize, Serialize};

use text::BufferId;
use util::post_inc;

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
    pub kind: ContextKind,
    /// Text to send to the model. This is not refreshed by `snapshot`.
    pub text: SharedString,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ContextKind {
    File,
    Directory,
    FetchedUrl,
    Thread,
}

// Some context referenced by the message editor.
#[derive(Debug)]
pub struct Context {
    pub id: ContextId,
    pub variant: ContextVariant,
}

#[derive(Debug)]
pub enum ContextVariant {
    File(FileContext),
    Directory(DirectoryContext),
    FetchedUrl(FetchedUrlContext),
    Thread(ThreadContext),
}

// todo! Model<Buffer> holds onto the buffer even if the file is deleted and closed. Should remove
// the context from the message editor in this case.

#[derive(Debug)]
pub struct FileContext {
    pub buffer: Model<Buffer>,
    pub version: clock::Global,
    pub text: SharedString,
}

// todo! Unlike with files, directory renames will not automatically update.

#[derive(Debug)]
pub struct DirectoryContext {
    pub path: Rc<Path>,
    // todo! The choice to make this a BTreeMap was a result of use in a version of
    // ContextStore::will_include_buffer before I realized that the path logic should be used there
    // too.
    pub buffers: BTreeMap<BufferId, (Model<Buffer>, clock::Global)>,
    pub snapshot: ContextSnapshot,
}

#[derive(Debug)]
pub struct FetchedUrlContext {
    pub url: SharedString,
    pub text: SharedString,
}

// todo! Model<Thread> holds onto the thread even if the thread is deleted. Can either handle this
// explicitly or have a WeakModel<Thread> and remove during snapshot.

#[derive(Debug)]
pub struct ThreadContext {
    pub thread: Model<Thread>,
    pub text: SharedString,
}

impl Context {
    pub fn snapshot(&self, cx: &AppContext) -> Option<ContextSnapshot> {
        match &self.variant {
            ContextVariant::File(file_context) => {
                let path = file_context.path(cx)?;
                let full_path: SharedString = path.to_string_lossy().into_owned().into();
                let name = match path.file_name() {
                    Some(name) => name.to_string_lossy().into_owned().into(),
                    None => full_path.clone(),
                };
                let parent = path
                    .parent()
                    .and_then(|p| p.file_name())
                    .map(|p| p.to_string_lossy().into_owned().into());

                Some(ContextSnapshot {
                    id: self.id,
                    name,
                    parent,
                    tooltip: Some(full_path),
                    kind: ContextKind::File,
                    text: file_context.text.clone(),
                })
            }

            ContextVariant::Directory(DirectoryContext { snapshot, .. }) => Some(snapshot.clone()),

            ContextVariant::FetchedUrl(FetchedUrlContext { url, text }) => Some(ContextSnapshot {
                id: self.id,
                name: url.clone(),
                parent: None,
                tooltip: None,
                kind: ContextKind::FetchedUrl,
                text: text.clone(),
            }),

            ContextVariant::Thread(thread_context) => {
                let thread = thread_context.thread.read(cx);

                Some(ContextSnapshot {
                    id: self.id,
                    name: thread.summary().unwrap_or("New thread".into()),
                    parent: None,
                    tooltip: None,
                    kind: ContextKind::Thread,
                    text: thread_context.text.clone(),
                })
            }
        }
    }
}

impl FileContext {
    pub fn path(&self, cx: &AppContext) -> Option<Arc<Path>> {
        let buffer = self.buffer.read(cx);
        if let Some(file) = buffer.file() {
            Some(file.path().clone())
        } else {
            log::error!("Buffer that had a path unexpectedly no longer has a path.");
            None
        }
    }
}

pub fn attach_context_to_message(
    message: &mut LanguageModelRequestMessage,
    contexts: impl Iterator<Item = ContextSnapshot>,
) {
    let mut file_context = String::new();
    let mut directory_context = String::new();
    let mut fetch_context = String::new();
    let mut thread_context = String::new();

    for context in contexts {
        match context.kind {
            ContextKind::File => {
                file_context.push_str(&context.text);
                file_context.push('\n');
            }
            ContextKind::Directory => {
                directory_context.push_str(&context.text);
                directory_context.push('\n');
            }
            ContextKind::FetchedUrl => {
                fetch_context.push_str(&context.name);
                fetch_context.push('\n');
                fetch_context.push_str(&context.text);
                fetch_context.push('\n');
            }
            ContextKind::Thread { .. } => {
                thread_context.push_str(&context.name);
                thread_context.push('\n');
                thread_context.push_str(&context.text);
                thread_context.push('\n');
            }
        }
    }

    let mut context_text = String::new();
    if !file_context.is_empty() {
        context_text.push_str("The following files are available:\n");
        context_text.push_str(&file_context);
    }

    if !directory_context.is_empty() {
        context_text.push_str("The following directories are available:\n");
        context_text.push_str(&directory_context);
    }

    if !fetch_context.is_empty() {
        context_text.push_str("The following fetched results are available\n");
        context_text.push_str(&fetch_context);
    }

    if !thread_context.is_empty() {
        context_text.push_str("The following previous conversation threads are available\n");
        context_text.push_str(&thread_context);
    }

    if !context_text.is_empty() {
        message.content.push(MessageContent::Text(context_text));
    }
}
