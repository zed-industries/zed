use std::collections::hash_map;

use cloud_llm_client::{PredictEditsEvent, PredictEditsGitInfo, SerializedJson};
use collections::{HashMap, HashSet};
use fs::MTime;
use gpui::{AppContext as _, Context, Entity, EntityId, Task, WeakEntity};
use language::{Buffer, BufferEvent};
use project::{
    Project, ProjectEntryId, ProjectPath,
    buffer_store::{BufferStore, BufferStoreEvent},
    git_store::{GitStore, GitStoreEvent, Repository, RepositoryId},
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};
use uuid::Uuid;

use crate::license_detection::LicenseDetectionWatcher;

// todos:
//
// * Don't subscribe to all buffers
//
// * Currently MoveCursor event will only happen for edit prediction requests.

pub struct TrainingDataUploader {
    projects: HashMap<EntityId, Entity<ZetaProject>>,
    _upload_task: Task<()>,
}

struct ZetaProject {
    project: WeakEntity<Project>,
    repositories: HashMap<RepositoryId, Entity<ZetaRepository>>,
    buffers_changed: HashSet<WeakEntity<Buffer>>,
    project_entries_changed: HashSet<ProjectEntryId>,
}

struct ZetaRepository {
    unsent_events: Vec<SerializedJson<PredictEditsEvent>>,
    pending_event: Option<PredictEditsEvent>,
    last_snapshot: Option<ZetaRepositorySnapshot>,
    license_watcher: LicenseDetectionWatcher,
}

struct ZetaRepositorySnapshot {
    request_id: Uuid,
    git_info: PredictEditsGitInfo,
    buffers: HashMap<ProjectEntryId, ZetaBufferSnapshot>,
    files: HashMap<ProjectEntryId, ZetaFileSnapshot>,
}

struct ZetaBufferSnapshot {
    path: ProjectPath,
    text: String,
    buffer: WeakEntity<Buffer>,
    version: clock::Global,
}

struct ZetaFileSnapshot {
    path: ProjectPath,
    text: String,
    mtime: MTime,
}

impl TrainingDataUploader {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let _upload_task = cx.spawn(|this, cx| {
            loop {
                todo!();
            }
        });
        Self {
            projects: HashMap::default(),
            _upload_task,
        }
    }

    fn register(&mut self, project: &Entity<Project>, path: ProjectPath, cx: &mut Context<Self>) {
        let project_entity_id = project.entity_id();

        let zeta_project = match self.projects.entry(project_entity_id) {
            hash_map::Entry::Vacant(entry) => {
                let zeta_project = cx.new(|cx| ZetaProject::new(project, cx));
                cx.observe_release(project, move |this, project, cx| {
                    this.projects.remove(&project_entity_id);
                });
                entry.insert(zeta_project)
            }
            hash_map::Entry::Occupied(entry) => entry.into_mut(),
        };

        // todo!
        // zeta_project.update(|zeta_project, cx| zeta_project.register(path, cx));
    }
}

impl ZetaProject {
    pub fn new(project: &Entity<Project>, cx: &mut Context<Self>) -> Self {
        cx.subscribe(&project, Self::handle_project_event).detach();
        cx.subscribe(
            &project.read(cx).git_store().clone(),
            Self::handle_git_store_event,
        )
        .detach();
        cx.subscribe(
            &project.read(cx).worktree_store(),
            Self::handle_worktree_store_event,
        )
        .detach();

        let buffer_store = project.read(cx).buffer_store().clone();
        for buffer in buffer_store.read(cx).buffers().collect::<Vec<_>>() {
            Self::register_buffer(&buffer, cx);
        }
        cx.subscribe(&buffer_store, Self::handle_buffer_store_event)
            .detach();

        Self {
            project: project.downgrade(),
            repositories: HashMap::default(),
            buffers_changed: HashSet::default(),
            project_entries_changed: HashSet::default(),
        }
    }

    fn handle_git_store_event(
        &mut self,
        _git_store: Entity<GitStore>,
        event: &GitStoreEvent,
        cx: &mut Context<Self>,
    ) {
        use GitStoreEvent::*;
        match event {
            RepositoryRemoved(repository_id) => {
                self.repositories.remove(&repository_id);
            }
            RepositoryAdded(repository_id) => {
                self.repositories
                    .insert(*repository_id, cx.new(|cx| ZetaRepository::new(cx)));
            }
            RepositoryUpdated(repository_id, event, is_active) => {}
            ActiveRepositoryChanged { .. }
            | IndexWriteError { .. }
            | JobsUpdated
            | ConflictsUpdated => {}
        }
    }

    fn handle_worktree_store_event(
        &mut self,
        _worktree_store: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        use WorktreeStoreEvent::*;
        match event {
            WorktreeAdded(worktree) => {}
            WorktreeRemoved(worktree_entity_id, worktree_id) => {}
            WorktreeUpdatedEntries(worktree_id, updated_entries_set) => {
                for (path, entry_id, _path_change) in updated_entries_set.iter() {
                    self.project_entries_changed.insert(*entry_id);
                }
            }
            WorktreeUpdatedGitRepositories(worktree_id, updated_git_repositories) => {}
            WorktreeDeletedEntry(worktree_id, project_entry_id) => {}
            WorktreeReleased { .. } | WorktreeOrderChanged | WorktreeUpdateSent { .. } => {}
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
            BufferAdded(buffer) => Self::register_buffer(buffer, cx),
            BufferOpened { .. }
            | BufferChangedFilePath { .. }
            | BufferDropped { .. }
            | SharedBufferClosed { .. } => {}
        }
    }

    fn register_buffer(buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
        cx.subscribe(buffer, Self::handle_buffer_event);
    }

    fn handle_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &BufferEvent,
        _cx: &mut Context<Self>,
    ) {
        match event {
            BufferEvent::Edited => {
                self.buffers_changed.insert(buffer.downgrade());
            }
            _ => {}
        }
    }

    fn handle_project_event(
        &mut self,
        _project: Entity<Project>,
        event: &project::Event,
        cx: &mut Context<Self>,
    ) {
        match event {
            project::Event::ActiveEntryChanged(entry_id) => {
                todo!()
            }
            _ => {}
        }
    }
}

impl ZetaRepository {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            unsent_events: Vec::new(),
            pending_event: None,
            last_snapshot: None,
            license_watcher: LicenseDetectionWatcher::new(cx),
        }
    }
}
