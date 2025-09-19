use std::sync::Arc;

use collections::{HashMap, HashSet};
use futures::lock::Mutex;
use gpui::{App, AppContext as _, Context, Entity, Task, WeakEntity};
use language::{Buffer, BufferEvent};
use project::buffer_store::{BufferStore, BufferStoreEvent};
use project::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use project::{PathChange, Project, ProjectEntryId, ProjectPath};
use slotmap::SlotMap;
use text::BufferId;
use util::{debug_panic, some_or_debug_panic};

use crate::declaration::{
    BufferDeclaration, Declaration, DeclarationId, FileDeclaration, Identifier,
};
use crate::outline::declarations_in_buffer;

// TODO:
//
// * Skip for remote projects
//
// * Consider making SyntaxIndex not an Entity.

// Potential future improvements:
//
// * Send multiple selected excerpt ranges. Challenge is that excerpt ranges influence which
// references are present and their scores.

// Potential future optimizations:
//
// * Cache of buffers for files
//
// * Parse files directly instead of loading into a Rope. Make SyntaxMap generic to handle embedded
// languages? Will also need to find line boundaries, but that can be done by scanning characters in
// the flat representation.
//
// * Use something similar to slotmap without key versions.
//
// * Concurrent slotmap
//
// * Use queue for parsing
//

pub struct SyntaxIndex {
    state: Arc<Mutex<SyntaxIndexState>>,
    project: WeakEntity<Project>,
}

#[derive(Default)]
pub struct SyntaxIndexState {
    declarations: SlotMap<DeclarationId, Declaration>,
    identifiers: HashMap<Identifier, HashSet<DeclarationId>>,
    files: HashMap<ProjectEntryId, FileState>,
    buffers: HashMap<BufferId, BufferState>,
}

#[derive(Debug, Default)]
struct FileState {
    declarations: Vec<DeclarationId>,
    task: Option<Task<()>>,
}

#[derive(Default)]
struct BufferState {
    declarations: Vec<DeclarationId>,
    task: Option<Task<()>>,
}

impl SyntaxIndex {
    pub fn new(project: &Entity<Project>, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            project: project.downgrade(),
            state: Arc::new(Mutex::new(SyntaxIndexState::default())),
        };

        let worktree_store = project.read(cx).worktree_store();
        cx.subscribe(&worktree_store, Self::handle_worktree_store_event)
            .detach();

        for worktree in worktree_store
            .read(cx)
            .worktrees()
            .map(|w| w.read(cx).snapshot())
            .collect::<Vec<_>>()
        {
            for entry in worktree.files(false, 0) {
                this.update_file(
                    entry.id,
                    ProjectPath {
                        worktree_id: worktree.id(),
                        path: entry.path.clone(),
                    },
                    cx,
                );
            }
        }

        let buffer_store = project.read(cx).buffer_store().clone();
        for buffer in buffer_store.read(cx).buffers().collect::<Vec<_>>() {
            this.register_buffer(&buffer, cx);
        }
        cx.subscribe(&buffer_store, Self::handle_buffer_store_event)
            .detach();

        this
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
                cx.spawn(async move |this, cx| {
                    let Some(state) = state.upgrade() else { return };
                    for (path, entry_id, path_change) in updated_entries_set.iter() {
                        if let PathChange::Removed = path_change {
                            state.lock().await.files.remove(entry_id);
                        } else {
                            let project_path = ProjectPath {
                                worktree_id,
                                path: path.clone(),
                            };
                            this.update(cx, |this, cx| {
                                this.update_file(*entry_id, project_path, cx);
                            })
                            .ok();
                        }
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

    fn register_buffer(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
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
            BufferEvent::Edited => self.update_buffer(buffer, cx),
            _ => {}
        }
    }

    fn update_buffer(&mut self, buffer_entity: Entity<Buffer>, cx: &mut Context<Self>) {
        let buffer = buffer_entity.read(cx);

        let Some(project_entry_id) =
            project::File::from_dyn(buffer.file()).and_then(|f| f.project_entry_id(cx))
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

        let parse_task = cx.background_spawn(async move {
            let snapshot = snapshot_task.await?;
            let rope = snapshot.text.as_rope().clone();

            anyhow::Ok((
                declarations_in_buffer(&snapshot)
                    .into_iter()
                    .map(|item| {
                        (
                            item.parent_index,
                            BufferDeclaration::from_outline(item, &rope),
                        )
                    })
                    .collect::<Vec<_>>(),
                rope,
            ))
        });

        let task = cx.spawn({
            async move |this, cx| {
                let Ok((declarations, rope)) = parse_task.await else {
                    return;
                };

                this.update(cx, move |this, cx| {
                    this.with_state(cx, move |state| {
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
                            declaration.parent = parent_index
                                .and_then(|ix| some_or_debug_panic(new_ids.get(ix).copied()));

                            let identifier = declaration.identifier.clone();
                            let declaration_id = state.declarations.insert(Declaration::Buffer {
                                rope: rope.clone(),
                                buffer_id,
                                declaration,
                                project_entry_id,
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
                })
                .ok();
            }
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
    ) {
        let Some(project) = self.project.upgrade() else {
            return;
        };
        let project = project.read(cx);
        let Some(worktree) = project.worktree_for_id(project_path.worktree_id, cx) else {
            return;
        };
        let language_registry = project.languages().clone();

        let snapshot_task = worktree.update(cx, |worktree, cx| {
            let load_task = worktree.load_file(&project_path.path, cx);
            cx.spawn(async move |_this, cx| {
                let loaded_file = load_task.await?;
                let language = language_registry
                    .language_for_file_path(&project_path.path)
                    .await
                    .ok();

                let buffer = cx.new(|cx| {
                    let mut buffer = Buffer::local(loaded_file.text, cx);
                    buffer.set_language(language, cx);
                    buffer
                })?;

                let mut parse_status = buffer.read_with(cx, |buffer, _| buffer.parse_status())?;
                while *parse_status.borrow() != language::ParseStatus::Idle {
                    parse_status.changed().await?;
                }

                buffer.read_with(cx, |buffer, _cx| buffer.snapshot())
            })
        });

        let parse_task = cx.background_spawn(async move {
            let snapshot = snapshot_task.await?;
            let rope = snapshot.as_rope();
            let declarations = declarations_in_buffer(&snapshot)
                .into_iter()
                .map(|item| (item.parent_index, FileDeclaration::from_outline(item, rope)))
                .collect::<Vec<_>>();
            anyhow::Ok(declarations)
        });

        let task = cx.spawn({
            async move |this, cx| {
                // TODO: how to handle errors?
                let Ok(declarations) = parse_task.await else {
                    return;
                };
                this.update(cx, |this, cx| {
                    this.with_state(cx, move |state| {
                        let file_state =
                            state.files.entry(entry_id).or_insert_with(Default::default);

                        for old_declaration_id in &file_state.declarations {
                            let Some(declaration) = state.declarations.remove(*old_declaration_id)
                            else {
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
                            declaration.parent = parent_index
                                .and_then(|ix| some_or_debug_panic(new_ids.get(ix).copied()));

                            let identifier = declaration.identifier.clone();
                            let declaration_id = state.declarations.insert(Declaration::File {
                                project_entry_id: entry_id,
                                declaration,
                            });
                            new_ids.push(declaration_id);

                            state
                                .identifiers
                                .entry(identifier)
                                .or_default()
                                .insert(declaration_id);
                        }

                        file_state.declarations = new_ids;
                    });
                })
                .ok();
            }
        });

        self.with_state(cx, move |state| {
            state
                .files
                .entry(entry_id)
                .or_insert_with(Default::default)
                .task = Some(task);
        });
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
    ) -> Vec<Declaration> {
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
                    result.push(declaration.clone());
                    if result.len() == N {
                        return Vec::new();
                    }
                }
                Declaration::File {
                    project_entry_id, ..
                } => {
                    if !included_buffer_entry_ids.contains(&project_entry_id) {
                        file_declarations.push(declaration.clone());
                    }
                }
            }
        }

        for declaration in file_declarations {
            match declaration {
                Declaration::File {
                    project_entry_id, ..
                } => {
                    if !included_buffer_entry_ids.contains(&project_entry_id) {
                        result.push(declaration);

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
    use std::{path::Path, sync::Arc};

    use gpui::TestAppContext;
    use indoc::indoc;
    use language::{Language, LanguageConfig, LanguageId, LanguageMatcher, tree_sitter_rust};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use text::OffsetRangeExt as _;
    use util::path;

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

            let decl = expect_file_decl("c.rs", &decls[0], &project, cx);
            assert_eq!(decl.identifier, main.clone());
            assert_eq!(decl.item_range_in_file, 32..280);

            let decl = expect_file_decl("a.rs", &decls[1], &project, cx);
            assert_eq!(decl.identifier, main);
            assert_eq!(decl.item_range_in_file, 0..98);
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

            let decl = expect_file_decl("c.rs", &decls[0], &project, cx);
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

            let decl = expect_buffer_decl("c.rs", &decls[0], &project, cx);
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
                let decl = expect_buffer_decl("c.rs", &decls[0], &project, cx);
                assert_eq!(decl.identifier, main);
                assert_eq!(decl.item_range.to_offset(&buffer.read(cx)), 32..280);

                expect_file_decl("a.rs", &decls[1], &project, cx);
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
            expect_file_decl("c.rs", &decls[0], &project, cx);
            expect_file_decl("a.rs", &decls[1], &project, cx);
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
            assert_eq!(project_path.path.as_ref(), Path::new(path),);
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
        } = declaration
        {
            assert_eq!(
                project
                    .read(cx)
                    .path_for_entry(*file, cx)
                    .unwrap()
                    .path
                    .as_ref(),
                Path::new(path),
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

        let index = cx.new(|cx| SyntaxIndex::new(&project, cx));
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
