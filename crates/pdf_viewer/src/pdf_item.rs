use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Subscription, Task};
use language::{DiskState, File as _};
use project::{Project, ProjectEntryId, ProjectItem, ProjectPath};
use util::ResultExt as _;
use worktree::{File, Worktree};

use crate::pdf_document::PdfDocument;

pub struct PdfItem {
    file: Arc<File>,
    document: Arc<PdfDocument>,
    reload_task: Option<Task<()>>,
    _worktree_subscription: Subscription,
}

pub enum PdfItemEvent {
    Reloaded,
    FileHandleChanged,
}

impl EventEmitter<PdfItemEvent> for PdfItem {}

pub fn is_pdf_file(path: &ProjectPath) -> bool {
    path.path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
}

impl PdfItem {
    pub fn load(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let Some(worktree) = project.read(cx).worktree_for_id(path.worktree_id, cx) else {
            return Task::ready(Err(anyhow::anyhow!("no worktree for path {:?}", path.path)));
        };
        let path = path.path.clone();
        cx.spawn(async move |cx| {
            let loaded = worktree
                .update(cx, |worktree, cx| worktree.load_binary_file(&path, cx))
                .await?;
            let document = cx
                .background_spawn(async move { PdfDocument::parse(loaded.content) })
                .await?;
            Ok(cx.new(|cx| Self::new(loaded.file, Arc::new(document), worktree, cx)))
        })
    }

    fn new(
        file: Arc<File>,
        document: Arc<PdfDocument>,
        worktree: Entity<Worktree>,
        cx: &mut Context<Self>,
    ) -> Self {
        let worktree_subscription = cx.subscribe(&worktree, |this, worktree, event, cx| {
            if let worktree::Event::UpdatedEntries(changes) = event
                && changes.iter().any(|(path, entry_id, _)| {
                    this.file.entry_id == Some(*entry_id) || *path == this.file.path
                })
            {
                this.file_changed(&worktree, cx);
            }
        });
        Self {
            file,
            document,
            reload_task: None,
            _worktree_subscription: worktree_subscription,
        }
    }

    pub fn document(&self) -> &Arc<PdfDocument> {
        &self.document
    }

    pub fn file(&self) -> &Arc<File> {
        &self.file
    }

    pub fn file_name(&self, cx: &App) -> String {
        self.file.file_name(cx).to_string()
    }

    pub fn abs_path(&self, cx: &App) -> Option<PathBuf> {
        Some(self.file.as_local()?.abs_path(cx))
    }

    pub fn project_path(&self, cx: &App) -> ProjectPath {
        ProjectPath {
            worktree_id: self.file.worktree_id(cx),
            path: self.file.path().clone(),
        }
    }

    fn file_changed(&mut self, worktree: &Entity<Worktree>, cx: &mut Context<Self>) {
        let snapshot = worktree.read(cx).snapshot();
        let entry = self
            .file
            .entry_id
            .and_then(|entry_id| snapshot.entry_for_id(entry_id))
            .or_else(|| snapshot.entry_for_path(&self.file.path));
        let new_file = match entry {
            Some(entry) => File {
                disk_state: entry
                    .mtime
                    .map_or(self.file.disk_state, |mtime| DiskState::Present {
                        mtime,
                        size: entry.size,
                    }),
                entry_id: Some(entry.id),
                path: entry.path.clone(),
                ..File::clone(&self.file)
            },
            None => File {
                disk_state: DiskState::Deleted,
                ..File::clone(&self.file)
            },
        };
        if new_file == *self.file {
            return;
        }

        let content_changed = new_file.disk_state != self.file.disk_state
            && matches!(new_file.disk_state, DiskState::Present { .. });
        self.file = Arc::new(new_file);
        cx.emit(PdfItemEvent::FileHandleChanged);
        if content_changed {
            self.reload(worktree.clone(), cx);
        }
        cx.notify();
    }

    fn reload(&mut self, worktree: Entity<Worktree>, cx: &mut Context<Self>) {
        let path = self.file.path.clone();
        self.reload_task = Some(cx.spawn(async move |this, cx| {
            let document = async {
                let loaded = worktree
                    .update(cx, |worktree, cx| worktree.load_binary_file(&path, cx))
                    .await?;
                cx.background_spawn(async move { PdfDocument::parse(loaded.content) })
                    .await
            }
            .await
            // Tools that write a PDF in several steps trigger reloads of incomplete files;
            // the next change event reloads the finished file.
            .with_context(|| format!("reloading PDF {path:?}"))
            .log_err();
            if let Some(document) = document {
                this.update(cx, |this, cx| {
                    this.document = Arc::new(document);
                    cx.emit(PdfItemEvent::Reloaded);
                    cx.notify();
                })
                .log_err();
            }
        }));
    }
}

impl ProjectItem for PdfItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        if !is_pdf_file(path) {
            return None;
        }
        // Remote worktrees cannot load binary files yet, so remote PDFs keep the default view.
        let worktree = project.read(cx).worktree_for_id(path.worktree_id, cx)?;
        if !worktree.read(cx).is_local() {
            return None;
        }
        Some(Self::load(project, path, cx))
    }

    fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
        self.file.entry_id
    }

    fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        Some(self.project_path(cx))
    }

    fn is_dirty(&self) -> bool {
        false
    }
}
