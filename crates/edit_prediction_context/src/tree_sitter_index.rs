use collections::HashMap;
use gpui::{AppContext as _, Context, Entity, Task, WeakEntity};
use language::{Buffer, BufferEvent};
use project::buffer_store::{BufferStore, BufferStoreEvent};
use project::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use project::{PathChange, Project, ProjectEntryId, ProjectItem as _, ProjectPath};
use std::ops::Range;
use std::sync::Arc;
use text::Anchor;

pub struct TreeSitterIndex {
    files: HashMap<ProjectEntryId, FileState>,
    buffers: HashMap<WeakEntity<Buffer>, BufferState>,
    project: WeakEntity<Project>,
}

struct FileState {
    declarations: Vec<Declaration<usize>>,
}

#[derive(Default)]
struct BufferState {
    declarations: Vec<Declaration<Anchor>>,
    task: Option<Task<()>>,
}

pub struct Declaration<D> {
    identifier: Identifier,
    item_range: Range<D>,
    signature_range: Range<D>,
}

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
                        self.update_file(entry_id, project_path, cx);
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
            if let Some(file) = project::File::from_dyn(buffer.file())
                && let Some(entry_id) = file.project_entry_id(cx)
                && let Some(project_path) = buffer.project_path(cx)
            {
                this.update_file(&entry_id, project_path, cx);
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
                .filter_map(|item| {
                    // todo! what to do about multiple names?
                    let name_range = item.name_ranges.get(0)?;
                    Some(Declaration {
                        identifier: Identifier(item.text[name_range.clone()].into()),
                        item_range: item.range,
                        signature_range: item.signature_range?,
                    })
                })
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
        entry_id: &ProjectEntryId,
        project_path: ProjectPath,
        cx: &mut Context<Self>,
    ) {
        let Some(project) = self.project.upgrade() else {
            return;
        };

        let abs_path = project.read(cx).absolute_path(&project_path, cx);

        let fs = project.read(cx).fs();

        let parse_task = cx.background_spawn(async move {
            let file = fs.open_handle(&abs_path).await?;
            anyhow::Ok(())
        });
    }
}

// Subscriptions:
//
// - worktree_store -> updates files
// - subscribe to buffer creation and drop -> updates buffers
// - subscribe to buffer changes -> updates buffers
//
// How to shadow files?

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
