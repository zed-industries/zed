use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};

use anyhow::{Context as _, Result, anyhow, bail};
use collections::HashMap;
use fs::{Fs, copy_recursive};
use futures::{FutureExt, future::Shared};
use gpui::{
    App, AppContext as _, AsyncApp, Context, Entity, EntityId, EventEmitter, Task, WeakEntity,
};
use rpc::{
    AnyProtoClient, ErrorExt, TypedEnvelope,
    proto::{self, REMOTE_SERVER_PROJECT_ID},
};
use text::ReplicaId;
use util::{
    ResultExt,
    paths::{PathStyle, RemotePathBuf, SanitizedPath},
    rel_path::RelPath,
};
use worktree::{
    CreatedEntry, Entry, ProjectEntryId, UpdatedEntriesSet, UpdatedGitRepositoriesSet, Worktree,
    WorktreeId,
};

use crate::{ProjectPath, trusted_worktrees::TrustedWorktrees};

enum WorktreeStoreState {
    Local {
        fs: Arc<dyn Fs>,
    },
    Remote {
        upstream_client: AnyProtoClient,
        upstream_project_id: u64,
        path_style: PathStyle,
    },
}

pub struct WorktreeStore {
    next_entry_id: Arc<AtomicUsize>,
    downstream_client: Option<(AnyProtoClient, u64)>,
    retain_worktrees: bool,
    worktrees: Vec<WorktreeHandle>,
    worktrees_reordered: bool,
    scanning_enabled: bool,
    #[allow(clippy::type_complexity)]
    loading_worktrees:
        HashMap<Arc<SanitizedPath>, Shared<Task<Result<Entity<Worktree>, Arc<anyhow::Error>>>>>,
    state: WorktreeStoreState,
}

#[derive(Debug)]
pub enum WorktreeStoreEvent {
    WorktreeAdded(Entity<Worktree>),
    WorktreeRemoved(EntityId, WorktreeId),
    WorktreeReleased(EntityId, WorktreeId),
    WorktreeOrderChanged,
    WorktreeUpdateSent(Entity<Worktree>),
    WorktreeUpdatedEntries(WorktreeId, UpdatedEntriesSet),
    WorktreeUpdatedGitRepositories(WorktreeId, UpdatedGitRepositoriesSet),
    WorktreeDeletedEntry(WorktreeId, ProjectEntryId),
}

impl EventEmitter<WorktreeStoreEvent> for WorktreeStore {}

impl WorktreeStore {
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_request_handler(Self::handle_create_project_entry);
        client.add_entity_request_handler(Self::handle_copy_project_entry);
        client.add_entity_request_handler(Self::handle_delete_project_entry);
        client.add_entity_request_handler(Self::handle_expand_project_entry);
        client.add_entity_request_handler(Self::handle_expand_all_for_project_entry);
    }

    pub fn local(retain_worktrees: bool, fs: Arc<dyn Fs>) -> Self {
        Self {
            next_entry_id: Default::default(),
            loading_worktrees: Default::default(),
            downstream_client: None,
            worktrees: Vec::new(),
            worktrees_reordered: false,
            scanning_enabled: true,
            retain_worktrees,
            state: WorktreeStoreState::Local { fs },
        }
    }

    pub fn remote(
        retain_worktrees: bool,
        upstream_client: AnyProtoClient,
        upstream_project_id: u64,
        path_style: PathStyle,
    ) -> Self {
        Self {
            next_entry_id: Default::default(),
            loading_worktrees: Default::default(),
            downstream_client: None,
            worktrees: Vec::new(),
            worktrees_reordered: false,
            scanning_enabled: true,
            retain_worktrees,
            state: WorktreeStoreState::Remote {
                upstream_client,
                upstream_project_id,
                path_style,
            },
        }
    }

    pub fn disable_scanner(&mut self) {
        self.scanning_enabled = false;
    }

    /// Iterates through all worktrees, including ones that don't appear in the project panel
    pub fn worktrees(&self) -> impl '_ + DoubleEndedIterator<Item = Entity<Worktree>> {
        self.worktrees
            .iter()
            .filter_map(move |worktree| worktree.upgrade())
    }

    /// Iterates through all user-visible worktrees, the ones that appear in the project panel.
    pub fn visible_worktrees<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl 'a + DoubleEndedIterator<Item = Entity<Worktree>> {
        self.worktrees()
            .filter(|worktree| worktree.read(cx).is_visible())
    }

    /// Iterates through all user-visible worktrees (directories and files that appear in the project panel) and other, invisible single files that could appear e.g. due to drag and drop.
    pub fn visible_worktrees_and_single_files<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl 'a + DoubleEndedIterator<Item = Entity<Worktree>> {
        self.worktrees()
            .filter(|worktree| worktree.read(cx).is_visible() || worktree.read(cx).is_single_file())
    }

    pub fn worktree_for_id(&self, id: WorktreeId, cx: &App) -> Option<Entity<Worktree>> {
        self.worktrees()
            .find(|worktree| worktree.read(cx).id() == id)
    }

    pub fn worktree_for_entry(
        &self,
        entry_id: ProjectEntryId,
        cx: &App,
    ) -> Option<Entity<Worktree>> {
        self.worktrees()
            .find(|worktree| worktree.read(cx).contains_entry(entry_id))
    }

    pub fn find_worktree(
        &self,
        abs_path: impl AsRef<Path>,
        cx: &App,
    ) -> Option<(Entity<Worktree>, Arc<RelPath>)> {
        let abs_path = SanitizedPath::new(abs_path.as_ref());
        for tree in self.worktrees() {
            let path_style = tree.read(cx).path_style();
            if let Ok(relative_path) = abs_path.as_ref().strip_prefix(tree.read(cx).abs_path())
                && let Ok(relative_path) = RelPath::new(relative_path, path_style)
            {
                return Some((tree.clone(), relative_path.into_arc()));
            }
        }
        None
    }

    pub fn project_path_for_absolute_path(&self, abs_path: &Path, cx: &App) -> Option<ProjectPath> {
        self.find_worktree(abs_path, cx)
            .map(|(worktree, relative_path)| ProjectPath {
                worktree_id: worktree.read(cx).id(),
                path: relative_path,
            })
    }

    pub fn absolutize(&self, project_path: &ProjectPath, cx: &App) -> Option<PathBuf> {
        let worktree = self.worktree_for_id(project_path.worktree_id, cx)?;
        Some(worktree.read(cx).absolutize(&project_path.path))
    }

    pub fn path_style(&self) -> PathStyle {
        match &self.state {
            WorktreeStoreState::Local { .. } => PathStyle::local(),
            WorktreeStoreState::Remote { path_style, .. } => *path_style,
        }
    }

    pub fn find_or_create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Entity<Worktree>, Arc<RelPath>)>> {
        let abs_path = abs_path.as_ref();
        if let Some((tree, relative_path)) = self.find_worktree(abs_path, cx) {
            Task::ready(Ok((tree, relative_path)))
        } else {
            let worktree = self.create_worktree(abs_path, visible, cx);
            cx.background_spawn(async move { Ok((worktree.await?, RelPath::empty().into())) })
        }
    }

    pub fn entry_for_id<'a>(&'a self, entry_id: ProjectEntryId, cx: &'a App) -> Option<&'a Entry> {
        self.worktrees()
            .find_map(|worktree| worktree.read(cx).entry_for_id(entry_id))
    }

    pub fn worktree_and_entry_for_id<'a>(
        &'a self,
        entry_id: ProjectEntryId,
        cx: &'a App,
    ) -> Option<(Entity<Worktree>, &'a Entry)> {
        self.worktrees().find_map(|worktree| {
            worktree
                .read(cx)
                .entry_for_id(entry_id)
                .map(|e| (worktree.clone(), e))
        })
    }

    pub fn entry_for_path<'a>(&'a self, path: &ProjectPath, cx: &'a App) -> Option<&'a Entry> {
        self.worktree_for_id(path.worktree_id, cx)?
            .read(cx)
            .entry_for_path(&path.path)
    }

    pub fn copy_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_project_path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Entry>>> {
        let Some(old_worktree) = self.worktree_for_entry(entry_id, cx) else {
            return Task::ready(Err(anyhow!("no such worktree")));
        };
        let Some(old_entry) = old_worktree.read(cx).entry_for_id(entry_id) else {
            return Task::ready(Err(anyhow!("no such entry")));
        };
        let Some(new_worktree) = self.worktree_for_id(new_project_path.worktree_id, cx) else {
            return Task::ready(Err(anyhow!("no such worktree")));
        };

        match &self.state {
            WorktreeStoreState::Local { fs } => {
                let old_abs_path = old_worktree.read(cx).absolutize(&old_entry.path);
                let new_abs_path = new_worktree.read(cx).absolutize(&new_project_path.path);
                let fs = fs.clone();
                let copy = cx.background_spawn(async move {
                    copy_recursive(
                        fs.as_ref(),
                        &old_abs_path,
                        &new_abs_path,
                        Default::default(),
                    )
                    .await
                });

                cx.spawn(async move |_, cx| {
                    copy.await?;
                    new_worktree
                        .update(cx, |this, cx| {
                            this.as_local_mut().unwrap().refresh_entry(
                                new_project_path.path,
                                None,
                                cx,
                            )
                        })
                        .await
                })
            }
            WorktreeStoreState::Remote {
                upstream_client,
                upstream_project_id,
                ..
            } => {
                let response = upstream_client.request(proto::CopyProjectEntry {
                    project_id: *upstream_project_id,
                    entry_id: entry_id.to_proto(),
                    new_path: new_project_path.path.to_proto(),
                    new_worktree_id: new_project_path.worktree_id.to_proto(),
                });
                cx.spawn(async move |_, cx| {
                    let response = response.await?;
                    match response.entry {
                        Some(entry) => new_worktree
                            .update(cx, |worktree, cx| {
                                worktree.as_remote_mut().unwrap().insert_entry(
                                    entry,
                                    response.worktree_scan_id as usize,
                                    cx,
                                )
                            })
                            .await
                            .map(Some),
                        None => Ok(None),
                    }
                })
            }
        }
    }

    pub fn rename_entry(
        &mut self,
        entry_id: ProjectEntryId,
        new_project_path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<CreatedEntry>> {
        let Some(old_worktree) = self.worktree_for_entry(entry_id, cx) else {
            return Task::ready(Err(anyhow!("no such worktree")));
        };
        let Some(old_entry) = old_worktree.read(cx).entry_for_id(entry_id).cloned() else {
            return Task::ready(Err(anyhow!("no such entry")));
        };
        let Some(new_worktree) = self.worktree_for_id(new_project_path.worktree_id, cx) else {
            return Task::ready(Err(anyhow!("no such worktree")));
        };

        match &self.state {
            WorktreeStoreState::Local { fs } => {
                let abs_old_path = old_worktree.read(cx).absolutize(&old_entry.path);
                let new_worktree_ref = new_worktree.read(cx);
                let is_root_entry = new_worktree_ref
                    .root_entry()
                    .is_some_and(|e| e.id == entry_id);
                let abs_new_path = if is_root_entry {
                    let abs_path = new_worktree_ref.abs_path();
                    let Some(root_parent_path) = abs_path.parent() else {
                        return Task::ready(Err(anyhow!("no parent for path {:?}", abs_path)));
                    };
                    root_parent_path.join(new_project_path.path.as_std_path())
                } else {
                    new_worktree_ref.absolutize(&new_project_path.path)
                };

                let fs = fs.clone();
                let case_sensitive = new_worktree
                    .read(cx)
                    .as_local()
                    .unwrap()
                    .fs_is_case_sensitive();

                let do_rename =
                    async move |fs: &dyn Fs, old_path: &Path, new_path: &Path, overwrite| {
                        fs.rename(
                            &old_path,
                            &new_path,
                            fs::RenameOptions {
                                overwrite,
                                ..fs::RenameOptions::default()
                            },
                        )
                        .await
                        .with_context(|| format!("renaming {old_path:?} into {new_path:?}"))
                    };

                let rename = cx.background_spawn({
                    let abs_new_path = abs_new_path.clone();
                    async move {
                        // If we're on a case-insensitive FS and we're doing a case-only rename (i.e. `foobar` to `FOOBAR`)
                        // we want to overwrite, because otherwise we run into a file-already-exists error.
                        let overwrite = !case_sensitive
                            && abs_old_path != abs_new_path
                            && abs_old_path.to_str().map(|p| p.to_lowercase())
                                == abs_new_path.to_str().map(|p| p.to_lowercase());

                        // The directory we're renaming into might not exist yet
                        if let Err(e) =
                            do_rename(fs.as_ref(), &abs_old_path, &abs_new_path, overwrite).await
                        {
                            if let Some(err) = e.downcast_ref::<std::io::Error>()
                                && err.kind() == std::io::ErrorKind::NotFound
                            {
                                if let Some(parent) = abs_new_path.parent() {
                                    fs.create_dir(parent).await.with_context(|| {
                                        format!("creating parent directory {parent:?}")
                                    })?;
                                    return do_rename(
                                        fs.as_ref(),
                                        &abs_old_path,
                                        &abs_new_path,
                                        overwrite,
                                    )
                                    .await;
                                }
                            }
                            return Err(e);
                        }
                        Ok(())
                    }
                });

                cx.spawn(async move |_, cx| {
                    rename.await?;
                    Ok(new_worktree
                        .update(cx, |this, cx| {
                            let local = this.as_local_mut().unwrap();
                            if is_root_entry {
                                // We eagerly update `abs_path` and refresh this worktree.
                                // Otherwise, the FS watcher would do it on the `RootUpdated` event,
                                // but with a noticeable delay, so we handle it proactively.
                                local.update_abs_path_and_refresh(
                                    SanitizedPath::new_arc(&abs_new_path),
                                    cx,
                                );
                                Task::ready(Ok(this.root_entry().cloned()))
                            } else {
                                // First refresh the parent directory (in case it was newly created)
                                if let Some(parent) = new_project_path.path.parent() {
                                    let _ = local.refresh_entries_for_paths(vec![parent.into()]);
                                }
                                // Then refresh the new path
                                local.refresh_entry(
                                    new_project_path.path.clone(),
                                    Some(old_entry.path),
                                    cx,
                                )
                            }
                        })
                        .await?
                        .map(CreatedEntry::Included)
                        .unwrap_or_else(|| CreatedEntry::Excluded {
                            abs_path: abs_new_path,
                        }))
                })
            }
            WorktreeStoreState::Remote {
                upstream_client,
                upstream_project_id,
                ..
            } => {
                let response = upstream_client.request(proto::RenameProjectEntry {
                    project_id: *upstream_project_id,
                    entry_id: entry_id.to_proto(),
                    new_path: new_project_path.path.to_proto(),
                    new_worktree_id: new_project_path.worktree_id.to_proto(),
                });
                cx.spawn(async move |_, cx| {
                    let response = response.await?;
                    match response.entry {
                        Some(entry) => new_worktree
                            .update(cx, |worktree, cx| {
                                worktree.as_remote_mut().unwrap().insert_entry(
                                    entry,
                                    response.worktree_scan_id as usize,
                                    cx,
                                )
                            })
                            .await
                            .map(CreatedEntry::Included),
                        None => {
                            let abs_path = new_worktree.read_with(cx, |worktree, _| {
                                worktree.absolutize(&new_project_path.path)
                            });
                            Ok(CreatedEntry::Excluded { abs_path })
                        }
                    }
                })
            }
        }
    }
    pub fn create_worktree(
        &mut self,
        abs_path: impl AsRef<Path>,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Worktree>>> {
        let abs_path: Arc<SanitizedPath> = SanitizedPath::new_arc(&abs_path);
        let is_via_collab = matches!(&self.state, WorktreeStoreState::Remote { upstream_client, .. } if upstream_client.is_via_collab());
        if !self.loading_worktrees.contains_key(&abs_path) {
            let task = match &self.state {
                WorktreeStoreState::Remote {
                    upstream_client,
                    path_style,
                    ..
                } => {
                    if upstream_client.is_via_collab() {
                        Task::ready(Err(Arc::new(anyhow!("cannot create worktrees via collab"))))
                    } else {
                        let abs_path = RemotePathBuf::new(abs_path.to_string(), *path_style);
                        self.create_remote_worktree(upstream_client.clone(), abs_path, visible, cx)
                    }
                }
                WorktreeStoreState::Local { fs } => {
                    self.create_local_worktree(fs.clone(), abs_path.clone(), visible, cx)
                }
            };

            self.loading_worktrees
                .insert(abs_path.clone(), task.shared());
        }
        let task = self.loading_worktrees.get(&abs_path).unwrap().clone();
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, _| this.loading_worktrees.remove(&abs_path))
                .ok();
            match result {
                Ok(worktree) => {
                    if !is_via_collab {
                        if let Some((trusted_worktrees, worktree_store)) = this
                            .update(cx, |_, cx| {
                                TrustedWorktrees::try_get_global(cx).zip(Some(cx.entity()))
                            })
                            .ok()
                            .flatten()
                        {
                            trusted_worktrees.update(cx, |trusted_worktrees, cx| {
                                trusted_worktrees.can_trust(
                                    &worktree_store,
                                    worktree.read(cx).id(),
                                    cx,
                                );
                            });
                        }
                    }
                    Ok(worktree)
                }
                Err(err) => Err((*err).cloned()),
            }
        })
    }

    fn create_remote_worktree(
        &mut self,
        client: AnyProtoClient,
        abs_path: RemotePathBuf,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Worktree>, Arc<anyhow::Error>>> {
        let path_style = abs_path.path_style();
        let mut abs_path = abs_path.to_string();
        // If we start with `/~` that means the ssh path was something like `ssh://user@host/~/home-dir-folder/`
        // in which case want to strip the leading the `/`.
        // On the host-side, the `~` will get expanded.
        // That's what git does too: https://github.com/libgit2/libgit2/issues/3345#issuecomment-127050850
        if abs_path.starts_with("/~") {
            abs_path = abs_path[1..].to_string();
        }
        if abs_path.is_empty() {
            abs_path = "~/".to_string();
        }

        cx.spawn(async move |this, cx| {
            let this = this.upgrade().context("Dropped worktree store")?;

            let path = RemotePathBuf::new(abs_path, path_style);
            let response = client
                .request(proto::AddWorktree {
                    project_id: REMOTE_SERVER_PROJECT_ID,
                    path: path.to_proto(),
                    visible,
                })
                .await?;

            if let Some(existing_worktree) = this.read_with(cx, |this, cx| {
                this.worktree_for_id(WorktreeId::from_proto(response.worktree_id), cx)
            }) {
                return Ok(existing_worktree);
            }

            let root_path_buf = PathBuf::from(response.canonicalized_path.clone());
            let root_name = root_path_buf
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or(root_path_buf.to_string_lossy().into_owned());

            let worktree = cx.update(|cx| {
                Worktree::remote(
                    REMOTE_SERVER_PROJECT_ID,
                    ReplicaId::REMOTE_SERVER,
                    proto::WorktreeMetadata {
                        id: response.worktree_id,
                        root_name,
                        visible,
                        abs_path: response.canonicalized_path,
                    },
                    client,
                    path_style,
                    cx,
                )
            });

            this.update(cx, |this, cx| {
                this.add(&worktree, cx);
            });
            Ok(worktree)
        })
    }

    fn create_local_worktree(
        &mut self,
        fs: Arc<dyn Fs>,
        abs_path: Arc<SanitizedPath>,
        visible: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Worktree>, Arc<anyhow::Error>>> {
        let next_entry_id = self.next_entry_id.clone();
        let scanning_enabled = self.scanning_enabled;

        cx.spawn(async move |this, cx| {
            let worktree = Worktree::local(
                SanitizedPath::cast_arc(abs_path.clone()),
                visible,
                fs,
                next_entry_id,
                scanning_enabled,
                cx,
            )
            .await?;

            this.update(cx, |this, cx| this.add(&worktree, cx))?;

            if visible {
                cx.update(|cx| {
                    cx.add_recent_document(abs_path.as_path());
                });
            }

            Ok(worktree)
        })
    }

    pub fn add(&mut self, worktree: &Entity<Worktree>, cx: &mut Context<Self>) {
        let worktree_id = worktree.read(cx).id();
        debug_assert!(self.worktrees().all(|w| w.read(cx).id() != worktree_id));

        let push_strong_handle = self.retain_worktrees || worktree.read(cx).is_visible();
        let handle = if push_strong_handle {
            WorktreeHandle::Strong(worktree.clone())
        } else {
            WorktreeHandle::Weak(worktree.downgrade())
        };
        if self.worktrees_reordered {
            self.worktrees.push(handle);
        } else {
            let i = match self
                .worktrees
                .binary_search_by_key(&Some(worktree.read(cx).abs_path()), |other| {
                    other.upgrade().map(|worktree| worktree.read(cx).abs_path())
                }) {
                Ok(i) | Err(i) => i,
            };
            self.worktrees.insert(i, handle);
        }

        cx.emit(WorktreeStoreEvent::WorktreeAdded(worktree.clone()));
        self.send_project_updates(cx);

        let handle_id = worktree.entity_id();
        cx.subscribe(worktree, |_, worktree, event, cx| {
            let worktree_id = worktree.read(cx).id();
            match event {
                worktree::Event::UpdatedEntries(changes) => {
                    cx.emit(WorktreeStoreEvent::WorktreeUpdatedEntries(
                        worktree_id,
                        changes.clone(),
                    ));
                }
                worktree::Event::UpdatedGitRepositories(set) => {
                    cx.emit(WorktreeStoreEvent::WorktreeUpdatedGitRepositories(
                        worktree_id,
                        set.clone(),
                    ));
                }
                worktree::Event::DeletedEntry(id) => {
                    cx.emit(WorktreeStoreEvent::WorktreeDeletedEntry(worktree_id, *id))
                }
            }
        })
        .detach();
        cx.observe_release(worktree, move |this, worktree, cx| {
            cx.emit(WorktreeStoreEvent::WorktreeReleased(
                handle_id,
                worktree.id(),
            ));
            cx.emit(WorktreeStoreEvent::WorktreeRemoved(
                handle_id,
                worktree.id(),
            ));
            this.send_project_updates(cx);
        })
        .detach();
    }

    pub fn remove_worktree(&mut self, id_to_remove: WorktreeId, cx: &mut Context<Self>) {
        self.worktrees.retain(|worktree| {
            if let Some(worktree) = worktree.upgrade() {
                if worktree.read(cx).id() == id_to_remove {
                    cx.emit(WorktreeStoreEvent::WorktreeRemoved(
                        worktree.entity_id(),
                        id_to_remove,
                    ));
                    false
                } else {
                    true
                }
            } else {
                false
            }
        });
        self.send_project_updates(cx);
    }

    pub fn set_worktrees_reordered(&mut self, worktrees_reordered: bool) {
        self.worktrees_reordered = worktrees_reordered;
    }

    fn upstream_client(&self) -> Option<(AnyProtoClient, u64)> {
        match &self.state {
            WorktreeStoreState::Remote {
                upstream_client,
                upstream_project_id,
                ..
            } => Some((upstream_client.clone(), *upstream_project_id)),
            WorktreeStoreState::Local { .. } => None,
        }
    }

    pub fn set_worktrees_from_proto(
        &mut self,
        worktrees: Vec<proto::WorktreeMetadata>,
        replica_id: ReplicaId,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let mut old_worktrees_by_id = self
            .worktrees
            .drain(..)
            .filter_map(|worktree| {
                let worktree = worktree.upgrade()?;
                Some((worktree.read(cx).id(), worktree))
            })
            .collect::<HashMap<_, _>>();

        let (client, project_id) = self.upstream_client().context("invalid project")?;

        for worktree in worktrees {
            if let Some(old_worktree) =
                old_worktrees_by_id.remove(&WorktreeId::from_proto(worktree.id))
            {
                let push_strong_handle =
                    self.retain_worktrees || old_worktree.read(cx).is_visible();
                let handle = if push_strong_handle {
                    WorktreeHandle::Strong(old_worktree.clone())
                } else {
                    WorktreeHandle::Weak(old_worktree.downgrade())
                };
                self.worktrees.push(handle);
            } else {
                self.add(
                    &Worktree::remote(
                        project_id,
                        replica_id,
                        worktree,
                        client.clone(),
                        self.path_style(),
                        cx,
                    ),
                    cx,
                );
            }
        }
        self.send_project_updates(cx);

        Ok(())
    }

    pub fn move_worktree(
        &mut self,
        source: WorktreeId,
        destination: WorktreeId,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if source == destination {
            return Ok(());
        }

        let mut source_index = None;
        let mut destination_index = None;
        for (i, worktree) in self.worktrees.iter().enumerate() {
            if let Some(worktree) = worktree.upgrade() {
                let worktree_id = worktree.read(cx).id();
                if worktree_id == source {
                    source_index = Some(i);
                    if destination_index.is_some() {
                        break;
                    }
                } else if worktree_id == destination {
                    destination_index = Some(i);
                    if source_index.is_some() {
                        break;
                    }
                }
            }
        }

        let source_index =
            source_index.with_context(|| format!("Missing worktree for id {source}"))?;
        let destination_index =
            destination_index.with_context(|| format!("Missing worktree for id {destination}"))?;

        if source_index == destination_index {
            return Ok(());
        }

        let worktree_to_move = self.worktrees.remove(source_index);
        self.worktrees.insert(destination_index, worktree_to_move);
        self.worktrees_reordered = true;
        cx.emit(WorktreeStoreEvent::WorktreeOrderChanged);
        cx.notify();
        Ok(())
    }

    pub fn disconnected_from_host(&mut self, cx: &mut App) {
        for worktree in &self.worktrees {
            if let Some(worktree) = worktree.upgrade() {
                worktree.update(cx, |worktree, _| {
                    if let Some(worktree) = worktree.as_remote_mut() {
                        worktree.disconnected_from_host();
                    }
                });
            }
        }
    }

    pub fn send_project_updates(&mut self, cx: &mut Context<Self>) {
        let Some((downstream_client, project_id)) = self.downstream_client.clone() else {
            return;
        };

        let update = proto::UpdateProject {
            project_id,
            worktrees: self.worktree_metadata_protos(cx),
        };

        // collab has bad concurrency guarantees, so we send requests in serial.
        let update_project = if downstream_client.is_via_collab() {
            Some(downstream_client.request(update))
        } else {
            downstream_client.send(update).log_err();
            None
        };
        cx.spawn(async move |this, cx| {
            if let Some(update_project) = update_project {
                update_project.await?;
            }

            this.update(cx, |this, cx| {
                let worktrees = this.worktrees().collect::<Vec<_>>();

                for worktree in worktrees {
                    worktree.update(cx, |worktree, cx| {
                        let client = downstream_client.clone();
                        worktree.observe_updates(project_id, cx, {
                            move |update| {
                                let client = client.clone();
                                async move {
                                    if client.is_via_collab() {
                                        client
                                            .request(update)
                                            .map(|result| result.log_err().is_some())
                                            .await
                                    } else {
                                        client.send(update).log_err().is_some()
                                    }
                                }
                            }
                        });
                    });

                    cx.emit(WorktreeStoreEvent::WorktreeUpdateSent(worktree.clone()))
                }

                anyhow::Ok(())
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn worktree_metadata_protos(&self, cx: &App) -> Vec<proto::WorktreeMetadata> {
        self.worktrees()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                proto::WorktreeMetadata {
                    id: worktree.id().to_proto(),
                    root_name: worktree.root_name_str().to_owned(),
                    visible: worktree.is_visible(),
                    abs_path: worktree.abs_path().to_string_lossy().into_owned(),
                }
            })
            .collect()
    }

    pub fn shared(
        &mut self,
        remote_id: u64,
        downstream_client: AnyProtoClient,
        cx: &mut Context<Self>,
    ) {
        self.retain_worktrees = true;
        self.downstream_client = Some((downstream_client, remote_id));

        // When shared, retain all worktrees
        for worktree_handle in self.worktrees.iter_mut() {
            match worktree_handle {
                WorktreeHandle::Strong(_) => {}
                WorktreeHandle::Weak(worktree) => {
                    if let Some(worktree) = worktree.upgrade() {
                        *worktree_handle = WorktreeHandle::Strong(worktree);
                    }
                }
            }
        }
        self.send_project_updates(cx);
    }

    pub fn unshared(&mut self, cx: &mut Context<Self>) {
        self.retain_worktrees = false;
        self.downstream_client.take();

        // When not shared, only retain the visible worktrees
        for worktree_handle in self.worktrees.iter_mut() {
            if let WorktreeHandle::Strong(worktree) = worktree_handle {
                let is_visible = worktree.update(cx, |worktree, _| {
                    worktree.stop_observing_updates();
                    worktree.is_visible()
                });
                if !is_visible {
                    *worktree_handle = WorktreeHandle::Weak(worktree.downgrade());
                }
            }
        }
    }

    pub async fn handle_create_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CreateProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let worktree = this.update(&mut cx, |this, cx| {
            let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
            this.worktree_for_id(worktree_id, cx)
                .context("worktree not found")
        })?;
        Worktree::handle_create_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_copy_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::CopyProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let new_worktree_id = WorktreeId::from_proto(envelope.payload.new_worktree_id);
        let new_project_path = (
            new_worktree_id,
            RelPath::from_proto(&envelope.payload.new_path)?,
        );
        let (scan_id, entry) = this.update(&mut cx, |this, cx| {
            let Some((_, project_id)) = this.downstream_client else {
                bail!("no downstream client")
            };
            let Some(entry) = this.entry_for_id(entry_id, cx) else {
                bail!("no such entry");
            };
            if entry.is_private && project_id != REMOTE_SERVER_PROJECT_ID {
                bail!("entry is private")
            }

            let new_worktree = this
                .worktree_for_id(new_worktree_id, cx)
                .context("no such worktree")?;
            let scan_id = new_worktree.read(cx).scan_id();
            anyhow::Ok((
                scan_id,
                this.copy_entry(entry_id, new_project_path.into(), cx),
            ))
        })?;
        let entry = entry.await?;
        Ok(proto::ProjectEntryResponse {
            entry: entry.as_ref().map(|entry| entry.into()),
            worktree_scan_id: scan_id as u64,
        })
    }

    pub async fn handle_delete_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::DeleteProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this.update(&mut cx, |this, cx| {
            let Some((_, project_id)) = this.downstream_client else {
                bail!("no downstream client")
            };
            let Some(entry) = this.entry_for_id(entry_id, cx) else {
                bail!("no entry")
            };
            if entry.is_private && project_id != REMOTE_SERVER_PROJECT_ID {
                bail!("entry is private")
            }
            this.worktree_for_entry(entry_id, cx)
                .context("worktree not found")
        })?;
        Worktree::handle_delete_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_rename_project_entry(
        this: Entity<Self>,
        request: proto::RenameProjectEntry,
        mut cx: AsyncApp,
    ) -> Result<proto::ProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(request.entry_id);
        let new_worktree_id = WorktreeId::from_proto(request.new_worktree_id);
        let rel_path = RelPath::from_proto(&request.new_path)
            .with_context(|| format!("received invalid relative path {:?}", &request.new_path))?;

        let (scan_id, task) = this.update(&mut cx, |this, cx| {
            let worktree = this
                .worktree_for_entry(entry_id, cx)
                .context("no such worktree")?;

            let Some((_, project_id)) = this.downstream_client else {
                bail!("no downstream client")
            };
            let entry = worktree
                .read(cx)
                .entry_for_id(entry_id)
                .ok_or_else(|| anyhow!("missing entry"))?;
            if entry.is_private && project_id != REMOTE_SERVER_PROJECT_ID {
                bail!("entry is private")
            }

            let scan_id = worktree.read(cx).scan_id();
            anyhow::Ok((
                scan_id,
                this.rename_entry(entry_id, (new_worktree_id, rel_path).into(), cx),
            ))
        })?;
        Ok(proto::ProjectEntryResponse {
            entry: match &task.await? {
                CreatedEntry::Included(entry) => Some(entry.into()),
                CreatedEntry::Excluded { .. } => None,
            },
            worktree_scan_id: scan_id as u64,
        })
    }

    pub async fn handle_expand_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExpandProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ExpandProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this
            .update(&mut cx, |this, cx| this.worktree_for_entry(entry_id, cx))
            .context("invalid request")?;
        Worktree::handle_expand_entry(worktree, envelope.payload, cx).await
    }

    pub async fn handle_expand_all_for_project_entry(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::ExpandAllForProjectEntry>,
        mut cx: AsyncApp,
    ) -> Result<proto::ExpandAllForProjectEntryResponse> {
        let entry_id = ProjectEntryId::from_proto(envelope.payload.entry_id);
        let worktree = this
            .update(&mut cx, |this, cx| this.worktree_for_entry(entry_id, cx))
            .context("invalid request")?;
        Worktree::handle_expand_all_for_entry(worktree, envelope.payload, cx).await
    }

    pub fn fs(&self) -> Option<Arc<dyn Fs>> {
        match &self.state {
            WorktreeStoreState::Local { fs } => Some(fs.clone()),
            WorktreeStoreState::Remote { .. } => None,
        }
    }
}

#[derive(Clone, Debug)]
enum WorktreeHandle {
    Strong(Entity<Worktree>),
    Weak(WeakEntity<Worktree>),
}

impl WorktreeHandle {
    fn upgrade(&self) -> Option<Entity<Worktree>> {
        match self {
            WorktreeHandle::Strong(handle) => Some(handle.clone()),
            WorktreeHandle::Weak(handle) => handle.upgrade(),
        }
    }
}
