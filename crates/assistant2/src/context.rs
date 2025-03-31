use std::{ops::Range, sync::Arc};

use file_icons::FileIcons;
use gpui::{App, Entity, SharedString};
use language::{Buffer, File};
use language_model::LanguageModelRequestMessage;
use project::ProjectPath;
use serde::{Deserialize, Serialize};
use text::{Anchor, BufferId};
use ui::{Icon, IconName};
use util::post_inc;

use crate::{context_store::buffer_path_log_err, thread::Thread};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct ContextId(pub(crate) usize);

impl ContextId {
    pub fn post_inc(&mut self) -> Self {
        Self(post_inc(&mut self.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextKind {
    File,
    Directory,
    Symbol,
    FetchedUrl,
    Thread,
}

impl ContextKind {
    pub fn icon(&self) -> IconName {
        match self {
            ContextKind::File => IconName::File,
            ContextKind::Directory => IconName::Folder,
            ContextKind::Symbol => IconName::Code,
            ContextKind::FetchedUrl => IconName::Globe,
            ContextKind::Thread => IconName::MessageBubbles,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssistantContext {
    File(FileContext),
    Directory(DirectoryContext),
    Symbol(SymbolContext),
    FetchedUrl(FetchedUrlContext),
    Thread(ThreadContext),
}

impl AssistantContext {
    pub fn id(&self) -> ContextId {
        match self {
            Self::File(file) => file.id,
            Self::Directory(directory) => directory.id,
            Self::Symbol(symbol) => symbol.id,
            Self::FetchedUrl(url) => url.id,
            Self::Thread(thread) => thread.id,
        }
    }

    pub fn name(&self, cx: &App) -> SharedString {
        match self {
            Self::File(file) => file.name(),
            Self::Directory(directory) => directory.name(),
            Self::Symbol(symbol) => symbol.name(),
            Self::FetchedUrl(url) => url.name(),
            Self::Thread(thread) => thread.name(cx),
        }
    }

    pub fn tooltip(&self) -> Option<SharedString> {
        match self {
            Self::File(file) => file.tooltip(),
            Self::Directory(directory) => directory.tooltip(),
            Self::Symbol(_) => None,
            Self::FetchedUrl(_) => None,
            Self::Thread(_) => None,
        }
    }

    pub fn parent(&self) -> Option<SharedString> {
        match self {
            Self::File(file) => file.parent(),
            Self::Directory(directory) => directory.parent(),
            Self::Symbol(_) => None,
            Self::FetchedUrl(_) => None,
            Self::Thread(_) => None,
        }
    }

    pub fn icon(&self, cx: &App) -> Icon {
        match self {
            Self::File(file) => file.icon(cx),
            Self::Directory(_) => IconName::Folder.into(),
            Self::Symbol(_) => IconName::Code.into(),
            Self::FetchedUrl(_) => IconName::Globe.into(),
            Self::Thread(_) => IconName::MessageBubbles.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileContext {
    pub id: ContextId,
    pub context_buffer: ContextBuffer,
}

#[derive(Debug, Clone)]
pub struct DirectoryContext {
    pub id: ContextId,
    pub project_path: ProjectPath,
    pub context_buffers: Vec<ContextBuffer>,
}

#[derive(Debug, Clone)]
pub struct SymbolContext {
    pub id: ContextId,
    pub context_symbol: ContextSymbol,
}

#[derive(Debug, Clone)]
pub struct FetchedUrlContext {
    pub id: ContextId,
    pub url: SharedString,
    pub text: SharedString,
}

// TODO: Model<Thread> holds onto the thread even if the thread is deleted. Can either handle this
// explicitly or have a WeakModel<Thread> and remove during snapshot.

#[derive(Debug, Clone)]
pub struct ThreadContext {
    pub id: ContextId,
    pub thread: Entity<Thread>,
    pub text: SharedString,
}

// TODO: Model<Buffer> holds onto the buffer even if the file is deleted and closed. Should remove
// the context from the message editor in this case.

#[derive(Clone)]
pub struct ContextBuffer {
    pub id: BufferId,
    pub buffer: Entity<Buffer>,
    pub file: Arc<dyn File>,
    pub version: clock::Global,
    pub text: SharedString,
}

impl std::fmt::Debug for ContextBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextBuffer")
            .field("id", &self.id)
            .field("buffer", &self.buffer)
            .field("version", &self.version)
            .field("text", &self.text)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ContextSymbol {
    pub id: ContextSymbolId,
    pub buffer: Entity<Buffer>,
    pub buffer_version: clock::Global,
    /// The range that the symbol encloses, e.g. for function symbol, this will
    /// include not only the signature, but also the body
    pub enclosing_range: Range<Anchor>,
    pub text: SharedString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSymbolId {
    pub path: ProjectPath,
    pub name: SharedString,
    pub range: Range<Anchor>,
}

impl FileContext {
    pub fn name(&self) -> SharedString {
        let path = self.context_buffer.file.path();
        match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().into(),
            None => path.to_string_lossy().into_owned().into(),
        }
    }

    pub fn icon(&self, cx: &App) -> Icon {
        FileIcons::get_icon(&self.context_buffer.file.path(), cx)
            .map(|icon_path| Icon::from_path(icon_path))
            .unwrap_or_else(|| IconName::File.into())
    }

    pub fn tooltip(&self) -> Option<SharedString> {
        let path = self.context_buffer.file.path();
        Some(path.to_string_lossy().into_owned().into())
    }

    pub fn parent(&self) -> Option<SharedString> {
        let path = self.context_buffer.file.path();
        path.parent()
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned().into())
    }
}

impl DirectoryContext {
    pub fn new(
        id: ContextId,
        project_path: ProjectPath,
        context_buffers: Vec<ContextBuffer>,
    ) -> DirectoryContext {
        DirectoryContext {
            id,
            project_path,
            context_buffers,
        }
    }

    pub fn name(&self) -> SharedString {
        match self.project_path.path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().into(),
            None => self.project_path.path.to_string_lossy().into_owned().into(),
        }
    }

    pub fn tooltip(&self) -> Option<SharedString> {
        Some(self.project_path.path.to_string_lossy().into_owned().into())
    }

    pub fn parent(&self) -> Option<SharedString> {
        self.project_path
            .path
            .parent()
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned().into())
    }
}

impl SymbolContext {
    pub fn name(&self) -> SharedString {
        self.context_symbol.id.name.clone()
    }
}

impl FetchedUrlContext {
    pub fn name(&self) -> SharedString {
        self.url.clone()
    }
}

impl ThreadContext {
    pub fn name(&self, cx: &App) -> SharedString {
        self.thread
            .read(cx)
            .summary()
            .unwrap_or("New thread".into())
    }
}

pub fn attach_context_to_message(
    message: &mut LanguageModelRequestMessage,
    contexts: impl Iterator<Item = AssistantContext>,
) {
    todo!()
    //     let mut file_context = Vec::new();
    //     let mut directory_context = Vec::new();
    //     let mut symbol_context = Vec::new();
    //     let mut fetch_context = Vec::new();
    //     let mut thread_context = Vec::new();

    //     let mut capacity = 0;
    //     for context in contexts {
    //         capacity += context.text.len();
    //         match context.kind {
    //             ContextKind::File => file_context.push(context),
    //             ContextKind::Directory => directory_context.push(context),
    //             ContextKind::Symbol => symbol_context.push(context),
    //             ContextKind::FetchedUrl => fetch_context.push(context),
    //             ContextKind::Thread => thread_context.push(context),
    //         }
    //     }
    //     if !file_context.is_empty() {
    //         capacity += 1;
    //     }
    //     if !directory_context.is_empty() {
    //         capacity += 1;
    //     }
    //     if !symbol_context.is_empty() {
    //         capacity += 1;
    //     }
    //     if !fetch_context.is_empty() {
    //         capacity += 1 + fetch_context.len();
    //     }
    //     if !thread_context.is_empty() {
    //         capacity += 1 + thread_context.len();
    //     }
    //     if capacity == 0 {
    //         return;
    //     }

    //     let mut context_chunks = Vec::with_capacity(capacity);

    //     if !file_context.is_empty() {
    //         context_chunks.push("The following files are available:\n");
    //         for context in &file_context {
    //             for chunk in &context.text {
    //                 context_chunks.push(&chunk);
    //             }
    //         }
    //     }

    //     if !directory_context.is_empty() {
    //         context_chunks.push("The following directories are available:\n");
    //         for context in &directory_context {
    //             for chunk in &context.text {
    //                 context_chunks.push(&chunk);
    //             }
    //         }
    //     }

    //     if !symbol_context.is_empty() {
    //         context_chunks.push("The following symbols are available:\n");
    //         for context in &symbol_context {
    //             for chunk in &context.text {
    //                 context_chunks.push(&chunk);
    //             }
    //         }
    //     }

    //     if !fetch_context.is_empty() {
    //         context_chunks.push("The following fetched results are available:\n");
    //         for context in &fetch_context {
    //             context_chunks.push(&context.name);
    //             for chunk in &context.text {
    //                 context_chunks.push(&chunk);
    //             }
    //         }
    //     }

    //     if !thread_context.is_empty() {
    //         context_chunks.push("The following previous conversation threads are available:\n");
    //         for context in &thread_context {
    //             context_chunks.push(&context.name);
    //             for chunk in &context.text {
    //                 context_chunks.push(&chunk);
    //             }
    //         }
    //     }

    //     debug_assert!(
    //         context_chunks.len() == capacity,
    //         "attach_context_message calculated capacity of {}, but length was {}",
    //         capacity,
    //         context_chunks.len()
    //     );

    //     if !context_chunks.is_empty() {
    //         message
    //             .content
    //             .push(MessageContent::Text(context_chunks.join("\n")));
    //     }
}
