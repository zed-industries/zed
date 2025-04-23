use std::hash::{Hash, Hasher};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use collections::{BTreeMap, HashMap, HashSet, IndexSet};
use futures::future::join_all;
use futures::{self, Future, FutureExt, future};
use gpui::{App, AppContext as _, Context, Entity, Image, SharedString, Task, WeakEntity};
use language::Buffer;
use language_model::LanguageModelImage;
use project::{Project, ProjectEntryId, ProjectItem, ProjectPath, Worktree};
use prompt_store::UserPromptId;
use ref_cast::RefCast;
use rope::{Point, Rope};
use text::{Anchor, BufferId, OffsetRangeExt};
use util::ResultExt as _;

use crate::ThreadStore;
use crate::context::{AssistantContext, DirectoryContext, FileContext, SymbolContext};
use crate::context_strip::SuggestedContext;
use crate::thread::{Thread, ThreadId};

pub struct ContextStore {
    project: WeakEntity<Project>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    thread_summary_tasks: Vec<Task<()>>,
    // todo! rename to context_set?
    context: IndexSet<ContextSetEntry>,
}

impl ContextStore {
    pub fn new(
        project: WeakEntity<Project>,
        thread_store: Option<WeakEntity<ThreadStore>>,
    ) -> Self {
        Self {
            project,
            thread_store,
            thread_summary_tasks: Vec::new(),
            context: IndexSet::default(),
        }
    }

    pub fn context(&self) -> impl Iterator<Item = &AssistantContext> {
        self.context.iter().map(|entry| entry.as_ref())
    }

    pub fn context_set(&self) -> &IndexSet<ContextSetEntry> {
        &self.context
    }

    pub fn clear(&mut self) {
        self.context.clear();
    }

    pub fn new_context_for_thread(&self, thread: &Thread) -> Vec<AssistantContext> {
        let existing_context = thread
            .messages()
            .flat_map(|message| &message.context)
            .map(|context| ContextSetEntry::ref_cast(context))
            .collect::<HashSet<_>>();
        self.context
            .iter()
            .filter(|context| !existing_context.contains(context))
            .map(|entry| entry.0.clone())
            .collect::<Vec<_>>()
    }

    pub fn add_file_from_path(
        &mut self,
        project_path: ProjectPath,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(Err(anyhow!("failed to read project")));
        };

        cx.spawn(async move |this, cx| {
            let open_buffer_task = project.update(cx, |project, cx| {
                project.open_buffer(project_path.clone(), cx)
            })?;

            let buffer = open_buffer_task.await?;
            let context = AssistantContext::File(FileContext { buffer });

            let already_included = this.update(cx, |this, cx| {
                if this.has_context(&context) {
                    if remove_if_exists {
                        this.remove_context(&context, cx);
                    }
                    true
                } else {
                    this.path_included_in_directory(&project_path, cx).is_some()
                }
            })?;

            if already_included {
                return anyhow::Ok(());
            }

            this.update(cx, |this, cx| {
                this.insert_context(context, cx);
            })?;

            anyhow::Ok(())
        })
    }

    pub fn add_directory(
        &mut self,
        project_path: ProjectPath,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let Some(project) = self.project.upgrade() else {
            return Err(anyhow!("failed to read project"));
        };

        let Some(entry_id) = project
            .read(cx)
            .entry_for_path(&project_path, cx)
            .map(|entry| entry.id)
        else {
            return Err(anyhow!("no entry found for directory context"));
        };

        let context = AssistantContext::Directory(DirectoryContext { entry_id });

        if self.has_context(&context) {
            if remove_if_exists {
                self.remove_context(&context, cx);
            }
            return Ok(());
        } else if self.path_included_in_directory(&project_path, cx).is_some() {
            return Ok(());
        }

        self.insert_context(context, cx);

        anyhow::Ok(())
    }

    pub fn add_symbol(
        &mut self,
        buffer: Entity<Buffer>,
        symbol: SharedString,
        range: Range<Anchor>,
        enclosing_range: Range<Anchor>,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let context = AssistantContext::Symbol(SymbolContext {
            buffer,
            symbol,
            range,
            enclosing_range,
        });

        if self.has_context(&context) {
            if remove_if_exists {
                self.remove_context(&context, cx);
            }
            return false;
        }

        self.insert_context(context, cx)
    }

    /*
    pub fn add_thread(
        &mut self,
        thread: Entity<Thread>,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) {
        if let Some(context_id) = self.includes_thread(&thread.read(cx).id()) {
            if remove_if_exists {
                self.remove_context(context_id, cx);
            }
        } else {
            self.insert_thread(thread, cx);
        }
    }

    pub fn wait_for_summaries(&mut self, cx: &App) -> Task<()> {
        let tasks = std::mem::take(&mut self.thread_summary_tasks);

        cx.spawn(async move |_cx| {
            join_all(tasks).await;
        })
    }

    fn insert_thread(&mut self, thread: Entity<Thread>, cx: &mut Context<Self>) {
        if let Some(summary_task) =
            thread.update(cx, |thread, cx| thread.generate_detailed_summary(cx))
        {
            let thread = thread.clone();
            let thread_store = self.thread_store.clone();

            self.thread_summary_tasks.push(cx.spawn(async move |_, cx| {
                summary_task.await;

                if let Some(thread_store) = thread_store {
                    // Save thread so its summary can be reused later
                    let save_task = thread_store
                        .update(cx, |thread_store, cx| thread_store.save_thread(&thread, cx));

                    if let Some(save_task) = save_task.ok() {
                        save_task.await.log_err();
                    }
                }
            }));
        }

        let id = self.next_context_id.post_inc();

        let text = thread.read(cx).latest_detailed_summary_or_text();

        self.threads.insert(thread.read(cx).id().clone(), id);
        self.context
            .push(AssistantContext::Thread(ThreadContext { id, thread, text }));
        cx.notify();
    }

    pub fn add_rules(
        &mut self,
        prompt_id: UserPromptId,
        title: impl Into<SharedString>,
        text: impl Into<SharedString>,
        remove_if_exists: bool,
        cx: &mut Context<ContextStore>,
    ) {
        if let Some(context_id) = self.includes_user_rules(&prompt_id) {
            if remove_if_exists {
                self.remove_context(context_id, cx);
            }
        } else {
            self.insert_user_rules(prompt_id, title, text, cx);
        }
    }

    pub fn insert_user_rules(
        &mut self,
        prompt_id: UserPromptId,
        title: impl Into<SharedString>,
        text: impl Into<SharedString>,
        cx: &mut Context<ContextStore>,
    ) {
        let id = self.next_context_id.post_inc();

        self.user_rules.insert(prompt_id, id);
        self.context.push(AssistantContext::Rules(RulesContext {
            id,
            prompt_id,
            title: title.into(),
            text: text.into(),
        }));
        cx.notify();
    }

    pub fn add_fetched_url(
        &mut self,
        url: String,
        text: impl Into<SharedString>,
        cx: &mut Context<ContextStore>,
    ) {
        if self.includes_url(&url).is_none() {
            self.insert_fetched_url(url, text, cx);
        }
    }

    fn insert_fetched_url(
        &mut self,
        url: String,
        text: impl Into<SharedString>,
        cx: &mut Context<ContextStore>,
    ) {
        let id = self.next_context_id.post_inc();

        self.fetched_urls.insert(url.clone(), id);
        self.context
            .push(AssistantContext::FetchedUrl(FetchedUrlContext {
                id,
                url: url.into(),
                text: text.into(),
            }));
        cx.notify();
    }

    pub fn add_image(&mut self, image: Arc<Image>, cx: &mut Context<ContextStore>) {
        let image_task = LanguageModelImage::from_image(image.clone(), cx).shared();
        let id = self.next_context_id.post_inc();
        self.context.push(AssistantContext::Image(ImageContext {
            id,
            original_image: image,
            image_task,
        }));
        cx.notify();
    }

    pub fn wait_for_images(&self, cx: &App) -> Task<()> {
        let tasks = self
            .context
            .iter()
            .filter_map(|ctx| match ctx {
                AssistantContext::Image(ctx) => Some(ctx.image_task.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        cx.spawn(async move |_cx| {
            join_all(tasks).await;
        })
    }

    pub fn add_selection(
        &mut self,
        buffer: Entity<Buffer>,
        range: Range<Anchor>,
        cx: &mut Context<ContextStore>,
    ) -> Task<Result<()>> {
        cx.spawn(async move |this, cx| {
            let (line_range, context_buffer_task) = this.update(cx, |_, cx| {
                load_context_buffer_range(buffer, range.clone(), cx)
            })??;

            let context_buffer = context_buffer_task.await;

            this.update(cx, |this, cx| {
                this.insert_selection(context_buffer, range, line_range, cx)
            })?;

            anyhow::Ok(())
        })
    }

    fn insert_selection(
        &mut self,
        context_buffer: ContextBuffer,
        range: Range<Anchor>,
        line_range: Range<Point>,
        cx: &mut Context<Self>,
    ) {
        let id = self.next_context_id.post_inc();
        self.context
            .push(AssistantContext::Selection(SelectionContext {
                id,
                range,
                line_range,
                context_buffer,
            }));
        cx.notify();
    }
    */

    pub fn accept_suggested_context(
        &mut self,
        suggested: &SuggestedContext,
        cx: &mut Context<ContextStore>,
    ) {
        match suggested {
            SuggestedContext::File {
                buffer,
                icon_path: _,
                name: _,
            } => {
                if let Some(buffer) = buffer.upgrade() {
                    self.insert_context(AssistantContext::File(FileContext { buffer }), cx);
                };
            } /*
              SuggestedContext::Thread { thread, name: _ } => {
                  if let Some(thread) = thread.upgrade() {
                      self.insert_thread(thread, cx);
                  };
              }
              */
        }
    }

    fn insert_context(&mut self, context: AssistantContext, cx: &mut Context<Self>) -> bool {
        let inserted = self.context.insert(ContextSetEntry(context));
        if inserted {
            cx.notify();
        }
        inserted
    }

    pub fn remove_context(&mut self, context: &AssistantContext, cx: &mut Context<Self>) {
        if self
            .context
            .shift_remove(ContextSetEntry::ref_cast(context))
        {
            cx.notify();
        }
    }

    pub fn has_context(&mut self, context: &AssistantContext) -> bool {
        self.context.contains(ContextSetEntry::ref_cast(context))
    }

    /// Returns whether this file path is already included directly in the context, or if it will be
    /// included in the context via a directory.
    pub fn file_path_included(&self, path: &ProjectPath, cx: &App) -> Option<FileInclusion> {
        let project = self.project.upgrade()?.read(cx);
        self.context().find_map(|context| match context {
            AssistantContext::File(file_context) => {
                FileInclusion::check_file(file_context, path, cx)
            }
            AssistantContext::Directory(directory_context) => {
                FileInclusion::check_directory(directory_context, path, project, cx)
            }
            _ => None,
        })
    }

    pub fn path_included_in_directory(
        &self,
        path: &ProjectPath,
        cx: &App,
    ) -> Option<FileInclusion> {
        let project = self.project.upgrade()?.read(cx);
        self.context().find_map(|context| match context {
            AssistantContext::Directory(directory_context) => {
                FileInclusion::check_directory(directory_context, path, project, cx)
            }
            _ => None,
        })
    }

    /*
    pub fn included_symbol(&self, symbol_id: &ContextSymbolId) -> Option<ContextId> {
        self.symbols.get(symbol_id).copied()
    }

    pub fn included_symbols_by_path(&self) -> &HashMap<ProjectPath, Vec<ContextSymbolId>> {
        &self.symbols_by_path
    }

    pub fn buffer_for_symbol(&self, symbol_id: &ContextSymbolId) -> Option<Entity<Buffer>> {
        self.symbol_buffers.get(symbol_id).cloned()
    }

    pub fn includes_thread(&self, thread_id: &ThreadId) -> Option<ContextId> {
        self.threads.get(thread_id).copied()
    }

    pub fn includes_user_rules(&self, prompt_id: &UserPromptId) -> Option<ContextId> {
        self.user_rules.get(prompt_id).copied()
    }

    pub fn includes_url(&self, url: &str) -> Option<ContextId> {
        self.fetched_urls.get(url).copied()
    }

    /// Replaces the context that matches the ID of the new context, if any match.
    fn replace_context(&mut self, new_context: AssistantContext) {
        let id = new_context.id();
        for context in self.context.iter_mut() {
            if context.id() == id {
                *context = new_context;
                break;
            }
        }
    }

    pub fn file_paths(&self, cx: &App) -> HashSet<ProjectPath> {
        self.context
            .iter()
            .filter_map(|context| match context {
                AssistantContext::File(file) => {
                    let buffer = file.context_buffer.buffer.read(cx);
                    buffer.project_path(cx)
                }
                AssistantContext::Directory(_)
                | AssistantContext::Symbol(_)
                | AssistantContext::Selection(_)
                | AssistantContext::FetchedUrl(_)
                | AssistantContext::Thread(_)
                | AssistantContext::Rules(_)
                | AssistantContext::Image(_) => None,
            })
            .collect()
    }

    pub fn thread_ids(&self) -> HashSet<ThreadId> {
        self.threads.keys().cloned().collect()
    }
    */
}

pub enum FileInclusion {
    Direct,
    InDirectory { full_path: PathBuf },
}

impl FileInclusion {
    fn check_file(file_context: &FileContext, path: &ProjectPath, cx: &App) -> Option<Self> {
        let file_path = file_context.buffer.read(cx).project_path(cx)?;
        if path == &file_path {
            Some(FileInclusion::Direct)
        } else {
            None
        }
    }

    fn check_directory(
        directory_context: &DirectoryContext,
        path: &ProjectPath,
        project: &Project,
        cx: &App,
    ) -> Option<Self> {
        let worktree = project
            .worktree_for_entry(directory_context.entry_id, cx)?
            .read(cx);
        let entry = worktree.entry_for_id(directory_context.entry_id)?;
        let directory_path = ProjectPath {
            worktree_id: worktree.id(),
            path: entry.path.clone(),
        };
        if path.starts_with(&directory_path) {
            if path == &directory_path {
                Some(FileInclusion::Direct)
            } else {
                Some(FileInclusion::InDirectory {
                    full_path: worktree.full_path(&entry.path),
                })
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, RefCast)]
#[repr(transparent)]
struct ContextSetEntry(AssistantContext);

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
            _ => {}
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
        }
    }
}

/*
fn make_context_symbol(
    context_buffer: ContextBuffer,
    path: ProjectPath,
    name: SharedString,
    range: Range<Anchor>,
    enclosing_range: Range<Anchor>,
) -> ContextSymbol {
    ContextSymbol {
        id: ContextSymbolId { name, range, path },
        buffer_version: context_buffer.version,
        enclosing_range,
        buffer: context_buffer.buffer,
        text: context_buffer.text,
    }
}

fn load_context_buffer_range(
    buffer: Entity<Buffer>,
    range: Range<Anchor>,
    cx: &App,
) -> Result<(Range<Point>, Task<ContextBuffer>)> {
    let buffer_ref = buffer.read(cx);
    let id = buffer_ref.remote_id();

    let file = buffer_ref.file().context("context buffer missing path")?;
    let full_path = file.full_path(cx);

    // Important to collect version at the same time as content so that staleness logic is correct.
    let version = buffer_ref.version();
    let content = buffer_ref.text_for_range(range.clone()).collect::<Rope>();
    let line_range = range.to_point(&buffer_ref.snapshot());

    // Build the text on a background thread.
    let task = cx.background_spawn({
        let line_range = line_range.clone();
        async move {
            let text = to_fenced_codeblock(&full_path, content, Some(line_range));
            ContextBuffer {
                id,
                buffer,
                last_full_path: full_path.into(),
                version,
                text,
            }
        }
    });

    Ok((line_range, task))
}
*/
