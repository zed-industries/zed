use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use collections::{BTreeMap, HashMap, HashSet};
use futures::{self, Future, FutureExt, future};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, SharedString, Task, WeakEntity};
use language::{Buffer, File};
use project::{ProjectItem, ProjectPath, Worktree};
use rope::Rope;
use text::{Anchor, BufferId, OffsetRangeExt};
use util::{ResultExt, maybe};
use workspace::Workspace;

use crate::context::{
    AssistantContext, ContextBuffer, ContextId, ContextSymbol, ContextSymbolId, DirectoryContext,
    FetchedUrlContext, FileContext, SymbolContext, ThreadContext,
};
use crate::context_strip::SuggestedContext;
use crate::thread::{Thread, ThreadId};

pub struct ContextStore {
    workspace: WeakEntity<Workspace>,
    context: Vec<AssistantContext>,
    // TODO: If an EntityId is used for all context types (like BufferId), can remove ContextId.
    next_context_id: ContextId,
    files: BTreeMap<BufferId, ContextId>,
    directories: HashMap<PathBuf, ContextId>,
    symbols: HashMap<ContextSymbolId, ContextId>,
    symbol_buffers: HashMap<ContextSymbolId, Entity<Buffer>>,
    symbols_by_path: HashMap<ProjectPath, Vec<ContextSymbolId>>,
    threads: HashMap<ThreadId, ContextId>,
    fetched_urls: HashMap<String, ContextId>,
}

impl ContextStore {
    pub fn new(workspace: WeakEntity<Workspace>) -> Self {
        Self {
            workspace,
            context: Vec::new(),
            next_context_id: ContextId(0),
            files: BTreeMap::default(),
            directories: HashMap::default(),
            symbols: HashMap::default(),
            symbol_buffers: HashMap::default(),
            symbols_by_path: HashMap::default(),
            threads: HashMap::default(),
            fetched_urls: HashMap::default(),
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
    }

    pub fn add_file_from_path(
        &mut self,
        project_path: ProjectPath,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let workspace = self.workspace.clone();

        let Some(project) = workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).project().clone())
        else {
            return Task::ready(Err(anyhow!("failed to read project")));
        };

        cx.spawn(async move |this, cx| {
            let open_buffer_task = project.update(cx, |project, cx| {
                project.open_buffer(project_path.clone(), cx)
            })?;

            let buffer_entity = open_buffer_task.await?;
            let buffer_id = this.update(cx, |_, cx| buffer_entity.read(cx).remote_id())?;

            let already_included = this.update(cx, |this, _cx| {
                match this.will_include_buffer(buffer_id, &project_path.path) {
                    Some(FileInclusion::Direct(context_id)) => {
                        if remove_if_exists {
                            this.remove_context(context_id);
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

            let (buffer_info, text_task) = this.update(cx, |_, cx| {
                let buffer = buffer_entity.read(cx);
                collect_buffer_info_and_text(
                    project_path.path.clone(),
                    buffer_entity,
                    buffer,
                    None,
                    cx.to_async(),
                )
            })??;

            let text = text_task.await;

            this.update(cx, |this, _cx| {
                this.insert_file(make_context_buffer(buffer_info, text));
            })?;

            anyhow::Ok(())
        })
    }

    pub fn add_file_from_buffer(
        &mut self,
        buffer_entity: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        cx.spawn(async move |this, cx| {
            let (buffer_info, text_task) = this.update(cx, |_, cx| {
                let buffer = buffer_entity.read(cx);
                let Some(file) = buffer.file() else {
                    return Err(anyhow!("Buffer has no path."));
                };
                collect_buffer_info_and_text(
                    file.path().clone(),
                    buffer_entity,
                    buffer,
                    None,
                    cx.to_async(),
                )
            })??;

            let text = text_task.await;

            this.update(cx, |this, _cx| {
                this.insert_file(make_context_buffer(buffer_info, text))
            })?;

            anyhow::Ok(())
        })
    }

    fn insert_file(&mut self, context_buffer: ContextBuffer) {
        let id = self.next_context_id.post_inc();
        self.files.insert(context_buffer.id, id);
        self.context.push(AssistantContext::File(FileContext {
            id,
            context_buffer: context_buffer,
        }));
    }

    pub fn add_directory(
        &mut self,
        project_path: ProjectPath,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let workspace = self.workspace.clone();
        let Some(project) = workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).project().clone())
        else {
            return Task::ready(Err(anyhow!("failed to read project")));
        };

        let already_included = match self.includes_directory(&project_path.path) {
            Some(FileInclusion::Direct(context_id)) => {
                if remove_if_exists {
                    self.remove_context(context_id);
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

            let mut buffer_infos = Vec::new();
            let mut text_tasks = Vec::new();
            this.update(cx, |_, cx| {
                for (path, buffer_entity) in files.into_iter().zip(buffers) {
                    // Skip all binary files and other non-UTF8 files
                    if let Ok(buffer_entity) = buffer_entity {
                        let buffer = buffer_entity.read(cx);
                        if let Some((buffer_info, text_task)) = collect_buffer_info_and_text(
                            path,
                            buffer_entity,
                            buffer,
                            None,
                            cx.to_async(),
                        )
                        .log_err()
                        {
                            buffer_infos.push(buffer_info);
                            text_tasks.push(text_task);
                        }
                    }
                }
                anyhow::Ok(())
            })??;

            let buffer_texts = future::join_all(text_tasks).await;
            let context_buffers = buffer_infos
                .into_iter()
                .zip(buffer_texts)
                .map(|(info, text)| make_context_buffer(info, text))
                .collect::<Vec<_>>();

            if context_buffers.is_empty() {
                return Err(anyhow!(
                    "No text files found in {}",
                    &project_path.path.display()
                ));
            }

            this.update(cx, |this, _| {
                this.insert_directory(project_path, context_buffers);
            })?;

            anyhow::Ok(())
        })
    }

    fn insert_directory(&mut self, project_path: ProjectPath, context_buffers: Vec<ContextBuffer>) {
        let id = self.next_context_id.post_inc();
        self.directories.insert(project_path.path.to_path_buf(), id);

        self.context
            .push(AssistantContext::Directory(DirectoryContext {
                id,
                project_path,
                context_buffers,
            }));
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
        let Some(file) = buffer_ref.file() else {
            return Task::ready(Err(anyhow!("Buffer has no path.")));
        };

        let Some(project_path) = buffer_ref.project_path(cx) else {
            return Task::ready(Err(anyhow!("Buffer has no project path.")));
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
                    self.remove_context(id);
                }
                return Task::ready(Ok(false));
            }
        }

        let (buffer_info, collect_content_task) = match collect_buffer_info_and_text(
            file.path().clone(),
            buffer,
            buffer_ref,
            Some(symbol_enclosing_range.clone()),
            cx.to_async(),
        ) {
            Ok((buffer_info, collect_context_task)) => (buffer_info, collect_context_task),
            Err(err) => return Task::ready(Err(err)),
        };

        cx.spawn(async move |this, cx| {
            let content = collect_content_task.await;

            this.update(cx, |this, _cx| {
                this.insert_symbol(make_context_symbol(
                    buffer_info,
                    project_path,
                    symbol_name,
                    symbol_range,
                    symbol_enclosing_range,
                    content,
                ))
            })?;
            anyhow::Ok(true)
        })
    }

    fn insert_symbol(&mut self, context_symbol: ContextSymbol) {
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
    }

    pub fn add_thread(
        &mut self,
        thread: Entity<Thread>,
        remove_if_exists: bool,
        cx: &mut Context<Self>,
    ) {
        if let Some(context_id) = self.includes_thread(&thread.read(cx).id()) {
            if remove_if_exists {
                self.remove_context(context_id);
            }
        } else {
            self.insert_thread(thread, cx);
        }
    }

    fn insert_thread(&mut self, thread: Entity<Thread>, cx: &App) {
        let id = self.next_context_id.post_inc();
        let text = thread.read(cx).text().into();

        self.threads.insert(thread.read(cx).id().clone(), id);
        self.context
            .push(AssistantContext::Thread(ThreadContext { id, thread, text }));
    }

    pub fn add_fetched_url(&mut self, url: String, text: impl Into<SharedString>) {
        if self.includes_url(&url).is_none() {
            self.insert_fetched_url(url, text);
        }
    }

    fn insert_fetched_url(&mut self, url: String, text: impl Into<SharedString>) {
        let id = self.next_context_id.post_inc();

        self.fetched_urls.insert(url.clone(), id);
        self.context
            .push(AssistantContext::FetchedUrl(FetchedUrlContext {
                id,
                url: url.into(),
                text: text.into(),
            }));
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

    pub fn remove_context(&mut self, id: ContextId) {
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
            AssistantContext::FetchedUrl(_) => {
                self.fetched_urls.retain(|_, context_id| *context_id != id);
            }
            AssistantContext::Thread(_) => {
                self.threads.retain(|_, context_id| *context_id != id);
            }
        }
    }

    /// Returns whether the buffer is already included directly in the context, or if it will be
    /// included in the context via a directory. Directory inclusion is based on paths rather than
    /// buffer IDs as the directory will be re-scanned.
    pub fn will_include_buffer(&self, buffer_id: BufferId, path: &Path) -> Option<FileInclusion> {
        if let Some(context_id) = self.files.get(&buffer_id) {
            return Some(FileInclusion::Direct(*context_id));
        }

        self.will_include_file_path_via_directory(path)
    }

    /// Returns whether this file path is already included directly in the context, or if it will be
    /// included in the context via a directory.
    pub fn will_include_file_path(&self, path: &Path, cx: &App) -> Option<FileInclusion> {
        if !self.files.is_empty() {
            let found_file_context = self.context.iter().find(|context| match &context {
                AssistantContext::File(file_context) => {
                    let buffer = file_context.context_buffer.buffer.read(cx);
                    if let Some(file_path) = buffer_path_log_err(buffer, cx) {
                        *file_path == *path
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

        self.will_include_file_path_via_directory(path)
    }

    fn will_include_file_path_via_directory(&self, path: &Path) -> Option<FileInclusion> {
        if self.directories.is_empty() {
            return None;
        }

        let mut buf = path.to_path_buf();

        while buf.pop() {
            if let Some(_) = self.directories.get(&buf) {
                return Some(FileInclusion::InDirectory(buf));
            }
        }

        None
    }

    pub fn includes_directory(&self, path: &Path) -> Option<FileInclusion> {
        if let Some(context_id) = self.directories.get(path) {
            return Some(FileInclusion::Direct(*context_id));
        }

        self.will_include_file_path_via_directory(path)
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

    pub fn file_paths(&self, cx: &App) -> HashSet<PathBuf> {
        self.context
            .iter()
            .filter_map(|context| match context {
                AssistantContext::File(file) => {
                    let buffer = file.context_buffer.buffer.read(cx);
                    buffer_path_log_err(buffer, cx).map(|p| p.to_path_buf())
                }
                AssistantContext::Directory(_)
                | AssistantContext::Symbol(_)
                | AssistantContext::FetchedUrl(_)
                | AssistantContext::Thread(_) => None,
            })
            .collect()
    }

    pub fn thread_ids(&self) -> HashSet<ThreadId> {
        self.threads.keys().cloned().collect()
    }
}

pub enum FileInclusion {
    Direct(ContextId),
    InDirectory(PathBuf),
}

// ContextBuffer without text.
struct BufferInfo {
    buffer_entity: Entity<Buffer>,
    file: Arc<dyn File>,
    id: BufferId,
    version: clock::Global,
}

fn make_context_buffer(info: BufferInfo, text: SharedString) -> ContextBuffer {
    ContextBuffer {
        id: info.id,
        buffer: info.buffer_entity,
        file: info.file,
        version: info.version,
        text,
    }
}

fn make_context_symbol(
    info: BufferInfo,
    path: ProjectPath,
    name: SharedString,
    range: Range<Anchor>,
    enclosing_range: Range<Anchor>,
    text: SharedString,
) -> ContextSymbol {
    ContextSymbol {
        id: ContextSymbolId { name, range, path },
        buffer_version: info.version,
        enclosing_range,
        buffer: info.buffer_entity,
        text,
    }
}

fn collect_buffer_info_and_text(
    path: Arc<Path>,
    buffer_entity: Entity<Buffer>,
    buffer: &Buffer,
    range: Option<Range<Anchor>>,
    cx: AsyncApp,
) -> Result<(BufferInfo, Task<SharedString>)> {
    let buffer_info = BufferInfo {
        id: buffer.remote_id(),
        buffer_entity,
        file: buffer
            .file()
            .context("buffer context must have a file")?
            .clone(),
        version: buffer.version(),
    };
    // Important to collect version at the same time as content so that staleness logic is correct.
    let content = if let Some(range) = range {
        buffer.text_for_range(range).collect::<Rope>()
    } else {
        buffer.as_rope().clone()
    };
    let text_task = cx.background_spawn(async move { to_fenced_codeblock(&path, content) });
    Ok((buffer_info, text_task))
}

pub fn buffer_path_log_err(buffer: &Buffer, cx: &App) -> Option<Arc<Path>> {
    if let Some(file) = buffer.file() {
        let mut path = file.path().clone();
        if path.as_os_str().is_empty() {
            path = file.full_path(cx).into();
        }
        Some(path)
    } else {
        log::error!("Buffer that had a path unexpectedly no longer has a path.");
        None
    }
}

fn to_fenced_codeblock(path: &Path, content: Rope) -> SharedString {
    let path_extension = path.extension().and_then(|ext| ext.to_str());
    let path_string = path.to_string_lossy();
    let capacity = 3
        + path_extension.map_or(0, |extension| extension.len() + 1)
        + path_string.len()
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
                    if changed_buffers.is_empty()
                        || changed_buffers.contains(&file_context.context_buffer.buffer)
                    {
                        let context_store = context_store.clone();
                        return refresh_file_text(context_store, file_context, cx);
                    }
                }
                AssistantContext::Directory(directory_context) => {
                    let should_refresh = changed_buffers.is_empty()
                        || changed_buffers.iter().any(|buffer| {
                            let buffer = buffer.read(cx);

                            buffer_path_log_err(&buffer, cx).map_or(false, |path| {
                                path.starts_with(&directory_context.project_path.path)
                            })
                        });

                    if should_refresh {
                        let context_store = context_store.clone();
                        return refresh_directory_text(context_store, directory_context, cx);
                    }
                }
                AssistantContext::Symbol(symbol_context) => {
                    if changed_buffers.is_empty()
                        || changed_buffers.contains(&symbol_context.context_symbol.buffer)
                    {
                        let context_store = context_store.clone();
                        return refresh_symbol_text(context_store, symbol_context, cx);
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
    let project_path = directory_context.project_path.clone();
    Some(cx.spawn(async move |cx| {
        let context_buffers = context_buffers.await;
        context_store
            .update(cx, |context_store, _| {
                let new_directory_context = DirectoryContext {
                    id,
                    project_path,
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
                let text = thread.read(cx).text().into();
                context_store.replace_context(AssistantContext::Thread(ThreadContext {
                    id,
                    thread,
                    text,
                }));
            })
            .ok();
    })
}

fn refresh_context_buffer(
    context_buffer: &ContextBuffer,
    cx: &App,
) -> Option<impl Future<Output = ContextBuffer> + use<>> {
    let buffer = context_buffer.buffer.read(cx);
    let path = buffer_path_log_err(buffer, cx)?;
    if buffer.version.changed_since(&context_buffer.version) {
        let (buffer_info, text_task) = collect_buffer_info_and_text(
            path,
            context_buffer.buffer.clone(),
            buffer,
            None,
            cx.to_async(),
        )
        .log_err()?;
        Some(text_task.map(move |text| make_context_buffer(buffer_info, text)))
    } else {
        None
    }
}

fn refresh_context_symbol(
    context_symbol: &ContextSymbol,
    cx: &App,
) -> Option<impl Future<Output = ContextSymbol> + use<>> {
    let buffer = context_symbol.buffer.read(cx);
    let path = buffer_path_log_err(buffer, cx)?;
    let project_path = buffer.project_path(cx)?;
    if buffer.version.changed_since(&context_symbol.buffer_version) {
        let (buffer_info, text_task) = collect_buffer_info_and_text(
            path,
            context_symbol.buffer.clone(),
            buffer,
            Some(context_symbol.enclosing_range.clone()),
            cx.to_async(),
        )
        .log_err()?;
        let name = context_symbol.id.name.clone();
        let range = context_symbol.id.range.clone();
        let enclosing_range = context_symbol.enclosing_range.clone();
        Some(text_task.map(move |text| {
            make_context_symbol(
                buffer_info,
                project_path,
                name,
                range,
                enclosing_range,
                text,
            )
        }))
    } else {
        None
    }
}
