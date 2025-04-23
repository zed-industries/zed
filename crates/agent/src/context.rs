use std::{ops::Range, path::Path, sync::Arc};

use anyhow::Context as _;
use anyhow::Result;
use collections::HashSet;
use fs::Fs;
use futures::future;
use futures::{FutureExt, future::Shared};
use gpui::{App, AppContext as _, Entity, SharedString, Task};
use itertools::Itertools;
use language::Buffer;
use language_model::{LanguageModelImage, LanguageModelRequestMessage};
use project::{Project, ProjectEntryId, ProjectPath, Worktree};
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
    Selection,
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
            ContextKind::Selection => IconName::Context,
            ContextKind::FetchedUrl => IconName::Globe,
            ContextKind::Thread => IconName::MessageBubbles,
            ContextKind::Rules => RULES_ICON,
            ContextKind::Image => IconName::Image,
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
    Directory(DirectoryContext),
    /*
    Symbol(SymbolContext),
    FetchedUrl(FetchedUrlContext),
    Thread(ThreadContext),
    Selection(SelectionContext),
    Rules(RulesContext),
    Image(ImageContext),
    */
}

impl AssistantContext {
    pub fn element_id(&self, name: &'static str) -> ElementId {
        // TODO: Ideally ElementId would have types that can avoid building a name String here.
        match self {
            Self::File(context) => ElementId::NamedInteger(
                (name.to_string() + "-file").into(),
                // TODO: avoid potential panic here on 32 bit machines
                context.buffer.entity_id().as_u64().try_into().unwrap(),
            ),
            Self::Directory(context) => ElementId::NamedInteger(
                (name.to_string() + "-directory").into(),
                context.entry_id.to_usize(),
            ),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FileContext {
    pub buffer: Entity<Buffer>,
}

/// File
impl FileContext {
    pub fn project_path(&self, project: &Project, cx: &App) -> Option<ProjectPath> {
        /*
        let worktree = project.worktree_for_entry(self.entry_id, cx)?.read(cx);
        let entry = worktree.entry_for_id(self.entry_id)?;
        Some(ProjectPath {
            worktree_id: worktree.id(),
            path: entry.path.clone(),
        })
        */
        todo!()
    }

    fn load(self, cx: &mut App) -> Option<Task<(String, Entity<Buffer>)>> {
        let buffer_ref = self.buffer.read(cx);
        let Some(file) = buffer_ref.file() else {
            log::error!("file context missing path");
            return None;
        };
        let full_path = file.full_path(cx);
        let rope = buffer_ref.as_rope().clone();
        let buffer = self.buffer.clone();
        Some(
            cx.background_spawn(
                async move { (to_fenced_codeblock(&full_path, rope, None), buffer) },
            ),
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DirectoryContext {
    pub entry_id: ProjectEntryId,
}

impl DirectoryContext {
    /* todo!
    pub fn entry<'a>(&self, cx: &'a App) -> Option<&'a project::Entry> {
        self.worktree.read(cx).entry_for_id(self.entry_id)
    }

    pub fn project_path(&self, project: &Project, cx: &App) -> Option<ProjectPath> {
        let worktree = project.worktree_for_entry(self.entry_id, cx)?.read(cx);
        let entry = worktree.entry_for_id(self.entry_id)?;
        Some(ProjectPath {
            worktree_id: worktree.id(),
            path: entry.path,
        })
    }
    */

    fn load(
        self,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Option<Task<Vec<(String, Entity<Buffer>)>>> {
        let worktree = project.read(cx).worktree_for_entry(self.entry_id, cx)?;
        let worktree_ref = worktree.read(cx);
        let entry = worktree_ref.entry_for_id(self.entry_id)?;
        if entry.is_file() {
            log::error!("DirectoryContext unexpectedly refers to a file.");
            return None;
        }

        // todo! Make sure this handles all descendants
        let file_paths = worktree_ref
            .child_entries(entry.path.as_ref())
            .filter_map(|entry| {
                if entry.is_file() {
                    Some(entry.path.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let texts_future = future::join_all(file_paths.into_iter().map(|path| {
            load_file_path_text_as_fenced_codeblock(project.clone(), worktree.clone(), path, cx)
        }));

        Some(cx.background_spawn(async move {
            texts_future
                .await
                .into_iter()
                .filter_map(|option| option)
                .collect::<Vec<_>>()
        }))
    }
}

/*
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

#[derive(Debug, Clone)]
pub struct SelectionContext {
    pub id: ContextId,
    pub range: Range<Anchor>,
    pub line_range: Range<Point>,
    pub context_buffer: ContextBuffer,
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
pub fn load_context<'a>(
    contexts: impl Iterator<Item = &'a AssistantContext>,
    project: Entity<Project>,
    cx: &mut App,
) -> Task<(Option<String>, HashSet<Entity<Buffer>>)> {
    let mut file_context_tasks = Vec::new();
    let mut directory_context_tasks = Vec::new();
    /*
    let mut symbol_context = Vec::new();
    let mut excerpt_context = Vec::new();
    let mut fetch_context = Vec::new();
    let mut thread_context = Vec::new();
    let mut rules_context = Vec::new();
    */

    let contexts = contexts.cloned().collect::<Vec<_>>();

    for context in contexts {
        match context {
            AssistantContext::File(context) => file_context_tasks.extend(context.load(cx)),
            AssistantContext::Directory(context) => {
                directory_context_tasks.extend(context.load(project.clone(), cx))
            } /*
              AssistantContext::Symbol(context) => symbol_context.push(context.load(cx)),
              AssistantContext::Excerpt(context) => excerpt_context.push(context.load(cx)),
              AssistantContext::FetchedUrl(context) => fetch_context.push(context.load(cx)),
              AssistantContext::Thread(context) => thread_context.push(context.load(cx)),
              AssistantContext::Rules(context) => rules_context.push(context.load(cx)),
              */
        }
    }

    if file_context_tasks.is_empty() && directory_context_tasks.is_empty()
    /*
    && symbol_context.is_empty()
    && excerpt_context.is_empty()
    && fetch_context.is_empty()
    && thread_context.is_empty()
    && rules_context.is_empty()
    */
    {
        return Task::ready((None, HashSet::default()));
    }

    cx.background_spawn(async move {
        let (file_context, directory_context) =
            futures::join!(
                future::join_all(file_context_tasks),
                future::join_all(directory_context_tasks));

        let directory_context = directory_context.into_iter().flat_map(|context| context).collect::<Vec<_>>();

        let mut buffers = HashSet::default();

        let mut result = String::new();
        result.push_str("\n<context>\n\
            The following items were attached by the user. You don't need to use other tools to read them.\n\n");

        if !file_context.is_empty() {
            result.push_str("<files>\n");
            for (text, buffer) in file_context {
                result.push_str(&text);
                buffers.insert(buffer);
            }
            result.push_str("</files>\n");
        }

        if !directory_context.is_empty() {
            result.push_str("<directories>\n");
            for (text, buffer) in directory_context {
                result.push_str(&text);
                buffers.insert(buffer);
            }
            result.push_str("</directories>\n");
        }

        /*
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

        if !selection_context.is_empty() {
            result.push_str("<selections>\n");
            for context in selection_context {
                result.push_str(&context.context_buffer.text);
                result.push('\n');
            }
            result.push_str("</selections>\n");
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
        (Some(result), buffers)
    })
}

/* todo!
fn collect_files_in_path(worktree: &Worktree, path: &Path) -> Vec<Arc<Path>> {
    let mut files = Vec::new();

    for entry in worktree.child_entries(path) {
        if entry.is_dir() {
            files.extend(collect_files_in_path(worktree, &entry.path));
        } else if entry.is_file() {
            files.push(entry.path.clone());
        }
    }

    files
}
*/

fn load_file_path_text_as_fenced_codeblock(
    project: Entity<Project>,
    worktree: Entity<Worktree>,
    path: Arc<Path>,
    cx: &mut App,
) -> Task<Option<(String, Entity<Buffer>)>> {
    let worktree_ref = worktree.read(cx);
    let worktree_id = worktree_ref.id();
    let full_path = worktree_ref.full_path(&path);

    let open_task = project.update(cx, |project, cx| {
        project.buffer_store().update(cx, |buffer_store, cx| {
            let project_path = ProjectPath { worktree_id, path };
            buffer_store.open_buffer(project_path, cx)
        })
    });

    let rope_task = cx.spawn(async move |cx| {
        let buffer = open_task.await.log_err()?;
        let rope = buffer
            .read_with(cx, |buffer, _cx| buffer.as_rope().clone())
            .log_err()?;
        Some((rope, buffer))
    });

    cx.background_spawn(async move {
        let (rope, buffer) = rope_task.await?;
        Some((to_fenced_codeblock(&full_path, rope, None), buffer))
    })
}

fn to_fenced_codeblock(
    full_path: &Path,
    content: Rope,
    line_range: Option<Range<Point>>,
) -> String {
    let line_range_text = line_range.map(|range| {
        if range.start.row == range.end.row {
            format!(":{}", range.start.row + 1)
        } else {
            format!(":{}-{}", range.start.row + 1, range.end.row + 1)
        }
    });

    let path_extension = full_path.extension().and_then(|ext| ext.to_str());
    let path_string = full_path.to_string_lossy();
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
        buffer.push_str(chunk);
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
