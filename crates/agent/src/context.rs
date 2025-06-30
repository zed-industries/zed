use crate::thread::Thread;
use assistant_context::AssistantContext;
use assistant_tool::outline;
use collections::HashSet;
use futures::future;
use futures::{FutureExt, future::Shared};
use gpui::{App, AppContext as _, ElementId, Entity, SharedString, Task};
use icons::IconName;
use language::{Buffer, ParseStatus};
use language_model::{LanguageModelImage, LanguageModelRequestMessage, MessageContent};
use project::{Project, ProjectEntryId, ProjectPath, Worktree};
use prompt_store::{PromptStore, UserPromptId};
use ref_cast::RefCast;
use rope::Point;
use std::fmt::{self, Display, Formatter, Write as _};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::{ops::Range, path::Path, sync::Arc};
use text::{Anchor, OffsetRangeExt as _};
use util::markdown::MarkdownCodeBlock;
use util::{ResultExt as _, post_inc};

pub const RULES_ICON: IconName = IconName::Context;

pub enum ContextKind {
    File,
    Directory,
    Symbol,
    Selection,
    FetchedUrl,
    Thread,
    TextThread,
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
            ContextKind::TextThread => IconName::MessageBubbles,
            ContextKind::Rules => RULES_ICON,
            ContextKind::Image => IconName::Image,
        }
    }
}

/// Handle for context that can be attached to a user message.
///
/// This uses IDs that are stable enough for tracking renames and identifying when context has
/// already been added to the thread. To use this in a set, wrap it in `AgentContextKey` to opt in
/// to `PartialEq` and `Hash` impls that use the subset of the fields used for this stable identity.
#[derive(Debug, Clone)]
pub enum AgentContextHandle {
    File(FileContextHandle),
    Directory(DirectoryContextHandle),
    Symbol(SymbolContextHandle),
    Selection(SelectionContextHandle),
    FetchedUrl(FetchedUrlContext),
    Thread(ThreadContextHandle),
    TextThread(TextThreadContextHandle),
    Rules(RulesContextHandle),
    Image(ImageContext),
}

impl AgentContextHandle {
    pub fn id(&self) -> ContextId {
        match self {
            Self::File(context) => context.context_id,
            Self::Directory(context) => context.context_id,
            Self::Symbol(context) => context.context_id,
            Self::Selection(context) => context.context_id,
            Self::FetchedUrl(context) => context.context_id,
            Self::Thread(context) => context.context_id,
            Self::TextThread(context) => context.context_id,
            Self::Rules(context) => context.context_id,
            Self::Image(context) => context.context_id,
        }
    }

    pub fn element_id(&self, name: SharedString) -> ElementId {
        ElementId::NamedInteger(name, self.id().0)
    }
}

/// Loaded context that can be attached to a user message. This can be thought of as a
/// snapshot of the context along with an `AgentContextHandle`.
#[derive(Debug, Clone)]
pub enum AgentContext {
    File(FileContext),
    Directory(DirectoryContext),
    Symbol(SymbolContext),
    Selection(SelectionContext),
    FetchedUrl(FetchedUrlContext),
    Thread(ThreadContext),
    TextThread(TextThreadContext),
    Rules(RulesContext),
    Image(ImageContext),
}

impl AgentContext {
    pub fn handle(&self) -> AgentContextHandle {
        match self {
            AgentContext::File(context) => AgentContextHandle::File(context.handle.clone()),
            AgentContext::Directory(context) => {
                AgentContextHandle::Directory(context.handle.clone())
            }
            AgentContext::Symbol(context) => AgentContextHandle::Symbol(context.handle.clone()),
            AgentContext::Selection(context) => {
                AgentContextHandle::Selection(context.handle.clone())
            }
            AgentContext::FetchedUrl(context) => AgentContextHandle::FetchedUrl(context.clone()),
            AgentContext::Thread(context) => AgentContextHandle::Thread(context.handle.clone()),
            AgentContext::TextThread(context) => {
                AgentContextHandle::TextThread(context.handle.clone())
            }
            AgentContext::Rules(context) => AgentContextHandle::Rules(context.handle.clone()),
            AgentContext::Image(context) => AgentContextHandle::Image(context.clone()),
        }
    }
}

/// ID created at time of context add, for use in ElementId. This is not the stable identity of a
/// context, instead that's handled by the `PartialEq` and `Hash` impls of `AgentContextKey`.
#[derive(Debug, Copy, Clone)]
pub struct ContextId(u64);

impl ContextId {
    pub fn zero() -> Self {
        ContextId(0)
    }

    fn for_lookup() -> Self {
        ContextId(u64::MAX)
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
pub struct FileContextHandle {
    pub buffer: Entity<Buffer>,
    pub context_id: ContextId,
}

#[derive(Debug, Clone)]
pub struct FileContext {
    pub handle: FileContextHandle,
    pub full_path: Arc<Path>,
    pub text: SharedString,
    pub is_outline: bool,
}

impl FileContextHandle {
    pub fn eq_for_key(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }

    pub fn hash_for_key<H: Hasher>(&self, state: &mut H) {
        self.buffer.hash(state)
    }

    pub fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        let file = self.buffer.read(cx).file()?;
        Some(ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        })
    }

    fn load(self, cx: &App) -> Task<Option<(AgentContext, Vec<Entity<Buffer>>)>> {
        let buffer_ref = self.buffer.read(cx);
        let Some(file) = buffer_ref.file() else {
            log::error!("file context missing path");
            return Task::ready(None);
        };
        let full_path: Arc<Path> = file.full_path(cx).into();
        let rope = buffer_ref.as_rope().clone();
        let buffer = self.buffer.clone();

        cx.spawn(async move |cx| {
            // For large files, use outline instead of full content
            if rope.len() > outline::AUTO_OUTLINE_SIZE {
                // Wait until the buffer has been fully parsed, so we can read its outline
                if let Ok(mut parse_status) =
                    buffer.read_with(cx, |buffer, _| buffer.parse_status())
                {
                    while *parse_status.borrow() != ParseStatus::Idle {
                        parse_status.changed().await.log_err();
                    }

                    if let Ok(snapshot) = buffer.read_with(cx, |buffer, _| buffer.snapshot()) {
                        if let Some(outline) = snapshot.outline(None) {
                            let items = outline
                                .items
                                .into_iter()
                                .map(|item| item.to_point(&snapshot));

                            if let Ok(outline_text) =
                                outline::render_outline(items, None, 0, usize::MAX).await
                            {
                                let context = AgentContext::File(FileContext {
                                    handle: self,
                                    full_path,
                                    text: outline_text.into(),
                                    is_outline: true,
                                });
                                return Some((context, vec![buffer]));
                            }
                        }
                    }
                }
            }

            // Fallback to full content if we couldn't build an outline
            // (or didn't need to because the file was small enough)
            let context = AgentContext::File(FileContext {
                handle: self,
                full_path,
                text: rope.to_string().into(),
                is_outline: false,
            });
            Some((context, vec![buffer]))
        })
    }
}

impl Display for FileContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            MarkdownCodeBlock {
                tag: &codeblock_tag(&self.full_path, None),
                text: &self.text,
            }
        )
    }
}

/// Directory contents provides the entire contents of text files in a directory.
///
/// This has a `ProjectEntryId` so that it follows renames.
#[derive(Debug, Clone)]
pub struct DirectoryContextHandle {
    pub entry_id: ProjectEntryId,
    pub context_id: ContextId,
}

#[derive(Debug, Clone)]
pub struct DirectoryContext {
    pub handle: DirectoryContextHandle,
    pub full_path: Arc<Path>,
    pub descendants: Vec<DirectoryContextDescendant>,
}

#[derive(Debug, Clone)]
pub struct DirectoryContextDescendant {
    /// Path within the directory.
    pub rel_path: Arc<Path>,
    pub fenced_codeblock: SharedString,
}

impl DirectoryContextHandle {
    pub fn eq_for_key(&self, other: &Self) -> bool {
        self.entry_id == other.entry_id
    }

    pub fn hash_for_key<H: Hasher>(&self, state: &mut H) {
        self.entry_id.hash(state)
    }

    fn load(
        self,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Option<(AgentContext, Vec<Entity<Buffer>>)>> {
        let Some(worktree) = project.read(cx).worktree_for_entry(self.entry_id, cx) else {
            return Task::ready(None);
        };
        let worktree_ref = worktree.read(cx);
        let Some(entry) = worktree_ref.entry_for_id(self.entry_id) else {
            return Task::ready(None);
        };
        if entry.is_file() {
            log::error!("DirectoryContext unexpectedly refers to a file.");
            return Task::ready(None);
        }

        let directory_path = entry.path.clone();
        let directory_full_path = worktree_ref.full_path(&directory_path).into();

        let file_paths = collect_files_in_path(worktree_ref, &directory_path);
        let descendants_future = future::join_all(file_paths.into_iter().map(|path| {
            let worktree_ref = worktree.read(cx);
            let worktree_id = worktree_ref.id();
            let full_path = worktree_ref.full_path(&path);

            let rel_path = path
                .strip_prefix(&directory_path)
                .log_err()
                .map_or_else(|| path.clone(), |rel_path| rel_path.into());

            let open_task = project.update(cx, |project, cx| {
                project.buffer_store().update(cx, |buffer_store, cx| {
                    let project_path = ProjectPath { worktree_id, path };
                    buffer_store.open_buffer(project_path, cx)
                })
            });

            // TODO: report load errors instead of just logging
            let rope_task = cx.spawn(async move |cx| {
                let buffer = open_task.await.log_err()?;
                let rope = buffer
                    .read_with(cx, |buffer, _cx| buffer.as_rope().clone())
                    .log_err()?;
                Some((rope, buffer))
            });

            cx.background_spawn(async move {
                let (rope, buffer) = rope_task.await?;
                let fenced_codeblock = MarkdownCodeBlock {
                    tag: &codeblock_tag(&full_path, None),
                    text: &rope.to_string(),
                }
                .to_string()
                .into();
                let descendant = DirectoryContextDescendant {
                    rel_path,
                    fenced_codeblock,
                };
                Some((descendant, buffer))
            })
        }));

        cx.background_spawn(async move {
            let (descendants, buffers) = descendants_future.await.into_iter().flatten().unzip();
            let context = AgentContext::Directory(DirectoryContext {
                handle: self,
                full_path: directory_full_path,
                descendants,
            });
            Some((context, buffers))
        })
    }
}

impl Display for DirectoryContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_first = true;
        for descendant in &self.descendants {
            if !is_first {
                write!(f, "\n")?;
            } else {
                is_first = false;
            }
            write!(f, "{}", descendant.fenced_codeblock)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SymbolContextHandle {
    pub buffer: Entity<Buffer>,
    pub symbol: SharedString,
    pub range: Range<Anchor>,
    /// The range that fully contains the symbol. e.g. for function symbol, this will include not
    /// only the signature, but also the body. Not used by `PartialEq` or `Hash` for
    /// `AgentContextKey`.
    pub enclosing_range: Range<Anchor>,
    pub context_id: ContextId,
}

#[derive(Debug, Clone)]
pub struct SymbolContext {
    pub handle: SymbolContextHandle,
    pub full_path: Arc<Path>,
    pub line_range: Range<Point>,
    pub text: SharedString,
}

impl SymbolContextHandle {
    pub fn eq_for_key(&self, other: &Self) -> bool {
        self.buffer == other.buffer && self.symbol == other.symbol && self.range == other.range
    }

    pub fn hash_for_key<H: Hasher>(&self, state: &mut H) {
        self.buffer.hash(state);
        self.symbol.hash(state);
        self.range.hash(state);
    }

    pub fn full_path(&self, cx: &App) -> Option<PathBuf> {
        Some(self.buffer.read(cx).file()?.full_path(cx))
    }

    pub fn enclosing_line_range(&self, cx: &App) -> Range<Point> {
        self.enclosing_range
            .to_point(&self.buffer.read(cx).snapshot())
    }

    pub fn text(&self, cx: &App) -> SharedString {
        self.buffer
            .read(cx)
            .text_for_range(self.enclosing_range.clone())
            .collect::<String>()
            .into()
    }

    fn load(self, cx: &App) -> Task<Option<(AgentContext, Vec<Entity<Buffer>>)>> {
        let buffer_ref = self.buffer.read(cx);
        let Some(file) = buffer_ref.file() else {
            log::error!("symbol context's file has no path");
            return Task::ready(None);
        };
        let full_path = file.full_path(cx).into();
        let line_range = self.enclosing_range.to_point(&buffer_ref.snapshot());
        let text = self.text(cx);
        let buffer = self.buffer.clone();
        let context = AgentContext::Symbol(SymbolContext {
            handle: self,
            full_path,
            line_range,
            text,
        });
        Task::ready(Some((context, vec![buffer])))
    }
}

impl Display for SymbolContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code_block = MarkdownCodeBlock {
            tag: &codeblock_tag(&self.full_path, Some(self.line_range.clone())),
            text: &self.text,
        };
        write!(f, "{code_block}",)
    }
}

#[derive(Debug, Clone)]
pub struct SelectionContextHandle {
    pub buffer: Entity<Buffer>,
    pub range: Range<Anchor>,
    pub context_id: ContextId,
}

#[derive(Debug, Clone)]
pub struct SelectionContext {
    pub handle: SelectionContextHandle,
    pub full_path: Arc<Path>,
    pub line_range: Range<Point>,
    pub text: SharedString,
}

impl SelectionContextHandle {
    pub fn eq_for_key(&self, other: &Self) -> bool {
        self.buffer == other.buffer && self.range == other.range
    }

    pub fn hash_for_key<H: Hasher>(&self, state: &mut H) {
        self.buffer.hash(state);
        self.range.hash(state);
    }

    pub fn full_path(&self, cx: &App) -> Option<PathBuf> {
        Some(self.buffer.read(cx).file()?.full_path(cx))
    }

    pub fn line_range(&self, cx: &App) -> Range<Point> {
        self.range.to_point(&self.buffer.read(cx).snapshot())
    }

    pub fn text(&self, cx: &App) -> SharedString {
        self.buffer
            .read(cx)
            .text_for_range(self.range.clone())
            .collect::<String>()
            .into()
    }

    fn load(self, cx: &App) -> Task<Option<(AgentContext, Vec<Entity<Buffer>>)>> {
        let Some(full_path) = self.full_path(cx) else {
            log::error!("selection context's file has no path");
            return Task::ready(None);
        };
        let text = self.text(cx);
        let buffer = self.buffer.clone();
        let context = AgentContext::Selection(SelectionContext {
            full_path: full_path.into(),
            line_range: self.line_range(cx),
            text,
            handle: self,
        });

        Task::ready(Some((context, vec![buffer])))
    }
}

impl Display for SelectionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code_block = MarkdownCodeBlock {
            tag: &codeblock_tag(&self.full_path, Some(self.line_range.clone())),
            text: &self.text,
        };
        write!(f, "{code_block}",)
    }
}

#[derive(Debug, Clone)]
pub struct FetchedUrlContext {
    pub url: SharedString,
    /// Text contents of the fetched url. Unlike other context types, the contents of this gets
    /// populated when added rather than when sending the message. Not used by `PartialEq` or `Hash`
    /// for `AgentContextKey`.
    pub text: SharedString,
    pub context_id: ContextId,
}

impl FetchedUrlContext {
    pub fn eq_for_key(&self, other: &Self) -> bool {
        self.url == other.url
    }

    pub fn hash_for_key<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }

    pub fn lookup_key(url: SharedString) -> AgentContextKey {
        AgentContextKey(AgentContextHandle::FetchedUrl(FetchedUrlContext {
            url,
            text: "".into(),
            context_id: ContextId::for_lookup(),
        }))
    }

    pub fn load(self) -> Task<Option<(AgentContext, Vec<Entity<Buffer>>)>> {
        Task::ready(Some((AgentContext::FetchedUrl(self), vec![])))
    }
}

impl Display for FetchedUrlContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Better format - url and contents are not delimited.
        write!(f, "{}\n{}\n", self.url, self.text)
    }
}

#[derive(Debug, Clone)]
pub struct ThreadContextHandle {
    pub thread: Entity<Thread>,
    pub context_id: ContextId,
}

#[derive(Debug, Clone)]
pub struct ThreadContext {
    pub handle: ThreadContextHandle,
    pub title: SharedString,
    pub text: SharedString,
}

impl ThreadContextHandle {
    pub fn eq_for_key(&self, other: &Self) -> bool {
        self.thread == other.thread
    }

    pub fn hash_for_key<H: Hasher>(&self, state: &mut H) {
        self.thread.hash(state)
    }

    pub fn title(&self, cx: &App) -> SharedString {
        self.thread.read(cx).summary().or_default()
    }

    fn load(self, cx: &App) -> Task<Option<(AgentContext, Vec<Entity<Buffer>>)>> {
        cx.spawn(async move |cx| {
            let text = Thread::wait_for_detailed_summary_or_text(&self.thread, cx).await?;
            let title = self
                .thread
                .read_with(cx, |thread, _cx| thread.summary().or_default())
                .ok()?;
            let context = AgentContext::Thread(ThreadContext {
                title,
                text,
                handle: self,
            });
            Some((context, vec![]))
        })
    }
}

impl Display for ThreadContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: Better format for this - doesn't distinguish title and contents.
        write!(f, "{}\n{}\n", &self.title, &self.text.trim())
    }
}

#[derive(Debug, Clone)]
pub struct TextThreadContextHandle {
    pub context: Entity<AssistantContext>,
    pub context_id: ContextId,
}

#[derive(Debug, Clone)]
pub struct TextThreadContext {
    pub handle: TextThreadContextHandle,
    pub title: SharedString,
    pub text: SharedString,
}

impl TextThreadContextHandle {
    // pub fn lookup_key() ->
    pub fn eq_for_key(&self, other: &Self) -> bool {
        self.context == other.context
    }

    pub fn hash_for_key<H: Hasher>(&self, state: &mut H) {
        self.context.hash(state)
    }

    pub fn title(&self, cx: &App) -> SharedString {
        self.context.read(cx).summary().or_default()
    }

    fn load(self, cx: &App) -> Task<Option<(AgentContext, Vec<Entity<Buffer>>)>> {
        let title = self.title(cx);
        let text = self.context.read(cx).to_xml(cx);
        let context = AgentContext::TextThread(TextThreadContext {
            title,
            text: text.into(),
            handle: self,
        });
        Task::ready(Some((context, vec![])))
    }
}

impl Display for TextThreadContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: escape title?
        write!(f, "<text_thread title=\"{}\">\n", self.title)?;
        write!(f, "{}", self.text.trim())?;
        write!(f, "\n</text_thread>")
    }
}

#[derive(Debug, Clone)]
pub struct RulesContextHandle {
    pub prompt_id: UserPromptId,
    pub context_id: ContextId,
}

#[derive(Debug, Clone)]
pub struct RulesContext {
    pub handle: RulesContextHandle,
    pub title: Option<SharedString>,
    pub text: SharedString,
}

impl RulesContextHandle {
    pub fn eq_for_key(&self, other: &Self) -> bool {
        self.prompt_id == other.prompt_id
    }

    pub fn hash_for_key<H: Hasher>(&self, state: &mut H) {
        self.prompt_id.hash(state)
    }

    pub fn lookup_key(prompt_id: UserPromptId) -> AgentContextKey {
        AgentContextKey(AgentContextHandle::Rules(RulesContextHandle {
            prompt_id,
            context_id: ContextId::for_lookup(),
        }))
    }

    pub fn load(
        self,
        prompt_store: &Option<Entity<PromptStore>>,
        cx: &App,
    ) -> Task<Option<(AgentContext, Vec<Entity<Buffer>>)>> {
        let Some(prompt_store) = prompt_store.as_ref() else {
            return Task::ready(None);
        };
        let prompt_store = prompt_store.read(cx);
        let prompt_id = self.prompt_id.into();
        let Some(metadata) = prompt_store.metadata(prompt_id) else {
            return Task::ready(None);
        };
        let title = metadata.title;
        let text_task = prompt_store.load(prompt_id, cx);
        cx.background_spawn(async move {
            // TODO: report load errors instead of just logging
            let text = text_task.await.log_err()?.into();
            let context = AgentContext::Rules(RulesContext {
                handle: self,
                title,
                text,
            });
            Some((context, vec![]))
        })
    }
}

impl Display for RulesContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(title) = &self.title {
            write!(f, "Rules title: {}\n", title)?;
        }
        let code_block = MarkdownCodeBlock {
            tag: "",
            text: self.text.trim(),
        };
        write!(f, "{code_block}")
    }
}

#[derive(Debug, Clone)]
pub struct ImageContext {
    pub project_path: Option<ProjectPath>,
    pub full_path: Option<Arc<Path>>,
    pub original_image: Arc<gpui::Image>,
    // TODO: handle this elsewhere and remove `ignore-interior-mutability` opt-out in clippy.toml
    // needed due to a false positive of `clippy::mutable_key_type`.
    pub image_task: Shared<Task<Option<LanguageModelImage>>>,
    pub context_id: ContextId,
}

pub enum ImageStatus {
    Loading,
    Error,
    Warning,
    Ready,
}

impl ImageContext {
    pub fn eq_for_key(&self, other: &Self) -> bool {
        self.original_image.id() == other.original_image.id()
    }

    pub fn hash_for_key<H: Hasher>(&self, state: &mut H) {
        self.original_image.id().hash(state);
    }

    pub fn image(&self) -> Option<LanguageModelImage> {
        self.image_task.clone().now_or_never().flatten()
    }

    pub fn status(&self, model: Option<&Arc<dyn language_model::LanguageModel>>) -> ImageStatus {
        match self.image_task.clone().now_or_never() {
            None => ImageStatus::Loading,
            Some(None) => ImageStatus::Error,
            Some(Some(_)) => {
                if model.is_some_and(|model| !model.supports_images()) {
                    ImageStatus::Warning
                } else {
                    ImageStatus::Ready
                }
            }
        }
    }

    pub fn load(self, cx: &App) -> Task<Option<(AgentContext, Vec<Entity<Buffer>>)>> {
        cx.background_spawn(async move {
            self.image_task.clone().await;
            Some((AgentContext::Image(self), vec![]))
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct ContextLoadResult {
    pub loaded_context: LoadedContext,
    pub referenced_buffers: HashSet<Entity<Buffer>>,
}

#[derive(Debug, Clone, Default)]
pub struct LoadedContext {
    pub contexts: Vec<AgentContext>,
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

    pub fn add_to_request_message_with_model(
        &self,
        request_message: &mut LanguageModelRequestMessage,
        model: &Arc<dyn language_model::LanguageModel>,
    ) {
        if !self.text.is_empty() {
            request_message
                .content
                .push(MessageContent::Text(self.text.to_string()));
        }

        if !self.images.is_empty() {
            let max_image_size = model.max_image_size();
            let mut images_added = false;

            for image in &self.images {
                let image_size = image.len() as u64;
                if image_size > max_image_size {
                    if max_image_size == 0 {
                        log::warn!(
                            "Skipping image attachment: model {:?} does not support images",
                            model.name()
                        );
                    } else {
                        log::warn!(
                            "Skipping image attachment: size {} bytes exceeds model {:?} limit of {} bytes",
                            image_size,
                            model.name(),
                            max_image_size
                        );
                    }
                    continue;
                }

                // Some providers only support image parts after an initial text part
                if !images_added && request_message.content.is_empty() {
                    request_message
                        .content
                        .push(MessageContent::Text("Images attached by user:".to_string()));
                }

                request_message
                    .content
                    .push(MessageContent::Image(image.clone()));
                images_added = true;
            }
        }
    }

    /// Checks images against model size limits and returns information about rejected images
    pub fn check_image_size_limits(
        &self,
        model: &Arc<dyn language_model::LanguageModel>,
    ) -> Vec<RejectedImage> {
        let mut rejected_images = Vec::new();

        if !self.images.is_empty() {
            let max_image_size = model.max_image_size();

            for image in &self.images {
                let image_size = image.len() as u64;
                if image_size > max_image_size {
                    rejected_images.push(RejectedImage {
                        size: image_size,
                        max_size: max_image_size,
                        model_name: model.name().0.to_string(),
                    });
                }
            }
        }

        rejected_images
    }

    pub fn add_to_request_message_with_validation<F>(
        &self,
        request_message: &mut LanguageModelRequestMessage,
        model: &Arc<dyn language_model::LanguageModel>,
        mut on_image_rejected: F,
    ) where
        F: FnMut(u64, u64, &str),
    {
        if !self.text.is_empty() {
            request_message
                .content
                .push(MessageContent::Text(self.text.to_string()));
        }

        if !self.images.is_empty() {
            let max_image_size = model.max_image_size();
            let mut images_added = false;

            for image in &self.images {
                let image_size = image.len() as u64;
                if image_size > max_image_size {
                    on_image_rejected(image_size, max_image_size, &model.name().0);

                    if max_image_size == 0 {
                        log::warn!(
                            "Skipping image attachment: model {:?} does not support images",
                            model.name()
                        );
                    } else {
                        log::warn!(
                            "Skipping image attachment: size {} bytes exceeds model {:?} limit of {} bytes",
                            image_size,
                            model.name(),
                            max_image_size
                        );
                    }
                    continue;
                }

                // Some providers only support image parts after an initial text part
                if !images_added && request_message.content.is_empty() {
                    request_message
                        .content
                        .push(MessageContent::Text("Images attached by user:".to_string()));
                }

                request_message
                    .content
                    .push(MessageContent::Image(image.clone()));
                images_added = true;
            }
        }
    }
}

/// Loads and formats a collection of contexts.
pub fn load_context(
    contexts: Vec<AgentContextHandle>,
    project: &Entity<Project>,
    prompt_store: &Option<Entity<PromptStore>>,
    cx: &mut App,
) -> Task<ContextLoadResult> {
    let load_tasks: Vec<_> = contexts
        .into_iter()
        .map(|context| match context {
            AgentContextHandle::File(context) => context.load(cx),
            AgentContextHandle::Directory(context) => context.load(project.clone(), cx),
            AgentContextHandle::Symbol(context) => context.load(cx),
            AgentContextHandle::Selection(context) => context.load(cx),
            AgentContextHandle::FetchedUrl(context) => context.load(),
            AgentContextHandle::Thread(context) => context.load(cx),
            AgentContextHandle::TextThread(context) => context.load(cx),
            AgentContextHandle::Rules(context) => context.load(prompt_store, cx),
            AgentContextHandle::Image(context) => context.load(cx),
        })
        .collect();

    cx.background_spawn(async move {
        let load_results = future::join_all(load_tasks).await;

        let mut contexts = Vec::new();
        let mut text = String::new();
        let mut referenced_buffers = HashSet::default();
        for context in load_results {
            let Some((context, buffers)) = context else {
                continue;
            };
            contexts.push(context);
            referenced_buffers.extend(buffers);
        }

        let mut file_context = Vec::new();
        let mut directory_context = Vec::new();
        let mut symbol_context = Vec::new();
        let mut selection_context = Vec::new();
        let mut fetched_url_context = Vec::new();
        let mut thread_context = Vec::new();
        let mut text_thread_context = Vec::new();
        let mut rules_context = Vec::new();
        let mut images = Vec::new();
        for context in &contexts {
            match context {
                AgentContext::File(context) => file_context.push(context),
                AgentContext::Directory(context) => directory_context.push(context),
                AgentContext::Symbol(context) => symbol_context.push(context),
                AgentContext::Selection(context) => selection_context.push(context),
                AgentContext::FetchedUrl(context) => fetched_url_context.push(context),
                AgentContext::Thread(context) => thread_context.push(context),
                AgentContext::TextThread(context) => text_thread_context.push(context),
                AgentContext::Rules(context) => rules_context.push(context),
                AgentContext::Image(context) => images.extend(context.image()),
            }
        }

        // Use empty text if there are no contexts that contribute to text (everything but image
        // context).
        if file_context.is_empty()
            && directory_context.is_empty()
            && symbol_context.is_empty()
            && selection_context.is_empty()
            && fetched_url_context.is_empty()
            && thread_context.is_empty()
            && text_thread_context.is_empty()
            && rules_context.is_empty()
        {
            return ContextLoadResult {
                loaded_context: LoadedContext {
                    contexts,
                    text,
                    images,
                },
                referenced_buffers,
            };
        }

        text.push_str(
            "\n<context>\n\
            The following items were attached by the user. \
            They are up-to-date and don't need to be re-read.\n\n",
        );

        if !file_context.is_empty() {
            text.push_str("<files>");
            for context in file_context {
                text.push('\n');
                let _ = write!(text, "{context}");
            }
            text.push_str("</files>\n");
        }

        if !directory_context.is_empty() {
            text.push_str("<directories>");
            for context in directory_context {
                text.push('\n');
                let _ = write!(text, "{context}");
            }
            text.push_str("</directories>\n");
        }

        if !symbol_context.is_empty() {
            text.push_str("<symbols>");
            for context in symbol_context {
                text.push('\n');
                let _ = write!(text, "{context}");
            }
            text.push_str("</symbols>\n");
        }

        if !selection_context.is_empty() {
            text.push_str("<selections>");
            for context in selection_context {
                text.push('\n');
                let _ = write!(text, "{context}");
            }
            text.push_str("</selections>\n");
        }

        if !fetched_url_context.is_empty() {
            text.push_str("<fetched_urls>");
            for context in fetched_url_context {
                text.push('\n');
                let _ = write!(text, "{context}");
            }
            text.push_str("</fetched_urls>\n");
        }

        if !thread_context.is_empty() {
            text.push_str("<conversation_threads>");
            for context in thread_context {
                text.push('\n');
                let _ = write!(text, "{context}");
            }
            text.push_str("</conversation_threads>\n");
        }

        if !text_thread_context.is_empty() {
            text.push_str("<text_threads>");
            for context in text_thread_context {
                text.push('\n');
                let _ = writeln!(text, "{context}");
            }
            text.push_str("<text_threads>");
        }

        if !rules_context.is_empty() {
            text.push_str(
                "<user_rules>\n\
                The user has specified the following rules that should be applied:\n",
            );
            for context in rules_context {
                text.push('\n');
                let _ = write!(text, "{context}");
            }
            text.push_str("</user_rules>\n");
        }

        text.push_str("</context>\n");

        ContextLoadResult {
            loaded_context: LoadedContext {
                contexts,
                text,
                images,
            },
            referenced_buffers,
        }
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

fn codeblock_tag(full_path: &Path, line_range: Option<Range<Point>>) -> String {
    let mut result = String::new();

    if let Some(extension) = full_path.extension().and_then(|ext| ext.to_str()) {
        let _ = write!(result, "{} ", extension);
    }

    let _ = write!(result, "{}", full_path.display());

    if let Some(range) = line_range {
        if range.start.row == range.end.row {
            let _ = write!(result, ":{}", range.start.row + 1);
        } else {
            let _ = write!(result, ":{}-{}", range.start.row + 1, range.end.row + 1);
        }
    }

    result
}

/// Wraps `AgentContext` to opt-in to `PartialEq` and `Hash` impls which use a subset of fields
/// needed for stable context identity.
#[derive(Debug, Clone, RefCast)]
#[repr(transparent)]
pub struct AgentContextKey(pub AgentContextHandle);

impl AsRef<AgentContextHandle> for AgentContextKey {
    fn as_ref(&self) -> &AgentContextHandle {
        &self.0
    }
}

impl Eq for AgentContextKey {}

impl PartialEq for AgentContextKey {
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            AgentContextHandle::File(context) => {
                if let AgentContextHandle::File(other_context) = &other.0 {
                    return context.eq_for_key(other_context);
                }
            }
            AgentContextHandle::Directory(context) => {
                if let AgentContextHandle::Directory(other_context) = &other.0 {
                    return context.eq_for_key(other_context);
                }
            }
            AgentContextHandle::Symbol(context) => {
                if let AgentContextHandle::Symbol(other_context) = &other.0 {
                    return context.eq_for_key(other_context);
                }
            }
            AgentContextHandle::Selection(context) => {
                if let AgentContextHandle::Selection(other_context) = &other.0 {
                    return context.eq_for_key(other_context);
                }
            }
            AgentContextHandle::FetchedUrl(context) => {
                if let AgentContextHandle::FetchedUrl(other_context) = &other.0 {
                    return context.eq_for_key(other_context);
                }
            }
            AgentContextHandle::Thread(context) => {
                if let AgentContextHandle::Thread(other_context) = &other.0 {
                    return context.eq_for_key(other_context);
                }
            }
            AgentContextHandle::Rules(context) => {
                if let AgentContextHandle::Rules(other_context) = &other.0 {
                    return context.eq_for_key(other_context);
                }
            }
            AgentContextHandle::Image(context) => {
                if let AgentContextHandle::Image(other_context) = &other.0 {
                    return context.eq_for_key(other_context);
                }
            }
            AgentContextHandle::TextThread(context) => {
                if let AgentContextHandle::TextThread(other_context) = &other.0 {
                    return context.eq_for_key(other_context);
                }
            }
        }
        false
    }
}

impl Hash for AgentContextKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            AgentContextHandle::File(context) => context.hash_for_key(state),
            AgentContextHandle::Directory(context) => context.hash_for_key(state),
            AgentContextHandle::Symbol(context) => context.hash_for_key(state),
            AgentContextHandle::Selection(context) => context.hash_for_key(state),
            AgentContextHandle::FetchedUrl(context) => context.hash_for_key(state),
            AgentContextHandle::Thread(context) => context.hash_for_key(state),
            AgentContextHandle::TextThread(context) => context.hash_for_key(state),
            AgentContextHandle::Rules(context) => context.hash_for_key(state),
            AgentContextHandle::Image(context) => context.hash_for_key(state),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RejectedImage {
    pub size: u64,
    pub max_size: u64,
    pub model_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AsyncApp, TestAppContext};
    use language_model::{LanguageModelCacheConfiguration, LanguageModelId, LanguageModelName};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    fn init_test_settings(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });
    }

    // Helper to create a test project with test files
    async fn create_test_project(
        cx: &mut TestAppContext,
        files: serde_json::Value,
    ) -> Entity<Project> {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/test"), files).await;
        Project::test(fs, [path!("/test").as_ref()], cx).await
    }

    #[gpui::test]
    async fn test_large_file_uses_outline(cx: &mut TestAppContext) {
        init_test_settings(cx);

        // Create a large file that exceeds AUTO_OUTLINE_SIZE
        const LINE: &str = "Line with some text\n";
        let large_content = LINE.repeat(2 * (outline::AUTO_OUTLINE_SIZE / LINE.len()));
        let content_len = large_content.len();

        assert!(content_len > outline::AUTO_OUTLINE_SIZE);

        let file_context = file_context_for(large_content, cx).await;

        assert!(
            file_context.is_outline,
            "Large file should use outline format"
        );

        assert!(
            file_context.text.len() < content_len,
            "Outline should be smaller than original content"
        );
    }

    #[gpui::test]
    async fn test_small_file_uses_full_content(cx: &mut TestAppContext) {
        init_test_settings(cx);

        let small_content = "This is a small file.\n";
        let content_len = small_content.len();

        assert!(content_len < outline::AUTO_OUTLINE_SIZE);

        let file_context = file_context_for(small_content.to_string(), cx).await;

        assert!(
            !file_context.is_outline,
            "Small files should not get an outline"
        );

        assert_eq!(file_context.text, small_content);
    }

    async fn file_context_for(content: String, cx: &mut TestAppContext) -> FileContext {
        // Create a test project with the file
        let project = create_test_project(
            cx,
            json!({
                "file.txt": content,
            }),
        )
        .await;

        // Open the buffer
        let buffer_path = project
            .read_with(cx, |project, cx| project.find_project_path("file.txt", cx))
            .unwrap();

        let buffer = project
            .update(cx, |project, cx| project.open_buffer(buffer_path, cx))
            .await
            .unwrap();

        let context_handle = AgentContextHandle::File(FileContextHandle {
            buffer: buffer.clone(),
            context_id: ContextId::zero(),
        });

        cx.update(|cx| load_context(vec![context_handle], &project, &None, cx))
            .await
            .loaded_context
            .contexts
            .into_iter()
            .find_map(|ctx| {
                if let AgentContext::File(file_ctx) = ctx {
                    Some(file_ctx)
                } else {
                    None
                }
            })
            .expect("Should have found a file context")
    }

    #[gpui::test]
    async fn test_image_size_limit_filtering(_cx: &mut TestAppContext) {
        use futures::stream::BoxStream;
        use gpui::{AsyncApp, DevicePixels, SharedString};
        use language_model::{
            LanguageModelId, LanguageModelImage, LanguageModelName, LanguageModelProviderId,
            LanguageModelProviderName, Role,
        };
        use std::sync::Arc;

        // Create a mock image that's 10 bytes
        let small_image = LanguageModelImage {
            source: "small_data".into(),
            size: gpui::size(DevicePixels(10), DevicePixels(10)),
        };

        // Create a mock image that's 1MB
        let large_image_source = "x".repeat(1_048_576);
        let large_image = LanguageModelImage {
            source: large_image_source.into(),
            size: gpui::size(DevicePixels(1024), DevicePixels(1024)),
        };

        let loaded_context = LoadedContext {
            contexts: vec![],
            text: "Some text".to_string(),
            images: vec![small_image.clone(), large_image.clone()],
        };

        // Test with a model that supports images with 500KB limit
        struct TestModel500KB;
        impl language_model::LanguageModel for TestModel500KB {
            fn id(&self) -> LanguageModelId {
                LanguageModelId(SharedString::from("test-500kb"))
            }
            fn name(&self) -> LanguageModelName {
                LanguageModelName(SharedString::from("Test Model 500KB"))
            }
            fn provider_id(&self) -> LanguageModelProviderId {
                LanguageModelProviderId(SharedString::from("test"))
            }
            fn provider_name(&self) -> LanguageModelProviderName {
                LanguageModelProviderName(SharedString::from("Test Provider"))
            }
            fn supports_tools(&self) -> bool {
                false
            }
            fn supports_tool_choice(&self, _: language_model::LanguageModelToolChoice) -> bool {
                false
            }
            fn max_image_size(&self) -> u64 {
                512_000
            } // 500KB
            fn telemetry_id(&self) -> String {
                "test-500kb".to_string()
            }
            fn max_token_count(&self) -> u64 {
                100_000
            }
            fn count_tokens(
                &self,
                _request: language_model::LanguageModelRequest,
                _cx: &App,
            ) -> futures::future::BoxFuture<'static, anyhow::Result<u64>> {
                Box::pin(async { Ok(0) })
            }
            fn stream_completion(
                &self,
                _request: language_model::LanguageModelRequest,
                _cx: &AsyncApp,
            ) -> futures::future::BoxFuture<
                'static,
                Result<
                    BoxStream<
                        'static,
                        Result<
                            language_model::LanguageModelCompletionEvent,
                            language_model::LanguageModelCompletionError,
                        >,
                    >,
                    language_model::LanguageModelCompletionError,
                >,
            > {
                use language_model::LanguageModelCompletionError;
                Box::pin(async {
                    Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                        "Not implemented"
                    )))
                })
            }
        }

        let model_500kb: Arc<dyn language_model::LanguageModel> = Arc::new(TestModel500KB);
        let mut request_message = LanguageModelRequestMessage {
            role: Role::User,
            content: vec![],
            cache: false,
        };

        loaded_context.add_to_request_message_with_model(&mut request_message, &model_500kb);

        // Should have text and only the small image
        assert_eq!(request_message.content.len(), 2); // text + small image
        assert!(
            matches!(&request_message.content[0], MessageContent::Text(text) if text == "Some text")
        );
        assert!(matches!(
            &request_message.content[1],
            MessageContent::Image(_)
        ));

        // Test with a model that doesn't support images
        struct TestModelNoImages;
        impl language_model::LanguageModel for TestModelNoImages {
            fn id(&self) -> LanguageModelId {
                LanguageModelId(SharedString::from("test-no-images"))
            }
            fn name(&self) -> LanguageModelName {
                LanguageModelName(SharedString::from("Test Model No Images"))
            }
            fn provider_id(&self) -> LanguageModelProviderId {
                LanguageModelProviderId(SharedString::from("test"))
            }
            fn provider_name(&self) -> LanguageModelProviderName {
                LanguageModelProviderName(SharedString::from("Test Provider"))
            }
            fn supports_tools(&self) -> bool {
                false
            }
            fn supports_tool_choice(&self, _: language_model::LanguageModelToolChoice) -> bool {
                false
            }
            fn max_image_size(&self) -> u64 {
                0
            } // No image support
            fn telemetry_id(&self) -> String {
                "test-no-images".to_string()
            }
            fn max_token_count(&self) -> u64 {
                100_000
            }
            fn count_tokens(
                &self,
                _request: language_model::LanguageModelRequest,
                _cx: &App,
            ) -> futures::future::BoxFuture<'static, anyhow::Result<u64>> {
                Box::pin(async { Ok(0) })
            }
            fn stream_completion(
                &self,
                _request: language_model::LanguageModelRequest,
                _cx: &AsyncApp,
            ) -> futures::future::BoxFuture<
                'static,
                Result<
                    BoxStream<
                        'static,
                        Result<
                            language_model::LanguageModelCompletionEvent,
                            language_model::LanguageModelCompletionError,
                        >,
                    >,
                    language_model::LanguageModelCompletionError,
                >,
            > {
                use language_model::LanguageModelCompletionError;
                Box::pin(async {
                    Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                        "Not implemented"
                    )))
                })
            }
        }

        let model_no_images: Arc<dyn language_model::LanguageModel> = Arc::new(TestModelNoImages);
        let mut request_message_no_images = LanguageModelRequestMessage {
            role: Role::User,
            content: vec![],
            cache: false,
        };

        loaded_context
            .add_to_request_message_with_model(&mut request_message_no_images, &model_no_images);

        // Should have only text, no images
        assert_eq!(request_message_no_images.content.len(), 1);
        assert!(
            matches!(&request_message_no_images.content[0], MessageContent::Text(text) if text == "Some text")
        );
    }

    #[gpui::test]
    async fn test_check_image_size_limits() {
        use gpui::DevicePixels;
        use language_model::LanguageModelImage;

        // Create test images of various sizes
        let tiny_image = LanguageModelImage {
            source: "tiny".into(),
            size: gpui::size(DevicePixels(10), DevicePixels(10)),
        };

        let small_image = LanguageModelImage {
            source: "x".repeat(100_000).into(), // 100KB
            size: gpui::size(DevicePixels(100), DevicePixels(100)),
        };

        let medium_image = LanguageModelImage {
            source: "x".repeat(500_000).into(), // 500KB
            size: gpui::size(DevicePixels(500), DevicePixels(500)),
        };

        let large_image = LanguageModelImage {
            source: "x".repeat(1_048_576).into(), // 1MB
            size: gpui::size(DevicePixels(1024), DevicePixels(1024)),
        };

        let huge_image = LanguageModelImage {
            source: "x".repeat(5_242_880).into(), // 5MB
            size: gpui::size(DevicePixels(2048), DevicePixels(2048)),
        };

        // Test with model that has 1MB limit
        let model_1mb = Arc::new(TestModel1MB);
        let loaded_context = LoadedContext {
            contexts: vec![],
            text: String::new(),
            images: vec![
                tiny_image.clone(),
                small_image.clone(),
                medium_image.clone(),
                large_image.clone(),
                huge_image.clone(),
            ],
        };

        let rejected = loaded_context.check_image_size_limits(
            &(model_1mb.clone() as Arc<dyn language_model::LanguageModel>),
        );
        assert_eq!(rejected.len(), 1);
        assert_eq!(rejected[0].size, 5_242_880);
        assert_eq!(rejected[0].max_size, 1_048_576);
        assert_eq!(rejected[0].model_name, "Test Model 1MB");

        // Test with model that doesn't support images
        let model_no_images = Arc::new(TestModelNoImages);
        let rejected = loaded_context.check_image_size_limits(
            &(model_no_images.clone() as Arc<dyn language_model::LanguageModel>),
        );
        assert_eq!(rejected.len(), 5); // All images rejected
        for (_i, rejected_image) in rejected.iter().enumerate() {
            assert_eq!(rejected_image.max_size, 0);
            assert_eq!(rejected_image.model_name, "Test Model No Images");
        }

        // Test with empty image list
        let empty_context = LoadedContext {
            contexts: vec![],
            text: String::new(),
            images: vec![],
        };
        let rejected = empty_context.check_image_size_limits(
            &(model_1mb.clone() as Arc<dyn language_model::LanguageModel>),
        );
        assert!(rejected.is_empty());

        // Test with all images within limit
        let small_context = LoadedContext {
            contexts: vec![],
            text: String::new(),
            images: vec![tiny_image.clone(), small_image.clone()],
        };
        let rejected = small_context
            .check_image_size_limits(&(model_1mb as Arc<dyn language_model::LanguageModel>));
        assert!(rejected.is_empty());
    }

    #[gpui::test]
    async fn test_add_to_request_message_with_validation() {
        use gpui::DevicePixels;
        use language_model::{LanguageModelImage, MessageContent, Role};

        let small_image = LanguageModelImage {
            source: "small".into(),
            size: gpui::size(DevicePixels(10), DevicePixels(10)),
        };

        let large_image = LanguageModelImage {
            source: "x".repeat(2_097_152).into(), // 2MB
            size: gpui::size(DevicePixels(1024), DevicePixels(1024)),
        };

        let loaded_context = LoadedContext {
            contexts: vec![],
            text: "Test message".to_string(),
            images: vec![small_image.clone(), large_image.clone()],
        };

        let model = Arc::new(TestModel1MB);
        let mut request_message = LanguageModelRequestMessage {
            role: Role::User,
            content: Vec::new(),
            cache: false,
        };

        let mut rejected_count = 0;
        let mut rejected_sizes = Vec::new();
        let mut rejected_model_names = Vec::new();

        loaded_context.add_to_request_message_with_validation(
            &mut request_message,
            &(model.clone() as Arc<dyn language_model::LanguageModel>),
            |size, max_size, model_name| {
                rejected_count += 1;
                rejected_sizes.push((size, max_size));
                rejected_model_names.push(model_name.to_string());
            },
        );

        // Verify callback was called for the large image
        assert_eq!(rejected_count, 1);
        assert_eq!(rejected_sizes[0], (2_097_152, 1_048_576));
        assert_eq!(rejected_model_names[0], "Test Model 1MB");

        // Verify the request message contains text and only the small image
        assert_eq!(request_message.content.len(), 2); // text + small image
        assert!(
            matches!(&request_message.content[0], MessageContent::Text(text) if text == "Test message")
        );
        assert!(matches!(
            &request_message.content[1],
            MessageContent::Image(_)
        ));
    }

    // Helper test models
    struct TestModel1MB;
    impl language_model::LanguageModel for TestModel1MB {
        fn id(&self) -> LanguageModelId {
            LanguageModelId(SharedString::from("test-1mb"))
        }
        fn name(&self) -> LanguageModelName {
            LanguageModelName(SharedString::from("Test Model 1MB"))
        }
        fn provider_id(&self) -> language_model::LanguageModelProviderId {
            language_model::LanguageModelProviderId(SharedString::from("test"))
        }
        fn provider_name(&self) -> language_model::LanguageModelProviderName {
            language_model::LanguageModelProviderName(SharedString::from("Test Provider"))
        }
        fn supports_tools(&self) -> bool {
            false
        }
        fn supports_tool_choice(&self, _: language_model::LanguageModelToolChoice) -> bool {
            false
        }
        fn max_image_size(&self) -> u64 {
            1_048_576 // 1MB
        }
        fn telemetry_id(&self) -> String {
            "test-1mb".to_string()
        }
        fn max_token_count(&self) -> u64 {
            100_000
        }
        fn max_output_tokens(&self) -> Option<u64> {
            Some(4096)
        }
        fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
            Some(LanguageModelCacheConfiguration {
                max_cache_anchors: 0,
                should_speculate: false,
                min_total_token: 1024,
            })
        }
        fn count_tokens(
            &self,
            _request: language_model::LanguageModelRequest,
            _cx: &App,
        ) -> futures::future::BoxFuture<'static, anyhow::Result<u64>> {
            Box::pin(async { Ok(0) })
        }
        fn stream_completion(
            &self,
            _request: language_model::LanguageModelRequest,
            _cx: &AsyncApp,
        ) -> futures::future::BoxFuture<
            'static,
            Result<
                futures::stream::BoxStream<
                    'static,
                    Result<
                        language_model::LanguageModelCompletionEvent,
                        language_model::LanguageModelCompletionError,
                    >,
                >,
                language_model::LanguageModelCompletionError,
            >,
        > {
            use language_model::LanguageModelCompletionError;
            Box::pin(async {
                Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                    "Not implemented"
                )))
            })
        }
    }

    struct TestModelNoImages;
    impl language_model::LanguageModel for TestModelNoImages {
        fn id(&self) -> LanguageModelId {
            LanguageModelId(SharedString::from("test-no-images"))
        }
        fn name(&self) -> LanguageModelName {
            LanguageModelName(SharedString::from("Test Model No Images"))
        }
        fn provider_id(&self) -> language_model::LanguageModelProviderId {
            language_model::LanguageModelProviderId(SharedString::from("test"))
        }
        fn provider_name(&self) -> language_model::LanguageModelProviderName {
            language_model::LanguageModelProviderName(SharedString::from("Test Provider"))
        }
        fn supports_tools(&self) -> bool {
            false
        }
        fn supports_tool_choice(&self, _: language_model::LanguageModelToolChoice) -> bool {
            false
        }
        fn max_image_size(&self) -> u64 {
            0 // No image support
        }
        fn telemetry_id(&self) -> String {
            "test-no-images".to_string()
        }
        fn max_token_count(&self) -> u64 {
            100_000
        }
        fn max_output_tokens(&self) -> Option<u64> {
            Some(4096)
        }
        fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
            Some(LanguageModelCacheConfiguration {
                max_cache_anchors: 0,
                should_speculate: false,
                min_total_token: 1024,
            })
        }
        fn count_tokens(
            &self,
            _request: language_model::LanguageModelRequest,
            _cx: &App,
        ) -> futures::future::BoxFuture<'static, anyhow::Result<u64>> {
            Box::pin(async { Ok(0) })
        }
        fn stream_completion(
            &self,
            _request: language_model::LanguageModelRequest,
            _cx: &AsyncApp,
        ) -> futures::future::BoxFuture<
            'static,
            Result<
                futures::stream::BoxStream<
                    'static,
                    Result<
                        language_model::LanguageModelCompletionEvent,
                        language_model::LanguageModelCompletionError,
                    >,
                >,
                language_model::LanguageModelCompletionError,
            >,
        > {
            use language_model::LanguageModelCompletionError;
            Box::pin(async {
                Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                    "Not implemented"
                )))
            })
        }
    }
}
