use std::hash::{Hash, Hasher};
use std::usize;
use std::{ops::Range, path::Path, sync::Arc};

use collections::HashSet;
use futures::future;
use futures::{FutureExt, future::Shared};
use gpui::{App, AppContext as _, Entity, SharedString, Task};
use language::Buffer;
use language_model::{LanguageModelImage, LanguageModelRequestMessage, MessageContent};
use project::{Project, ProjectEntryId, ProjectPath, Worktree};
use prompt_store::{PromptStore, UserPromptId};
use ref_cast::RefCast;
use rope::{Point, Rope};
use text::{Anchor, OffsetRangeExt as _};
use ui::{ElementId, IconName};
use util::{ResultExt as _, post_inc};

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
#[derive(Debug, Clone)]
pub enum AssistantContext {
    File(FileContext),
    Directory(DirectoryContext),
    Symbol(SymbolContext),
    Selection(SelectionContext),
    FetchedUrl(FetchedUrlContext),
    Thread(ThreadContext),
    Rules(RulesContext),
    Image(ImageContext),
}

impl AssistantContext {
    fn id(&self) -> ContextId {
        match self {
            Self::File(context) => context.context_id,
            Self::Directory(context) => context.context_id,
            Self::Symbol(context) => context.context_id,
            Self::Selection(context) => context.context_id,
            Self::FetchedUrl(context) => context.context_id,
            Self::Thread(context) => context.context_id,
            Self::Rules(context) => context.context_id,
            Self::Image(context) => context.context_id,
        }
    }

    pub fn element_id(&self, name: SharedString) -> ElementId {
        ElementId::NamedInteger(name, self.id().0)
    }
}

/// ID created at time of context add, for use in ElementId. This is not the stable identity of a
/// context, instead that's handled by the `Eq` and `Hash` of `ContextSetEntry`.
#[derive(Debug, Copy, Clone)]
pub struct ContextId(usize);

impl ContextId {
    pub fn zero() -> Self {
        ContextId(0)
    }

    fn for_query() -> Self {
        ContextId(usize::MAX)
    }

    pub fn post_inc(&mut self) -> Self {
        Self(post_inc(&mut self.0))
    }
}

/// File context provides the entire contents of a file.
///
/// This holds an `Entity<Buffer>` so that file path renames affect its display and so that it can
/// be opened even if the file has been deleted. An alternative might be to use `ProjectEntryId`,
/// but then when deleted there is no path info or ability to open.
#[derive(Debug, Clone)]
pub struct FileContext {
    pub buffer: Entity<Buffer>,
    pub context_id: ContextId,
}

impl FileContext {
    pub fn eq_for_context_set(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }

    pub fn hash_for_context_set<H: Hasher>(&self, state: &mut H) {
        self.buffer.hash(state)
    }

    pub fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        let file = self.buffer.read(cx).file()?;
        Some(ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        })
    }

    fn load(self, cx: &App) -> Option<Task<(String, Entity<Buffer>)>> {
        let buffer_ref = self.buffer.read(cx);
        let Some(file) = buffer_ref.file() else {
            log::error!("file context missing path");
            return None;
        };
        let full_path = file.full_path(cx);
        let rope = buffer_ref.as_rope().clone();
        let buffer = self.buffer;
        Some(
            cx.background_spawn(
                async move { (to_fenced_codeblock(&full_path, rope, None), buffer) },
            ),
        )
    }
}

/// Directory contents provides the entire contents of text files in a directory.
///
/// This has a `ProjectEntryId` so that it follows renames.
#[derive(Debug, Clone)]
pub struct DirectoryContext {
    pub entry_id: ProjectEntryId,
    pub context_id: ContextId,
}

impl DirectoryContext {
    pub fn eq_for_context_set(&self, other: &Self) -> bool {
        self.entry_id == other.entry_id
    }

    pub fn hash_for_context_set<H: Hasher>(&self, state: &mut H) {
        self.entry_id.hash(state)
    }

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

        let file_paths = collect_files_in_path(worktree_ref, entry.path.as_ref());
        let texts_future = future::join_all(file_paths.into_iter().map(|path| {
            load_file_path_text_as_fenced_codeblock(project.clone(), worktree.clone(), path, cx)
        }));

        Some(cx.background_spawn(async move {
            texts_future.await.into_iter().flatten().collect::<Vec<_>>()
        }))
    }
}

#[derive(Debug, Clone)]
pub struct SymbolContext {
    pub buffer: Entity<Buffer>,
    pub symbol: SharedString,
    pub range: Range<Anchor>,
    /// The range that fully contain the symbol. e.g. for function symbol, this will include not
    /// only the signature, but also the body.
    ///
    /// Note: not used by Eq and Hash for ContextSetEntry
    pub enclosing_range: Range<Anchor>,
    pub context_id: ContextId,
}

impl SymbolContext {
    pub fn eq_for_context_set(&self, other: &Self) -> bool {
        self.buffer == other.buffer && self.symbol == other.symbol && self.range == other.range
    }

    pub fn hash_for_context_set<H: Hasher>(&self, state: &mut H) {
        self.buffer.hash(state);
        self.symbol.hash(state);
        self.range.hash(state);
    }

    fn load(self, cx: &App) -> Option<Task<(String, Entity<Buffer>)>> {
        let buffer_ref = self.buffer.read(cx);
        let Some(file) = buffer_ref.file() else {
            log::error!("symbol context's file has no path");
            return None;
        };
        let full_path = file.full_path(cx);
        let rope = buffer_ref
            .text_for_range(self.enclosing_range.clone())
            .collect::<Rope>();
        let line_range = self.enclosing_range.to_point(&buffer_ref.snapshot());
        let buffer = self.buffer;
        Some(cx.background_spawn(async move {
            (
                to_fenced_codeblock(&full_path, rope, Some(line_range)),
                buffer,
            )
        }))
    }
}

#[derive(Debug, Clone)]
pub struct SelectionContext {
    pub buffer: Entity<Buffer>,
    pub range: Range<Anchor>,
    pub context_id: ContextId,
}

impl SelectionContext {
    pub fn eq_for_context_set(&self, other: &Self) -> bool {
        self.buffer == other.buffer && self.range == other.range
    }

    pub fn hash_for_context_set<H: Hasher>(&self, state: &mut H) {
        self.buffer.hash(state);
        self.range.hash(state);
    }

    fn load(self, cx: &App) -> Option<Task<(String, Entity<Buffer>)>> {
        let buffer_ref = self.buffer.read(cx);
        let Some(file) = buffer_ref.file() else {
            log::error!("selection context's file has no path");
            return None;
        };
        let full_path = file.full_path(cx);
        let rope = buffer_ref
            .text_for_range(self.range.clone())
            .collect::<Rope>();
        let line_range = self.range.to_point(&buffer_ref.snapshot());
        let buffer = self.buffer;
        Some(cx.background_spawn(async move {
            (
                to_fenced_codeblock(&full_path, rope, Some(line_range)),
                buffer,
            )
        }))
    }
}

#[derive(Debug, Clone)]
pub struct FetchedUrlContext {
    pub url: SharedString,
    /// Text contents of the fetched url. Unlike other context types, the contents of this gets
    /// populated when added rather than when sending the message.
    ///
    /// Note: not used by Eq and Hash for ContextSetEntry
    pub text: SharedString,
    pub context_id: ContextId,
}

impl FetchedUrlContext {
    pub fn eq_for_context_set(&self, other: &Self) -> bool {
        self.url == other.url
    }

    pub fn hash_for_context_set<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }

    pub fn new_context_set_query(url: SharedString) -> ContextSetEntry {
        ContextSetEntry(AssistantContext::FetchedUrl(FetchedUrlContext {
            url: url.into(),
            text: "".into(),
            context_id: ContextId::for_query(),
        }))
    }
}

#[derive(Debug, Clone)]
pub struct ThreadContext {
    pub thread: Entity<Thread>,
    pub context_id: ContextId,
}

impl ThreadContext {
    pub fn eq_for_context_set(&self, other: &Self) -> bool {
        self.thread == other.thread
    }

    pub fn hash_for_context_set<H: Hasher>(&self, state: &mut H) {
        self.thread.hash(state)
    }

    pub fn name(&self, cx: &App) -> SharedString {
        self.thread
            .read(cx)
            .summary()
            .unwrap_or_else(|| "New thread".into())
    }

    pub fn load(self, cx: &App) -> String {
        let name = self.name(cx);
        let contents = self.thread.read(cx).latest_detailed_summary_or_text();
        // todo! better format
        let mut text = String::new();
        text.push_str(&name);
        text.push('\n');
        text.push_str(&contents.trim());
        text.push('\n');
        text
    }
}

#[derive(Debug, Clone)]
pub struct RulesContext {
    pub prompt_id: UserPromptId,
    pub context_id: ContextId,
}

impl RulesContext {
    pub fn eq_for_context_set(&self, other: &Self) -> bool {
        self.prompt_id == other.prompt_id
    }

    pub fn hash_for_context_set<H: Hasher>(&self, state: &mut H) {
        self.prompt_id.hash(state)
    }

    pub fn new_context_set_query(prompt_id: UserPromptId) -> ContextSetEntry {
        ContextSetEntry(AssistantContext::Rules(RulesContext {
            prompt_id,
            context_id: ContextId::for_query(),
        }))
    }

    pub fn load(
        self,
        prompt_store: &Option<Entity<PromptStore>>,
        cx: &App,
    ) -> Task<Option<String>> {
        // todo! better error handling
        let Some(prompt_store) = prompt_store.as_ref() else {
            return Task::ready(None);
        };
        let prompt_store = prompt_store.read(cx);
        let prompt_id = self.prompt_id.into();
        let Some(metadata) = prompt_store.metadata(prompt_id) else {
            return Task::ready(None);
        };
        let contents_task = prompt_store.load(prompt_id, cx);
        cx.background_spawn(async move {
            // todo! better error handling
            let contents = contents_task.await.ok()?;
            let mut text = String::new();
            if let Some(title) = metadata.title {
                text.push_str("Rules title: ");
                text.push_str(&title);
                text.push('\n');
            }
            text.push_str("``````\n");
            text.push_str(contents.trim());
            text.push_str("\n``````\n");
            Some(text)
        })
    }
}

#[derive(Debug, Clone)]
pub struct ImageContext {
    pub original_image: Arc<gpui::Image>,
    pub image_task: Shared<Task<Option<LanguageModelImage>>>,
    pub context_id: ContextId,
}

pub enum ImageStatus {
    Loading,
    Error,
    Ready,
}

impl ImageContext {
    pub fn eq_for_context_set(&self, other: &Self) -> bool {
        self.original_image.id == other.original_image.id
    }

    pub fn hash_for_context_set<H: Hasher>(&self, state: &mut H) {
        self.original_image.id.hash(state);
    }

    pub fn image(&self) -> Option<LanguageModelImage> {
        self.image_task.clone().now_or_never().flatten()
    }

    pub fn status(&self) -> ImageStatus {
        match self.image_task.clone().now_or_never() {
            None => ImageStatus::Loading,
            Some(None) => ImageStatus::Error,
            Some(Some(_)) => ImageStatus::Ready,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LoadedContext {
    pub text: String,
    pub images: Vec<LanguageModelImage>,
}

impl LoadedContext {
    pub fn is_empty(&self) -> bool {
        self.text.is_empty() && self.images.is_empty()
    }

    pub fn add_to_request_message(&self, request_message: &mut LanguageModelRequestMessage) {
        if !self.text.is_empty() {
            request_message
                .content
                .push(MessageContent::Text(self.text.to_string()));
        }

        if !self.images.is_empty() {
            // Some providers only support image parts after an initial text part
            if request_message.content.is_empty() {
                request_message
                    .content
                    .push(MessageContent::Text("Images attached by user:".to_string()));
            }

            for image in &self.images {
                request_message
                    .content
                    .push(MessageContent::Image(image.clone()))
            }
        }
    }
}

/// Loads and formats a collection of contexts.
pub fn load_context(
    contexts: Vec<AssistantContext>,
    project: &Entity<Project>,
    prompt_store: &Option<Entity<PromptStore>>,
    cx: &mut App,
) -> Task<(LoadedContext, HashSet<Entity<Buffer>>)> {
    let mut file_tasks = Vec::new();
    let mut directory_tasks = Vec::new();
    let mut symbol_tasks = Vec::new();
    let mut selection_tasks = Vec::new();
    let mut fetch_context = Vec::new();
    let mut thread_context = Vec::new();
    let mut rules_tasks = Vec::new();
    let mut image_tasks = Vec::new();

    for context in contexts {
        match context {
            AssistantContext::File(context) => file_tasks.extend(context.load(cx)),
            AssistantContext::Directory(context) => {
                directory_tasks.extend(context.load(project.clone(), cx))
            }
            AssistantContext::Symbol(context) => symbol_tasks.extend(context.load(cx)),
            AssistantContext::Selection(context) => selection_tasks.extend(context.load(cx)),
            AssistantContext::FetchedUrl(context) => fetch_context.push(context),
            AssistantContext::Thread(context) => thread_context.push(context.load(cx)),
            AssistantContext::Rules(context) => rules_tasks.push(context.load(prompt_store, cx)),
            AssistantContext::Image(context) => image_tasks.push(context.image_task.clone()),
        }
    }

    cx.background_spawn(async move {
        let (
            file_context,
            directory_context,
            symbol_context,
            selection_context,
            rules_context,
            images,
        ) = futures::join!(
            future::join_all(file_tasks),
            future::join_all(directory_tasks),
            future::join_all(symbol_tasks),
            future::join_all(selection_tasks),
            future::join_all(rules_tasks),
            future::join_all(image_tasks)
        );

        let directory_context = directory_context.into_iter().flatten().collect::<Vec<_>>();
        let rules_context = rules_context.into_iter().flatten().collect::<Vec<_>>();
        let images = images.into_iter().flatten().collect::<Vec<_>>();

        let mut buffers = HashSet::default();
        let mut text = String::new();

        if file_context.is_empty()
            && directory_context.is_empty()
            && symbol_context.is_empty()
            && selection_context.is_empty()
            && fetch_context.is_empty()
            && thread_context.is_empty()
            && rules_context.is_empty()
        {
            return (LoadedContext { text, images }, buffers);
        }

        text.push_str(
            "\n<context>\n\
            The following items were attached by the user. \
            You don't need to use other tools to read them.\n\n",
        );

        if !file_context.is_empty() {
            text.push_str("<files>");
            for (file_text, buffer) in file_context {
                text.push('\n');
                text.push_str(&file_text);
                buffers.insert(buffer);
            }
            text.push_str("</files>\n");
        }

        if !directory_context.is_empty() {
            text.push_str("<directories>");
            for (file_text, buffer) in directory_context {
                text.push('\n');
                text.push_str(&file_text);
                buffers.insert(buffer);
            }
            text.push_str("</directories>\n");
        }

        if !symbol_context.is_empty() {
            text.push_str("<symbols>");
            for (symbol_text, buffer) in symbol_context {
                text.push('\n');
                text.push_str(&symbol_text);
                buffers.insert(buffer);
            }
            text.push_str("</symbols>\n");
        }

        if !selection_context.is_empty() {
            text.push_str("<selections>");
            for (selection_text, buffer) in selection_context {
                text.push('\n');
                text.push_str(&selection_text);
                buffers.insert(buffer);
            }
            text.push_str("</selections>\n");
        }

        if !fetch_context.is_empty() {
            text.push_str("<fetched_urls>");
            for context in fetch_context {
                // todo! Better formatting
                text.push('\n');
                text.push_str(&context.url);
                text.push('\n');
                text.push_str(&context.text);
            }
            text.push_str("</fetched_urls>\n");
        }

        if !thread_context.is_empty() {
            text.push_str("<conversation_threads>");
            for thread_text in thread_context {
                text.push('\n');
                text.push_str(&thread_text);
            }
            text.push_str("</conversation_threads>\n");
        }

        if !rules_context.is_empty() {
            text.push_str(
                "<user_rules>\n\
                The user has specified the following rules that should be applied:\n",
            );
            for rules_text in rules_context {
                text.push('\n');
                text.push_str(&rules_text);
            }
            text.push_str("</user_rules>\n");
        }

        text.push_str("</context>\n");

        (LoadedContext { text, images }, buffers)
    })
}

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

/// Wraps `AssistantContext` to opt-in to `PartialEq` and `Hash` impls which use a subset of fields
/// needed for stable context identity.
#[derive(Debug, Clone, RefCast)]
#[repr(transparent)]
pub struct ContextSetEntry(pub AssistantContext);

impl AsRef<AssistantContext> for ContextSetEntry {
    fn as_ref(&self) -> &AssistantContext {
        &self.0
    }
}

impl Eq for ContextSetEntry {}

impl PartialEq for ContextSetEntry {
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            AssistantContext::File(context) => {
                if let AssistantContext::File(other_context) = &other.0 {
                    return context.eq_for_context_set(other_context);
                }
            }
            AssistantContext::Directory(context) => {
                if let AssistantContext::Directory(other_context) = &other.0 {
                    return context.eq_for_context_set(other_context);
                }
            }
            AssistantContext::Symbol(context) => {
                if let AssistantContext::Symbol(other_context) = &other.0 {
                    return context.eq_for_context_set(other_context);
                }
            }
            AssistantContext::Selection(context) => {
                if let AssistantContext::Selection(other_context) = &other.0 {
                    return context.eq_for_context_set(other_context);
                }
            }
            AssistantContext::FetchedUrl(context) => {
                if let AssistantContext::FetchedUrl(other_context) = &other.0 {
                    return context.eq_for_context_set(other_context);
                }
            }
            AssistantContext::Thread(context) => {
                if let AssistantContext::Thread(other_context) = &other.0 {
                    return context.eq_for_context_set(other_context);
                }
            }
            AssistantContext::Rules(context) => {
                if let AssistantContext::Rules(other_context) = &other.0 {
                    return context.eq_for_context_set(other_context);
                }
            }
            AssistantContext::Image(context) => {
                if let AssistantContext::Image(other_context) = &other.0 {
                    return context.eq_for_context_set(other_context);
                }
            }
        }
        return false;
    }
}

impl Hash for ContextSetEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            AssistantContext::File(context) => context.hash_for_context_set(state),
            AssistantContext::Directory(context) => context.hash_for_context_set(state),
            AssistantContext::Symbol(context) => context.hash_for_context_set(state),
            AssistantContext::Selection(context) => context.hash_for_context_set(state),
            AssistantContext::FetchedUrl(context) => context.hash_for_context_set(state),
            AssistantContext::Thread(context) => context.hash_for_context_set(state),
            AssistantContext::Rules(context) => context.hash_for_context_set(state),
            AssistantContext::Image(context) => context.hash_for_context_set(state),
        }
    }
}
