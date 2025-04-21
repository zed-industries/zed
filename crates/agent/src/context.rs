use std::{ops::Range, path::Path, sync::Arc};

use futures::{FutureExt, future::Shared};
use gpui::{App, Entity, SharedString, Task};
use language::{Buffer, File};
use language_model::{LanguageModelImage, LanguageModelRequestMessage};
use project::{ProjectPath, Worktree};
use rope::Point;
use serde::{Deserialize, Serialize};
use text::{Anchor, BufferId};
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
pub enum ContextKind {
    File,
    Directory,
    Symbol,
    Excerpt,
    FetchedUrl,
    Thread,
    Image,
}

impl ContextKind {
    pub fn icon(&self) -> IconName {
        match self {
            ContextKind::File => IconName::File,
            ContextKind::Directory => IconName::Folder,
            ContextKind::Symbol => IconName::Code,
            ContextKind::Excerpt => IconName::Code,
            ContextKind::FetchedUrl => IconName::Globe,
            ContextKind::Thread => IconName::MessageBubbles,
            ContextKind::Image => IconName::Image,
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
    Excerpt(ExcerptContext),
    Image(ImageContext),
}

impl AssistantContext {
    pub fn id(&self) -> ContextId {
        match self {
            Self::File(file) => file.id,
            Self::Directory(directory) => directory.id,
            Self::Symbol(symbol) => symbol.id,
            Self::FetchedUrl(url) => url.id,
            Self::Thread(thread) => thread.id,
            Self::Excerpt(excerpt) => excerpt.id,
            Self::Image(image) => image.id,
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
    pub worktree: Entity<Worktree>,
    pub path: Arc<Path>,
    /// Buffers of the files within the directory.
    pub context_buffers: Vec<ContextBuffer>,
}

impl DirectoryContext {
    pub fn project_path(&self, cx: &App) -> ProjectPath {
        ProjectPath {
            worktree_id: self.worktree.read(cx).id(),
            path: self.path.clone(),
        }
    }
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

#[derive(Debug, Clone)]
pub struct ThreadContext {
    pub id: ContextId,
    // TODO: Entity<Thread> holds onto the thread even if the thread is deleted. Should probably be
    // a WeakEntity and handle removal from the UI when it has dropped.
    pub thread: Entity<Thread>,
    pub text: SharedString,
}

impl ThreadContext {
    pub fn summary(&self, cx: &App) -> SharedString {
        self.thread
            .read(cx)
            .summary()
            .unwrap_or("New thread".into())
    }
}

#[derive(Debug, Clone)]
pub struct ImageContext {
    pub id: ContextId,
    pub image_task: Shared<Task<Option<LanguageModelImage>>>,
}

impl ImageContext {
    pub fn image(&self) -> Option<LanguageModelImage> {
        self.image_task.clone().now_or_never().flatten()
    }

    pub fn is_loading(&self) -> bool {
        self.image_task.clone().now_or_never().is_none()
    }

    pub fn is_error(&self) -> bool {
        self.image_task
            .clone()
            .now_or_never()
            .map(|result| result.is_none())
            .unwrap_or(false)
    }
}

#[derive(Clone)]
pub struct ContextBuffer {
    pub id: BufferId,
    // TODO: Entity<Buffer> holds onto the thread even if the thread is deleted. Should probably be
    // a WeakEntity and handle removal from the UI when it has dropped.
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

#[derive(Debug, Clone)]
pub struct ExcerptContext {
    pub id: ContextId,
    pub range: Range<Anchor>,
    pub line_range: Range<Point>,
    pub context_buffer: ContextBuffer,
}

/// Formats a collection of contexts into a string representation
pub fn format_context_as_string<'a>(
    contexts: impl Iterator<Item = &'a AssistantContext>,
    cx: &App,
) -> Option<String> {
    let mut file_context = Vec::new();
    let mut directory_context = Vec::new();
    let mut symbol_context = Vec::new();
    let mut excerpt_context = Vec::new();
    let mut fetch_context = Vec::new();
    let mut thread_context = Vec::new();

    for context in contexts {
        match context {
            AssistantContext::File(context) => file_context.push(context),
            AssistantContext::Directory(context) => directory_context.push(context),
            AssistantContext::Symbol(context) => symbol_context.push(context),
            AssistantContext::Excerpt(context) => excerpt_context.push(context),
            AssistantContext::FetchedUrl(context) => fetch_context.push(context),
            AssistantContext::Thread(context) => thread_context.push(context),
            AssistantContext::Image(_) => {}
        }
    }

    if file_context.is_empty()
        && directory_context.is_empty()
        && symbol_context.is_empty()
        && excerpt_context.is_empty()
        && fetch_context.is_empty()
        && thread_context.is_empty()
    {
        return None;
    }

    let mut result = String::new();
    result.push_str("\n<context>\n\
        The following items were attached by the user. You don't need to use other tools to read them.\n\n");

    if !file_context.is_empty() {
        result.push_str("<files>\n");
        for context in file_context {
            result.push_str(&context.context_buffer.text);
        }
        result.push_str("</files>\n");
    }

    if !directory_context.is_empty() {
        result.push_str("<directories>\n");
        for context in directory_context {
            for context_buffer in &context.context_buffers {
                result.push_str(&context_buffer.text);
            }
        }
        result.push_str("</directories>\n");
    }

    if !symbol_context.is_empty() {
        result.push_str("<symbols>\n");
        for context in symbol_context {
            result.push_str(&context.context_symbol.text);
            result.push('\n');
        }
        result.push_str("</symbols>\n");
    }

    if !excerpt_context.is_empty() {
        result.push_str("<excerpts>\n");
        for context in excerpt_context {
            result.push_str(&context.context_buffer.text);
            result.push('\n');
        }
        result.push_str("</excerpts>\n");
    }

    if !fetch_context.is_empty() {
        result.push_str("<fetched_urls>\n");
        for context in &fetch_context {
            result.push_str(&context.url);
            result.push('\n');
            result.push_str(&context.text);
            result.push('\n');
        }
        result.push_str("</fetched_urls>\n");
    }

    if !thread_context.is_empty() {
        result.push_str("<conversation_threads>\n");
        for context in &thread_context {
            result.push_str(&context.summary(cx));
            result.push('\n');
            result.push_str(&context.text);
            result.push('\n');
        }
        result.push_str("</conversation_threads>\n");
    }

    result.push_str("</context>\n");
    Some(result)
}

pub fn attach_context_to_message<'a>(
    message: &mut LanguageModelRequestMessage,
    contexts: impl Iterator<Item = &'a AssistantContext>,
    cx: &App,
) {
    if let Some(context_string) = format_context_as_string(contexts, cx) {
        message.content.push(context_string.into());
    }
}
