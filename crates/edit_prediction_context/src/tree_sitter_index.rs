use anyhow::Result;
use collections::{HashMap, HashSet};
use gpui::{App, AppContext as _, Context, Entity, Task, WeakEntity};
use language::{Buffer, BufferEvent, BufferSnapshot, Language, OutlineItem};
use project::buffer_store::{BufferStore, BufferStoreEvent};
use project::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use project::{PathChange, Project, ProjectEntryId, ProjectPath};
use slotmap::SlotMap;
use std::ops::Range;
use std::sync::Arc;
use text::{Anchor, OffsetRangeExt as _};
use util::{ResultExt as _, debug_panic};

// To discuss: Strings in FileDeclaration?

// TODO:
//
// * Need an efficient way to get outline parents (see parents field / outline_id in
// `zeta_context/src/outline.rs`, as well as logic for figuring it out). Could be indexes into
// `declarations` instead of the OutlineId mechanism.
//
// * Skip for remote projects

// Potential future improvements:
//
// * Send multiple selected excerpt ranges. Challenge is that excerpt ranges influence which
// references are present and their scores.

// Potential future optimizations:
//
// * Cache of buffers for files
//
// * Parse files directly instead of loading into a Rope.
//
// * Use something similar to slotmap without key versions.
//
// * Concurrent slotmap

slotmap::new_key_type! {
    struct DeclarationId;
}

pub struct TreeSitterIndex {
    declarations: SlotMap<DeclarationId, Declaration>,
    identifiers: HashMap<Identifier, HashSet<DeclarationId>>,
    files: HashMap<ProjectEntryId, FileState>,
    buffers: HashMap<WeakEntity<Buffer>, BufferState>,
    project: WeakEntity<Project>,
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

#[derive(Debug, Clone)]
pub enum Declaration {
    File {
        project_entry_id: ProjectEntryId,
        declaration: FileDeclaration,
    },
    Buffer {
        buffer: WeakEntity<Buffer>,
        declaration: BufferDeclaration,
    },
}

impl Declaration {
    fn identifier(&self) -> &Identifier {
        match self {
            Declaration::File { declaration, .. } => &declaration.identifier,
            Declaration::Buffer { declaration, .. } => &declaration.identifier,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileDeclaration {
    identifier: Identifier,
    item_range: Range<usize>,
    annotation_range: Option<Range<usize>>,
    signature_range: Range<usize>,
    signature_text: String,
}

#[derive(Debug, Clone)]
pub struct BufferDeclaration {
    identifier: Identifier,
    item_range: Range<Anchor>,
    annotation_range: Option<Range<Anchor>>,
    signature_range: Range<Anchor>,
    signature_text: String,
}

pub struct DeclarationText {
    text: String,
    // Offset range within the `text` field containing the lines of the signature.
    signature_range: Range<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Identifier(Arc<str>);

impl<T: Into<Arc<str>>> From<T> for Identifier {
    fn from(value: T) -> Self {
        Identifier(value.into())
    }
}

impl TreeSitterIndex {
    pub fn new(project: &Entity<Project>, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            declarations: SlotMap::with_key(),
            identifiers: HashMap::default(),
            project: project.downgrade(),
            files: HashMap::default(),
            buffers: HashMap::default(),
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
            // todo! bg?
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

    pub fn declarations_for_identifier<const N: usize>(
        &self,
        identifier: impl Into<Identifier>,
        cx: &App,
    ) -> Vec<Declaration> {
        // make sure to not have a large stack allocation
        assert!(N < 32);

        let identifier = identifier.into();

        let Some(declaration_ids) = self.identifiers.get(&identifier) else {
            return vec![];
        };

        let mut result = Vec::with_capacity(N);
        let mut included_buffer_entry_ids = arrayvec::ArrayVec::<_, N>::new();
        let mut file_declarations = Vec::new();

        for declaration_id in declaration_ids {
            let declaration = self.declarations.get(*declaration_id);
            let Some(declaration) = declaration else {
                debug_panic!("bug: declaration not found");
                continue;
            };
            match declaration {
                Declaration::Buffer { buffer, .. } => {
                    if let Ok(Some(entry_id)) = buffer.read_with(cx, |buffer, cx| {
                        project::File::from_dyn(buffer.file()).and_then(|f| f.project_entry_id(cx))
                    }) {
                        included_buffer_entry_ids.push(entry_id);
                        result.push(declaration.clone());
                        if result.len() == N {
                            return result;
                        }
                    }
                }
                Declaration::File {
                    project_entry_id, ..
                } => {
                    if !included_buffer_entry_ids.contains(project_entry_id) {
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
                            return result;
                        }
                    }
                }
                Declaration::Buffer { .. } => {}
            }
        }

        result
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
                for (path, entry_id, path_change) in updated_entries_set.iter() {
                    if let PathChange::Removed = path_change {
                        self.files.remove(entry_id);
                    } else {
                        let project_path = ProjectPath {
                            worktree_id: *worktree_id,
                            path: path.clone(),
                        };
                        self.update_file(*entry_id, project_path, cx);
                    }
                }
            }
            WorktreeDeletedEntry(_worktree_id, project_entry_id) => {
                // TODO: Is this needed?
                self.files.remove(project_entry_id);
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

    fn register_buffer(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
        self.buffers
            .insert(buffer.downgrade(), BufferState::default());
        let weak_buf = buffer.downgrade();
        cx.observe_release(buffer, move |this, _buffer, _cx| {
            this.buffers.remove(&weak_buf);
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

    fn update_buffer(&mut self, buffer_entity: Entity<Buffer>, cx: &Context<Self>) {
        let buffer = buffer_entity.read(cx);

        let snapshot = buffer.snapshot();
        let parse_task: Task<Vec<BufferDeclaration>> = cx.background_spawn(async move {
            snapshot
                .outline(None)
                .items
                .into_iter()
                .filter_map(BufferDeclaration::try_from_outline_item)
                .collect()
        });

        let task = cx.spawn({
            let weak_buffer = buffer_entity.downgrade();
            async move |this, cx| {
                let declarations = parse_task.await;

                this.update(cx, |this, _cx| {
                    let buffer_state = this
                        .buffers
                        .entry(weak_buffer.clone())
                        .or_insert_with(Default::default);

                    for old_declaration_id in &buffer_state.declarations {
                        let Some(declaration) = this.declarations.remove(*old_declaration_id)
                        else {
                            debug_panic!("declaration not found");
                            continue;
                        };
                        if let Some(identifier_declarations) =
                            this.identifiers.get_mut(declaration.identifier())
                        {
                            identifier_declarations.remove(old_declaration_id);
                        }
                    }

                    let mut new_ids = Vec::with_capacity(declarations.len());
                    this.declarations.reserve(declarations.len());
                    for declaration in declarations {
                        let identifier = declaration.identifier.clone();
                        let declaration_id = this.declarations.insert(Declaration::Buffer {
                            buffer: weak_buffer.clone(),
                            declaration,
                        });
                        new_ids.push(declaration_id);
                        this.identifiers
                            .entry(identifier)
                            .or_default()
                            .insert(declaration_id);
                    }

                    buffer_state.declarations = new_ids;
                })
                .ok();
            }
        });

        self.buffers
            .entry(buffer_entity.downgrade())
            .or_insert_with(Default::default)
            .task = Some(task);
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
                    .log_err();

                let buffer = cx.new(|cx| {
                    let mut buffer = Buffer::local(loaded_file.text, cx);
                    buffer.set_language(language, cx);
                    buffer
                })?;
                buffer.read_with(cx, |buffer, _cx| buffer.snapshot())
            })
        });

        let parse_task: Task<Result<Vec<FileDeclaration>>> = cx.background_spawn(async move {
            let snapshot = snapshot_task.await?;
            Ok(snapshot
                .outline(None)
                .items
                .into_iter()
                .filter_map(BufferDeclaration::try_from_outline_item)
                .map(|declaration| declaration.into_file_declaration(&snapshot))
                .collect())
        });

        let task = cx.spawn({
            async move |this, cx| {
                // TODO: how to handle errors?
                let Ok(declarations) = parse_task.await else {
                    return;
                };
                this.update(cx, |this, _cx| {
                    let file_state = this.files.entry(entry_id).or_insert_with(Default::default);

                    for old_declaration_id in &file_state.declarations {
                        let Some(declaration) = this.declarations.remove(*old_declaration_id)
                        else {
                            debug_panic!("declaration not found");
                            continue;
                        };
                        if let Some(identifier_declarations) =
                            this.identifiers.get_mut(declaration.identifier())
                        {
                            identifier_declarations.remove(old_declaration_id);
                        }
                    }

                    let mut new_ids = Vec::with_capacity(declarations.len());
                    this.declarations.reserve(declarations.len());
                    for declaration in declarations {
                        let identifier = declaration.identifier.clone();
                        let declaration_id = this.declarations.insert(Declaration::File {
                            project_entry_id: entry_id,
                            declaration,
                        });
                        new_ids.push(declaration_id);
                        this.identifiers
                            .entry(identifier)
                            .or_default()
                            .insert(declaration_id);
                    }

                    file_state.declarations = new_ids;
                })
                .ok();
            }
        });

        self.files
            .entry(entry_id)
            .or_insert_with(Default::default)
            .task = Some(task);
    }
}

impl BufferDeclaration {
    pub fn try_from_outline_item(item: OutlineItem<Anchor>) -> Option<Self> {
        // todo! what to do about multiple names?
        let name_range = item.name_ranges.get(0)?;
        Some(BufferDeclaration {
            identifier: Identifier(item.text[name_range.clone()].into()),
            item_range: item.range,
            annotation_range: item.annotation_range,
            signature_range: item.signature_range?,
            // todo! this should instead be the signature_range but expanded to line boundaries.
            signature_text: item.text.clone(),
        })
    }

    pub fn into_file_declaration(self, snapshot: &BufferSnapshot) -> FileDeclaration {
        FileDeclaration {
            identifier: self.identifier,
            item_range: self.item_range.to_offset(snapshot),
            annotation_range: self.annotation_range.map(|range| range.to_offset(snapshot)),
            signature_range: self.signature_range.to_offset(snapshot),
            signature_text: self.signature_text.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::Path, sync::Arc};

    use futures::channel::oneshot;
    use gpui::TestAppContext;
    use indoc::indoc;
    use language::{Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};
    use project::{FakeFs, Project, ProjectItem};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    use crate::tree_sitter_index::TreeSitterIndex;

    #[gpui::test]
    async fn test_unopen_indexed_files(cx: &mut TestAppContext) {
        let (project, index) = init_test(cx).await;

        index.read_with(cx, |index, cx| {
            let decls = index.declarations_for_identifier::<8>("main", cx);
            assert_eq!(decls.len(), 2);

            let decl = expect_file_decl("c.rs", &decls[0], &project, cx);
            assert_eq!(decl.identifier, "main".into());
            assert_eq!(decl.item_range, 32..279);

            let decl = expect_file_decl("a.rs", &decls[1], &project, cx);
            assert_eq!(decl.identifier, "main".into());
            assert_eq!(decl.item_range, 0..97);
        });
    }

    #[gpui::test]
    async fn test_declarations_limt(cx: &mut TestAppContext) {
        let (_, index) = init_test(cx).await;

        // todo! test with buffers
        index.read_with(cx, |index, cx| {
            let decls = index.declarations_for_identifier::<1>("main", cx);
            assert_eq!(decls.len(), 1);
        });
    }

    #[gpui::test]
    async fn test_buffer_shadow(cx: &mut TestAppContext) {
        let (project, index) = init_test(cx).await;

        let buffer = project
            .update(cx, |project, cx| {
                let project_path = project.find_project_path("c.rs", cx).unwrap();
                project.open_buffer(project_path, cx)
            })
            .await
            .unwrap();

        cx.run_until_parked();

        index.read_with(cx, |index, cx| {
            let decls = index.declarations_for_identifier::<8>("main", cx);
            assert_eq!(decls.len(), 2);

            let decl = expect_buffer_decl("c.rs", &decls[0], cx);
            assert_eq!(decl.identifier, "main".into());
            assert_eq!(decl.item_range.to_offset(&buffer.read(cx)), 32..279);

            expect_file_decl("a.rs", &decls[1], &project, cx);
        });

        // Drop the buffer and wait for release
        let (release_tx, release_rx) = oneshot::channel();
        cx.update(|cx| {
            cx.observe_release(&buffer, |_, _| {
                release_tx.send(()).ok();
            })
            .detach();
        });
        drop(buffer);
        cx.run_until_parked();
        release_rx.await.ok();
        cx.run_until_parked();

        index.read_with(cx, |index, cx| {
            let decls = index.declarations_for_identifier::<8>("main", cx);
            assert_eq!(decls.len(), 2);
            expect_file_decl("c.rs", &decls[0], &project, cx);
            expect_file_decl("a.rs", &decls[1], &project, cx);
        });
    }

    fn expect_buffer_decl<'a>(
        path: &str,
        declaration: &'a Declaration,
        cx: &App,
    ) -> &'a BufferDeclaration {
        if let Declaration::Buffer {
            declaration,
            buffer,
        } = declaration
        {
            assert_eq!(
                buffer
                    .upgrade()
                    .unwrap()
                    .read(cx)
                    .project_path(cx)
                    .unwrap()
                    .path
                    .as_ref(),
                Path::new(path),
            );
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

    async fn init_test(cx: &mut TestAppContext) -> (Entity<Project>, Entity<TreeSitterIndex>) {
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
        language_registry.add(Arc::new(rust_lang()));

        let index = cx.new(|cx| TreeSitterIndex::new(&project, cx));
        cx.run_until_parked();

        (project, index)
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

/*
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
#[serde(transparent)]
pub struct Identifier(pub Arc<str>);

#[derive(Debug)]
pub struct IdentifierIndex {
    pub identifier_to_definitions:
        HashMap<(Identifier, LanguageName), MultiMap<Arc<Path>, OutlineItem>>,
    pub path_to_source: HashMap<Arc<Path>, String>,
    pub path_to_items: HashMap<Arc<Path>, Vec<OutlineItem>>,
    pub outline_id_to_item: HashMap<OutlineId, OutlineItem>,
}

impl IdentifierIndex {
    pub fn index_path(languages: &[Arc<Language>], path: &Path) -> Result<IdentifierIndex> {
        let mut identifier_to_definitions = HashMap::new();
        let mut path_to_source = HashMap::new();
        let mut path_to_items = HashMap::new();
        let mut outline_id_to_item = HashMap::new();

        for entry in Walk::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.metadata().unwrap().is_file())
        {
            let file_path = entry.path();
            let Some(language) = language_for_file(languages, file_path) else {
                continue;
            };
            if !language.supports_references {
                continue;
            }
            let source = fs::read_to_string(file_path)
                .map_err(|e| anyhow!("Failed to read file {:?}: {}", file_path, e))?;
            let tree = parse_source(&language, &source);

            let mut outline_items = query_outline_items(&language, &tree, &source);
            outline_items.sort_by_key(|item| item.item_range.start);
            for outline_item in outline_items.iter() {
                let identifier = Identifier(outline_item.name(&source).into());
                let definitions: &mut MultiMap<Arc<Path>, OutlineItem> = identifier_to_definitions
                    .entry((identifier, language.name.clone()))
                    .or_default();
                definitions.insert(file_path.into(), outline_item.clone());
                outline_id_to_item.insert(outline_item.id, outline_item.clone());
            }
            path_to_source.insert(file_path.into(), source);
            path_to_items.insert(file_path.into(), outline_items);
        }

        Ok(IdentifierIndex {
            identifier_to_definitions,
            path_to_source,
            path_to_items,
            outline_id_to_item,
        })
    }
}
*/
