use crate::{
    Project, ProjectEntryId, ProjectItem, ProjectPath,
    worktree_store::{WorktreeStore, WorktreeStoreEvent},
};
use anyhow::{Context as _, Result};
use collections::{HashMap, hash_map};
use futures::StreamExt;
use gpui::{App, Context, Entity, EventEmitter, Subscription, Task, WeakEntity, prelude::*};
use language::{DiskState, File};
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::sync::Arc;
use util::ResultExt;
use worktree::{LoadedBinaryFile, PathChange, Worktree};

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct PdfId(NonZeroU64);

impl std::fmt::Display for PdfId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<NonZeroU64> for PdfId {
    fn from(id: NonZeroU64) -> Self {
        PdfId(id)
    }
}

#[derive(Debug)]
pub enum PdfItemEvent {
    Reloaded,
    FileHandleChanged,
}

impl EventEmitter<PdfItemEvent> for PdfItem {}

pub enum PdfStoreEvent {
    PdfAdded(Entity<PdfItem>),
}

impl EventEmitter<PdfStoreEvent> for PdfStore {}

pub struct PdfItem {
    pub id: PdfId,
    pub file: Arc<worktree::File>,
    pub data: Vec<u8>,
}

impl PdfItem {
    pub fn project_path(&self, cx: &App) -> ProjectPath {
        ProjectPath {
            worktree_id: self.file.worktree_id(cx),
            path: self.file.path().clone(),
        }
    }

    pub fn abs_path(&self, cx: &App) -> Option<PathBuf> {
        Some(self.file.as_local()?.abs_path(cx))
    }

    pub fn file_name<'a>(&'a self, cx: &'a App) -> &'a str {
        self.file.file_name(cx)
    }

    fn file_updated(&mut self, new_file: Arc<worktree::File>, cx: &mut Context<Self>) {
        let mut file_changed = false;

        let old_file = &self.file;
        if new_file.path() != old_file.path() {
            file_changed = true;
        }

        let old_state = old_file.disk_state();
        let new_state = new_file.disk_state();
        if old_state != new_state {
            file_changed = true;
        }

        self.file = new_file;
        if file_changed {
            cx.emit(PdfItemEvent::FileHandleChanged);
            cx.notify();
        }
    }

    pub fn reload(&mut self, cx: &mut Context<Self>) {
        let Some(local_file) = self.file.as_local() else {
            return;
        };

        let content = local_file.load_bytes(cx);
        cx.spawn(async move |this, cx| {
            if let Some(data) = content
                .await
                .context("Failed to load PDF content")
                .log_err()
            {
                this.update(cx, |this, cx| {
                    this.data = data;
                    cx.emit(PdfItemEvent::Reloaded);
                })
                .log_err();
            }
        })
        .detach();
    }
}

pub fn is_pdf_file(project: &Entity<Project>, path: &ProjectPath, cx: &App) -> bool {
    // First check the relative path's extension
    if let Some(ext) = path.path.extension() {
        return ext.eq_ignore_ascii_case("pdf");
    }

    // When opening a single file, Zed treats the file as the worktree root,
    // so the relative path is empty. Check the worktree's abs path instead.
    if let Some(abs_path) = project.read(cx).absolute_path(path, cx) {
        if let Some(ext) = abs_path.extension() {
            return ext.to_str().is_some_and(|e| e.eq_ignore_ascii_case("pdf"));
        }
    }

    false
}

impl ProjectItem for PdfItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        log::info!("PdfItem::try_open called for path: {:?}, extension: {:?}, is_pdf: {}", path.path, path.path.extension(), is_pdf_file(project, path, cx));
        if is_pdf_file(project, path, cx) {
            Some(cx.spawn({
                let path = path.clone();
                let project = project.clone();
                async move |cx| {
                    project
                        .update(cx, |project, cx| project.open_pdf(path, cx))
                        .await
                }
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
        self.file.entry_id
    }

    fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        Some(PdfItem::project_path(self, cx))
    }

    fn is_dirty(&self) -> bool {
        false
    }
}

struct LocalPdfStore {
    local_pdf_ids_by_path: HashMap<ProjectPath, PdfId>,
    local_pdf_ids_by_entry_id: HashMap<ProjectEntryId, PdfId>,
    pdf_store: WeakEntity<PdfStore>,
    _subscription: Subscription,
}

pub struct PdfStore {
    opened_pdfs: HashMap<PdfId, WeakEntity<PdfItem>>,
    worktree_store: Entity<WorktreeStore>,
    local_store: Entity<LocalPdfStore>,
    #[allow(clippy::type_complexity)]
    loading_pdfs_by_path: HashMap<
        ProjectPath,
        postage::watch::Receiver<Option<Result<Entity<PdfItem>, Arc<anyhow::Error>>>>,
    >,
}

impl PdfStore {
    pub fn local(worktree_store: Entity<WorktreeStore>, cx: &mut Context<Self>) -> Self {
        let this = cx.weak_entity();
        let local_store = cx.new(|cx| {
            let subscription = cx.subscribe(
                &worktree_store,
                |this: &mut LocalPdfStore, _, event, cx| {
                    if let WorktreeStoreEvent::WorktreeAdded(worktree) = event {
                        this.subscribe_to_worktree(worktree, cx);
                    }
                },
            );

            LocalPdfStore {
                local_pdf_ids_by_path: Default::default(),
                local_pdf_ids_by_entry_id: Default::default(),
                pdf_store: this,
                _subscription: subscription,
            }
        });

        Self {
            opened_pdfs: Default::default(),
            loading_pdfs_by_path: Default::default(),
            worktree_store,
            local_store,
        }
    }

    pub fn pdfs(&self) -> impl '_ + Iterator<Item = Entity<PdfItem>> {
        self.opened_pdfs
            .values()
            .filter_map(|pdf| pdf.upgrade())
    }

    pub fn get(&self, pdf_id: PdfId) -> Option<Entity<PdfItem>> {
        self.opened_pdfs
            .get(&pdf_id)
            .and_then(|pdf| pdf.upgrade())
    }

    pub fn get_by_path(&self, path: &ProjectPath, cx: &App) -> Option<Entity<PdfItem>> {
        self.pdfs()
            .find(|pdf| &pdf.read(cx).project_path(cx) == path)
    }

    pub fn open_pdf(
        &mut self,
        project_path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<PdfItem>>> {
        let existing_pdf = self.get_by_path(&project_path, cx);
        if let Some(existing_pdf) = existing_pdf {
            return Task::ready(Ok(existing_pdf));
        }

        let Some(worktree) = self
            .worktree_store
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow::anyhow!("no such worktree")));
        };

        let loading_watch = match self.loading_pdfs_by_path.entry(project_path.clone()) {
            hash_map::Entry::Occupied(entry) => entry.get().clone(),

            hash_map::Entry::Vacant(entry) => {
                let (mut tx, rx) = postage::watch::channel();
                entry.insert(rx.clone());

                let load_pdf = self.open_pdf_in_worktree(project_path.path.clone(), worktree, cx);

                cx.spawn({
                    let project_path = project_path.clone();
                    async move |this, cx| {
                        let load_result = load_pdf.await;
                        *tx.borrow_mut() = Some(this.update(cx, |this, _cx| {
                            this.loading_pdfs_by_path.remove(&project_path);
                            let pdf = load_result.map_err(Arc::new)?;
                            Ok(pdf)
                        })?);
                        anyhow::Ok(())
                    }
                })
                .detach();
                rx
            }
        };

        cx.background_spawn(async move {
            Self::wait_for_loading_pdf(loading_watch)
                .await
                .map_err(|e| anyhow::anyhow!("{}", e))
        })
    }

    fn open_pdf_in_worktree(
        &self,
        path: Arc<util::rel_path::RelPath>,
        worktree: Entity<Worktree>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<PdfItem>>> {
        let local_store = self.local_store.clone();

        let load_file = worktree.update(cx, |worktree, cx| {
            worktree.load_binary_file(path.as_ref(), cx)
        });

        cx.spawn(async move |pdf_store, cx| {
            let LoadedBinaryFile { file, content } = load_file.await?;

            let entity = cx.new(|cx| PdfItem {
                id: cx.entity_id().as_non_zero_u64().into(),
                file: file.clone(),
                data: content,
            });

            let pdf_id = cx.read_entity(&entity, |model, _| model.id);

            local_store.update(cx, |local_store, cx| {
                pdf_store.update(cx, |pdf_store, cx| {
                    pdf_store.add_pdf(entity.clone(), cx)
                })??;
                local_store.local_pdf_ids_by_path.insert(
                    ProjectPath {
                        worktree_id: file.worktree_id(cx),
                        path: file.path.clone(),
                    },
                    pdf_id,
                );

                if let Some(entry_id) = file.entry_id {
                    local_store
                        .local_pdf_ids_by_entry_id
                        .insert(entry_id, pdf_id);
                }

                anyhow::Ok(())
            })?;

            Ok(entity)
        })
    }

    async fn wait_for_loading_pdf(
        mut receiver: postage::watch::Receiver<
            Option<Result<Entity<PdfItem>, Arc<anyhow::Error>>>,
        >,
    ) -> Result<Entity<PdfItem>, Arc<anyhow::Error>> {
        loop {
            if let Some(result) = receiver.borrow().as_ref() {
                match result {
                    Ok(pdf) => return Ok(pdf.to_owned()),
                    Err(e) => return Err(e.to_owned()),
                }
            }
            receiver.next().await;
        }
    }

    fn add_pdf(&mut self, pdf: Entity<PdfItem>, cx: &mut Context<PdfStore>) -> Result<()> {
        let pdf_id = pdf.read(cx).id;
        self.opened_pdfs.insert(pdf_id, pdf.downgrade());
        cx.subscribe(&pdf, Self::on_pdf_event).detach();
        cx.emit(PdfStoreEvent::PdfAdded(pdf));
        Ok(())
    }

    fn on_pdf_event(
        &mut self,
        pdf: Entity<PdfItem>,
        event: &PdfItemEvent,
        cx: &mut Context<Self>,
    ) {
        if let PdfItemEvent::FileHandleChanged = event {
            self.local_store.update(cx, |local_store, cx| {
                local_store.pdf_changed_file(pdf, cx);
            });
        }
    }
}

impl LocalPdfStore {
    fn subscribe_to_worktree(&mut self, worktree: &Entity<Worktree>, cx: &mut Context<Self>) {
        cx.subscribe(worktree, |this, worktree, event, cx| {
            if worktree.read(cx).is_local()
                && let worktree::Event::UpdatedEntries(changes) = event
            {
                this.local_worktree_entries_changed(&worktree, changes, cx);
            }
        })
        .detach();
    }

    fn local_worktree_entries_changed(
        &mut self,
        worktree_handle: &Entity<Worktree>,
        changes: &[(Arc<util::rel_path::RelPath>, ProjectEntryId, PathChange)],
        cx: &mut Context<Self>,
    ) {
        let snapshot = worktree_handle.read(cx).snapshot();
        for (path, entry_id, _) in changes {
            self.local_worktree_entry_changed(*entry_id, path, worktree_handle, &snapshot, cx);
        }
    }

    fn local_worktree_entry_changed(
        &mut self,
        entry_id: ProjectEntryId,
        path: &Arc<util::rel_path::RelPath>,
        worktree: &Entity<worktree::Worktree>,
        snapshot: &worktree::Snapshot,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let project_path = ProjectPath {
            worktree_id: snapshot.id(),
            path: path.clone(),
        };
        let pdf_id = match self.local_pdf_ids_by_entry_id.get(&entry_id) {
            Some(&pdf_id) => pdf_id,
            None => self.local_pdf_ids_by_path.get(&project_path).copied()?,
        };

        let pdf = self
            .pdf_store
            .update(cx, |pdf_store, _| {
                if let Some(pdf) = pdf_store.get(pdf_id) {
                    Some(pdf)
                } else {
                    pdf_store.opened_pdfs.remove(&pdf_id);
                    None
                }
            })
            .ok()
            .flatten();
        let pdf = if let Some(pdf) = pdf {
            pdf
        } else {
            self.local_pdf_ids_by_path.remove(&project_path);
            self.local_pdf_ids_by_entry_id.remove(&entry_id);
            return None;
        };

        pdf.update(cx, |pdf, cx| {
            let old_file = &pdf.file;
            if old_file.worktree != *worktree {
                return;
            }

            let snapshot_entry = old_file
                .entry_id
                .and_then(|entry_id| snapshot.entry_for_id(entry_id))
                .or_else(|| snapshot.entry_for_path(old_file.path.as_ref()));

            let new_file = if let Some(entry) = snapshot_entry {
                worktree::File {
                    disk_state: match entry.mtime {
                        Some(mtime) => DiskState::Present {
                            mtime,
                            size: entry.size,
                        },
                        None => old_file.disk_state,
                    },
                    is_local: true,
                    entry_id: Some(entry.id),
                    path: entry.path.clone(),
                    worktree: worktree.clone(),
                    is_private: entry.is_private,
                }
            } else {
                worktree::File {
                    disk_state: DiskState::Deleted,
                    is_local: true,
                    entry_id: old_file.entry_id,
                    path: old_file.path.clone(),
                    worktree: worktree.clone(),
                    is_private: old_file.is_private,
                }
            };

            if new_file == **old_file {
                return;
            }

            if new_file.path != old_file.path {
                self.local_pdf_ids_by_path.remove(&ProjectPath {
                    path: old_file.path.clone(),
                    worktree_id: old_file.worktree_id(cx),
                });
                self.local_pdf_ids_by_path.insert(
                    ProjectPath {
                        worktree_id: new_file.worktree_id(cx),
                        path: new_file.path.clone(),
                    },
                    pdf_id,
                );
            }

            if new_file.entry_id != old_file.entry_id {
                if let Some(entry_id) = old_file.entry_id {
                    self.local_pdf_ids_by_entry_id.remove(&entry_id);
                }
                if let Some(entry_id) = new_file.entry_id {
                    self.local_pdf_ids_by_entry_id.insert(entry_id, pdf_id);
                }
            }

            pdf.file_updated(Arc::new(new_file), cx);
        });
        None
    }

    fn pdf_changed_file(&mut self, pdf: Entity<PdfItem>, cx: &mut App) -> Option<()> {
        let pdf = pdf.read(cx);
        let file = &pdf.file;

        let pdf_id = pdf.id;
        if let Some(entry_id) = file.entry_id {
            match self.local_pdf_ids_by_entry_id.get(&entry_id) {
                Some(_) => {
                    return None;
                }
                None => {
                    self.local_pdf_ids_by_entry_id.insert(entry_id, pdf_id);
                }
            }
        };
        self.local_pdf_ids_by_path.insert(
            ProjectPath {
                worktree_id: file.worktree_id(cx),
                path: file.path.clone(),
            },
            pdf_id,
        );

        Some(())
    }
}
