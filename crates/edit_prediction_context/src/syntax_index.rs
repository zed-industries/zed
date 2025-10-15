use anyhow::{Result, anyhow};
use collections::{HashMap, HashSet};
use futures::channel::mpsc;
use futures::lock::Mutex;
use futures::{FutureExt as _, StreamExt, future};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, Task, WeakEntity};
use itertools::Itertools;

use language::{Buffer, BufferEvent};
use postage::stream::Stream as _;
use project::buffer_store::{BufferStore, BufferStoreEvent};
use project::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use project::{PathChange, Project, ProjectEntryId, ProjectPath};
use slotmap::SlotMap;
use std::iter;
use std::ops::{DerefMut, Range};
use std::sync::Arc;
use text::BufferId;
use util::{RangeExt as _, debug_panic, some_or_debug_panic};

use crate::CachedDeclarationPath;
use crate::declaration::{
    BufferDeclaration, Declaration, DeclarationId, FileDeclaration, Identifier,
};
use crate::outline::declarations_in_buffer;

// TODO
//
// * Also queue / debounce buffer changes. A challenge for this is that use of
// `buffer_declarations_containing_range` assumes that the index is always immediately up to date.
//
// * Add a per language configuration for skipping indexing.
//
// * Handle tsx / ts / js referencing each-other

// Potential future improvements:
//
// * Prevent indexing of a large file from blocking the queue.
//
// * Send multiple selected excerpt ranges. Challenge is that excerpt ranges influence which
// references are present and their scores.
//
// * Include single-file worktrees / non visible worktrees? E.g. go to definition that resolves to a
// file in a build dependency. Should not be editable in that case - but how to distinguish the case
// where it should be editable?

// Potential future optimizations:
//
// * Index files on multiple threads in Zed (currently only parallel for the CLI). Adding some kind
// of priority system to the background executor could help - it's single threaded for now to avoid
// interfering with other work.
//
// * Parse files directly instead of loading into a Rope.
//
//   - This would allow the task handling dirty_files to be done entirely on the background executor.
//
//   - Make SyntaxMap generic to handle embedded languages? Will also need to find line boundaries,
//   but that can be done by scanning characters in the flat representation.
//
// * Use something similar to slotmap without key versions.
//
// * Concurrent slotmap

pub struct SyntaxIndex {
    state: Arc<Mutex<SyntaxIndexState>>,
    project: WeakEntity<Project>,
    initial_file_indexing_done_rx: postage::watch::Receiver<bool>,
    _file_indexing_task: Option<Task<()>>,
}

pub struct SyntaxIndexState {
    declarations: SlotMap<DeclarationId, Declaration>,
    identifiers: HashMap<Identifier, HashSet<DeclarationId>>,
    files: HashMap<ProjectEntryId, FileState>,
    buffers: HashMap<BufferId, BufferState>,
    dirty_files: HashMap<ProjectEntryId, ProjectPath>,
    dirty_files_tx: mpsc::Sender<()>,
}

#[derive(Debug, Default)]
struct FileState {
    declarations: Vec<DeclarationId>,
}

#[derive(Default)]
struct BufferState {
    declarations: Vec<DeclarationId>,
    task: Option<Task<()>>,
}

impl SyntaxIndex {
    pub fn new(
        project: &Entity<Project>,
        file_indexing_parallelism: usize,
        cx: &mut Context<Self>,
    ) -> Self {
        assert!(file_indexing_parallelism > 0);
        let (dirty_files_tx, mut dirty_files_rx) = mpsc::channel::<()>(1);
        let (mut initial_file_indexing_done_tx, initial_file_indexing_done_rx) =
            postage::watch::channel();

        let initial_state = SyntaxIndexState {
            declarations: SlotMap::default(),
            identifiers: HashMap::default(),
            files: HashMap::default(),
            buffers: HashMap::default(),
            dirty_files: HashMap::default(),
            dirty_files_tx,
        };
        let mut this = Self {
            project: project.downgrade(),
            state: Arc::new(Mutex::new(initial_state)),
            initial_file_indexing_done_rx,
            _file_indexing_task: None,
        };

        let worktree_store = project.read(cx).worktree_store();
        let initial_worktree_snapshots = worktree_store
            .read(cx)
            .worktrees()
            .map(|w| w.read(cx).snapshot())
            .collect::<Vec<_>>();
        this._file_indexing_task = Some(cx.spawn(async move |this, cx| {
            let snapshots_file_count = initial_worktree_snapshots
                .iter()
                .map(|worktree| worktree.file_count())
                .sum::<usize>();
            if snapshots_file_count > 0 {
                let chunk_size = snapshots_file_count.div_ceil(file_indexing_parallelism);
                let chunk_count = snapshots_file_count.div_ceil(chunk_size);
                let file_chunks = initial_worktree_snapshots
                    .iter()
                    .flat_map(|worktree| {
                        let worktree_id = worktree.id();
                        worktree.files(false, 0).map(move |entry| {
                            (
                                entry.id,
                                ProjectPath {
                                    worktree_id,
                                    path: entry.path.clone(),
                                },
                            )
                        })
                    })
                    .chunks(chunk_size);

                let mut tasks = Vec::with_capacity(chunk_count);
                for chunk in file_chunks.into_iter() {
                    tasks.push(Self::update_dirty_files(
                        &this,
                        chunk.into_iter().collect(),
                        cx.clone(),
                    ));
                }
                futures::future::join_all(tasks).await;
                log::info!("Finished initial file indexing");
            }

            *initial_file_indexing_done_tx.borrow_mut() = true;

            let Ok(state) = this.read_with(cx, |this, _cx| Arc::downgrade(&this.state)) else {
                return;
            };
            while dirty_files_rx.next().await.is_some() {
                let Some(state) = state.upgrade() else {
                    return;
                };
                let mut state = state.lock().await;
                let was_underused = state.dirty_files.capacity() > 255
                    && state.dirty_files.len() * 8 < state.dirty_files.capacity();
                let dirty_files = state.dirty_files.drain().collect::<Vec<_>>();
                if was_underused {
                    state.dirty_files.shrink_to_fit();
                }
                drop(state);
                if dirty_files.is_empty() {
                    continue;
                }

                let chunk_size = dirty_files.len().div_ceil(file_indexing_parallelism);
                let chunk_count = dirty_files.len().div_ceil(chunk_size);
                let mut tasks = Vec::with_capacity(chunk_count);
                let chunks = dirty_files.into_iter().chunks(chunk_size);
                for chunk in chunks.into_iter() {
                    tasks.push(Self::update_dirty_files(
                        &this,
                        chunk.into_iter().collect(),
                        cx.clone(),
                    ));
                }
                futures::future::join_all(tasks).await;
            }
        }));

        cx.subscribe(&worktree_store, Self::handle_worktree_store_event)
            .detach();

        let buffer_store = project.read(cx).buffer_store().clone();
        for buffer in buffer_store.read(cx).buffers().collect::<Vec<_>>() {
            this.register_buffer(&buffer, cx);
        }
        cx.subscribe(&buffer_store, Self::handle_buffer_store_event)
            .detach();

        this
    }

    async fn update_dirty_files(
        this: &WeakEntity<Self>,
        dirty_files: Vec<(ProjectEntryId, ProjectPath)>,
        mut cx: AsyncApp,
    ) {
        for (entry_id, project_path) in dirty_files {
            let Ok(task) = this.update(&mut cx, |this, cx| {
                this.update_file(entry_id, project_path, cx)
            }) else {
                return;
            };
            task.await;
        }
    }

    pub fn wait_for_initial_file_indexing(&self, cx: &App) -> Task<Result<()>> {
        if *self.initial_file_indexing_done_rx.borrow() {
            Task::ready(Ok(()))
        } else {
            let mut rx = self.initial_file_indexing_done_rx.clone();
            cx.background_spawn(async move {
                loop {
                    match rx.recv().await {
                        Some(true) => return Ok(()),
                        Some(false) => {}
                        None => {
                            return Err(anyhow!(
                                "SyntaxIndex dropped while waiting for initial file indexing"
                            ));
                        }
                    }
                }
            })
        }
    }

    pub fn indexed_file_paths(&self, cx: &App) -> Task<Vec<ProjectPath>> {
        let state = self.state.clone();
        let project = self.project.clone();

        cx.spawn(async move |cx| {
            let state = state.lock().await;
            let Some(project) = project.upgrade() else {
                return vec![];
            };
            project
                .read_with(cx, |project, cx| {
                    state
                        .files
                        .keys()
                        .filter_map(|entry_id| project.path_for_entry(*entry_id, cx))
                        .collect()
                })
                .unwrap_or_default()
        })
    }

    fn handle_worktree_store_event(
        &mut self,
        _worktree_store: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        use WorktreeStoreEvent::*;
        match event {
            WorktreeUpdatedEntries(worktree_id, updated_entries_set) => {
                let state = Arc::downgrade(&self.state);
                let worktree_id = *worktree_id;
                let updated_entries_set = updated_entries_set.clone();
                cx.background_spawn(async move {
                    let Some(state) = state.upgrade() else { return };
                    let mut state = state.lock().await;
                    for (path, entry_id, path_change) in updated_entries_set.iter() {
                        if let PathChange::Removed = path_change {
                            state.files.remove(entry_id);
                            state.dirty_files.remove(entry_id);
                        } else {
                            let project_path = ProjectPath {
                                worktree_id,
                                path: path.clone(),
                            };
                            state.dirty_files.insert(*entry_id, project_path);
                        }
                    }
                    match state.dirty_files_tx.try_send(()) {
                        Err(err) if err.is_disconnected() => {
                            log::error!("bug: syntax indexing queue is disconnected");
                        }
                        _ => {}
                    }
                })
                .detach();
            }
            WorktreeDeletedEntry(_worktree_id, project_entry_id) => {
                let project_entry_id = *project_entry_id;
                self.with_state(cx, move |state| {
                    state.files.remove(&project_entry_id);
                })
            }
            _ => {}
        }
    }

    fn handle_buffer_store_event(
        &mut self,
        _buffer_store: Entity<BufferStore>,
        event: &BufferStoreEvent,
        cx: &mut Context<Self>,
    ) {
        use BufferStoreEvent::*;
        match event {
            BufferAdded(buffer) => self.register_buffer(buffer, cx),
            BufferOpened { .. }
            | BufferChangedFilePath { .. }
            | BufferDropped { .. }
            | SharedBufferClosed { .. } => {}
        }
    }

    pub fn state(&self) -> &Arc<Mutex<SyntaxIndexState>> {
        &self.state
    }

    fn with_state(&self, cx: &mut App, f: impl FnOnce(&mut SyntaxIndexState) + Send + 'static) {
        if let Some(mut state) = self.state.try_lock() {
            f(&mut state);
            return;
        }
        let state = Arc::downgrade(&self.state);
        cx.background_spawn(async move {
            let Some(state) = state.upgrade() else {
                return;
            };
            let mut state = state.lock().await;
            f(&mut state)
        })
        .detach();
    }

    fn register_buffer(&self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
        let buffer_id = buffer.read(cx).remote_id();
        cx.observe_release(buffer, move |this, _buffer, cx| {
            this.with_state(cx, move |state| {
                if let Some(buffer_state) = state.buffers.remove(&buffer_id) {
                    SyntaxIndexState::remove_buffer_declarations(
                        &buffer_state.declarations,
                        &mut state.declarations,
                        &mut state.identifiers,
                    );
                }
            })
        })
        .detach();
        cx.subscribe(buffer, Self::handle_buffer_event).detach();

        self.update_buffer(buffer.clone(), cx);
    }

    fn handle_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &BufferEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            BufferEvent::Edited |
            // paths are cached and so should be updated
            BufferEvent::FileHandleChanged => self.update_buffer(buffer, cx),
            _ => {}
        }
    }

    fn update_buffer(&self, buffer_entity: Entity<Buffer>, cx: &mut Context<Self>) {
        let buffer = buffer_entity.read(cx);
        if buffer.language().is_none() {
            return;
        }

        let Some((project_entry_id, cached_path)) = project::File::from_dyn(buffer.file())
            .and_then(|f| {
                let project_entry_id = f.project_entry_id()?;
                let cached_path = CachedDeclarationPath::new(
                    f.worktree.read(cx).abs_path(),
                    &f.path,
                    buffer.language(),
                );
                Some((project_entry_id, cached_path))
            })
        else {
            return;
        };
        let buffer_id = buffer.remote_id();

        let mut parse_status = buffer.parse_status();
        let snapshot_task = cx.spawn({
            let weak_buffer = buffer_entity.downgrade();
            async move |_, cx| {
                while *parse_status.borrow() != language::ParseStatus::Idle {
                    parse_status.changed().await?;
                }
                weak_buffer.read_with(cx, |buffer, _cx| buffer.snapshot())
            }
        });

        let state = Arc::downgrade(&self.state);
        let task = cx.background_spawn(async move {
            // TODO: How to handle errors?
            let Ok(snapshot) = snapshot_task.await else {
                return;
            };
            let rope = snapshot.text.as_rope();

            let declarations = declarations_in_buffer(&snapshot)
                .into_iter()
                .map(|item| {
                    (
                        item.parent_index,
                        BufferDeclaration::from_outline(item, &rope),
                    )
                })
                .collect::<Vec<_>>();

            let Some(state) = state.upgrade() else {
                return;
            };
            let mut state = state.lock().await;
            let state = state.deref_mut();

            let buffer_state = state
                .buffers
                .entry(buffer_id)
                .or_insert_with(Default::default);

            SyntaxIndexState::remove_buffer_declarations(
                &buffer_state.declarations,
                &mut state.declarations,
                &mut state.identifiers,
            );

            let mut new_ids = Vec::with_capacity(declarations.len());
            state.declarations.reserve(declarations.len());
            for (parent_index, mut declaration) in declarations {
                declaration.parent =
                    parent_index.and_then(|ix| some_or_debug_panic(new_ids.get(ix).copied()));

                let identifier = declaration.identifier.clone();
                let declaration_id = state.declarations.insert(Declaration::Buffer {
                    rope: rope.clone(),
                    buffer_id,
                    declaration,
                    project_entry_id,
                    cached_path: cached_path.clone(),
                });
                new_ids.push(declaration_id);

                state
                    .identifiers
                    .entry(identifier)
                    .or_default()
                    .insert(declaration_id);
            }

            buffer_state.declarations = new_ids;
        });

        self.with_state(cx, move |state| {
            state
                .buffers
                .entry(buffer_id)
                .or_insert_with(Default::default)
                .task = Some(task)
        });
    }

    fn update_file(
        &mut self,
        entry_id: ProjectEntryId,
        project_path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let Some(project) = self.project.upgrade() else {
            return Task::ready(());
        };
        let project = project.read(cx);

        let language_registry = project.languages();
        let Some(available_language) =
            language_registry.language_for_file_path(project_path.path.as_std_path())
        else {
            return Task::ready(());
        };
        let language = if let Some(Ok(Ok(language))) = language_registry
            .load_language(&available_language)
            .now_or_never()
        {
            if language
                .grammar()
                .is_none_or(|grammar| grammar.outline_config.is_none())
            {
                return Task::ready(());
            }
            future::Either::Left(async { Ok(language) })
        } else {
            let language_registry = language_registry.clone();
            future::Either::Right(async move {
                anyhow::Ok(
                    language_registry
                        .load_language(&available_language)
                        .await??,
                )
            })
        };

        let Some(worktree) = project.worktree_for_id(project_path.worktree_id, cx) else {
            return Task::ready(());
        };

        let snapshot_task = worktree.update(cx, |worktree, cx| {
            let load_task = worktree.load_file(&project_path.path, cx);
            let worktree_abs_path = worktree.abs_path();
            cx.spawn(async move |_this, cx| {
                let loaded_file = load_task.await?;
                let language = language.await?;

                let buffer = cx.new(|cx| {
                    let mut buffer = Buffer::local(loaded_file.text, cx);
                    buffer.set_language(Some(language.clone()), cx);
                    buffer
                })?;

                let mut parse_status = buffer.read_with(cx, |buffer, _| buffer.parse_status())?;
                while *parse_status.borrow() != language::ParseStatus::Idle {
                    parse_status.changed().await?;
                }

                let cached_path = CachedDeclarationPath::new(
                    worktree_abs_path,
                    &project_path.path,
                    Some(&language),
                );

                let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

                anyhow::Ok((snapshot, cached_path))
            })
        });

        let state = Arc::downgrade(&self.state);
        cx.background_spawn(async move {
            // TODO: How to handle errors?
            let Ok((snapshot, cached_path)) = snapshot_task.await else {
                return;
            };
            let rope = snapshot.as_rope();
            let declarations = declarations_in_buffer(&snapshot)
                .into_iter()
                .map(|item| (item.parent_index, FileDeclaration::from_outline(item, rope)))
                .collect::<Vec<_>>();

            let Some(state) = state.upgrade() else {
                return;
            };
            let mut state = state.lock().await;
            let state = state.deref_mut();

            let file_state = state.files.entry(entry_id).or_insert_with(Default::default);
            for old_declaration_id in &file_state.declarations {
                let Some(declaration) = state.declarations.remove(*old_declaration_id) else {
                    debug_panic!("declaration not found");
                    continue;
                };
                if let Some(identifier_declarations) =
                    state.identifiers.get_mut(declaration.identifier())
                {
                    identifier_declarations.remove(old_declaration_id);
                }
            }

            let mut new_ids = Vec::with_capacity(declarations.len());
            state.declarations.reserve(declarations.len());
            for (parent_index, mut declaration) in declarations {
                declaration.parent =
                    parent_index.and_then(|ix| some_or_debug_panic(new_ids.get(ix).copied()));

                let identifier = declaration.identifier.clone();
                let declaration_id = state.declarations.insert(Declaration::File {
                    project_entry_id: entry_id,
                    declaration,
                    cached_path: cached_path.clone(),
                });
                new_ids.push(declaration_id);

                state
                    .identifiers
                    .entry(identifier)
                    .or_default()
                    .insert(declaration_id);
            }
            file_state.declarations = new_ids;
        })
    }
}

impl SyntaxIndexState {
    pub fn declaration(&self, id: DeclarationId) -> Option<&Declaration> {
        self.declarations.get(id)
    }

    /// Returns declarations for the identifier. If the limit is exceeded, returns an empty vector.
    ///
    /// TODO: Consider doing some pre-ranking and instead truncating when N is exceeded.
    pub fn declarations_for_identifier<const N: usize>(
        &self,
        identifier: &Identifier,
    ) -> Vec<(DeclarationId, &Declaration)> {
        // make sure to not have a large stack allocation
        assert!(N < 32);

        let Some(declaration_ids) = self.identifiers.get(&identifier) else {
            return vec![];
        };

        let mut result = Vec::with_capacity(N);
        let mut included_buffer_entry_ids = arrayvec::ArrayVec::<_, N>::new();
        let mut file_declarations = Vec::new();

        for declaration_id in declaration_ids {
            let declaration = self.declarations.get(*declaration_id);
            let Some(declaration) = some_or_debug_panic(declaration) else {
                continue;
            };
            match declaration {
                Declaration::Buffer {
                    project_entry_id, ..
                } => {
                    included_buffer_entry_ids.push(*project_entry_id);
                    result.push((*declaration_id, declaration));
                    if result.len() == N {
                        return Vec::new();
                    }
                }
                Declaration::File {
                    project_entry_id, ..
                } => {
                    if !included_buffer_entry_ids.contains(&project_entry_id) {
                        file_declarations.push((*declaration_id, declaration));
                    }
                }
            }
        }

        for (declaration_id, declaration) in file_declarations {
            match declaration {
                Declaration::File {
                    project_entry_id, ..
                } => {
                    if !included_buffer_entry_ids.contains(&project_entry_id) {
                        result.push((declaration_id, declaration));

                        if result.len() == N {
                            return Vec::new();
                        }
                    }
                }
                Declaration::Buffer { .. } => {}
            }
        }

        result
    }

    pub fn buffer_declarations_containing_range(
        &self,
        buffer_id: BufferId,
        range: Range<usize>,
    ) -> impl Iterator<Item = (DeclarationId, &BufferDeclaration)> {
        let Some(buffer_state) = self.buffers.get(&buffer_id) else {
            return itertools::Either::Left(iter::empty());
        };

        let iter = buffer_state
            .declarations
            .iter()
            .filter_map(move |declaration_id| {
                let Some(declaration) = self
                    .declarations
                    .get(*declaration_id)
                    .and_then(|d| d.as_buffer())
                else {
                    log::error!("bug: missing buffer outline declaration");
                    return None;
                };
                if declaration.item_range.contains_inclusive(&range) {
                    return Some((*declaration_id, declaration));
                }
                return None;
            });
        itertools::Either::Right(iter)
    }

    pub fn file_declaration_count(&self, declaration: &Declaration) -> usize {
        match declaration {
            Declaration::File {
                project_entry_id, ..
            } => self
                .files
                .get(project_entry_id)
                .map(|file_state| file_state.declarations.len())
                .unwrap_or_default(),
            Declaration::Buffer { buffer_id, .. } => self
                .buffers
                .get(buffer_id)
                .map(|buffer_state| buffer_state.declarations.len())
                .unwrap_or_default(),
        }
    }

    fn remove_buffer_declarations(
        old_declaration_ids: &[DeclarationId],
        declarations: &mut SlotMap<DeclarationId, Declaration>,
        identifiers: &mut HashMap<Identifier, HashSet<DeclarationId>>,
    ) {
        for old_declaration_id in old_declaration_ids {
            let Some(declaration) = declarations.remove(*old_declaration_id) else {
                debug_panic!("declaration not found");
                continue;
            };
            if let Some(identifier_declarations) = identifiers.get_mut(declaration.identifier()) {
                identifier_declarations.remove(old_declaration_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use gpui::TestAppContext;
    use indoc::indoc;
    use language::{Language, LanguageConfig, LanguageId, LanguageMatcher, tree_sitter_rust};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use text::OffsetRangeExt as _;
    use util::{path, rel_path::rel_path};

    use crate::syntax_index::SyntaxIndex;

    #[gpui::test]
    async fn test_unopen_indexed_files(cx: &mut TestAppContext) {
        let (project, index, rust_lang_id) = init_test(cx).await;
        let main = Identifier {
            name: "main".into(),
            language_id: rust_lang_id,
        };

        let index_state = index.read_with(cx, |index, _cx| index.state().clone());
        let index_state = index_state.lock().await;
        cx.update(|cx| {
            let decls = index_state.declarations_for_identifier::<8>(&main);
            assert_eq!(decls.len(), 2);

            let decl = expect_file_decl("a.rs", &decls[0].1, &project, cx);
            assert_eq!(decl.identifier, main);
            assert_eq!(decl.item_range, 0..98);

            let decl = expect_file_decl("c.rs", &decls[1].1, &project, cx);
            assert_eq!(decl.identifier, main.clone());
            assert_eq!(decl.item_range, 32..280);
        });
    }

    #[gpui::test]
    async fn test_parents_in_file(cx: &mut TestAppContext) {
        let (project, index, rust_lang_id) = init_test(cx).await;
        let test_process_data = Identifier {
            name: "test_process_data".into(),
            language_id: rust_lang_id,
        };

        let index_state = index.read_with(cx, |index, _cx| index.state().clone());
        let index_state = index_state.lock().await;
        cx.update(|cx| {
            let decls = index_state.declarations_for_identifier::<8>(&test_process_data);
            assert_eq!(decls.len(), 1);

            let decl = expect_file_decl("c.rs", &decls[0].1, &project, cx);
            assert_eq!(decl.identifier, test_process_data);

            let parent_id = decl.parent.unwrap();
            let parent = index_state.declaration(parent_id).unwrap();
            let parent_decl = expect_file_decl("c.rs", &parent, &project, cx);
            assert_eq!(
                parent_decl.identifier,
                Identifier {
                    name: "tests".into(),
                    language_id: rust_lang_id
                }
            );
            assert_eq!(parent_decl.parent, None);
        });
    }

    #[gpui::test]
    async fn test_parents_in_buffer(cx: &mut TestAppContext) {
        let (project, index, rust_lang_id) = init_test(cx).await;
        let test_process_data = Identifier {
            name: "test_process_data".into(),
            language_id: rust_lang_id,
        };

        let buffer = project
            .update(cx, |project, cx| {
                let project_path = project.find_project_path("c.rs", cx).unwrap();
                project.open_buffer(project_path, cx)
            })
            .await
            .unwrap();

        cx.run_until_parked();

        let index_state = index.read_with(cx, |index, _cx| index.state().clone());
        let index_state = index_state.lock().await;
        cx.update(|cx| {
            let decls = index_state.declarations_for_identifier::<8>(&test_process_data);
            assert_eq!(decls.len(), 1);

            let decl = expect_buffer_decl("c.rs", &decls[0].1, &project, cx);
            assert_eq!(decl.identifier, test_process_data);

            let parent_id = decl.parent.unwrap();
            let parent = index_state.declaration(parent_id).unwrap();
            let parent_decl = expect_buffer_decl("c.rs", &parent, &project, cx);
            assert_eq!(
                parent_decl.identifier,
                Identifier {
                    name: "tests".into(),
                    language_id: rust_lang_id
                }
            );
            assert_eq!(parent_decl.parent, None);
        });

        drop(buffer);
    }

    #[gpui::test]
    async fn test_declarations_limt(cx: &mut TestAppContext) {
        let (_, index, rust_lang_id) = init_test(cx).await;

        let index_state = index.read_with(cx, |index, _cx| index.state().clone());
        let index_state = index_state.lock().await;
        let decls = index_state.declarations_for_identifier::<1>(&Identifier {
            name: "main".into(),
            language_id: rust_lang_id,
        });
        assert_eq!(decls.len(), 0);
    }

    #[gpui::test]
    async fn test_buffer_shadow(cx: &mut TestAppContext) {
        let (project, index, rust_lang_id) = init_test(cx).await;

        let main = Identifier {
            name: "main".into(),
            language_id: rust_lang_id,
        };

        let buffer = project
            .update(cx, |project, cx| {
                let project_path = project.find_project_path("c.rs", cx).unwrap();
                project.open_buffer(project_path, cx)
            })
            .await
            .unwrap();

        cx.run_until_parked();

        let index_state_arc = index.read_with(cx, |index, _cx| index.state().clone());
        {
            let index_state = index_state_arc.lock().await;

            cx.update(|cx| {
                let decls = index_state.declarations_for_identifier::<8>(&main);
                assert_eq!(decls.len(), 2);
                let decl = expect_buffer_decl("c.rs", &decls[0].1, &project, cx);
                assert_eq!(decl.identifier, main);
                assert_eq!(decl.item_range.to_offset(&buffer.read(cx)), 32..280);

                expect_file_decl("a.rs", &decls[1].1, &project, cx);
            });
        }

        // Drop the buffer and wait for release
        cx.update(|_| {
            drop(buffer);
        });
        cx.run_until_parked();

        let index_state = index_state_arc.lock().await;

        cx.update(|cx| {
            let decls = index_state.declarations_for_identifier::<8>(&main);
            assert_eq!(decls.len(), 2);
            expect_file_decl("a.rs", &decls[0].1, &project, cx);
            expect_file_decl("c.rs", &decls[1].1, &project, cx);
        });
    }

    fn expect_buffer_decl<'a>(
        path: &str,
        declaration: &'a Declaration,
        project: &Entity<Project>,
        cx: &App,
    ) -> &'a BufferDeclaration {
        if let Declaration::Buffer {
            declaration,
            project_entry_id,
            ..
        } = declaration
        {
            let project_path = project
                .read(cx)
                .path_for_entry(*project_entry_id, cx)
                .unwrap();
            assert_eq!(project_path.path.as_ref(), rel_path(path),);
            declaration
        } else {
            panic!("Expected a buffer declaration, found {:?}", declaration);
        }
    }

    fn expect_file_decl<'a>(
        path: &str,
        declaration: &'a Declaration,
        project: &Entity<Project>,
        cx: &App,
    ) -> &'a FileDeclaration {
        if let Declaration::File {
            declaration,
            project_entry_id: file,
            ..
        } = declaration
        {
            assert_eq!(
                project
                    .read(cx)
                    .path_for_entry(*file, cx)
                    .unwrap()
                    .path
                    .as_ref(),
                rel_path(path),
            );
            declaration
        } else {
            panic!("Expected a file declaration, found {:?}", declaration);
        }
    }

    async fn init_test(
        cx: &mut TestAppContext,
    ) -> (Entity<Project>, Entity<SyntaxIndex>, LanguageId) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "a.rs": indoc! {r#"
                    fn main() {
                        let x = 1;
                        let y = 2;
                        let z = add(x, y);
                        println!("Result: {}", z);
                    }

                    fn add(a: i32, b: i32) -> i32 {
                        a + b
                    }
                "#},
                "b.rs": indoc! {"
                    pub struct Config {
                        pub name: String,
                        pub value: i32,
                    }

                    impl Config {
                        pub fn new(name: String, value: i32) -> Self {
                            Config { name, value }
                        }
                    }
                "},
                "c.rs": indoc! {r#"
                    use std::collections::HashMap;

                    fn main() {
                        let args: Vec<String> = std::env::args().collect();
                        let data: Vec<i32> = args[1..]
                            .iter()
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        let result = process_data(data);
                        println!("{:?}", result);
                    }

                    fn process_data(data: Vec<i32>) -> HashMap<i32, usize> {
                        let mut counts = HashMap::new();
                        for value in data {
                            *counts.entry(value).or_insert(0) += 1;
                        }
                        counts
                    }

                    #[cfg(test)]
                    mod tests {
                        use super::*;

                        #[test]
                        fn test_process_data() {
                            let data = vec![1, 2, 2, 3];
                            let result = process_data(data);
                            assert_eq!(result.get(&2), Some(&2));
                        }
                    }
                "#}
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let lang = rust_lang();
        let lang_id = lang.id();
        language_registry.add(Arc::new(lang));

        let file_indexing_parallelism = 2;
        let index = cx.new(|cx| SyntaxIndex::new(&project, file_indexing_parallelism, cx));
        cx.run_until_parked();

        (project, index, lang_id)
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_outline_query(include_str!("../../languages/src/rust/outline.scm"))
        .unwrap()
    }
}
