use std::{ops::Range, path::Path};

use anyhow::Context as _;
use futures::future;
use gpui::{App, AppContext as _, Entity, SharedString, Task};
use language::Buffer;
use language_model::LanguageModelRequestMessage;
use project::ProjectEntryId;
use prompt_store::UserPromptId;
use rope::{Point, Rope};
use text::Anchor;
use ui::{ElementId, IconName};
use util::ResultExt as _;

use crate::thread::Thread;

pub const RULES_ICON: IconName = IconName::Context;

pub enum ContextKind {
    File,
    Directory,
    Symbol,
    Excerpt,
    FetchedUrl,
    Thread,
    Rules,
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
            ContextKind::Rules => RULES_ICON,
        }
    }
}

/// Handle for context that can be added to the message thread. This type has the following properties:
///
/// * Cheap to clone.
///
/// * `Eq + Hash` for detecting when context has already been added.
///
/// * Use IDs that are stable enough for tracking renames and identifying when context has already
/// been added to the thread. For example, `ProjectEntryId` is used instead of `ProjectPath` for
/// `DirectoryContext` so that it follows renames.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AssistantContext {
    File(FileContext),
    /*
    Directory(DirectoryContext),
    Symbol(SymbolContext),
    FetchedUrl(FetchedUrlContext),
    Thread(ThreadContext),
    Excerpt(ExcerptContext),
    Rules(RulesContext),
    */
}

impl AssistantContext {
    pub fn element_id(&self, name: &'static str) -> ElementId {
        match self {
            Self::File(context) => ElementId::NamedInteger(
                name.into(),
                context.buffer.entity_id().as_u64().try_into().unwrap(),
            ),
        }
    }
}

/// todo! document decisions
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FileContext {
    pub buffer: Entity<Buffer>,
}

impl FileContext {
    fn loader(self, cx: &App) -> Option<impl FnOnce() -> String + Send + 'static> {
        let buffer_ref = self.buffer.read(cx);
        let Some(file) = buffer_ref.file() else {
            log::error!("file context missing path");
            return None;
        };
        let full_path = file.full_path(cx);
        let content = buffer_ref.as_rope().clone();
        Some(move || to_fenced_codeblock(&full_path, content, None))
    }
}

/*
#[derive(Debug, Clone)]
pub struct DirectoryContext {
    entry_id: ProjectEntryId,
}

impl DirectoryContext {
    pub fn entry<'a>(&self, cx: &'a App) -> Option<&'a project::Entry> {
        self.worktree.read(cx).entry_for_id(self.entry_id)
    }

    fn loader(&self, cx: &App) -> Option<impl FnOnce() -> String> {
        todo!()
    }

}

#[derive(Debug, Clone)]
pub struct SymbolContext {
    pub name: SharedString,
    pub excerpt: ExcerptContext,
}

#[derive(Debug, Clone)]
pub struct ExcerptContext {
    pub buffer: Entity<Buffer>,
    pub range: Range<Anchor>,
}

#[derive(Debug, Clone)]
pub struct FetchedUrlContext {
    pub url: SharedString,
}

#[derive(Debug, Clone)]
pub struct ThreadContext {
    pub thread_id: ThreadId,
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
pub struct RulesContext {
    pub prompt_id: UserPromptId,
}

pub fn attach_context_to_message<'a>(
    message: &mut LanguageModelRequestMessage,
    contexts: impl Iterator<Item = &'a AssistantContext>,
    cx: &App,
) {
    if let Some(context_string) = load_context_string(contexts, cx) {
        message.content.push(context_string.into());
    }
}
*/

/// Loads and formats a collection of contexts.
pub fn load_context_text<'a>(
    contexts: impl Iterator<Item = &'a AssistantContext>,
    cx: &App,
) -> Task<String> {
    let mut file_context = Vec::new();
    /*
    let mut directory_context = Vec::new();
    let mut symbol_context = Vec::new();
    let mut excerpt_context = Vec::new();
    let mut fetch_context = Vec::new();
    let mut thread_context = Vec::new();
    let mut rules_context = Vec::new();
    */

    let contexts = contexts.cloned().collect::<Vec<_>>();

    for context in contexts {
        match context {
            AssistantContext::File(context) => file_context.extend(context.loader(cx)),
            /*
            AssistantContext::Directory(context) => directory_context.push(context.loader(cx)),
            AssistantContext::Symbol(context) => symbol_context.push(context.loader(cx)),
            AssistantContext::Excerpt(context) => excerpt_context.push(context.loader(cx)),
            AssistantContext::FetchedUrl(context) => fetch_context.push(context.loader(cx)),
            AssistantContext::Thread(context) => thread_context.push(context.loader(cx)),
            AssistantContext::Rules(context) => rules_context.push(context.loader(cx)),
            */
        }
    }

    if file_context.is_empty()
    /*
    && directory_context.is_empty()
    && symbol_context.is_empty()
    && excerpt_context.is_empty()
    && fetch_context.is_empty()
    && thread_context.is_empty()
    && rules_context.is_empty()
    */
    {
        return Task::ready("".to_string());
    }

    cx.background_spawn(async move {
        let mut result = String::new();
        result.push_str("\n<context>\n\
            The following items were attached by the user. You don't need to use other tools to read them.\n\n");

        if !file_context.is_empty() {
            result.push_str("<files>\n");
            for loader in file_context {
                result.push_str(&loader());
            }
            result.push_str("</files>\n");
        }

        /*
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
        */

        result.push_str("</context>\n");
        result
    })
}

fn to_fenced_codeblock(path: &Path, content: Rope, line_range: Option<Range<Point>>) -> String {
    let line_range_text = line_range.map(|range| {
        if range.start.row == range.end.row {
            format!(":{}", range.start.row + 1)
        } else {
            format!(":{}-{}", range.start.row + 1, range.end.row + 1)
        }
    });

    let path_extension = path.extension().and_then(|ext| ext.to_str());
    let path_string = path.to_string_lossy();
    let capacity = 3
        + path_extension.map_or(0, |extension| extension.len() + 1)
        + path_string.len()
        + line_range_text.as_ref().map_or(0, |text| text.len())
        + 1
        + content.len()
        + 5;
    let mut buffer = String::with_capacity(capacity);

    buffer.push_str("```");

    if let Some(extension) = path_extension {
        buffer.push_str(extension);
        buffer.push(' ');
    }
    buffer.push_str(&path_string);

    if let Some(line_range_text) = line_range_text {
        buffer.push_str(&line_range_text);
    }

    buffer.push('\n');
    for chunk in content.chunks() {
        buffer.push_str(&chunk);
    }

    if !buffer.ends_with('\n') {
        buffer.push('\n');
    }

    buffer.push_str("```\n");

    debug_assert!(
        buffer.len() == capacity - 1 || buffer.len() == capacity,
        "to_fenced_codeblock calculated capacity of {}, but length was {}",
        capacity,
        buffer.len(),
    );

    buffer
}
