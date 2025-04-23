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
use crate::context::{
    AssistantContext, ContextElementId, DirectoryContext, FetchedUrlContext, FileContext,
    RulesContext, SelectionContext, SymbolContext, ThreadContext,
};
use crate::context_strip::SuggestedContext;
use crate::thread::{Thread, ThreadId};

pub struct ContextStore {
    project: WeakEntity<Project>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    thread_summary_tasks: Vec<Task<()>>,
    next_context_element_id: ContextElementId,
    // todo! rename to context_set?
    context: IndexSet<ContextSetEntry>,
    context_thread_ids: HashSet<ThreadId>,
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
            next_context_element_id: ContextElementId::zero(),
            context: IndexSet::default(),
            context_thread_ids: HashSet::default(),
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
        self.context_thread_ids.clear();
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

        let element_id = self.next_context_element_id.post_inc();

        cx.spawn(async move |this, cx| {
            let open_buffer_task = project.update(cx, |project, cx| {
                project.open_buffer(project_path.clone(), cx)
            })?;

            let buffer = open_buffer_task.await?;
            let context = AssistantContext::File(FileContext { buffer, element_id });

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

            if !already_included {
                this.update(cx, |this, cx| {
                    this.insert_context(context, cx);
                })?;
            }

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

        let element_id = self.next_context_element_id.post_inc();
        let context = AssistantContext::Directory(DirectoryContext {
            entry_id,
            element_id,
        });

        if self.has_context(&context) {
            if remove_if_exists {
                self.remove_context(&context, cx);
            }
        } else if !self.path_included_in_directory(&project_path, cx).is_some() {
            self.insert_context(context, cx);
        }

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
        let element_id = self.next_context_element_id.post_inc();
        let context = AssistantContext::Symbol(SymbolContext {
            buffer,
            symbol,
            range,
            enclosing_range,
            element_id,
        });

        if self.has_context(&context) {
            if remove_if_exists {
                self.remove_context(&context, cx);
            }
            return false;
        }

        self.insert_context(context, cx)
    }

    pub fn add_thread(
        &mut self,
        thread: Entity<Thread>,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) {
        let element_id = self.next_context_element_id.post_inc();
        let context = AssistantContext::Thread(ThreadContext { thread, element_id });

        if self.has_context(&context) {
            if remove_if_exists {
                self.remove_context(&context, cx);
            }
        } else {
            self.insert_context(context, cx);
            // todo! handle summaries tasks
        }
    }

    /*
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
    */

    pub fn add_rules(
        &mut self,
        prompt_id: UserPromptId,
        remove_if_exists: bool,
        cx: &mut Context<ContextStore>,
    ) {
        let element_id = self.next_context_element_id.post_inc();
        let context = AssistantContext::Rules(RulesContext {
            prompt_id,
            element_id,
        });

        if self.has_context(&context) {
            if remove_if_exists {
                self.remove_context(&context, cx);
            }
        } else {
            self.insert_context(context, cx);
        }
    }

    pub fn add_fetched_url(
        &mut self,
        url: String,
        text: impl Into<SharedString>,
        cx: &mut Context<ContextStore>,
    ) {
        let context = AssistantContext::FetchedUrl(FetchedUrlContext {
            url: url.into(),
            text: text.into(),
            element_id: self.next_context_element_id.post_inc(),
        });

        self.insert_context(context, cx);
    }

    /*
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
    */

    pub fn add_selection(
        &mut self,
        buffer: Entity<Buffer>,
        range: Range<Anchor>,
        cx: &mut Context<ContextStore>,
    ) {
        let element_id = self.next_context_element_id.post_inc();
        let context = AssistantContext::Selection(SelectionContext {
            buffer,
            range,
            element_id,
        });
        self.insert_context(context, cx);
    }

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
                    let element_id = self.next_context_element_id.post_inc();
                    self.insert_context(
                        AssistantContext::File(FileContext { buffer, element_id }),
                        cx,
                    );
                };
            }
            SuggestedContext::Thread { thread, name: _ } => {
                if let Some(thread) = thread.upgrade() {
                    let element_id = self.next_context_element_id.post_inc();
                    self.insert_context(
                        AssistantContext::Thread(ThreadContext { thread, element_id }),
                        cx,
                    );
                }
            }
        }
    }

    fn insert_context(&mut self, context: AssistantContext, cx: &mut Context<Self>) -> bool {
        match &context {
            AssistantContext::Thread(thread_context) => {
                self.context_thread_ids
                    .insert(thread_context.thread.read(cx).id().clone());
            }
            _ => {}
        }
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
            match context {
                AssistantContext::Thread(thread_context) => {
                    self.context_thread_ids
                        .remove(thread_context.thread.read(cx).id());
                }
                _ => {}
            }
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
    */

    pub fn includes_thread(&self, thread_id: &ThreadId) -> bool {
        self.context_thread_ids.contains(thread_id)
    }

    pub fn includes_user_rules(&self, prompt_id: UserPromptId) -> bool {
        let context_query = AssistantContext::Rules(RulesContext {
            prompt_id,
            element_id: ContextElementId::for_query(),
        });
        self.context
            .contains(ContextSetEntry::ref_cast(&context_query))
    }

    pub fn includes_url(&self, url: impl Into<SharedString>) -> bool {
        let context_query = AssistantContext::FetchedUrl(FetchedUrlContext {
            url: url.into(),
            text: "".into(),
            element_id: ContextElementId::for_query(),
        });
        self.context
            .contains(ContextSetEntry::ref_cast(&context_query))
    }

    /*
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
    */

    pub fn file_paths(&self, cx: &App) -> HashSet<ProjectPath> {
        self.context()
            .filter_map(|context| {
                match context {
                    AssistantContext::File(file) => {
                        let buffer = file.buffer.read(cx);
                        buffer.project_path(cx)
                    }
                    AssistantContext::Directory(_)
                    | AssistantContext::Symbol(_)
                    | AssistantContext::Selection(_)
                    | AssistantContext::FetchedUrl(_)
                    | AssistantContext::Thread(_)
                    | AssistantContext::Rules(_) => None,
                    // | AssistantContext::Image(_) => None,
                }
            })
            .collect()
    }

    pub fn thread_ids(&self) -> &HashSet<ThreadId> {
        &self.context_thread_ids
    }
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
        }
    }
}
