use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};

use futures::{FutureExt, future::Shared};
use gpui::{App, Entity, SharedString, Task};
use language::Buffer;
use language_model::{LanguageModelImage, LanguageModelRequestMessage};
use project::{ProjectEntryId, ProjectPath, Worktree};
use prompt_store::UserPromptId;
use rope::Point;
use serde::{Deserialize, Serialize};
use text::{Anchor, BufferId};
use ui::IconName;
use util::post_inc;

use crate::thread::Thread;

pub const RULES_ICON: IconName = IconName::Context;

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
    Rules,
    Image,
}

impl ContextKind {
    pub fn icon(&self) -> IconName {
        match self {
            ContextKind::File => IconName::File,
            ContextKind::Directory => IconName::Folder,
            ContextKind::Symbol => IconName::Code,
            ContextKind::Excerpt => IconName::Context,
            ContextKind::FetchedUrl => IconName::Globe,
            ContextKind::Thread => IconName::MessageBubbles,
            ContextKind::Rules => RULES_ICON,
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
    Rules(RulesContext),
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
            Self::Rules(rules) => rules.id,
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
    pub entry_id: ProjectEntryId,
    pub last_path: Arc<Path>,
    /// Buffers of the files within the directory.
    pub context_buffers: Vec<ContextBuffer>,
}

impl DirectoryContext {
    pub fn entry<'a>(&self, cx: &'a App) -> Option<&'a project::Entry> {
        self.worktree.read(cx).entry_for_id(self.entry_id)
    }

    pub fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        let worktree = self.worktree.read(cx);
        worktree
            .entry_for_id(self.entry_id)
            .map(|entry| ProjectPath {
                worktree_id: worktree.id(),
                path: entry.path.clone(),
            })
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
    pub original_image: Arc<gpui::Image>,
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
    // TODO: Entity<Buffer> holds onto the buffer even if the buffer is deleted. Should probably be
    // a WeakEntity and handle removal from the UI when it has dropped.
    pub buffer: Entity<Buffer>,
    pub last_full_path: Arc<Path>,
    pub version: clock::Global,
    pub text: SharedString,
}

impl ContextBuffer {
    pub fn full_path(&self, cx: &App) -> PathBuf {
        let file = self.buffer.read(cx).file();
        // Note that in practice file can't be `None` because it is present when this is created and
        // there's no way for buffers to go from having a file to not.
        file.map_or(self.last_full_path.to_path_buf(), |file| file.full_path(cx))
    }
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

#[derive(Debug, Clone)]
pub struct RulesContext {
    pub id: ContextId,
    pub prompt_id: UserPromptId,
    pub title: SharedString,
    pub text: SharedString,
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
    let mut rules_context = Vec::new();

    for context in contexts {
        match context {
            AssistantContext::File(context) => file_context.push(context),
            AssistantContext::Directory(context) => directory_context.push(context),
            AssistantContext::Symbol(context) => symbol_context.push(context),
            AssistantContext::Excerpt(context) => excerpt_context.push(context),
            AssistantContext::FetchedUrl(context) => fetch_context.push(context),
            AssistantContext::Thread(context) => thread_context.push(context),
            AssistantContext::Rules(context) => rules_context.push(context),
            AssistantContext::Image(_) => {}
        }
    }

    if file_context.is_empty()
        && directory_context.is_empty()
        && symbol_context.is_empty()
        && excerpt_context.is_empty()
        && fetch_context.is_empty()
        && thread_context.is_empty()
        && rules_context.is_empty()
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

    if !rules_context.is_empty() {
        result.push_str(
            "<user_rules>\n\
            The user has specified the following rules that should be applied:\n\n",
        );
        for context in &rules_context {
            result.push_str(&context.text);
            result.push('\n');
        }
        result.push_str("</user_rules>\n");
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
