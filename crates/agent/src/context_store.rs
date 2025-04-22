use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use collections::{BTreeMap, HashMap, HashSet};
use futures::future::join_all;
use futures::{self, Future, FutureExt, future};
use gpui::{App, AppContext as _, Context, Entity, Image, SharedString, Task, WeakEntity};
use language::Buffer;
use language_model::LanguageModelImage;
use project::{Project, ProjectEntryId, ProjectItem, ProjectPath, Worktree};
use prompt_store::UserPromptId;
use rope::{Point, Rope};
use text::{Anchor, BufferId, OffsetRangeExt};
use util::{ResultExt as _, maybe};

use crate::ThreadStore;
use crate::context::{
    AssistantContext, ContextBuffer, ContextId, ContextSymbol, ContextSymbolId, DirectoryContext,
    ExcerptContext, FetchedUrlContext, FileContext, ImageContext, RulesContext, SymbolContext,
    ThreadContext,
};
use crate::context_strip::SuggestedContext;
use crate::thread::{Thread, ThreadId};

pub struct ContextStore {
    project: WeakEntity<Project>,
    context: Vec<AssistantContext>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    next_context_id: ContextId,
    files: BTreeMap<BufferId, ContextId>,
    directories: HashMap<ProjectPath, ContextId>,
    symbols: HashMap<ContextSymbolId, ContextId>,
    symbol_buffers: HashMap<ContextSymbolId, Entity<Buffer>>,
    symbols_by_path: HashMap<ProjectPath, Vec<ContextSymbolId>>,
    threads: HashMap<ThreadId, ContextId>,
    thread_summary_tasks: Vec<Task<()>>,
    fetched_urls: HashMap<String, ContextId>,
    user_rules: HashMap<UserPromptId, ContextId>,
}

impl ContextStore {
    pub fn new(
        project: WeakEntity<Project>,
        thread_store: Option<WeakEntity<ThreadStore>>,
    ) -> Self {
        Self {
            project,
            thread_store,
            context: Vec::new(),
            next_context_id: ContextId(0),
            files: BTreeMap::default(),
            directories: HashMap::default(),
            symbols: HashMap::default(),
            symbol_buffers: HashMap::default(),
            symbols_by_path: HashMap::default(),
            threads: HashMap::default(),
            thread_summary_tasks: Vec::new(),
            fetched_urls: HashMap::default(),
            user_rules: HashMap::default(),
        }
    }

    pub fn context(&self) -> &Vec<AssistantContext> {
        &self.context
    }

    pub fn context_for_id(&self, id: ContextId) -> Option<&AssistantContext> {
        self.context().iter().find(|context| context.id() == id)
    }

    pub fn clear(&mut self) {
        self.context.clear();
        self.files.clear();
        self.directories.clear();
        self.threads.clear();
        self.fetched_urls.clear();
        self.user_rules.clear();
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
            let buffer_id = this.update(cx, |_, cx| buffer.read(cx).remote_id())?;

            let already_included = this.update(cx, |this, cx| {
                match this.will_include_buffer(buffer_id, &project_path) {
                    Some(FileInclusion::Direct(context_id)) => {
                        if remove_if_exists {
                            this.remove_context(context_id, cx);
                        }
                        true
                    }
                    Some(FileInclusion::InDirectory(_)) => true,
                    None => false,
                }
            })?;

            if already_included {
                return anyhow::Ok(());
            }

            let context_buffer = this
                .update(cx, |_, cx| load_context_buffer(buffer, cx))??
                .await;

            this.update(cx, |this, cx| {
                this.insert_file(context_buffer, cx);
            })?;

            anyhow::Ok(())
        })
    }

    pub fn add_file_from_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        cx.spawn(async move |this, cx| {
            let context_buffer = this
                .update(cx, |_, cx| load_context_buffer(buffer, cx))??
                .await;

            this.update(cx, |this, cx| this.insert_file(context_buffer, cx))?;

            anyhow::Ok(())
        })
    }

    fn insert_file(&mut self, context_buffer: ContextBuffer, cx: &mut Context<Self>) {
        let id = self.next_context_id.post_inc();
        self.files.insert(context_buffer.id, id);
        self.context
            .push(AssistantContext::File(FileContext { id, context_buffer }));
        cx.notify();
    }

    pub fn add_directory(
        &mut self,
        project_path: ProjectPath,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(Err(anyhow!("failed to read project")));
        };

        let Some(entry_id) = project
            .read(cx)
            .entry_for_path(&project_path, cx)
            .map(|entry| entry.id)
        else {
            return Task::ready(Err(anyhow!("no entry found for directory context")));
        };

        let already_included = match self.includes_directory(&project_path) {
            Some(FileInclusion::Direct(context_id)) => {
                if remove_if_exists {
                    self.remove_context(context_id, cx);
                }
                true
            }
            Some(FileInclusion::InDirectory(_)) => true,
            None => false,
        };
        if already_included {
            return Task::ready(Ok(()));
        }

        let worktree_id = project_path.worktree_id;
        cx.spawn(async move |this, cx| {
            let worktree = project.update(cx, |project, cx| {
                project
                    .worktree_for_id(worktree_id, cx)
                    .ok_or_else(|| anyhow!("no worktree found for {worktree_id:?}"))
            })??;

            let files = worktree.update(cx, |worktree, _cx| {
                collect_files_in_path(worktree, &project_path.path)
            })?;

            let open_buffers_task = project.update(cx, |project, cx| {
                let tasks = files.iter().map(|file_path| {
                    project.open_buffer(
                        ProjectPath {
                            worktree_id,
                            path: file_path.clone(),
                        },
                        cx,
                    )
                });
                future::join_all(tasks)
            })?;

            let buffers = open_buffers_task.await;

            let context_buffer_tasks = this.update(cx, |_, cx| {
                buffers
                    .into_iter()
                    .flatten()
                    .flat_map(move |buffer| load_context_buffer(buffer, cx).log_err())
                    .collect::<Vec<_>>()
            })?;

            let context_buffers = future::join_all(context_buffer_tasks).await;

            if context_buffers.is_empty() {
                let full_path = cx.update(|cx| worktree.read(cx).full_path(&project_path.path))?;
                return Err(anyhow!("No text files found in {}", &full_path.display()));
            }

            this.update(cx, |this, cx| {
                this.insert_directory(worktree, entry_id, project_path, context_buffers, cx);
            })?;

            anyhow::Ok(())
        })
    }

    fn insert_directory(
        &mut self,
        worktree: Entity<Worktree>,
        entry_id: ProjectEntryId,
        project_path: ProjectPath,
        context_buffers: Vec<ContextBuffer>,
        cx: &mut Context<Self>,
    ) {
        let id = self.next_context_id.post_inc();
        let last_path = project_path.path.clone();
        self.directories.insert(project_path, id);

        self.context
            .push(AssistantContext::Directory(DirectoryContext {
                id,
                worktree,
                entry_id,
                last_path,
                context_buffers,
            }));
        cx.notify();
    }

    pub fn add_symbol(
        &mut self,
        buffer: Entity<Buffer>,
        symbol_name: SharedString,
        symbol_range: Range<Anchor>,
        symbol_enclosing_range: Range<Anchor>,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<bool>> {
        let buffer_ref = buffer.read(cx);
        let Some(project_path) = buffer_ref.project_path(cx) else {
            return Task::ready(Err(anyhow!("buffer has no path")));
        };

        if let Some(symbols_for_path) = self.symbols_by_path.get(&project_path) {
            let mut matching_symbol_id = None;
            for symbol in symbols_for_path {
                if &symbol.name == &symbol_name {
                    let snapshot = buffer_ref.snapshot();
                    if symbol.range.to_offset(&snapshot) == symbol_range.to_offset(&snapshot) {
                        matching_symbol_id = self.symbols.get(symbol).cloned();
                        break;
                    }
                }
            }

            if let Some(id) = matching_symbol_id {
                if remove_if_exists {
                    self.remove_context(id, cx);
                }
                return Task::ready(Ok(false));
            }
        }

        let context_buffer_task =
            match load_context_buffer_range(buffer, symbol_enclosing_range.clone(), cx) {
                Ok((_line_range, context_buffer_task)) => context_buffer_task,
                Err(err) => return Task::ready(Err(err)),
            };

        cx.spawn(async move |this, cx| {
            let context_buffer = context_buffer_task.await;

            this.update(cx, |this, cx| {
                this.insert_symbol(
                    make_context_symbol(
                        context_buffer,
                        project_path,
                        symbol_name,
                        symbol_range,
                        symbol_enclosing_range,
                    ),
                    cx,
                )
            })?;
            anyhow::Ok(true)
        })
    }

    fn insert_symbol(&mut self, context_symbol: ContextSymbol, cx: &mut Context<Self>) {
        let id = self.next_context_id.post_inc();
        self.symbols.insert(context_symbol.id.clone(), id);
        self.symbols_by_path
            .entry(context_symbol.id.path.clone())
            .or_insert_with(Vec::new)
            .push(context_symbol.id.clone());
        self.symbol_buffers
            .insert(context_symbol.id.clone(), context_symbol.buffer.clone());
        self.context.push(AssistantContext::Symbol(SymbolContext {
            id,
            context_symbol,
        }));
        cx.notify();
    }

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

    pub fn add_excerpt(
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
                this.insert_excerpt(context_buffer, range, line_range, cx)
            })?;

            anyhow::Ok(())
        })
    }

    fn insert_excerpt(
        &mut self,
        context_buffer: ContextBuffer,
        range: Range<Anchor>,
        line_range: Range<Point>,
        cx: &mut Context<Self>,
    ) {
        let id = self.next_context_id.post_inc();
        self.context.push(AssistantContext::Excerpt(ExcerptContext {
            id,
            range,
            line_range,
            context_buffer,
        }));
        cx.notify();
    }

    pub fn accept_suggested_context(
        &mut self,
        suggested: &SuggestedContext,
        cx: &mut Context<ContextStore>,
    ) -> Task<Result<()>> {
        match suggested {
            SuggestedContext::File {
                buffer,
                icon_path: _,
                name: _,
            } => {
                if let Some(buffer) = buffer.upgrade() {
                    return self.add_file_from_buffer(buffer, cx);
                };
            }
            SuggestedContext::Thread { thread, name: _ } => {
                if let Some(thread) = thread.upgrade() {
                    self.insert_thread(thread, cx);
                };
            }
        }
        Task::ready(Ok(()))
    }

    pub fn remove_context(&mut self, id: ContextId, cx: &mut Context<Self>) {
        let Some(ix) = self.context.iter().position(|context| context.id() == id) else {
            return;
        };

        match self.context.remove(ix) {
            AssistantContext::File(_) => {
                self.files.retain(|_, context_id| *context_id != id);
            }
            AssistantContext::Directory(_) => {
                self.directories.retain(|_, context_id| *context_id != id);
            }
            AssistantContext::Symbol(symbol) => {
                if let Some(symbols_in_path) =
                    self.symbols_by_path.get_mut(&symbol.context_symbol.id.path)
                {
                    symbols_in_path.retain(|s| {
                        self.symbols
                            .get(s)
                            .map_or(false, |context_id| *context_id != id)
                    });
                }
                self.symbol_buffers.remove(&symbol.context_symbol.id);
                self.symbols.retain(|_, context_id| *context_id != id);
            }
            AssistantContext::Excerpt(_) => {}
            AssistantContext::FetchedUrl(_) => {
                self.fetched_urls.retain(|_, context_id| *context_id != id);
            }
            AssistantContext::Thread(_) => {
                self.threads.retain(|_, context_id| *context_id != id);
            }
            AssistantContext::Rules(RulesContext { prompt_id, .. }) => {
                self.user_rules.remove(&prompt_id);
            }
            AssistantContext::Image(_) => {}
        }

        cx.notify();
    }

    /// Returns whether the buffer is already included directly in the context, or if it will be
    /// included in the context via a directory. Directory inclusion is based on paths rather than
    /// buffer IDs as the directory will be re-scanned.
    pub fn will_include_buffer(
        &self,
        buffer_id: BufferId,
        project_path: &ProjectPath,
    ) -> Option<FileInclusion> {
        if let Some(context_id) = self.files.get(&buffer_id) {
            return Some(FileInclusion::Direct(*context_id));
        }

        self.will_include_file_path_via_directory(project_path)
    }

    /// Returns whether this file path is already included directly in the context, or if it will be
    /// included in the context via a directory.
    pub fn will_include_file_path(
        &self,
        project_path: &ProjectPath,
        cx: &App,
    ) -> Option<FileInclusion> {
        if !self.files.is_empty() {
            let found_file_context = self.context.iter().find(|context| match &context {
                AssistantContext::File(file_context) => {
                    let buffer = file_context.context_buffer.buffer.read(cx);
                    if let Some(context_path) = buffer.project_path(cx) {
                        &context_path == project_path
                    } else {
                        false
                    }
                }
                _ => false,
            });
            if let Some(context) = found_file_context {
                return Some(FileInclusion::Direct(context.id()));
            }
        }

        self.will_include_file_path_via_directory(project_path)
    }

    fn will_include_file_path_via_directory(
        &self,
        project_path: &ProjectPath,
    ) -> Option<FileInclusion> {
        if self.directories.is_empty() {
            return None;
        }

        let mut path_buf = project_path.path.to_path_buf();

        while path_buf.pop() {
            // TODO: This isn't very efficient. Consider using a better representation of the
            // directories map.
            let directory_project_path = ProjectPath {
                worktree_id: project_path.worktree_id,
                path: path_buf.clone().into(),
            };
            if let Some(_) = self.directories.get(&directory_project_path) {
                return Some(FileInclusion::InDirectory(directory_project_path));
            }
        }

        None
    }

    pub fn includes_directory(&self, project_path: &ProjectPath) -> Option<FileInclusion> {
        if let Some(context_id) = self.directories.get(project_path) {
            return Some(FileInclusion::Direct(*context_id));
        }

        self.will_include_file_path_via_directory(project_path)
    }

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
                | AssistantContext::Excerpt(_)
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
}

pub enum FileInclusion {
    Direct(ContextId),
    InDirectory(ProjectPath),
}

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

fn load_context_buffer(buffer: Entity<Buffer>, cx: &App) -> Result<Task<ContextBuffer>> {
    let buffer_ref = buffer.read(cx);
    let id = buffer_ref.remote_id();

    let file = buffer_ref.file().context("context buffer missing path")?;
    let full_path = file.full_path(cx);

    // Important to collect version at the same time as content so that staleness logic is correct.
    let version = buffer_ref.version();
    let content = buffer_ref.as_rope().clone();

    // Build the text on a background thread.
    Ok(cx.background_spawn(async move {
        let text = to_fenced_codeblock(&full_path, content, None);
        ContextBuffer {
            id,
            buffer,
            last_full_path: full_path.into(),
            version,
            text,
        }
    }))
}

fn to_fenced_codeblock(
    path: &Path,
    content: Rope,
    line_range: Option<Range<Point>>,
) -> SharedString {
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

    buffer.into()
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

pub fn refresh_context_store_text(
    context_store: Entity<ContextStore>,
    changed_buffers: &HashSet<Entity<Buffer>>,
    cx: &App,
) -> impl Future<Output = Vec<ContextId>> + use<> {
    let mut tasks = Vec::new();

    for context in &context_store.read(cx).context {
        let id = context.id();

        let task = maybe!({
            match context {
                AssistantContext::File(file_context) => {
                    // TODO: Should refresh if the path has changed, as it's in the text.
                    if changed_buffers.is_empty()
                        || changed_buffers.contains(&file_context.context_buffer.buffer)
                    {
                        let context_store = context_store.clone();
                        return refresh_file_text(context_store, file_context, cx);
                    }
                }
                AssistantContext::Directory(directory_context) => {
                    let directory_path = directory_context.project_path(cx)?;
                    let should_refresh = directory_path.path != directory_context.last_path
                        || changed_buffers.is_empty()
                        || changed_buffers.iter().any(|buffer| {
                            let Some(buffer_path) = buffer.read(cx).project_path(cx) else {
                                return false;
                            };
                            buffer_path.starts_with(&directory_path)
                        });

                    if should_refresh {
                        let context_store = context_store.clone();
                        return refresh_directory_text(
                            context_store,
                            directory_context,
                            directory_path,
                            cx,
                        );
                    }
                }
                AssistantContext::Symbol(symbol_context) => {
                    // TODO: Should refresh if the path has changed, as it's in the text.
                    if changed_buffers.is_empty()
                        || changed_buffers.contains(&symbol_context.context_symbol.buffer)
                    {
                        let context_store = context_store.clone();
                        return refresh_symbol_text(context_store, symbol_context, cx);
                    }
                }
                AssistantContext::Excerpt(excerpt_context) => {
                    // TODO: Should refresh if the path has changed, as it's in the text.
                    if changed_buffers.is_empty()
                        || changed_buffers.contains(&excerpt_context.context_buffer.buffer)
                    {
                        let context_store = context_store.clone();
                        return refresh_excerpt_text(context_store, excerpt_context, cx);
                    }
                }
                AssistantContext::Thread(thread_context) => {
                    if changed_buffers.is_empty() {
                        let context_store = context_store.clone();
                        return Some(refresh_thread_text(context_store, thread_context, cx));
                    }
                }
                // Intentionally omit refreshing fetched URLs as it doesn't seem all that useful,
                // and doing the caching properly could be tricky (unless it's already handled by
                // the HttpClient?).
                AssistantContext::FetchedUrl(_) => {}
                AssistantContext::Rules(user_rules_context) => {
                    let context_store = context_store.clone();
                    return Some(refresh_user_rules(context_store, user_rules_context, cx));
                }
                AssistantContext::Image(_) => {}
            }

            None
        });

        if let Some(task) = task {
            tasks.push(task.map(move |_| id));
        }
    }

    future::join_all(tasks)
}

fn refresh_file_text(
    context_store: Entity<ContextStore>,
    file_context: &FileContext,
    cx: &App,
) -> Option<Task<()>> {
    let id = file_context.id;
    let task = refresh_context_buffer(&file_context.context_buffer, cx);
    if let Some(task) = task {
        Some(cx.spawn(async move |cx| {
            let context_buffer = task.await;
            context_store
                .update(cx, |context_store, _| {
                    let new_file_context = FileContext { id, context_buffer };
                    context_store.replace_context(AssistantContext::File(new_file_context));
                })
                .ok();
        }))
    } else {
        None
    }
}

fn refresh_directory_text(
    context_store: Entity<ContextStore>,
    directory_context: &DirectoryContext,
    directory_path: ProjectPath,
    cx: &App,
) -> Option<Task<()>> {
    let mut stale = false;
    let futures = directory_context
        .context_buffers
        .iter()
        .map(|context_buffer| {
            if let Some(refresh_task) = refresh_context_buffer(context_buffer, cx) {
                stale = true;
                future::Either::Left(refresh_task)
            } else {
                future::Either::Right(future::ready((*context_buffer).clone()))
            }
        })
        .collect::<Vec<_>>();

    if !stale {
        return None;
    }

    let context_buffers = future::join_all(futures);

    let id = directory_context.id;
    let worktree = directory_context.worktree.clone();
    let entry_id = directory_context.entry_id;
    let last_path = directory_path.path;
    Some(cx.spawn(async move |cx| {
        let context_buffers = context_buffers.await;
        context_store
            .update(cx, |context_store, _| {
                let new_directory_context = DirectoryContext {
                    id,
                    worktree,
                    entry_id,
                    last_path,
                    context_buffers,
                };
                context_store.replace_context(AssistantContext::Directory(new_directory_context));
            })
            .ok();
    }))
}

fn refresh_symbol_text(
    context_store: Entity<ContextStore>,
    symbol_context: &SymbolContext,
    cx: &App,
) -> Option<Task<()>> {
    let id = symbol_context.id;
    let task = refresh_context_symbol(&symbol_context.context_symbol, cx);
    if let Some(task) = task {
        Some(cx.spawn(async move |cx| {
            let context_symbol = task.await;
            context_store
                .update(cx, |context_store, _| {
                    let new_symbol_context = SymbolContext { id, context_symbol };
                    context_store.replace_context(AssistantContext::Symbol(new_symbol_context));
                })
                .ok();
        }))
    } else {
        None
    }
}

fn refresh_excerpt_text(
    context_store: Entity<ContextStore>,
    excerpt_context: &ExcerptContext,
    cx: &App,
) -> Option<Task<()>> {
    let id = excerpt_context.id;
    let range = excerpt_context.range.clone();
    let task = refresh_context_excerpt(&excerpt_context.context_buffer, range.clone(), cx);
    if let Some(task) = task {
        Some(cx.spawn(async move |cx| {
            let (line_range, context_buffer) = task.await;
            context_store
                .update(cx, |context_store, _| {
                    let new_excerpt_context = ExcerptContext {
                        id,
                        range,
                        line_range,
                        context_buffer,
                    };
                    context_store.replace_context(AssistantContext::Excerpt(new_excerpt_context));
                })
                .ok();
        }))
    } else {
        None
    }
}

fn refresh_thread_text(
    context_store: Entity<ContextStore>,
    thread_context: &ThreadContext,
    cx: &App,
) -> Task<()> {
    let id = thread_context.id;
    let thread = thread_context.thread.clone();
    cx.spawn(async move |cx| {
        context_store
            .update(cx, |context_store, cx| {
                let text = thread.read(cx).latest_detailed_summary_or_text();
                context_store.replace_context(AssistantContext::Thread(ThreadContext {
                    id,
                    thread,
                    text,
                }));
            })
            .ok();
    })
}

fn refresh_user_rules(
    context_store: Entity<ContextStore>,
    user_rules_context: &RulesContext,
    cx: &App,
) -> Task<()> {
    let id = user_rules_context.id;
    let prompt_id = user_rules_context.prompt_id;
    let Some(thread_store) = context_store.read(cx).thread_store.as_ref() else {
        return Task::ready(());
    };
    let Ok(load_task) = thread_store.read_with(cx, |thread_store, cx| {
        thread_store.load_rules(prompt_id, cx)
    }) else {
        return Task::ready(());
    };
    cx.spawn(async move |cx| {
        if let Ok((metadata, text)) = load_task.await {
            if let Some(title) = metadata.title.clone() {
                context_store
                    .update(cx, |context_store, _cx| {
                        context_store.replace_context(AssistantContext::Rules(RulesContext {
                            id,
                            prompt_id,
                            title,
                            text: text.into(),
                        }));
                    })
                    .ok();
                return;
            }
        }
        context_store
            .update(cx, |context_store, cx| {
                context_store.remove_context(id, cx);
            })
            .ok();
    })
}

fn refresh_context_buffer(context_buffer: &ContextBuffer, cx: &App) -> Option<Task<ContextBuffer>> {
    let buffer = context_buffer.buffer.read(cx);
    if buffer.version.changed_since(&context_buffer.version) {
        load_context_buffer(context_buffer.buffer.clone(), cx).log_err()
    } else {
        None
    }
}

fn refresh_context_excerpt(
    context_buffer: &ContextBuffer,
    range: Range<Anchor>,
    cx: &App,
) -> Option<impl Future<Output = (Range<Point>, ContextBuffer)> + use<>> {
    let buffer = context_buffer.buffer.read(cx);
    if buffer.version.changed_since(&context_buffer.version) {
        let (line_range, context_buffer_task) =
            load_context_buffer_range(context_buffer.buffer.clone(), range, cx).log_err()?;
        Some(context_buffer_task.map(move |context_buffer| (line_range, context_buffer)))
    } else {
        None
    }
}

fn refresh_context_symbol(
    context_symbol: &ContextSymbol,
    cx: &App,
) -> Option<impl Future<Output = ContextSymbol> + use<>> {
    let buffer = context_symbol.buffer.read(cx);
    let project_path = buffer.project_path(cx)?;
    if buffer.version.changed_since(&context_symbol.buffer_version) {
        let (_line_range, context_buffer_task) = load_context_buffer_range(
            context_symbol.buffer.clone(),
            context_symbol.enclosing_range.clone(),
            cx,
        )
        .log_err()?;
        let name = context_symbol.id.name.clone();
        let range = context_symbol.id.range.clone();
        let enclosing_range = context_symbol.enclosing_range.clone();
        Some(context_buffer_task.map(move |context_buffer| {
            make_context_symbol(context_buffer, project_path, name, range, enclosing_range)
        }))
    } else {
        None
    }
}
