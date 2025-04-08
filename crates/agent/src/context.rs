use std::{ops::Range, sync::Arc};

use gpui::{App, Entity, SharedString};
use language::{Buffer, File};
use language_model::LanguageModelRequestMessage;
use project::ProjectPath;
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

impl ThreadContext {
    pub fn summary(&self, cx: &App) -> SharedString {
        self.thread
            .read(cx)
            .summary()
            .unwrap_or("New thread".into())
    }
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

/// Formats a collection of contexts into a string representation
pub fn format_context_as_string<'a>(
    contexts: impl Iterator<Item = &'a AssistantContext>,
    cx: &App,
) -> Option<String> {
    let mut file_context = Vec::new();
    let mut directory_context = Vec::new();
    let mut symbol_context = Vec::new();
    let mut fetch_context = Vec::new();
    let mut thread_context = Vec::new();

    for context in contexts {
        match context {
            AssistantContext::File(context) => file_context.push(context),
            AssistantContext::Directory(context) => directory_context.push(context),
            AssistantContext::Symbol(context) => symbol_context.push(context),
            AssistantContext::FetchedUrl(context) => fetch_context.push(context),
            AssistantContext::Thread(context) => thread_context.push(context),
        }
    }

    if file_context.is_empty()
        && directory_context.is_empty()
        && symbol_context.is_empty()
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
