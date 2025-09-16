use anyhow::Result;
use collections::HashMap;
use gpui::{AppContext as _, Context, Entity, Task, WeakEntity};
use language::{Buffer, BufferEvent, BufferSnapshot, OutlineItem};
use project::buffer_store::{BufferStore, BufferStoreEvent};
use project::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use project::{PathChange, Project, ProjectEntryId, ProjectItem as _, ProjectPath};
use std::ops::Range;
use std::sync::Arc;
use text::{Anchor, OffsetRangeExt as _};

// TODO:
//
// * Need an efficient way to get outline parents (see parents field / outline_id in
// `zeta_context/src/outline.rs`, as well as logic for figuring it out). Could be indexes into
// `declarations` instead of the OutlineId mechanism.
//
// * Skip for remote projects

// Potential future optimizations:
//
// * Cache of buffers for files
//
// * Parse files directly instead of loading into a Rope.

pub struct TreeSitterIndex {
    files: HashMap<ProjectEntryId, FileState>,
    buffers: HashMap<WeakEntity<Buffer>, BufferState>,
    project: WeakEntity<Project>,
}

#[derive(Default)]
struct FileState {
    declarations: Vec<FileDeclaration>,
    task: Option<Task<()>>,
}

#[derive(Default)]
struct BufferState {
    declarations: Vec<BufferDeclaration>,
    task: Option<Task<()>>,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    File {
        file: ProjectEntryId,
        declaration: FileDeclaration,
    },
    Buffer {
        buffer: WeakEntity<Buffer>,
        declaration: BufferDeclaration,
    },
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

impl TreeSitterIndex {
    pub fn new(project: &Entity<Project>, cx: &mut Context<Self>) -> Self {
        cx.subscribe(
            &project.read(cx).worktree_store(),
            Self::handle_worktree_store_event,
        )
        .detach();
        let mut this = Self {
            project: project.downgrade(),
            files: HashMap::default(),
            buffers: HashMap::default(),
        };
        let buffer_store = project.read(cx).buffer_store().clone();
        for buffer in buffer_store.read(cx).buffers().collect::<Vec<_>>() {
            this.register_buffer(&buffer, cx);
        }
        cx.subscribe(&buffer_store, Self::handle_buffer_store_event)
            .detach();
        this
    }

    pub fn declarations_for_identifier(&self, identifier: Identifier) -> Vec<Declaration> {
        let mut declarations = Vec::new();

        for (buffer, buffer_state) in &self.buffers {
            for declaration in &buffer_state.declarations {
                if declaration.identifier == identifier {
                    declarations.push(Declaration::Buffer {
                        buffer: buffer.clone(),
                        declaration: declaration.clone(),
                    });
                }
            }
        }

        // todo! handle buffers shadowing files
        for (file, file_state) in &self.files {
            for declaration in &file_state.declarations {
                if declaration.identifier == identifier {
                    declarations.push(Declaration::File {
                        file: *file,
                        declaration: declaration.clone(),
                    });
                }
            }
        }

        declarations
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
        cx.observe_release(buffer, move |this, buffer, cx| {
            this.buffers.remove(&weak_buf);
            // todo! now that files and buffers are tracked separately, need to implement shadowing
            // logic and this file update is no longer needed.
            if let Some(file) = project::File::from_dyn(buffer.file())
                && let Some(entry_id) = file.project_entry_id(cx)
                && let Some(project_path) = buffer.project_path(cx)
            {
                this.update_file(entry_id, project_path, cx);
            }
        })
        .detach();
        cx.subscribe(buffer, Self::handle_buffer_event).detach();
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
        let parse_task = cx.background_spawn(async move {
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
                    this.buffers
                        .entry(weak_buffer)
                        .or_insert_with(Default::default)
                        .declarations = declarations;
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

        let snapshot_task = worktree.update(cx, |worktree, cx| {
            let load_task = worktree.load_file(&project_path.path, cx);
            cx.spawn(async move |_this, cx| {
                let loaded_file = load_task.await?;
                let buffer = cx.new(|cx| Buffer::local(loaded_file.text, cx))?;
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
                .map(|declaration| declaration.into_buffer_declaration(&snapshot))
                .collect())
        });

        let task = cx.spawn({
            async move |this, cx| {
                // TODO: how to handle errors?
                let Ok(declarations) = parse_task.await else {
                    return;
                };
                this.update(cx, |this, _cx| {
                    this.files
                        .entry(entry_id)
                        .or_insert_with(Default::default)
                        .declarations = declarations;
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

    pub fn into_buffer_declaration(self, snapshot: &BufferSnapshot) -> FileDeclaration {
        FileDeclaration {
            identifier: self.identifier,
            item_range: self.item_range.to_offset(snapshot),
            annotation_range: self.annotation_range.map(|range| range.to_offset(snapshot)),
            signature_range: self.signature_range.to_offset(snapshot),
            signature_text: self.signature_text.clone(),
        }
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
