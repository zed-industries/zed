use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Result, anyhow};
use assistant_context_editor::AssistantContext;
use collections::{HashSet, IndexSet};
use futures::{self, FutureExt};
use gpui::{App, Context, Entity, EventEmitter, Image, SharedString, Task, WeakEntity};
use language::Buffer;
use language_model::LanguageModelImage;
use project::image_store::is_image_file;
use project::{Project, ProjectItem, ProjectPath, Symbol};
use prompt_store::UserPromptId;
use ref_cast::RefCast as _;
use text::{Anchor, OffsetRangeExt};

use crate::ThreadStore;
use crate::context::{
    AgentContextHandle, AgentContextKey, ContextId, DirectoryContextHandle, FetchedUrlContext,
    FileContextHandle, ImageContext, RulesContextHandle, SelectionContextHandle,
    SymbolContextHandle, TextThreadContextHandle, ThreadContextHandle,
};
use crate::context_strip::SuggestedContext;
use crate::thread::{MessageId, Thread, ThreadId};

pub struct ContextStore {
    project: WeakEntity<Project>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    next_context_id: ContextId,
    context_set: IndexSet<AgentContextKey>,
    context_thread_ids: HashSet<ThreadId>,
    context_text_thread_paths: HashSet<Arc<Path>>,
}

pub enum ContextStoreEvent {
    ContextRemoved(AgentContextKey),
}

impl EventEmitter<ContextStoreEvent> for ContextStore {}

impl ContextStore {
    pub fn new(
        project: WeakEntity<Project>,
        thread_store: Option<WeakEntity<ThreadStore>>,
    ) -> Self {
        Self {
            project,
            thread_store,
            next_context_id: ContextId::zero(),
            context_set: IndexSet::default(),
            context_thread_ids: HashSet::default(),
            context_text_thread_paths: HashSet::default(),
        }
    }

    pub fn context(&self) -> impl Iterator<Item = &AgentContextHandle> {
        self.context_set.iter().map(|entry| entry.as_ref())
    }

    pub fn clear(&mut self) {
        self.context_set.clear();
        self.context_thread_ids.clear();
    }

    pub fn new_context_for_thread(
        &self,
        thread: &Thread,
        exclude_messages_from_id: Option<MessageId>,
    ) -> Vec<AgentContextHandle> {
        let existing_context = thread
            .messages()
            .take_while(|message| exclude_messages_from_id.is_none_or(|id| message.id != id))
            .flat_map(|message| {
                message
                    .loaded_context
                    .contexts
                    .iter()
                    .map(|context| AgentContextKey(context.handle()))
            })
            .collect::<HashSet<_>>();
        self.context_set
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
    ) -> Task<Result<Option<AgentContextHandle>>> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(Err(anyhow!("failed to read project")));
        };

        if is_image_file(&project, &project_path, cx) {
            self.add_image_from_path(project_path, remove_if_exists, cx)
        } else {
            cx.spawn(async move |this, cx| {
                let open_buffer_task = project.update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?;
                let buffer = open_buffer_task.await?;
                this.update(cx, |this, cx| {
                    this.add_file_from_buffer(&project_path, buffer, remove_if_exists, cx)
                })
            })
        }
    }

    pub fn add_file_from_buffer(
        &mut self,
        project_path: &ProjectPath,
        buffer: Entity<Buffer>,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> Option<AgentContextHandle> {
        let context_id = self.next_context_id.post_inc();
        let context = AgentContextHandle::File(FileContextHandle { buffer, context_id });

        if let Some(key) = self.context_set.get(AgentContextKey::ref_cast(&context)) {
            if remove_if_exists {
                self.remove_context(&context, cx);
                None
            } else {
                Some(key.as_ref().clone())
            }
        } else if self.path_included_in_directory(project_path, cx).is_some() {
            None
        } else {
            self.insert_context(context.clone(), cx);
            Some(context)
        }
    }

    pub fn add_directory(
        &mut self,
        project_path: &ProjectPath,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> Result<Option<AgentContextHandle>> {
        let Some(project) = self.project.upgrade() else {
            return Err(anyhow!("failed to read project"));
        };

        let Some(entry_id) = project
            .read(cx)
            .entry_for_path(project_path, cx)
            .map(|entry| entry.id)
        else {
            return Err(anyhow!("no entry found for directory context"));
        };

        let context_id = self.next_context_id.post_inc();
        let context = AgentContextHandle::Directory(DirectoryContextHandle {
            entry_id,
            context_id,
        });

        let context =
            if let Some(existing) = self.context_set.get(AgentContextKey::ref_cast(&context)) {
                if remove_if_exists {
                    self.remove_context(&context, cx);
                    None
                } else {
                    Some(existing.as_ref().clone())
                }
            } else {
                self.insert_context(context.clone(), cx);
                Some(context)
            };

        anyhow::Ok(context)
    }

    pub fn add_symbol(
        &mut self,
        buffer: Entity<Buffer>,
        symbol: SharedString,
        range: Range<Anchor>,
        enclosing_range: Range<Anchor>,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> (Option<AgentContextHandle>, bool) {
        let context_id = self.next_context_id.post_inc();
        let context = AgentContextHandle::Symbol(SymbolContextHandle {
            buffer,
            symbol,
            range,
            enclosing_range,
            context_id,
        });

        if let Some(key) = self.context_set.get(AgentContextKey::ref_cast(&context)) {
            let handle = if remove_if_exists {
                self.remove_context(&context, cx);
                None
            } else {
                Some(key.as_ref().clone())
            };
            return (handle, false);
        }

        let included = self.insert_context(context.clone(), cx);
        (Some(context), included)
    }

    pub fn add_thread(
        &mut self,
        thread: Entity<Thread>,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> Option<AgentContextHandle> {
        let context_id = self.next_context_id.post_inc();
        let context = AgentContextHandle::Thread(ThreadContextHandle { thread, context_id });

        if let Some(existing) = self.context_set.get(AgentContextKey::ref_cast(&context)) {
            if remove_if_exists {
                self.remove_context(&context, cx);
                None
            } else {
                Some(existing.as_ref().clone())
            }
        } else {
            self.insert_context(context.clone(), cx);
            Some(context)
        }
    }

    pub fn add_text_thread(
        &mut self,
        context: Entity<AssistantContext>,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> Option<AgentContextHandle> {
        let context_id = self.next_context_id.post_inc();
        let context = AgentContextHandle::TextThread(TextThreadContextHandle {
            context,
            context_id,
        });

        if let Some(existing) = self.context_set.get(AgentContextKey::ref_cast(&context)) {
            if remove_if_exists {
                self.remove_context(&context, cx);
                None
            } else {
                Some(existing.as_ref().clone())
            }
        } else {
            self.insert_context(context.clone(), cx);
            Some(context)
        }
    }

    pub fn add_rules(
        &mut self,
        prompt_id: UserPromptId,
        remove_if_exists: bool,
        cx: &mut Context<ContextStore>,
    ) -> Option<AgentContextHandle> {
        let context_id = self.next_context_id.post_inc();
        let context = AgentContextHandle::Rules(RulesContextHandle {
            prompt_id,
            context_id,
        });

        if let Some(existing) = self.context_set.get(AgentContextKey::ref_cast(&context)) {
            if remove_if_exists {
                self.remove_context(&context, cx);
                None
            } else {
                Some(existing.as_ref().clone())
            }
        } else {
            self.insert_context(context.clone(), cx);
            Some(context)
        }
    }

    pub fn add_fetched_url(
        &mut self,
        url: String,
        text: impl Into<SharedString>,
        cx: &mut Context<ContextStore>,
    ) -> AgentContextHandle {
        let context = AgentContextHandle::FetchedUrl(FetchedUrlContext {
            url: url.into(),
            text: text.into(),
            context_id: self.next_context_id.post_inc(),
        });

        self.insert_context(context.clone(), cx);
        context
    }

    pub fn add_image_from_path(
        &mut self,
        project_path: ProjectPath,
        remove_if_exists: bool,
        cx: &mut Context<ContextStore>,
    ) -> Task<Result<Option<AgentContextHandle>>> {
        let project = self.project.clone();
        cx.spawn(async move |this, cx| {
            let open_image_task = project.update(cx, |project, cx| {
                project.open_image(project_path.clone(), cx)
            })?;
            let image_item = open_image_task.await?;
            let image = image_item.read_with(cx, |image_item, _| image_item.image.clone())?;
            this.update(cx, |this, cx| {
                this.insert_image(
                    Some(image_item.read(cx).project_path(cx)),
                    image,
                    remove_if_exists,
                    cx,
                )
            })
        })
    }

    pub fn add_image_instance(&mut self, image: Arc<Image>, cx: &mut Context<ContextStore>) {
        self.insert_image(None, image, false, cx);
    }

    fn insert_image(
        &mut self,
        project_path: Option<ProjectPath>,
        image: Arc<Image>,
        remove_if_exists: bool,
        cx: &mut Context<ContextStore>,
    ) -> Option<AgentContextHandle> {
        let image_task = LanguageModelImage::from_image(image.clone(), cx).shared();
        let context = AgentContextHandle::Image(ImageContext {
            project_path,
            original_image: image,
            image_task,
            context_id: self.next_context_id.post_inc(),
        });
        if self.has_context(&context) {
            if remove_if_exists {
                self.remove_context(&context, cx);
                return None;
            }
        }

        self.insert_context(context.clone(), cx);
        Some(context)
    }

    pub fn add_selection(
        &mut self,
        buffer: Entity<Buffer>,
        range: Range<Anchor>,
        cx: &mut Context<ContextStore>,
    ) {
        let context_id = self.next_context_id.post_inc();
        let context = AgentContextHandle::Selection(SelectionContextHandle {
            buffer,
            range,
            context_id,
        });
        self.insert_context(context, cx);
    }

    pub fn add_suggested_context(
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
                    let context_id = self.next_context_id.post_inc();
                    self.insert_context(
                        AgentContextHandle::File(FileContextHandle { buffer, context_id }),
                        cx,
                    );
                };
            }
            SuggestedContext::Thread { thread, name: _ } => {
                if let Some(thread) = thread.upgrade() {
                    let context_id = self.next_context_id.post_inc();
                    self.insert_context(
                        AgentContextHandle::Thread(ThreadContextHandle { thread, context_id }),
                        cx,
                    );
                }
            }
            SuggestedContext::TextThread { context, name: _ } => {
                if let Some(context) = context.upgrade() {
                    let context_id = self.next_context_id.post_inc();
                    self.insert_context(
                        AgentContextHandle::TextThread(TextThreadContextHandle {
                            context,
                            context_id,
                        }),
                        cx,
                    );
                }
            }
        }
    }

    fn insert_context(&mut self, context: AgentContextHandle, cx: &mut Context<Self>) -> bool {
        match &context {
            AgentContextHandle::Thread(thread_context) => {
                if let Some(thread_store) = self.thread_store.clone() {
                    thread_context.thread.update(cx, |thread, cx| {
                        thread.start_generating_detailed_summary_if_needed(thread_store, cx);
                    });
                    self.context_thread_ids
                        .insert(thread_context.thread.read(cx).id().clone());
                } else {
                    return false;
                }
            }
            AgentContextHandle::TextThread(text_thread_context) => {
                self.context_text_thread_paths
                    .extend(text_thread_context.context.read(cx).path().cloned());
            }
            _ => {}
        }
        let inserted = self.context_set.insert(AgentContextKey(context));
        if inserted {
            cx.notify();
        }
        inserted
    }

    pub fn remove_context(&mut self, context: &AgentContextHandle, cx: &mut Context<Self>) {
        if let Some((_, key)) = self
            .context_set
            .shift_remove_full(AgentContextKey::ref_cast(context))
        {
            match context {
                AgentContextHandle::Thread(thread_context) => {
                    self.context_thread_ids
                        .remove(thread_context.thread.read(cx).id());
                }
                AgentContextHandle::TextThread(text_thread_context) => {
                    if let Some(path) = text_thread_context.context.read(cx).path() {
                        self.context_text_thread_paths.remove(path);
                    }
                }
                _ => {}
            }
            cx.emit(ContextStoreEvent::ContextRemoved(key));
            cx.notify();
        }
    }

    pub fn has_context(&mut self, context: &AgentContextHandle) -> bool {
        self.context_set
            .contains(AgentContextKey::ref_cast(context))
    }

    /// Returns whether this file path is already included directly in the context, or if it will be
    /// included in the context via a directory.
    pub fn file_path_included(&self, path: &ProjectPath, cx: &App) -> Option<FileInclusion> {
        let project = self.project.upgrade()?.read(cx);
        self.context().find_map(|context| match context {
            AgentContextHandle::File(file_context) => {
                FileInclusion::check_file(file_context, path, cx)
            }
            AgentContextHandle::Image(image_context) => {
                FileInclusion::check_image(image_context, path)
            }
            AgentContextHandle::Directory(directory_context) => {
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
            AgentContextHandle::Directory(directory_context) => {
                FileInclusion::check_directory(directory_context, path, project, cx)
            }
            _ => None,
        })
    }

    pub fn includes_symbol(&self, symbol: &Symbol, cx: &App) -> bool {
        self.context().any(|context| match context {
            AgentContextHandle::Symbol(context) => {
                if context.symbol != symbol.name {
                    return false;
                }
                let buffer = context.buffer.read(cx);
                let Some(context_path) = buffer.project_path(cx) else {
                    return false;
                };
                if context_path != symbol.path {
                    return false;
                }
                let context_range = context.range.to_point_utf16(&buffer.snapshot());
                context_range.start == symbol.range.start.0
                    && context_range.end == symbol.range.end.0
            }
            _ => false,
        })
    }

    pub fn includes_thread(&self, thread_id: &ThreadId) -> bool {
        self.context_thread_ids.contains(thread_id)
    }

    pub fn includes_text_thread(&self, path: &Arc<Path>) -> bool {
        self.context_text_thread_paths.contains(path)
    }

    pub fn includes_user_rules(&self, prompt_id: UserPromptId) -> bool {
        self.context_set
            .contains(&RulesContextHandle::lookup_key(prompt_id))
    }

    pub fn includes_url(&self, url: impl Into<SharedString>) -> bool {
        self.context_set
            .contains(&FetchedUrlContext::lookup_key(url.into()))
    }

    pub fn get_url_context(&self, url: SharedString) -> Option<AgentContextHandle> {
        self.context_set
            .get(&FetchedUrlContext::lookup_key(url))
            .map(|key| key.as_ref().clone())
    }

    pub fn file_paths(&self, cx: &App) -> HashSet<ProjectPath> {
        self.context()
            .filter_map(|context| match context {
                AgentContextHandle::File(file) => {
                    let buffer = file.buffer.read(cx);
                    buffer.project_path(cx)
                }
                AgentContextHandle::Directory(_)
                | AgentContextHandle::Symbol(_)
                | AgentContextHandle::Selection(_)
                | AgentContextHandle::FetchedUrl(_)
                | AgentContextHandle::Thread(_)
                | AgentContextHandle::TextThread(_)
                | AgentContextHandle::Rules(_)
                | AgentContextHandle::Image(_) => None,
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
    fn check_file(file_context: &FileContextHandle, path: &ProjectPath, cx: &App) -> Option<Self> {
        let file_path = file_context.buffer.read(cx).project_path(cx)?;
        if path == &file_path {
            Some(FileInclusion::Direct)
        } else {
            None
        }
    }

    fn check_image(image_context: &ImageContext, path: &ProjectPath) -> Option<Self> {
        let image_path = image_context.project_path.as_ref()?;
        if path == image_path {
            Some(FileInclusion::Direct)
        } else {
            None
        }
    }

    fn check_directory(
        directory_context: &DirectoryContextHandle,
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
